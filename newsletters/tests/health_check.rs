//! tests/health_check.rs

use std::{net::TcpListener};
use once_cell::sync::Lazy;
use sqlx::{PgConnection, Connection, PgPool, Executor};

use uuid::Uuid;
use zero2prod::{configuration::{get_configuration, DatabaseSettings}, telemetry::{get_subscriber, init_subscriber}};
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app  = spawn_app().await;
    // println!("testing on address : {:?}", address);

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
                                .get(format!("{}/health_check", app.address))
                                .send()
                                .await
                                .expect("failed to execute request");


    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());                            
}


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    
    // Act
    let client = reqwest::Client::new();
    let body = "name=chenlog&email=loc.tranbao%40outlook.com";
    let response = client
                        .post(format!("{}/subscriptions", &app.address))
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(body)
                        .send()
                        .await
                        .expect("failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "loc.tranbao@outlook.com");
    assert_eq!(saved.name, "chenlog");

}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_when_data_is_missing() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let test_cases = vec![
        ("name=chenlog", "missing the email"),
        ("email=loc.tranbao%40outlook.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
                        .post(format!("{}/subscriptions", app.address))
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(invalid_body)
                        .send()
                        .await
                        .expect("failed to execute request");

        // Assert
        assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not fail with 400 bad request when the payload was {}",
                error_message
            );        
    }

}

//  Ensure that the tracing stack is only initlalised once using once_cell
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    // We cannot assign the output of get_subscriber to a variable based on the value of TEST_LOG
    // because the sink is part of the type returned by get_subscriber, therefore they are not the
    // same type, We could work around it, but this is the most straight-forward way of moving forward.

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);        
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
async fn spawn_app() -> TestApp {
    //  The first time initialize is invoked the code in TRACING is executed
    //  All other invocation will instead skip execution
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

    let mut configuration = get_configuration().expect("failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    
    // the 'Connection' trait MUST be in scope for us to invoke
    // 'PgConnection::connect' - it is not an inherent method of the struct!
    let connection_pool = configure_database(&configuration.database).await;

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let address = format!("http://127.0.0.1:{}", port);
    let _ = tokio::spawn(server);

    // We return the application address to the caller!
    TestApp {
        address,
        db_pool: connection_pool
    }

}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    //  Create databse
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    //  migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool

}