#![windows_subsystem = "windows"]

use drawing_api::backend::{Device, RenderTarget};
use drawing_api::color::*;
use drawing_api::font::Font;
use drawing_api::primitive::*;
use drawing_api::renderer::Renderer;
use drawing_api::resources::Resources;
use drawing_api::units::*;

use drawing_gl::{GlContextData, GlDevice, GlRenderTarget};
use euclid::{Angle, Vector2D};
use rust_embed::RustEmbed;
use std::cell::{Ref, RefCell};

use gl::types::*;

type DrawingDevice = drawing_gl::GlDevice;
type DrawingFont = drawing_gl::TextureFont<DrawingDevice>;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let window_builder = winit::window::WindowBuilder::new().with_title("Example: simple (winit)");
    let event_loop = winit::event_loop::EventLoop::new();

    let mut device = DrawingDevice::new().unwrap();
    let mut window_target =
        create_window_target(&mut device, window_builder, &event_loop, None).unwrap();
    let mut renderer = Renderer::new();

    //
    // create resources
    //
    let mut resources = Resources::new();

    // font
    let font =
        DrawingFont::create(Assets::get("OpenSans-Regular.ttf").unwrap().data.to_vec()).unwrap();

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
            winit::event::Event::MainEventsCleared => {
                // Application update code.
     
                // Queue a RedrawRequested event.
                window_target.get_window().request_redraw();
            },

            winit::event::Event::RedrawRequested(_window_id) => {
                // Redraw the application.
                //
                // It's preferrable to render in this event rather than in EventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
                if width <= 0 || height <= 0 {
                    return;
                }

                let cpu_time = cpu_time::ProcessTime::now();

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
                            PathElement::MoveTo(PixelPoint::new(100.0f32, 350.0f32)),
                            PathElement::LineTo(PixelPoint::new(300.0f32, 350.0f32)),
                            PathElement::LineTo(PixelPoint::new(300.0f32, 550.0f32)),
                            PathElement::LineTo(PixelPoint::new(100.0f32, 550.0f32)),
                        ],
                        brush: Brush::ImagePattern {
                            resource_key: image2_resource_id,
                            transform: PixelTransform::identity()
                                .pre_translate(Vector2D::new(100.0f32, 350.0f32))
                                .pre_rotate(Angle::radians(pos_y / 100.0f32)),
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
                    Primitive::Line {
                        color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
                        thickness: PixelThickness::new(1.0f32),
                        start_point: PixelPoint::new(width as f32, height as f32),
                        end_point: PixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
                    },
                    Primitive::Image {
                        resource_key: image1_resource_id,
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
                        text: "Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                            .to_string(),
                    },
                    Primitive::Text {
                        resource_key: "F1".to_string(),
                        color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                        position: PixelPoint::new(350.0f32 - pos_y, 240.0f32 + pos_y * 2.0f32),
                        clipping_rect,
                        size: PixelThickness::new(14.0f32),
                        text: "Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk\nABCDEFGHIJK XYZ xyz"
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
                            PathElement::BezierTo(PixelPoint::new(120.0f32, 50.0f32),
                                PixelPoint::new(180.0f32, 50.0f32),
                                PixelPoint::new(300.0f32, 150.0f32)),
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
                            PathElement::BezierTo(PixelPoint::new(520.0f32, 50.0f32),
                                PixelPoint::new(580.0f32, 50.0f32),
                                PixelPoint::new(700.0f32, 150.0f32)),
                            PathElement::ClosePath,

                            PathElement::MoveTo(PixelPoint::new(550.0f32, 250.0f32)),
                            PathElement::BezierTo(PixelPoint::new(580.0f32, 150.0f32),
                                PixelPoint::new(620.0f32, 150.0f32),
                                PixelPoint::new(650.0f32, 180.0f32)),
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
                            PathElement::BezierTo(PixelPoint::new(320.0f32, 250.0f32),
                                PixelPoint::new(380.0f32, 250.0f32),
                                PixelPoint::new(500.0f32, 350.0f32)),
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
                    }
                ];

		// doesn't work on wayland?
                /*unsafe {
                    gl::BeginQuery(gl::TIME_ELAPSED, window_target.time_query);
                }*/

                // make current context
                window_target.make_current_context();

                device.begin(&window_target.gl_context_data).unwrap();

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

                // end
                let cpu_time = cpu_time.elapsed();
                println!("CPU time: {:?}", cpu_time);

		// doesn't work on wayland?
                /*unsafe {
                    gl::EndQuery(gl::TIME_ELAPSED);

                    // retrieving the recorded elapsed time
                    // wait until the query result is available
                    let mut done = 0i32;
                    while done == 0 {
                        gl::GetQueryObjectiv(
                            window_target.time_query,
                            gl::QUERY_RESULT_AVAILABLE,
                            &mut done,
                        );
                    }

                    // get the query result
                    let mut elapsed_time: GLuint64 = 0;
                    gl::GetQueryObjectui64v(
                        window_target.time_query,
                        gl::QUERY_RESULT,
                        &mut elapsed_time,
                    );
                    println!("GPU time: {} ms", elapsed_time as f64 / 1000000.0);
                }*/

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
                    winit::event::WindowEvent::Resized(physical_size) => {
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

pub fn create_window_target(
    device: &mut GlDevice,
    window_builder: winit::window::WindowBuilder,
    events_loop: &winit::event_loop::EventLoop<()>,
    shared_window_target: Option<&GlWindowTarget>,
) -> Result<GlWindowTarget, ()> {
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_vsync(true);

    // make current gl context
    let windowed_context = if let Some(ref shared_window_target) = shared_window_target {
        if let Some(ref gl_windowed_context) =
            shared_window_target.gl_windowed_context.borrow().as_ref()
        {
            unsafe {
                context_builder
                    .with_shared_lists(gl_windowed_context.context())
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
                    .make_current()
                    .unwrap()
            }
        } else {
            unsafe {
                context_builder
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
                    .make_current()
                    .unwrap()
            }
        }
    } else {
        unsafe {
            context_builder
                .build_windowed(window_builder, &events_loop)
                .unwrap()
                .make_current()
                .unwrap()
        }
    };

    // initialize gl context
    let gl_context_data = device
        .init_context(|symbol| windowed_context.context().get_proc_address(symbol) as *const _);

    let aspect_ratio = windowed_context.window().scale_factor() as f32;

    // doesn't work on wayland?
    /*let mut time_query: GLuint = 0;
    unsafe {
        gl::GenQueries(1, &mut time_query);
        gl::BeginQuery(gl::TIME_ELAPSED, time_query);
        gl::EndQuery(gl::TIME_ELAPSED);
    }
    print!("time_query: {}", time_query);*/

    Ok(GlWindowTarget {
        gl_windowed_context: RefCell::new(Some(windowed_context)),
        gl_context_data,
        gl_render_target: GlRenderTarget::new(0, 0, 0, aspect_ratio),
    })
}

pub struct GlWindowTarget {
    gl_windowed_context:
        RefCell<Option<glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>>>,
    gl_context_data: GlContextData,
    gl_render_target: GlRenderTarget,
}

impl GlWindowTarget {
    pub fn get_window(&self) -> Ref<winit::window::Window> {
        Ref::map(self.gl_windowed_context.borrow(), |context| {
            context.as_ref().unwrap().window()
        })
    }

    pub fn get_render_target(&self) -> &GlRenderTarget {
        &self.gl_render_target
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        unsafe {
            self.gl_render_target.update_size(width, height);
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.gl_windowed_context
            .borrow()
            .as_ref()
            .unwrap()
            .swap_buffers()
            .unwrap();
    }

    pub fn make_current_context(&mut self) {
        unsafe {
            let context = self.gl_windowed_context.replace(None);
            let context = context.unwrap().make_current().unwrap();
            self.gl_windowed_context.replace(Some(context));
        }
    }

    pub fn get_context(
        &self,
    ) -> Ref<glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>> {
        Ref::map(self.gl_windowed_context.borrow(), |context| {
            context.as_ref().unwrap()
        })
    }
}

impl Drop for GlWindowTarget {
    fn drop(&mut self) {
        unsafe {
            let context = self.gl_windowed_context.replace(None);
            let context = context.unwrap().make_current().unwrap();
            self.gl_windowed_context.replace(Some(context));
        }
    }
}
