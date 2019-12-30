// file based on https://github.com/sunli829/nvg
// released on MIT license
// which was translated from https://github.com/memononen/nanovg (zlib license)

use crate::backend::TexturedVertex;
use crate::primitive::LineCap;
use crate::primitive::LineJoin;
use crate::primitive::PathElement;
use crate::primitive::Solidity;
use crate::units::Point;
use clamped::Clamp;
use rawpointer::ptrdistance;
use std::f32::consts::PI;

bitflags! {
    struct PointFlags: u32 {
        const PT_CORNER = 0x1;
        const PT_LEFT = 0x2;
        const PT_BEVEL = 0x4;
        const PR_INNERBEVEL	= 0x8;
    }
}

#[derive(Copy, Clone)]
pub struct VPoint {
    xy: Point,
    d: Point,
    len: f32,
    dm: Point,
    flags: PointFlags,
}

pub struct Path {
    pub first: usize,
    pub count: usize,
    pub closed: bool,
    pub num_bevel: usize,
    pub solidity: Solidity,
    pub fill: *mut TexturedVertex,
    pub num_fill: usize,
    pub stroke: *mut TexturedVertex,
    pub num_stroke: usize,
    pub convex: bool,
}

impl Path {
    pub fn get_fill(&self) -> &[TexturedVertex] {
        if self.fill.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.fill, self.num_fill) }
        }
    }

    pub fn get_stroke(&self) -> &[TexturedVertex] {
        if self.stroke.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.stroke, self.num_stroke) }
        }
    }
}

pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

pub struct FlattenedPath {
    pub points: Vec<VPoint>,
    pub paths: Vec<Path>,
    pub vertexes: Vec<TexturedVertex>,
    pub bounds: Bounds,
}

trait PointExt {
    fn equals(self, pt: Point, tol: f32) -> bool;
    fn normalize(&mut self) -> f32;
}

impl PointExt for Point {
    fn equals(self, pt: Point, tol: f32) -> bool {
        let dx = pt.x - self.x;
        let dy = pt.y - self.y;
        dx * dx + dy * dy < tol * tol
    }

    fn normalize(&mut self) -> f32 {
        let d = ((self.x) * (self.x) + (self.y) * (self.y)).sqrt();
        if d > 1e-6 {
            let id = 1.0 / d;
            self.x *= id;
            self.y *= id;
        }
        d
    }
}

impl FlattenedPath {
    pub fn clear(&mut self) {
        self.points.clear();
        self.paths.clear();
    }

    pub fn new(path: &[PathElement], dist_tol: f32, tess_tol: f32) -> Self {
        let mut flattened_path = FlattenedPath {
            points: Vec::new(),
            paths: Vec::new(),
            vertexes: Vec::new(),
            bounds: Bounds {
                min: Point::new(std::f32::MAX, std::f32::MAX),
                max: Point::new(std::f32::MIN, std::f32::MIN),
            },
        };

        // Flatten
        for cmd in path {
            match cmd {
                PathElement::MoveTo(point) => {
                    flattened_path.add_path();
                    flattened_path.add_point(point.to_untyped(), PointFlags::PT_CORNER, dist_tol);
                }

                PathElement::LineTo(point) => {
                    flattened_path.add_point(point.to_untyped(), PointFlags::PT_CORNER, dist_tol);
                }

                PathElement::BezierTo(c1, c2, point) => {
                    if let Some(last) = flattened_path.points.last().map(|point| *point) {
                        flattened_path.tesselate_bezier(
                            last.xy,
                            c1.to_untyped(),
                            c2.to_untyped(),
                            point.to_untyped(),
                            0,
                            PointFlags::PT_CORNER,
                            tess_tol,
                        );
                    }
                }

                PathElement::ClosePath => flattened_path.close_path(),

                PathElement::Solidity(solidity) => flattened_path.path_solidity(*solidity),
            }
        }

        unsafe {
            // Calculate the direction and length of line segments.
            for j in 0..flattened_path.paths.len() {
                let path = &mut flattened_path.paths[j];
                let pts = &mut flattened_path.points[path.first] as *mut VPoint;

                // If the first and last points are the same, remove the last, mark as closed path.
                let mut p0 = pts.offset(path.count as isize - 1);
                let mut p1 = pts;
                if (*p0).xy.equals((*p1).xy, dist_tol) {
                    path.count -= 1;
                    p0 = pts.offset(path.count as isize - 1);
                    path.closed = true;
                }

                // Enforce winding.
                if path.count > 2 {
                    let area = poly_area(std::slice::from_raw_parts(pts, path.count));
                    match path.solidity {
                        Solidity::Solid => {
                            if area < 0.0 {
                                poly_reverse(std::slice::from_raw_parts_mut(pts, path.count));
                            }
                        }
                        Solidity::Hole => {
                            if area > 0.0 {
                                poly_reverse(std::slice::from_raw_parts_mut(pts, path.count));
                            }
                        }
                    }
                }

                for _ in 0..path.count {
                    // Calculate segment direction and length
                    (*p0).d.x = (*p1).xy.x - (*p0).xy.x;
                    (*p0).d.y = (*p1).xy.y - (*p0).xy.y;
                    (*p0).len = (*p0).d.normalize();

                    // Update bounds
                    flattened_path.bounds.min.x = flattened_path.bounds.min.x.min((*p0).xy.x);
                    flattened_path.bounds.min.y = flattened_path.bounds.min.y.min((*p0).xy.y);
                    flattened_path.bounds.max.x = flattened_path.bounds.max.x.max((*p0).xy.x);
                    flattened_path.bounds.max.y = flattened_path.bounds.max.y.max((*p0).xy.y);

                    // Advance
                    p0 = p1;
                    p1 = p1.add(1);
                }
            }
        }

        flattened_path
    }

