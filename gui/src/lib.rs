#[cfg(target_os = "windows")]
use pixels::{Pixels, SurfaceTexture};
#[cfg(target_os = "windows")]
use winit::{
    platform::windows::EventLoopExtWindows,
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};

use shared::utils::env_utils::get_env_var_as_u16;

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use gtk::{Application, ApplicationWindow, Image};
#[cfg(target_os = "linux")]
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};

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

/// Function for creating a window and displaying an image on Linux.
/// 
/// # Arguments
/// * `image_buffer` - A reference to the buffer containing the image data in RGB format.
/// * `width` - The width of the window and the image to display.
/// * `height` - The height of the window and the image to display.
/// 
/// This function does not return a value but directly runs the GTK application loop to create the window and display the image.
///
///
#[cfg(target_os = "linux")]
pub fn create_window_and_display_image(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32) {
    let application = Application::builder()
        .application_id("frakt")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Fractal Image")
            .default_width(600)
            .default_height(600)
            .build();

        let image_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let image = convert_image_buffer_to_pixbuf(image_buffer, width, height);
        let image_widget = Image::new_from_pixbuf(Some(&image));
        image_container.add(&image_widget);

        window.set_child(Some(&image_container));

        window.show();
    });

    application.run();
}

#[cfg(target_os = "linux")]
fn convert_image_buffer_to_pixbuf(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32) -> Pixbuf {
    let mut pixbuf = Pixbuf::new(
        Colorspace::Rgb,
        false,
        8,
        width as i32,
        height as i32,
    )
        .unwrap();
    pixbuf.fill(0);

    for y in 0..height {
        for x in 0..width {
            let pixel = image_buffer.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            pixbuf.put_pixel(x, y, r, g, b, 255);
        }
    }

    pixbuf
}
