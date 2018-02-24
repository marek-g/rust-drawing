extern crate winit;
extern crate drawing;
extern crate drawing_gfx;
extern crate shared_library;
extern crate find_folder;

use drawing::backend::WindowBackend;
use drawing::renderer::Renderer;
use drawing::primitive::Primitive;
use drawing::units::*;
use drawing_gfx::backend::GfxBackend;
use drawing_gfx::backend::GfxWindowBackend;

fn main() {
    set_process_high_dpi_aware();
    let window_builder = winit::WindowBuilder::new()
        .with_title("Title");
    let mut events_loop = winit::EventsLoop::new(); 


    let mut renderer = Renderer::new(GfxWindowBackend::create_window_backend(window_builder, &events_loop));
    let image_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join("test.png").into_os_string().into_string().unwrap();

    // main loop
    let mut running = true;
    let mut width = 0;
    let mut height = 0;
    while running {
        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                match event {
                    winit::WindowEvent::KeyboardInput {
                        input: winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                            .. },
                        ..
                    } | winit::WindowEvent::Closed => running = false,
                    winit::WindowEvent::Resized(w, h) => {
                        width = w; height = h;
                        renderer.update_window_size(w as u16, h as u16)
                    }
                    _ => (),
                }
            }
        });

        if width <= 0 || height <= 0 { continue }

        let primitives = vec![
            Primitive::Rectangle { color: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
                rect: UserPixelRect::new(UserPixelPoint::new(100.5f32, 101.5f32),
                    UserPixelSize::new(200.0f32, 50.0f32)) },
            Primitive::Line { color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(100.5f32, 100.5f32),
                end_point: UserPixelPoint::new(300.5f32, 100.5f32) },
            Primitive::Image { path: &image_path, rect: UserPixelRect::new(
                UserPixelPoint::new(100.5f32, 150.0f32),
                UserPixelSize::new(200.0f32, 200.0f32)
            )}
        ];
        renderer.draw(PhysPixelSize::new(width as f32, height as f32), primitives);
    }
}

type Result = std::result::Result<(), ()>;

// Helper function to dynamically load a function pointer and call it.
// The result of the callback is forwarded.
#[cfg(windows)]
fn try_get_function_pointer<F>(dll: &str, name: &str, callback: &Fn(&F) -> Result) -> Result {
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