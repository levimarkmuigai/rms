use std::time::SystemTime;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NoticeForm {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub submitted_at: SystemTime,
    pub date: String,
}

pub struct NoticeDisplay {
    pub date: String,
    pub status: String,
    pub submitted_at: SystemTime,
    pub reason: Option<String>,
}
