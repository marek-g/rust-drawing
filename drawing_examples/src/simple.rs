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
                    _ => (),
                }
            }
        });

        let primitives = vec![
            Primitive::Rectangle { color: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
                rect: UserPixelRect::new(UserPixelPoint::new(100.0f32, 100.0f32),
                    UserPixelSize::new(200.0f32, 50.0f32)) },
            Primitive::Line { color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
                thickness: UserPixelThickness::new(1.0f32),
                start_point: UserPixelPoint::new(100.0f32, 100.0f32),
                end_point: UserPixelPoint::new(300.0f32, 100.0f32) }
        ];
        renderer.draw(PhysPixelSize::new(800.0f32, 600.0f32), primitives);
    }
}
