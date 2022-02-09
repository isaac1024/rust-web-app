mod courses;

use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct BadRequest {
    description: String
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health-check", web::get().to(HttpResponse::Ok))
        .service(web::scope("/courses").configure(courses::routes));
}
