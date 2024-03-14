use actix_web::{ http::header::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

mod tweet;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::get)
            .service(tweet::get_by_id)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
