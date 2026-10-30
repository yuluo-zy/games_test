//! B Rectangle2d：axis@0、center@8、size@16，共24B观察字段。
use super::{Aabb2, Circle2d, Ray2};
use glam::{Affine2, Mat2, Mat3, Vec2, Vec3};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle2d {
    pub axis: Vec2,
    pub center: Vec2,
    pub size: Vec2,
}
/// Port of the original callback payload; native closure/ABI identity is not an API.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectangleSlicePoint {
    pub position: Vec2,
    pub normal: Vec2,
    pub relative_u: f32,
    /// Original callback discriminants 4 and 5; their nominal enum name is unknown.
    pub tag: u8,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectangleSliceState {
    pub pending_normal: Option<Vec2>,
    pub join_pending: bool,
}
impl RectangleSliceState {
    fn emit(
        &mut self,
        mut point: RectangleSlicePoint,
        receiver: &mut impl FnMut(RectangleSlicePoint),
    ) {
        if let Some(previous) = self.pending_normal.take() {
            point.tag = 5;
            if self.join_pending {
                receiver(RectangleSlicePoint {
                    normal: previous + point.normal,
                    tag: 4,
                    ..point
                });
                point.tag = 4;
            }
            receiver(point);
        } else {
            point.tag = 5;
            receiver(point);
            self.pending_normal = Some(point.normal);
            self.join_pending = true;
        }
    }
}
impl Rectangle2d {
    /// 原 B 0x140c9c730：方向沿有符号最短角插值，中心和尺寸作线性插值。
    /// glam 0.29 的 angle_to 保留原 acos 多项式与叉积符号规则，不能换成轴向量归一化插值。
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        let complement = 1.0 - factor;
        let angle = self.axis.angle_to(other.axis) * factor + complement * 0.0;
        let (sin, cos) = angle.sin_cos();
        let axis = Vec2::new(
            -sin * self.axis.y + cos * self.axis.x,
            cos * self.axis.y + sin * self.axis.x,
        );
        Self::new(
            self.center * complement + other.center * factor,
            other.size * factor + self.size * complement,
            axis,
        )
    }
    pub fn orientation_mat2(self) -> Mat2 {
        Mat2::from_cols(self.axis, Vec2::new(-self.axis.y, self.axis.x))
    }
    /// Embeds the local 2D orientation in the XZ plane.
    pub fn orientation_mat3(self) -> Mat3 {
        Mat3::from_cols(
            Vec3::new(self.axis.x, 0.0, self.axis.y),
            Vec3::Y,
            Vec3::new(-self.axis.y, 0.0, self.axis.x),
        )
    }
    /// B inverse-matrix projection is intentionally not replaced by a transpose:
    /// public mutable fields can hold a nearly unit axis.
    fn perimeter_local_point(self, point: Vec2) -> Vec2 {
        self.orientation_mat2().inverse() * (point - self.center)
    }
    /// B 0x140c995c0 returns orientation + translation separately from size.
    pub fn as_transformation_and_size(self) -> (Affine2, Vec2) {
        (
            Affine2::from_mat2_translation(self.orientation_mat2(), self.center),
            self.size,
        )
    }
    pub fn new_centered_axis_aligned(size: Vec2) -> Self {
        Self::new_axis_aligned(Vec2::ZERO, size)
    }
    pub fn new_axis_aligned(center: Vec2, size: Vec2) -> Self {
        Self::new(center, size, Vec2::X)
    }
    pub fn new(center: Vec2, size: Vec2, axis: Vec2) -> Self {
        assert!(size.x.is_finite(), "原宽度finite guard");
        assert!(size.y.is_finite(), "原高度finite guard");
        assert!(
            (axis.y * axis.y + axis.x * axis.x - 1.0).abs() <= 0.0002,
            "原axis单位长度guard"
        );
        Self { axis, center, size }
    }
    pub fn local_to_world(self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x * self.axis.x + p.y * (-self.axis.y) + self.center.x,
            p.x * self.axis.y + p.y * self.axis.x + self.center.y,
        )
    }
    pub fn world_to_local(self, p: Vec2) -> Vec2 {
        let p = p - self.center;
        Vec2::new(
            p.y * self.axis.y + p.x * self.axis.x,
            p.y * self.axis.x + p.x * (-self.axis.y),
        )
    }
    pub fn as_points2_centered_aligned(self) -> [Vec2; 4] {
        assert!(self.size.x * self.size.y > 0.0, "原as_points正面积guard");
        let h = self.size * 0.5;
        [
            Vec2::new(-h.x, -h.y),
            Vec2::new(-h.x, h.y),
            Vec2::new(h.x, h.y),
            Vec2::new(h.x, -h.y),
        ]
    }
    pub fn as_points2(self) -> [Vec2; 4] {
        self.as_points2_centered_aligned()
            .map(|p| self.local_to_world(p))
    }
    pub fn area(self) -> f32 {
        self.size.x * self.size.y
    }
    pub fn perimeter(self) -> f32 {
        let sum = self.size.x + self.size.y;
        sum + sum
    }
    pub fn corner_us(self) -> [f32; 5] {
        let p = self.perimeter();
        [
            0.0,
            self.size.y / p,
            (self.size.x + self.size.y) / p,
            (self.size.y + self.size.y + self.size.x) / p,
            1.0,
        ]
    }
    pub fn get_pos_at_u(self, u: f32) -> Vec2 {
        let us = self.corner_us();
        assert!(us.iter().all(|x| x.is_finite() && *x >= 0.0 && *x <= 1.0));
        let p = self.as_points2();
        if u == 0.0 || u == 1.0 {
            return p[0];
        }
        for i in 1..4 {
            if u == us[i] {
                return p[i];
            }
        }
        let i = if u < us[1] {
            0
        } else if u < us[2] {
            1
        } else if u < us[3] {
            2
        } else {
            3
        };
        let t = (u - us[i]) / (us[i + 1] - us[i]);
        p[(i + 1) % 4] * t + p[i] * (1.0 - t)
    }
    pub fn get_edge_normal(self, index: usize) -> Vec2 {
        assert!(index < 4, "原边下标guard");
        let local = [-Vec2::X, Vec2::Y, Vec2::X, -Vec2::Y][index];
        Vec2::new(
            (-self.axis.y) * local.y + self.axis.x * local.x,
            self.axis.x * local.y + self.axis.y * local.x,
        )
    }
    /// B 140c9a210：返回该角点分别指向前一角、后一角的单位方向。
    pub fn get_edge_dirs_from_corner_id(self, index: usize) -> [Vec2; 2] {
        // 原版先检查面积，再检查角序；复用同一套世界坐标角点，保留大坐标消减。
        let points = self.as_points2();
        let (previous, next) = match index {
            0 => (3, 1),
            1 => (0, 2),
            2 => (1, 3),
            3 => (2, 0),
            _ => unreachable!("原角序超出0..4"),
        };
        let normalize = |delta: Vec2| {
            // 原指令顺序为 y²+x²；零长度按倒数平方根路径自然产生 NaN。
            let reciprocal = 1.0 / (delta.y * delta.y + delta.x * delta.x).sqrt();
            delta * reciprocal
        };
        [
            normalize(points[previous] - points[index]),
            normalize(points[next] - points[index]),
        ]
    }
    pub fn get_point(self, index: usize) -> Vec2 {
        assert!(index < 5);
        self.as_points2()[if index == 4 { 0 } else { index }]
    }
    pub fn get_point_u(self, index: usize) -> f32 {
        assert!(index < 5);
        self.corner_us()[index]
    }
    pub fn get_corner_normal(self, index: usize) -> Vec2 {
        assert!(index < 4);
        let local = [
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, -1.0),
        ][index];
        let p = Vec2::new(
            (-self.axis.y) * local.y + self.axis.x * local.x,
            self.axis.x * local.y + self.axis.y * local.x,
        );
        p.normalize()
    }
    fn normal_at_u(self, u: f32, corner_length: f32) -> Vec2 {
        let corners = self.corner_us();
        assert!(
            corners
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        );
        if u == 0.0 || u == 1.0 {
            return self.get_corner_normal(0) * corner_length;
        }
        for (i, &corner) in corners.iter().enumerate().take(4).skip(1) {
            if u == corner {
                return self.get_corner_normal(i) * corner_length;
            }
        }
        self.get_edge_normal(self.get_edge_id_at_u(u))
    }
    pub fn get_normal_at_u(self, u: f32) -> Vec2 {
        self.normal_at_u(u, 1.0)
    }
    /// Original 0x140c99e20 selects sqrt(2) corner length, not a different edge rule.
    pub fn get_normal_at_u_non_normalized_corners(self, u: f32) -> Vec2 {
        self.normal_at_u(u, std::f32::consts::SQRT_2)
    }
    pub fn get_edge_id_at_u(self, u: f32) -> usize {
        let corners = self.corner_us();
        assert!(
            corners
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        );
        // An exact corner belongs to its incoming edge; u=1 belongs to edge 3.
        if 0.0 <= u && u <= corners[1] {
            0
        } else if corners[1] <= u && u <= corners[2] {
            1
        } else if corners[2] <= u && u <= corners[3] {
            2
        } else {
            3
        }
    }
    /// B chooses the dominant size-normalized local component. Ties choose Y,
    /// including the rectangle center, whose selected normal points down.
    pub fn closest_edge_normal(self, point: Vec2) -> Vec2 {
        let local = self.world_to_local(point);
        let relative = local.abs() / self.size;
        if relative.x > relative.y {
            if local.x > 0.0 { self.axis } else { -self.axis }
        } else if local.y > 0.0 {
            Vec2::new(-self.axis.y, self.axis.x)
        } else {
            Vec2::new(self.axis.y, -self.axis.x)
        }
    }
    pub fn get_closest_edge_id(self, point: Vec2) -> usize {
        // B140c9acb0/5B tail-jumps to140c9a9d0. The scalar return is the
        // existing common helper's first index, including its1000 threshold.
        self.closest_perimeter_edge_pair(point).0
    }
    // B140c9a9d0: common pair search, also used by the scalar edge-ID wrapper.
    fn closest_perimeter_edge_pair(self, point: Vec2) -> (usize, usize) {
        let points = self.as_points2();
        let mut pair = (0, 1);
        let mut best_distance = 1000.0;
        for left in 0..4 {
            let right = left + 1;
            let a = points[left];
            let b = points[right % 4];
            let t = Self::perimeter_projection(point, a, b);
            let closest = b * t + a * (1.0 - t);
            let delta = point - closest;
            let distance = (delta.y * delta.y + delta.x * delta.x).sqrt();
            if best_distance > distance {
                pair = (left, right);
            }
            best_distance = Self::perimeter_sse_min(distance, best_distance);
        }
        pair
    }

    // MAXSS/MINSS select their second operand for unordered comparisons and ties.
    fn perimeter_sse_max(first: f32, second: f32) -> f32 {
        if first > second { first } else { second }
    }
    fn perimeter_sse_min(first: f32, second: f32) -> f32 {
        if first < second { first } else { second }
    }
    fn perimeter_projection(point: Vec2, a: Vec2, b: Vec2) -> f32 {
        let edge = b - a;
        let offset = point - a;
        let denominator = Self::perimeter_sse_max(
            edge.y * edge.y + edge.x * edge.x,
            f32::from_bits(0x1e3ce508),
        );
        let value = (offset.y * edge.y + offset.x * edge.x) / denominator;
        Self::perimeter_sse_min(1.0, Self::perimeter_sse_max(0.0, value))
    }

    /// B 140c9ae20: project onto the original selected perimeter segment.
    /// Nonfinite point coordinates propagate; the original checks positive area.
    pub fn closest_point_on_perimeter(self, point: Vec2) -> Vec2 {
        let (left, right) = self.closest_perimeter_edge_pair(point);
        let points = self.as_points2();
        let a = points[left % 4];
        let b = points[right % 4];
        let t = Self::perimeter_projection(point, a, b);
        b * t + a * (1.0 - t)
    }

    /// B 140c9b000: keep the closing endpoint's U=1 while its point wraps to0.
    pub fn get_closest_u_from_pos(self, point: Vec2) -> f32 {
        let (left, right) = self.closest_perimeter_edge_pair(point);
        let points = self.as_points2();
        let left_u = self.get_point_u(left);
        let right_u = self.get_point_u(right);
        let valid_u = |u: f32| u.is_finite() && u >= 0.0 && u <= 1.0;
        assert!(valid_u(left_u), "original perimeter U finite range guard");
        assert!(valid_u(right_u), "original perimeter U finite range guard");
        let t = Self::perimeter_projection(point, points[left % 4], points[right % 4]);
        let u = left_u * (1.0 - t) + right_u * t;
        assert!(valid_u(u), "original perimeter U finite range guard");
        u
    }

    /// Signed distance to the rectangle perimeter, negative in the interior.

    pub fn signed_distance_to_closest_edge(self, point: Vec2) -> f32 {
        let q = self.perimeter_local_point(point).abs() - self.size * 0.5;
        if q.x < 0.0 || q.y < 0.0 {
            q.x.max(q.y)
        } else {
            q.x.hypot(q.y)
        }
    }
    /// B 140c9a750：此入口在逆投影后直接减半尺寸，原指令没有取绝对值。
    /// 圆柱选邻墙依赖这个非对称门，不能复用上面的有符号矩形距离。
    pub fn distance_to_closest_edge(self, point: Vec2) -> f32 {
        let q = self.perimeter_local_point(point) - self.size * 0.5;
        let distance = if q.x < 0.0 || q.y < 0.0 {
            // 保留原maxnum及最后MAXSS的无序、相等和有符号零选择。
            if q.x.is_nan() { q.y }
            else if q.y.is_nan() || q.x >= q.y { q.x }
            else { q.y }
        } else {
            q.x.hypot(q.y)
        };
        if distance > 0.0 { distance } else { 0.0 }
    }
    pub fn get_edge_pts(self, index: usize) -> (Vec2, Vec2) {
        assert!(index < 4);
        let points = self.as_points2();
        (points[index], points[(index + 1) % 4])
    }
    pub fn edge_length(self, index: usize) -> f32 {
        assert!(index < 4);
        if index.is_multiple_of(2) {
            self.size.y
        } else {
            self.size.x
        }
    }
    pub fn try_as_curve2(self) -> Result<crate::curve::Curve2, crate::curve::CurveError> {
        let p = self.as_points2();
        let mut points = p.to_vec();
        points.push(p[0]);
        crate::curve::Curve2::try_new(points, false)
    }
    pub fn as_curve2(self) -> crate::curve::Curve2 {
        self.try_as_curve2().unwrap()
    }
    pub fn as_points3(self, height: f32) -> [Vec3; 4] {
        self.as_points2().map(|p| Vec3::new(p.x, height, p.y))
    }
    pub fn as_curve3(self, height: f32) -> crate::curve::Curve3XZ {
        let points = self.as_points3(height);
        let mut closed = points.to_vec();
        closed.push(points[0]);
        crate::curve::Curve3XZ::try_new(closed, false).unwrap()
    }
    /// B 0x140c9bd90 produces perimeter parameters, not a Curve object.
    /// Only strictly interior corners are included; endpoints remain even if equal.
    pub fn circular_slice(self, start: f32, end: f32) -> Vec<f32> {
        let _ = crate::curve::CurveU::<crate::curve::CurveCoordSemantic2D>::new(start);
        let _ = crate::curve::CurveU::<crate::curve::CurveCoordSemantic2D>::new(end);
        let corners = self.corner_us();
        assert!(
            corners
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        );
        let stop = if end < start { end + 1.0 } else { end };
        let mut result = Vec::with_capacity(7);
        result.push(start);
        for shift in [0.0, 1.0] {
            for &corner in &corners[..4] {
                let unwrapped = corner + shift;
                if start < unwrapped && unwrapped < stop {
                    result.push(corner);
                }
            }
        }
        result.push(end);
        result
    }
    /// B 0x1412b1b60 streams each segment's endpoint/normal records, retaining
    /// the receiver's pair state across calls. Interior corners generate both
    /// neighboring edge records; an exact end-corner uses its outgoing normal.
    pub fn circular_slice_segment_points(
        self,
        start: f32,
        end: f32,
        state: &mut RectangleSliceState,
        mut receiver: impl FnMut(RectangleSlicePoint),
    ) {
        let _ = crate::curve::CurveU::<crate::curve::CurveCoordSemantic2D>::new(start);
        let _ = crate::curve::CurveU::<crate::curve::CurveCoordSemantic2D>::new(end);
        let corners = self.corner_us();
        let points = self.as_points2();
        let stop = if end < start { end + 1.0 } else { end };
        let span = (stop - start).max(1e-10);
        let reciprocal = 1.0 / span;
        let mut started = false;
        for index in 1..9 {
            let unwrapped = corners[index % 4] + (index / 4) as f32;
            if start >= unwrapped {
                continue;
            }
            let normal = self.get_edge_normal((index - 1) % 4);
            if !started {
                state.emit(
                    RectangleSlicePoint {
                        position: self.get_pos_at_u(start),
                        normal,
                        relative_u: 0.0,
                        tag: 5,
                    },
                    &mut receiver,
                );
                started = true;
            }
            if stop < unwrapped {
                state.emit(
                    RectangleSlicePoint {
                        position: self.get_pos_at_u(end),
                        normal,
                        relative_u: 1.0,
                        tag: 5,
                    },
                    &mut receiver,
                );
                return;
            }
            if stop == unwrapped {
                continue;
            }
            let position = points[index % 4];
            let relative_u = unwrapped * reciprocal - reciprocal * start;
            state.emit(
                RectangleSlicePoint {
                    position,
                    normal,
                    relative_u,
                    tag: 5,
                },
                &mut receiver,
            );
            state.emit(
                RectangleSlicePoint {
                    position,
                    normal: self.get_edge_normal(index % 4),
                    relative_u,
                    tag: 5,
                },
                &mut receiver,
            );
        }
    }
    /// Boundary intersections differ from overlap: contained contours return false.
    pub fn intersects_circle(self, circle: Circle2d) -> bool {
        let points = self.as_points2();
        (0..4).any(|index| {
            let ray = Ray2 {
                origin: points[index],
                direction: points[(index + 1) % 4] - points[index],
            };
            circle.intersect_ray(ray).is_some_and(|(near, far)| {
                (0.0..=1.0).contains(&near) || (0.0..=1.0).contains(&far)
            })
        })
    }
    /// B 0x140c9b990: test the other rectangle's four edges against the
    /// local slabs. Contained and coincident contours are not intersections.
    pub fn intersects(self, other: Self) -> bool {
        // B140c9b990：MOVSD搬运两个f32；全部运算为PS/SS，不是f64。
        let length = (self.size.y * self.size.y + self.size.x * self.size.x).sqrt();
        assert!(length >= 0.0, "原intersects尺寸长度guard");
        let determinant = self.axis.x * self.axis.x - (-self.axis.y) * self.axis.y;
        let positive = 1.0 / determinant;
        let negative = -1.0 / determinant;
        let column_x = Vec2::new(self.axis.x * positive, self.axis.y * negative);
        let column_y = Vec2::new((-self.axis.y) * negative, self.axis.x * positive);
        let transform = |point: Vec2| column_x * point.x + column_y * point.y;
        let relative_x = transform(other.axis);
        let relative_y = transform(Vec2::new(-other.axis.y, other.axis.x));
        // 原先分别转换两个中心再相减，不能先减世界坐标后变换。
        let relative_center = transform(other.center) - transform(self.center);
        let points = other.as_points2_centered_aligned()
            .map(|point| relative_x * point.x + relative_y * point.y + relative_center);
        let half = self.size * 0.5;
        let minnum = |first: f32, second: f32| {
            if first.is_nan() || second < first { second } else { first }
        };
        let maxnum = |first: f32, second: f32| {
            if first.is_nan() || second > first { second } else { first }
        };
        (0..4).any(|index| {
            let start = points[index];
            let direction = points[(index + 1) % 4] - start;
            let first_x = ((half.x + 0.0) - start.x) / direction.x;
            let last_x = ((0.0 - half.x) - start.x) / direction.x;
            let first_y = ((half.y + 0.0) - start.y) / direction.y;
            let last_y = ((0.0 - half.y) - start.y) / direction.y;
            let near = maxnum(minnum(first_x, last_x), minnum(first_y, last_y));
            let far = minnum(maxnum(first_x, last_x), maxnum(first_y, last_y));
            near <= far && 0.0 <= far
                && (far <= 1.0 || (0.0 <= near && near <= 1.0))
        })
    }
    /// 原 B 0x140c9c620：按给定上下界限制整条边的尺寸变化，保持对边不动。
    /// 原指令先 MAXSS(min, delta + old)，再 MINSS(max, result)；NaN 增量须传播。
    pub fn expand_along_edge(&mut self, edge: usize, delta: f32, min: f32, max: f32) {
        // 原 SSE 加减乘优先保留左操作数 NaN，再保留右操作数 NaN，并 quiet signaling NaN。
        // 两个操作数都不是 NaN 时产生的 invalid（∞-∞、0*∞ 等）则为原 x86 indefinite。
        // 仅在异常路径使用，不能把所有最终 NaN 统一置负而丢掉输入 payload。
        #[inline]
        fn sse_result(left: f32, right: f32, result: f32) -> f32 {
            if left.is_nan() {
                f32::from_bits(left.to_bits() | 0x0040_0000)
            } else if right.is_nan() {
                f32::from_bits(right.to_bits() | 0x0040_0000)
            } else if result.is_nan() {
                f32::from_bits(0xffc0_0000)
            } else {
                result
            }
        }
        #[cold]
        fn exceptional_center(axis: Vec2, center: Vec2, edge: usize, new: f32, old: f32) -> Vec2 {
            let mul = |a, b| sse_result(a, b, a * b);
            let add = |a, b| sse_result(a, b, a + b);
            let difference = sse_result(new, old, new - old);
            let shift = mul(difference, 0.5);
            let local = [-Vec2::X, Vec2::Y, Vec2::X, -Vec2::Y][edge];
            let normal_x = add(mul(-axis.y, local.y), mul(axis.x, local.x));
            let normal_y = add(mul(axis.x, local.y), mul(axis.y, local.x));
            // 原 MULPS 的左操作数为 shift；末次 ADDPS 的右操作数为旧中心。
            Vec2::new(add(mul(shift, normal_x), center.x), add(mul(shift, normal_y), center.y))
        }
        let dimension = if edge == 1 || edge == 3 { 1 } else { 0 };
        let old = self.size[dimension];
        let sum = delta + old;
        let sum = if sum.is_nan() { sse_result(delta, old, sum) } else { sum };
        let new = sum.clamp(min, max);
        self.size[dimension] = new;
        let normal = self.get_edge_normal(edge);
        let old_center = self.center;
        let center = old_center + normal * ((new - old) * 0.5);
        // 有限正常路径保留原向量算术；跨平台 NaN 契约只在实际出现 NaN 时处理。
        self.center = if center.is_nan() {
            exceptional_center(self.axis, old_center, edge, new, old)
        } else {
            center
        };
    }
    /// B separating-axis test includes touching edges as overlap.
    pub fn overlaps_rectangle2d(self, other: Self) -> bool {
        fn within(a: Rectangle2d, b: Rectangle2d) -> bool {
            let t = a.world_to_local(b.center);
            let h = a.size * 0.5;
            let bh = b.size * 0.5;
            let cos = (b.axis.y * a.axis.y + b.axis.x * a.axis.x).abs();
            let sin = (b.axis.y * a.axis.x + b.axis.x * (-a.axis.y)).abs();
            let x = bh.y * sin + bh.x * cos;
            let y = bh.x * sin + bh.y * cos;
            t.x - x <= h.x && -h.x <= x + t.x && t.y - y <= h.y && -h.y <= y + t.y
        }
        within(self, other) && within(other, self)
    }
    pub fn contains_point(self, p: Vec2) -> bool {
        self.contains_point_padding(p, Vec2::ZERO)
    }
    pub fn contains_point_padding(self, p: Vec2, dimensions: Vec2) -> bool {
        let p = self.world_to_local(p);
        let h = (self.size + dimensions) * 0.5;
        -p.x <= h.x && -p.y <= h.y && p.x <= h.x && p.y <= h.y
    }
    pub fn with_expanded_size(self, delta: Vec2) -> Self {
        Self {
            size: delta + self.size,
            ..self
        }
    }
    pub fn aabb(self) -> Aabb2 {
        let h = self.size * 0.5;
        let r = Vec2::new(
            self.axis.y.abs() * h.y + h.x * self.axis.x.abs(),
            self.axis.y.abs() * h.x + h.y * self.axis.x.abs(),
        );
        Aabb2 {
            min: self.center - r,
            max: self.center + r,
        }
    }
}
/// B footprint followed by y bounds. Horizontal edges are inclusive; vertical
/// containment is [bottom, top), while overlap includes touching y planes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle3d {
    pub footprint: Rectangle2d,
    pub bottom: f32,
    pub top: f32,
}
#[allow(clippy::assign_op_pattern)] // Preserve observed operand order for public mutable fields.
impl Rectangle3d {
    /// B rotates projected axis and center with the supplied two-dimensional
    /// linear transform; size and vertical bounds are preserved.
    pub fn rotate_projective_xz(&mut self, rotation: Mat2) {
        self.footprint.axis = rotation * self.footprint.axis;
        self.footprint.center = rotation * self.footprint.center;
    }
    pub fn new(center: Vec3, size: Vec3, axis: Vec2) -> Self {
        let half = size.y * 0.5;
        Self {
            footprint: Rectangle2d::new(
                Vec2::new(center.x, center.z),
                Vec2::new(size.x, size.z),
                axis,
            ),
            bottom: center.y - half,
            top: half + center.y,
        }
    }
    pub fn translate(&mut self, offset: Vec3) {
        self.footprint.center += Vec2::new(offset.x, offset.z);
        self.bottom = offset.y + self.bottom;
        self.top = offset.y + self.top;
    }
    pub fn set_orientation_xz(&mut self, axis: Vec2) {
        self.footprint.axis = axis;
    }
    pub fn set_size(&mut self, size: Vec3) {
        self.footprint.size = Vec2::new(size.x, size.z);
        let center = (self.bottom + self.top) * 0.5;
        let half = size.y * 0.5;
        self.bottom = center - half;
        self.top = half + center;
    }
    pub fn expand(&mut self, size_delta: Vec3) {
        self.footprint.size += Vec2::new(size_delta.x, size_delta.z);
        let half = size_delta.y * 0.5;
        self.bottom -= half;
        self.top = half + self.top;
    }
    pub fn extent(self) -> Vec3 {
        Vec3::new(
            self.footprint.size.x,
            self.top - self.bottom,
            self.footprint.size.y,
        )
    }
    pub fn contains_point(self, point: Vec3) -> bool {
        self.bottom <= point.y
            && point.y < self.top
            && self.footprint.contains_point(Vec2::new(point.x, point.z))
    }
    pub fn overlaps_rectangle3d(self, other: Self) -> bool {
        self.top >= other.bottom
            && other.top >= self.bottom
            && self.footprint.overlaps_rectangle2d(other.footprint)
    }
    pub fn points(self) -> [Vec3; 8] {
        let p = self.footprint.as_points2();
        [
            Vec3::new(p[0].x, self.bottom, p[0].y),
            Vec3::new(p[1].x, self.bottom, p[1].y),
            Vec3::new(p[2].x, self.bottom, p[2].y),
            Vec3::new(p[3].x, self.bottom, p[3].y),
            Vec3::new(p[0].x, self.top, p[0].y),
            Vec3::new(p[1].x, self.top, p[1].y),
            Vec3::new(p[2].x, self.top, p[2].y),
            Vec3::new(p[3].x, self.top, p[3].y),
        ]
    }
}

