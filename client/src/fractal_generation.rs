use complex::complex_operations::ComplexOperations;
use complex::fractal_operations::FractalOperations;
use image::{ImageBuffer, Rgb};
use log::info;
use shared::types::complex::Complex;
use shared::types::error::FractalError;
use shared::types::fractal_descriptor::FractalType::{
    BurningShip, IteratedSinZ, Julia, Mandelbrot, NewtonRaphsonZ3, NewtonRaphsonZ4,
};
use shared::types::messages::FragmentTask;
use shared::types::pixel_intensity::PixelIntensity;
use shared::utils::colors_utils::color;

/// Generates an image of a Fractal Type based on the provided fragment task.
///
/// # Arguments
/// * `fragment_task`: A `FragmentTask` containing details such as the fractal type, resolution, and range.
///
/// # Returns
/// Returns a tuple containing the generated image, the pixel data, and the pixel intensity matrice.
///
/// # Details
/// This function scales the coordinates based on the provided resolution and range, computes the number of
/// iterations for each pixel, and then maps these iterations to a color value.
pub fn generate_fractal_set(
    fragment_task: FragmentTask,
) -> Result<(ImageBuffer<Rgb<u8>, Vec<u8>>, Vec<u8>, Vec<PixelIntensity>), FractalError> {
    let descriptor = &fragment_task.fractal.fractal_type;
    let descriptor: &dyn FractalOperations = match descriptor {
        Julia(julia_descriptor) => julia_descriptor,
        IteratedSinZ(iterated_sinz_descriptor) => iterated_sinz_descriptor,
        Mandelbrot(mandelbrot_descriptor) => mandelbrot_descriptor,
        NewtonRaphsonZ3(newton_raphson_z3_descriptor) => newton_raphson_z3_descriptor,
        NewtonRaphsonZ4(newton_raphson_z4_descriptor) => newton_raphson_z4_descriptor,
        BurningShip(burning_ship_descriptor) => burning_ship_descriptor,
    };
    let resolution = &fragment_task.resolution;
    let range = &fragment_task.range;

    let scale_x = (range.max.x - range.min.x) / resolution.nx as f64;
    let scale_y = (range.max.y - range.min.y) / resolution.ny as f64;

    let mut img = ImageBuffer::new(resolution.nx.into(), resolution.ny.into());
    let mut pixel_data_vec = Vec::new();
    let mut pixel_matrice_intensity = Vec::new();

    info!("Generating fractal set...");
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let scaled_x = x as f64 * scale_x + range.min.x;
        let scaled_y = y as f64 * scale_y + range.min.y;
        let complex_point = Complex::new(scaled_x, scaled_y);

        let pixel_intensity =
            descriptor.compute_pixel_intensity(&complex_point, fragment_task.max_iteration);

        *pixel = Rgb(color(pixel_intensity));

        pixel_matrice_intensity.push(pixel_intensity);
        pixel_data_vec.push(pixel[0]);
        pixel_data_vec.push(pixel[1]);
        pixel_data_vec.push(pixel[2]);
    }

    Ok((img, pixel_data_vec, pixel_matrice_intensity))
}

#[cfg(test)]
mod julia_descriptor_tests {
    use complex::complex_operations::ComplexOperations;
    use shared::types::complex::Complex;
    use shared::types::fractal_descriptor::FractalType::Julia;
    use shared::types::fractal_descriptor::{
        BurningShipDescriptor, IteratedSinZDescriptor, JuliaDescriptor, MandelbrotDescriptor,
        NewtonRaphsonZ3Descriptor, NewtonRaphsonZ4Descriptor,
    };
    use shared::types::messages::FragmentTask;
    use shared::types::point::Point;
    use shared::types::range::Range;
    use shared::types::resolution::Resolution;
    use shared::types::u8data::U8Data;

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

        if let Ok((img, _, _)) = generate_fractal_set(fragment_task) {
            assert_eq!(img.dimensions(), (800, 600));
        }
    }

    #[test]
    fn test_generate_iterated_sin_z() {
        let fragment_task = FragmentTask {
            fractal: shared::types::fractal_descriptor::FractalDescriptor {
                fractal_type: IteratedSinZ(IteratedSinZDescriptor {
                    c: Complex::new(-0.8, 0.156),
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

        if let Ok((img, _, _)) = generate_fractal_set(fragment_task) {
            assert_eq!(img.dimensions(), (800, 600));
        }
    }

    #[test]
    fn test_generate_mandelbrot() {
        let fragment_task = FragmentTask {
            fractal: shared::types::fractal_descriptor::FractalDescriptor {
                fractal_type: Mandelbrot(MandelbrotDescriptor {}),
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

        if let Ok((img, _, _)) = generate_fractal_set(fragment_task) {
            assert_eq!(img.dimensions(), (800, 600));
        }
    }

    #[test]
    fn test_generate_newton_raphson_z3() {
        let fragment_task = FragmentTask {
            fractal: shared::types::fractal_descriptor::FractalDescriptor {
                fractal_type: NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor {}),
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

        if let Ok((img, _, _)) = generate_fractal_set(fragment_task) {
            assert_eq!(img.dimensions(), (800, 600));
        }
    }

    #[test]
    fn test_generate_newton_raphson_z4() {
        let fragment_task = FragmentTask {
            fractal: shared::types::fractal_descriptor::FractalDescriptor {
                fractal_type: NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor {}),
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

        if let Ok((img, _, _)) = generate_fractal_set(fragment_task) {
            assert_eq!(img.dimensions(), (800, 600));
        }
    }

    #[test]
    fn test_generate_burning_ship() {
        let fragment_task: FragmentTask = FragmentTask {
            id: U8Data {
                offset: 0,
                count: 16,
            },
            fractal: shared::types::fractal_descriptor::FractalDescriptor {
                fractal_type: BurningShip(BurningShipDescriptor {
                    c: Complex { re: 0.0, im: 0.0 },
                    divergence_threshold_square: 4.0,
                }),
            },
            max_iteration: 255,
            resolution: Resolution { nx: 800, ny: 600 },
            range: Range {
                min: Point { x: -1.8, y: -0.08 },
                max: Point { x: -1.7, y: 0.01 },
            },
        };

        if let Ok((img, _, _)) = generate_fractal_set(fragment_task) {
            assert_eq!(img.dimensions(), (800, 600));
        }
    }
}
