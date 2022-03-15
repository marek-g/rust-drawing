const INSIDE: u32 = 0;
const LEFT: u32 = 1;
const RIGHT: u32 = 2;
const BOTTOM: u32 = 4;
const TOP: u32 = 8;

fn clip_code(x: f32, y: f32, clip_x1: f32, clip_y1: f32, clip_width: f32, clip_height: f32) -> u32 {
    let mut code = INSIDE;
    if x < clip_x1 {
        code |= LEFT
    };
    if x > clip_x1 + clip_width {
        code |= RIGHT
    };
    if y < clip_y1 {
        code |= BOTTOM
    };
    if y > clip_y1 + clip_height {
        code |= TOP
    };
    code
}

// Cohen–Sutherland Algorithm for line clipping
pub fn clip_line(
    mut x1: f32,
    mut y1: f32,
    mut x2: f32,
    mut y2: f32,
    clip_x1: f32,
    clip_y1: f32,
    clip_width: f32,
    clip_height: f32,
) -> Option<(f32, f32, f32, f32)> {
    let mut code1 = clip_code(x1, y1, clip_x1, clip_y1, clip_width, clip_height);
    let mut code2 = clip_code(x2, y2, clip_x1, clip_y1, clip_width, clip_height);
    let mut accept = false;
    loop {
        if code1 == INSIDE && code2 == INSIDE {
            accept = true;
            break;
        } else if code1 & code2 != INSIDE {
            break;
        } else {
            let x;
            let y;

            let code_out = if code1 != INSIDE { code1 } else { code2 };

            if code_out & TOP != INSIDE {
                x = x1 + (x2 - x1) * (clip_y1 + clip_height - y1) / (y2 - y1);
                y = clip_y1 + clip_height;
            } else if code_out & BOTTOM != INSIDE {
                x = x1 + (x2 - x1) * (clip_y1 - y1) / (y2 - y1);
                y = clip_y1;
            } else if code_out & RIGHT != INSIDE {
                y = y1 + (y2 - y1) * (clip_x1 + clip_width - x1) / (x2 - x1);
                x = clip_x1 + clip_width;
            } else {
                y = y1 + (y2 - y1) * (clip_x1 - x1) / (x2 - x1);
                x = clip_x1;
            }

            if code_out == code1 {
                x1 = x;
                y1 = y;
                code1 = clip_code(x1, y1, clip_x1, clip_y1, clip_width, clip_height);
            } else {
                x2 = x;
                y2 = y;
                code2 = clip_code(x2, y2, clip_x1, clip_y1, clip_width, clip_height);
            }
        }
    }

    if accept {
        Some((x1, y1, x2, y2))
    } else {
        None
    }
}

pub fn clip_rect(
    x1: f32,
    y1: f32,
    width: f32,
    height: f32,
    clip_x1: f32,
    clip_y1: f32,
    clip_width: f32,
    clip_height: f32,
) -> Option<(f32, f32, f32, f32)> {
    let overlap_x1 = x1.max(clip_x1);
    let overlap_x2 = (x1 + width).min(clip_x1 + clip_width);

    if overlap_x2 <= overlap_x1 {
        None
    } else {
        let overlap_y1 = y1.max(clip_y1);
        let overlap_y2 = (y1 + height).min(clip_y1 + clip_height);

        if overlap_y2 <= overlap_y1 {
            None
        } else {
            Some((
                overlap_x1,
                overlap_y1,
                overlap_x2 - overlap_x1,
                overlap_y2 - overlap_y1,
            ))
        }
    }
}

pub fn clip_image(
    x1: f32,
    y1: f32,
    width: f32,
    height: f32,
    clip_x1: f32,
    clip_y1: f32,
    clip_width: f32,
    clip_height: f32,
    uv: &[f32; 4],
) -> Option<(f32, f32, f32, f32, [f32; 4])> {
    clip_rect(
        x1,
        y1,
        width,
        height,
        clip_x1,
        clip_y1,
        clip_width,
        clip_height,
    )
    .map(|clipped| {
        (
            clipped.0,
            clipped.1,
            clipped.2,
            clipped.3,
            [
                ((clipped.0 - x1) / width) * (uv[2] - uv[0]) + uv[0],
                ((clipped.1 - y1) / height) * (uv[3] - uv[1]) + uv[1],
                ((clipped.0 + clipped.2 - x1) / width) * (uv[2] - uv[0]) + uv[0],
                ((clipped.1 + clipped.3 - y1) / height) * (uv[3] - uv[1]) + uv[1],
            ],
        )
    })
}
