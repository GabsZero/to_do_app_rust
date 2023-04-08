use actix_web::{web, App, HttpServer, Responder, HttpRequest};

mod views;
mod state;
mod to_do;
mod processes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("http server factory is firing");
        let app = App::new().configure(views::views_factory);
        return app;
        
    }).bind("127.0.0.1:8080")?.run().await
}