extern crate winit;
extern crate drawing;
extern crate drawing_gfx;

use drawing::backend::Backend;
use drawing::renderer::Renderer;
use drawing::primitive::Primitive;
use drawing::units::*;
use drawing_gfx::backend::GfxBackend;

fn main() {
    let window_builder = winit::WindowBuilder::new()
            .with_title("Title");
    let mut events_loop = winit::EventsLoop::new(); 


    let mut renderer = Renderer::new(GfxBackend::create_backend_window(window_builder, &events_loop));

    // main loop
    let mut running = true;
    let mut width = 0;
    let mut height = 0;
    while running {
        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                match event {
                    winit::WindowEvent::KeyboardInput {
                        input: winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                            .. },
                        ..
                    } | winit::WindowEvent::Closed => running = false,
                    winit::WindowEvent::Resized(w, h) => {
                        width = w; height = h;
                        renderer.update_window_size(w as u16, h as u16)
                    }
                    _ => (),
                }
            }
        });

        if width <= 0 || height <= 0 { continue }

        let primitives = vec![
            Primitive::Rectangle { color: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
                rect: UserPixelRect::new(UserPixelPoint::new(100.5f32, 101.5f32),
                    UserPixelSize::new(200.0f32, 50.0f32)) },
            Primitive::Line { color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(100.5f32, 100.5f32),
                end_point: UserPixelPoint::new(300.5f32, 100.5f32) }
        ];
        renderer.draw(PhysPixelSize::new(width as f32, height as f32), primitives);
    }
}
