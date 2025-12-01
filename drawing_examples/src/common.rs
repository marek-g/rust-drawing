use std::{cell::RefCell, rc::Rc};

use drawing_api::{
    Color, ColorFormat, Context, DisplayListBuilder, Paint, ParagraphBuilder, ParagraphStyle,
    PathBuilder, TextureSampling,
};
use euclid::rect;
use gl::types::GLuint;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub struct GlWindow<C: drawing_api::Context> {
    pub window: windowing_qt::Window,
    pub gl_context: Option<C>,

    pub time_query: GLuint,
    pub pos_y: f32,
}

pub struct Resources<C: drawing_api::Context> {
    pub fonts: C::Fonts,
    pub image1: C::Texture,
    pub image2: C::Texture,
}

pub fn setup_window<C, F>(title: &str, new_context_func: F) -> Rc<RefCell<GlWindow<C>>>
where
    C: drawing_api::Context + 'static,
    F: Fn(Rc<RefCell<GlWindow<C>>>) -> C + 'static,
{
    let gl_window_rc = Rc::new(RefCell::new(GlWindow::<C> {
        window: windowing_qt::Window::new(None).unwrap(),
        gl_context: None,
        time_query: 0,
        pos_y: 0.0f32,
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
                let drawing_context = new_context_func(gl_window_clone.clone());
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
        Assets::get("OpenSans-Regular.ttf")
            .unwrap()
            .data
            .into_owned()
            .into_boxed_slice(),
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

    drawing_context
        .create_texture(
            data.into_boxed_slice(),
            w as u16,
            h as u16,
            ColorFormat::RGBA,
        )
        .unwrap()
}

fn draw<C: drawing_api::Context>(gl_window: &mut GlWindow<C>, resources: &Resources<C>) {
    let width = gl_window.window.get_width();
    let height = gl_window.window.get_height();

    if width <= 0 || height <= 0 {
        return;
    }

    let framebuffer_id = gl_window.window.get_default_framebuffer_id();
    gl_window.pos_y += 1.0f32;
    let pos_y = gl_window.pos_y;

    let cpu_time = cpu_time::ProcessTime::now();

    let mut dlb = <C as Context>::DisplayListBuilder::new(None);
    let mut paint = <C as Context>::Paint::default();

    paint.set_color(Color::rgb(1.0f32, 0.66f32, 0.33f32));
    dlb.draw_paint(&paint);

    paint.set_color(Color::rgb(1.0f32, 0.0f32, 0.0f32));
    dlb.draw_rect(rect(100.5f32, 101.5f32, 200.0f32, 50.0f32), &paint);

    paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    dlb.draw_line((100.0f32, 100.0f32), (300.5f32, 100.5f32), &paint);

    dlb.draw_texture_rect(
        &resources.image2,
        // rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, 0.0f32, 200.0f32, 200.0f32),
        rect(100.0f32, 150.0f32, 200.0f32, 200.0f32),
        TextureSampling::NearestNeighbor,
        None,
    );

    paint.set_color(Color::rgb(0.0f32, 1.0f32, 0.0f32));
    dlb.draw_line((100.0f32, 350.0f32), (300.0f32, 150.0f32), &paint);
    dlb.draw_line((100.0f32, 150.0f32), (300.0f32, 350.0f32), &paint);

    let mut pb = C::PathBuilder::default();
    pb.move_to((100.0f32, 350.0f32));
    pb.line_to((300.0f32, 350.0f32));
    pb.line_to((300.0f32, 550.0f32));
    pb.line_to((100.0f32, 550.0f32));
    /*paint.set_color_source(Some(drawing_api::ColorSource::Image {
        image: resources.image2.clone(),
        horizontal_tile_mode: drawing_api::TileMode::Repeat,
        vertical_tile_mode: drawing_api::TileMode::Repeat,
        sampling: TextureSampling::NearestNeighbor,
        transformation: Some(
            Transform3D::identity()
                .pre_translate(Vector3D::new(100.0f32, 350.0f32, 0.0f32))
                .pre_rotate(0.0f32, 0.0f32, 1.0f32, Angle::radians(pos_y / 100.0f32)),
        ),
    }));*/
    dlb.draw_path(&pb.build().unwrap(), &paint);

    dlb.draw_texture_rect(
        &resources.image1,
        //rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, 0.0f32, 4.0f32, 4.0f32),
        rect(0.0f32, 0.0f32, 4.0f32, 4.0f32),
        TextureSampling::NearestNeighbor,
        None,
    );
    dlb.draw_line((0.0f32, 0.0f32), (4.0f32, 4.0f32), &paint);

    dlb.draw_texture_rect(
        &resources.image1,
        //rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, 0.0f32, 4.0f32, 4.0f32),
        rect(width as f32 - 4.0f32, 0.0f32, 4.0f32, 4.0f32),
        TextureSampling::NearestNeighbor,
        None,
    );
    dlb.draw_line(
        (width as f32 - 4.0f32, 4.0f32),
        (width as f32, 0.0f32),
        &paint,
    );

    dlb.draw_texture_rect(
        &resources.image1,
        //rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, 0.0f32, 4.0f32, 4.0f32),
        rect(
            width as f32 - 4.0f32,
            height as f32 - 4.0f32,
            4.0f32,
            4.0f32,
        ),
        TextureSampling::NearestNeighbor,
        None,
    );
    dlb.draw_line(
        (width as f32, height as f32),
        (width as f32 - 4.0f32, height as f32 - 4.0f32),
        &paint,
    );

    dlb.draw_texture_rect(
        &resources.image1,
        //rect(0.0f32, 0.0f32, 1.0f32, 1.0f32),
        rect(0.0f32, 0.0f32, 4.0f32, 4.0f32),
        rect(0.0f32, height as f32 - 4.0f32, 4.0f32, 4.0f32),
        TextureSampling::Linear,
        None,
    );
    dlb.draw_line(
        (0.0f32, height as f32),
        (4.0f32, height as f32 - 4.0f32),
        &paint,
    );

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.family = "F1".to_string();
    let mut font_paint = C::Paint::default();
    font_paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    paragraph_style.foreground = Some(font_paint);
    paragraph_style.size = 10.0f32;
    pb.push_style(paragraph_style);
    pb.add_text("Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0f32 + pos_y, 200.0f32), &paragraph);

    let mut pb = <C as drawing_api::Context>::ParagraphBuilder::new(&resources.fonts).unwrap();
    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.family = "F1".to_string();
    let mut font_paint = C::Paint::default();
    font_paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    paragraph_style.foreground = Some(font_paint);
    paragraph_style.size = 12.0f32;
    pb.push_style(paragraph_style);
    pb.add_text("Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0f32, 220.0f32 - pos_y), &paragraph);

    let mut pb = <C as drawing_api::Context>::ParagraphBuilder::new(&resources.fonts).unwrap();
    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.family = "F1".to_string();
    let mut font_paint = C::Paint::default();
    font_paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    paragraph_style.foreground = Some(font_paint);
    paragraph_style.size = 14.0f32;
    pb.push_style(paragraph_style);
    pb.add_text("Hello World!! yyy ąęśżółw,.\n01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0f32 - pos_y, 240.0f32 + pos_y * 2.0f32), &paragraph);

    let mut pb = <C as drawing_api::Context>::ParagraphBuilder::new(&resources.fonts).unwrap();
    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.family = "F1".to_string();
    let mut font_paint = C::Paint::default();
    font_paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    paragraph_style.foreground = Some(font_paint);
    paragraph_style.size = 16.0f32;
    pb.push_style(paragraph_style);
    pb.add_text("Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0f32 - pos_y, 260.0f32), &paragraph);

    let mut pb = <C as drawing_api::Context>::ParagraphBuilder::new(&resources.fonts).unwrap();
    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.family = "F1".to_string();
    let mut font_paint = C::Paint::default();
    font_paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    paragraph_style.foreground = Some(font_paint);
    paragraph_style.size = 18.0f32;
    pb.push_style(paragraph_style);
    pb.add_text("Hello World!! yyy ąęśżółw,. 01234567890 abcdefghijk ABCDEFGHIJK XYZ xyz");
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((350.0f32 + pos_y, 280.0f32 + pos_y), &paragraph);

    let mut pb = <C as drawing_api::Context>::PathBuilder::default();
    pb.move_to((100.0f32, 350.0f32));
    pb.bezier_curve_to(
        (120.0f32, 50.0f32),
        (180.0f32, 50.0f32),
        (300.0f32, 150.0f32),
    );
    paint.set_color_source(Some(drawing_api::ColorSource::LinearGradient {
        start: (100.0f32, 150.0f32).into(),
        end: (350.0f32, 350.0f32).into(),
        colors: vec![
            Color::rgba(1.0f32, 0.0f32, 0.0f32, 0.75f32),
            Color::rgba(1.0f32, 1.0f32, 0.0f32, 0.75f32),
        ],
        stops: vec![0.0f32, 1.0f32],
        tile_mode: drawing_api::TileMode::Mirror,
        transformation: None,
    }));
    dlb.draw_path(&pb.build().unwrap(), &paint);

    let mut pb = <C as drawing_api::Context>::PathBuilder::default();
    pb.move_to((500.0f32, 350.0f32));
    pb.bezier_curve_to(
        (520.0f32, 50.0f32),
        (580.0f32, 50.0f32),
        (700.0f32, 150.0f32),
    );
    pb.close();
    pb.move_to((450.0f32, 100.0f32));
    pb.line_to((650.0f32, 200.0f32));
    pb.line_to((750.0f32, 150.0f32));
    pb.close();
    paint.set_color_source(Some(drawing_api::ColorSource::LinearGradient {
        start: (500.0f32, 150.0f32).into(),
        end: (750.0f32, 350.0f32).into(),
        colors: vec![
            Color::rgba(1.0f32, 0.0f32, 0.0f32, 0.75f32),
            Color::rgba(1.0f32, 1.0f32, 0.0f32, 0.75f32),
        ],
        stops: vec![0.0f32, 1.0f32],
        tile_mode: drawing_api::TileMode::Mirror,
        transformation: None,
    }));
    dlb.draw_path(&pb.build().unwrap(), &paint);

    let mut pb = <C as drawing_api::Context>::PathBuilder::default();
    pb.move_to((300.0f32, 550.0f32));
    pb.bezier_curve_to(
        (320.0f32, 250.0f32),
        (380.0f32, 250.0f32),
        (500.0f32, 350.0f32),
    );
    pb.close();
    paint.set_color_source(Some(drawing_api::ColorSource::LinearGradient {
        start: (200.0f32, 450.0f32).into(),
        end: (450.0f32, 650.0f32).into(),
        colors: vec![
            Color::rgba(1.0f32, 0.0f32, 0.0f32, 0.75f32),
            Color::rgba(1.0f32, 1.0f32, 0.0f32, 0.75f32),
        ],
        stops: vec![0.0f32, 1.0f32],
        tile_mode: drawing_api::TileMode::Mirror,
        transformation: None,
    }));
    paint.set_draw_style(drawing_api::DrawStyle::Stroke);
    dlb.draw_path(&pb.build().unwrap(), &paint);

    let mut paint_layer = C::Paint::default();
    paint_layer.set_color(Color::rgba(1.0f32, 1.0f32, 1.0f32, 0.5f32));
    dlb.save_layer(
        rect(0.0f32, 0.0f32, 1000.0f32, 1000.0f32),
        Some(&paint_layer),
        None,
        /*None,
        Some(drawing_api::ImageFilter::Blur {
            x_sigma: 8.0f32,
            y_sigma: 8.0f32,
            tile_mode: drawing_api::TileMode::Clamp,
        }),*/
    );

    let mut paint2 = <C as Context>::Paint::default();
    paint2.set_color(Color::rgba(0.0f32, 0.5f32, 0.3f32, 1.0f32));
    dlb.draw_rect(rect(200.5f32, 220.5f32, 200.0f32, 50.0f32), &paint2);

    let mut pb = C::ParagraphBuilder::new(&resources.fonts).unwrap();
    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.family = "F1".to_string();
    let mut font_paint = C::Paint::default();
    font_paint.set_color(Color::rgb(1.0f32, 1.0f32, 1.0f32));
    paragraph_style.foreground = Some(font_paint);
    paragraph_style.size = 22.0f32;
    pb.push_style(paragraph_style);
    pb.add_text("Render target test");
    pb.pop_style();
    let paragraph = pb.build().unwrap();
    dlb.draw_paragraph((207.0f32, 232.0f32), &paragraph);

    dlb.restore();

    let display_list = dlb.build().unwrap();

    if let Some(ref mut drawing_context) = gl_window.gl_context {
        let mut drawing_surface = drawing_context
            .wrap_gl_framebuffer(
                framebuffer_id,
                width as u16,
                height as u16,
                drawing_api::ColorFormat::RGBA,
            )
            .unwrap();

        drawing_context
            .draw(&mut drawing_surface, &display_list)
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
