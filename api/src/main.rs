use actix_cors::Cors;
use actix_web::{App, HttpResponse, post, HttpServer, Responder, web};
use mooc_context::courses::controller::CreateCourse as CreateCourseController;
use mooc_context::courses::controller::CourseRequest;
use mooc_context::courses::repository::PostgresCourseRepository;
use mooc_context::courses::service::CreateCourse as CreateCourseService;

const MOOC_API_IP: &str = env!("MOOC_API_IP");
const MOOC_API_PORT: &str = env!("MOOC_API_PORT");

fn get_address() -> String {
    format!("{}:{}", MOOC_API_IP, MOOC_API_PORT)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("🚀 Server thread running at {}", get_address());

        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .route("/health-check", web::get().to(HttpResponse::Ok))
            .service(create_course)
    })
    .bind(get_address())
    .unwrap_or_else(|_| panic!("🔥 Couldn't start the server at {}", get_address()))
    .run()
    .await
}

#[post("/courses")]
async fn create_course(course_reques: web::Json<CourseRequest>) -> impl Responder {
    let repository = PostgresCourseRepository::new().await.expect("Error");
    let service = CreateCourseService::new(repository);
    let controller = CreateCourseController::new(service);

    controller.execute(&course_reques).await;

    HttpResponse::Created()
}
