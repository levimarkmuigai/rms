use std::time::SystemTime;

use uuid::Uuid;

pub struct ActivityLog {
    pub log_id: Uuid,
    pub action: String,
    pub created_at: SystemTime,
    pub email: String,
}
