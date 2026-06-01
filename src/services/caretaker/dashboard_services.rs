use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::maintenance_repo};

pub fn to_inprogress(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    maintenance_repo::pending_inprogress(pool, id)
}

pub fn to_resolved(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    maintenance_repo::inprogress_resolved(pool, id)
}
