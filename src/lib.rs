pub mod assets;
pub mod docs;
pub mod endpoints;
pub mod error;
pub mod error_codes;
pub mod macros;

use include_dir::{include_dir, Dir};

const TEMPLATES: Dir = include_dir!("./templates");
const ASSETS: Dir = include_dir!("./build");
