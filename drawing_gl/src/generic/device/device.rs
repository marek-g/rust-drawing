use crate::generic::clipping::Scissor;
use crate::generic::device::colored_vertex::ColoredVertex;
use crate::generic::device::textured_vertex::TexturedVertex;
use crate::generic::device::textured_y8_vertex::TexturedY8Vertex;
use crate::generic::device::Paint;
use crate::generic::device::RenderTarget;
use crate::generic::path::{Bounds, Path};
use crate::generic::renderer::CompositeOperationState;
use crate::generic::transformation::Transformation;
use crate::units::PixelToDeviceTransform;
use crate::units::PixelToUvTransform;
use crate::PathElement;
use core::option::Option;
use drawing_api::ColorFormat;
use drawing_api::Texture;
use drawing_api::{PixelPoint, PixelRect};

use super::Color;

pub trait Device {
    type Texture: Texture;
    type RenderTarget: RenderTarget;

    fn create_texture(
        &self,
        contents: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
    ) -> Result<Self::Texture, &'static str>;

    fn create_render_target(
        &mut self,
        width: u16,
        height: u16,
    ) -> Result<(Self::Texture, Self::RenderTarget), &'static str>;

    fn clear(&mut self, target: &Self::RenderTarget, color: &Color);

    fn triangles_colored(
        &mut self,
        target: &Self::RenderTarget,
        vertices: &[ColoredVertex],
        transform: PixelToDeviceTransform,
    );

    fn triangles_textured(
        &mut self,
        target: &Self::RenderTarget,
        texture: &Self::Texture,
        filtering: bool,
        vertices: &[TexturedVertex],
        transform: PixelToDeviceTransform,
    );

    fn triangles_textured_y8(
        &mut self,
        target: &Self::RenderTarget,
        texture: &Self::Texture,
        filtering: bool,
        vertices: &[TexturedY8Vertex],
        transform: PixelToDeviceTransform,
    );

    fn line(
        &mut self,
        target: &Self::RenderTarget,
        color: &Color,
        thickness: f32,
        start_point: PixelPoint,
        end_point: PixelPoint,
        transform: PixelToDeviceTransform,
    );

    fn rect_colored(
        &mut self,
        target: &Self::RenderTarget,
        color: &Color,
        rect: PixelRect,
        transform: PixelToDeviceTransform,
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
        rect: PixelRect,
        src: PixelRect,
        transform: PixelToDeviceTransform,
        uv_transform: PixelToUvTransform,
    ) {
        let p1 = [rect.origin.x, rect.origin.y];
        let p2 = [
            rect.origin.x + rect.size.width,
            rect.origin.y + rect.size.height,
        ];

        let x1y1 = uv_transform.transform_point(src.origin);
        let x2y2 = uv_transform.transform_point(PixelPoint::new(
            src.origin.x + src.size.width,
            src.origin.y + src.size.height,
        ));
        let uv = [x1y1.x, x1y1.y, x2y2.x, x2y2.y];

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
        rect: PixelRect,
        src: PixelRect,
        transform: PixelToDeviceTransform,
        uv_transform: PixelToUvTransform,
    ) {
        let p1 = [rect.origin.x, rect.origin.y];
        let p2 = [
            rect.origin.x + rect.size.width,
            rect.origin.y + rect.size.height,
        ];

        let x1y1 = uv_transform.transform_point(src.origin);
        let x2y2 = uv_transform.transform_point(PixelPoint::new(
            src.origin.x + src.size.width,
            src.origin.y + src.size.height,
        ));
        let uv = [x1y1.x, x1y1.y, x2y2.x, x2y2.y];

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
        paint: &Paint<Self::Texture>,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[Path],
        thickness: f32,
        fringe_width: f32,
        antialiasing: bool,
        scissor: Scissor,
        composite_operation_state: CompositeOperationState,
        transform: PixelToDeviceTransform,
    );

    fn fill(
        &mut self,
        target: &Self::RenderTarget,
        paint: &Paint<Self::Texture>,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[Path],
        bounds: Bounds,
        fringe_width: f32,
        antialiasing: bool,
        scissor: Scissor,
        composite_operation_state: CompositeOperationState,
        transform: PixelToDeviceTransform,
    );

    // state

    fn save_state(&mut self) {}

    fn restore_state(&mut self) {}

    fn set_clip_rect(&mut self, _rect: PixelRect) {}

    fn set_clip_path(&mut self, _path: &[PathElement]) {}

    fn transform(&mut self, _transform: PixelToDeviceTransform) {}
}
