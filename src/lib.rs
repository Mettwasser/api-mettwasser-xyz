pub mod assets;
pub mod docs;
pub mod endpoints;
pub mod error;
pub mod macros;

use include_dir::{include_dir, Dir};

const TEMPLATES: Dir = include_dir!("./templates");
const ASSETS: Dir = include_dir!("./assets");
const BUILD: Dir = include_dir!("./build");

pub trait GetFileContentsFromDir<'a> {
    fn get_str(&self, filename: &str) -> &'a str;
}

impl<'a> GetFileContentsFromDir<'a> for Dir<'a> {
    fn get_str(&self, filename: &str) -> &'a str {
        self.get_file(filename).unwrap().contents_utf8().unwrap()
    }
}
