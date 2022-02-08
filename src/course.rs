use actix_web::{HttpResponse, post, Responder, Result, web};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::{Execute, PgPool};
use serde::Deserialize;

const DATABASE_URL: &str = env!("DATABASE_URL");

pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Deserialize)]
struct CourseRequest {
    id: String,
    title: String,
}

#[post("/courses")]
pub async fn create_course(course_reques: web::Json<CourseRequest>) -> impl Responder {
    create_course_service(&course_reques.id, &course_reques.title).await;

    HttpResponse::Created()
}

async fn create_course_service(id: &String, title: &String) {
    let repository = SqlxCourseRepository::new()
        .await
        .expect("Failed load course repository");

    let uuid = Uuid::parse_str(id).expect("Failed parse uuid");
    let now = Utc::now();

    let course = Course {id: uuid, title: title.to_string(), created_at: now, updated_at: now};

    repository.create_course(course).await;
}

struct SqlxCourseRepository {
    pool: PgPool
}

impl SqlxCourseRepository {
    async fn new() -> Result<Self> {
        let pool = PgPool::connect(DATABASE_URL)
            .await
            .expect("Error to connect to database");

        Ok(Self {pool})
    }

    async fn create_course(&self, course: Course) {
        sqlx::query(
            "INSERT INTO courses (uuid, title, created_at, updated_at) VALUES ($1, $2, $3, $4)"
        )
            .bind(course.id)
            .bind(course.title)
            .bind(course.created_at)
            .bind(course.updated_at)
            .execute(&self.pool)
            .await;
    }
}
