use std::time::SystemTime;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Notice {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub submitted_at: SystemTime,
    pub date: String,
}
