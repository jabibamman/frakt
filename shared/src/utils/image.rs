use image::{ImageBuffer, Rgb};

use crate::types::{pixel_intensity::PixelIntensity, resolution::Resolution};

use super::colors_utils::color;



pub fn image_from_pixel_intensity(pixel_intensity: &[PixelIntensity], resolution: Resolution) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(resolution.nx.into(), resolution.ny.into());
    for (i, pixel_intensity) in pixel_intensity.iter().enumerate() {
        let x = (i % 250 as usize) as u32;
        let y = (i / 250 as usize) as u32;
    
        img.put_pixel(x, y, Rgb(color(*pixel_intensity)));
    }

    img
}