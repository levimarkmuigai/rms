use uuid::Uuid;

use crate::{db::PgPool, entities::maintenance::RequestPanelRow, error::AppError};

pub fn dash_overview_row(pool: &PgPool, caretaker_id: &Uuid) -> Result<(i64, i64, i64), AppError> {
    let mut client = pool.get()?;

    let row = client.query_one(
        "SELECT
        COUNT(DISTINCT r.id) FILTER (WHERE r.status = 'pending') AS pending_count,
        COUNT(DISTINCT r.id) FILTER (WHERE r.status = 'in_progress') AS inprogress_count,
        COUNT(DISTINCT r.id) FILTER (WHERE r.status = 'resolved') AS resolved_count
        FROM maintenance_requests r
        JOIN units u ON u.id = r.unit_id
        JOIN buildings b ON b.id = u.building_id
        JOIN caretaker_buildings cb ON cb.building_id = b.id
        WHERE cb.caretaker_id = $1 AND cb.released_at IS NULL",
        &[&caretaker_id],
    )?;

    Ok((
        row.get::<_, i64>("pending_count"),
        row.get::<_, i64>("inprogress_count"),
        row.get::<_, i64>("resolved_count"),
    ))
}

pub fn find_panel_row(
    pool: &PgPool,
    caretaker_id: &Uuid,
) -> Result<Vec<RequestPanelRow>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT 
        r.id, r.description, u.unit_number as unit_number, r.created_at, r.status
        FROM maintenance_requests r
        JOIN units u ON u.id = r.unit_id
        JOIN buildings b ON b.id = u.building_id
        JOIN caretaker_buildings cb On cb.building_id = b.id
        WHERE cb.caretaker_id = $1
        AND cb.released_at IS NULL
        AND r.status != 'resolved'
        ORDER BY r.created_at ASC",
        &[&caretaker_id],
    )?;

    Ok(rows
        .iter()
        .map(|r| RequestPanelRow {
            id: r.get("id"),
            desc: r.get("description"),
            unit: r.get("unit_number"),
            status: r.get("status"),
            created_at: r.get("created_at"),
        })
        .collect())
}

pub fn pending_inprogress(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "UPDATE maintenance_requests SET status = 'in_progress'
        WHERE id = $1",
        &[&id],
    )?;

    tracing::debug!(%id, "set to in progress");
    Ok(())
}

pub fn inprogress_resolved(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "UPDATE maintenance_requests SET status = 'resolved'
        WHERE id = $1",
        &[id],
    )?;

    tracing::debug!(%id, "set to resolved");
    Ok(())
}
