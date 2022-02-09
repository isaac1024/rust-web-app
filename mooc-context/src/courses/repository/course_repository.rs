use std::error::Error;
use sqlx::PgPool;
use async_trait::async_trait;
use crate::courses::{Course, CourseRepository};

const DATABASE_URL: &str = env!("DATABASE_URL");

pub struct PostgresCourseRepository {
    pool: PgPool
}

impl PostgresCourseRepository {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let pool = PgPool::connect(DATABASE_URL)
            .await?;

        Ok(Self {pool})
    }
}

#[async_trait]
impl CourseRepository for PostgresCourseRepository {
    async fn create_course(&self, course: Course) -> Result<(), Box<dyn Error>> {
        sqlx::query(
                r#"
                INSERT INTO courses
                (uuid, title, created_at, updated_at)
                VALUES ($1, $2, $3, $4)
                "#
            )
            .bind(course.id)
            .bind(course.title)
            .bind(course.created_at)
            .bind(course.updated_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
