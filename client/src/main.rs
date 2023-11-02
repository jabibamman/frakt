mod julia;
use complex::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::julia_descriptor::JuliaDescriptor;
use shared::types::resolution::Resolution;
use crate::julia::generate_julia_set;
use complex::julia_descriptor_impl::JuliaOperations;

fn main() {
    let c = Complex::new(0.285, 0.013 );
    let descriptor = JuliaDescriptor::new(c.clone(), 4.0, 200);
    let resolution = Resolution { nx: 800, ny: 800 };

    let img = generate_julia_set(&descriptor, &resolution);
    if img.save("target/julia.png").is_ok() {
        println!("L'image du Julia Set a été sauvegardée !");
    }
    else{
        println!("L'image du Julia Set n'a pas été sauvegardée !");
    }

}