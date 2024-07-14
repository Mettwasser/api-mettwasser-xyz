pub mod api;
pub mod error;
pub mod extract;
mod home;
pub mod utils;

use axum::{routing::get, Router};
use error::ApiError;
use extract::Json;
use include_dir::{include_dir, Dir};

const TEMPLATES: Dir = include_dir!("./templates");

pub trait GetFileContentsFromDir<'a> {
    fn get_str(&self, filename: &str) -> &'a str;
}

impl<'a> GetFileContentsFromDir<'a> for Dir<'a> {
    fn get_str(&self, filename: &str) -> &'a str {
        self.get_file(filename).unwrap().contents_utf8().unwrap()
    }
}

pub fn router() -> Router {
    Router::new()
        .merge(api::router())
        .route("/", get(home::home))
}

pub type ApiResult<T> = std::result::Result<Json<T>, ApiError>;
