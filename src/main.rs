use chrono::{DateTime, Utc};
use mpb::{Ingredient, Ingredients, Nutrition, ServerInfo, CORS};
use polodb_core::{bson::doc, ClientCursor, CollectionT, Database};
use rocket::{routes, State};
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
        .manage(db)
        .attach(CORS)
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
    ingredients_collection.insert_many(ingredients);
}
