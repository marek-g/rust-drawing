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
        resource_key: &'a str,
        size: u16,
        color: Color,
        position: UserPixelPoint,
        text: &'a str,
    },
    Image {
        resource_key: i32,
        rect: UserPixelRect,
    },
    PushLayer {
        opacity: u8
    },
    PopLayer
}