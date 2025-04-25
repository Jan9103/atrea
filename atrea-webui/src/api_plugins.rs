use rocket::{
    http::Status,
    response::content::{self, RawJson},
};
use rocket_db_pools::sqlx::{self, Row};
use sqlx::sqlite::SqliteRow;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::AtreaSettingsDb;

#[get("/api/plugins/get_css/<plugin_name>/<css_name>")]
pub async fn get_css(plugin_name: &str, css_name: &str) -> Result<content::RawCss<String>, Status> {
    if !is_valid_filename(plugin_name) || !is_valid_filename(css_name) {
        return Err(Status::BadRequest);
    }
    let file_path: PathBuf = PathBuf::from("plugins").join(plugin_name).join(css_name);
    if !file_path.is_file() {
        return Err(Status::NotFound);
    }
    match fs::read_to_string(file_path) {
        Ok(v) => Ok(content::RawCss(v)),
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/plugins/get_js/<plugin_name>/<js_name>")]
pub async fn get_js(
    plugin_name: &str,
    js_name: &str,
) -> Result<content::RawJavaScript<String>, Status> {
    if !is_valid_filename(plugin_name) || !is_valid_filename(js_name) {
        return Err(Status::BadRequest);
    }
    let file_path: PathBuf = PathBuf::from("plugins").join(plugin_name).join(js_name);
    if !file_path.is_file() {
        return Err(Status::NotFound);
    }
    match fs::read_to_string(file_path) {
        Ok(v) => Ok(content::RawJavaScript(v)),
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/plugins/enable/<plugin_name>")]
pub async fn enable_plugin(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
    plugin_name: String,
) -> Result<Status, Status> {
    if let Err(err) = sqlx::query("UPDATE plugins SET enabled = 1 WHERE plugin_name == ?;")
        .bind(plugin_name)
        .execute(&mut **db)
        .await
    {
        eprintln!("{}", err);
        return Err(Status::InternalServerError);
    };
    Ok(Status::Ok)
}

#[get("/api/plugins/disable/<plugin_name>")]
pub async fn disable_plugin(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
    plugin_name: String,
) -> Result<Status, Status> {
    if let Err(err) = sqlx::query("UPDATE plugins SET enabled = 0 WHERE plugin_name == ?;")
        .bind(plugin_name)
        .execute(&mut **db)
        .await
    {
        eprintln!("{}", err);
        return Err(Status::InternalServerError);
    };
    Ok(Status::Ok)
}

#[get("/api/plugins/list")]
pub async fn list(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query(
        r#"
        SELECT JSON_OBJECT(
            'plugin_name', plugin_name,
            'description', description,
            'enabled', enabled
        ) FROM plugins;
        "#,
    )
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

#[get("/api/plugins/update_db")]
pub async fn update_db(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<Status, Status> {
    let enabled_plugins: Vec<String> =
        match sqlx::query("SELECT plugin_name FROM plugins WHERE enabled == TRUE;")
            .fetch_all(&mut **db)
            .await
        {
            Ok(res) => res
                .into_iter()
                .map(|plugin| -> String { plugin.get(0) })
                .collect::<Vec<String>>(),
            Err(err) => {
                eprintln!("{}", err);
                return Err(Status::InternalServerError);
            }
        };
    // empty db
    if let Err(err) = sqlx::query("DELETE FROM plugins; DELETE FROM plugin_files;")
        .execute(&mut **db)
        .await
    {
        eprintln!("{}", err);
        return Err(Status::InternalServerError);
    };

    match fs::read_dir("plugins") {
        Ok(lsr) => {
            for lsi in lsr.into_iter() {
                match lsi {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(ft) => {
                            if ft.is_dir() {
                                register_plugin(&mut db, &dir_entry.path(), &enabled_plugins)
                                    .await?;
                            }
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            return Err(Status::InternalServerError);
                        }
                    },
                    Err(err) => {
                        eprintln!("{}", err);
                        return Err(Status::InternalServerError);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    }

    Ok(Status::Ok)
}

async fn register_plugin(
    db: &mut sqlx::SqliteConnection,
    plugin_dir: &Path,
    enabled_plugins: &[String],
) -> Result<(), Status> {
    let plugin_name: &str = plugin_dir
        .file_name()
        .expect("read_dir generated a invalid path")
        .to_str()
        .expect("Path is not a valid string");

    // read meta.json
    let json_file_path: PathBuf = plugin_dir.join("meta.json");
    if !json_file_path.is_file() {
        return Ok(());
    }
    match fs::read_to_string(&json_file_path) {
        Ok(raw_json_string) => {
            match serde_json::from_str(&raw_json_string) {
                Ok(serde_json::Value::Object(json_root)) => {
                    let description: &String = match json_root.get("description") {
                        Some(serde_json::Value::String(v)) => v,
                        Some(_) => {
                            eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (invalid description)");
                            return Err(Status::InternalServerError);
                        }
                        None => &String::from(""),
                    };
                    if let Err(err) = sqlx::query("INSERT INTO plugins ( plugin_name, description, enabled ) VALUES ( ?, ?, ? )")
                        .bind(plugin_name)
                        .bind(description)
                        .bind(enabled_plugins.contains(&String::from(plugin_name)))
                        .execute(&mut *db)
                        .await
                    {
                        eprintln!("{}", err);
                        return Err(Status::InternalServerError);
                    };
                }
                Ok(_) => {
                    eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?}");
                    return Err(Status::InternalServerError);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    return Err(Status::InternalServerError);
                }
            };
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };

    // plugin_files
    match fs::read_dir(plugin_dir) {
        Ok(lsr) => {
            for lsi in lsr.into_iter() {
                match lsi {
                    Ok(dir_entry) => {
                        let dir_entry_path: PathBuf = dir_entry.path();
                        if !dir_entry_path.is_file() {
                            continue;
                        }
                        let filename: &str = dir_entry_path
                            .file_name()
                            .expect("read_dir generated a invalid path")
                            .to_str()
                            .expect("Path is not a valid string");
                        if filename == "meta.json" {
                            continue;
                        }
                        if let Err(err) = sqlx::query(
                            "INSERT INTO plugin_files ( plugin_name, file_name ) VALUES ( ?, ? )",
                        )
                        .bind(plugin_name)
                        .bind(filename)
                        .execute(&mut *db)
                        .await
                        {
                            eprintln!("{}", err);
                            return Err(Status::InternalServerError);
                        };
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        return Err(Status::InternalServerError);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    }
    Ok(())
}

fn is_valid_filename(name: &str) -> bool {
    !name.chars().all(|i| i == '.')
        && name.chars().all(|i| {
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-.".contains(i)
        })
}
