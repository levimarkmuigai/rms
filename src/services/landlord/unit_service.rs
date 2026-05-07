use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::unit_repo};

#[derive(Debug, Clone)]
pub struct UnitStats {
    pub id: Uuid,
    pub number: String,
    pub status: String,
    pub rent_amount: i32,
}

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

pub fn unit_stats(pool: &PgPool, building_id: &Uuid) -> Result<Vec<UnitStats>, AppError> {
    let unit_stats = unit_repo::unit_summary_row(pool, building_id)?;

    Ok(unit_stats
        .into_iter()
        .map(|us| UnitStats {
            id: us.id,
            number: us.number,
            status: us.status,
            rent_amount: us.rent_amount,
        })
        .collect())
}
