#![windows_subsystem = "windows"]

use drawing_gl::{Brush, PathElement, Primitive, Solidity};
use euclid::{default::Transform3D, rect, Angle, Vector2D, Vector3D};
use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::error::Error;
use std::ptr::null;
use std::rc::Rc;

use drawing_api::{
    Color, ColorFormat, Context, DipLength, DipPoint, DipRect, DisplayListBuilder, Fonts, Paint,
    PathBuilder, PixelLength, PixelPoint, PixelRect, PixelSize, PixelTransform, Point, Surface,
    TextureSampling,
};
use gl::types::*;
use windowing_qt::{Application, ApplicationOptions};

type DrawingContext = drawing_gl::GlContext;

type DisplayListBuilder1 = <DrawingContext as Context>::DisplayListBuilder;
type PathBuilder1 = <DrawingContext as Context>::PathBuilder;
type Paint1 = <DrawingContext as Context>::Paint;
type Texture1 = <DrawingContext as Context>::Texture;
type Fonts1 = <DrawingContext as Context>::Fonts;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub struct GlWindow {
    pub window: windowing_qt::Window,
    pub gl_context: Option<DrawingContext>,

    pub time_query: GLuint,
    pub pos_y: f32,
}

pub struct Resources {
    pub image1: Texture1,
    pub image2: Texture1,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: simple2")
            .with_opengl_stencil_bits(8),
    )
    .unwrap();

    let gl_window_rc = Rc::new(RefCell::new(GlWindow {
        window: windowing_qt::Window::new(None).unwrap(),
        gl_context: None,
        time_query: 0,
        pos_y: 0.0f32,
    }));

    setup_window(&gl_window_rc);

    app.message_loop();

    Ok(())
}

fn setup_window(gl_window_rc: &Rc<RefCell<GlWindow>>) {
    let window = &mut gl_window_rc.borrow_mut().window;
    window.set_title("Example: simple2").unwrap();
    window.set_frame_position(800, 100);
    window.resize(800, 600);

    window.on_paint_gl({
        let gl_window_clone = gl_window_rc.clone();
        let mut initialized = false;
        let mut resources = None;
        let fonts = Fonts1::new();

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

                register_fonts(&fonts).unwrap();
                resources = Some(initialize_resources(&drawing_context));

                gl_window.gl_context = Some(drawing_context);

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

            if let Some(resources) = &resources {
                draw(&mut gl_window_clone.borrow_mut(), &resources, &fonts);
            }

            // continue animation
            gl_window_clone.borrow_mut().window.update();
        }
    });

    window.set_visible(true).unwrap();
}

fn register_fonts(fonts: &Fonts1) -> Result<(), &'static str> {
    fonts.register_font(
        &Assets::get("OpenSans-Regular.ttf").unwrap().data,
        Some("F1"),
    )
}

fn initialize_resources(drawing_context: &DrawingContext) -> Resources {
    Resources {
        image1: create_chessboard(drawing_context, 4, 4),
        image2: create_chessboard(drawing_context, 200, 200),
    }
}

pub fn create_chessboard(drawing_context: &DrawingContext, w: usize, h: usize) -> Texture1 {
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

    drawing_context
        .create_texture(&data, w as u16, h as u16, ColorFormat::RGBA)
        .unwrap()
}

