use std::error::Error;
use chrono::Utc;
use uuid::Uuid;
use crate::courses::{Course, CourseRepository};

pub struct CreateCourse<R: CourseRepository> {
    repository: R
}

impl<R: CourseRepository> CreateCourse<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub(crate) async fn execute(&self, id: &String, title: &String) -> Result<(), Box<dyn Error>> {
        let uuid = Uuid::parse_str(id)?;
        let now = Utc::now();

        let course = Course {id: uuid, title: title.to_string(), created_at: now, updated_at: now};

        self.repository.create_course(course).await?;

        Ok(())
    }
}
