use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::unit_repo};

pub fn add(
    pool: &PgPool,
    id: &Uuid,
    building_id: &Uuid,
    unit_number: &str,
    rent_amount: i32,
) -> Result<(), AppError> {
    unit_repo::insert(pool, id, building_id, unit_number, rent_amount)
}

pub fn assign(pool: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<(), AppError> {
    if unit_repo::is_occupied(pool, id)? {
        return Err(AppError::BadRequest("unit is occupied".into()));
    }
    unit_repo::assign_tenant(pool, id, user_id)
}

pub fn vacate_tenant(pool: &PgPool, unit_id: &Uuid) -> Result<(), AppError> {
    unit_repo::vacate_tenant(pool, unit_id)
}
