mod fractal_generation;
mod image;

use std::io;

use crate::fractal_generation::generate_fractal_set;
use crate::image::open_image;

use cli::operation::parse_to_address;
use cli::parser::{CliArgs, CliClientArgs, Parser};
use server::services::{connect::connect, reader::get_response, write::write};
use server::messages::serialization::serialize_request;
use shared::types::filesystem::FileExtension;
use shared::types::fractal_descriptor::FractalType::IteratedSinZ;
use shared::types::fractal_descriptor::{FractalDescriptor, IteratedSinZDescriptor};
use shared::types::messages::{FragmentTask, FragmentRequest};
use shared::types::point::Point;
use shared::types::range::Range;
use shared::types::u8data::U8Data;
use shared::types::{complex::Complex, resolution::Resolution};
use shared::utils::filesystem::{get_dir_path_buf, get_extension_str, get_file_path};
use log::{info, error, debug};

fn main() -> io::Result<()> {
    shared::logger::init_logger();
 
    let cli_args: CliArgs = CliArgs::Client(CliClientArgs::parse());
    let connection_result: Result<std::net::TcpStream, io::Error> = connect(&parse_to_address(cli_args));

    let fragment_request = FragmentRequest {
        worker_name: "Worker 1".to_string(),
        maximal_work_load: 1000,
    };

    let serialized_request = match serialize_request(&fragment_request) {
        Ok(serialized_request) => serialized_request,
        Err(e) => {
            error!("Erreur lors de la sérialisation de la requête : {}", e);
            return Ok(());
        }
    };
 
    if let Ok(mut stream) = connection_result {
        info!("Connected to the server!");
        match write(&mut stream, &serialized_request) {
            Ok(_) => info!(" Message sent!"),
            Err(error) => error!("Failed to send message, {}", error),
        }

        let response = get_response(&mut stream)?;
        debug!("Response received: {:?}", response);
    } else if let Err(e) = connection_result {
        error!("Failed to connect to the server: {}", e);
    }

    let img_path = match get_dir_path_buf() {
        Ok(dir_path_buf) => {
            match get_file_path("julia", dir_path_buf, get_extension_str(FileExtension::PNG)) {
                Ok(img_path) => img_path,
                Err(e) => {
                    error!(
                        "Erreur lors de la récupération du chemin du fichier : {}",
                        e
                    );
                    return Ok(());
                }
            }
        }
        Err(e) => {
            error!("Erreur lors de la récupération du répertoire : {}", e);
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
        Ok(_) => info!("L'image du Julia Set a été sauvegardée !"),
        Err(e) => error!(
            "Erreur lors de la sauvegarde de l'image du Julia Set : {}",
            e
        ),
    }

    match open_image(img_path.as_str()) {
        Ok(_) => {
            info!("L'image du Julia Set a été ouverte !");
            Ok(())
        }
        Err(e) => {
            error!("Erreur lors de l'ouverture de l'image du Julia Set : {}", e);
            Err(e)
        }
    }
}
