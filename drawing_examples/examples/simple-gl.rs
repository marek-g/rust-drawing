#![windows_subsystem = "windows"]

use drawing_examples::setup_window;
use drawing_gl::GlContext;
use std::error::Error;

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: simple-gl")
            .with_opengl_stencil_bits(8),
    )
    .unwrap();

    let new_context_func = |gl_window_rc| {
        GlContext::new_gl_context(|symbol| {
            gl_window_rc
                .borrow_mut()
                .window
                .get_opengl_proc_address(symbol)
                .unwrap_or_else(|_| null())
        })
        .unwrap()
    };

    let _gl_window_rc = setup_window("Window", new_context_func);

    app.message_loop();

    Ok(())
}
