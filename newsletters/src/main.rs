
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};


/*
    asynchronous runtime for webserver
    we use tokio to generate the run time for our webserver

    what it does is simply generate code to wrap the code in the main
    to an async block

    then it generates and executor and put that async task to run
    and wait for future correspondin the that task
 */
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration");

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let connection_pool = PgPoolOptions::new()
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.with_db());
    
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
