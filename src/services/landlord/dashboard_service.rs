use uuid::Uuid;

use crate::{
    db::PgPool,
    error::AppError,
    repositories::{building_repo, maintenance_repo},
};

pub struct PortfolioSummary {
    pub has_buildings: bool,
    pub collected_revenue: i64,
    pub expected_revenue: i64,
    pub occupancy_pct: i64,
    pub vacant_units: i64,
    pub total_arrears: i64,
    pub arrears_tenants: i64,
}

pub struct OpenRequest {
    pub category: String,
    pub unit_label: String,
    pub status: String,
    pub age_label: String,
}

pub struct BuildingCard {
    pub id: Uuid,
    pub name: String,
    pub total_units: i64,
    pub is_occupied: i64,
    pub vacant: i64,
    pub collected: i64,
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
            occupancy_pct: 0,
            vacant_units: 0,
            total_arrears: 0,
            arrears_tenants: 0,
        });
    }

    let collected = building_repo::collected_this_month(pool, landlord_id, month_year)?;
    let (arrears, arr_tenants) = building_repo::arrears_stats(pool, landlord_id, month_year)?;
    let occupancy_pct = if total > 0 { occupied * 100 / total } else { 0 };

    Ok(PortfolioSummary {
        has_buildings: true,
        collected_revenue: collected,
        expected_revenue: expected,
        occupancy_pct,
        vacant_units: vacant,
        total_arrears: arrears,
        arrears_tenants: arr_tenants,
    })
}

pub fn open_requests(pool: &PgPool, landlord_id: &Uuid) -> Result<Vec<OpenRequest>, AppError> {
    let open_requests = maintenance_repo::open_for_landlord_with_label(pool, landlord_id)?;
    Ok(open_requests
        .into_iter()
        .map(|op| OpenRequest {
            category: op.category,
            unit_label: op.unit_label,
            status: op.status,
            age_label: match op.age_days {
                0 => "today".into(),
                1 => "1 day".into(),
                d => format!("{d} days"),
            },
        })
        .collect())
}

pub fn building_cards(
    pool: &PgPool,
    landlord_id: &Uuid,
    month_year: &str,
) -> Result<Vec<BuildingCard>, AppError> {
    let building_cards = building_repo::building_summeries(pool, landlord_id, month_year)?;

    Ok(building_cards
        .into_iter()
        .map(|bc| BuildingCard {
            id: bc.id,
            name: bc.name,
            total_units: bc.total_units,
            is_occupied: bc.occupied,
            vacant: bc.total_units - bc.occupied,
            collected: bc.collected,
        })
        .collect())
}

