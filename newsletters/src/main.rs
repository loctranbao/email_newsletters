
use std::net::TcpListener;
use sqlx::{PgPool};
use  env_logger::Env;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

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

    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();    

    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to posgres");
    
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
