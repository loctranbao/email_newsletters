use actix_web::{HttpRequest, Responder, HttpResponse};

/*
    handler to check if our web server still healthy or not
    simply by return Http 200 OK code
 */
pub async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}