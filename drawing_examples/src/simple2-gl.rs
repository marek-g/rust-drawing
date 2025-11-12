#![windows_subsystem = "windows"]

use drawing_gl::{GlContextData, GlDevice, GlRenderTarget};
use euclid::{Angle, Vector2D};
use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::error::Error;
use std::ptr::null;
use std::rc::Rc;

use gl::types::*;
use windowing_qt::{Application, ApplicationOptions};

type DrawingContext = drawing_gl::GlContext;

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
    context: &mut DrawingContext,
    gl_window: &mut GlWindow,
    //app_resources: &mut AppResources,
) {
    let width = gl_window.window.get_width();
    let height = gl_window.window.get_height();

    if width <= 0 || height <= 0 {
        return;
    }

    context.draw();
}
