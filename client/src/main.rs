mod image;
mod julia;

use crate::image::open_image;
use crate::julia::generate_julia_set;
use complex::complex_operations::ComplexOperations;
use complex::julia_descriptor_impl::JuliaOperations;
use shared::types::{complex::Complex, julia_descriptor::JuliaDescriptor, resolution::Resolution};
use shared::utils::filesystem::{get_dir_str, get_extension_str};
use shared::types::filesystem::{DirType, FileExtension};

fn main() {    
    let c = Complex::new(-0.9, 0.27015);
    let descriptor = JuliaDescriptor::new(c, 4.0, 500);
    let resolution = Resolution { nx: 800, ny: 800 };

    let img_path = format!("{}/target/julia.{}", get_dir_str(DirType::Workspace), get_extension_str(FileExtension::PNG));


    println!("{}", img_path.as_str());
    match generate_julia_set(&descriptor, &resolution).save(img_path.clone().as_str()) {
        Ok(_) => println!("L'image du Julia Set a été sauvegardée !"),
        Err(e) => println!(
            "Erreur lors de la sauvegarde de l'image du Julia Set : {}",
            e
        ),
    }

    open_image(img_path.as_str());

}

