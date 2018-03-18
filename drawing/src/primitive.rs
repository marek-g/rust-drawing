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
        font_path: &'a str,
        color: Color,
        position: UserPixelPoint,
        size: u16,
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