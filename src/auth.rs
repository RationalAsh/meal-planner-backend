use serde::{Deserialize, Serialize};

/// Structure to hold claims for JWT.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    /// Expires at time.
    pub exp: usize,
    /// Issued at time.
    pub iat: usize,
    /// Email
    pub email: String,
}

/// Struct to represent data sent during login.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    /// Email of the user.
    pub email: String,
    /// Password of the user.
    pub hashed_password: String,
}
