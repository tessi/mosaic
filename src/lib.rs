pub mod color_map;

use image::Rgb;

pub type ImageRow = Vec<Rgb<u8>>;
pub type Image = Vec<ImageRow>;

pub fn resize_and_extract_pixel_rows(
    image: image::DynamicImage,
    width: u32,
    height: u32,
) -> Result<Image, String> {
    let image = image.resize_exact(width, height, image::imageops::FilterType::Nearest);
    let image = image.to_rgb8();
    let rows = image
        .rows()
        .map(|r| r.map(|p| p.clone()).collect::<ImageRow>())
        .collect::<Image>();
    Ok(rows)
}

// possibly switch to https://docs.rs/kmeans_colors/0.6.0/kmeans_colors/#getting-the-dominant-color
pub fn average_color(pixel_rows: &Image) -> Rgb<u8> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    let mut count: u32 = 0;
    for row in pixel_rows {
        for pixel in row {
            red += pixel[0] as u32;
            green += pixel[1] as u32;
            blue += pixel[2] as u32;
            count += 1;
        }
    }
    Rgb([
        (red / count) as u8,
        (green / count) as u8,
        (blue / count) as u8,
    ])
}

pub fn combine_images(images: Vec<Image>) -> Image {
    let mut combined = Vec::new();
    let row_count = images[0].len();

    for row in 0..row_count {
        let combined_row = images.iter().fold(vec![], |mut acc, image| {
            acc.extend(image[row].clone());
            acc
        });
        combined.push(combined_row);
    }
    combined
}
