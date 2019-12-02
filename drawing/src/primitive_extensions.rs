use crate::primitive::*;
use crate::units::*;
use crate::utils::clipping::clip_image;
use crate::utils::clipping::clip_line;
use crate::utils::clipping::clip_rect;

pub trait PrimitiveTransformations {
    fn translate(&mut self, offset: UserPixelPoint);
    fn clip(self, rect: UserPixelRect) -> Self;
}

impl PrimitiveTransformations for Vec<Primitive> {
    fn translate(&mut self, offset: UserPixelPoint) {
        for primitive in self.iter_mut() {
            match primitive {
                Primitive::Line {
                    ref mut start_point,
                    ref mut end_point,
                    ..
                } => {
                    start_point.x += offset.x;
                    start_point.y += offset.y;
                    end_point.x += offset.x;
                    end_point.y += offset.y;
                }

                Primitive::Rectangle { ref mut rect, .. } => {
                    rect.origin.x += offset.x;
                    rect.origin.y += offset.y;
                }

                Primitive::Text {
                    ref mut position,
                    ref mut clipping_rect,
                    ..
                } => {
                    position.x += offset.x;
                    position.y += offset.y;
                    clipping_rect.origin.x += offset.x;
                    clipping_rect.origin.y += offset.y;
                }

                Primitive::Image { ref mut rect, .. } => {
                    rect.origin.x += offset.x;
                    rect.origin.y += offset.y;
                }

                Primitive::PushLayer { .. } => (),

                Primitive::PopLayer => (),
            }
        }
    }

    fn clip(self, clipping_rect: UserPixelRect) -> Self {
        let mut res = Vec::new();
        for primitive in self.into_iter() {
            match primitive {
                Primitive::Line {
                    color,
                    thickness,
                    start_point,
                    end_point,
                } => {
                    if let Some(clipped) = clip_line(
                        start_point.x,
                        start_point.y,
                        end_point.x,
                        end_point.y,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                    ) {
                        res.push(Primitive::Line {
                            color,
                            thickness,
                            start_point: UserPixelPoint::new(clipped.0, clipped.1),
                            end_point: UserPixelPoint::new(clipped.2, clipped.3),
                        });
                    }
                }

                Primitive::Rectangle { color, rect } => {
                    if let Some(clipped) = clip_rect(
                        rect.origin.x,
                        rect.origin.y,
                        rect.size.width,
                        rect.size.height,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                    ) {
                        res.push(Primitive::Rectangle {
                            color,
                            rect: UserPixelRect::new(
                                UserPixelPoint::new(clipped.0, clipped.1),
                                UserPixelSize::new(clipped.2, clipped.3),
                            ),
                        });
                    }
                }

                Primitive::Text {
                    resource_key,
                    size,
                    color,
                    position,
                    clipping_rect: rect,
                    text,
                } => {
                    if let Some(clipped) = clip_rect(
                        rect.origin.x,
                        rect.origin.y,
                        rect.size.width,
                        rect.size.height,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                    ) {
                        res.push(Primitive::Text {
                            resource_key,
                            size,
                            color,
                            position,
                            clipping_rect: UserPixelRect::new(
                                UserPixelPoint::new(clipped.0, clipped.1),
                                UserPixelSize::new(clipped.2, clipped.3),
                            ),
                            text,
                        });
                    }
                }

                Primitive::Image {
                    resource_key,
                    rect,
                    uv,
                } => {
                    if let Some(clipped) = clip_image(
                        rect.origin.x,
                        rect.origin.y,
                        rect.size.width,
                        rect.size.height,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                        &uv,
                    ) {
                        res.push(Primitive::Image {
                            resource_key,
                            rect: UserPixelRect::new(
                                UserPixelPoint::new(clipped.0, clipped.1),
                                UserPixelSize::new(clipped.2, clipped.3),
                            ),
                            uv: clipped.4,
                        });
                    }
                }

                Primitive::PushLayer { .. } => {
                    res.push(primitive);
                }

                Primitive::PopLayer => {
                    res.push(primitive);
                }
            }
        }
        res
    }
}
