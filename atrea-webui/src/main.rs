#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

use atrea_webui::{ai_training_data::ai_training_data, api_channel, api_raids, api_recs, frontend};

const BASEPATH: &str = "/";

#[launch]
fn rocket() -> _ {
    rocket::build().attach(atrea_webui::AtreaDb::init()).mount(
        BASEPATH,
        routes![
            api_channel::get_twitch_info,
            api_channel::redirect_image,
            api_raids::raids_from,
            api_raids::raids_from_to,
            api_raids::raids_to,
            api_raids::raidstats_from,
            api_raids::raidstats_from_to,
            api_raids::raidstats_to,
            api_recs::get_general,
            api_recs::list_general_algorithms,
            frontend::get_channel_view_iframe,
            frontend::get_credits,
            frontend::get_index,
            frontend::get_loading_image,
            frontend::get_recommend_channels,
            frontend::get_recommend_channels_js,
            frontend::get_slash,
            frontend::get_style,
            frontend::get_xeact,
            frontend::get_twitch_glitch_logo,
            ai_training_data, // a gift for all the ai-companies out there
        ],
    )
}
