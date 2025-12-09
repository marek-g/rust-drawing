#![windows_subsystem = "windows"]

use drawing_examples::setup_window;
use drawing_gl::GlContext;
use std::error::Error;

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: simple-gl")
            .with_opengl_stencil_bits(8)
            .with_force_xwayland(true),
    )
    .unwrap();

    let _gl_window_rc = setup_window::<GlContext>("Window");

    app.message_loop();

    Ok(())
}
