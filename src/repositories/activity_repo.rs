use uuid::Uuid;

use crate::{db::PgPool, entities::activity::ActivityLog, error::AppError};

pub fn insert(pool: &PgPool, user_id: &Uuid, action: &str) -> Result<(), AppError> {
    let mut client = pool.get()?;
    client.execute(
        "INSERT INTO activity_logs (user_id, action) VALUES ($1, $2)",
        &[user_id, &action],
    )?;
    Ok(())
}

pub fn find_all(pool: &PgPool) -> Result<Vec<ActivityLog>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query(
        "SELECT l.log_id, l.action, l.created_at, u.email
        FROM activity_logs l
        JOIN users u ON u.id = l.user_id
        ORDER BY l.created_at DESC",
        &[],
    )?;
    Ok(rows
        .iter()
        .map(|r| ActivityLog {
            log_id: r.get("log_id"),
            action: r.get("action"),
            created_at: r.get("created_at"),
            email: r.get("email"),
        })
        .collect())
}
