use image;
use mosaic::{self, color_map::Palette, Image};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::env;
use std::io::BufWriter;
use std::io::Write;

const TILE_SIZE: u32 = 140;
const NUM_COLORS: usize = 128;
const SUPPORTED_TILE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];
    let tile_dir_path = &args[3];

    let mut image = image::open(input_path)
        .expect("failed to open image")
        .to_rgb8();

    let mut color_map = Palette::new(&image, NUM_COLORS);
    let mut image_map: HashMap<image::Rgb<u8>, Vec<Image>> = HashMap::new();
    read_sample_imaged_into_map(&mut image_map, &color_map, tile_dir_path);

    let available_colors = image_map.keys().cloned().collect::<Vec<_>>();
    color_map.shrink(available_colors.clone());
    image::imageops::dither(&mut image, &color_map);
    // un-comment the following line to see the dithered image without applying
    // the mosaic (and comment-out the rest :D so it's not overwritten)
    // image.save(output_path).expect("failed to save image");

    let file = std::fs::File::create(output_path).expect("failed to create file");
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image.width() * TILE_SIZE, image.height() * TILE_SIZE);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder
        .write_header()
        .expect("failed to write header")
        .into_stream_writer()
        .expect("failed to create stream writer");

    for row in image.rows() {
        let tiles = row
            .map(|p| {
                image_map
                    .get(&color_map.map(&p).expect("a colors should be mappable"))
                    .and_then(|images| images.choose(&mut rand::thread_rng()))
                    .expect("expected pixel to be mapped")
            })
            .cloned()
            .collect();
        let combined_tiles = mosaic::combine_images(tiles);
        let bytes = &combined_tiles
            .iter()
            .flatten()
            .flat_map(|pixel| pixel.0)
            .collect::<Vec<u8>>();
        writer.write_all(bytes).expect("failed to write row");
    }
    writer.finish().expect("failed to finish writing out file");
}

fn read_sample_imaged_into_map(
    image_map: &mut HashMap<image::Rgb<u8>, Vec<Image>>,
    color_map: &Palette,
    tile_dir_path: &str,
) {
    let tile_paths = std::fs::read_dir(tile_dir_path).expect("failed to read sample directory");

    for tile_image_path in tile_paths {
        let tile_image_path = tile_image_path.expect("failed to read sample path");
        let tile_image_path = tile_image_path.path();

        if let Some(ext) = tile_image_path.extension() {
            if !SUPPORTED_TILE_EXTENSIONS.contains(&ext.to_str().unwrap()) {
                continue;
            }
        } else {
            continue;
        }

        let tile_image_path = tile_image_path
            .to_str()
            .expect("failed to convert sample path to str");
        let tile_image = image::open(tile_image_path).expect("failed to open sample image");
        add_to_map(image_map, tile_image, color_map);
    }
}

fn add_to_map(
    image_map: &mut HashMap<image::Rgb<u8>, Vec<Image>>,
    tile_image: image::DynamicImage,
    palette: &Palette,
) {
    let rows = mosaic::resize_and_extract_pixel_rows(tile_image, TILE_SIZE, TILE_SIZE)
        .expect("failed to resize");
    let mut tile_main_color = mosaic::average_color(&rows);
    palette.map_mut(&mut tile_main_color);
    if let Some(images) = image_map.get_mut(&tile_main_color) {
        images.push(rows);
    } else {
        image_map.insert(tile_main_color, vec![rows]);
    }
}
