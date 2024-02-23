use std::error::Error;

use shared::types::fractal_descriptor::FractalType::IteratedSinZ;
use shared::types::{
    complex::Complex,
    fractal_descriptor::{FractalDescriptor, IteratedSinZDescriptor},
    messages::{FragmentResult, FragmentTask},
    point::Point,
    range::Range,
    resolution::Resolution,
    u8data::U8Data,
};
use shared::utils::env_utils::get_env_var_as_u16;

pub fn create_tasks() -> Result<Vec<FragmentTask>, Box<dyn Error>> {
    let width = get_env_var_as_u16("RESOLUTION_WIDTH")?;
    let height = get_env_var_as_u16("RESOLUTION_HEIGHT")?;

    let range = generate_range(
        Range::new(Point::new(-3.0, -3.0), Point::new(3.0, 3.0)),
        2.0,
    );
    let mut tasks = vec![];

    for r in range {
        let task = FragmentTask {
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
            resolution: Resolution {
                nx: width,
                ny: height,
            },
            range: r,
        };

        tasks.push(task);
    }

    Ok(tasks)
}

pub fn process_result(_result: FragmentResult) {}

pub fn generate_range(full_image: Range, step: f64) -> Vec<Range> {
    let mut ranges = Vec::new();
    let y_step = step;
    let x_step = step;

    let mut y = full_image.min.y;
    while y < full_image.max.y {
        let mut x = full_image.min.x;
        while x < full_image.max.x {
            let min = Point::new(x, y);
            let max = Point::new(
                (x + x_step).min(full_image.max.x),
                (y + y_step).min(full_image.max.y),
            );
            ranges.push(Range::new(min, max));
            x += x_step;
        }
        y += y_step;
    }
    ranges
}
