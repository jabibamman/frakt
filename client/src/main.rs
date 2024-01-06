use shared::types::complex::Complex;
use shared::utils::fragment_request_impl::FragmentRequestOperation;

mod fractal_generation;
mod image;

use std::io;

use crate::fractal_generation::generate_fractal_set;
use crate::image::open_image;

use cli::parser::{CliClientArgs, Parser};
use log::{debug, error, info};
use server::services::{connect::connect, reader::get_response, write::write};
use shared::types::filesystem::FileExtension;
use shared::types::fractal_descriptor::FractalType::Julia;
use shared::types::fractal_descriptor::{FractalDescriptor, JuliaDescriptor};
use shared::types::messages::{FragmentRequest, FragmentTask};
use shared::types::point::Point;
use shared::types::range::Range;
use shared::types::resolution::Resolution;
use shared::types::u8data::U8Data;
use shared::utils::filesystem::{get_dir_path_buf, get_extension_str, get_file_path};

fn main() -> io::Result<()> {
    shared::logger::init_logger();

    let cli_args: CliClientArgs = CliClientArgs::parse();
    let fragment_request = FragmentRequest::new(cli_args.worker_name, 1000);
    let serialized_request = fragment_request.serialize()?;
    let connection_result = connect(format!("{}:{}", cli_args.hostname, cli_args.port).as_str());

    if let Ok(mut stream) = connection_result {
        info!("Connected to the server!");
        debug!("Sending message: {}", serialized_request);
        match write(&mut stream, serialized_request.as_str()) {
            Ok(_) => info!("Message sent!"),
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
            fractal_type: Julia(JuliaDescriptor {
                c: Complex { re: 0.2, im: 1.0 },
                divergence_threshold_square: 4.0,
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
        Ok(_) => info!("L'image de la fractale a été sauvegardée !"),
        Err(e) => error!(
            "Erreur lors de la sauvegarde de l'image de la fractale : {}",
            e
        ),
    }

    if cli_args.open {
        match open_image(img_path.as_str()) {
            Ok(_) => {
                info!("L'image de la fractale a été ouverte !");
            }
            Err(e) => {
                error!(
                    "Erreur lors de l'ouverture de l'image de la fractale: {}",
                    e
                );
            }
        }
    }

    Ok(())
}
