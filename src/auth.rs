use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Structure to hold claims for JWT.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    /// Expires at time.
    pub expires_at: usize,
    /// Issued at time.
    pub issued_at: usize,
    /// Email
    pub email: String,
    /// Hashed password
    pub hashed_password: String,
}

/// Struct to represent data sent during login.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    /// Email of the user.
    pub email: String,
    /// Password of the user.
    pub hashed_password: String,
}

/// Function to generate a JWT token.
pub fn generate_jwt(email: String, hashed_password: String) -> String {
    // Get the secret key from the environment.
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Get the expiry and issued time.
    let now = Utc::now();
    let expires_at = now
        .checked_add_signed(Duration::seconds(300))
        .unwrap()
        .timestamp();

    let header = Header::new(Algorithm::HS512);
    let claims = Claims {
        expires_at: expires_at as usize,
        issued_at: now.timestamp() as usize,
        email,
        hashed_password,
    };
    let key = EncodingKey::from_secret(secret.as_ref());
    encode(&header, &claims, &key).unwrap()
}
