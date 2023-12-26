use complex::complex_operations::ComplexOperations;
use complex::fractal_operations::FractalOperations;
use image::{ImageBuffer, Rgb};
use shared::types::color::{RGB, HSL};
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::FractalType::{
    IteratedSinZ, Julia, Mandelbrot, NewtonRaphsonZ3, NewtonRaphsonZ4,
};
use shared::types::messages::FragmentTask;
use shared::types::pixel_intensity::PixelIntensity;

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
        Mandelbrot(mandelbrot_descriptor) => mandelbrot_descriptor,
        NewtonRaphsonZ3(newton_raphson_z3_descriptor) => newton_raphson_z3_descriptor,
        NewtonRaphsonZ4(newton_raphson_z4_descriptor) => newton_raphson_z4_descriptor,
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

        let pixel_intensity = descriptor.compute_pixel_intensity(&complex_point, fragment_task.max_iteration);
        *pixel = Rgb(color(pixel_intensity));
    }

    img
}

///Generates a color based on the provided pixel intensity.
/// # Arguments
/// * `pixel_intensity`: A `PixelIntensity` containing the number of iterations and the norm of the complex point.
/// 
/// # Returns
/// Returns an array containing the RGB values of the color.
/// 
fn color(pixel_intensity: PixelIntensity) -> [u8; 3] {
    let hsl = HSL {
        h: pixel_intensity.count * 360.0,
        s: 0.5 + 0.5 * (pixel_intensity.zn * 3.14).cos(),
        l: 0.5,
    };

    let color = hsl_to_rgb(hsl);

    [color.r, color.g, color.b]
}

    /// Convert a color from HSL to RGB
    /// # Arguments
    /// * `hsl`: A `HSL` containing the HSL values of the color (Hue, Saturation, Lightness)
    /// 
    /// # Returns
    /// Returns a tuple containing the RGB values of the color
    /// 
    /// # Details
    /// This function is based on the algorithm found at https://www.rapidtables.com/convert/color/hsl-to-rgb.html
    /// 
    fn hsl_to_rgb(hsl: HSL) -> RGB {
        let c = (1.0 - (2.0 * hsl.l - 1.0).abs()) * hsl.s;
        let h_prime = hsl.h / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let m = hsl.l - c / 2.0;
    
        let (r_temp, g_temp, b_temp) = match h_prime.floor() as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
    
        RGB {
            r: ((r_temp + m) * 255.0) as u8,
            g: ((g_temp + m) * 255.0) as u8,
            b: ((b_temp + m) * 255.0) as u8,
        }
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
