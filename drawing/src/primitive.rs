use color::*;
use units::*;

pub enum Primitive<'a> {
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
        text: &'a str,
    },
    Image {
        path: &'a str,
        rect: UserPixelRect,
    },
    PushLayer {
        opacity: u8
    },
    PopLayer
}