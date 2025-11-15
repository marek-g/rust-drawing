#![windows_subsystem = "windows"]

use drawing_gl::{GlSurface, Primitive};
use euclid::{Angle, Vector2D};
use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::error::Error;
use std::ptr::null;
use std::rc::Rc;

use drawing_api::{
    Color, Context, DisplayListBuilder, Paint, PixelPoint, PixelRect, PixelSize, PixelThickness,
    Point, Surface,
};
use drawing_gl::Device;
use gl::types::*;
use windowing_qt::{Application, ApplicationOptions};

type DrawingContext = drawing_gl::GlContext;
type DisplayListBuilder1 = drawing_gl::DisplayListBuilder;
type Paint1 = drawing_gl::Paint;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub struct GlWindow {
    pub window: windowing_qt::Window,
    //pub gl_context: Option<DrawingContext>,
    pub time_query: GLuint,
    pub pos_y: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: simple2")
            .with_opengl_stencil_bits(8),
    )
    .unwrap();

    let drawing_context_rc = Rc::new(RefCell::new(None));

    let gl_window_rc = Rc::new(RefCell::new(GlWindow {
        window: windowing_qt::Window::new(None).unwrap(),
        //gl_context: None,
        time_query: 0,
        pos_y: 0.0f32,
    }));

    setup_window(&gl_window_rc, &drawing_context_rc);

    app.message_loop();

    Ok(())
}

fn setup_window(
    gl_window_rc: &Rc<RefCell<GlWindow>>,
    drawing_context_rc: &Rc<RefCell<Option<DrawingContext>>>,
) {
    let window = &mut gl_window_rc.borrow_mut().window;
    window.set_title("Example: simple2").unwrap();
    window.set_frame_position(800, 100);
    window.resize(800, 600);

    window.on_paint_gl({
        let gl_window_clone = gl_window_rc.clone();
        let drawing_context_clone = drawing_context_rc.clone();
        //let app_resources_clone = app_resources_rc.clone();
        let mut initialized = false;

        move || {
            if !initialized {
                let mut gl_window = gl_window_clone.borrow_mut();
                let drawing_context = DrawingContext::new_gl_context(|symbol| {
                    gl_window
                        .window
                        .get_opengl_proc_address(symbol)
                        .unwrap_or_else(|_| null())
                })
                .unwrap();

                drawing_context_clone.borrow_mut().insert(drawing_context);

                let mut time_query: GLuint = 0;
                unsafe {
                    gl::GenQueries(1, &mut time_query);
                    gl::BeginQuery(gl::TIME_ELAPSED, time_query);
                    gl::EndQuery(gl::TIME_ELAPSED);
                }
                gl_window.time_query = time_query;
                print!("time_query: {}", time_query);

                initialized = true;
            }

            if let Some(drawing_context) = drawing_context_clone.borrow_mut().as_mut() {
                draw(
                    drawing_context,
                    &mut gl_window_clone.borrow_mut(),
                    //&mut app_resources_clone.borrow_mut(),
                );
            }

            // continue animation
            gl_window_clone.borrow_mut().window.update();
        }
    });

    window.set_visible(true).unwrap();
}

pub fn draw(
    drawing_context: &mut DrawingContext,
    gl_window: &mut GlWindow,
    //app_resources: &mut AppResources,
) {
    let width = gl_window.window.get_width();
    let height = gl_window.window.get_height();

    if width <= 0 || height <= 0 {
        return;
    }

    let drawing_surface = drawing_context.wrap_framebuffer(
        gl_window.window.get_default_framebuffer_id(),
        width as u16,
        height as u16,
        drawing_api::ColorFormat::RGBA,
    );

    /*let mut display_list_builder = DisplayListBuilder1::new();
    let mut paint = Paint1::new();
    paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    display_list_builder.draw_line(
        PixelPoint::new(100.0f32, 100.0f32),
        PixelPoint::new(300.5f32, 100.5f32),
        &paint,
    );
    let display_list = display_list_builder.build().unwrap();*/

    let display_list = vec![
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
        /*Primitive::Image {
            resource_key: app_resources.image2_resource_id,
            rect: PixelRect::new(
                PixelPoint::new(100.0f32, 150.0f32),
                PixelSize::new(200.0f32, 200.0f32),
            ),
            uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
        },*/
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
        /*Primitive::Fill {
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
        },*/
        Primitive::Line {
            color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
            thickness: PixelThickness::new(1.0f32),
            start_point: PixelPoint::new(0.0f32, 0.0f32),
            end_point: PixelPoint::new(4.0f32, 4.0f32),
        },
        /*Primitive::Image {
            resource_key: app_resources.image1_resource_id,
            rect: PixelRect::new(
                PixelPoint::new(width as f32 - 4.0f32, 0.0f32),
                PixelSize::new(4.0f32, 4.0f32),
            ),
            uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
        },*/
        Primitive::Line {
            color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
            thickness: PixelThickness::new(1.0f32),
            start_point: PixelPoint::new(width as f32, 0.0f32),
            end_point: PixelPoint::new(width as f32 - 4.0f32, 4.0f32),
        },
        /*Primitive::Image {
            resource_key: app_resources.image1_resource_id,
            rect: PixelRect::new(
                PixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
                PixelSize::new(4.0f32, 4.0f32),
            ),
            uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
        },*/
        Primitive::Line {
            color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
            thickness: PixelThickness::new(1.0f32),
            start_point: PixelPoint::new(width as f32, height as f32),
            end_point: PixelPoint::new(width as f32 - 4.0f32, height as f32 - 4.0f32),
        },
        /*Primitive::Image {
            resource_key: app_resources.image1_resource_id,
            rect: PixelRect::new(
                PixelPoint::new(0.0f32, height as f32 - 4.0f32),
                PixelSize::new(4.0f32, 4.0f32),
            ),
            uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
        },*/
        Primitive::Line {
            color: [0.0f32, 1.0f32, 0.0f32, 1.0f32],
            thickness: PixelThickness::new(1.0f32),
            start_point: PixelPoint::new(0.0f32, height as f32),
            end_point: PixelPoint::new(4.0f32, height as f32 - 4.0f32),
        },
        /*Primitive::Text {
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
        },*/
        /*Primitive::Fill {
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
        },*/
        /*Primitive::Fill {
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
        },*/
        /*Primitive::Stroke {
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
        },*/
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
                /*Primitive::Text {
                    resource_key: "F1".to_string(),
                    color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                    position: PixelPoint::new(207.0f32, 232.0f32),
                    clipping_rect,
                    size: PixelThickness::new(22.0f32),
                    text: "Render target test".to_string(),
                },*/
            ],
        },
    ];

    //drawing_surface.draw(&drawing_list);
    //drawing_context.set_render_target(&render_target);
    drawing_context.clear(&drawing_surface, &[1.0f32, 0.66f32, 0.33f32, 1.0f32]);

    drawing_context.draw(&drawing_surface, &display_list);
}