pub fn draw(gl_window: &mut GlWindow, resources: &Resources, fonts: &Fonts1) {
    let width = gl_window.window.get_width();
    let height = gl_window.window.get_height();

    if width <= 0 || height <= 0 {
        return;
    }

    let framebuffer_id = gl_window.window.get_default_framebuffer_id();
    gl_window.pos_y += 1.0f32;
    let pos_y = gl_window.pos_y;

    let cpu_time = cpu_time::ProcessTime::now();

    let mut display_list_builder = DisplayListBuilder1::new();
    let mut paint = Paint1::new();

    paint.set_color(Color::rgb(1.0f32, 0.66f32, 0.33f32));
    display_list_builder.draw_paint(&paint);

    paint.set_color(Color::rgb(1.0f32, 0.0f32, 0.0f32));
    display_list_builder.draw_rect(rect(100.5f32, 101.5f32, 200.0f32, 50.0f32), &paint);

    paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    display_list_builder.draw_line((100.0f32, 100.0f32), (300.5f32, 100.5f32), &paint);

    display_list_builder.draw_texture_rect(
        &resources.image2,
        rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(100.0f32, 150.0f32, 200.0f32, 200.0f32),
        TextureSampling::Linear,
        None,
    );

    paint.set_color(Color::rgb(0.0f32, 1.0f32, 0.0f32));
    display_list_builder.draw_line((100.0f32, 350.0f32), (300.0f32, 150.0f32), &paint);
    display_list_builder.draw_line((100.0f32, 150.0f32), (300.0f32, 350.0f32), &paint);

    let mut path_builder = PathBuilder1::new();
    path_builder.move_to((100.0f32, 350.0f32));
    path_builder.line_to((300.0f32, 350.0f32));
    path_builder.line_to((300.0f32, 550.0f32));
    path_builder.line_to((100.0f32, 550.0f32));
    paint.set_color_source(Some(drawing_api::ColorSource::Image {
        image: resources.image2.clone(),
        horizontal_tile_mode: drawing_api::TileMode::Repeat,
        vertical_tile_mode: drawing_api::TileMode::Repeat,
        sampling: TextureSampling::Linear,
        transformation: Some(
            Transform3D::identity()
                .pre_translate(Vector3D::new(100.0f32, 350.0f32, 0.0f32))
                .pre_rotate(0.0f32, 0.0f32, 1.0f32, Angle::radians(pos_y / 100.0f32)),
        ),
    }));
    display_list_builder.draw_path(&path_builder.build().unwrap(), &paint);

    display_list_builder.draw_texture_rect(
        &resources.image1,
        rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, 0.0f32, 4.0f32, 4.0f32),
        TextureSampling::Linear,
        None,
    );
    display_list_builder.draw_line((0.0f32, 0.0f32), (4.0f32, 4.0f32), &paint);

    display_list_builder.draw_texture_rect(
        &resources.image1,
        rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(width as f32 - 4.0f32, 0.0f32, 4.0f32, 4.0f32),
        TextureSampling::Linear,
        None,
    );
    display_list_builder.draw_line(
        (width as f32 - 4.0f32, 4.0f32),
        (width as f32, 0.0f32),
        &paint,
    );

    display_list_builder.draw_texture_rect(
        &resources.image1,
        rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(
            width as f32 - 4.0f32,
            height as f32 - 4.0f32,
            4.0f32,
            4.0f32,
        ),
        TextureSampling::Linear,
        None,
    );
    display_list_builder.draw_line(
        (width as f32, height as f32),
        (width as f32 - 4.0f32, height as f32 - 4.0f32),
        &paint,
    );

    display_list_builder.draw_texture_rect(
        &resources.image1,
        rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, height as f32 - 4.0f32, 4.0f32, 4.0f32),
        TextureSampling::Linear,
        None,
    );
    display_list_builder.draw_line(
        (0.0f32, height as f32),
        (4.0f32, height as f32 - 4.0f32),
        &paint,
    );

    let display_list = display_list_builder.build().unwrap();

    /*let clipping_rect = PixelRect::new(
        PixelPoint::new(0.0f32, 0.0f32),
        PixelSize::new(width as f32, height as f32),
    );

    let display_list = vec![
        Primitive::Text {
            fonts: fonts.clone(),
            family_name: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 + pos_y, 200.0f32),
            clipping_rect,
            size: PixelThickness::new(10.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            fonts: fonts.clone(),
            family_name: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32, 220.0f32 - pos_y),
            clipping_rect,
            size: PixelThickness::new(12.0f32),
            text: "Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            fonts: fonts.clone(),
            family_name: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 - pos_y, 240.0f32 + pos_y * 2.0f32),
            clipping_rect,
            size: PixelThickness::new(14.0f32),
            text: "Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk\nABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            fonts: fonts.clone(),
            family_name: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(350.0f32 - pos_y, 260.0f32),
            clipping_rect,
            size: PixelThickness::new(16.0f32),
            text: "Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz"
                .to_string(),
        },
        Primitive::Text {
            fonts: fonts.clone(),
            family_name: "F1".to_string(),
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
                    fonts: fonts.clone(),
                    family_name: "F1".to_string(),
                    color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                    position: PixelPoint::new(207.0f32, 232.0f32),
                    clipping_rect,
                    size: PixelThickness::new(22.0f32),
                    text: "Render target test".to_string(),
                },
            ],
        },
    ];*/

    //drawing_surface.draw(&drawing_list);
    //drawing_context.set_render_target(&render_target);

    if let Some(ref drawing_context) = gl_window.gl_context {
        let drawing_surface = drawing_context.wrap_framebuffer(
            framebuffer_id,
            width as u16,
            height as u16,
            drawing_api::ColorFormat::RGBA,
        );

        drawing_context
            .draw(&drawing_surface, &display_list)
            .unwrap();
    }

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
            gl::GetQueryObjectiv(gl_window.time_query, gl::QUERY_RESULT_AVAILABLE, &mut done);
        }

        // get the query result
        let mut elapsed_time: GLuint64 = 0;
        gl::GetQueryObjectui64v(gl_window.time_query, gl::QUERY_RESULT, &mut elapsed_time);
        println!("GPU time: {} ms", elapsed_time as f64 / 1000000.0);
    }*/
}
