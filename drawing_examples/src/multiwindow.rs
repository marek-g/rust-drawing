#![windows_subsystem="windows"]
extern crate winit;
extern crate drawing;
extern crate drawing_gl;
extern crate shared_library;
extern crate find_folder;

use drawing::backend::Device;
use drawing::backend::WindowTarget;
use drawing::color::*;
use drawing::font::Font;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::primitive::Primitive;
use drawing::units::*;

type DrawingDevice = drawing_gl::GlDevice;
type DrawingFont = drawing::TextureFont<DrawingDevice>;

use winit::dpi::LogicalSize;

use std::fs::File;
use std::io::Read;

fn main() {
    set_process_high_dpi_aware();

    let mut events_loop = winit::EventsLoop::new(); 

    let mut device = DrawingDevice::new().unwrap();
    let mut renderer = Renderer::new();

    let window_builder1 = winit::WindowBuilder::new().with_title("Window 1");
    let mut window_target1 = device.create_window_target(window_builder1, &events_loop).unwrap();

    let window_builder2 = winit::WindowBuilder::new().with_title("Window 2");
    let mut window_target2 = device.create_window_target(window_builder2, &events_loop).unwrap();

    let font_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join("OpenSans-Regular.ttf").into_os_string().into_string().unwrap();


    //
    // create resources
    //
    let mut resources = Resources::new();

    // font
    let mut file = File::open(font_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let font = DrawingFont::create(&mut device, buffer).unwrap();

    resources.fonts_mut().insert("F1".to_string(), font);


    //
    // main loop
    //
    let mut running = true;
    events_loop.run_forever(|event| {
        let mut refresh = false;

        if let winit::Event::WindowEvent { ref window_id, ref event } = event {
            let mut window_target = if window_id == &window_target1.get_window().id() {
                    &mut window_target1
                } else {
                    &mut window_target2
                };

            match event {
                winit::WindowEvent::CloseRequested => {
                    running = false;
                },

                winit::WindowEvent::Refresh => {
                    refresh = true;
                },

                winit::WindowEvent::Resized(logical_size) => {
                    let physical_size = logical_size.to_physical(window_target.get_window().get_hidpi_factor());
                    window_target.update_size(physical_size.width as u16, physical_size.height as u16)
                },
                
                _ => ()
            }
        };

        if running && refresh
        {
            // window 1
            draw_window(&mut device, &mut renderer, &mut resources,
                &window_target1, "Window 1");
            draw_window(&mut device, &mut renderer, &mut resources,
                &window_target2, "Window 2");

            window_target1.swap_buffers();
            window_target2.swap_buffers();
        }

        if running { winit::ControlFlow::Continue } else { winit::ControlFlow::Break }
    });
}

fn draw_window(device: &mut DrawingDevice, renderer: &mut Renderer, resources: &mut Resources<DrawingDevice, DrawingFont>,
    window_target: &<DrawingDevice as drawing::backend::Device>::WindowTarget, text: &str) {
    let logical_size = window_target.get_window().get_inner_size().unwrap_or(LogicalSize::new(0.0, 0.0));
    let physical_size = logical_size.to_physical(window_target.get_window().get_hidpi_factor());
    let width = physical_size.width as f32;
    let height = physical_size.height as f32;

    if width > 0.0 && height > 0.0 {
        let primitives = vec![
            Primitive::Text { resource_key: "F1".to_string(), color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                position: UserPixelPoint::new(width / 2.0f32, height / 2.0f32),
                size: 20,
                text: text.to_string(),
            },
        ];

        device.begin(window_target);
        device.clear(window_target.get_render_target(), &[0.5f32, 0.4f32, 0.3f32, 1.0f32]);
        renderer.draw(device, window_target.get_render_target(),
        PhysPixelSize::new(width, height), primitives, resources).unwrap();
        device.end(window_target);
    }
}

// Helper function to dynamically load a function pointer and call it.
// The result of the callback is forwarded.
#[cfg(windows)]
fn try_get_function_pointer<F>(dll: &str, name: &str, callback: &Fn(&F) -> Result<(), ()>) -> Result<(), ()> {
    use shared_library::dynamic_library::DynamicLibrary;
    use std::path::Path;

    // Try to load the function dynamically.
    let lib = DynamicLibrary::open(Some(Path::new(dll))).map_err(|_| ())?;

    let func_ptr = unsafe {
        lib.symbol::<F>(name).map_err(|_| ())?
    };

    let func = unsafe { std::mem::transmute(&func_ptr) };

    callback(func)
}

#[cfg(windows)]
pub fn set_process_high_dpi_aware() {
    let _result = try_get_function_pointer::<unsafe extern "system" fn() -> u32>(
        "User32.dll",
        "SetProcessDPIAware",
        &|SetProcessDPIAware| {
            // See https://msdn.microsoft.com/en-us/library/windows/desktop/ms633543(v=vs.85).aspx
            let result = unsafe {
                SetProcessDPIAware()
            };

            match result {
                0 => Err(()),
                _ => Ok(())
            }
        }
    );
}

/// This function only works on Windows.
#[cfg(not(windows))]
pub fn set_process_high_dpi_aware() {
}