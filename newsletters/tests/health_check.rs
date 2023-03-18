//! tests/health_check.rs

use std::{net::TcpListener};
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address  = spawn_app();
    // println!("testing on address : {:?}", address);

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
                                .get(format!("{}/health_check", address))
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
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=chenlog&email=loc.tranbao%40outlook.com";
    let response = client
                        .post(format!("{}/subscriptions", address))
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(body)
                        .send()
                        .await
                        .expect("failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_when_data_is_missing() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let test_cases = vec![
        ("name=chenlog", "missing the email"),
        ("email=loc.tranbao%40outlook.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
                        .post(format!("{}/subscriptions", address))
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

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)

}