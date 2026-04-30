use uuid::Uuid;

use crate::{
    db::PgPool,
    entities::user::{Id, User},
    error::AppError,
    repositories::user_repo,
};

pub fn signup(
    pool: &PgPool,
    name: String,
    email: String,
    number: String,
    role: &str,
    password: String,
) -> Result<(), AppError> {
    if user_repo::email_exists(pool, &email)? {
        return Err(AppError::BadRequest("email already registered".into()));
    }

    let user = User::new(Id::id(), name, email, number, role, password)?;

    user_repo::insert(pool, &user)
}

pub fn authenticate(pool: &PgPool, email: &str, password: &str) -> Result<User, AppError> {
    let user = user_repo::find_by_email(pool, &email)?.ok_or_else(|| {
        tracing::warn!(email, "failed login attempt");
        AppError::Unauthorized
    })?;

    if !user.password.verify(password) {
        tracing::warn!(%email, "login: wrong password");
        return Err(AppError::Unauthorized);
    }

    tracing::info!(%email, "login success");

    Ok(user)
}

pub fn get(pool: &PgPool, id: &Uuid) -> Result<User, AppError> {
    user_repo::find_by_id(pool, id)?.ok_or_else(|| AppError::NotFound(id.to_string()))
}

pub fn update(
    pool: &PgPool,
    id: &Uuid,
    name: String,
    email: String,
    number: String,
) -> Result<(), AppError> {
    if name.is_empty() || email.is_empty() {
        return Err(AppError::BadRequest("name and email required".into()));
    }

    user_repo::update(pool, id, &name, &email, &number)
}
