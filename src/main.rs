#[macro_use]
extern crate rocket;

use rocket::response::content;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
}
