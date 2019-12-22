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
        rect: UserPixelRect,
        angle: f32,
        alpha: f32,
    },
}

pub struct StrokeStyle {
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
}

pub enum LineCap {
    Butt,
    Round,
    Square,
}

pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}
