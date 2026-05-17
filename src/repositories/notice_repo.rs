use std::time::SystemTime;

use uuid::Uuid;

use crate::{db::PgPool, entities::notice::Notice, error::AppError};

pub fn insert(
    pool: &PgPool,
    unit_id: &Uuid,
    tenant_id: &Uuid,
    date: String,
) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "INSERT INTO vacation_notices (unit_id,submitted_by,date)
        VALUES($1,$2,$3)",
        &[unit_id, tenant_id, &date],
    )?;

    tracing::debug!(%tenant_id, "notice submitted");

    Ok(())
}

pub fn approve(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "UPDATE vacation_notices SET status = 'approved'
        WHERE id = $1",
        &[id],
    )?;

    tracing::debug!(%id, "notice approved");
    Ok(())
}

pub fn reject(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "UPDATE vacation_notices SET status = 'rejected'
        WHERE id = $1",
        &[id],
    )?;

    tracing::debug!(%id, "notice rejected");
    Ok(())
}

pub fn find_pending(pool: &PgPool, unit_id: &Uuid) -> Result<Option<Notice>, AppError> {
    let mut client = pool.get()?;

    let row = client.query_opt(
        "SELECT id, unit_id, submitted_at, date
        FROM vacation_notices
        WHERE unit_id = $1 AND status = 'pending' ",
        &[unit_id],
    )?;

    Ok(row.map(|r| Notice {
        id: r.get("id"),
        unit_id: r.get("unit_id"),
        submitted_at: r.get("submitted_at"),
        date: r.get("date"),
    }))
}

pub fn tenant_view(
    pool: &PgPool,
    tenant_id: &Uuid,
) -> Result<Vec<(String, String, SystemTime)>, AppError> {
    let mut client = pool.get()?;

    let rows = client.query(
        "SELECT date, status, submitted_at
        FROM vacation_notices
        WHERE submitted_by = $1",
        &[tenant_id],
    )?;

    Ok(rows
        .into_iter()
        .map(|r| (r.get("date"), r.get("status"), r.get("submitted_at")))
        .collect())
}
