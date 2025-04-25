#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Database};

use atrea_webui::{
    ai_training_data::ai_training_data, api_channel, api_plugins, api_raids, api_recs, api_viewers,
    frontend, AtreaDb, AtreaSettingsDb,
};

const BASEPATH: &str = "/";

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AtreaDb::init())
        .attach(AtreaSettingsDb::init())
        .attach(AdHoc::try_on_ignite(
            "atrea_settings_db init",
            init_settings_db,
        ))
        .mount(
            BASEPATH,
            routes![
                api_channel::get_known_viewers,
                api_channel::get_twitch_info,
                api_channel::redirect_image,
                api_plugins::disable_plugin,
                api_plugins::enable_plugin,
                api_plugins::get_css,
                api_plugins::get_js,
                api_plugins::list,
                api_plugins::update_db,
                api_raids::force_graph_data,
                api_raids::raids_from,
                api_raids::raids_from_to,
                api_raids::raids_to,
                api_raids::raidstats_from,
                api_raids::raidstats_from_to,
                api_raids::raidstats_to,
                api_recs::get_general,
                api_recs::get_liked_channels,
                api_recs::list_general_algorithms,
                api_viewers::get_known_viewers,
                frontend::get_css_biglist,
                frontend::get_css_box_style,
                frontend::get_css_style,
                frontend::get_html_box_channel,
                frontend::get_html_box_help_credits,
                frontend::get_html_box_help_recs,
                frontend::get_html_box_known_viewers,
                frontend::get_html_box_nav,
                frontend::get_html_box_plugins,
                frontend::get_html_box_recs,
                frontend::get_html_box_rel_graph,
                frontend::get_html_index,
                frontend::get_js_box_channel,
                frontend::get_js_box_nav,
                frontend::get_js_box_recs,
                frontend::get_js_index_mailbox,
                frontend::get_js_lib_force_graph,
                frontend::get_js_lib_winbox,
                frontend::get_js_lib_xeact,
                frontend::get_slash,
                frontend::get_svg_loading,
                frontend::get_svg_twitch_glitch,
                ai_training_data, // a gift for all the ai-companies out there
            ],
        )
}

async fn init_settings_db(rocket: Rocket<Build>) -> fairing::Result {
    match AtreaSettingsDb::fetch(&rocket) {
        Some(db) => {
            if let Err(e) = sqlx::query(include_str!("sql/settings_init.sql"))
                .fetch_optional(&**db)
                .await
            {
                error!("Failed to initialize SQLx database: {}", e);
                return Err(rocket);
            };
            Ok(rocket)
        }
        None => Err(rocket),
    }
}
