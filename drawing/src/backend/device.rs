use anyhow::Result;
use core::marker::Sized;
use core::option::Option;
use crate::backend::{RenderTarget, Texture};
use crate::backend::colored_vertex::ColoredVertex;
use crate::backend::textured_vertex::TexturedVertex;
use crate::backend::textured_y8_vertex::TexturedY8Vertex;
use crate::clipping::Scissor;
use crate::color::{Color, ColorFormat};
use crate::composite_operation_state::CompositeOperationState;
use crate::paint::Paint;
use crate::path::{Bounds, Path};
use crate::primitive::PathElement;
use crate::units::{DeviceThickness, PixelRect, Point, Rect, UnknownToDeviceTransform};

pub trait Device {
    type Texture: Texture;
    type RenderTarget: RenderTarget;

    fn new() -> Result<Self>
    where
        Self: Sized;

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

    fn stroke(
        &mut self,
        target: &Self::RenderTarget,
        paint: &Paint,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[Path],
        thickness: f32,
        fringe_width: f32,
        antialiasing: bool,
        scissor: Scissor,
        composite_operation_state: CompositeOperationState,
        transform: UnknownToDeviceTransform,
    );

    fn fill(
        &mut self,
        target: &Self::RenderTarget,
        paint: &Paint,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[Path],
        bounds: Bounds,
        fringe_width: f32,
        antialiasing: bool,
        scissor: Scissor,
        composite_operation_state: CompositeOperationState,
        transform: UnknownToDeviceTransform,
    );

    // state

    fn save_state(&mut self) {}

    fn restore_state(&mut self) {}

    fn set_clip_rect(&mut self, _rect: PixelRect) {}

    fn set_clip_path(&mut self, _path: &[PathElement]) {}

    fn transform(&mut self, _transform: UnknownToDeviceTransform) {}
}
