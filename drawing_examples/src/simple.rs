#![windows_subsystem="windows"]
extern crate winit;
extern crate drawing;
extern crate drawing_gfx;
extern crate shared_library;
extern crate find_folder;

use drawing::backend::Backend;
use drawing::backend::WindowBackend;
use drawing::font::Font;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::primitive::Primitive;
use drawing::units::*;
use drawing_gfx::backend::GfxWindowBackend;
use drawing_gfx::font_gfx_text::GfxTextFont;

use std::fs::File;
use std::io::Read;

fn main() {
    set_process_high_dpi_aware();
    let window_builder = winit::WindowBuilder::new()
        .with_title("Title");
    let mut events_loop = winit::EventsLoop::new(); 


    let mut renderer = Renderer::new(GfxWindowBackend::create_window_backend(window_builder, &events_loop));

    let image_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join("test.png").into_os_string().into_string().unwrap();
    let font_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join("OpenSans-Regular.ttf").into_os_string().into_string().unwrap();


    //
    // create resources
    //
    let mut resources = Resources::new();

    // font
    let mut file = File::open(font_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let font = GfxTextFont::create(renderer.backend(), buffer);

    resources.fonts_mut().insert("F1".to_string(), font);

    // image
    let image1_resource_id = resources.get_next_texture_id();
    let texture = create_chessboard(renderer.backend(), 4, 4);
    resources.textures_mut().insert(image1_resource_id, texture);

    let image2_resource_id = resources.get_next_texture_id();
    let texture = create_chessboard(renderer.backend(), 200, 200);
    resources.textures_mut().insert(image2_resource_id, texture);

    //let img = image::open(path).unwrap().to_rgba();
	//let (w, h) = img.dimensions();
	//let data: &[u8] = &img;


    //
    // main loop
    //
    let mut running = true;
    let mut width = 0;
    let mut height = 0;
    let mut pos_y = 0.0f32;
    while running {
        pos_y += 1.0f32;

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

        if running == false { return }

        let primitives = vec![
            Primitive::Rectangle { color: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
                rect: UserPixelRect::new(UserPixelPoint::new(100.5f32, 101.5f32),
                    UserPixelSize::new(200.0f32, 50.0f32)) },
            Primitive::Line { color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(100.0f32, 100.0f32),
                end_point: UserPixelPoint::new(300.5f32, 100.5f32) },
            
            Primitive::Image { resource_key: image2_resource_id, rect: UserPixelRect::new(
                UserPixelPoint::new(100.0f32, 150.0f32),
                UserPixelSize::new(200.0f32, 200.0f32),
            )},
            Primitive::Line { color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(100.0f32, 350.0f32),
                end_point: UserPixelPoint::new(300.0f32, 150.0f32) },
            Primitive::Line { color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(100.0f32, 150.0f32),
                end_point: UserPixelPoint::new(300.0f32, 350.0f32) },

            Primitive::Image { resource_key: image1_resource_id, rect: UserPixelRect::new(
                UserPixelPoint::new(0.0f32, 0.0f32),
                UserPixelSize::new(4.0f32, 4.0f32),
            )},
            Primitive::Line { color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(0.0f32, 0.0f32),
                end_point: UserPixelPoint::new(4.0f32, 4.0f32) },
            Primitive::Image { resource_key: image1_resource_id, rect: UserPixelRect::new(
                UserPixelPoint::new(width as f32 - 4.0f32, 0.0f32),
                UserPixelSize::new(4.0f32, 4.0f32),
            )},
            Primitive::Line { color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(width as f32, 0.0f32),
                end_point: UserPixelPoint::new(width as f32 - 4.0f32, 4.0f32) },
            Primitive::Image { resource_key: image1_resource_id, rect: UserPixelRect::new(
                UserPixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
                UserPixelSize::new(4.0f32, 4.0f32),
            )},
            Primitive::Image { resource_key: image1_resource_id, rect: UserPixelRect::new(
                UserPixelPoint::new(0.0f32, height as f32 - 4.0f32),
                UserPixelSize::new(4.0f32, 4.0f32),
            )},

            Primitive::Text { resource_key: "F1", color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                position: UserPixelPoint::new(350.0f32 + pos_y, 200.0f32),
                size: 10,
                text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz".to_string(),
            },
            Primitive::Text { resource_key: "F1", color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                position: UserPixelPoint::new(350.0f32, 220.0f32 - pos_y),
                size: 12,
                text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz".to_string(),
            },
            Primitive::Text { resource_key: "F1", color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                position: UserPixelPoint::new(350.0f32 - pos_y, 240.0f32 + pos_y*2.0f32),
                size: 14,
                text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz".to_string(),
            },
            Primitive::Text { resource_key: "F1", color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                position: UserPixelPoint::new(350.0f32 - pos_y, 260.0f32),
                size: 16,
                text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz".to_string(),
            },
            Primitive::Text { resource_key: "F1", color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                position: UserPixelPoint::new(350.0f32 + pos_y, 280.0f32 + pos_y),
                size: 18,
                text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz".to_string(),
            },
        ];
        renderer.draw(PhysPixelSize::new(width as f32, height as f32), primitives, &mut resources);
    }
}

pub fn create_chessboard<B: Backend>(backend: &mut B, w: usize, h: usize) -> B::Texture {
    let mut data: Vec<u8> = Vec::with_capacity(w*h*4);
    for y in 0..h {
        for x in 0..w {
            let color: u8 = if ((x + y)/1 % 2) == 0 { 255 - x as u8 } else { 0 };
            data.push(color);
            data.push(color);
            data.push(color);
            data.push(255);
        }
    }

    backend.create_texture(&data, w as u16, h as u16)
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