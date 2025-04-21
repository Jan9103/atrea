#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

pub mod ai_training_data;
pub mod api_channel;
pub mod api_raids;
pub mod api_recs;
pub mod frontend;
//pub mod recommender;

#[derive(Database)]
#[database("atrea_db")]
pub struct AtreaDb(rocket_db_pools::sqlx::SqlitePool);

/// escape chars within a string. DOES NOT ADD QUOTES!
pub fn json_escape_string(text: &str) -> String {
    text.replace("\\", "\\\\").as_str().replace("\"", "\\\"")
}
