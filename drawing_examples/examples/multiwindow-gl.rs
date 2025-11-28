#![windows_subsystem = "windows"]

use drawing_examples::{setup_window, GlWindow};
use drawing_gl::GlContext;
use std::{cell::RefCell, error::Error, ptr::null, rc::Rc};

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: multiwindow-gl")
            .with_opengl_stencil_bits(8),
    )
    .unwrap();

    let new_context_func = |gl_window_rc: Rc<RefCell<GlWindow<GlContext>>>| {
        GlContext::new_gl_context(|symbol| {
            gl_window_rc
                .borrow_mut()
                .window
                .get_opengl_proc_address(symbol)
                .unwrap_or_else(|_| null())
        })
        .unwrap()
    };

    let _gl_window1_rc = setup_window("Window 1", new_context_func);
    let _gl_window2_rc = setup_window("Window 2", new_context_func);

    app.message_loop();

    Ok(())
}
