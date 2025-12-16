#![windows_subsystem = "windows"]

use drawing_examples::setup_window;
use drawing_impeller::ImpellerContextGl;
use std::error::Error;

use windowing_qt::{Application, ApplicationOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        ApplicationOptions::new()
            .with_title("Example: simple-impeller-gl")
            .with_opengl_stencil_bits(8) // needed for paths
            .with_opengl_depth_bits(16), // needed for clipping
    )
    .unwrap();

    let _gl_window_rc = setup_window::<ImpellerContextGl>("Window");

    app.message_loop();

    Ok(())
}
