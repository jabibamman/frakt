use pixels::{Pixels, SurfaceTexture};
use winit::{
    platform::windows::EventLoopExtWindows, 
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};
use image::{ImageBuffer, Rgb};
use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn fill_image_buffer_from_file(image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(Path::new(file_path))?;
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some(rgb_values) = line.strip_prefix("Rgb(").and_then(|s| s.strip_suffix(")")) {
            let rgb: Vec<u8> = rgb_values
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect();

            if rgb.len() == 3 {
                let x = y as u32 % width;
                let y = y as u32 / width;

                if x < width && y < height {
                    let pixel = image_buffer.get_pixel_mut(x, y);
                    *pixel = Rgb([rgb[0], rgb[1], rgb[2]]);
                }
            }
        }
    }

    Ok(())
}

pub fn create_window_and_display_image(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop: EventLoop<winit::event::VirtualKeyCode> = EventLoop::new_any_thread();
    let window = WindowBuilder::new()
        .with_title("Affichage Pixel par Pixel")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)?;

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)?;
    let cloned_image_buffer = image_buffer.clone();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame_mut();

                for (x, y, pixel) in cloned_image_buffer.enumerate_pixels() {
                    let index = ((y * width + x) * 4) as usize;
                    if index + 3 < frame.len() {
                        frame[index] = pixel[0];
                        frame[index + 1] = pixel[1];
                        frame[index + 2] = pixel[2];
                        frame[index + 3] = 0xff;
                    }
                }

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => {}
        }

        if let Event::WindowEvent { event, .. } = event {
            if matches!(event, WindowEvent::CloseRequested | WindowEvent::KeyboardInput { .. }) {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}



