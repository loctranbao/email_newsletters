use std::{net::TcpListener};

use actix_web::{web, HttpServer, App, HttpRequest, Responder, HttpResponse, dev::Server};

#[derive(serde::Deserialize)]
struct FormData {
    email : String,
    name  : String
}


/*
    handler to check if our web server still healthy or not
    simply by return Http 200 OK code
 */
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

/*
    handler to process user subscribe to newsletters
 */
async fn subscriptions(form : web::Form<FormData>) -> impl Responder{
    HttpResponse::Ok().finish()
}

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