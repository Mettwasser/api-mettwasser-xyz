pub mod api;
pub mod error;
mod home;

use {
    axum::{routing::get, Router},
    error::ApiError,
    include_dir::{include_dir, Dir},
};

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

pub type Result<T> = std::result::Result<T, ApiError>;
