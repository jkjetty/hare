use std::collections::HashMap;
use std::convert::From;
use std::fmt::Write;

use lazy_static::lazy_static;
use maplit::hashmap;
use rocket::{
    fs::NamedFile,
    get, launch,
    response::{content::RawHtml, Redirect},
    routes,
};

mod utils;

lazy_static! {
    static ref PATH_MAP:HashMap<Vec<String>, HareOption> = hashmap!{
        // Keep in alphabetic order
        vec!["cal".to_owned()] => HareOption::GCal,
        vec!["crates".to_owned()] => HareOption::CratesIo,
        vec!["drive".to_owned()] => HareOption::GDrive,
        vec!["dp".to_owned(), "disney".to_owned(), "disneyplus".to_owned()] => HareOption::DisneyPlus,
        vec!["g".to_owned()] => HareOption::Google,
        vec!["gh".to_owned()] => HareOption::Github,
        vec!["hn".to_owned()] => HareOption::HackerNews,
        vec!["ip".to_owned()] => HareOption::Instapaper,
        vec!["l3".to_owned()] => HareOption::Localhost3000,
        vec!["l8".to_owned()] => HareOption::Localhost8000,
        vec!["lh".to_owned()] => HareOption::Localhost,
        vec!["ls".to_owned()] => HareOption::Ls,
        vec!["lyrics".to_owned()] => HareOption::Lyrics,
        vec!["mail".to_owned()] => HareOption::GMail,
        vec!["map".to_owned(), "maps".to_owned()] => HareOption::GMaps,
        vec!["photo".to_owned(), "photos".to_owned()] => HareOption::GPhotos,
        vec!["rc".to_owned()] => HareOption::Recollect,
        vec!["sg".to_owned()] => HareOption::SourceGraph,
        vec!["tw".to_owned()] => HareOption::Twitter,
    };
}

#[derive(Debug, Clone)]
enum HareOption {
    GCal,     // gcal
    CratesIo, // crates.io
    GDrive,   // gdrive
    DisneyPlus,
    Google,
    Github,
    HackerNews,
    Instapaper,
    Localhost3000,
    Localhost8000,
    Localhost,
    Ls,     // List all Hare commands
    Lyrics, // Genius lyrics
    GMail,
    GMaps,
    GPhotos,
    Recollect,
    SourceGraph,
    Twitter,
}

impl From<&str> for HareOption {
    fn from(cmd: &str) -> Self {
        PATH_MAP
            .get(&vec![cmd.to_owned()])
            .unwrap_or_else(|| {
                // PATH_MAP.iter().map(|(cmds, option)|{
                for (cmds, option) in PATH_MAP.iter() {
                    if cmds.contains(&cmd.to_owned()) {
                        return option;
                    }
                }
                &HareOption::Google
            })
            .clone()
    }
}

impl ToString for HareOption {
    fn to_string(&self) -> String {
        match self {
            Self::GCal => "Google Calendar".to_string(),
            Self::CratesIo => "crates.io".to_string(),
            Self::GDrive => "Google Drive".to_string(),
            Self::DisneyPlus => "Disney Plus".to_string(),
            Self::Google => "Google search".to_string(),
            Self::Github => "Github".to_string(),
            Self::HackerNews => "Hacker News landing page".to_string(),
            Self::Instapaper => "Instapaper home".to_string(),
            Self::Localhost3000 => "localhost:3000".to_string(),
            Self::Localhost8000 => "localhost:8000".to_string(),
            Self::Localhost => "localhost".to_string(),
            Self::Ls => "List all Hare commands".to_string(),
            Self::Lyrics => "Genius lyrics".to_string(),
            Self::GMail => "gmail".to_string(),
            Self::GMaps => "Google Maps".to_string(),
            Self::GPhotos => "Google Photos".to_string(),
            Self::Recollect => "Recollect home".to_string(),
            Self::SourceGraph => "Sourcegraph search".to_string(),
            Self::Twitter => "Twitter".to_string(),
        }
    }
}

impl HareOption {
    fn url(self, cmd: &str) -> String {
        match self {
            Self::GCal => String::from("https://calendar.google.com/"),
            Self::CratesIo => utils::crates::construct_crates_search_url(&cmd),
            Self::GDrive => String::from("https://drive.google.com/"),
            Self::DisneyPlus => String::from("https://disneyplus.com"),
            Self::Google => utils::google::construct_google_search_url(&cmd),
            Self::Github => utils::github::construct_github_url(&cmd),
            Self::HackerNews => String::from("https://news.ycombinator.com/"),
            Self::Instapaper => String::from("https://instapaper.com"),
            Self::Localhost3000 => String::from("http://localhost:3000/"),
            Self::Localhost8000 => String::from("http://localhost:8000/"),
            Self::Localhost => utils::localhost::construct_localhost_url(&cmd),
            Self::Ls => String::from("http://localhost:8000/ls"),
            Self::Lyrics => utils::lyrics::construct_lyrics_url(&cmd),
            Self::GMail => String::from("https://mail.google.com/"),
            Self::GMaps => String::from("https://maps.google.com/"),
            Self::GPhotos => String::from("https://photos.google.com/"),
            Self::Recollect => String::from("https://app.re-collect.ai/"),
            Self::SourceGraph => utils::sourcegraph::construct_sourcegraph_search_url(&cmd),
            Self::Twitter => utils::twitter::construct_twitter_url(&cmd),
        }
    }
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/favicon.co").await.ok()
}

#[get("/")]
fn index() -> &'static str {
    "This is hare. Use it to augment your search."
}

#[get("/ls")]
// fn ls() -> RawHtml<String> {
fn ls() -> String {
    let mut listing = String::new();
    for (cmds, opt) in PATH_MAP.iter() {
        write!(listing, "{}\n", cmds.join(" | "));
        write!(listing, "\t{}\n", &opt.to_string());
    }
    // format!("{:#?}", PATH_MAP.iter().collect::<Vec<(&Vec<String>, &HareOption)>>())
    listing
}

// rename cmd to query
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    // We need a way to match only on the cmd, without losing the rest of the query
    // "tw something"
    let command = utils::get_command_from_query_string(&cmd);

    let option = HareOption::from(command);
    let redirect_url = option.url(&cmd);
    Redirect::to(redirect_url)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search, favicon, ls])
}
