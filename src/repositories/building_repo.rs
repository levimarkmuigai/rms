use uuid::Uuid;

use crate::{
    db::PgPool,
    entities::building::{Building, BuildingOverviewRow, BuildingSummaryRow},
    error::AppError,
};

pub fn find_by_landlord(pool: &PgPool, landlord_id: &Uuid) -> Result<Vec<Building>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT * FROM buildings WHERE landlord_id = $1 ORDER BY created_at ASC",
        &[landlord_id],
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
    let rows = client.query(
        "SELECT * FROM buildings WHERE id = $1 ORDER BY created_at ASC",
        &[id],
    )?;

    Ok(rows.first().map(|r| Building {
        id: r.get("id"),
        landlord_id: r.get("landlord_id"),
        name: r.get("name"),
    }))
}

pub fn portfolio_unit_stats(
    pool: &PgPool,
    landlord_id: &Uuid,
) -> Result<(i64, i64, i64, i32), AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "
        SELECT
        COUNT(u.id) AS total_units,
        COUNT(u.id) FILTER (WHERE u.status = 'occupied') AS occupied,
        COUNT(u.id) FILTER (WHERE u.status = 'vacant') AS vacant,
        CAST(COALESCE(SUM(u.rent_amount) FILTER (WHERE u.status = 'occupied'), 0) AS INT) AS expected_revenue
        FROM units u
        JOIN buildings b on b.id = u.building_id
        WHERE b.landlord_id = $1",
        &[landlord_id],
    )?;

    let row = rows.first().ok_or(AppError::NotFound("stats".into()))?;
    Ok((
        row.get::<_, i64>("total_units"),
        row.get::<_, i64>("occupied"),
        row.get::<_, i64>("vacant"),
        row.get::<_, i32>("expected_revenue"),
    ))
}

pub fn collected_this_month(
    pool: &PgPool,
    landlord_id: &Uuid,
    month_year: &str,
) -> Result<i32, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "
        SELECT CAST(COALESCE(SUM(p.amount), 0) AS INT) AS collected
        FROM payments p
        JOIN units u ON u.id = p.unit_id
        JOIN buildings b on b.id = u.building_id
        WHERE b.landlord_id = $1
        AND p.month_year = $2",
        &[landlord_id, &month_year],
    )?;

    Ok(rows
        .first()
        .map(|r| r.get::<_, i32>("collected"))
        .unwrap_or(0))
}

pub fn arrears_stats(
    pool: &PgPool,
    landlord_id: &Uuid,
    month_year: &str,
) -> Result<(i32, i64), AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "
        SELECT CAST(COALESCE(SUM(u.rent_amount), 0) AS INT) AS total_arrears,
        COUNT(DISTINCT u.tenant_id) AS tenant_count
        FROM units u
        JOIN buildings b ON b.id = u.building_id
        WHERE b.landlord_id = $1
        AND u.status = 'occupied'
        AND NOT EXISTS (
        SELECT 1 FROM payments p
        WHERE p.unit_id = u.id
        AND p.month_year = $2
        )",
        &[landlord_id, &month_year],
    )?;

    let row = rows
        .first()
        .ok_or_else(|| AppError::NotFound("arrears".into()))?;

    Ok((
        row.get::<_, i32>("total_arrears"),
        row.get::<_, i64>("tenant_count"),
    ))
}

pub fn building_summeries(
    pool: &PgPool,
    landlord_id: &Uuid,
    month_year: &str,
) -> Result<Vec<BuildingSummaryRow>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT
        b.id,
        b.name,
        COUNT(u.id) AS total_units,
        COUNT(u.id) FILTER (WHERE u.status = 'occupied') AS occupied,
        CAST(COALESCE(SUM(p.amount), 0) AS INT) AS collected
        FROM buildings b
        LEFT JOIN units u ON u.building_id = b.id
        LEFT JOIN payments p ON p.unit_id = u.id AND p.month_year = $2
        WHERE b.landlord_id = $1
        GROUP BY b.id, b.name",
        &[landlord_id, &month_year],
    )?;

    Ok(rows
        .iter()
        .map(|r| BuildingSummaryRow {
            id: r.get("id"),
            name: r.get("name"),
            total_units: r.get::<_, i64>("total_units"),
            occupied: r.get::<_, i64>("occupied"),
            collected: r.get::<_, i32>("collected"),
        })
        .collect())
}

pub fn buildings_overview_rows(
    pool: &PgPool,
    landlord_id: &Uuid,
) -> Result<Vec<BuildingOverviewRow>, AppError> {
    let mut client = pool.get()?;

    let request_status = "resolved";

    let rows = client.query(
        "SELECT
        b.name AS name,
        c.id AS caretaker_id,
        c.name AS caretaker_name,
        c.number AS caretaker_number,
        COUNT(DISTINCT m.id) as requests
        FROM buildings b
        LEFT JOIN caretaker_buildings cb ON cb.building_id = b.id AND cb.released_at IS NULL
        LEFT JOIN users c on c.id = cb.caretaker_id
        LEFT JOIN units u ON u.building_id = b.id
        LEFT JOIN maintenance_requests m ON m.unit_id = u.id AND m.status != $2
        WHERE b.landlord_id = $1
        GROUP BY b.id, b.name, c.id,c.name,c.email
        ORDER BY b.created_at ASC",
        &[landlord_id, &request_status],
    )?;

    Ok(rows
        .iter()
        .map(|r| BuildingOverviewRow {
            name: r.get("name"),
            caretaker_id: r.get("caretaker_id"),
            caretaker_name: r.get("caretaker_name"),
            caretaker_number: r.get("caretaker_number"),
            requests: r.get("requests"),
        })
        .collect())
}

pub fn insert(pool: &PgPool, landlord_id: &Uuid, id: &Uuid, name: &str) -> Result<(), AppError> {
    let mut client = pool.get()?;
    client.execute(
        "INSERT INTO buildings (id,landlord_id,name) VALUES($1,$2,$3)",
        &[id, landlord_id, &name],
    )?;
    tracing::debug!(%name, "building inserted");
    Ok(())
}

pub fn delete(pool: &PgPool, landlord_id: &Uuid, id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "DELETE FROM buildings WHERE id = $1 AND landlord_id = $2",
        &[id, landlord_id],
    )?;
    tracing::debug!(%id, "building deleted");
    Ok(())
}

pub fn assign_caretaker(
    pool: &PgPool,
    caretaker_id: &Uuid,
    building_id: &Uuid,
) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "INSERT INTO caretaker_buildings (caretaker_id, building_id) VALUES ($1,$2)",
        &[caretaker_id, building_id],
    )?;
    tracing::debug!(%caretaker_id, %building_id, "assigned");
    Ok(())
}

pub fn caretaker_is_assigned(pool: &PgPool, building_id: &Uuid) -> Result<bool, AppError> {
    let mut client = pool.get()?;
    let rows = client.query_opt(
        "SELECT 1 FROM caretaker_buildings WHERE building_id = $1 AND released_at IS NULL",
        &[building_id],
    )?;
    Ok(rows.is_some())
}

pub fn release_caretaker(pool: &PgPool, caretaker_id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;
    client.execute(
        "UPDATE caretaker_buildings 
        SET released_at = NOW() 
        WHERE caretaker_id = $1 
        AND released_at IS NULL",
        &[caretaker_id],
    )?;

    tracing::debug!(%caretaker_id, "unassigned");
    Ok(())
}
