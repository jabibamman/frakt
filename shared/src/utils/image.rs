use image::{ImageBuffer, Rgb};

use crate::types::{pixel_intensity::PixelIntensity, resolution::Resolution};

use super::{colors_utils::color, env_utils::get_env_var_as_u16};



pub fn image_from_pixel_intensity(pixel_intensity: Vec<PixelIntensity>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let width = get_env_var_as_u16("RESOLUTION_WIDTH")?;
    let height = get_env_var_as_u16("RESOLUTION_HEIGHT")?;

    let resolution = Resolution::new(width, height);
    let mut img = ImageBuffer::new(resolution.nx.into(), resolution.ny.into());
    for (i, pixel_intensity) in pixel_intensity.iter().enumerate() {
        let x = (i as u32) % resolution.nx as u32;
        let y = (i as u32) / resolution.nx as u32;
        img.put_pixel(x, y, Rgb(color(*pixel_intensity)));
    }

    Ok(img)
}