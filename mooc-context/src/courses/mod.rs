pub mod service;
pub mod repository;
pub mod controller;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository {
    async fn create_course(&self, course: Course);
}

pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}