#[cfg(test)]
mod original_intersects_regressions {
    use super::*;
    #[test]
    fn complete_original_intersects_and_iterator_match_results_and_guards() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/fixtures/native/m01-rectangle-intersects.json")).unwrap();
        let mut mismatches = Vec::new();
        for row in fixture["cases"].as_array().unwrap() {
            let bits = row["bits"].as_array().unwrap();
            let rectangle = |start: usize| {
                let f = |index: usize| f32::from_bits(bits[start + index].as_u64().unwrap() as u32);
                Rectangle2d { axis: Vec2::new(f(0), f(1)), center: Vec2::new(f(2), f(3)), size: Vec2::new(f(4), f(5)) }
            };
            let actual = std::panic::catch_unwind(|| rectangle(0).intersects(rectangle(6)))
                .map_or(2, u8::from);
            let expected = row["expected"].as_u64().unwrap() as u8;
            if actual != expected { mismatches.push((row["id"].clone(), row["label"].clone(), actual, expected)); }
        }
        assert_eq!(mismatches.len(), 0, "前20项原侧差异: {:?}", &mismatches[..mismatches.len().min(20)]);
    }
}

#[cfg(test)]
mod normal_regressions {

    fn native_fixture_rectangle(value: &serde_json::Value) -> Rectangle2d {
        let values: Vec<f32> = value.as_array().unwrap().iter()
            .map(|v| f32::from_bits(v.as_u64().unwrap() as u32)).collect();
        Rectangle2d { axis: Vec2::new(values[0], values[1]),
            center: Vec2::new(values[2], values[3]), size: Vec2::new(values[4], values[5]) }
    }

