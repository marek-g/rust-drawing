use drawing_api::*;

use crate::units::PixelTransform;

//
// Scissoring
//
// Scissoring allows you to clip the rendering into a rectangle. This is useful for various
// user interface cases like rendering a text edit or a timeline.
#[derive(Debug, Copy, Clone)]
pub struct Scissor {
    pub xform: PixelTransform, // transform of the middle of the rect
    pub extent: [f32; 2],      // half ot the width & height
}

impl Scissor {
    pub fn empty() -> Self {
        Scissor {
            xform: PixelTransform::identity(),
            extent: [-1.0, -1.0],
        }
    }

    pub fn new<R: Into<PixelRect>>(rect: R) -> Self {
        let rect = rect.into();
        let width = rect.size.width.max(0.0);
        let height = rect.size.height.max(0.0);
        Scissor {
            xform: PixelTransform::identity().then_translate(euclid::Vector2D::new(
                rect.origin.x + width * 0.5f32,
                rect.origin.y + height * 0.5f32,
            )),
            extent: [width * 0.5f32, height * 0.5f32],
        }
    }

    // Intersects current scissor rectangle with the specified rectangle.
    // The scissor rectangle is transformed by the current transform.
    // Note: in case the rotation of previous scissor rect differs from
    // the current one, the intersection will be done between the specified
    // rectangle and the previous scissor rectangle transformed in the current
    // transform space. The resulting shape is always rectangle.
    pub fn intersect_with_rect<R: Into<PixelRect>>(
        &self,
        rect: R,
        current_transform: &PixelTransform,
    ) -> Self {
        let rect = rect.into();

        // If no previous scissor has been set, set the scissor as current scissor.
        if self.extent[0] < 0.0 {
            return Self::new(rect);
        }

        // Transform the current scissor rect into current transform space.
        // If there is difference in rotation, this will be approximation.
        let [ex, ey] = self.extent;
        let invxorm = current_transform
            .inverse()
            .unwrap_or_else(PixelTransform::identity);
        let pxform = self.xform.then(&invxorm);
        let tex = ex * pxform.m11.abs() + ey * pxform.m21.abs();
        let tey = ex * pxform.m12.abs() + ey * pxform.m22.abs();
        if let Some(new_rect) = PixelRect::new(
            PixelPoint::new(pxform.m31 - tex, pxform.m32 - tey),
            PixelSize::new(tex * 2.0, tey * 2.0),
        )
        .intersection(&rect)
        {
            Self::new(new_rect.to_f32())
        } else {
            Self::empty()
        }
    }

    pub fn apply_transform(&self, transform: &PixelTransform) -> Self {
        Scissor {
            xform: self.xform.then(transform),
            extent: self.extent,
        }
    }
}
