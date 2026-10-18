//! Geometry queries, perimeter sampling, and coordinate transformations.
pub mod aabb;
pub mod barycentric;
pub mod circle;
pub mod intersect;
pub mod line_relation;
pub mod plane;
pub mod position_from_mesh_uvs;
pub mod ray;
pub mod rectangle;
pub mod split_heightmap;
pub mod triangle_triangle;
pub use aabb::{Aabb2, Aabb3, RayBoxIntersection};
pub use circle::Circle2d;
pub use plane::Plane;
pub use ray::{Ray, Ray2};
pub use rectangle::{Rectangle2d, Rectangle3d};
/// B 0x14127c970 projects a world-size unit at a given distance into normalized
/// screen-height units. It is a scalar scale factor, not a world-position transform.
pub fn ws_to_cs(distance: f32, projection: glam::Mat4) -> f32 {
    (0.5 / distance) * projection.y_axis.y
}
/// B 0x14127c950 additionally converts normalized screen height to pixels.
pub fn ws_to_ss(distance: f32, projection: glam::Mat4, screen_height: f32) -> f32 {
    ws_to_cs(distance, projection) * screen_height
}

/// Builds an orientation whose local +Z axis follows a finite normal.
/// Unit normals produce unit rotations. Zero or overflowing intermediate
/// vectors use a deterministic basis fallback.
pub fn rotation_from_normal(normal: glam::Vec3) -> glam::Quat {
    let n = normal;
    assert!(n.x.is_finite() && n.y.is_finite() && n.z.is_finite());
    // Preserve the established basis orientation, including the vertical fallback.
    let inv = 1.0 / (n.x * n.x + n.z * n.z).sqrt();
    let valid = inv.is_finite() && inv != 0.0;
    let w = glam::Vec3::new(
        if valid { n.x * inv } else { 0.0 },
        if valid { -n.z * inv } else { 1.0 },
        0.0,
    );
    // Keep the scalar summation order stable for existing numeric consumers.
    let raw = glam::Vec3::new(
        n.y * w.x - n.z * w.z,
        n.z * w.y - n.x * w.x,
        n.x * w.z - n.y * w.y,
    );
    // B 140c90bb3..be5: z² + (y² + x²), preserving scalar rounding order.
    let c_inv = 1.0 / (raw.z * raw.z + (raw.y * raw.y + raw.x * raw.x)).sqrt();
    let c_valid = c_inv.is_finite() && c_inv != 0.0;
    let c = glam::Vec3::new(
        if c_valid { raw.x * c_inv } else { 0.0 },
        if c_valid { raw.y * c_inv } else { 1.0 },
        if c_valid { raw.z * c_inv } else { 0.0 },
    );

    let q = glam::Quat::from_mat3(&glam::Mat3::from_cols(glam::Vec3::new(w.y, w.z, w.x), c, n));

    if q.is_finite() {
        q
    } else {
        glam::Quat::IDENTITY
    }
}

#[cfg(test)]
mod rotation_from_normal_tests {
    use super::*;

    fn approx(a: glam::Quat, b: glam::Quat) -> bool {
        (a.x - b.x).abs() < 1e-5
            && (a.y - b.y).abs() < 1e-5
            && (a.z - b.z).abs() < 1e-5
            && (a.w - b.w).abs() < 1e-5
    }

    #[test]
    fn z_axis_maps_to_normal() {
        // 反编译手算用例（fresh 0x140c90a40 逐分支复算，与 glam from_mat3 双证）：
        let cases = [
            (
                glam::Vec3::new(1.0, 0.0, 0.0),
                glam::Quat::from_xyzw(0.70710677, 0.0, 0.70710677, 0.0),
            ),
            (
                glam::Vec3::new(0.0, 0.0, 1.0),
                glam::Quat::from_xyzw(0.0, 0.0, 1.0, 0.0),
            ),
            (
                glam::Vec3::new(0.0, 0.0, -1.0),
                glam::Quat::from_xyzw(1.0, 0.0, 0.0, 0.0),
            ),
            (
                glam::Vec3::new(0.0, 0.5, 0.8660254),
                glam::Quat::from_xyzw(0.0, 0.258819, 0.9659258, 0.0),
            ),
            (
                glam::Vec3::new(0.6, 0.8, 0.0),
                glam::Quat::from_xyzw(0.6324555, 0.3162277, 0.6324555, -0.3162277),
            ),
        ];
        for (n, expected) in cases {
            let q = rotation_from_normal(n);
            // q 与期望值同旋转（q 与 -q 等价；方向分量符号以期望为准）。
            assert!(
                approx(q, expected) || approx(-q, expected),
                "n={n:?} q={q:?}"
            );
            // 旋转把 Z 对齐到 n（q * Z * q⁻¹ ≈ n）。
            let rotated = q * glam::Vec3::Z;
            assert!(
                (rotated - n.normalize()).length() < 1e-4,
                "n={n:?} rotated={rotated:?}"
            );
        }
    }

    #[test]
    fn degenerate_vertical_normal() {
        // n=(0,1,0)：v=(0,0,0) 退化 → w=(0,1,0)；nz=0 → nz≤0 分支
        // cy'-wy = -1-1 = -2 ≤ 0：t'=(1-0)-(-2)=3、f=0.5/sqrt(3)，
        // q=(f·t', f·(cx'+wz), f·(nx+wx), f·(cz'-ny))——实测 from_mat3 = -90° 绕 X。
        let q = rotation_from_normal(glam::Vec3::Y);
        assert!(
            approx(q, glam::Quat::from_xyzw(0.70710677, 0.0, 0.0, -0.70710677)),
            "q={q:?}"
        );
        let rotated = q * glam::Vec3::Z;
        assert!((rotated - glam::Vec3::Y).length() < 1e-4);
    }

    #[test]
    fn overflow_normal_uses_degenerate_fallback() {
        // reviewer-gap-round FIX-2：分量有限但平方溢出 inf → inv=1/inf=0。
        // 原生有效掩码 = 有限且非零 → 排除 0 → w 走退化 (0,1,0)（旧 `is_finite()` 会取
        // w=v*0=(0,-0,0)）。本实现 `is_finite() && inv != 0.0` 同判。
        // 注：n 非单位时 from_mat3 输出非单位（原函数不归一化 n，两实现同），故只断言
        // 有限（不 panic/不 NaN），不断言单位长度。
        let n = glam::Vec3::new(3.0e38, 0.0, 3.0e38);
        let q = rotation_from_normal(n);
        assert!(q.is_finite(), "q={q:?}");
        // 非退化单位输入仍按正常归一化（Z→n）：用单位化 (1,0,1)/√2。
        let unit = glam::Vec3::new(1.0, 0.0, 1.0).normalize();
        let q2 = rotation_from_normal(unit);
        assert!(q2.is_finite());
        let rotated = q2 * glam::Vec3::Z;
        assert!((rotated - unit).length() < 1e-4, "rotated={rotated:?}");
    }
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod completion {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/geometry/native_tests.rs"
    ));
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod native_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/geometry_tests.rs"
    ));
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod boundary_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/boundary_tests.rs"
    ));
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod edge_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/edge_tests.rs"
    ));
}
#[cfg(test)]
mod linear_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/linear_tests.rs"
    ));
}
#[cfg(test)]
mod triangle_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/triangle_tests.rs"
    ));
}
