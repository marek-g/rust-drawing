use crate::clipping::utils::*;
use crate::primitive::*;
use crate::units::*;

pub trait Clipping {
    fn clip(self, rect: PixelRect) -> Self;
}

impl Clipping for Vec<Primitive> {
    fn clip(self, clipping_rect: PixelRect) -> Self {
        let mut res = Vec::new();
        let mut need_scissors = false;

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
                            start_point: PixelPoint::new(clipped.0, clipped.1),
                            end_point: PixelPoint::new(clipped.2, clipped.3),
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
                            rect: PixelRect::new(
                                PixelPoint::new(clipped.0, clipped.1),
                                PixelSize::new(clipped.2, clipped.3),
                            ),
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
                            rect: PixelRect::new(
                                PixelPoint::new(clipped.0, clipped.1),
                                PixelSize::new(clipped.2, clipped.3),
                            ),
                            uv: clipped.4,
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
                            clipping_rect: PixelRect::new(
                                PixelPoint::new(clipped.0, clipped.1),
                                PixelSize::new(clipped.2, clipped.3),
                            ),
                            text,
                        });
                    }
                }

                Primitive::Stroke {
                    path,
                    thickness,
                    brush,
                } => {
                    need_scissors = true;
                    res.push(Primitive::Stroke {
                        path,
                        thickness,
                        brush,
                    });

                    /*let clipped_path = path.clip(clipping_rect);
                    if clipped_path.len() > 0 {
                        res.push(Primitive::Stroke {
                            path: clipped_path,
                            thickness,
                            brush,
                        })
                    }*/
                }

                Primitive::StrokeStyled {
                    path,
                    thickness,
                    brush,
                    style,
                } => {
                    need_scissors = true;
                    res.push(Primitive::StrokeStyled {
                        path,
                        thickness,
                        brush,
                        style,
                    })

                    /*let clipped_path = path.clip(clipping_rect);
                    if clipped_path.len() > 0 {
                        res.push(Primitive::StrokeStyled {
                            path: clipped_path,
                            thickness,
                            brush,
                            style,
                        })
                    }*/
                }

                Primitive::Fill { path, brush } => {
                    need_scissors = true;
                    res.push(Primitive::Fill { path, brush });

                    /*let clipped_path = path.clip(clipping_rect);
                    if clipped_path.len() > 0 {
                        res.push(Primitive::Fill {
                            path: clipped_path,
                            brush,
                        })
                    }*/
                }

                Primitive::ClipRect { rect, primitives } => {
                    res.push(Primitive::ClipRect { rect, primitives })
                }

                Primitive::ClipPath { path, primitives } => {
                    res.push(Primitive::ClipPath { path, primitives })
                }

                Primitive::Transform {
                    transform,
                    primitives,
                } => res.push(Primitive::Transform {
                    transform,
                    primitives,
                }),

                Primitive::Composite { color, primitives } => {
                    let clipped_primitives = primitives.clip(clipping_rect);
                    if clipped_primitives.len() > 0 {
                        res.push(Primitive::Composite {
                            color,
                            primitives: clipped_primitives,
                        })
                    }
                }
            }
        }

        if need_scissors {
            vec![Primitive::ClipRect {
                rect: clipping_rect,
                primitives: res,
            }]
        } else {
            res
        }
    }
}

impl Clipping for Vec<PathElement> {
    fn clip(self, _clipping_rect: PixelRect) -> Self {
        let mut res = Vec::new();
        for path_element in self.into_iter() {
            // TODO: implement!
            res.push(path_element);
        }
        res
    }
}
