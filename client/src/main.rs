mod fractal_generation;
mod image;

use std::io;

use crate::fractal_generation::generate_fractal_set;
use crate::image::open_image;

use cli::parser::{CliArgs, Parser};
use server::services::{connect::connect, reader::get_response, write::write};
use shared::types::filesystem::FileExtension;
use shared::types::fractal_descriptor::FractalType::IteratedSinZ;
use shared::types::fractal_descriptor::{FractalDescriptor, IteratedSinZDescriptor};
use shared::types::messages::FragmentTask;
use shared::types::point::Point;
use shared::types::range::Range;
use shared::types::u8data::U8Data;
use shared::types::{complex::Complex, resolution::Resolution};
use shared::utils::filesystem::{get_dir_path_buf, get_extension_str, get_file_path};

fn main() -> io::Result<()> {
    let args: CliArgs = CliArgs::parse();
    let mut stream = connect(format!("{}:{}", args.hostname, args.port).as_str())?;
    println!("Connecté au serveur !");

    let writed = write(&mut stream, "Hello World !");
    match writed {
        Ok(_) => {
            println!("Message envoyé !")
        }
        Err(error) => {
            println!("Échec de l'envoie du message : {}", error)
        }
    }
    let response = get_response(&mut stream)?;
    println!("Réponse reçue: {:?}", response);

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
            fractal_type: IteratedSinZ(IteratedSinZDescriptor {
                c: Complex { re: 0.2, im: 1.0 },
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