    pub fn expand_stroke(
        &mut self,
        mut w: f32,
        fringe: f32,
        line_cap: LineCap,
        line_join: LineJoin,
        miter_limit: f32,
        tess_tol: f32,
    ) {
        let aa = fringe;
        let mut u0 = 0.0;
        let mut u1 = 1.0;
        let ncap = curve_divs(w, PI, tess_tol); // Calculate divisions per half circle

        w += aa * 0.5;

        // Disable the gradient used for antialiasing when antialiasing is not used.
        if aa == 0.0 {
            u0 = 0.5;
            u1 = 0.5;
        }

        self.calculate_joins(w, line_join, miter_limit);

        // Calculate max vertex usage.
        let mut cverts = 0;
        for path in &self.paths {
            let loop_ = path.closed;
            match line_join {
                LineJoin::Round => {
                    // plus one for loop
                    cverts += (path.count + path.num_bevel * (ncap + 2) + 1) * 2;
                }
                LineJoin::Bevel | LineJoin::Miter => {
                    // plus one for loop
                    cverts += (path.count + path.num_bevel * 5 + 1) * 2;
                    if !loop_ {
                        // space for caps
                        cverts += match line_cap {
                            LineCap::Round => (ncap * 2 + 2) * 2,
                            LineCap::Butt | LineCap::Square => (3 + 3) * 2,
                        };
                    }
                }
            }
        }

        unsafe {
            let mut vertexes = self.alloc_temp_vertexes(cverts);
            if vertexes.is_null() {
                return;
            }

            for i in 0..self.paths.len() {
                let path = &mut self.paths[i];
                let pts = &mut self.points[path.first] as *mut VPoint;

                path.fill = std::ptr::null_mut();
                path.num_fill = 0;

                // Calculate fringe or stroke
                let loop_ = path.closed;
                let mut dst = vertexes;
                path.stroke = dst;

                let (mut p0, mut p1, s, e) = if loop_ {
                    // Looping
                    (pts.offset(path.count as isize - 1), pts, 0, path.count)
                } else {
                    // Add cap
                    (pts, pts.add(1), 1, path.count - 1)
                };

                if !loop_ {
                    // Add cap
                    let mut d = Point::new((*p1).xy.x - (*p0).xy.x, (*p1).xy.y - (*p0).xy.y);
                    d.normalize();
                    match line_cap {
                        LineCap::Butt => {
                            dst = butt_cap_start(
                                dst,
                                p0.as_mut().unwrap(),
                                d.x,
                                d.y,
                                w,
                                -aa * 0.5,
                                aa,
                                u0,
                                u1,
                            )
                        }
                        LineCap::Square => {
                            dst = butt_cap_start(
                                dst,
                                p0.as_mut().unwrap(),
                                d.x,
                                d.y,
                                w,
                                w - aa,
                                aa,
                                u0,
                                u1,
                            )
                        }
                        LineCap::Round => {
                            dst = round_cap_start(
                                dst,
                                p0.as_mut().unwrap(),
                                d.x,
                                d.y,
                                w,
                                ncap,
                                aa,
                                u0,
                                u1,
                            )
                        }
                    }
                }

                for _ in s..e {
                    if (*p1).flags.contains(PointFlags::PT_BEVEL)
                        || (*p1).flags.contains(PointFlags::PR_INNERBEVEL)
                    {
                        match line_join {
                            LineJoin::Round => {
                                dst = round_join(
                                    dst,
                                    p0.as_mut().unwrap(),
                                    p1.as_mut().unwrap(),
                                    w,
                                    w,
                                    u0,
                                    u1,
                                    ncap,
                                    aa,
                                );
                            }
                            _ => {
                                dst = bevel_join(
                                    dst,
                                    p0.as_mut().unwrap(),
                                    p1.as_mut().unwrap(),
                                    w,
                                    w,
                                    u0,
                                    u1,
                                    aa,
                                );
                            }
                        }
                    } else {
                        *dst = TexturedVertex::new(
                            [(*p1).xy.x + ((*p1).dm.x * w), (*p1).xy.y + ((*p1).dm.y * w)],
                            [u0, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        );
                        dst = dst.add(1);

                        *dst = TexturedVertex::new(
                            [(*p1).xy.x - ((*p1).dm.x * w), (*p1).xy.y - ((*p1).dm.y * w)],
                            [u1, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        );
                        dst = dst.add(1);
                    }
                    p0 = p1;
                    p1 = p1.add(1);
                }

                if loop_ {
                    // Loop it
                    let v0 = vertexes;
                    let v1 = vertexes.add(1);

                    *dst = TexturedVertex::new(
                        [(*v0).pos[0], (*v0).pos[1]],
                        [u0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    );
                    dst = dst.add(1);

                    *dst = TexturedVertex::new(
                        [(*v1).pos[0], (*v1).pos[1]],
                        [u1, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    );
                    dst = dst.add(1);
                } else {
                    // Add cap
                    let mut d = Point::new((*p1).xy.x - (*p0).xy.x, (*p1).xy.y - (*p0).xy.y);
                    d.normalize();
                    match line_cap {
                        LineCap::Butt => {
                            dst = butt_cap_end(
                                dst,
                                p1.as_mut().unwrap(),
                                d.x,
                                d.y,
                                w,
                                -aa * 0.5,
                                aa,
                                u0,
                                u1,
                            );
                        }
                        LineCap::Round => {
                            dst = butt_cap_end(
                                dst,
                                p1.as_mut().unwrap(),
                                d.x,
                                d.y,
                                w,
                                w - aa,
                                aa,
                                u0,
                                u1,
                            );
                        }
                        LineCap::Square => {
                            dst = round_cap_end(
                                dst,
                                p1.as_mut().unwrap(),
                                d.x,
                                d.y,
                                w,
                                ncap,
                                aa,
                                u0,
                                u1,
                            );
                        }
                    }
                }

                path.num_stroke = ptrdistance(vertexes, dst);
                vertexes = dst;
            }
        }
    }

    pub fn expand_fill(
        &mut self,
        w: f32,
        line_join: LineJoin,
        miter_limit: f32,
        fringe_width: f32,
    ) {
        let aa = fringe_width;
        let fringe = w > 0.0;

        self.calculate_joins(w, line_join, miter_limit);

        // Calculate max vertex usage.
        let mut cverts = 0;
        for path in &self.paths {
            cverts += path.count + path.num_bevel + 1;
            if fringe {
                // plus one for loop
                cverts += (path.count + path.num_bevel * 5 + 1) * 2;
            }
        }

        unsafe {
            let mut vertexes = self.alloc_temp_vertexes(cverts);
            if vertexes.is_null() {
                return;
            }

            let convex = self.paths.len() == 1 && self.paths[0].convex;

            for i in 0..self.paths.len() {
                // Calculate shape vertices.
                let path = &mut self.paths[i];
                let pts = &mut self.points[path.first] as *mut VPoint;
                let woff = 0.5 * aa;
                let mut dst = vertexes;

                path.fill = dst;

                if fringe {
                    // Looping
                    let mut p0 = pts.offset(path.count as isize - 1);
                    let mut p1 = pts;
                    for _ in 0..path.count {
                        if (*p1).flags.contains(PointFlags::PT_BEVEL) {
                            let dlx0 = (*p0).d.y;
                            let dly0 = -(*p0).d.x;
                            let dlx1 = (*p1).d.y;
                            let dly1 = -(*p1).d.x;
                            if (*p1).flags.contains(PointFlags::PT_LEFT) {
                                let lx = (*p1).xy.x + (*p1).dm.x * woff;
                                let ly = (*p1).xy.y + (*p1).dm.y * woff;
                                *dst =
                                    TexturedVertex::new([lx, ly], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
                                dst = dst.add(1);
                            } else {
                                let lx0 = (*p1).xy.x + dlx0 * woff;
                                let ly0 = (*p1).xy.y + dly0 * woff;
                                let lx1 = (*p1).xy.x + dlx1 * woff;
                                let ly1 = (*p1).xy.y + dly1 * woff;

                                *dst = TexturedVertex::new(
                                    [lx0, ly0],
                                    [0.5, 1.0],
                                    [1.0, 1.0, 1.0, 1.0],
                                );
                                dst = dst.add(1);

                                *dst = TexturedVertex::new(
                                    [lx1, ly1],
                                    [0.5, 1.0],
                                    [1.0, 1.0, 1.0, 1.0],
                                );
                                dst = dst.add(1);
                            }
                        } else {
                            *dst = TexturedVertex::new(
                                [
                                    (*p1).xy.x + ((*p1).dm.x * woff),
                                    (*p1).xy.y + ((*p1).dm.y * woff),
                                ],
                                [0.5, 1.0],
                                [1.0, 1.0, 1.0, 1.0],
                            );
                            dst = dst.add(1);
                        }

                        p0 = p1;
                        p1 = p1.add(1);
                    }
                } else {
                    for j in 0..path.count {
                        let pt = pts.add(j);
                        *dst = TexturedVertex::new(
                            [(*pt).xy.x, (*pt).xy.y],
                            [0.5, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        );
                        dst = dst.add(1);
                    }
                }

                path.num_fill = ptrdistance(vertexes, dst);
                vertexes = dst;

                // Calculate fringe
                if fringe {
                    let mut lw = w + woff;
                    let rw = w - woff;
                    let mut lu = 0.0;
                    let ru = 1.0;
                    let mut dst = vertexes;
                    path.stroke = dst;

                    // Create only half a fringe for convex shapes so that
                    // the shape can be rendered without stenciling.
                    if convex {
                        lw = woff; // This should generate the same vertex as fill inset above.
                        lu = 0.5; // Set outline fade at middle.
                    }

                    // Looping
                    let mut p0 = pts.offset(path.count as isize - 1);
                    let mut p1 = pts;

                    for _ in 0..path.count {
                        if (*p1).flags.contains(PointFlags::PT_BEVEL)
                            || (*p1).flags.contains(PointFlags::PR_INNERBEVEL)
                        {
                            dst = bevel_join(
                                dst,
                                p0.as_mut().unwrap(),
                                p1.as_mut().unwrap(),
                                lw,
                                rw,
                                lu,
                                ru,
                                fringe_width,
                            );
                        } else {
                            *dst = TexturedVertex::new(
                                [
                                    (*p1).xy.x + ((*p1).dm.x * lw),
                                    (*p1).xy.y + ((*p1).dm.y * lw),
                                ],
                                [lu, 1.0],
                                [1.0, 1.0, 1.0, 1.0],
                            );
                            dst = dst.add(1);

                            *dst = TexturedVertex::new(
                                [
                                    (*p1).xy.x - ((*p1).dm.x * rw),
                                    (*p1).xy.y - ((*p1).dm.y * rw),
                                ],
                                [ru, 1.0],
                                [1.0, 1.0, 1.0, 1.0],
                            );
                            dst = dst.add(1);
                        }
                        p0 = p1;
                        p1 = p1.add(1);
                    }

                    // Loop it
                    let v0 = vertexes;
                    let v1 = vertexes.add(1);

                    *dst = TexturedVertex::new(
                        [(*v0).pos[0], (*v0).pos[1]],
                        [lu, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    );
                    dst = dst.add(1);

                    *dst = TexturedVertex::new(
                        [(*v1).pos[0], (*v1).pos[1]],
                        [ru, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    );
                    dst = dst.add(1);

                    path.num_stroke = ptrdistance(vertexes, dst);
                    vertexes = dst;
                } else {
                    path.stroke = std::ptr::null_mut();
                    path.num_stroke = 0;
                }
            }
        }
    }

    fn add_path(&mut self) -> &mut Path {
        self.paths.push(Path {
            first: self.points.len(),
            count: 0,
            closed: false,
            num_bevel: 0,
            solidity: Solidity::Solid,
            fill: std::ptr::null_mut(),
            num_fill: 0,
            stroke: std::ptr::null_mut(),
            num_stroke: 0,
            convex: false,
        });
        self.paths.last_mut().unwrap()
    }

    fn add_point(&mut self, pt: Point, flags: PointFlags, dist_tol: f32) {
        if let Some(path) = self.paths.last_mut() {
            if let Some(last_pt) = self.points.last_mut() {
                if path.count > 0 {
                    if last_pt.xy.equals(pt, dist_tol) {
                        last_pt.flags |= flags;
                        return;
                    }
                }
            }

            self.points.push(VPoint {
                xy: pt,
                d: Default::default(),
                len: 0.0,
                dm: Default::default(),
                flags,
            });
            path.count += 1;
        }
    }

    fn close_path(&mut self) {
        if let Some(path) = self.paths.last_mut() {
            path.closed = true;
        }
    }

    fn path_solidity(&mut self, solidity: Solidity) {
        if let Some(path) = self.paths.last_mut() {
            path.solidity = solidity;
        }
    }

    unsafe fn alloc_temp_vertexes(&mut self, count: usize) -> *mut TexturedVertex {
        self.vertexes.resize(
            count,
            TexturedVertex::new([0.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
        );
        if self.vertexes.is_empty() {
            return std::ptr::null_mut();
        }
        &mut self.vertexes[0] as *mut TexturedVertex
    }

    fn tesselate_bezier(
        &mut self,
        pt1: Point,
        pt2: Point,
        pt3: Point,
        pt4: Point,
        level: usize,
        flags: PointFlags,
        tess_tol: f32,
    ) {
        if level > 10 {
            return;
        }

        let Point { x: x1, y: y1, .. } = pt1;
        let Point { x: x2, y: y2, .. } = pt2;
        let Point { x: x3, y: y3, .. } = pt3;
        let Point { x: x4, y: y4, .. } = pt4;

        let x12 = (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23 = (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x34 = (x3 + x4) * 0.5;
        let y34 = (y3 + y4) * 0.5;
        let x123 = (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;

        let dx = x4 - x1;
        let dy = y4 - y1;
        let d2 = ((x2 - x4) * dy - (y2 - y4) * dx).abs();
        let d3 = ((x3 - x4) * dy - (y3 - y4) * dx).abs();

        if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
            self.add_point(Point::new(x4, y4), flags, tess_tol);
            return;
        }

        let x234 = (x23 + x34) * 0.5;
        let y234 = (y23 + y34) * 0.5;
        let x1234 = (x123 + x234) * 0.5;
        let y1234 = (y123 + y234) * 0.5;

        self.tesselate_bezier(
            Point::new(x1, y1),
            Point::new(x12, y12),
            Point::new(x123, y123),
            Point::new(x1234, y1234),
            level + 1,
            PointFlags::empty(),
            tess_tol,
        );
        self.tesselate_bezier(
            Point::new(x1234, y1234),
            Point::new(x234, y234),
            Point::new(x34, y34),
            Point::new(x4, y4),
            level + 1,
            flags,
            tess_tol,
        );
    }

    fn calculate_joins(&mut self, w: f32, line_join: LineJoin, miter_limit: f32) {
        let mut iw = 0.0;
        if w > 0.0 {
            iw = 1.0 / w;
        }

        unsafe {
            // Calculate which joins needs extra vertices to append, and gather vertex count.
            for i in 0..self.paths.len() {
                let path = &mut self.paths[i];
                let pts = &mut self.points[path.first] as *mut VPoint;
                let mut p0 = pts.offset(path.count as isize - 1);
                let mut p1 = pts;
                let mut nleft = 0;

                path.num_bevel = 0;

                for _ in 0..path.count {
                    let dlx0 = (*p0).d.y;
                    let dly0 = -(*p0).d.x;
                    let dlx1 = (*p1).d.y;
                    let dly1 = -(*p1).d.x;

                    // Calculate extrusions
                    (*p1).dm.x = (dlx0 + dlx1) * 0.5;
                    (*p1).dm.y = (dly0 + dly1) * 0.5;
                    let dmr2 = (*p1).dm.x * (*p1).dm.x + (*p1).dm.y * (*p1).dm.y;

                    if dmr2 > 0.000001 {
                        let mut scale = 1.0 / dmr2;
                        if scale > 600.0 {
                            scale = 600.0;
                        }
                        (*p1).dm.x *= scale;
                        (*p1).dm.y *= scale;
                    }

                    // Clear flags, but keep the corner.
                    (*p1).flags &= PointFlags::PT_CORNER;

                    // Keep track of left turns.
                    let cross = (*p1).d.x * (*p0).d.y - (*p0).d.x * (*p1).d.y;
                    if cross > 0.0 {
                        nleft += 1;
                        (*p1).flags |= PointFlags::PT_LEFT;
                    }

                    // Calculate if we should use bevel or miter for inner join.
                    let limit = (((*p0).len.min((*p1).len) as f32) * iw).max(1.01);
                    if (dmr2 * limit * limit) < 1.0 {
                        (*p1).flags |= PointFlags::PR_INNERBEVEL;
                    }

                    // Check to see if the corner needs to be beveled.
                    if (*p1).flags.contains(PointFlags::PT_CORNER) {
                        let line_join_bevel = match line_join {
                            LineJoin::Bevel => true,
                            LineJoin::Round => true,
                            LineJoin::Miter => false,
                        };
                        if (dmr2 * miter_limit * miter_limit) < 1.0 || line_join_bevel {
                            (*p1).flags |= PointFlags::PT_BEVEL;
                        }
                    }

                    if (*p1).flags.contains(PointFlags::PT_BEVEL)
                        || (*p1).flags.contains(PointFlags::PR_INNERBEVEL)
                    {
                        path.num_bevel += 1;
                    }

                    p0 = p1;
                    p1 = p1.add(1);
                }

                path.convex = nleft == path.count;
            }
        }
    }
}

fn triangle_area(a: &VPoint, b: &VPoint, c: &VPoint) -> f32 {
    let a = &a.xy;
    let b = &b.xy;
    let c = &c.xy;
    let abx = b.x - a.x;
    let aby = b.y - a.y;
    let acx = c.x - a.x;
    let acy = c.y - a.y;
    acx * aby - abx * acy
}

fn poly_area(pts: &[VPoint]) -> f32 {
    let mut area = 0.0;
    for i in 2..pts.len() {
        let a = &pts[0];
        let b = &pts[i - 1];
        let c = &pts[i];
        area += triangle_area(a, b, c);
    }
    area * 0.5
}

fn poly_reverse(pts: &mut [VPoint]) {
    let mut i = 0;
    let mut j = pts.len() as i32 - 1;
    while i < j {
        pts.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}

fn curve_divs(r: f32, arc: f32, tess_tol: f32) -> usize {
    let da = (r / (r + tess_tol)).acos() * 2.0;
    ((arc / da).ceil() as i32).max(2) as usize
}

fn choose_bevel(bevel: bool, p0: &mut VPoint, p1: &mut VPoint, w: f32) -> (f32, f32, f32, f32) {
    if bevel {
        let x0 = p1.xy.x + p0.d.y * w;
        let y0 = p1.xy.y - p0.d.x * w;
        let x1 = p1.xy.x + p1.d.y * w;
        let y1 = p1.xy.y - p1.d.x * w;
        (x0, y0, x1, y1)
    } else {
        let x0 = p1.xy.x + p1.dm.x * w;
        let y0 = p1.xy.y + p1.dm.y * w;
        let x1 = p1.xy.x + p1.dm.x * w;
        let y1 = p1.xy.y + p1.dm.y * w;
        (x0, y0, x1, y1)
    }
}

unsafe fn round_join(
    mut dst: *mut TexturedVertex,
    p0: &mut VPoint,
    p1: &mut VPoint,
    lw: f32,
    rw: f32,
    lu: f32,
    ru: f32,
    ncap: usize,
    _fringe: f32,
) -> *mut TexturedVertex {
    let dlx0 = p0.d.y;
    let dly0 = -p0.d.x;
    let dlx1 = p1.d.y;
    let dly1 = -p1.d.x;

    if p1.flags.contains(PointFlags::PT_LEFT) {
        let (lx0, ly0, lx1, ly1) =
            choose_bevel(p1.flags.contains(PointFlags::PR_INNERBEVEL), p0, p1, lw);
        let a0 = -dly0.atan2(-dlx0);
        let mut a1 = -dly1.atan2(-dlx1);
        if a1 > a0 {
            a1 -= PI * 2.0;
        }

        *dst = TexturedVertex::new([lx0, ly0], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);

        *dst = TexturedVertex::new(
            [p1.xy.x - dlx0 * rw, p1.xy.y - dly0 * rw],
            [ru, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        let n = ((((a0 - a1) / PI) * (ncap as f32)).ceil() as i32).clamped(2, ncap as i32);
        for i in 0..n {
            let u = (i as f32) / ((n - 1) as f32);
            let a = a0 + u * (a1 - a0);
            let rx = p1.xy.x + a.cos() * rw;
            let ry = p1.xy.y + a.sin() * rw;

            *dst = TexturedVertex::new([p1.xy.x, p1.xy.y], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new([rx, ry], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);
        }

        *dst = TexturedVertex::new([lx1, ly1], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);

        *dst = TexturedVertex::new(
            [p1.xy.x - dlx1 * rw, p1.xy.y - dly1 * rw],
            [ru, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);
    } else {
        let (rx0, ry0, rx1, ry1) =
            choose_bevel(p1.flags.contains(PointFlags::PR_INNERBEVEL), p0, p1, -rw);
        let a0 = dly0.atan2(dlx0);
        let mut a1 = dly1.atan2(dlx1);
        if a1 < a0 {
            a1 += PI * 2.0;
        }

        *dst = TexturedVertex::new(
            [p1.xy.x + dlx0 * rw, p1.xy.y + dly0 * rw],
            [lu, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        *dst = TexturedVertex::new([rx0, ry0], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);

        let n = ((((a0 - a1) / PI) * (ncap as f32)).ceil() as i32).clamped(2, ncap as i32);
        for i in 0..n {
            let u = (i as f32) / ((n - 1) as f32);
            let a = a0 + u * (a1 - a0);
            let lx = p1.xy.x + a.cos() * lw;
            let ly = p1.xy.y + a.cos() * lw;

            *dst = TexturedVertex::new([lx, ly], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new([p1.xy.x, p1.xy.y], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);
        }

        *dst = TexturedVertex::new(
            [p1.xy.x + dlx1 * rw, p1.xy.y + dly1 * rw],
            [lu, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        *dst = TexturedVertex::new([rx1, ry1], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);
    }

    dst
}

unsafe fn bevel_join(
    mut dst: *mut TexturedVertex,
    p0: &mut VPoint,
    p1: &mut VPoint,
    lw: f32,
    rw: f32,
    lu: f32,
    ru: f32,
    _fringe: f32,
) -> *mut TexturedVertex {
    let dlx0 = p0.d.y;
    let dly0 = -p0.d.x;
    let dlx1 = p1.d.y;
    let dly1 = -p1.d.x;

    if p1.flags.contains(PointFlags::PT_LEFT) {
        let (lx0, ly0, lx1, ly1) =
            choose_bevel(p1.flags.contains(PointFlags::PR_INNERBEVEL), p0, p1, lw);

        *dst = TexturedVertex::new([lx0, ly0], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);

        *dst = TexturedVertex::new(
            [p1.xy.x - dlx0 * rw, p1.xy.y - dly0 * rw],
            [ru, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        if p1.flags.contains(PointFlags::PT_BEVEL) {
            *dst = TexturedVertex::new([lx0, ly0], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new(
                [p1.xy.x - dlx0 * rw, p1.xy.y - dly0 * rw],
                [ru, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);

            *dst = TexturedVertex::new([lx1, ly1], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new(
                [p1.xy.x - dlx1 * rw, p1.xy.y - dly1 * rw],
                [ru, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);
        } else {
            let rx0 = p1.xy.x - p1.dm.x * rw;
            let ry0 = p1.xy.y - p1.dm.y * rw;

            *dst = TexturedVertex::new([p1.xy.x, p1.xy.y], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new(
                [p1.xy.x - dlx0 * rw, p1.xy.y - dly0 * rw],
                [ru, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);

            *dst = TexturedVertex::new([rx0, ry0], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new([rx0, ry0], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new([p1.xy.x, p1.xy.y], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new(
                [p1.xy.x - dlx1 * rw, p1.xy.y - dly1 * rw],
                [ru, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);
        }

        *dst = TexturedVertex::new([lx1, ly1], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);

        *dst = TexturedVertex::new(
            [p1.xy.x - dlx1 * rw, p1.xy.y - dly1 * rw],
            [ru, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);
    } else {
        let (rx0, ry0, rx1, ry1) =
            choose_bevel(p1.flags.contains(PointFlags::PR_INNERBEVEL), p0, p1, -rw);

        *dst = TexturedVertex::new(
            [p1.xy.x + dlx0 * lw, p1.xy.y + dly0 * lw],
            [lu, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        if p1.flags.contains(PointFlags::PT_BEVEL) {
            *dst = TexturedVertex::new(
                [p1.xy.x + dlx0 * lw, p1.xy.y + dly0 * lw],
                [lu, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);

            *dst = TexturedVertex::new([rx0, ry0], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new(
                [p1.xy.x + dlx1 * lw, p1.xy.y + dly1 * lw],
                [lu, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);

            *dst = TexturedVertex::new([rx1, ry1], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);
        } else {
            let lx0 = p1.xy.x + p1.dm.x * lw;
            let ly0 = p1.xy.y + p1.dm.y * lw;

            *dst = TexturedVertex::new(
                [p1.xy.x + dlx0 * lw, p1.xy.y + dly0 * lw],
                [lu, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);

            *dst = TexturedVertex::new([p1.xy.x, p1.xy.y], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new([lx0, ly0], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new([lx0, ly0], [lu, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);

            *dst = TexturedVertex::new(
                [p1.xy.x + dlx1 * lw, p1.xy.y + dly1 * lw],
                [lu, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            );
            dst = dst.add(1);

            *dst = TexturedVertex::new([p1.xy.x, p1.xy.y], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
            dst = dst.add(1);
        }

        *dst = TexturedVertex::new(
            [p1.xy.x + dlx1 * lw, p1.xy.y + dly1 * lw],
            [lu, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        *dst = TexturedVertex::new([rx1, ry1], [ru, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);
    }

    dst
}

unsafe fn butt_cap_start(
    mut dst: *mut TexturedVertex,
    p: &mut VPoint,
    dx: f32,
    dy: f32,
    w: f32,
    d: f32,
    aa: f32,
    u0: f32,
    u1: f32,
) -> *mut TexturedVertex {
    let px = p.xy.x - dx * d;
    let py = p.xy.y - dy * d;
    let dlx = dy;
    let dly = -dx;

    *dst = TexturedVertex::new(
        [px + dlx * w - dx * aa, py + dly * w - dy * aa],
        [u0, 0.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px - dlx * w - dx * aa, py - dly * w - dy * aa],
        [u1, 0.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px + dlx * w, py + dly * w],
        [u0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px - dlx * w, py - dly * w],
        [u1, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    dst
}

unsafe fn butt_cap_end(
    mut dst: *mut TexturedVertex,
    p: &mut VPoint,
    dx: f32,
    dy: f32,
    w: f32,
    d: f32,
    aa: f32,
    u0: f32,
    u1: f32,
) -> *mut TexturedVertex {
    let px = p.xy.x - dx * d;
    let py = p.xy.y - dy * d;
    let dlx = dy;
    let dly = -dx;

    *dst = TexturedVertex::new(
        [px + dlx * w, py + dly * w],
        [u0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px - dlx * w, py - dly * w],
        [u1, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px + dlx * w + dx * aa, py + dly * w + dy * aa],
        [u0, 0.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px - dlx * w + dx * aa, py - dly * w + dy * aa],
        [u1, 0.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    dst
}

unsafe fn round_cap_start(
    mut dst: *mut TexturedVertex,
    p: &mut VPoint,
    dx: f32,
    dy: f32,
    w: f32,
    ncap: usize,
    _aa: f32,
    u0: f32,
    u1: f32,
) -> *mut TexturedVertex {
    let px = p.xy.x;
    let py = p.xy.y;
    let dlx = dy;
    let dly = -dx;

    for i in 0..ncap {
        let a = (i as f32) / ((ncap - 1) as f32) * PI;
        let ax = a.cos() * w;
        let ay = a.sin() * w;

        *dst = TexturedVertex::new(
            [px - dlx * ax - dx * ay, py - dly * ax - dy * ay],
            [u0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);

        *dst = TexturedVertex::new([px, py], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);
    }

    *dst = TexturedVertex::new(
        [px + dlx * w, py + dly * w],
        [u0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px - dlx * w, py - dly * w],
        [u1, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    dst
}

unsafe fn round_cap_end(
    mut dst: *mut TexturedVertex,
    p: &mut VPoint,
    dx: f32,
    dy: f32,
    w: f32,
    ncap: usize,
    _aa: f32,
    u0: f32,
    u1: f32,
) -> *mut TexturedVertex {
    let px = p.xy.x;
    let py = p.xy.y;
    let dlx = dy;
    let dly = -dx;

    *dst = TexturedVertex::new(
        [px + dlx * w, py + dly * w],
        [u0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    *dst = TexturedVertex::new(
        [px - dlx * w, py - dly * w],
        [u1, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    dst = dst.add(1);

    for i in 0..ncap {
        let a = (i as f32) / ((ncap - 1) as f32) * PI;
        let ax = a.cos() * w;
        let ay = a.sin() * w;

        *dst = TexturedVertex::new([px, py], [0.5, 1.0], [1.0, 1.0, 1.0, 1.0]);
        dst = dst.add(1);

        *dst = TexturedVertex::new(
            [px - dlx * ax + dx * ay, py - dly * ax + dy * ay],
            [u0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        dst = dst.add(1);
    }

    dst
}
