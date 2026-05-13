use uuid::Uuid;

use crate::{db::PgPool, entities::payment::PaymentView, error::AppError};

pub fn payment_view_row(pool: &PgPool, tenant_id: &Uuid) -> Result<Vec<PaymentView>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT p.amount, p.month_year
        FROM payments p
        JOIN tenant_units tu ON tu.unit_id = p.unit_id
        WHERE tu.tenant_id = $1",
        &[tenant_id],
    )?;

    Ok(rows
        .into_iter()
        .map(|r| PaymentView {
            month_year: r.get("month_year"),
            amount: r.get::<_, i32>("amount"),
        })
        .collect::<Vec<PaymentView>>())
}

pub fn insert(pool: &PgPool, unit: &Uuid, amount: i32, month_year: String) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "INSERT INTO payments (unit_id, amount, month_year, confirmed) VALUES($1,$2,$3,$4)",
        &[unit, &amount, &month_year],
    )?;

    tracing::debug!(unit_id = %unit, "payment initiated");
    Ok(())
}
