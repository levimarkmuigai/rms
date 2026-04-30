use uuid::Uuid;

use crate::{db::PgPool, entities::user::User, error::AppError};

pub fn insert(pool: &PgPool, user: &User) -> Result<(), AppError> {
    let mut client = pool.get()?;

    client.execute(
        "INSERT INTO users (id, name, email, number, role, password) 
        VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            user.id.value(),
            &user.name,
            &user.email,
            &user.number,
            &user.role.to_string(),
            &user.password.value(),
        ],
    )?;

    tracing::debug!(email = %user.email, "user inserted");
    Ok(())
}

pub fn update(
    pool: &PgPool,
    id: &Uuid,
    name: &str,
    email: &str,
    number: &str,
) -> Result<(), AppError> {
    let mut client = pool.get()?;
    client.execute(
        "UPDATE users SET name = $1, email = $2, number = $3 WHERE id = $4",
        &[&name, &email, &number, &id],
    )?;
    tracing::debug!(%id, "user profile updated");
    Ok(())
}

pub fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query("SELECT * FROM users WHERE email = $1", &[&email])?;
    rows.first().map(row_to_user).transpose()
}

pub fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<User>, AppError> {
    let mut client = pool.get()?;
    let rows = client.query("SELECT * FROM users WHERE id = $1", &[&id])?;
    rows.first().map(row_to_user).transpose()
}

pub fn email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
    let mut client = pool.get()?;
    let rows = client.query("SELECT 1 FROM users WHERE email = $1 LIMIT  1", &[&email])?;

    Ok(!rows.is_empty())
}

fn row_to_user(row: &postgres::Row) -> Result<User, AppError> {
    User::from_row(
        row.get("id"),
        row.get("name"),
        row.get("email"),
        row.get("number"),
        row.get("role"),
        row.get("password"),
    )
}
