use uuid::Uuid;

use crate::{
    db::PgPool, entities::building::Building, error::AppError, repositories::building_repo,
};

pub fn add(
    pool: &PgPool,
    landlord_id: &Uuid,
    name: String,
    city: String,
    location: String,
    owner: String,
) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".into()));
    }

    if city.is_empty() {
        return Err(AppError::BadRequest("city is required".into()));
    }

    if location.is_empty() {
        return Err(AppError::BadRequest("location is required".into()));
    }

    if owner.is_empty() {
        return Err(AppError::BadRequest("owner is required".into()));
    }
    let id = Uuid::new_v4();
    building_repo::insert(pool, landlord_id, &id, &name, &city, &location, &owner)
}

pub fn remove(pool: &PgPool, landlord_id: &Uuid, id: &Uuid) -> Result<(), AppError> {
    building_repo::delete(pool, landlord_id, id)
}

pub fn assign(pool: &PgPool, caretaker_id: &Uuid, id: &Uuid) -> Result<(), AppError> {
    if building_repo::caretaker_is_assigned(pool, id)? {
        return Err(AppError::BadRequest("building already assigned".into()));
    }
    building_repo::assign_caretaker(pool, caretaker_id, id)
}

pub fn release(pool: &PgPool, caretaker_id: &Uuid) -> Result<(), AppError> {
    building_repo::release_caretaker(pool, caretaker_id)
}

pub fn find_by_lanlord(pool: &PgPool, landlord_id: &Uuid) -> Result<Vec<Building>, AppError> {
    building_repo::find_by_landlord(pool, landlord_id)
}
