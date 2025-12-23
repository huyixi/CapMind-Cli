use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Note {
    pub id: Uuid,
    pub content: String,
    pub create_at: DateTime<Utc>,
}

impl Note {
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            create_at: Utc::now(),
        }
    }
}
