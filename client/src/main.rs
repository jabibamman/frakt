mod calculations;
use complex::complex_impl::ComplexOperations;
use image::{ImageBuffer, Rgb};
use shared::types::complex::Complex;

fn julia(
    c: Complex,
    max_iterations: u32,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let zx = 4.0 * (x as f64) / (width as f64) - 2.0;
        let zy = 4.0 * (y as f64) / (height as f64) - 2.0;
        let mut z = Complex::new(zx, zy);
        let mut i = 0;
        while i < max_iterations && z.norm() <= 2.0 {
            z = z.square().add(&c);
            i += 1;
        }

        let color = if i < max_iterations {
            let log_zn = z.norm().ln();
            let nu = log_zn / 2.0f64.ln();
            let iteration_ratio = (i as f64 + 1.0 - nu) / max_iterations as f64;
            Rgb([
                (255.0 * iteration_ratio) as u8,
                (100.0 * iteration_ratio) as u8,
                (0.0 * iteration_ratio) as u8,
            ])
        } else {
            Rgb([0, 0, 0])
        };

        *pixel = color;
    }
    img
}

fn main() {
    println!("[CLIENT] Hello, world!");
    let c2 = Complex::new(-0.9, 0.27015);
    let img = julia(c2.clone(), 100, 800, 800);
    img.save("target/julia.png").unwrap();
    println!("Valeur du complexe : {:?}", c2);
}
