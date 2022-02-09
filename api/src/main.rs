mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use routes::routes;

const MOOC_API_IP: &str = env!("MOOC_API_IP");
const MOOC_API_PORT: &str = env!("MOOC_API_PORT");

fn get_address() -> String {
    format!("{}:{}", MOOC_API_IP, MOOC_API_PORT)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .configure(routes)
    })
    .bind(get_address());

    match server {
        Ok(s) => {
            println!("🚀 Server running at {}", get_address());
            s.run().await
        }
        Err(_) => panic!("🔥 Could not start the server at {}", get_address())
    }
}
