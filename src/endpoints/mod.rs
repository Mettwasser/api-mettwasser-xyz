// home
pub use home::home;

// Image
pub use image::generate_captcha_response;
pub use image::preview_color;
pub use image::round_image;

// No Category
pub use random_color::random_color;

mod home;
mod image;
mod random_color;
