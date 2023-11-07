use complex::julia_descriptor_impl::JuliaOperations;
use image::{ImageBuffer, Rgb};
use shared::types::julia_descriptor::JuliaDescriptor;
use shared::types::resolution::Resolution;

pub fn generate_julia_set(
    descriptor: &JuliaDescriptor,
    resolution: &Resolution,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(resolution.nx.into(), resolution.ny.into());

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let complex_point = descriptor.to_complex(x as u16, y as u16, resolution);
        let iterations = descriptor.iterate_complex_point(&complex_point);
        *pixel = Rgb([
            iterations_to_color(iterations, descriptor.max_iteration()),
            0,
            0,
        ]);
    }

    img
}

fn iterations_to_color(iterations: u16, max_iterations: u16) -> u8 {
    if iterations == max_iterations {
        0
    } else {
        ((iterations as f64 / max_iterations as f64) * 255.0 * 5.0) as u8 // (to edit the intensity of the fractal, you could modify 12.0)
    }
}
