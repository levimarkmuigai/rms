use uuid::Uuid;

use crate::{
    db::PgPool,
    entities::maintenance::{MaintenanceRequest, RequestStatus, RequestWithLabel},
    error::AppError,
};

pub fn open_for_landlord(
    pool: &PgPool,
    landlord_id: &Uuid,
) -> Result<Vec<MaintenanceRequest>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT
        mr.id,
        mr.unit_id,
        mr.category,
        mr.status EXTRACT(DAY FROM NOW() - mr.created_at)::INT AS age_days
        FROM maintenance_requests mr
        JOIN units u ON u.id = mr.unit_id
        JOIN buildings b ON b.id = u.building_id
        WHERE b.landlord_id = $1
        AND mr.status != 'resolved'
        ORDER BY mr.created_at ASC
        LIMIT 10",
        &[landlord_id],
    )?;

    Ok(rows
        .iter()
        .map(|r| {
            let status_str: String = r.get("status");
            MaintenanceRequest {
                id: r.get("id"),
                unit_id: r.get("unit_id"),
                category: r.get("category"),
                status: status_str.parse().unwrap_or(RequestStatus::Pending),
                age_days: r.get::<_, i32>("age_days"),
            }
        })
        .collect())
}

pub fn open_for_landlord_with_label(
    pool: &PgPool,
    landlord_id: &Uuid,
) -> Result<Vec<RequestWithLabel>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT
        mr.category,
        b.name || '-' || u.unit_number AS unit_label,
        mr.status,
        EXtract(DAY FROM NOW() - mr.created_at)::INT AS age_days
        FROM maintenance_requests mr
        JOIN units u ON u.id = mr.unit_id
        JOIN buildings b ON b.id = u.building_id
        WHERE b.landlord_id = $1
        AND mr.status != 'resolved'
        ORDER BY mr.created_at ASC
        LIMIT 10",
        &[landlord_id],
    )?;

    Ok(rows
        .iter()
        .map(|r| {
            let status_str: String = r.get("status");
            RequestWithLabel {
                category: r.get("category"),
                unit_label: r.get("unit_label"),
                status: status_str
                    .parse()
                    .unwrap_or(RequestStatus::Pending)
                    .to_string(),
                age_days: r.get::<_, i32>("age_days"),
            }
        })
        .collect())
}
