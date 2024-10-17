use chrono::{DateTime, Utc};
use mpb::{Ingredient, Ingredients, Nutrition, ServerInfo, CORS};
use polodb_core::{bson::doc, ClientCursor, CollectionT, Database};
use rocket::{http::Status, routes, serde::json::Json, State};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[rocket::get("/api/v1/test")]
fn index() -> ServerInfo {
    ServerInfo {
        ts: Utc::now().to_rfc3339(),
        version: "0.1.0".to_string(),
    }
}

#[rocket::launch]
fn rocket() -> _ {
    let db_path = "mpb.db";
    let db = Database::open_file(db_path).unwrap();

    initialize_db(&db);

    // Open a connection to the database.
    let db = Arc::new(Mutex::new(db));

    // Initialize the database with some initial data.

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_ingredients])
        .mount("/", routes![add_ingredient])
        .mount("/", routes![get_pantry])
        .mount("/", routes![get_dishes])
        .manage(db)
        .attach(CORS)
}

#[rocket::get("/api/v1/dishes")]
fn get_dishes() -> String {
    json!(["Dish 1", "Dish 2", "Dish 3"]).to_string()
}

#[rocket::get("/api/v1/pantry")]
fn get_pantry(db: &State<Arc<Mutex<Database>>>) -> String {
    let db = db.lock().unwrap();
    let pantry_collection = db.collection("pantry");
    let pantry = pantry_collection.find(doc! {}).run().unwrap();
    let pantry: Vec<Ingredient> = pantry.into_iter().map(|r| r.unwrap()).collect();
    json!(pantry).to_string()
}

#[rocket::get("/api/v1/ingredients")]
fn get_ingredients(db: &State<Arc<Mutex<Database>>>) -> Ingredients {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Ingredients" collection.
    let ingredients_collection = db.collection("ingredients");

    // Find all ingredients in the collection.
    let ingredients: ClientCursor<Ingredient> = ingredients_collection.find(doc! {}).run().unwrap();

    // Convert the ingredients to a vector.
    let ingredients: Vec<Ingredient> = ingredients.into_iter().map(|r| r.unwrap()).collect();

    Ingredients(ingredients)
}

#[rocket::post("/api/v1/ingredients", data = "<ingredient>")]
fn add_ingredient(
    db: &State<Arc<Mutex<Database>>>,
    ingredient: Json<Ingredient>,
) -> Result<Ingredient, Status> {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Ingredients" collection.
    let ingredients_collection = db.collection("ingredients");

    // Insert the ingredient into the collection.
    let ingredient = ingredient.into_inner();
    match ingredients_collection.insert_one(ingredient.clone()) {
        Ok(_) => Ok(ingredient),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Initialize database with some initial data.
fn initialize_db(db: &Database) {
    let ingredients = vec![
        Ingredient {
            name: "Egg".to_string(),
            serving_size: 100.0,
            nutrition: Nutrition {
                calories: 140.0,
                protein: 12.0,
                fat: 10.0,
            },
        },
        Ingredient {
            name: "Rice (Cooked)".to_string(),
            serving_size: 200.0,
            nutrition: Nutrition {
                calories: 230.0,
                protein: 5.0,
                fat: 1.0,
            },
        },
    ];

    // Get the "Ingredients" collection.
    let ingredients_collection = db.collection("ingredients");

    // Insert the ingredients into the collection.
    let _ = ingredients_collection.insert_many(ingredients);
}
