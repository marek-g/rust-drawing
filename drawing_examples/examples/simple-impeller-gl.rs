#![windows_subsystem = "windows"]

use drawing_examples::setup_window;
use drawing_impeller::ImpellerContext;
use std::error::Error;

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: simple-impeller-gl")
            .with_opengl_stencil_bits(8),
    )
    .unwrap();

    let _gl_window_rc = setup_window::<ImpellerContext>("Window");

    app.message_loop();

    Ok(())
}
