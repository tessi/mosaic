use image::EncodableLayout;
use image::Rgb;
use kmeans_colors::{get_kmeans, Calculate, Kmeans};
use palette::cast::from_component_slice;
use palette::{IntoColor, Lab, Srgb};

use image::imageops::colorops::ColorMap;

#[derive(Clone, Debug)]
pub struct Palette {
    kmeans: Kmeans<Lab>,
}

fn centroid_color(centroid: Lab) -> Rgb<u8> {
    let rgb: Srgb<u8> = Srgb::from_linear(centroid.into_color());
    Rgb(rgb.into_components().into())
}

impl Palette {
    pub fn new(image: &image::RgbImage, max_colors: usize) -> Palette {
        let mut result = Kmeans::new();

        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).expect("getrandom failed");
        let seed = u64::from_le_bytes(buf);

        let lab = from_component_slice::<Srgb<u8>>(image.as_bytes())
            .iter()
            .map(|rgb| rgb.into_format().into_color())
            .collect::<Vec<Lab>>();

        for i in 0..3 {
            let run_result = get_kmeans(max_colors, 20, 5.0, false, &lab, seed + i as u64);
            if run_result.score < result.score {
                result = run_result;
            }
        }

        Palette { kmeans: result }
    }

    pub fn shrink(&mut self, colors: Vec<Rgb<u8>>) {
        let used_lab_colors = colors
            .iter()
            .map(|color| self.kmeans.centroids[self.index_of(color)])
            .collect::<Vec<Lab>>();

        self.kmeans
            .centroids
            .retain(|c| used_lab_colors.contains(c));
    }

    #[inline(always)]
    pub fn map_mut(&self, color: &mut Rgb<u8>) {
        let index = self.index_of(color);
        if let Some(new_color) = self.lookup(index) {
            *color = new_color;
        }
    }

    #[inline(always)]
    pub fn map(&self, color: &Rgb<u8>) -> Option<Rgb<u8>> {
        let index = self.index_of(color);
        self.lookup(index)
    }
}

impl ColorMap for Palette {
    type Color = Rgb<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Rgb<u8>) -> usize {
        let lab: Lab = Srgb::from_components(color.0.into())
            .into_format()
            .into_color();
        let buffer = [lab];
        let mut indices = vec![];
        Lab::get_closest_centroid(&buffer, &self.kmeans.centroids, &mut indices);
        indices[0] as usize
    }

    #[inline(always)]
    fn lookup(&self, idx: usize) -> Option<Self::Color> {
        let rgb = centroid_color(self.kmeans.centroids[idx]);
        Some(rgb)
    }

    fn has_lookup(&self) -> bool {
        true
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Rgb<u8>) {
        self.map_mut(color)
    }
}
