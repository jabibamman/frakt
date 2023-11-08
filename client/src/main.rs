mod image;
mod julia;

use crate::image::open_image;
use crate::julia::generate_julia_set;

use shared::types::filesystem::FileExtension;
use shared::types::fractal_descriptor::FractalType::Julia;
use shared::types::fractal_descriptor::{FractalDescriptor, JuliaDescriptor};
use shared::types::messages::FragmentTask;
use shared::types::point::Point;
use shared::types::range::Range;
use shared::types::u8data::U8Data;
use shared::types::{complex::Complex, resolution::Resolution};
use shared::utils::filesystem::{get_dir_str, get_extension_str, get_file_path};

fn main() {
    let img_path = get_file_path("julia", get_dir_str(), get_extension_str(FileExtension::PNG));
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

    open_image(img_path.as_str());
}
