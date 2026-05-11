use uuid::Uuid;

use crate::{
    db::PgPool,
    entities::unit::{Unit, UnitStatus, UnitSummaryRow},
    error::AppError,
};

pub fn insert(
    pool: &PgPool,
    id: &Uuid,
    building_id: &Uuid,
    unit_number: &str,
    rent_amount: i32,
) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "INSERT INTO units (id, building_id, unit_number,rent_amount)
        VALUES ($1,$2,$3,$4)",
        &[id, building_id, &unit_number, &rent_amount],
    )?;
    Ok(())
}

pub fn assign_tenant(pool: &PgPool, unit_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;
    let mut transaction = client.transaction()?;

    transaction.execute(
        "UPDATE units set tenant_id = $1, status = 'occupied' WHERE id = $2",
        &[user_id, unit_id],
    )?;

    transaction.execute(
        "INSERT INTO tenant_units (tenant_id, unit_id) VALUES($1,$2)",
        &[user_id, unit_id],
    )?;

    transaction.commit()?;

    tracing::debug!(%unit_id, %user_id, "tenant assigned");
    Ok(())
}

pub fn vacate_tenant(pool: &PgPool, unit_id: &Uuid) -> Result<(), AppError> {
    let mut client = pool.get()?;
    let mut transaction = client.transaction()?;

    transaction.execute(
        "UPDATE units SET tenant_id = NULL, status = 'vacant' WHERE id = $1",
        &[&unit_id],
    )?;

    transaction.execute(
        "UPDATE tenant_units SET vacated_at = NOW() WHERE unit_id = $1 AND vacated_at IS NULL",
        &[&unit_id],
    )?;

    transaction.commit()?;
    tracing::debug!(%unit_id, "vacated");
    Ok(())
}

pub fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Unit>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query("SELECT * FROM units WHERE id = $1", &[id])?;

    Ok(rows.first().map(|r| {
        let status_str: String = r.get("status");
        Unit {
            id: r.get("id"),
            building_id: r.get("building_id"),
            unit_number: r.get("unit_number"),
            rent_amount: r.get::<_, i32>("rent_amount"),
            status: status_str.parse().unwrap_or(UnitStatus::Vacant),
            tenant_id: r.get("tenant_id"),
        }
    }))
}

pub fn find_by_tenant(pool: &PgPool, tenant_id: &Uuid) -> Result<Option<Uuid>, AppError> {
    let mut client = pool.get()?;
    let row = client.query_opt("SELECT id FROM units WHERE tenant_id = $1", &[tenant_id])?;
    Ok(row.map(|r| r.get("id")))
}

pub fn is_occupied(pool: &PgPool, unit_id: &Uuid) -> Result<bool, AppError> {
    let mut client = pool.get()?;
    let rows = client.query_opt(
        "SELECT 1 FROM tenant_units WHERE unit_id = $1 AND vacated_at IS NULL",
        &[unit_id],
    )?;

    Ok(rows.is_some())
}

pub fn unit_summary_row(
    pool: &PgPool,
    building_id: &Uuid,
) -> Result<Vec<UnitSummaryRow>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT
        id,
        rent_amount,
        unit_number,
        status
        FROM units
        WHERE building_id = $1
        ORDER BY created_at ASC
        ",
        &[&building_id],
    )?;

    Ok(rows
        .iter()
        .map(|r| UnitSummaryRow {
            id: r.get("id"),
            number: r.get("unit_number"),
            rent_amount: r.get::<_, i32>("rent_amount"),
            status: r.get("status"),
        })
        .collect())
}

pub fn tenant_header_row(
    pool: &PgPool,
    tenant_id: &Uuid,
) -> Result<(String, String, i32), AppError> {
    let mut client = pool.get()?;
    let rows = client.query_one(
        "SELECT u.unit_number, b.name AS building_name, u.rent_amount
        FROM units u
        JOIN buildings b ON b.id = u.building_id
        JOIN tenant_units tu ON tu.unit_id = u.id
        WHERE tu.tenant_id = $1 AND vacated_at IS NULL",
        &[tenant_id],
    )?;

    Ok((
        rows.get("unit_number"),
        rows.get("building_name"),
        rows.get::<_, i32>("rent_amount"),
    ))
}
