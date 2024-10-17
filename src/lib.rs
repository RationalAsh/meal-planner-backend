use chrono::{DateTime, Utc};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    response::Responder,
    Request, Response,
};
use serde::{Deserialize, Serialize};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    pub ts: String,
    pub version: String,
}

impl<'r> Responder<'r, 'static> for ServerInfo {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingredients(pub Vec<Ingredient>);

impl Ingredients {
    pub fn new(ingredients: Vec<Ingredient>) -> Self {
        Ingredients(ingredients)
    }
}

impl<'r> Responder<'r, 'static> for Ingredients {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingredient {
    /// Name of the ingredient.
    pub name: String,
    /// Serving size in grams.
    pub serving_size: f32,
    /// Nutrition information per serving.
    pub nutrition: Nutrition,
}

impl<'r> Responder<'r, 'static> for Ingredient {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nutrition {
    /// Calories in kcal.
    pub calories: f32,
    /// Protein in grams.
    pub protein: f32,
    /// Fat in grams.
    pub fat: f32,
}
