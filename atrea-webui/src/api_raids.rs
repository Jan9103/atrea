use crate::AtreaDb;

use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket_db_pools::{sqlx, sqlx::sqlite::SqliteRow, sqlx::Row};

#[get("/api/raids/from/<raider>/to/<target>")]
pub async fn raids_from_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    raider: &str,
    target: &str,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('timestamp', unixepoch(timestamp), 'raider', raider, 'target', target, 'size', size) FROM raids WHERE raider == ? AND target == ?")
        .bind(raider)
        .bind(target)
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

#[get("/api/raids/from/<raider>")]
pub async fn raids_from(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    raider: &str,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('timestamp', unixepoch(timestamp), 'raider', raider, 'target', target, 'size', size) FROM raids WHERE raider == ?")
        .bind(raider)
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

#[get("/api/raids/to/<target>")]
pub async fn raids_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    target: &str,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('timestamp', unixepoch(timestamp), 'raider', raider, 'target', target, 'size', size) FROM raids WHERE target == ?")
        .bind(target)
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

#[get("/api/raids/from/<raider>/to/<target>/stats?<offset>&<limit>")]
pub async fn raidstats_from_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    raider: &str,
    target: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('raider', raider, 'target', target, 'total_viewers', total_viewers, 'raid_count', raid_count, 'average_raid_size', average_raid_size) FROM raid_connections rc WHERE raider == ? AND target == ? ORDER BY rc.raid_count DESC LIMIT ? OFFSET ?")
        .bind(raider)
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
            r#"{{"raider":"{}","target":"{}","total_viewers":0,"raid_count":0,"average_raid_size":0}}"#,
            crate::json_escape_string(raider),
            crate::json_escape_string(target),
        ),
    }))
}

#[get("/api/raids/from/<raider>/stats?<limit>&<offset>")]
pub async fn raidstats_from(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    raider: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('raider', raider, 'target', target, 'total_viewers', total_viewers, 'raid_count', raid_count, 'average_raid_size', average_raid_size) FROM raid_connections rc WHERE raider == ? ORDER BY rc.raid_count DESC LIMIT ? OFFSET ?")
        .bind(raider)
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

#[get("/api/raids/to/<target>/stats?<limit>&<offset>")]
pub async fn raidstats_to(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    target: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query("SELECT json_object('raider', raider, 'target', target, 'total_viewers', total_viewers, 'raid_count', raid_count, 'average_raid_size', average_raid_size) FROM raid_connections rc WHERE target == ? ORDER BY rc.raid_count DESC LIMIT ? OFFSET ?")
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
