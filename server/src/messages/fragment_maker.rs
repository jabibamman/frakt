use std::env;
use std::error::Error;

use shared::types::fractal_descriptor::FractalType::IteratedSinZ;
use shared::types::{
    complex::Complex,
    fractal_descriptor::{FractalDescriptor, IteratedSinZDescriptor},
    messages::{FragmentRequest, FragmentResult, FragmentTask},
    point::Point,
    range::Range,
    resolution::Resolution,
    u8data::U8Data,
};
use shared::utils::env_utils::get_env_var_as_u16;

pub fn create_task_for_request(_request: FragmentRequest) -> Result<FragmentTask, Box<dyn Error>> {
    let width = get_env_var_as_u16("RESOLUTION_WIDTH")?;
    let height = get_env_var_as_u16("RESOLUTION_HEIGHT")?;
    
    Ok(FragmentTask {
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
        resolution: Resolution { nx: width, ny: height },
        range: Range {
            min: Point {
                x: -2.0,
                y: -3.55556,
            },
            max: Point { x: 2.0, y: 3.55556 },
        },
    })
}

pub fn generate_range(full_image: Range) -> Vec<Range> {
    let mut ranges = vec![];
    for y in (full_image.min.y..full_image.max.y).step_by(2) {
        for x in (full_image.min.x..full_image.min.x).step_by(2) {
            let min = Point::new(x, y);
            let max = Point::new(x + 2, y + 2);
            ranges.push(Range::new(min, max));
        }
    }
    ranges
}