    fn native_fixture_bits(rectangle: Rectangle2d) -> Vec<u32> {
        [rectangle.axis.x, rectangle.axis.y, rectangle.center.x, rectangle.center.y,
            rectangle.size.x, rectangle.size.y].map(f32::to_bits).to_vec()
    }

    #[test]
    fn original_expand_along_edge_complete_function_matches_native_bits_and_panics() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/fixtures/native/m01-shared-geometry.json")).unwrap();
        let mut mismatches = Vec::new();
        for (index, row) in fixtures["expand"].as_array().unwrap().iter().enumerate() {
            let mut rectangle = native_fixture_rectangle(&row["before"]);
            let value = |key: &str| f32::from_bits(row[key].as_u64().unwrap() as u32);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                rectangle.expand_along_edge(row["edge"].as_u64().unwrap() as usize,
                    value("delta"), value("lower"), value("upper"));
            }));
            assert_eq!(result.is_err(), row["panic"].as_bool().unwrap(), "原 expand guard {index}");
            let expected: Vec<u32> = serde_json::from_value(row["after"].clone()).unwrap();
            let actual = native_fixture_bits(rectangle);
            if actual != expected {
                mismatches.push(serde_json::json!({"index":index,"actual":actual,"expected":expected,"input":row}));
            }
        }
        if let Some(path) = std::env::var_os("TG_M01_GEOMETRY_DIFF_OUTPUT") {
            std::fs::write(path, serde_json::to_string_pretty(&mismatches).unwrap()).unwrap();
        }
        assert!(mismatches.is_empty(), "原 expand 位型不匹配 {} 例；首例 {:?}", mismatches.len(), mismatches.first());
    }

    #[test]
    fn original_rectangle_lerp_complete_function_matches_native_bits_and_panics() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/fixtures/native/m01-shared-geometry.json")).unwrap();
        for (index, row) in fixtures["lerp"].as_array().unwrap().iter().enumerate() {
            let before = native_fixture_rectangle(&row["before"]);
            let after = native_fixture_rectangle(&row["after"]);
            let factor = f32::from_bits(row["factor"].as_u64().unwrap() as u32);
            let result = std::panic::catch_unwind(|| before.lerp(after, factor));
            assert_eq!(result.is_err(), row["panic"].as_bool().unwrap(), "原 lerp guard {index}: {row}");
            if let Ok(actual) = result {
                let expected: Vec<u32> = serde_json::from_value(row["output"].clone()).unwrap();
                assert_eq!(native_fixture_bits(actual), expected, "原 lerp 完整输出 {index}: {row}");
            }
        }
    }

    #[test]
    fn original_corner_edge_directions_preserve_incoming_then_outgoing_order() {
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(4.0, 6.0));
        let expected = [
            [Vec2::X, Vec2::Y],
            [-Vec2::Y, Vec2::X],
            [-Vec2::X, -Vec2::Y],
            [Vec2::Y, -Vec2::X],
        ];
        for (index, directions) in expected.into_iter().enumerate() {
            assert_eq!(rectangle.get_edge_dirs_from_corner_id(index), directions);
        }
    }

    #[test]
    fn original_corner_edge_directions_use_actual_rotated_points() {
        let rectangle = Rectangle2d::new(Vec2::new(2.0, 3.0), Vec2::new(4.0, 6.0), Vec2::Y);
        let expected = [
            [Vec2::Y, -Vec2::X],
            [Vec2::X, Vec2::Y],
            [-Vec2::Y, Vec2::X],
            [-Vec2::X, -Vec2::Y],
        ];
        for (index, directions) in expected.into_iter().enumerate() {
            assert_eq!(rectangle.get_edge_dirs_from_corner_id(index), directions);
        }
    }

    #[test]
    fn original_corner_edge_directions_keep_positive_area_with_two_negative_sizes() {
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(-4.0, -6.0));
        assert_eq!(rectangle.get_edge_dirs_from_corner_id(0), [-Vec2::X, -Vec2::Y]);
    }

    #[test]
    fn original_corner_edge_directions_preserve_collapsed_world_point_nan() {
        // 原世界角点在此量级全部舍入到同一点；不能用轴查表掩盖零边。
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::splat(16_777_216.0), Vec2::ONE);
        for index in 0..4 {
            for direction in rectangle.get_edge_dirs_from_corner_id(index) {
                assert!(direction.x.is_nan() && direction.y.is_nan());
            }
        }
    }

    #[test]
    #[should_panic(expected = "原as_points正面积guard")]
    fn original_corner_edge_directions_check_area_before_corner_index() {
        Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::ZERO)
            .get_edge_dirs_from_corner_id(4);
    }

    #[test]
    #[should_panic(expected = "原角序超出0..4")]
    fn original_corner_edge_directions_reject_out_of_range_corner() {
        Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::ONE)
            .get_edge_dirs_from_corner_id(4);
    }
    #[test]
    fn original_perimeter_helpers_preserve_ties_and_closing_u() {
        let r = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
        assert_eq!(
            r.closest_point_on_perimeter(Vec2::ZERO),
            Vec2::new(-1.0, 0.0)
        );
        assert_eq!(r.get_closest_u_from_pos(Vec2::ZERO), 0.125);
        assert_eq!(r.get_closest_u_from_pos(Vec2::new(-0.5, -2.0)), 0.9375);
        assert_eq!(
            r.closest_point_on_perimeter(Vec2::new(-0.5, -2.0)),
            Vec2::new(-0.5, -1.0)
        );
    }

    #[test]
    fn original_pair_search_keeps_its_finite_initial_distance() {
        let r = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
        assert_eq!(
            r.closest_point_on_perimeter(Vec2::new(2000.0, 0.0)),
            Vec2::new(-1.0, 0.0)
        );
        assert_eq!(r.get_closest_u_from_pos(Vec2::new(2000.0, 0.0)), 0.125);
    }

    #[test]
    fn original_perimeter_helpers_use_actual_rotated_frame() {
        let r = Rectangle2d::new(Vec2::new(3.0, -4.0), Vec2::splat(2.0), Vec2::Y);
        assert_eq!(
            r.closest_point_on_perimeter(Vec2::new(3.0, -4.0)),
            Vec2::new(3.0, -5.0)
        );
        assert_eq!(r.get_closest_u_from_pos(Vec2::new(3.0, -4.0)), 0.125);
    }

    #[test]
    fn original_nan_point_propagates_for_point_but_rejects_u() {
        let r = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
        let point = Vec2::new(f32::NAN, 0.0);
        assert!(r.closest_point_on_perimeter(point).is_nan());
        assert!(std::panic::catch_unwind(|| r.get_closest_u_from_pos(point)).is_err());
        let zero = Rectangle2d {
            size: Vec2::ZERO,
            ..r
        };
        assert!(std::panic::catch_unwind(|| zero.closest_point_on_perimeter(Vec2::ZERO)).is_err());
    }

    use super::*;
    #[test]
    fn original_3d_overlap_rejects_each_unordered_vertical_bound() {
        let finite = Rectangle3d::new(Vec3::ZERO, Vec3::splat(2.0), Vec2::X);
        // Native305B constructor permits height NaN. Overlap29B, rather than
        // a new constructor guard, rejects the resulting unordered bounds.
        let nan_height = Rectangle3d::new(
            Vec3::ZERO,
            Vec3::new(2.0, f32::NAN, 2.0),
            Vec2::X,
        );
        assert!(nan_height.bottom.is_nan() && nan_height.top.is_nan());
        assert!(!nan_height.overlaps_rectangle3d(finite));
        assert!(!finite.overlaps_rectangle3d(nan_height));
        for field in 0..4 {
            let (mut a, mut b) = (finite, finite);
            match field {
                0 => a.top = f32::NAN,
                1 => a.bottom = f32::NAN,
                2 => b.top = f32::NAN,
                _ => b.bottom = f32::NAN,
            }
            assert!(!a.overlaps_rectangle3d(b), "bound {field}");
            assert!(!b.overlaps_rectangle3d(a), "reversed bound {field}");
        }
    }

    #[test]
    fn original_3d_overlap_keeps_touching_separation_and_ordered_infinity() {
        let a = Rectangle3d::new(Vec3::ZERO, Vec3::splat(2.0), Vec2::X);
        let touching = Rectangle3d::new(Vec3::new(0.0, 2.0, 0.0), Vec3::splat(2.0), Vec2::X);
        let separated = Rectangle3d::new(Vec3::new(0.0, 2.25, 0.0), Vec3::splat(2.0), Vec2::X);
        assert!(a.overlaps_rectangle3d(a));
        assert!(a.overlaps_rectangle3d(touching));
        assert!(touching.overlaps_rectangle3d(a));
        assert!(!a.overlaps_rectangle3d(separated));
        assert!(!separated.overlaps_rectangle3d(a));
        let unbounded = Rectangle3d { bottom: f32::NEG_INFINITY, top: f32::INFINITY, ..a };
        assert!(unbounded.overlaps_rectangle3d(a));
        assert!(a.overlaps_rectangle3d(unbounded));
    }

    #[test]
    fn original_edge_id_wrapper_keeps_finite_1000_distance_threshold() {
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
        assert_eq!(rectangle.get_closest_edge_id(Vec2::new(2000.0, 0.0)), 0);
        assert_eq!(rectangle.get_closest_edge_id(Vec2::new(0.0, 1001.0)), 0);
        assert_eq!(rectangle.get_closest_edge_id(Vec2::new(0.0, 1000.0)), 1);
        let rotated = Rectangle2d::new(Vec2::new(5.0, -6.0), Vec2::splat(2.0), Vec2::Y);
        assert_eq!(rotated.get_closest_edge_id(Vec2::new(5.0, 1994.0)), 0);
    }

    #[test]
    fn original_edge_id_wrapper_keeps_near_edge_and_first_ties() {
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
        assert_eq!(rectangle.get_closest_edge_id(Vec2::new(1.2, 0.0)), 2);
        assert_eq!(rectangle.get_closest_edge_id(Vec2::ZERO), 0);
        assert_eq!(rectangle.get_closest_edge_id(Vec2::ONE), 1);
    }

    #[test]
    #[should_panic(expected = "原as_points正面积guard")]
    fn original_edge_id_wrapper_requires_actual_positive_area() {
        Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::ZERO)
            .get_closest_edge_id(Vec2::new(1.0, 0.0));
    }

    #[test]
    fn size_normalized_normal_is_independent_of_nearest_segment() {
        let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::new(10.0, 2.0));
        let point = Vec2::new(4.0, 0.5);
        assert_eq!(rectangle.get_closest_edge_id(point), 1);
        assert_eq!(rectangle.closest_edge_normal(point), Vec2::X);
        assert_eq!(rectangle.closest_edge_normal(-point), -Vec2::X);
        assert_eq!(rectangle.closest_edge_normal(Vec2::ZERO), -Vec2::Y);
        // Equal normalized components select the Y axis.
        assert_eq!(rectangle.closest_edge_normal(Vec2::new(2.5, 0.5)), Vec2::Y);
    }
    #[test]
    fn rotated_normals_keep_world_orientation() {
        let axis = Vec2::new(0.6, 0.8);
        let rectangle = Rectangle2d::new(Vec2::new(2.0, 3.0), Vec2::new(10.0, 2.0), axis);
        assert_eq!(
            rectangle.closest_edge_normal(rectangle.local_to_world(Vec2::new(4.0, 0.1))),
            axis
        );
        assert_eq!(
            rectangle.closest_edge_normal(rectangle.local_to_world(Vec2::new(-4.0, 0.1))),
            -axis
        );
        assert_eq!(
            rectangle.closest_edge_normal(rectangle.local_to_world(Vec2::new(0.1, 0.8))),
            Vec2::new(-0.8, 0.6)
        );
        assert_eq!(
            rectangle.closest_edge_normal(rectangle.center),
            Vec2::new(0.8, -0.6)
        );
    }
    #[test]
    fn signed_distance_and_original_unsigned_gate_have_distinct_contracts() {
        let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::splat(2.0));
        assert_eq!(
            rectangle.signed_distance_to_closest_edge(Vec2::new(-3.0, -2.0)),
            2.0f32.hypot(1.0)
        );
        assert_eq!(
            rectangle.distance_to_closest_edge(Vec2::new(-3.0, -2.0)),
            0.0
        );
        assert_eq!(
            rectangle.signed_distance_to_closest_edge(Vec2::new(3.0, 2.0)),
            2.0f32.hypot(1.0)
        );
        assert_eq!(rectangle.distance_to_closest_edge(Vec2::new(3.0, 2.0)), 2.0f32.hypot(1.0));
        // 覆盖原圆柱邻墙反例的四个方位，保留非对称选择门。
        let neighbour = Rectangle2d::new_axis_aligned(Vec2::new(3.5, 0.0), Vec2::ONE);
        for (point, expected) in [
            (Vec2::new(2.1, 0.0), 0.0f32),
            (Vec2::new(0.0, -2.1), 0.0),
            (Vec2::new(-2.1, -0.0), 0.0),
            (Vec2::new(-0.0, 2.1), 2.1 - 0.5),
        ] {
            assert_eq!(neighbour.distance_to_closest_edge(point).to_bits(), expected.to_bits());
        }
    }
}

