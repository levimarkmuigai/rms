use std::time::SystemTime;

use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::maintenance_repo};

pub struct PanelRequests {
    pub id: Uuid,
    pub desc: String,
    pub unit: String,
    pub status: String,
    pub timestamp: SystemTime,
}

pub fn dash_overview(pool: &PgPool, id: &Uuid) -> Result<(i64, i64, i64), AppError> {
    maintenance_repo::dash_overview_row(pool, id)
}

pub fn request_panel(pool: &PgPool, id: &Uuid) -> Result<Vec<PanelRequests>, AppError> {
    let panel_row = maintenance_repo::find_panel_row(pool, id)?;

    Ok(panel_row
        .into_iter()
        .map(|p| PanelRequests {
            id: p.id,
            desc: p.desc,
            unit: p.unit,
            status: p.status,
            timestamp: p.created_at,
        })
        .collect())
}

pub fn to_inprogress(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    maintenance_repo::pending_inprogress(pool, id)
}

pub fn to_resolved(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
    maintenance_repo::inprogress_resolved(pool, id)
}
