use chrono::{DateTime, Utc};
use mpb::{
    Catalog, Dish, Ingredient, Ingredients, Nutrition, Pantry, PantryIngredient, ServerInfo, CORS,
};
use polodb_core::{bson::doc, ClientCursor, Collection, CollectionT, Database};
use rocket::{http::Status, routes, serde::json::Json, State};
use serde_json::{json, Value};
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
        .mount("/", routes![add_pantry_ingredient])
        .mount("/", routes![clear_pantry])
        .mount("/", routes![get_dishes])
        .mount("/", routes![add_dish])
        .manage(db)
        .attach(CORS)
}

#[rocket::get("/api/v1/dishes")]
fn get_dishes(db: &State<Arc<Mutex<Database>>>) -> Catalog {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Dishes" collection.
    let dishes_collection = db.collection("dishes");

    // Find all dishes in the collection.
    let dishes: ClientCursor<Dish> = dishes_collection.find(doc! {}).run().unwrap();

    // Convert the dishes to a vector.
    let dishes: Vec<Dish> = dishes.into_iter().map(|r| r.unwrap()).collect();

    Catalog(dishes)
}

#[rocket::post("/api/v1/dishes", data = "<dish>")]
fn add_dish(db: &State<Arc<Mutex<Database>>>, dish: Json<Dish>) -> Result<Dish, Status> {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Dishes" collection.
    let dishes_collection = db.collection("dishes");

    // Insert the dish into the collection.
    let result = dishes_collection.insert_one(dish.into_inner()).unwrap();

    // Get the inserted dish.
    let dish = dishes_collection
        .find_one(doc! { "_id": result.inserted_id })
        .unwrap_or(None);

    // Return the inserted dish.
    match dish {
        Some(dish) => Ok(dish),
        None => Err(Status::InternalServerError),
    }
}

#[rocket::get("/api/v1/pantry")]
fn get_pantry(db: &State<Arc<Mutex<Database>>>) -> Pantry {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Pantry" collection.
    let pantry_collection = db.collection("pantry");

    // Find all pantry ingredients in the collection.
    let pantry_ingredients: ClientCursor<PantryIngredient> =
        pantry_collection.find(doc! {}).run().unwrap();

    // Convert the pantry ingredients to a vector.
    let pantry_ingredients: Vec<PantryIngredient> =
        pantry_ingredients.into_iter().map(|r| r.unwrap()).collect();

    Pantry(pantry_ingredients)
}

#[rocket::post("/api/v1/pantry", data = "<pantry_ingredient>")]
fn add_pantry_ingredient(
    db: &State<Arc<Mutex<Database>>>,
    pantry_ingredient: Json<PantryIngredient>,
) -> Result<PantryIngredient, Status> {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Pantry" collection.
    let pantry_collection = db.collection("pantry");

    // Insert the pantry ingredient into the collection.
    let pantry_ingredient = pantry_ingredient.into_inner();

    // Return the inserted pantry ingredient.
    match pantry_collection.insert_one(pantry_ingredient.clone()) {
        Ok(_) => Ok(pantry_ingredient),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::post("/api/v1/pantry/clearall")]
fn clear_pantry(db: &State<Arc<Mutex<Database>>>) -> Result<Json<Value>, Status> {
    // Acquire lock on the database.
    let db = db.lock().unwrap();

    // Get the "Pantry" collection.
    let pantry_collection: Collection<PantryIngredient> = db.collection("pantry");

    // Clear the pantry collection.
    let res = match pantry_collection.delete_many(doc! {}) {
        Ok(_) => Ok(Json(json!({
            "message": "Pantry cleared"
        }))),
        Err(_) => Err(Status::InternalServerError),
    };

    res
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
