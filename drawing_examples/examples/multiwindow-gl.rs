#![windows_subsystem = "windows"]

use drawing_examples::setup_window;
use drawing_gl::GlContext;
use std::error::Error;

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: multiwindow-gl")
            .with_opengl_stencil_bits(8)
            .with_force_xwayland(true),
    )
    .unwrap();

    let _gl_window1_rc = setup_window::<GlContext>("Window 1");
    let _gl_window2_rc = setup_window::<GlContext>("Window 2");

    app.message_loop();

    Ok(())
}
