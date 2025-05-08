use rocket::{
    http::Status,
    response::content::{self, RawJson},
};
use rocket_db_pools::sqlx::{self, Row};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{json_escape_string, AtreaSettingsDb};

#[get("/get_css_plugin?<plugin_name>&<css_name>")]
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
#[get("/get_js_plugin?<plugin_name>&<js_name>")]
pub async fn get_js(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
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
    let mut js_text: String = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    match sqlx::query(
        "SELECT setting_key, setting_value FROM plugin_settings WHERE plugin_name == ?;",
    )
    .bind(plugin_name)
    .fetch_all(&mut **db)
    .await
    {
        Ok(res) => {
            for setting in res.into_iter() {
                let setting_key: &str = setting.get(0);
                let setting_value: &str = setting.get(1);
                js_text = js_text.replace(
                    format!("@!{}!@", setting_key).as_str(),
                    crate::json_escape_string(setting_value).as_str(),
                );
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    Ok(content::RawJavaScript(js_text))
}

#[get("/api/plugins/enable/<plugin_name>")]
pub async fn enable_plugin(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
    plugin_name: &str,
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
    plugin_name: &str,
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

#[get("/api/plugins/update_setting/<plugin_name>/<setting_key>/<setting_value>")]
pub async fn update_setting(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
    plugin_name: &str,
    setting_key: &str,
    setting_value: &str,
) -> Result<Status, Status> {
    if let Err(err) = sqlx::query(
        "UPDATE plugin_settings SET setting_value = ? WHERE plugin_name == ? AND setting_key == ?;",
    )
    .bind(setting_value)
    .bind(plugin_name)
    .bind(setting_key)
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
    match sqlx::query(
        "SELECT plugin_name, description, enabled, category FROM plugins ORDER BY plugin_name ASC;",
    )
    .fetch_all(&mut **db)
    .await
    {
        Ok(res) => {
            let mut plugins: Vec<String> = Vec::with_capacity(res.len());
            for ir in res.into_iter() {
                let plugin_name: &str = ir.get(0);
                let description: &str = ir.get(1);
                let enabled: bool = ir.get(2);
                let category: &str = ir.get(3);
                let settings: String = match sqlx::query("SELECT setting_key, setting_value FROM plugin_settings WHERE plugin_name == ?;").bind(plugin_name).fetch_all(&mut **db).await {
                    Ok(v) => format!("{{{}}}", v.into_iter().map(|sir| -> Result<String, Status> {
                        let setting_key: &str = sir.get(0);
                        let setting_value: &str = sir.get(1);
                        Ok(format!("\"{}\":\"{}\"", json_escape_string(setting_key), json_escape_string(setting_value)))
                    }).collect::<Result<Vec<String>, Status>>()?.join(",")),
                    Err(err) => {
                        eprintln!("{}", err);
                        return Err(Status::InternalServerError);
                    }
                };
                plugins.push(format!(
                    r#"{{"plugin_name": "{}", "description": "{}", "enabled": {}, "settings": {}, "category": "{}"}}"#,
                    json_escape_string(plugin_name),
                    json_escape_string(description),
                    if enabled { "true" } else { "false" },
                    settings,
                    category,
                ));
            }

            Ok(RawJson(format!("[{}]", plugins.join(","))))
        }
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
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
    let changed_settings: Vec<(String,String,String)> = match sqlx::query("SELECT plugin_name, setting_key, setting_value FROM plugin_settings WHERE setting_value != setting_default").fetch_all(&mut **db).await {
        Ok(res) => res.into_iter().map(|it| {
            let plugin_name: &str = it.get(0);
            let setting_key: &str = it.get(1);
            let setting_value: &str = it.get(2);
            (plugin_name.into(), setting_key.into(), setting_value.into())
        }).collect::<Vec<(String, String, String)>>(),
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    // empty db
    if let Err(err) =
        sqlx::query("DELETE FROM plugins; DELETE FROM plugin_files; DELETE FROM plugin_settings")
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
                                let plugin_name: String = dir_entry
                                    .file_name()
                                    .into_string()
                                    .expect("Path not string compatible");
                                for setting in
                                    changed_settings.iter().filter(|si| si.0 == plugin_name)
                                {
                                    if let Err(err) = sqlx::query(
                                        "UPDATE plugin_settings SET setting_value = ? WHERE plugin_name == ? AND setting_key == ?;",
                                    )
                                    .bind(&setting.2)
                                    .bind(&setting.0)
                                    .bind(&setting.1)
                                    .execute(&mut **db)
                                    .await
                                    {
                                        eprintln!("{}", err);
                                        return Err(Status::InternalServerError);
                                    };
                                }
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
                    let category: &String = match json_root.get("category") {
                        Some(serde_json::Value::String(v)) => v,
                        Some(_) => {
                            eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (invalid description)");
                            return Err(Status::InternalServerError);
                        }
                        None => &String::from("other"),
                    };
                    if let Err(err) = sqlx::query("INSERT INTO plugins ( plugin_name, description, enabled, category ) VALUES ( ?, ?, ?, ? )")
                        .bind(plugin_name)
                        .bind(description)
                        .bind(enabled_plugins.contains(&String::from(plugin_name)))
                        .bind(category)
                        .execute(&mut *db)
                        .await
                    {
                        eprintln!("{}", err);
                        return Err(Status::InternalServerError);
                    };
                    match json_root.get("algorithms") {
                        Some(serde_json::Value::Array(algo_list)) => {
                            for algo in algo_list.into_iter() {
                                match algo {
                                    serde_json::Value::Object(algo_obj) => {
                                        let algo_name: &String = match algo_obj.get("name") {
                                            Some(serde_json::Value::String(v)) => v,
                                            _ => {
                                                eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (algorithm name missing or not a string)");
                                                return Err(Status::InternalServerError);
                                            }
                                        };
                                        let algo_desc: &String = match algo_obj.get("description") {
                                            Some(serde_json::Value::String(v)) => v,
                                            None => &String::new(),
                                            _ => {
                                                eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (algorithm description not a string)");
                                                return Err(Status::InternalServerError);
                                            }
                                        };
                                        let algo_used_data: &String = match algo_obj
                                            .get("used_data")
                                        {
                                            Some(serde_json::Value::String(v)) => v,
                                            None => &String::new(),
                                            _ => {
                                                eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (algorithm used_data not a string)");
                                                return Err(Status::InternalServerError);
                                            }
                                        };
                                        let algo_is_primary: bool = match algo_obj.get("is_primary")
                                        {
                                            Some(serde_json::Value::Bool(v)) => *v,
                                            None => false,
                                            _ => {
                                                eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (algorithm is_primary not a bool)");
                                                return Err(Status::InternalServerError);
                                            }
                                        };
                                        if let Err(err) = sqlx::query("INSERT INTO plugin_algorithms ( algorithm_name, algorithm_description, plugin_name, is_primary, used_data ) VALUES ( ?, ?, ?, ?, ? )")
                                            .bind(algo_name)
                                            .bind(algo_desc)
                                            .bind(plugin_name)
                                            .bind(algo_is_primary)
                                            .bind(algo_used_data)
                                            .execute(&mut *db)
                                            .await
                                        {
                                            eprintln!("{}", err);
                                            return Err(Status::InternalServerError);
                                        };
                                    }
                                    _ => {
                                        eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (algorithm not a record)");
                                        return Err(Status::InternalServerError);
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (algorithms not a list)");
                            return Err(Status::InternalServerError);
                        }
                        None => (),
                    }
                    match json_root.get("settings") {
                        Some(serde_json::Value::Object(map)) => {
                            for (k, rv) in map.into_iter() {
                                if let serde_json::Value::String(v) = rv {
                                    if let Err(err) = sqlx::query("INSERT INTO plugin_settings ( plugin_name, setting_key, setting_value, setting_default ) VALUES ( ?, ?, ?, ? )")
                                        .bind(plugin_name)
                                        .bind(k)
                                        .bind(v)
                                        .bind(v)
                                        .execute(&mut *db)
                                        .await
                                    {
                                        eprintln!("{}", err);
                                        return Err(Status::InternalServerError);
                                    };
                                } else {
                                    eprintln!(
                                        "ERROR: plugin {} contains a non-string default value",
                                        plugin_name
                                    );
                                    return Err(Status::InternalServerError);
                                }
                            }
                        }
                        Some(_) => {
                            eprintln!("Plugin parse issue: invalid meta.json at {json_file_path:?} (invalid settings)");
                            return Err(Status::InternalServerError);
                        }
                        None => (),
                    }
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
