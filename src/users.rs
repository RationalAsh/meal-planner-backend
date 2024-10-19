use rocket::response::Responder;
use serde::{Deserialize, Serialize};

/// Struct to represent user information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Name of the user
    pub first_name: String,
    /// Last name of the user.
    pub last_name: String,
    /// Email of the user.
    pub email: String,
    /// Password of the user.
    pub hashed_password: String,
}

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(
                serde_json::to_string(&self).unwrap().len(),
                std::io::Cursor::new(serde_json::to_string(&self).unwrap()),
            )
            .ok()
    }
}
