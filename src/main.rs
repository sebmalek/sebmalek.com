#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

use std::path::PathBuf;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {

}

#[get("/")]
fn index() -> content::Html<String> {
    let template = HomepageTemplate { };

    let response = content::Html(template.to_string());
    response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
}
