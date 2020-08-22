#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::templates::Template;
use rocket::response::NamedFile;
use rocket::State;
use rocket::request::Form;

use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsStr;
use std::sync::{RwLock, Arc};

pub mod builds;
pub mod files;

use builds::{Commits, Commit};

#[get("/")]
fn index() -> Template {
    Template::render("index", 0)
}

#[get("/tutorial")]
fn tutorial() -> Template {
    Template::render("tutorial", 0)
}

#[get("/manual")]
fn manual() -> Template {
    Template::render("manual", 0)
}

#[get("/tas")]
fn tas() -> Template {
    Template::render("tas", 0)
}

#[derive(FromForm)]
struct BuildsPageRequest {
    page:     Option<u32>,
    per_page: Option<u32>,
}

#[derive(Serialize, Clone)]
pub struct BuildsPage {
    pub netplay:          Option<Commit>,
    pub builds:           Vec<Commit>,
    pub current_page:     u32,
    pub last_page:        u32,
    pub next_page:        Option<u32>,
    pub prev_page:        Option<u32>,
    pub prev_page_enable: bool, // separate flag because otherwise handlebars doesnt handle when prev_page == 0
}

#[get("/builds")]
fn builds(builds_rw: State<Arc<RwLock<Commits>>>) -> Template {
    let builds_page = Form(BuildsPageRequest {
        page:     None,
        per_page: None,
    });
    builds_paginated(builds_rw, builds_page)
}

#[get("/builds?<request..>")]
fn builds_paginated(builds_rw: State<Arc<RwLock<Commits>>>, request: Form<BuildsPageRequest>) -> Template {
    let current_page = request.page.unwrap_or(0);
    let per_page = request.per_page.unwrap_or(20);

    let commits = {
        let builds_r: &Commits = &builds_rw.read().unwrap();
        let start = (current_page * per_page) as usize;
        let end = (((current_page + 1) * per_page) as usize).min(builds_r.builds.len());

        let netplay = if current_page == 0 {
            builds_r.netplay.clone()
        } else {
            None
        };

        let mut builds: Vec<Commit> = vec!();
        for i in start..end {
             builds.push(builds_r.builds[i].clone());
        }

        let prev_page = if current_page > 0 {
            Some(current_page - 1)
        } else {
            None
        };
        let prev_page_enable = prev_page.is_some();

        let next_page = if end < builds_r.builds.len() {
            Some(current_page + 1)
        } else {
            None
        };

        let last_page = if per_page == 0 { 0 } else { builds_r.builds.len() as u32 / per_page };

        BuildsPage { netplay, builds, prev_page, current_page, next_page, last_page, prev_page_enable }
    };
    Template::render("builds", commits)
}

#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> rocket::Rocket {
    if env::current_dir().unwrap().file_name().unwrap() != OsStr::new("website") {
        // templates are correctly located by finding the Rocket.toml
        // However static files are not handled, so if we aren't in the root directory, close immediately to avoid headaches.
        panic!("Wrong directory, dummy!");
    }
    let builds = builds::build_reader();
    rocket::ignite()
        .manage(builds)
        .mount("/", routes![index, builds_paginated, builds, tutorial, manual, tas, files])
        .attach(Template::fairing())
}
