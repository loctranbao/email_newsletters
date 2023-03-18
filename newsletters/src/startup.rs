use std::{net::TcpListener};
use actix_web::{web, HttpServer, App, dev::Server};
use crate::routes::{health_check, subscriptions};

/*
    Create http web server with contain an app
    to handle Http requests parser, routine to request handler
*/
pub fn run(listener : TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();

    Ok(server)
}