use complex::complex_operations::ComplexOperations;
use complex::julia_descriptor_impl::JuliaOperations;
use image::{ImageBuffer, Rgb};
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::FractalType::Julia;
use shared::types::messages::FragmentTask;

pub fn generate_julia_set(fragment_task: FragmentTask) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor = match descriptor {
        Julia(julia_descriptor) => julia_descriptor,
    };
    let resolution = &fragment_task.resolution;
    let range = &fragment_task.range;

    let scale_x = (range.max.x - range.min.x) / resolution.nx as f64;
    let scale_y = (range.max.y - range.min.y) / resolution.ny as f64;

    let mut img = ImageBuffer::new(resolution.nx.into(), resolution.ny.into());

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let scaled_x = x as f64 * scale_x + range.min.x;
        let scaled_y = y as f64 * scale_y + range.min.y;
        let complex_point = Complex::new(scaled_x, scaled_y);

        let iterations =
            descriptor.iterate_complex_point(&complex_point, fragment_task.max_iteration);
        *pixel = Rgb([
            iterations_to_color(iterations, fragment_task.max_iteration),
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
