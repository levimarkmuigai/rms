use uuid::Uuid;

use crate::{db::PgPool, error::AppError, repositories::building_repo};

pub struct PortfolioSummary {
    pub has_buildings: bool,
    pub collected_revenue: i32,
    pub expected_revenue: i32,
    pub occupied: i64,
    pub vacant_units: i64,
    pub total_arrears: i32,
    pub arrears_tenants: i64,
}

pub struct BuildingOverview {
    pub name: String,
    pub cartaker_id: Option<Uuid>,
    pub caretaker_name: Option<String>,
    pub caretaker_number: Option<String>,
    pub requests: Option<i64>,
}

pub fn portfolio_summary(
    pool: &PgPool,
    landlord_id: &Uuid,
    month_year: &str,
) -> Result<PortfolioSummary, AppError> {
    let (total, occupied, vacant, expected) =
        building_repo::portfolio_unit_stats(pool, landlord_id)?;

    if total == 0 {
        return Ok(PortfolioSummary {
            has_buildings: false,
            collected_revenue: 0,
            expected_revenue: 0,
            occupied: 0,
            vacant_units: 0,
            total_arrears: 0,
            arrears_tenants: 0,
        });
    }

    let collected = building_repo::collected_this_month(pool, landlord_id, month_year)?;
    let (arrears, arr_tenants) = building_repo::arrears_stats(pool, landlord_id, month_year)?;

    Ok(PortfolioSummary {
        has_buildings: true,
        collected_revenue: collected,
        expected_revenue: expected,
        occupied,
        vacant_units: vacant,
        total_arrears: arrears,
        arrears_tenants: arr_tenants,
    })
}

pub fn building_overview(
    pool: &PgPool,
    landlord_id: &Uuid,
) -> Result<Vec<BuildingOverview>, AppError> {
    let overview = building_repo::buildings_overview_rows(pool, landlord_id)?;

    Ok(overview
        .into_iter()
        .map(|o| BuildingOverview {
            name: o.name,
            cartaker_id: o.caretaker_id,
            caretaker_name: o.caretaker_name,
            caretaker_number: o.caretaker_number,
            requests: o.requests,
        })
        .collect())
}
