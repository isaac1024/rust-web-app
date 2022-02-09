use actix_web::{HttpResponse, web};
use mooc_context::courses::controller::CreateCourse as CreateCourseController;
use mooc_context::courses::controller::CourseRequest;
use mooc_context::courses::repository::PostgresCourseRepository;
use mooc_context::courses::service::CreateCourse as CreateCourseService;
use crate::routes::BadRequest;

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_course))
    );
}

async fn create_course(course_request: web::Json<CourseRequest>) -> HttpResponse {
    let repository =  match PostgresCourseRepository::new().await {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().json(
            BadRequest{ description: e.to_string() }
        )
    };
    let service = CreateCourseService::new(repository);
    let controller = CreateCourseController::new(service);

    match controller.execute(&course_request).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::BadRequest().json(
            BadRequest{ description: e.to_string() }
        )
    }


}

