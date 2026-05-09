use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::maintenance_repo};

pub fn dash_overview(pool: &PgPool, id: &Uuid) -> Result<(i64, i64, i64), AppError> {
    maintenance_repo::dash_overview_row(pool, id)
}
