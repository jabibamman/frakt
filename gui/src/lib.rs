#[cfg(target_os = "windows")]
use pixels::{Pixels, SurfaceTexture};
#[cfg(target_os = "windows")]
use winit::{
    platform::windows::EventLoopExtWindows,
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};

#[cfg(target_os = "windows")]
use shared::utils::env_utils::get_env_var_as_u16;
#[cfg(target_os = "windows")]
use image::{ImageBuffer, Rgb};

/// Function for creating a window and displaying an image on Windows.
/// 
/// # Arguments
/// * `image_buffer` - A reference to the buffer containing the image data in RGB format.
/// * `width` - The width of the window and the image to display.
/// * `height` - The height of the window and the image to display.
/// 
/// # Returns
/// This function returns a Result<(), Box<dyn std::error::Error>>, which indicates either
/// successful execution or an error during the process.
///
#[cfg(target_os = "windows")]
pub fn create_window_and_display_image(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {

    let width =  get_env_var_as_u16("RESOLUTION_WIDTH")?;
    let height = get_env_var_as_u16("RESOLUTION_HEIGHT")?;
    
    let event_loop: EventLoop<winit::event::VirtualKeyCode> = EventLoop::new_any_thread();
    let window = WindowBuilder::new()
        .with_title("Frakt Visualizer")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)?;

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture)?;
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
                    let index = ((y * width as u32 + x) * 4) as usize;
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