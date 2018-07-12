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
        resource_key: String,
        size: u16,
        color: Color,
        position: UserPixelPoint,
        text: String,
    },
    Image {
        resource_key: i32,
        rect: UserPixelRect,
    },
    PushLayer {
        color: Color,
    },
    PopLayer
}
