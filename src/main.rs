use mpb::{Ingredient, Ingredients, Nutrition};
use polodb_core::Database;
use rocket::routes;
use serde_json::json;

#[rocket::get("/api/v1/test")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> _ {
    let db_path = "mpb.db";
    // Open a connection to the database.
    let db = Database::open_file(db_path).unwrap();

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_ingredients])
}

#[rocket::get("/api/v1/ingredients")]
fn get_ingredients() -> Ingredients {
    let ingredients = vec![
        Ingredient {
            name: "Apple".to_string(),
            serving_size: 100.0,
            nutrition: Nutrition {
                calories: 52.0,
                protein: 0.3,
                fat: 0.2,
            },
        },
        Ingredient {
            name: "Banana".to_string(),
            serving_size: 100.0,
            nutrition: Nutrition {
                calories: 89.0,
                protein: 1.1,
                fat: 0.3,
            },
        },
    ];
    Ingredients(ingredients)
}
