#![windows_subsystem = "windows"]

use drawing_examples::setup_window;
use std::error::Error;

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: multiwindow-gl")
            .with_opengl_stencil_bits(8),
    )
    .unwrap();

    let _gl_window1_rc = setup_window("Window 1");
    let _gl_window2_rc = setup_window("Window 2");

    app.message_loop();

    Ok(())
}
