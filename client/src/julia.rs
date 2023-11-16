use complex::complex_operations::ComplexOperations;
use complex::julia_descriptor_impl::JuliaOperations;
use image::{ImageBuffer, Rgb};
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::FractalType::Julia;
use shared::types::messages::FragmentTask;

/// Generates an image of the Julia set fractal based on the provided fragment task.
///
/// # Arguments
/// * `fragment_task`: A `FragmentTask` containing details such as the fractal type, resolution, and range.
///
/// # Returns
/// Returns an `ImageBuffer` containing the generated Julia set fractal.
///
/// # Details
/// This function scales the coordinates based on the provided resolution and range, computes the number of
/// iterations for each pixel, and then maps these iterations to a color value.
pub fn generate_julia_set(fragment_task: FragmentTask) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let descriptor = match &fragment_task.fractal.fractal_type {
        Julia(julia_descriptor) => julia_descriptor,
    };
    let resolution = &fragment_task.resolution;
    let range = &fragment_task.range;

    let scale_x = (range.max.x - range.min.x) / resolution.nx as f64;
    let scale_y = (range.max.y - range.min.y) / resolution.ny as f64;

    ImageBuffer::from_fn(resolution.nx.into(), resolution.ny.into(), |x, y| {
        let scaled_x = x as f64 * scale_x + range.min.x;
        let scaled_y = y as f64 * scale_y + range.min.y;
        let complex_point = Complex::new(scaled_x, scaled_y);

        let iterations =
            descriptor.iterate_complex_point(&complex_point, fragment_task.max_iteration);
        let color_value = iterations_to_color(iterations, fragment_task.max_iteration);

        Rgb([color_value, 0, 0])
    })
}

fn iterations_to_color(iterations: u16, max_iterations: u16) -> u8 {
    if iterations == max_iterations {
        0
    } else {
        ((iterations as f64 / max_iterations as f64) * 255.0 * 5.0) as u8 // (to edit the intensity of the fractal, you could modify 12.0)
    }
}
