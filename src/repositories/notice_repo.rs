use uuid::Uuid;

use crate::{db::PgPool, error::AppError};

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
