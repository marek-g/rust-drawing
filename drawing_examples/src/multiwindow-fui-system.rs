#![windows_subsystem = "windows"]

use drawing::backend::Device;
use drawing::color::*;
use drawing::font::Font;
use drawing::primitive::*;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;

use std::fs::File;
use std::io::Read;

use drawing::TextureFont;
use drawing_gl::{GlContextData, GlDevice, GlRenderTarget};
use euclid::{Angle, Vector2D};
use std::cell::RefCell;
use std::rc::Rc;

use fui_system::{Application, ApplicationOptionsBuilder};
use gl::types::*;

type DrawingDevice = drawing_gl::GlDevice;
type DrawingFont = drawing::TextureFont<DrawingDevice>;

pub struct GlWindow {
    pub window: fui_system::Window,
    pub gl_context_data: Option<GlContextData>,

    pub time_query: GLuint,
    pub pos_y: f32,
}

pub struct AppResources {
    pub initialized: bool,

    pub resources: Resources<DrawingDevice, TextureFont<DrawingDevice>>,

    pub image1_resource_id: i32,
    pub image2_resource_id: i32,
}

impl AppResources {
    pub fn new() -> Self {
        AppResources {
            initialized: false,
            resources: Resources::new(),
            image1_resource_id: 0,
            image2_resource_id: 0,
        }
    }
}

fn main() {
    set_process_high_dpi_aware();

    let app = Application::new(
        ApplicationOptionsBuilder::new()
            .with_title("Example: tray")
            .with_opengl_share_contexts(true)
            .with_opengl_stencil_bits(8)
            .build(),
    );

    let device_rc = Rc::new(RefCell::new(DrawingDevice::new().unwrap()));
    let app_resources_rc = Rc::new(RefCell::new(AppResources::new()));

    let gl_window1_rc = Rc::new(RefCell::new(GlWindow {
        window: fui_system::Window::new(None).unwrap(),
        gl_context_data: None,
        time_query: 0,
        pos_y: 0.0f32,
    }));

    let gl_window2_rc = Rc::new(RefCell::new(GlWindow {
        window: fui_system::Window::new(None).unwrap(),
        gl_context_data: None,
        time_query: 0,
        pos_y: 0.0f32,
    }));

    gl_window1_rc.borrow_mut().window.set_title("Window 1");
    gl_window2_rc.borrow_mut().window.set_title("Window 2");

    setup_window(&gl_window1_rc, &device_rc, &app_resources_rc);
    setup_window(&gl_window2_rc, &device_rc, &app_resources_rc);

    gl_window1_rc.borrow_mut().window.set_visible(true);
    gl_window2_rc.borrow_mut().window.set_visible(true);

    fui_system::Application::message_loop();
}

fn setup_window(
    gl_window_rc: &Rc<RefCell<GlWindow>>,
    device_rc: &Rc<RefCell<GlDevice>>,
    app_resources_rc: &Rc<RefCell<AppResources>>,
) {
    let mut window = &mut gl_window_rc.borrow_mut().window;
    window.resize(800, 600);

    window.on_initialize_gl({
        let gl_window_clone = gl_window_rc.clone();
        let device_clone = device_rc.clone();
        let app_resources_clone = app_resources_rc.clone();

        move || {
            {
                // initialize gl context
                let mut gl_window = gl_window_clone.borrow_mut();
                gl_window.gl_context_data =
                    Some(device_clone.borrow_mut().init_context(|symbol| {
                        gl_window.window.get_opengl_proc_address(symbol).unwrap()
                    }));

                initialize_resources(
                    &mut app_resources_clone.borrow_mut(),
                    &mut device_clone.borrow_mut(),
                );
            }
        }
    });

    window.on_paint_gl({
        let gl_window_clone = gl_window_rc.clone();
        let device_clone = device_rc.clone();
        let app_resources_clone = app_resources_rc.clone();

        move || unsafe {
            draw(
                &mut device_clone.borrow_mut(),
                &mut gl_window_clone.borrow_mut(),
                &mut app_resources_clone.borrow_mut(),
            );

            // continue animation
            gl_window_clone.borrow_mut().window.update();
        }
    });
}

