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
