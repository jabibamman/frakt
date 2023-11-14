use complex::complex_operations::ComplexOperations;
use image::{ImageBuffer, Rgb};
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::FractalType::{IteratedSinZ};
use complex::iterated_sinz_impl::IteratedSinZOperations;
use shared::types::messages::FragmentTask;

pub fn generate_julia_set(fragment_task: FragmentTask) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor = match descriptor {
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

        let iterations =
            descriptor.iterate_complex_point(&complex_point);
        *pixel = Rgb(color((iterations as f32) / 255.0));
    }

    img
}


fn color(t: f32) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}


fn iterations_to_color(mut iterations: u16, max_iterations: u16) -> u8 {
    iterations *= 5;
    if iterations > 255 {
        255
    }
    else {
        iterations as u8
    }



    // if iterations == max_iterations {
    //     0
    // } else {
    //     ((iterations / max_iterations) * 255) as u8 // (to edit the intensity of the fractal, you could modify 12.0)
    // }
}
