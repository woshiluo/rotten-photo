// Modify from https://users.rust-lang.org/t/converting-png-jpeg-image-to-webp/71080
//
use image::io::Reader as ImageReader;
use image::{DynamicImage, EncodableLayout}; // Using image crate: https://github.com/image-rs/image
use webp::{Encoder, WebPMemory}; // Using webp crate: https://github.com/jaredforth/webp

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::BackendError;

pub async fn image_to_webp(
    source_file: impl AsRef<Path>,
    target_file: impl AsRef<Path>,
) -> Result<std::path::PathBuf, BackendError> {
    let source_file = source_file.as_ref();
    let target_file = target_file.as_ref();
    let image = ImageReader::open(source_file)?;
    let image: DynamicImage = image.with_guessed_format()?.decode()?;

    // Make webp::Encoder from DynamicImage.
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory.
    let encoded_webp: WebPMemory = encoder.encode(65f32);

    // Make File-stream for WebP-result and write bytes into it, and save to path "output.webp".
    let mut webp_image = File::create(target_file)?;
    webp_image.write_all(encoded_webp.as_bytes())?;
    Ok(target_file.to_path_buf())
}
