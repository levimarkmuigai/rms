use uuid::Uuid;

use crate::{db::PgPool, entities::building::Building, error::AppError};

pub fn find_by_landlord(pool: &PgPool, landlord_id: &Uuid) -> Result<Vec<Building>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT * FROM buildings WHERE landlord_id = $1",
        &[&landlord_id],
    )?;

    Ok(rows
        .iter()
        .map(|r| Building {
            id: r.get("id"),
            landlord_id: r.get("landlord_id"),
            name: r.get("name"),
        })
        .collect())
}

pub fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Building>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query("SELECT * FROM buildings WHERE id = $1", &[&id])?;

    Ok(rows.first().map(|r| Building {
        id: r.get("id"),
        landlord_id: r.get("landlord_id"),
        name: r.get("name"),
    }))
}