#[cfg(test)]
mod original_closed_corner_tests {
    use super::*;
    #[test]
    fn fifth_corner_closes_and_sixth_panics() {
        let rectangle = Rectangle2d::new(Vec2::new(3.0, -4.0), Vec2::new(8.0, 6.0), Vec2::new(0.6, 0.8));
        assert_eq!(rectangle.get_point(4), rectangle.get_point(0));
        assert!(std::panic::catch_unwind(|| rectangle.get_point(5)).is_err());
    }
}

#[cfg(test)]
mod original_intersection_tests {
    use super::*;
    #[test]
    fn matches_original_complete_rectangle_intersection() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-pin-rectangle-intersects-fixture.json"))).unwrap();
        assert_eq!(fixture["pe_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        let rectangle = |value: &serde_json::Value| {
            let f = |v: &serde_json::Value| v.as_f64().map(|n| n as f32).unwrap_or_else(|| v.as_str().unwrap().parse::<f32>().unwrap());
            let vec = |v: &serde_json::Value| Vec2::new(f(&v[0]), f(&v[1]));
            Rectangle2d { center: vec(&value[1]), size: vec(&value[2]), axis: vec(&value[0]) }
        };
        for case in fixture["cases"].as_array().unwrap() {
            let a = rectangle(&case["a"]); let b = rectangle(&case["b"]);
            let actual = std::panic::catch_unwind(|| a.intersects(b));
            if case["intersects"].as_str() == Some("panic_assert") {
                assert!(actual.is_err(), "{}", case["name"]);
            } else {
                assert_eq!(actual.unwrap(), case["intersects"].as_bool().unwrap(), "{}", case["name"]);
            }
        }
    }
}
