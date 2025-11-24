use drawing_api::{PixelLength, PixelPoint, PixelRect, PixelTransform};

use crate::generic::device::Color;

#[derive(Clone, Debug)]
pub enum Primitive<Texture: drawing_api::Texture, Fonts: drawing_api::Fonts> {
    Clear {
        color: Color,
    },

    Line {
        color: Color,
        thickness: PixelLength,
        start_point: PixelPoint,
        end_point: PixelPoint,
    },

    Rectangle {
        color: Color,
        rect: PixelRect,
    },

    Image {
        texture: Texture,
        rect: PixelRect,
        uv: [f32; 4],
    },

    Text {
        fonts: Fonts,
        family_name: String,
        size: PixelLength,
        color: Color,
        position: PixelPoint,
        clipping_rect: Option<PixelRect>,
        text: String,
    },

    Stroke {
        path: Vec<PathElement>,
        thickness: PixelLength,
        brush: Brush<Texture>,
    },

    StrokeStyled {
        path: Vec<PathElement>,
        thickness: PixelLength,
        brush: Brush<Texture>,
        style: StrokeStyle,
    },

    Fill {
        path: Vec<PathElement>,
        brush: Brush<Texture>,
    },

    ClipRect {
        rect: PixelRect,
        primitives: Vec<Primitive<Texture, Fonts>>,
    },

    ClipPath {
        path: Vec<PathElement>,
        primitives: Vec<Primitive<Texture, Fonts>>,
    },

    Transform {
        transform: PixelTransform,
        primitives: Vec<Primitive<Texture, Fonts>>,
    },

    Composite {
        color: Color,
        primitives: Vec<Primitive<Texture, Fonts>>,
    },
}

#[derive(Clone, Debug)]
pub enum PathElement {
    MoveTo(PixelPoint),

    LineTo(PixelPoint),

    BezierTo(PixelPoint, PixelPoint, PixelPoint),

    ClosePath,

    Solidity(Solidity),
}

#[derive(Clone, Debug)]
pub enum Brush<Texture: drawing_api::Texture> {
    Color {
        color: Color,
    },

    LinearGradient {
        start_point: PixelPoint,
        end_point: PixelPoint,
        inner_color: Color,
        outer_color: Color,
    },

    RadialGradient {
        center_point: PixelPoint,
        in_radius: f32,
        out_radius: f32,
        inner_color: Color,
        outer_color: Color,
    },

    ShadowGradient {
        rect: PixelRect,
        radius: f32,
        feather: f32,
        inner_color: Color,
        outer_color: Color,
    },

    ImagePattern {
        texture: Texture,
        transform: PixelTransform,
        alpha: f32,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct StrokeStyle {
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        StrokeStyle {
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 10.0f32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Copy, Clone)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Copy, Clone)]
pub enum Solidity {
    Solid,
    Hole,
}

#[derive(Debug, Copy, Clone)]
pub enum CompositeOperation {
    Basic(BasicCompositeOperation),
    BlendFunc {
        src: BlendFactor,
        dst: BlendFactor,
    },
    BlendFuncSeparate {
        src_rgb: BlendFactor,
        dst_rgb: BlendFactor,
        src_alpha: BlendFactor,
        dst_alpha: BlendFactor,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum BasicCompositeOperation {
    SrcOver,
    SrcIn,
    SrcOut,
    Atop,
    DstOver,
    DstIn,
    DstOut,
    DstAtop,
    Lighter,
    Copy,
    Xor,
}

#[derive(Debug, Copy, Clone)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturate,
}
