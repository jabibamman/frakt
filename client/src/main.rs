mod image;
mod julia;

use std::io;

use crate::image::open_image;
use crate::julia::generate_julia_set;

use cli::parser::{CliArgs, Parser};
use server::services::{connect::connect, reader::read_message};
use shared::types::filesystem::FileExtension;
use shared::types::fractal_descriptor::FractalType::Julia;
use shared::types::fractal_descriptor::{FractalDescriptor, JuliaDescriptor};
use shared::types::messages::FragmentTask;
use shared::types::point::Point;
use shared::types::range::Range;
use shared::types::u8data::U8Data;
use shared::types::{complex::Complex, resolution::Resolution};
use shared::utils::filesystem::{get_dir_path_buf, get_extension_str, get_file_path};

fn main() -> io::Result<()> {
    let args: CliArgs = CliArgs::parse();
    let stream = connect(format!("{}:{}", args.hostname, args.port).as_str())?;
    let message = read_message(stream);
    println!("{}", message);
    let img_path = match get_dir_path_buf() {
        Ok(dir_path_buf) => {
            match get_file_path("julia", dir_path_buf, get_extension_str(FileExtension::PNG)) {
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
            fractal_type: Julia(JuliaDescriptor {
                c: Complex {
                    re: 0.285,
                    im: 0.013,
                },
                divergence_threshold_square: 4.0,
            }),
        },
        max_iteration: 64,
        resolution: Resolution { nx: 300, ny: 300 },
        range: Range {
            min: Point { x: -1.2, y: 0.0 },
            max: Point {
                x: -0.6,
                y: 0.60000000000000001,
            },
        },
    };

    match generate_julia_set(fragment_task).save(img_path.clone().as_str()) {
        Ok(_) => println!("L'image du Julia Set a été sauvegardée !"),
        Err(e) => println!(
            "Erreur lors de la sauvegarde de l'image du Julia Set : {}",
            e
        ),
    }

    match open_image(img_path.as_str()) {
        Ok(_) => {
            println!("L'image du Julia Set a été ouverte !");
            Ok(())
        }
        Err(e) => {
            println!("Erreur lors de l'ouverture de l'image du Julia Set : {}", e);
            Err(e)
        }
    }
}
