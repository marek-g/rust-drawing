use std::{borrow::Cow, cell::RefCell, ptr::null_mut, rc::Rc};

use drawing_api::{
    Color, DisplayListBuilder, Matrix, Paint, ParagraphBuilder, ParagraphStyle, PathBuilder,
    Surface, TextureDescriptor, TextureSampling,
};
use euclid::{rect, Angle, Vector3D};
use gl::types::GLuint;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub struct GlWindow<C: drawing_api::Context> {
    pub window: windowing_qt::Window,
    pub gl_context: Option<C>,

    pub initial_width: u32,
    pub initial_height: u32,

    pub time_query: GLuint,
    pub pos_y: f32,
}

pub struct Resources<C: drawing_api::Context> {
    pub fonts: C::Fonts,
    pub image1: C::Texture,
    pub image2: C::Texture,
}

pub fn setup_window<C>(title: &str) -> Rc<RefCell<GlWindow<C>>>
where
    C: drawing_api::ContextGl + 'static,
{
    let gl_window_rc = Rc::new(RefCell::new(GlWindow::<C> {
        window: windowing_qt::Window::new(None).unwrap(),
        gl_context: None,
        initial_width: 0,
        initial_height: 0,
        time_query: 0,
        pos_y: 0.0,
    }));
    let gl_window_rc_clone = gl_window_rc.clone();

    let window = &mut gl_window_rc.borrow_mut().window;
    window.set_title(title).unwrap();
    window.set_frame_position(800, 100);
    window.resize(800, 600);

    window.on_paint_gl({
        let gl_window_clone = gl_window_rc.clone();
        let mut initialized = false;
        let mut resources = None;

        move || {
            if !initialized {
                let drawing_context = unsafe {
                    C::new_gl(|symbol| {
                        gl_window_clone
                            .borrow_mut()
                            .window
                            .get_opengl_proc_address(symbol)
                            .unwrap_or_else(|_| null_mut())
                    })
                    .unwrap()
                };

                resources = Some(initialize_resources(&drawing_context));

                register_fonts(&mut resources.as_mut().unwrap().fonts).unwrap();

                gl_window_clone.borrow_mut().gl_context = Some(drawing_context);

                let time_query: GLuint = 0;
                /*unsafe {
                    gl::GenQueries(1, &mut time_query);
                    gl::BeginQuery(gl::TIME_ELAPSED, time_query);
                    gl::EndQuery(gl::TIME_ELAPSED);
                }*/
                gl_window_clone.borrow_mut().time_query = time_query;
                print!("time_query: {}", time_query);

                initialized = true;
            }

            if let Some(resources) = &resources {
                draw(&mut gl_window_clone.borrow_mut(), &resources);
            }

            // continue animation
            gl_window_clone.borrow_mut().window.update();
        }
    });

    window.set_visible(true).unwrap();

    gl_window_rc_clone
}

fn register_fonts<F: drawing_api::Fonts>(fonts: &mut F) -> Result<(), &'static str> {
    fonts.register_font(
        Assets::get("OpenSans-Regular.ttf").unwrap().data,
        Some("F1"),
    )
}

fn initialize_resources<C: drawing_api::Context>(drawing_context: &C) -> Resources<C> {
    Resources::<C> {
        fonts: C::Fonts::default(),
        image1: create_chessboard(drawing_context, 4, 4),
        image2: create_chessboard(drawing_context, 200, 200),
    }
}

pub fn create_chessboard<C: drawing_api::Context>(
    drawing_context: &C,
    w: usize,
    h: usize,
) -> C::Texture {
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

    unsafe {
        drawing_context
            .create_texture(
                Cow::Owned(data),
                TextureDescriptor {
                    width: w as u32,
                    height: h as u32,
                    ..Default::default()
                },
            )
            .unwrap()
    }
}

