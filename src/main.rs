use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn parson(req: HttpRequest) -> impl Responder {
    match req.headers().get("Accept") {
        Some(a) if a == "application/activity+json" => {
            let file = std::fs::read_to_string("./resources/actor.json").unwrap();
            HttpResponse::Ok()
                .content_type("application/activity+json")
                .body(file)
        }
        _ => HttpResponse::NotAcceptable().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls_server()).unwrap();
    builder
        .set_private_key_file("./resources/localhost+1-key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("./resources/localhost+1.pem")
        .unwrap();

    HttpServer::new(|| App::new().service(parson))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
