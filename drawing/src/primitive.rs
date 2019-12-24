use crate::color::*;
use crate::units::*;

pub enum Primitive {
    Line {
        color: Color,
        thickness: UserPixelThickness,
        start_point: UserPixelPoint,
        end_point: UserPixelPoint,
    },

    Rectangle {
        color: Color,
        rect: UserPixelRect,
    },

    Image {
        resource_key: i32,
        rect: UserPixelRect,
        uv: [f32; 4],
    },

    Text {
        resource_key: String,
        size: u16,
        color: Color,
        position: UserPixelPoint,
        clipping_rect: UserPixelRect,
        text: String,
    },

    Stroke {
        path: Vec<PathElement>,
        thickness: f32,
        brush: Brush,
    },

    StrokeStyled {
        path: Vec<PathElement>,
        thickness: f32,
        brush: Brush,
        style: StrokeStyle,
    },

    Fill {
        path: Vec<PathElement>,
        brush: Brush,
    },

    ClipRect {
        rect: UserPixelRect,
        primitives: Vec<Primitive>,
    },

    ClipPath {
        path: Vec<PathElement>,
        primitives: Vec<Primitive>,
    },

    Transform {
        transform: UnknownToDeviceTransform,
        primitives: Vec<Primitive>,
    },

    Composite {
        color: Color,
        primitives: Vec<Primitive>,
    },
}

pub enum PathElement {
    MoveTo {
        point: UserPixelPoint,
    },

    LineTo {
        point: UserPixelPoint,
    },

    BezierTo {
        point: UserPixelPoint,
        c1: UserPixelPoint,
        c2: UserPixelPoint,
    },

    ClosePath,

    Solidity {
        solidity: Solidity,
    },
}

pub enum Brush {
    Color {
        color: Color,
    },

    LinearGradient {
        start_point: UserPixelPoint,
        end_point: UserPixelPoint,
        inner_color: Color,
        outer_color: Color,
    },

    RadialGradient {
        center_point: UserPixelPoint,
        in_radius: f32,
        out_radius: f32,
        inner_color: Color,
        outer_color: Color,
    },

    ShadowGradient {
        rect: UserPixelRect,
        radius: f32,
        feather: f32,
        inner_color: Color,
        outer_color: Color,
    },

    ImagePattern {
        resource_key: i32,
        center: UserPixelPoint,
        size: UserPixelSize,
        angle: f32,
        alpha: f32,
    },
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Copy, Clone)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Copy, Clone)]
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
