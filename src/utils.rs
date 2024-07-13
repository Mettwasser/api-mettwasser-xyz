use {
    crate::error::ApiError,
    axum::body::Bytes,
    image::io::Reader,
    std::{io::Cursor, time::Duration},
};

pub async fn fetch_raw_image(url: &str) -> Result<Bytes, ApiError> {
    let resp = reqwest::Client::builder()
        .build()?
        .get(url)
        .timeout(Duration::from_secs(3))
        .send()
        .await?;

    if let Some(length) = resp.content_length() {
        if length > 3 * 1024 * 1024 {
            return Err(ApiError::FetchError(
                "The requested content cannot exceed 3mb.".to_owned(),
            ));
        }
    } else {
        return Err(ApiError::FetchError(
            "Couldn't retrieve the content length of the requested URL.".to_owned(),
        ));
    }

    Ok(resp.bytes().await?)
}

pub fn image_from_bytes(
    bytes: Bytes,
) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ApiError> {
    let format = Reader::new(Cursor::new(bytes)).with_guessed_format()?;
    Ok(format.decode()?.to_rgba8())
}

pub fn rgb_to_hex(rgb: &[u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}
