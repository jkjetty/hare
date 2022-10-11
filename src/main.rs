#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket::response::Redirect;

mod utils;

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/favicon.co").ok()
}

#[get("/")]
fn index() -> &'static str {
    "This is hare. Use it to augment your search."
}
// rename cmd to query
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    // We need a way to match only on the cmd, without losing the rest of the query
    // "tw something"
    let command = utils::get_command_from_query_string(&cmd);

    // Keep in alphabetic order
    let redirect_url = match command {
        "cal" => String::from("https://calendar.google.com/"),
        "ck" => String::from("https://app.convertkit.com/"),
        "drive" => String::from("https://drive.google.com/"),
        "dp" | "disney" | "disneyplus" => String::from("https://disneyplus.com"),
        "figma" => String::from("https://figma.com"),
        "g" => utils::google::construct_google_search_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "hn" => String::from("https://news.ycombinator.com/"),
        "ip" => String::from("https://instapaper.com"),
        "l3" => String::from("http://localhost:3000/"),
        "l8" => String::from("http://localhost:8000/"),
        "lh" => utils::localhost::construct_localhost_url(&cmd),
        "lyrics" => utils::lyrics::construct_lyrics_url(&cmd),
        "mail" => String::from("https://mail.google.com/"),
        "map" | "maps" => String::from("https://maps.google.com/"),
        "photo" | "photos" => String::from("https://photos.google.com/"),
        "rc" => String::from("https://app.re-collect.ai/"),
        "sg" => utils::sourcegraph::construct_sourcegraph_search_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "wp" => String::from("https://jettylabs.workplace.com/"),
        "wpp" => String::from("https://jettylabs.workplace.com/profile.php"),
        // Default to google search.
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search, favicon])
        .launch();
}
