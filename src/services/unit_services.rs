use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::unit_repo};

pub fn add(
    pool: &PgPool,
    building_id: &Uuid,
    unit_number: &str,
    rent_amount: i32,
) -> Result<(), AppError> {
    if unit_number.is_empty() {
        return Err(AppError::BadRequest("unit number required".into()));
    }
    if rent_amount == 0 {
        return Err(AppError::BadRequest("rent amount required".into()));
    }
    let id = Uuid::new_v4();
    unit_repo::insert(pool, &id, building_id, unit_number, rent_amount)
}

pub fn assign(pool: &PgPool, unit_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
    let unit =
        unit_repo::find_by_id(pool, unit_id)?.ok_or_else(|| AppError::NotFound("unit".into()))?;

    if unit.tenant_id.is_some() {
        return Err(AppError::BadRequest("unit already occupied".into()));
    }

    unit_repo::assign_tenant(pool, unit_id, user_id)
}
