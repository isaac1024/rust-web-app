use sqlx::{PgPool, Result};
use async_trait::async_trait;
use crate::courses::{Course, CourseRepository};

const DATABASE_URL: &str = env!("DATABASE_URL");

pub struct PostgresCourseRepository {
    pool: PgPool
}

impl PostgresCourseRepository {
    pub async fn new() -> Result<Self> {
        let pool = PgPool::connect(DATABASE_URL)
            .await
            .expect("Error to connect to database");

        Ok(Self {pool})
    }
}

#[async_trait]
impl CourseRepository for PostgresCourseRepository {
    async fn create_course(&self, course: Course) {
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
            .await;
    }
}
