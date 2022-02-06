use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};

const MOOC_API_IP: &str = env!("MOOC_API_IP");
const MOOC_API_PORT: &str = env!("MOOC_API_PORT");

fn get_address() -> String {
    format!("{}:{}", MOOC_API_IP, MOOC_API_PORT)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("🚀 Server thread running at {}", get_address());

        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .route("/health-check", web::get().to(HttpResponse::Ok))
    })
    .bind(get_address())
    .unwrap_or_else(|_| panic!("🔥 Couldn't start the server at {}", get_address()))
    .run()
    .await
}
