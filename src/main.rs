#[macro_use]
extern crate rocket;

use rocket::response::content::Html;
use rocket::fs::{FileServer, relative};

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {

}

#[get("/")]
fn index() -> Html<String> {
    let template = HomepageTemplate { };
    Html(template.to_string())
}

#[derive(Template)]
#[template(path = "portfolio.html")]
struct PortfolioTemplate {

}

#[get("/portfolio")]
fn portfolio() -> Html<String> {
    let template = PortfolioTemplate { };
    Html(template.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![index, portfolio])
}
