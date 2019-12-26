#![windows_subsystem = "windows"]

use drawing::backend::Device;
use drawing::color::*;
use drawing::font::Font;
use drawing::primitive::*;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;

type DrawingDevice = drawing_gl::GlDevice;
type DrawingFont = drawing::TextureFont<DrawingDevice>;

use std::fs::File;
use std::io::Read;

fn main() {
    set_process_high_dpi_aware();
    let window_builder = winit::window::WindowBuilder::new().with_title("Title");
    let event_loop = winit::event_loop::EventLoop::new();

    let mut device = DrawingDevice::new().unwrap();
    let mut window_target = device
        .create_window_target(window_builder, &event_loop, None)
        .unwrap();
    let mut renderer = Renderer::new();

    //let image_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join("test.png").into_os_string().into_string().unwrap();
    let font_path = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap()
        .join("OpenSans-Regular.ttf")
        .into_os_string()
        .into_string()
        .unwrap();

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

    // image
    let image1_resource_id = resources.get_next_texture_id();
    let texture = create_chessboard(&mut device, 4, 4);
    resources.textures_mut().insert(image1_resource_id, texture);

    let image2_resource_id = resources.get_next_texture_id();
    let texture = create_chessboard(&mut device, 200, 200);
    resources.textures_mut().insert(image2_resource_id, texture);

    //let img = image::open(path).unwrap().to_rgba();
    //let (w, h) = img.dimensions();
    //let data: &[u8] = &img;

    //
    // main loop
    //
    let mut width = 0;
    let mut height = 0;
    let mut pos_y = 0.0f32;

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::EventsCleared => {
                // Application update code.
     
                // Queue a RedrawRequested event.
                window_target.get_window().request_redraw();
            },

            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferrable to render in this event rather than in EventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
                if width <= 0 || height <= 0 {
                    return;
                }

                pos_y += 1.0f32;

                let clipping_rect = PixelRect::new(
                    PixelPoint::new(0.0f32, 0.0f32),
                    PixelSize::new(width as f32, height as f32),
                );

                let primitives = vec![
                    Primitive::Rectangle {
                        color: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
                        rect: PixelRect::new(
                            PixelPoint::new(100.5f32, 101.5f32),
                            PixelSize::new(200.0f32, 50.0f32),
                        ),
                    },
                    Primitive::Line {
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        thickness: PixelThickness::new(1.0f32),
                        start_point: PixelPoint::new(100.0f32, 100.0f32),
                        end_point: PixelPoint::new(300.5f32, 100.5f32),
                    },
                    Primitive::Image {
                        resource_key: image2_resource_id,
                        rect: PixelRect::new(
                            PixelPoint::new(100.0f32, 150.0f32),
                            PixelSize::new(200.0f32, 200.0f32),
                        ),
                        uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
                    },
                    Primitive::Line {
                        color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                        thickness: PixelThickness::new(1.0f32),
                        start_point: PixelPoint::new(100.0f32, 350.0f32),
                        end_point: PixelPoint::new(300.0f32, 150.0f32),
                    },
                    Primitive::Line {
                        color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                        thickness: PixelThickness::new(1.0f32),
                        start_point: PixelPoint::new(100.0f32, 150.0f32),
                        end_point: PixelPoint::new(300.0f32, 350.0f32),
                    },

                    Primitive::Fill {
                        path: vec![
                            PathElement::MoveTo {
                                point: PixelPoint::new(100.0f32, 350.0f32),
                            },
                            PathElement::LineTo {
                                point: PixelPoint::new(300.0f32, 350.0f32),
                            },
                            PathElement::LineTo {
                                point: PixelPoint::new(300.0f32, 550.0f32),
                            },
                            PathElement::LineTo {
                                point: PixelPoint::new(100.0f32, 550.0f32),
                            },
                        ],
                        brush: Brush::ImagePattern {
                            resource_key: image2_resource_id,
                            center_point: PixelPoint::new(100.0f32 + pos_y / 10.0f32, 350.0f32 + pos_y / 5.0f32),
                            size: PixelSize::new(200.0f32, 200.0f32),
                            angle: pos_y / 100.0f32, // 0.0f32,
                            alpha: 1.0f32,
                        },
                    },

                    Primitive::Image {
                        resource_key: image1_resource_id,
                        rect: PixelRect::new(
                            PixelPoint::new(0.0f32, 0.0f32),
                            PixelSize::new(4.0f32, 4.0f32),
                        ),
                        uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
                    },
                    Primitive::Line {
                        color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                        thickness: PixelThickness::new(1.0f32),
                        start_point: PixelPoint::new(0.0f32, 0.0f32),
                        end_point: PixelPoint::new(4.0f32, 4.0f32),
                    },
                    Primitive::Image {
                        resource_key: image1_resource_id,
                        rect: PixelRect::new(
                            PixelPoint::new(width as f32 - 4.0f32, 0.0f32),
                            PixelSize::new(4.0f32, 4.0f32),
                        ),
                        uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
                    },
                    Primitive::Line {
                        color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                        thickness: PixelThickness::new(1.0f32),
                        start_point: PixelPoint::new(width as f32, 0.0f32),
                        end_point: PixelPoint::new(width as f32 - 4.0f32, 4.0f32),
                    },
                    Primitive::Image {
                        resource_key: image1_resource_id,
                        rect: PixelRect::new(
                            PixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
                            PixelSize::new(4.0f32, 4.0f32),
                        ),
                        uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
                    },
                    Primitive::Image {
                        resource_key: image1_resource_id,
                        rect: PixelRect::new(
                            PixelPoint::new(0.0f32, height as f32 - 4.0f32),
                            PixelSize::new(4.0f32, 4.0f32),
                        ),
                        uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
                    },
                    Primitive::Text {
                        resource_key: "F1".to_string(),
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        position: PixelPoint::new(350.0f32 + pos_y, 200.0f32),
                        clipping_rect,
                        size: 10,
                        text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                            .to_string(),
                    },
                    Primitive::Text {
                        resource_key: "F1".to_string(),
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        position: PixelPoint::new(350.0f32, 220.0f32 - pos_y),
                        clipping_rect,
                        size: 12,
                        text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                            .to_string(),
                    },
                    Primitive::Text {
                        resource_key: "F1".to_string(),
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        position: PixelPoint::new(350.0f32 - pos_y, 240.0f32 + pos_y * 2.0f32),
                        clipping_rect,
                        size: 14,
                        text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                            .to_string(),
                    },
                    Primitive::Text {
                        resource_key: "F1".to_string(),
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        position: PixelPoint::new(350.0f32 - pos_y, 260.0f32),
                        clipping_rect,
                        size: 16,
                        text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                            .to_string(),
                    },
                    Primitive::Text {
                        resource_key: "F1".to_string(),
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        position: PixelPoint::new(350.0f32 + pos_y, 280.0f32 + pos_y),
                        clipping_rect,
                        size: 18,
                        text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                            .to_string(),
                    },

                    Primitive::Fill {
                        path: vec![
                            PathElement::MoveTo {
                                point: PixelPoint::new(100.0f32, 350.0f32),
                            },
                            PathElement::BezierTo {
                                point: PixelPoint::new(300.0f32, 150.0f32),
                                c1: PixelPoint::new(120.0f32, 50.0f32),
                                c2: PixelPoint::new(180.0f32, 50.0f32),
                            }
                        ],
                        brush: Brush::LinearGradient {
                            start_point: PixelPoint::new(100.0f32, 150.0f32),
                            end_point: PixelPoint::new(350.0f32, 350.0f32),
                            inner_color: [1.0f32, 0.0f32, 0.0f32, 0.75f32],
                            outer_color: [1.0f32, 1.0f32, 0.0f32, 0.75f32],
                        },
                    },

                    Primitive::Fill {
                        path: vec![
                            PathElement::MoveTo {
                                point: PixelPoint::new(500.0f32, 350.0f32),
                            },
                            PathElement::BezierTo {
                                point: PixelPoint::new(700.0f32, 150.0f32),
                                c1: PixelPoint::new(520.0f32, 50.0f32),
                                c2: PixelPoint::new(580.0f32, 50.0f32),
                            },
                            PathElement::ClosePath,

                            PathElement::MoveTo {
                                point: PixelPoint::new(550.0f32, 250.0f32),
                            },
                            PathElement::BezierTo {
                                point: PixelPoint::new(650.0f32, 180.0f32),
                                c1: PixelPoint::new(580.0f32, 150.0f32),
                                c2: PixelPoint::new(620.0f32, 150.0f32),
                            },
                            PathElement::ClosePath,
                            PathElement::Solidity { solidity: Solidity::Hole },
                        ],
                        brush: Brush::LinearGradient {
                            start_point: PixelPoint::new(500.0f32, 150.0f32),
                            end_point: PixelPoint::new(750.0f32, 350.0f32),
                            inner_color: [1.0f32, 0.0f32, 0.0f32, 0.75f32],
                            outer_color: [1.0f32, 1.0f32, 0.0f32, 0.75f32],
                        },
                    },

                    Primitive::Stroke {
                        path: vec![
                            PathElement::MoveTo {
                                point: PixelPoint::new(300.0f32, 550.0f32),
                            },
                            PathElement::BezierTo {
                                point: PixelPoint::new(500.0f32, 350.0f32),
                                c1: PixelPoint::new(320.0f32, 250.0f32),
                                c2: PixelPoint::new(380.0f32, 250.0f32),
                            },
                            PathElement::ClosePath,
                        ],
                        thickness: 1.0f32,
                        //brush: Brush::Color { color: [1.0f32, 1.0f32, 0.0f32, 0.75f32] },
                        brush: Brush::LinearGradient {
                            start_point: PixelPoint::new(200.0f32, 450.0f32),
                            end_point: PixelPoint::new(450.0f32, 650.0f32),
                            inner_color: [1.0f32, 0.0f32, 0.0f32, 0.75f32],
                            outer_color: [1.0f32, 1.0f32, 0.0f32, 0.75f32],
                        },
                    },

                    // render target test
                    Primitive::Composite {
                        color: [1.0f32, 1.0f32, 1.0f32, 0.5f32],
                        primitives: vec![
                            Primitive::Rectangle {
                                color: [0.0f32, 0.5f32, 0.3f32, 1.0f32],
                                rect: PixelRect::new(
                                    PixelPoint::new(200.5f32, 220.5f32),
                                    PixelSize::new(200.0f32, 50.0f32),
                                ),
                            },
                            Primitive::Text {
                                resource_key: "F1".to_string(),
                                color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                                position: PixelPoint::new(207.0f32, 232.0f32),
                                clipping_rect,
                                size: 22,
                                text: "Render target test".to_string(),
                            },
                        ],
                    }
                ];

                device.begin(&window_target);
                device.clear(
                    window_target.get_render_target(),
                    &[0.5f32, 0.4f32, 0.3f32, 1.0f32],
                );
                renderer
                    .draw(
                        &mut device,
                        window_target.get_render_target(),
                        &primitives,
                        &mut resources,
                        false,
                    )
                    .unwrap();
                device.end(&window_target);

                window_target.swap_buffers();
            },

            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    }
                    | winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::Resized(logical_size) => {
                        let physical_size =
                            logical_size.to_physical(window_target.get_window().hidpi_factor());
                        width = physical_size.width as u16;
                        height = physical_size.height as u16;
                        window_target.update_size(width, height)
                    }
                    _ => (),
                }
            },

            _ => {},
        }
    });
}

