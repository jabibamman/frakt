use complex::complex_operations::ComplexOperations;
use image::{ImageBuffer, Rgb};
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::FractalType::{IteratedSinZ, Julia};
use complex::iterated_sinz_impl::IteratedSinZOperations;
use shared::types::messages::FragmentTask;

pub fn generate_julia_set(fragment_task: FragmentTask) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor = match descriptor {
        Julia(julia_descriptor) => julia_descriptor,
        IteratedSinZ(iterated_sinz_descriptor) => iterated_sinz_descriptor,
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

        let iterations = descriptor.iterate_complex_point(&complex_point, fragment_task.max_iteration);
        *pixel = Rgb(color((iterations as f32) / 255.0));
    }

    img
}


///Gets a number between 0 and 1 and return the color that correspond to its intensity
fn color(intensity: f32) -> [u8; 3] {
    let brightness = (0.5, 0.5, 0.5);
    let bright_color = (0.5, 0.5, 0.5);
    let frequency_change = (1.0, 1.0, 1.0);
    let base_color = (0.1, 0.2, 0.3);
    let r = bright_color.0 * (6.28318 * (frequency_change.0 * intensity + base_color.0)).cos() + brightness.0;
    let g = bright_color.1 * (6.28318 * (frequency_change.1 * intensity + base_color.1)).cos() + brightness.1;
    let b = bright_color.2 * (6.28318 * (frequency_change.2 * intensity + base_color.2)).cos() + brightness.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}


fn iterations_to_color(mut iterations: u16, max_iterations: u16) -> u8 {
    if iterations == max_iterations {
        0
    } else {
        ((iterations / max_iterations) * 255) as u8 // (to edit the intensity of the fractal, you could modify 12.0)
    }
}
