use uuid::Uuid;

use crate::{
    db::PgPool,
    entities::unit::{Unit, UnitPortfolioSummary, UnitStatus},
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

    client.execute(
        "UPDATE units set tenant_id = $1, status = 'occupied' WHERE id = $2",
        &[user_id, unit_id],
    )?;
    tracing::debug!(%unit_id, %user_id, "tenant assigned");
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
            rent_amount: r.get("rent_amount"),
            status: status_str.parse().unwrap_or(UnitStatus::Vacant),
            tenant_id: r.get("tenant_id"),
        }
    }))
}

pub fn unit_portfolio_summary(
    pool: &PgPool,
    building_id: &Uuid,
) -> Result<Vec<UnitPortfolioSummary>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT
        id,
        rent_amount,
        unit_number,
        status
        FROM units
        WHERE building_id = $1
        ",
        &[&building_id],
    )?;

    Ok(rows
        .iter()
        .map(|r| UnitPortfolioSummary {
            id: r.get("id"),
            number: r.get("unit_number"),
            rent_amount: r.get("rent_amount"),
            status: r.get("status"),
        })
        .collect())
}