pub fn create_chessboard<D: Device>(device: &mut D, w: usize, h: usize) -> D::Texture {
    let mut data: Vec<u8> = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let color: u8 = if ((x + y) / 1 % 2) == 0 {
                255 - x as u8
            } else {
                0
            };
            data.push(color);
            data.push(color);
            data.push(color);
            data.push(255);
        }
    }

    device
        .create_texture(Some(&data), w as u16, h as u16, ColorFormat::RGBA, false)
        .unwrap()
}

// Helper function to dynamically load a function pointer and call it.
// The result of the callback is forwarded.
#[cfg(windows)]
fn try_get_function_pointer<F>(
    dll: &str,
    name: &str,
    callback: &Fn(&F) -> Result<(), ()>,
) -> Result<(), ()> {
    use shared_library::dynamic_library::DynamicLibrary;
    use std::path::Path;

    // Try to load the function dynamically.
    let lib = DynamicLibrary::open(Some(Path::new(dll))).map_err(|_| ())?;

    let func_ptr = unsafe { lib.symbol::<F>(name).map_err(|_| ())? };

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
            let result = unsafe { SetProcessDPIAware() };

            match result {
                0 => Err(()),
                _ => Ok(()),
            }
        },
    );
}

/// This function only works on Windows.
#[cfg(not(windows))]
pub fn set_process_high_dpi_aware() {}
