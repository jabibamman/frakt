use complex::julia_descriptor_impl::JuliaOperations;
use image::{ImageBuffer, Rgb};
use shared::types::fractal_descriptor::FractalType::Julia;
use shared::types::messages::FragmentTask;

pub fn generate_julia_set(
    fragment_task: FragmentTask,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor = match descriptor {
        Julia(julia_descriptor) => julia_descriptor,
    };
    let resolution = &fragment_task.resolution;

    let mut img = ImageBuffer::new(resolution.nx.into(), resolution.ny.into());

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let complex_point = descriptor.to_complex(x as u16, y as u16, resolution);
        let iterations = descriptor.iterate_complex_point(&complex_point, fragment_task.max_iteration);
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
        ((iterations as f64 / max_iterations as f64) * 255.0) as u8
    }
}
