use crate::color::*;
use crate::primitive::*;
use crate::units::*;
use crate::Result;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ColoredVertex {
	pub pos: [f32; 2],
	pub color: [f32; 4],
}

impl ColoredVertex {
	pub fn new(pos: [f32; 2], color: [f32; 4]) -> Self {
		ColoredVertex { pos, color }
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TexturedVertex {
	pub pos: [f32; 2],
	pub tex_coords: [f32; 2],
	pub color: [f32; 4],
}

impl TexturedVertex {
	pub fn new(pos: [f32; 2], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
		TexturedVertex {
			pos,
			tex_coords,
			color,
		}
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TexturedY8Vertex {
	pub pos: [f32; 2],
	pub tex_coords: [f32; 2],
	pub color: [f32; 4],
}

impl TexturedY8Vertex {
	pub fn new(pos: [f32; 2], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
		TexturedY8Vertex {
			pos,
			tex_coords,
			color,
		}
	}
}

pub trait Device {
	type Texture: Texture;
	type RenderTarget;

	fn new() -> Result<Self>
	where
		Self: Sized;

	fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform;

	fn create_texture(
		&mut self,
		memory: Option<&[u8]>,
		width: u16,
		height: u16,
		format: ColorFormat,
		updatable: bool,
	) -> Result<Self::Texture>;

	fn create_render_target(
		&mut self,
		width: u16,
		height: u16,
	) -> Result<(Self::Texture, Self::RenderTarget)>;

	fn clear(&mut self, target: &Self::RenderTarget, color: &Color);

	fn triangles_colored(
		&mut self,
		target: &Self::RenderTarget,
		vertices: &[ColoredVertex],
		transform: UnknownToDeviceTransform,
	);

	fn triangles_textured(
		&mut self,
		target: &Self::RenderTarget,
		texture: &Self::Texture,
		filtering: bool,
		vertices: &[TexturedVertex],
		transform: UnknownToDeviceTransform,
	);

	fn triangles_textured_y8(
		&mut self,
		target: &Self::RenderTarget,
		texture: &Self::Texture,
		filtering: bool,
		vertices: &[TexturedY8Vertex],
		transform: UnknownToDeviceTransform,
	);

	fn line(
		&mut self,
		target: &Self::RenderTarget,
		color: &Color,
		thickness: DeviceThickness,
		start_point: Point,
		end_point: Point,
		transform: UnknownToDeviceTransform,
	);

	fn rect_colored(
		&mut self,
		target: &Self::RenderTarget,
		color: &Color,
		rect: Rect,
		transform: UnknownToDeviceTransform,
	) {
		let p1 = [rect.origin.x, rect.origin.y];
		let p2 = [
			rect.origin.x + rect.size.width,
			rect.origin.y + rect.size.height,
		];

		self.triangles_colored(
			target,
			&[
				ColoredVertex::new([p1[0], p1[1]], *color),
				ColoredVertex::new([p2[0], p1[1]], *color),
				ColoredVertex::new([p1[0], p2[1]], *color),
				ColoredVertex::new([p2[0], p1[1]], *color),
				ColoredVertex::new([p2[0], p2[1]], *color),
				ColoredVertex::new([p1[0], p2[1]], *color),
			],
			transform,
		);
	}

	fn rect_textured(
		&mut self,
		target: &Self::RenderTarget,
		texture: &Self::Texture,
		filtering: bool,
		color: &Color,
		rect: Rect,
		uv: &[f32; 4],
		transform: UnknownToDeviceTransform,
	) {
		let p1 = [rect.origin.x, rect.origin.y];
		let p2 = [
			rect.origin.x + rect.size.width,
			rect.origin.y + rect.size.height,
		];

		self.triangles_textured(
			target,
			texture,
			filtering,
			&[
				TexturedVertex::new([p1[0], p1[1]], [uv[0], uv[1]], *color),
				TexturedVertex::new([p2[0], p1[1]], [uv[2], uv[1]], *color),
				TexturedVertex::new([p1[0], p2[1]], [uv[0], uv[3]], *color),
				TexturedVertex::new([p2[0], p1[1]], [uv[2], uv[1]], *color),
				TexturedVertex::new([p2[0], p2[1]], [uv[2], uv[3]], *color),
				TexturedVertex::new([p1[0], p2[1]], [uv[0], uv[3]], *color),
			],
			transform,
		);
	}

	fn rect_textured_y8(
		&mut self,
		target: &Self::RenderTarget,
		texture: &Self::Texture,
		filtering: bool,
		color: &Color,
		rect: Rect,
		uv: &[f32; 4],
		transform: UnknownToDeviceTransform,
	) {
		let p1 = [rect.origin.x, rect.origin.y];
		let p2 = [
			rect.origin.x + rect.size.width,
			rect.origin.y + rect.size.height,
		];

		self.triangles_textured_y8(
			target,
			texture,
			filtering,
			&[
				TexturedY8Vertex::new([p1[0], p1[1]], [uv[0], uv[1]], *color),
				TexturedY8Vertex::new([p2[0], p1[1]], [uv[2], uv[1]], *color),
				TexturedY8Vertex::new([p1[0], p2[1]], [uv[0], uv[3]], *color),
				TexturedY8Vertex::new([p2[0], p1[1]], [uv[2], uv[1]], *color),
				TexturedY8Vertex::new([p2[0], p2[1]], [uv[2], uv[3]], *color),
				TexturedY8Vertex::new([p1[0], p2[1]], [uv[0], uv[3]], *color),
			],
			transform,
		);
	}

	// paths

	fn stroke(&mut self, path: &[PathElement], thickness: f32, brush: &Brush) {}

	fn stroke_styled(
		&mut self,
		path: &[PathElement],
		thickness: f32,
		brush: &Brush,
		style: &StrokeStyle,
	) {
	}

	fn fill(&mut self, path: &[PathElement], brush: &Brush) {}

	// state

	fn save_state(&mut self) {}

	fn restore_state(&mut self) {}

	fn set_clip_rect(&mut self, rect: UserPixelRect) {}

	fn set_clip_path(&mut self, path: &[PathElement]) {}

	fn transform(&mut self, transform: UnknownToDeviceTransform) {}
}

pub trait Texture: Sized {
	fn get_size(&self) -> (u16, u16);

	fn update(
		&mut self,
		memory: &[u8],
		offset_x: u16,
		offset_y: u16,
		width: u16,
		height: u16,
	) -> Result<()>;
}
