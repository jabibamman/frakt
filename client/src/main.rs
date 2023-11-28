mod fractal_generation;
mod image;

use std::io;

use crate::image::open_image;
use crate::fractal_generation::generate_fractal_set;

use cli::parser::{CliArgs, Parser};
use server::services::{connect::connect, reader::read_message};
use shared::types::complex::Complex;
use shared::types::filesystem::FileExtension;
use shared::types::fractal_descriptor::FractalType::{Mandelbrot, NewtonRaphsonZ3, NewtonRaphsonZ4};
use shared::types::fractal_descriptor::{FractalDescriptor, MandelbrotDescriptor, NewtonRaphsonZ3Descriptor, NewtonRaphsonZ4Descriptor};
use shared::types::messages::FragmentTask;
use shared::types::point::Point;
use shared::types::range::Range;
use shared::types::u8data::U8Data;
use shared::types::resolution::Resolution;
use shared::utils::filesystem::{get_dir_path_buf, get_extension_str, get_file_path};

fn main() -> io::Result<()> {
    /*let args: CliArgs = CliArgs::parse();
    let stream = connect(format!("{}:{}", args.hostname, args.port).as_str())?;
    let message = read_message(stream);
    println!("{}", message);*/
    let img_path = match get_dir_path_buf() {
        Ok(dir_path_buf) => {
            match get_file_path("newtonZ3", dir_path_buf, get_extension_str(FileExtension::PNG)) {
                Ok(img_path) => img_path,
                Err(e) => {
                    eprintln!(
                        "Erreur lors de la récupération du chemin du fichier : {}",
                        e
                    );
                    return Ok(());
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la récupération du répertoire : {}", e);
            return Ok(());
        }
    };

    let fragment_task: FragmentTask = FragmentTask {
        id: U8Data {
            offset: 0,
            count: 16,
        },
        fractal: FractalDescriptor {
            fractal_type:NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor{
                //c: Complex { re: 0.2, im: 1.0 },
            }),
        },
        max_iteration: 64,
        resolution: Resolution { nx: 1080, ny: 1920 },
        range: Range {
            min: Point {
                x: -2.0,
                y: -3.55556,
            },
            max: Point { x: 2.0, y: 3.55556 },
        },
    };

    match generate_fractal_set(fragment_task).save(img_path.clone().as_str()) {
        Ok(_) => println!("L'image de la fractale a été sauvegardée !"),
        Err(e) => println!(
            "Erreur lors de la sauvegarde de l'image de la fractale : {}",
            e
        ),
    }

    match open_image(img_path.as_str()) {
        Ok(_) => {
            println!("L'image de la fractale a été ouverte !");
            Ok(())
        }
        Err(e) => {
            println!("Erreur lors de l'ouverture de l'image de la fractale : {}", e);
            Err(e)
        }
    }
}