fn draw<C: drawing_api::ContextGl>(gl_window: &mut GlWindow<C>, resources: &Resources<C>) {
    let width = gl_window.window.get_width();
    let height = gl_window.window.get_height();

    if width <= 0 || height <= 0 {
        return;
    }

    if gl_window.initial_width == 0 && gl_window.initial_height == 0 {
        gl_window.initial_width = width as u32;
        gl_window.initial_height = height as u32;
    }

    let framebuffer_id = gl_window.window.get_default_framebuffer_id();
    gl_window.pos_y += 1.0;
    let pos_y = gl_window.pos_y;

    let cpu_time = cpu_time::ProcessTime::now();

    let mut dlb = C::DisplayListBuilder::new(None);
    dlb.scale(
        width as f32 / gl_window.initial_width as f32,
        height as f32 / gl_window.initial_height as f32,
    );

    dlb.draw_paint(&C::Paint::color("#FFA5"));

    dlb.draw_rect(rect(100.5, 101.5, 200.0, 50.0), &C::Paint::color("#F00"));

    dlb.draw_line((100.0, 100.0), (300.5, 100.5), &C::Paint::color("#FFF"));

    dlb.draw_texture_rect(
        &resources.image2,
        rect(0.0, 0.0, 200.0, 200.0),
        rect(100.0, 150.0, 200.0, 200.0),
        TextureSampling::NearestNeighbor,
        None,
    );

    let paint = C::Paint::color("#0F0");
    dlb.draw_line((100.0, 350.0), (300.0, 150.0), &paint);
    dlb.draw_line((100.0, 150.0), (300.0, 350.0), &paint);

    let mut pb = C::PathBuilder::default();
    pb.move_to((100.0, 350.0));
    pb.line_to((300.0, 350.0));
    pb.line_to((300.0, 550.0));
    pb.line_to((100.0, 550.0));
    let paint = C::Paint::color_source(drawing_api::ColorSource::Image {
        image: resources.image2.clone(),
        horizontal_tile_mode: drawing_api::TileMode::Repeat,
        vertical_tile_mode: drawing_api::TileMode::Repeat,
        sampling: TextureSampling::Linear,
        transformation: Some(
            Matrix::identity()
                .pre_translate(Vector3D::new(100.0, 350.0, 0.0))
                .pre_rotate(0.0, 0.0, 1.0, Angle::radians(pos_y / 100.0)),
        ),
    });
    dlb.draw_path(&pb.build().unwrap(), &paint);

    let paint = C::Paint::color((0.0, 1.0, 0.0));
    dlb.draw_texture_rect(
        &resources.image1,
        rect(0.0, 0.0, 4.0, 4.0),
        rect(0.0, 0.0, 4.0, 4.0),
        TextureSampling::NearestNeighbor,
        None,
    );
    dlb.draw_line((0.0, 0.0), (4.0, 4.0), &paint);

    dlb.draw_texture_rect(
        &resources.image1,
        rect(0.0, 0.0, 4.0, 4.0),
        rect(gl_window.initial_width as f32 - 4.0, 0.0, 4.0, 4.0),
        TextureSampling::NearestNeighbor,
        None,
    );
    dlb.draw_line(
        (gl_window.initial_width as f32 - 4.0, 4.0),
        (gl_window.initial_width as f32, 0.0),
        &paint,
    );

    dlb.draw_texture_rect(
        &resources.image1,
        rect(0.0, 0.0, 4.0, 4.0),
        rect(
            gl_window.initial_width as f32 - 4.0,
            gl_window.initial_height as f32 - 4.0,
            4.0,
            4.0,
        ),
        TextureSampling::NearestNeighbor,
        None,
    );
    dlb.draw_line(
        (
            gl_window.initial_width as f32,
            gl_window.initial_height as f32,
        ),
        (
            gl_window.initial_width as f32 - 4.0,
            gl_window.initial_height as f32 - 4.0,
        ),
        &paint,
    );

    dlb.draw_texture_rect(
        &resources.image1,
        rect(0.0, 0.0, 4.0, 4.0),
        rect(0.0, gl_window.initial_height as f32 - 4.0, 4.0, 4.0),
        TextureSampling::Linear,
        None,
    );
    dlb.draw_line(
        (0.0, gl_window.initial_height as f32),
        (4.0, gl_window.initial_height as f32 - 4.0),
        &paint,
    );

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    pb.push_style(ParagraphStyle::simple("F1", 10.0, C::Paint::color("#FFF")));
    pb.add_text("Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0 + pos_y, 200.0), &paragraph);

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    pb.push_style(ParagraphStyle::simple("F1", 12.0, C::Paint::color("#FFF")));
    pb.add_text("Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0, 220.0 - pos_y), &paragraph);

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    pb.push_style(ParagraphStyle::simple("F1", 14.0, C::Paint::color("#FFF")));
    pb.add_text("Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0 - pos_y, 240.0 + pos_y * 2.0), &paragraph);

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    pb.push_style(ParagraphStyle::simple("F1", 16.0, C::Paint::color("#FFF")));
    pb.add_text("Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0 - pos_y, 260.0), &paragraph);

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    pb.push_style(ParagraphStyle::simple("F1", 18.0, C::Paint::color("#FFF")));
    pb.add_text("Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0 + pos_y, 280.0 + pos_y), &paragraph);

    let mut pb = C::PathBuilder::default();
    pb.move_to((100.0, 350.0));
    pb.cubic_curve_to((120.0, 50.0), (180.0, 50.0), (300.0, 150.0));
    let paint = C::Paint::color_source(drawing_api::ColorSource::LinearGradient {
        start: (100.0, 150.0).into(),
        end: (350.0, 350.0).into(),
        colors: vec![
            Color::rgba(1.0, 0.0, 0.0, 0.75),
            Color::rgba(1.0, 1.0, 0.0, 0.75),
        ],
        stops: vec![0.0, 1.0],
        tile_mode: drawing_api::TileMode::Mirror,
        transformation: None,
    });
    dlb.draw_path(&pb.build().unwrap(), &paint);

    //
    // Filled path
    //

    let mut pb = C::PathBuilder::default();
    pb.move_to((500.0, 350.0));
    pb.cubic_curve_to((520.0, 50.0), (580.0, 50.0), (700.0, 150.0));
    pb.close();
    pb.move_to((450.0, 100.0));
    pb.line_to((650.0, 200.0));
    pb.line_to((750.0, 150.0));
    pb.close();
    let paint = C::Paint::color_source(drawing_api::ColorSource::LinearGradient {
        start: (500.0, 150.0).into(),
        end: (750.0, 350.0).into(),
        colors: vec![
            Color::rgba(1.0, 0.0, 0.0, 0.75),
            Color::rgba(1.0, 1.0, 0.0, 0.75),
        ],
        stops: vec![0.0, 1.0],
        tile_mode: drawing_api::TileMode::Mirror,
        transformation: None,
    });
    dlb.draw_path(&pb.build().unwrap(), &paint);

    //
    // Stroke path
    //

    let mut pb = C::PathBuilder::default();
    pb.move_to((300.0, 550.0));
    pb.cubic_curve_to((320.0, 250.0), (380.0, 250.0), (500.0, 350.0));
    pb.close();
    let mut paint = C::Paint::color_source(drawing_api::ColorSource::LinearGradient {
        start: (200.0, 450.0).into(),
        end: (450.0, 650.0).into(),
        colors: vec![
            Color::rgba(1.0, 0.0, 0.0, 0.75),
            Color::rgba(1.0, 1.0, 0.0, 0.75),
        ],
        stops: vec![0.0, 1.0],
        tile_mode: drawing_api::TileMode::Mirror,
        transformation: None,
    });
    paint.set_draw_style(drawing_api::DrawStyle::Stroke);
    dlb.draw_path(&pb.build().unwrap(), &paint);

    //
    // Clipping & transformation test
    //

    dlb.save();

    dlb.translate(600.0, 450.0);
    dlb.rotate(pos_y);
    dlb.translate(-100.0, -100.0);

    dlb.clip_oval(
        rect(0.0, 0.0, 200.0, 200.0),
        drawing_api::ClipOperation::Intersect,
    );

    dlb.clip_rect(
        rect(50.0, 50.0, 100.0, 100.0),
        drawing_api::ClipOperation::Difference,
    );

    dlb.draw_texture_rect(
        &resources.image2,
        rect(0.0, 0.0, 200.0, 200.0),
        rect(0.0, 0.0, 200.0, 200.0),
        TextureSampling::Linear,
        None,
    );

    dlb.restore();

    //
    // Render target test
    //

    let paint_layer = C::Paint::color((1.0, 1.0, 1.0, 0.5));
    dlb.save_layer(
        rect(200.5, 220.5, 200.0, 50.0),
        Some(&paint_layer),
        None,
        /*Some(drawing_api::ImageFilter::Blur {
            x_sigma: 8.0,
            y_sigma: 8.0,
            tile_mode: drawing_api::TileMode::Clamp,
        }),*/
    );

    let paint2 = C::Paint::color((0.0, 0.5, 0.3, 1.0));
    dlb.draw_rect(rect(200.5, 220.5, 200.0, 50.0), &paint2);

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    pb.push_style(ParagraphStyle::simple("F1", 22.0, C::Paint::color("#FFF")));
    pb.add_text("Render target test");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((207.0, 232.0), &paragraph);

    dlb.restore();

    //
    // Build display list
    //

    let display_list = dlb.build().unwrap();

    if let Some(ref mut drawing_context) = gl_window.gl_context {
        let mut drawing_surface = unsafe {
            drawing_context
                .wrap_gl_framebuffer(
                    framebuffer_id,
                    width as u32,
                    height as u32,
                    drawing_api::ColorFormat::RGBA,
                )
                .unwrap()
        };

        drawing_surface.draw(&display_list).unwrap();
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
