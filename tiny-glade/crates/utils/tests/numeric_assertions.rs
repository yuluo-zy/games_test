//! 已判等纯函数 / 常量的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据：
//! - `utils::rotation_from_normal`（B 0x140c90a40）：`reports/gap/utils-toolbar/RESTORE.md`
//!   「判等（4 case 半 sqrt 逐语句）」；4 分支期望由 fresh 反编译手算（见
//!   `crates/utils/src/geometry/mod.rs` 模块注释）。
//! - `Rectangle2d::orientation_mat3`（B 0x140c99570）：`reports/wave3/reviews/tg-tr/REVIEW.md`
//!   「维度/列序判等证实」（`Mat3::from_cols((axis.x,0,axis.y), Y, (-axis.y,0,axis.x))`）。
//!
//! 只断言已判等面；不做近似替代。

use glam::{Mat3, Vec2, Vec3};
use utils::geometry::Rectangle2d;
use utils::geometry::rotation_from_normal;

fn approx_quat(a: glam::Quat, b: glam::Quat) -> bool {
    (a.x - b.x).abs() < 1e-5
        && (a.y - b.y).abs() < 1e-5
        && (a.z - b.z).abs() < 1e-5
        && (a.w - b.w).abs() < 1e-5
}

fn assert_quat_or_neg(actual: glam::Quat, expected: glam::Quat, normal: Vec3) {
    assert!(
        approx_quat(actual, expected) || approx_quat(-actual, expected),
        "n={normal:?} actual={actual:?} expected={expected:?}"
    );
    // 语义校验：q 把默认 Z 轴对齐到 n（输入需为单位向量）。
    let rotated = actual * Vec3::Z;
    assert!(
        (rotated - normal.normalize()).length() < 1e-4,
        "n={normal:?} rotated={rotated:?}"
    );
}

/// `rotation_from_normal` 4 类法向的逐分支数值：
/// 退化竖直法向走 `nz<=0` 且 `cy'-wy<=0` 分支；Z 正/负向分别落
/// `nz>0` / `nz<=0` 两条；X 向落 `nz>0, wy+cy'>0` 分支。
#[test]
fn rotation_from_normal_four_branch_values() {
    // 法向 = +X：w=(1,0,0)、c=(0,-1,0)，矩阵列 (0,0,1)/(0,-1,0)/(1,0,0)
    // → 手算四元数 (√2/2, 0, √2/2, 0)，把 Z 转到 X。
    assert_quat_or_neg(
        rotation_from_normal(Vec3::X),
        glam::Quat::from_xyzw(0.707_106_77, 0.0, 0.707_106_77, 0.0),
        Vec3::X,
    );
    // 法向 = +Z：w=(0,-1,0)、c=(0,-1,0)，矩阵为绕 Z 180°。
    assert_quat_or_neg(
        rotation_from_normal(Vec3::Z),
        glam::Quat::from_xyzw(0.0, 0.0, 1.0, 0.0),
        Vec3::Z,
    );
    // 法向 = -Z：矩阵为绕 X 180°。
    assert_quat_or_neg(
        rotation_from_normal(-Vec3::Z),
        glam::Quat::from_xyzw(1.0, 0.0, 0.0, 0.0),
        -Vec3::Z,
    );
    // 退化法向 = +Y（n.x==n.z==0 → w=(0,1,0)、c=(0,1,0)）。
    assert_quat_or_neg(
        rotation_from_normal(Vec3::Y),
        glam::Quat::from_xyzw(0.707_106_77, 0.0, 0.0, -0.707_106_77),
        Vec3::Y,
    );
}

/// 斜法向（0.6, 0.8, 0）的非退化主路径：手算期望
/// (0.6324555, 0.3162277, 0.6324555, -0.3162277)（fresh decomp 复算，见模块注释）。
#[test]
fn rotation_from_normal_oblique_normal() {
    let n = Vec3::new(0.6, 0.8, 0.0);
    assert_quat_or_neg(
        rotation_from_normal(n),
        glam::Quat::from_xyzw(0.632_455_5, 0.316_227_7, 0.632_455_5, -0.316_227_7),
        n,
    );
}

/// 输入有限守卫（反编译第 1 段：三分量任一非有限 → panic）。
#[test]
#[should_panic]
fn rotation_from_normal_rejects_non_finite() {
    let _ = rotation_from_normal(Vec3::new(f32::NAN, 0.0, 0.0));
}

/// `Rectangle2d::orientation_mat3` 的 3 列逐字段（0x140c99570，判等）：
/// 列 = `(axis.x, 0, axis.y)`、`(0, 1, 0)`、`(-axis.y, 0, axis.x)`。
#[test]
fn rectangle_orientation_mat3_columns() {
    let identity = Mat3::IDENTITY;
    let r = Rectangle2d::new(Vec2::ZERO, Vec2::new(2.0, 1.0), Vec2::X);
    assert_eq!(r.orientation_mat3(), identity);

    let rotated = Rectangle2d::new(Vec2::ZERO, Vec2::new(2.0, 1.0), Vec2::Y);
    assert_eq!(
        rotated.orientation_mat3(),
        Mat3::from_cols(Vec3::new(0.0, 0.0, 1.0), Vec3::Y, Vec3::new(-1.0, 0.0, 0.0))
    );

    let oblique = Rectangle2d::new(Vec2::ZERO, Vec2::new(2.0, 1.0), Vec2::new(0.6, 0.8));
    assert_eq!(
        oblique.orientation_mat3(),
        Mat3::from_cols(Vec3::new(0.6, 0.0, 0.8), Vec3::Y, Vec3::new(-0.8, 0.0, 0.6))
    );
}
