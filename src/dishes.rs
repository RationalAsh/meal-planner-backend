use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Catalog(pub Vec<Dish>);

impl Catalog {
    pub fn new(dishes: Vec<Dish>) -> Self {
        Catalog(dishes)
    }
}

impl<'r> Responder<'r, 'static> for Catalog {
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

/// Struct to represent a dish
/// with its name and ingredients.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dish {
    /// Name of the dish.
    pub name: String,
    /// Ingredients required to make the dish.
    pub ingredients: Vec<String>,
    /// Tags associated with the dish.
    pub tags: Vec<String>,
}

impl<'r> Responder<'r, 'static> for Dish {
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
