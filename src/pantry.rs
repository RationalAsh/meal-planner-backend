use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pantry(pub Vec<PantryIngredient>);

impl Pantry {
    pub fn new(pantry: Vec<PantryIngredient>) -> Self {
        Pantry(pantry)
    }
}

impl<'r> Responder<'r, 'static> for Pantry {
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

/// Struct to represent the expiry and quantity
/// of an ingredient in the pantry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PantryIngredient {
    /// Name of the ingredient.
    pub name: String,
    /// Quantity of the ingredient in grams.
    pub quantity: f32,
    /// Expiry date of the ingredient in number of days.
    pub expiry: i32,
}

impl<'r> Responder<'r, 'static> for PantryIngredient {
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
