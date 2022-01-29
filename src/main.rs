#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::fs::{FileServer, relative};

use rocket::response::status::NoContent;

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

#[post("/upload")]
async fn upload() -> NoContent {
    rocket::response::status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index, upload])
        .mount("/i", FileServer::from(relative!("images")))
}
