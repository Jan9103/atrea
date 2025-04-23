use rocket::{http::Status, response::content::RawJson};
use rocket_db_pools::sqlx::{self, sqlite::SqliteRow, Row};

use crate::AtreaDb;

#[get("/api/viewers/known_viewers?<offset>&<limit>")]
pub async fn get_known_viewers(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query(include_str!("sql/viewers_you_know_global.sql"))
        .bind(limit.unwrap_or(20))
        .bind(offset.unwrap_or(0))
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    Ok(RawJson(format!(
        "[{}]",
        res.into_iter()
            .map(|r| r.get(0))
            .collect::<Vec<String>>()
            .join(",")
    )))
}
