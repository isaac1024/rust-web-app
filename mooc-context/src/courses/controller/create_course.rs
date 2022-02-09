use std::error::Error;
use serde::Deserialize;
use crate::courses::repository::PostgresCourseRepository;
use crate::courses::service::CreateCourse as CreateCourseService;

#[derive(Deserialize)]
pub struct CourseRequest {
    id: String,
    title: String,
}

pub struct CreateCourse {
    service: CreateCourseService<PostgresCourseRepository>
}

impl CreateCourse {
    pub fn new(service: CreateCourseService<PostgresCourseRepository>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, request: &CourseRequest) -> Result<(), Box<dyn Error>> {
        self.service.execute(&request.id, &request.title).await?;

        Ok(())
    }
}
