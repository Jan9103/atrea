use crate::AtreaDb;

use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket_db_pools::{sqlx, sqlx::sqlite::SqliteRow, sqlx::Row};

#[get("/api/shoutouts/from/<author>/to/<target>?<offset>&<limit>")]
pub async fn shoutouts_from_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    author: &str,
    target: &str,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('timestamp', unixepoch(timestamp), 'author', author, 'target', target) FROM shoutouts WHERE author == ? AND target == ? OFFSET ? LIMIT ?")
        .bind(author)
        .bind(target)
        .bind(offset.unwrap_or(0))
        .bind(limit.unwrap_or(20))
        .fetch_all(&mut **db)
        .await {
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

#[get("/api/shoutouts/from/<author>?<offset>&<limit>")]
pub async fn shoutouts_from(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    author: &str,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('timestamp', unixepoch(timestamp), 'author', author, 'target', target) FROM shoutouts WHERE author == ? OFFSET ? LIMIT ?")
        .bind(author)
        .bind(offset.unwrap_or(0))
        .bind(limit.unwrap_or(20))
        .fetch_all(&mut **db)
        .await {
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

#[get("/api/shoutouts/to/<target>?<offset>&<limit>")]
pub async fn shoutouts_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    target: &str,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('timestamp', unixepoch(timestamp), 'author', author, 'target', target) FROM shoutouts WHERE target == ? OFFSET ? LIMIT ?")
        .bind(target)
        .bind(offset.unwrap_or(0))
        .bind(limit.unwrap_or(20))
        .fetch_all(&mut **db)
        .await {
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

#[get("/api/shoutouts/from/<author>/to/<target>/stats?<offset>&<limit>")]
pub async fn shoutoutstats_from_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    author: &str,
    target: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('author', author, 'target', target, 'shoutout_count', shoutout_count) FROM shoutout_connections rc WHERE author == ? AND target == ? ORDER BY rc.shoutout_count DESC LIMIT ? OFFSET ?")
        .bind(author)
        .bind(target)
        .bind(limit.unwrap_or(20))
        .bind(offset.unwrap_or(0))
        .fetch_all(&mut **db)
        .await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    Ok(RawJson(match res.first() {
        Some(r) => r.get(0),
        None => format!(
            r#"{{"author":"{}","target":"{}","shoutout_count":0,}}"#,
            crate::json_escape_string(author),
            crate::json_escape_string(target),
        ),
    }))
}

#[get("/api/shoutouts/from/<author>/stats?<limit>&<offset>")]
pub async fn shoutoutstats_from(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    author: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('author', author, 'target', target, 'shoutout_count', shoutout_count) FROM shoutout_connections rc WHERE author == ? ORDER BY rc.shoutout_count DESC LIMIT ? OFFSET ?")
        .bind(author)
        .bind(limit.unwrap_or(20))
        .bind(offset.unwrap_or(0))
        .fetch_all(&mut **db)
        .await {
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

#[get("/api/shoutouts/to/<target>/stats?<limit>&<offset>")]
pub async fn shoutoutstats_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    target: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('author', author, 'target', target, 'shoutout_count', shoutout_count) FROM shoutout_connections rc WHERE target == ? ORDER BY rc.shoutout_count DESC LIMIT ? OFFSET ?")
        .bind(target)
        .bind(limit.unwrap_or(20))
        .bind(offset.unwrap_or(0))
        .fetch_all(&mut **db)
        .await {
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
