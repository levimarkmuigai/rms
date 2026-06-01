use uuid::Uuid;

use crate::{
    db::PgPool,
    entities::building::Building,
    error::AppError,
    repositories::{building_repo, unit_repo},
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
    let new_buildings_units = unit_repo::count_by_building(pool, id)?;
    let current_assignments = building_repo::find_by_caretaker(pool, caretaker_id)?;

    if !current_assignments.is_empty() {
        if new_buildings_units >= 20 {
            return Err(AppError::BadRequest(
                "building with 20 or more units require a dedicated caretaker".into(),
            ));
        }
        for assigned_id in &current_assignments {
            let count = unit_repo::count_by_building(pool, assigned_id)?;
            if count >= 20 {
                return Err(AppError::BadRequest(
                    "caretaker already manages a large building cannot take additional assignments"
                        .into(),
                ));
            }
        }
    }
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
