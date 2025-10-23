use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/index")]
async fn index()-> impl Responder{
    HttpResponse::Ok().body("This is the main page ")
}
#[actix_web::main]
async fn main ()-> std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    env_logger::init();
    HttpServer::new(||{
        App::new()
        .service(index)
        .wrap(Logger::default())
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}