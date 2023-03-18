
use std::net::TcpListener;
use zero2prod::startup::run;
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
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}


//  this is the code tokio generated behind the scene look like
/*
    fn main() -> std::io::Result<()> {
        let body = async move {
        HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
        };
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
        }
 */
