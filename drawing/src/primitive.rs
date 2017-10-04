use color::*;
use units::*;

pub enum Primitive {
    Line {
        color: Color,
        thickness: UserPixelThickness,
        start_point: UserPixelPoint,
        end_point: UserPixelPoint
    },
    Rectangle {
        color: Color,
        rect: UserPixelRect
    },
    Text {
        color: Color,
        position: UserPixelPoint,
        size: UserPixelThickness,
        text: &'static str,
    },
    PushLayer {
        opacity: u8
    },
    PopLayer
}