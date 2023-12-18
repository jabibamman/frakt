use complex::complex_operations::ComplexOperations;
use complex::fractal_operations::{BurningShipFractalOperations, FractalOperations};
use image::{ImageBuffer, Rgb, Rgba};
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::BurningFractalType::BurningShip;
use shared::types::fractal_descriptor::FractalType::{IteratedSinZ, Julia};
use shared::types::messages::{BurningShipFragmentTask, FragmentTask};

/// Generates an image of a Fractal Type based on the provided fragment task.
///
/// # Arguments
/// * `fragment_task`: A `FragmentTask` containing details such as the fractal type, resolution, and range.
///
/// # Returns
/// Returns an `ImageBuffer` containing the generated Fractal.
///
/// # Details
/// This function scales the coordinates based on the provided resolution and range, computes the number of
/// iterations for each pixel, and then maps these iterations to a color value.
pub fn generate_fractal_set(fragment_task: FragmentTask) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor: &dyn FractalOperations = match descriptor {
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

        let iterations =
            descriptor.iterate_complex_point(&complex_point, fragment_task.max_iteration);
        *pixel = Rgb(color((iterations as f32) / 255.0));
    }

    img
}

pub fn generate_burning_ship_fractal_set(
    fragment_task: BurningShipFragmentTask,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor: &dyn BurningShipFractalOperations = match descriptor {
        BurningShip(burning_ship_descriptor) => burning_ship_descriptor,
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

        let (iterations, escape_distance) =
            descriptor.iterate_complex_point(&complex_point, fragment_task.max_iteration);
        *pixel = Rgba(burning_ship_color(iterations as f32));
    }

    img
}

///Gets a number between 0 and 1 and return the color that correspond to its intensity
fn color(intensity: f32) -> [u8; 3] {
    let brightness = (0.5, 0.5, 0.5);
    let bright_color = (0.5, 0.5, 0.5);
    let frequency_change = (1.0, 1.0, 1.0);
    let base_color = (0.1, 0.2, 0.3);
    let r = bright_color.0 * (6.28318 * (frequency_change.0 * intensity + base_color.0)).cos()
        + brightness.0;
    let g = bright_color.1 * (6.28318 * (frequency_change.1 * intensity + base_color.1)).cos()
        + brightness.1;
    let b = bright_color.2 * (6.28318 * (frequency_change.2 * intensity + base_color.2)).cos()
        + brightness.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}

fn burning_ship_color(intensity: f32) -> [u8; 4] {
    [255, (intensity * 7.0) as u8, 0, (intensity * 15.0) as u8]
}

#[cfg(test)]
mod julia_descriptor_tests {
    use complex::complex_operations::ComplexOperations;
    use shared::types::complex::Complex;
    use shared::types::fractal_descriptor::FractalType::Julia;
    use shared::types::fractal_descriptor::JuliaDescriptor;
    use shared::types::messages::FragmentTask;
    use shared::types::point::Point;
    use shared::types::range::Range;
    use shared::types::resolution::Resolution;
    use shared::types::u8data::U8Data;
    use shared::utils::type_of::type_of;

    use super::*;

    #[test]
    fn test_generate_julia_set() {
        let fragment_task = FragmentTask {
            fractal: shared::types::fractal_descriptor::FractalDescriptor {
                fractal_type: Julia(JuliaDescriptor {
                    c: Complex::new(-0.8, 0.156),
                    divergence_threshold_square: 0.0,
                }),
            },
            resolution: Resolution { nx: 800, ny: 600 },
            range: Range {
                min: Point { x: -2.0, y: -1.5 },
                max: Point { x: 2.0, y: 1.5 },
            },
            max_iteration: 100,
            id: U8Data {
                offset: 0,
                count: 0,
            },
        };

        let result = generate_fractal_set(fragment_task);

        assert_eq!(result.dimensions(), (800, 600));
    }

    #[test]
    fn test_color() {
        let intensity = 0.5;

        let result = color(intensity);

        let test0 = type_of(result[0]);
        let test1 = type_of(result[1]);
        let test2 = type_of(result[2]);

        assert!(test0.eq("u8"));
        assert!(test1.eq("u8"));
        assert!(test2.eq("u8"));
    }
}
