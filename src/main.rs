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
fn root() -> content::Html<String> {
    let template = HomepageTemplate { };

    let response = content::Html(template.to_string());
    response
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("static").join(path);
    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
        .mount("/static", routes![static_files])
}
