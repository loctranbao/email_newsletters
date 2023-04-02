use std::{net::TcpListener};
use actix_web::{web, HttpServer, App, dev::Server};
use sqlx::{PgPool};
use crate::routes::{health_check, subscriptions};
use tracing_actix_web::TracingLogger;

/*
    Create http web server with contain an app
    to handle Http requests parser, routine to request handler
*/
pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    
    let pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            // register the connection as part of the application state
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}