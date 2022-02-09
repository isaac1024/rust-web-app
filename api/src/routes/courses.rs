use actix_web::{HttpResponse, Responder, web};
use mooc_context::courses::controller::CreateCourse as CreateCourseController;
use mooc_context::courses::controller::CourseRequest;
use mooc_context::courses::repository::PostgresCourseRepository;
use mooc_context::courses::service::CreateCourse as CreateCourseService;

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_course))
    );
}

async fn create_course(course_request: web::Json<CourseRequest>) -> impl Responder {
    let repository = PostgresCourseRepository::new().await.expect("Error");
    let service = CreateCourseService::new(repository);
    let controller = CreateCourseController::new(service);

    controller.execute(&course_request).await;

    HttpResponse::Created()
}
