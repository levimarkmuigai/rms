use std::time::SystemTime;

use uuid::Uuid;

use crate::{
    db::PgPool,
    error::AppError,
    repositories::{maintenance_repo, payment_repo, unit_repo},
};

pub struct RequestActivity {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub desc: String,
    pub timestamp: SystemTime,
    pub status: String,
}

pub struct PaymentActivity {
    pub month_year: String,
    pub amount: i32,
}

pub fn header_data(pool: &PgPool, id: &Uuid) -> Result<(String, String, i32), AppError> {
    unit_repo::tenant_header_row(pool, id)
}

pub fn request_activity(pool: &PgPool, id: &Uuid) -> Result<Vec<RequestActivity>, AppError> {
    let request_row = maintenance_repo::request_view_row(pool, id)?;

    Ok(request_row
        .into_iter()
        .map(|r| RequestActivity {
            id: r.id,
            unit_id: r.unit_id,
            desc: r.description,
            timestamp: r.submitted_at,
            status: r.status,
        })
        .collect::<Vec<RequestActivity>>())
}

pub fn payment_activity(pool: &PgPool, id: &Uuid) -> Result<Vec<PaymentActivity>, AppError> {
    let payment_row = payment_repo::payment_view_row(pool, id)?;

    Ok(payment_row
        .into_iter()
        .map(|p| PaymentActivity {
            month_year: p.month_year,
            amount: p.amount,
        })
        .collect::<Vec<PaymentActivity>>())
}
