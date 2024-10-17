use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Nutrition {
    /// Calories in kcal.
    pub calories: f32,
    /// Protein in grams.
    pub protein: f32,
    /// Fat in grams.
    pub fat: f32,
}
