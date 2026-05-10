use core::fmt;
use std::str::FromStr;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use uuid::Uuid;

use thiserror::Error;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub number: String,
    pub role: Role,
    pub password: Password,
}

impl User {
    pub fn new(
        id: Id,
        name: String,
        email: String,
        number: String,
        role: &str,
        password: String,
    ) -> Result<User, AppError> {
        Ok(Self {
            id,
            name,
            email,
            number,
            role: role
                .parse()
                .map_err(|_| AppError::BadRequest("invalid role".into()))?,
            password: Password::try_from(password)?,
        })
    }

    pub fn from_row(
        id: Uuid,
        name: String,
        email: String,
        number: String,
        role: &str,
        hash: String,
    ) -> Result<User, AppError> {
        Ok(Self {
            id: Id::from(id),
            name,
            email,
            number,
            role: role
                .parse()
                .map_err(|_| AppError::BadRequest("invalid role".into()))?,
            password: Password::from_hash(hash),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Role {
    Tenant,
    Caretaker,
    Landlord,
    Admin,
}

impl FromStr for Role {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "landlord" => Ok(Role::Landlord),
            "caretaker" => Ok(Role::Caretaker),
            "tenant" => Ok(Role::Tenant),
            "admin" => Ok(Role::Admin),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Role::Landlord => "landlord",
            Role::Caretaker => "caretaker",
            Role::Tenant => "tenant",
            Role::Admin => "admin",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
pub struct Id(Uuid);

impl Id {
    pub fn id() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Failed to hash password")]
    Hashing(#[from] argon2::password_hash::Error),
}

impl Password {
    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn verify(&self, plain: &str) -> bool {
        PasswordHash::new(&self.0)
            .map(|h| {
                Argon2::default()
                    .verify_password(plain.as_bytes(), &h)
                    .is_ok()
            })
            .unwrap_or(false)
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let hash = argon2.hash_password(value.as_bytes(), &salt)?.to_string();

        Ok(Self(hash))
    }
}

pub struct RoleCount {
    pub landlords: i64,
    pub tenants: i64,
    pub caretakers: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn password_hashes_on_construction() {
        let plain = "plaintext".to_string();

        let hashed = Password::try_from(plain.clone()).expect("hashing failed");

        assert_ne!(
            hashed.value(),
            plain,
            "hashed text and plain text should no match"
        );
    }

    #[test]
    fn user_new_hashes_password() {
        let id = Id::id();
        let name = "First Last".to_string();
        let email = "first@last.com".to_string();
        let number = "0712345678".to_string();
        let role = "landlord";
        let password = "securepassword".to_string();

        let user = User::new(id, name, email.clone(), number, role, password.clone())
            .expect("user creation failed");

        assert_eq!(user.email, email);

        assert_ne!(user.password.value(), password, "password not hashed");
    }

    #[test]
    fn password_verifies_correctly() {
        let plain = "securepassword".to_string();
        let hashed = Password::try_from(plain.clone()).expect("hashing failed");

        assert!(
            hashed.verify(&plain),
            "verify returns false for correct password"
        )
    }
}
