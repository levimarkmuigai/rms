use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::building_repo};

pub fn add(pool: &PgPool, landlord_id: &Uuid, name: String) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".into()));
    }

    let id = Uuid::new_v4();
    building_repo::insert(pool, landlord_id, &id, &name)
}

pub fn remove(pool: &PgPool, landlord_id: &Uuid, id: &Uuid) -> Result<(), AppError> {
    building_repo::delete(pool, landlord_id, id)
}
