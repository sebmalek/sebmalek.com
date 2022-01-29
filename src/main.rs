#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::response::status::BadRequest;

use rocket::form::Form;
use rocket::fs::{TempFile, FileServer, relative};

use rocket::serde::Serialize;
use rocket::serde::json::Json;

use rocket::http::ContentType;

use askama::Template;

use std::path::Path;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {

}

#[get("/")]
fn index() -> content::Html<String> {
    let template = HomepageTemplate { };
    content::Html(template.to_string())
}

//const BASE_URL: &'static str = "http://127.0.0.1:8000/images/";
//const UPLOAD_PATH: &'static str = "./images";

const BASE_URL: &'static str = "https://sebmalek.com/i/";
const UPLOAD_PATH: &'static str = "/var/www/images";

fn gen_random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(10)
        .collect::<String>()
}

#[derive(Serialize)]
struct UploadResult {
    link: String,
}

#[post("/upload", data = "<file>")]
async fn upload(mut file: Form<TempFile<'_>>) -> Result<Json<UploadResult>, BadRequest<String>> {
    let content_type = file.content_type().ok_or_else(|| {
        println!("tried to upload file without content type");

        BadRequest(Some("missing content type".to_string()))
    })?;

    if content_type != &ContentType::PNG
        && content_type != &ContentType::JPEG
    {
        println!("tried to upload file that is not png nor jpeg");

        return Err(BadRequest(Some("only PNG and JPEG are allowed".to_string())));
    }

    if file.name().is_none() {
        println!("tried to upload file with invalid or missing filename");

        return Err(BadRequest(Some("invalid or missing filename".to_string())));
    }

    let extension = content_type.extension().unwrap().as_str().to_lowercase();
    let filename = format!("{}.{}", gen_random_string(), extension);

    let path = Path::new(UPLOAD_PATH).join(&filename);
    println!("will store file @ {}", path.display());

    if let Err(e) = file.move_copy_to(path).await {
        println!("error while storing file: {}", e);

        return Err(BadRequest(Some("error while storing file".to_string())));
    }

    Ok(Json(UploadResult {
        link: format!("{}{}", BASE_URL, filename),
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
//        .mount("/images", FileServer::from(relative!("images")))
//        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![index, upload])
}
