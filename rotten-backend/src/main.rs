#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{FileServer, TempFile};
use rocket::serde::{json::Json, Serialize};

pub mod webp;

#[derive(FromForm, Debug)]
struct Upload<'r> {
    image: TempFile<'r>,
}

#[derive(Debug)]
pub enum BackendError {
    IOError,
    ImageError,
}

impl From<std::io::Error> for BackendError {
    fn from(_: std::io::Error) -> BackendError {
        BackendError::IOError
    }
}

impl From<image::error::ImageError> for BackendError {
    fn from(_: image::error::ImageError) -> BackendError {
        BackendError::ImageError
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RottenResponse {
    code: u8,
    path: Option<String>,
    message: String,
}

impl From<String> for RottenResponse {
    fn from(string: String) -> RottenResponse {
        RottenResponse {
            code: 0,
            path: Some(string),
            message: "ok".to_string(),
        }
    }
}

impl From<BackendError> for RottenResponse {
    fn from(err: BackendError) -> RottenResponse {
        RottenResponse {
            code: 1,
            path: None,
            message: format!("{:?}", err),
        }
    }
}

#[post("/", data = "<upload>")]
async fn upload_image(upload: Form<Upload<'_>>) -> Json<RottenResponse> {
    let upload: Upload<'_> = upload.into_inner();
    let path = format!("static/{}.webp", uuid::Uuid::new_v4());
    let source_file = match upload.image.path() {
        Some(path) => path,
        None => return Json(RottenResponse::from(BackendError::IOError)),
    };
    match webp::image_to_webp(source_file, &path).await {
        Ok(_) => Json(RottenResponse::from(path)),
        Err(err) => Json(RottenResponse::from(err)),
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[options("/")]
fn options() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload_image, options])
        .mount("/static", FileServer::from("static/"))
}
