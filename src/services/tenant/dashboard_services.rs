use uuid::Uuid;

use crate::{
    db::PgPool,
    error::AppError,
    repositories::{payment_repo, unit_repo},
};

pub struct PaymentActivity {
    pub month_year: String,
    pub amount: i32,
    pub status: String,
}

pub fn header_data(pool: &PgPool, id: &Uuid) -> Result<(String, String, i32), AppError> {
    unit_repo::tenant_header_row(pool, id)
}

pub fn payment_activity(pool: &PgPool, id: &Uuid) -> Result<Vec<PaymentActivity>, AppError> {
    let payment_row = payment_repo::payment_view_row(pool, id)?;

    Ok(payment_row
        .into_iter()
        .map(|p| PaymentActivity {
            month_year: p.month_year,
            amount: p.amount,
            status: p.status,
        })
        .collect::<Vec<PaymentActivity>>())
}
