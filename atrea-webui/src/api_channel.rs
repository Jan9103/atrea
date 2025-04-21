use crate::AtreaDb;

use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;

#[get("/api/channel/<channel>/twitch_info")]
pub async fn get_twitch_info(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    channel: &str,
) -> Result<RawJson<String>, Status> {
    match sqlx::query("SELECT json_object('created_at', unixepoch(created_at), 'id', id, 'login', login, 'display_name', display_name, 'broadcaster_type', broadcaster_type, 'description', description, 'profile_image_url', profile_image_url) FROM channel_info WHERE login == ?")
        .bind(channel)
        .fetch_all(&mut **db)
        .await {
        Ok(res) => match res.first() {
            Some(res) => Ok(RawJson(res.get(0))),
            None => Err(Status::NotFound),
        },
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/channel/<channel>/image")]
pub async fn redirect_image(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    channel: &str,
) -> Result<rocket::response::Redirect, Status> {
    match sqlx::query("SELECT profile_image_url FROM channel_info WHERE login == ?")
        .bind(channel)
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => match res.first() {
            Some(res) => {
                let image_url: String = res.get(0);
                if image_url.trim().is_empty() {
                    Err(Status::NotFound)
                } else {
                    Ok(rocket::response::Redirect::to(image_url))
                }
            }
            None => Err(Status::NotFound),
        },
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}
