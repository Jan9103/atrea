use std::borrow::Borrow;

use crate::AtreaDb;
use rocket::{futures::FutureExt, http::Status, response::content::RawJson};
use rocket_db_pools::sqlx::{self, sqlite::SqliteRow, Row, SqliteConnection};

pub struct RecommendedChannel {
    name: String,
    reasons: Vec<(String, usize)>,
    score: usize,
}

#[get("/api/<channel>/a")]
pub async fn overview(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    channel: &str,
) -> Result<RawJson<String>, Status> {
    let conn: &mut SqliteConnection = &mut **db;
}

pub async fn recommend(
    conn: &mut SqliteConnection,
) -> Result<Vec<RecommendedChannel>, sqlx::Error> {
    let mut recommended_channels: Vec<RecommendedChannel> = Vec::new();

    let liked_channels: Vec<String> = match sqlx::query("SELECT name FROM liked_channels")
        .fetch_all(&mut *conn)
        .await
    {
        Ok(res) => res.iter().map(|r| r.get(0)).collect::<Vec<String>>(),
        Err(err) => {
            return Err(err);
        }
    };
    for liked_channel in liked_channels {
        let channel_recommendations: Vec<RecommendedChannel> =
            recommend_for(liked_channel.as_str(), conn)?;
    }

    // Ok(recommended_channels)
    todo!()
}

pub fn recommend_for(
    channel: &str,
    conn: &mut SqliteConnection,
) -> Result<Vec<RecommendedChannel>, sqlx::Error> {
    let mut recommended_channels: Vec<RecommendedChannel> = Vec::new();

    // Ok(recommended_channels)
    todo!()
}
