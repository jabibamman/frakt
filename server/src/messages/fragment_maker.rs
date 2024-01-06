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

pub fn create_task_for_request(_request: FragmentRequest) -> FragmentTask {
    FragmentTask {
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
    }
}

pub fn process_result(_result: FragmentResult) {
    todo!()
}
