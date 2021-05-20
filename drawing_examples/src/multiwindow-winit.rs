#![windows_subsystem = "windows"]

use drawing::backend::{Device, RenderTarget};
use drawing::font::Font;
use drawing::primitive::Primitive;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;

use drawing_gl::{GlContextData, GlDevice, GlRenderTarget};

use rust_embed::RustEmbed;
use std::cell::{Ref, RefCell};

type DrawingDevice = drawing_gl::GlDevice;
type DrawingFont = drawing::TextureFont<DrawingDevice>;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let mut device = DrawingDevice::new().unwrap();
    let mut renderer = Renderer::new();

    let window_builder1 = winit::window::WindowBuilder::new().with_title("Window 1 (winit)");
    let mut window_target1 =
        create_window_target(&mut device, window_builder1, &event_loop, None).unwrap();

    let window_builder2 = winit::window::WindowBuilder::new().with_title("Window 2 (winit)");
    let mut window_target2 = create_window_target(
        &mut device,
        window_builder2,
        &event_loop,
        Some(&window_target1),
    )
    .unwrap();

    //
    // create resources
    //
    let mut resources = Resources::new();

    let font = DrawingFont::create(
        &mut device,
        Assets::get("OpenSans-Regular.ttf").unwrap().to_vec(),
    )
    .unwrap();

    resources.fonts_mut().insert("F1".to_string(), font);

    //
    // main loop
    //
    event_loop.run(move |event, _, control_flow| {
        if let winit::event::Event::MainEventsCleared = event {
            // Application update code.
            // Queue a RedrawRequested event.
            window_target1.get_window().request_redraw();
            window_target2.get_window().request_redraw();
        };

        if let winit::event::Event::RedrawRequested(ref window_id) = event {
            let _window_target = if window_id == &window_target1.get_window().id() {
                draw_window(
                    &mut device,
                    &mut renderer,
                    &mut resources,
                    &mut window_target1,
                    "Window 1",
                );

                window_target1.swap_buffers();
            } else {
                draw_window(
                    &mut device,
                    &mut renderer,
                    &mut resources,
                    &mut window_target2,
                    "Window 2",
                );

                window_target2.swap_buffers();
            };
        };

        if let winit::event::Event::WindowEvent {
            ref window_id,
            ref event,
        } = event
        {
            let window_target = if window_id == &window_target1.get_window().id() {
                &mut window_target1
            } else {
                &mut window_target2
            };

            match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }

                winit::event::WindowEvent::Resized(physical_size) => window_target
                    .update_size(physical_size.width as u16, physical_size.height as u16),

                _ => (),
            }
        };
    });
}

fn draw_window(
    device: &mut DrawingDevice,
    renderer: &mut Renderer,
    resources: &mut Resources<DrawingDevice, DrawingFont>,
    window_target: &mut GlWindowTarget,
    text: &str,
) {
    let physical_size = window_target.get_window().inner_size();
    let width = physical_size.width as f32;
    let height = physical_size.height as f32;

    if width > 0.0 && height > 0.0 {
        let primitives = vec![Primitive::Text {
            resource_key: "F1".to_string(),
            color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            position: PixelPoint::new(width / 2.0f32, height / 2.0f32),
            clipping_rect: PixelRect::new(
                PixelPoint::new(0.0f32, 0.0f32),
                PixelSize::new(width, height),
            ),
            size: PixelThickness::new(20.0f32),
            text: text.to_string(),
        }];

        // make current context
        window_target.make_current_context();

        device.begin(&window_target.gl_context_data).unwrap();
        device.clear(
            window_target.get_render_target(),
            &[0.5f32, 0.4f32, 0.3f32, 1.0f32],
        );
        renderer
            .draw(
                device,
                window_target.get_render_target(),
                &primitives,
                resources,
                false,
            )
            .unwrap();
    }
}

pub fn create_window_target(
    device: &mut GlDevice,
    window_builder: winit::window::WindowBuilder,
    events_loop: &winit::event_loop::EventLoop<()>,
    shared_window_target: Option<&GlWindowTarget>,
) -> Result<GlWindowTarget, ()> {
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_vsync(true);

    // make current gl context
    let windowed_context = if let Some(ref shared_window_target) = shared_window_target {
        if let Some(ref gl_windowed_context) =
            shared_window_target.gl_windowed_context.borrow().as_ref()
        {
            unsafe {
                context_builder
                    .with_shared_lists(gl_windowed_context.context())
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
                    .make_current()
                    .unwrap()
            }
        } else {
            unsafe {
                context_builder
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
                    .make_current()
                    .unwrap()
            }
        }
    } else {
        unsafe {
            context_builder
                .build_windowed(window_builder, &events_loop)
                .unwrap()
                .make_current()
                .unwrap()
        }
    };

    // initialize gl context
    let gl_context_data = device
        .init_context(|symbol| windowed_context.context().get_proc_address(symbol) as *const _);

    let aspect_ratio = windowed_context.window().scale_factor() as f32;

    Ok(GlWindowTarget {
        gl_windowed_context: RefCell::new(Some(windowed_context)),
        gl_context_data,
        gl_render_target: GlRenderTarget::new(0, 0, 0, aspect_ratio),
    })
}

pub struct GlWindowTarget {
    gl_windowed_context:
        RefCell<Option<glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>>>,
    gl_context_data: GlContextData,
    gl_render_target: GlRenderTarget,
}

impl GlWindowTarget {
    pub fn get_window(&self) -> Ref<winit::window::Window> {
        Ref::map(self.gl_windowed_context.borrow(), |context| {
            context.as_ref().unwrap().window()
        })
    }

    pub fn get_render_target(&self) -> &GlRenderTarget {
        &self.gl_render_target
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        unsafe {
            self.gl_render_target.update_size(width, height);
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.gl_windowed_context
            .borrow()
            .as_ref()
            .unwrap()
            .swap_buffers()
            .unwrap();
    }

    pub fn make_current_context(&mut self) {
        unsafe {
            let context = self.gl_windowed_context.replace(None);
            let context = context.unwrap().make_current().unwrap();
            self.gl_windowed_context.replace(Some(context));
        }
    }

    pub fn get_context(
        &self,
    ) -> Ref<glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>> {
        Ref::map(self.gl_windowed_context.borrow(), |context| {
            context.as_ref().unwrap()
        })
    }
}

impl Drop for GlWindowTarget {
    fn drop(&mut self) {
        unsafe {
            let context = self.gl_windowed_context.replace(None);
            let context = context.unwrap().make_current().unwrap();
            self.gl_windowed_context.replace(Some(context));
        }
    }
}
