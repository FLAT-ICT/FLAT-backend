use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use std::{fs::File, io::Cursor};

fn image_to_base64(image_path: &str) -> String {
    let img = image::open(image_path).unwrap();
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
    let base64 = base64::encode(&buf.get_ref());

    base64
}

pub fn base64_to_image(base64_encoded_bytes: &str) -> DynamicImage {
    let bytes = base64::decode(base64_encoded_bytes).unwrap();
    let mut buf = &bytes[..];
    let img = image::load_from_memory_with_format(&mut buf, ImageFormat::Png).unwrap();

    img
}

pub fn resize_and_center_crop(img: DynamicImage, size: u32) -> DynamicImage {
    // clip rectangle to square
    let (width, height) = img.dimensions();
    let min = width.min(height);
    let cropped_img = img.crop_imm((width - min) / 2, (height - min) / 2, min, min);
    let resized_img = cropped_img.resize_to_fill(size, size, FilterType::Gaussian);

    resized_img
}

fn save_img(img: DynamicImage, path: &str) {
    let mut file = File::create(path).unwrap();
    img.write_to(&mut file, ImageOutputFormat::Png).unwrap();
}
