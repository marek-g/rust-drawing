pub type Color = [f32; 4];

pub fn convert_color(color: &drawing_api::Color) -> Color {
    [color.red, color.green, color.blue, color.alpha]
}
