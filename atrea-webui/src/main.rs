#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

use atrea_webui::{
    ai_training_data::ai_training_data, api_channel, api_raids, api_recs, api_viewers, frontend,
};

const BASEPATH: &str = "/";

#[launch]
fn rocket() -> _ {
    rocket::build().attach(atrea_webui::AtreaDb::init()).mount(
        BASEPATH,
        routes![
            api_channel::get_known_viewers,
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
            api_viewers::get_known_viewers,
            frontend::get_css_biglist,
            frontend::get_css_box_style,
            frontend::get_css_index_style,
            frontend::get_css_style,
            frontend::get_html_box_channel,
            frontend::get_html_box_help_credits,
            frontend::get_html_box_help_recs,
            frontend::get_html_box_known_viewers,
            frontend::get_html_box_recs,
            frontend::get_html_index,
            frontend::get_js_box_channel,
            frontend::get_js_box_recs,
            frontend::get_js_index_mailbox,
            frontend::get_js_index_nav,
            frontend::get_js_lib_winbox,
            frontend::get_js_lib_xeact,
            frontend::get_slash,
            frontend::get_svg_loading,
            frontend::get_svg_twitch_glitch,
            ai_training_data, // a gift for all the ai-companies out there
        ],
    )
}