fn initialize_resources(app_resources: &mut AppResources, device: &mut DrawingDevice) {
    if app_resources.initialized {
        return;
    }

    //let image_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join("test.png").into_os_string().into_string().unwrap();
    let font_path = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap()
        .join("OpenSans-Regular.ttf")
        .into_os_string()
        .into_string()
        .unwrap();

    // font
    let mut file = File::open(font_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let font = DrawingFont::create(device, buffer).unwrap();

    app_resources
        .resources
        .fonts_mut()
        .insert("F1".to_string(), font);

    // image
    app_resources.image1_resource_id = app_resources.resources.get_next_texture_id();
    let texture = create_chessboard(device, 4, 4);
    app_resources
        .resources
        .textures_mut()
        .insert(app_resources.image1_resource_id, texture);

    app_resources.image2_resource_id = app_resources.resources.get_next_texture_id();
    let texture = create_chessboard(device, 200, 200);
    app_resources
        .resources
        .textures_mut()
        .insert(app_resources.image2_resource_id, texture);

    app_resources.initialized = true;
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

pub fn draw(
    device: &mut DrawingDevice,
    gl_window: &mut GlWindow,
    app_resources: &mut AppResources,
) {
    let width = gl_window.window.get_width();
    let height = gl_window.window.get_height();

    if width <= 0 || height <= 0 {
        return;
    }

    // TODO: make Render methods static
    let mut renderer = Renderer::new();

    // TODO: how do we know that framebuffer id 0 is ok?
    let render_target = GlRenderTarget::new(0, width as u16, height as u16, 1.0f32);

    let cpu_time = cpu_time::ProcessTime::now();

    gl_window.pos_y += 1.0f32;
    let pos_y = gl_window.pos_y;

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
            resource_key: app_resources.image2_resource_id,
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
                PathElement::MoveTo(PixelPoint::new(100.0f32, 350.0f32)),
                PathElement::LineTo(PixelPoint::new(300.0f32, 350.0f32)),
                PathElement::LineTo(PixelPoint::new(300.0f32, 550.0f32)),
                PathElement::LineTo(PixelPoint::new(100.0f32, 550.0f32)),
            ],
            brush: Brush::ImagePattern {
                resource_key: app_resources.image2_resource_id,
                transform: PixelTransform::identity()
                    .pre_translate(Vector2D::new(100.0f32, 350.0f32))
                    .pre_rotate(Angle::radians(pos_y / 100.0f32)),
                alpha: 1.0f32,
            },
        },
        Primitive::Image {
            resource_key: app_resources.image1_resource_id,
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
            resource_key: app_resources.image1_resource_id,
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
            resource_key: app_resources.image1_resource_id,
            rect: PixelRect::new(
                PixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
                PixelSize::new(4.0f32, 4.0f32),
            ),
            uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
        },
        Primitive::Line {
            color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
            thickness: PixelThickness::new(1.0f32),
            start_point: PixelPoint::new(width as f32, height as f32),
            end_point: PixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
        },
        Primitive::Image {
            resource_key: app_resources.image1_resource_id,
            rect: PixelRect::new(
                PixelPoint::new(0.0f32, height as f32 - 4.0f32),
                PixelSize::new(4.0f32, 4.0f32),
            ),
            uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
        },
        Primitive::Line {
            color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
            thickness: PixelThickness::new(1.0f32),
            start_point: PixelPoint::new(0.0f32, height as f32),
            end_point: PixelPoint::new(4.0f32, height as f32 - 4.0f32),
        },
        Primitive::Text {
            resource_key: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 + pos_y, 200.0f32),
            clipping_rect,
            size: PixelThickness::new(10.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            resource_key: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32, 220.0f32 - pos_y),
            clipping_rect,
            size: PixelThickness::new(12.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            resource_key: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 - pos_y, 240.0f32 + pos_y * 2.0f32),
            clipping_rect,
            size: PixelThickness::new(14.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            resource_key: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 - pos_y, 260.0f32),
            clipping_rect,
            size: PixelThickness::new(16.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            resource_key: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 + pos_y, 280.0f32 + pos_y),
            clipping_rect,
            size: PixelThickness::new(18.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Fill {
            path: vec![
                PathElement::MoveTo(PixelPoint::new(100.0f32, 350.0f32)),
                PathElement::BezierTo(
                    PixelPoint::new(120.0f32, 50.0f32),
                    PixelPoint::new(180.0f32, 50.0f32),
                    PixelPoint::new(300.0f32, 150.0f32),
                ),
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
                PathElement::MoveTo(PixelPoint::new(500.0f32, 350.0f32)),
                PathElement::BezierTo(
                    PixelPoint::new(520.0f32, 50.0f32),
                    PixelPoint::new(580.0f32, 50.0f32),
                    PixelPoint::new(700.0f32, 150.0f32),
                ),
                PathElement::ClosePath,
                PathElement::MoveTo(PixelPoint::new(550.0f32, 250.0f32)),
                PathElement::BezierTo(
                    PixelPoint::new(580.0f32, 150.0f32),
                    PixelPoint::new(620.0f32, 150.0f32),
                    PixelPoint::new(650.0f32, 180.0f32),
                ),
                PathElement::ClosePath,
                PathElement::Solidity(Solidity::Hole),
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
                PathElement::MoveTo(PixelPoint::new(300.0f32, 550.0f32)),
                PathElement::BezierTo(
                    PixelPoint::new(320.0f32, 250.0f32),
                    PixelPoint::new(380.0f32, 250.0f32),
                    PixelPoint::new(500.0f32, 350.0f32),
                ),
                PathElement::ClosePath,
            ],
            thickness: PixelThickness::new(1.0f32),
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
                    size: PixelThickness::new(22.0f32),
                    text: "Render target test".to_string(),
                },
            ],
        },
    ];

    /*unsafe {
        gl::BeginQuery(gl::TIME_ELAPSED, gl_window.time_query);
    }*/

    device.begin(gl_window.gl_context_data.as_ref().unwrap());

    device.clear(
        //window_target.get_render_target(),
        &render_target,
        &[0.5f32, 0.4f32, 0.3f32, 1.0f32],
    );
    renderer
        .draw(
            device,
            //window_target.get_render_target(),
            &render_target,
            &primitives,
            &mut app_resources.resources,
            false,
        )
        .unwrap();

    // end
    /*unsafe {
        gl::EndQuery(gl::TIME_ELAPSED);

        // retrieving the recorded elapsed time
        // wait until the query result is available
        let mut done = 0i32;
        while done == 0 {
            gl::GetQueryObjectiv(gl_window.time_query, gl::QUERY_RESULT_AVAILABLE, &mut done);
        }

        // get the query result
        let mut elapsed_time: GLuint64 = 0;
        gl::GetQueryObjectui64v(gl_window.time_query, gl::QUERY_RESULT, &mut elapsed_time);
        println!("GPU time: {} ms", elapsed_time as f64 / 1000000.0);
    }*/

    let cpu_time = cpu_time.elapsed();
    println!("CPU time: {:?}", cpu_time);
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
