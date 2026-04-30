use uuid::Uuid;

use crate::entities::user::Role;

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: Uuid,
    pub role: Role,
    pub name: String,
}
