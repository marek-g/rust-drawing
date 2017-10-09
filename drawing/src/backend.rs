extern crate winit;

use color::*;
use units::*;

pub trait Backend {
	fn create_backend_window(window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self;

	fn update_window_size(&mut self, width: u16, height: u16);
	
	fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform;

	fn begin(&mut self);
	
	fn line(&mut self, color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform);

	fn rect(&mut self, color: &Color, rect: Rect,
        transform: UnknownToDeviceTransform);

	fn end(&mut self);
}