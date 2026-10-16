//! `roof_ridge.rs` — 屋顶脊线尺寸与几何（B `resources/roofs/roof_ridge.rs`，7 业务）。
//!
//! 全部业务已由 reconstruction/roof/ridge.rs 恢复并经 `evidence/roof-ridge/` 的
//! 14,776 案例 0 差异验证；本文件按 PDB 原名暴露 `RoofRidgeDims` 的尺寸/几何查询，
//! 数值实现在共享 `system-roof` 的 `ridge` 模块（同一套机器码还原）。
//! 证据：`chapters/resources/roofs/roof_ridge.rs.md`。
use crate::resources::roofs::roof_shape::Roof;
use system_roof::ridge::{RidgeDims, RidgeError, RidgeGeometry};

/// 脊线最小轴尺寸（O：`RIDGE_MIN` = 0.1，`f32::from_bits(0x3dcccccd)`）。
pub const RIDGE_MIN: f32 = f32::from_bits(0x3dcccccd);

/// `RoofRidgeDims`（脊线最小/指定轴尺寸，两 f32）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofRidgeDims(RidgeDims);

impl RoofRidgeDims {
    /// 由两轴尺寸构造。
    pub fn new(dims: [f32; 2]) -> Self {
        Self(RidgeDims(dims))
    }
    /// 原始两轴尺寸。
    pub fn get(self) -> [f32; 2] {
        self.0 .0
    }
    /// `RoofRidgeDims::from`（0x1408e2690，O）：从 Roof 构造脊线尺寸：
    /// tag≠1 → `{0.1,0.1}`；否则 `(1−ridge)·0.1 + ridge·shape_dims[轴]`，ridge_dir bit(+0x3c) 选轴。
    pub fn from(roof: &Roof) -> Self {
        Self(RidgeDims(system_roof::ridge::ridge_dims_from_observed(roof.observed()).0))
    }
    /// `RoofRidgeDims::calc`（0x1408e2700，O）：脊线尺寸计算（ridge_length_01 线性混合，
    /// 最小轴 0.1）。
    pub fn calc(ridge_length_01: f32, ridge_dir: u8, shape_dims: [f32; 2]) -> Self {
        Self(RidgeDims(
            system_roof::ridge::ridge_dims_calc(ridge_length_01, ridge_dir, shape_dims).0,
        ))
    }
    /// `RoofRidgeDims::to_world_space`（0x1408e2750，O）：世界脊线——max_dim≤0.1 → 单点
    /// （保留尖顶偏移）；否则要求矩形并按 ridge_dir 旋转出线段端点（`0.0·height` 保留符号零）。
    pub fn to_world_space(self, roof: &Roof) -> Result<RidgeGeometry, RidgeError> {
        system_roof::ridge::ridge_to_world_space_observed(self.0, roof.observed())
    }
    /// `RoofRidgeDims::get_inner`（0x1400885d0，O）：内侧脊线尺寸（原 `return *param_1`
    /// 即首字段拷贝）。
    pub fn get_inner(self) -> Self {
        Self(self.0)
    }
    /// `RoofRidgeDims::max_dim`（内联，行 27）：两轴取 max（NaN 优先 = SSE MAXSS 语义）。
    pub fn max_dim(self) -> f32 {
        let [a, b] = self.0 .0;
        if a.is_nan() {
            b
        } else if b.is_nan() || b <= a {
            a
        } else {
            b
        }
    }
}

/// `calc_roof_tip_center_ls`（0x1408e30b0，O）：局部尖顶中心——circle→(2r, 2r)，
/// rect→(dim_x, dim_z)，乘 tip_offset_01 与 scale。
pub fn calc_roof_tip_center_ls(roof: &Roof, scale: [f32; 2]) -> [f32; 2] {
    system_roof::ridge::calc_roof_tip_center_ls_observed(roof.observed(), scale)
}

/// `inverse_ridge_01_param`（0x1408e30f0，O）：`(ridge_len−0.1)/(dim−0.1)`，NaN→panic，
/// 夹紧 [0,1]。
pub fn inverse_ridge_01_param(
    ridge_length: f32,
    ridge_dir: u8,
    shape_dims: [f32; 2],
) -> Result<f32, RidgeError> {
    system_roof::ridge::inverse_ridge_01_param(ridge_length, ridge_dir, shape_dims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::roof_shape::Roof;

    fn roof_with(bytes: &[(usize, u32)]) -> Roof {
        let mut b = [0u8; 88];
        for (offset, value) in bytes {
            b[*offset..*offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(b)
    }

    #[test]
    fn from_rect_uses_ridge_dir_axis() {
        let roof = roof_with(&[
            (8, 1),                        // rect tag
            (0x1c, f32::to_bits(4.0)),     // dim_x
            (0x20, f32::to_bits(6.0)),     // dim_z
            (0x34, f32::to_bits(0.5)),     // ridge_length_01
            (0x3c, 1),                     // ridge_dir → z axis
        ]);
        let dims = RoofRidgeDims::from(&roof).get();
        assert!((dims[0] - 0.1).abs() < 1e-6);
        assert!((dims[1] - (0.5 * 6.0 + 0.05)).abs() < 1e-5);
    }

    #[test]
    fn from_circle_is_min_dim() {
        let roof = roof_with(&[(8, 0)]);
        assert_eq!(RoofRidgeDims::from(&roof).get(), [0.1, 0.1]);
    }

    #[test]
    fn inverse_ridge_clamps_and_rejects_nan() {
        // (5.0−0.1)/(4.0−0.1)=1.256 → 夹紧 1.0。
        assert_eq!(inverse_ridge_01_param(5.0, 0, [4.0, 3.0]).unwrap(), 1.0);
        // 下界夹紧 0.0。
        assert_eq!(inverse_ridge_01_param(-1.0, 0, [4.0, 3.0]).unwrap(), 0.0);
        assert!(inverse_ridge_01_param(f32::NAN, 0, [4.0, 3.0]).is_err());
    }

    #[test]
    fn world_space_point_for_flat_circle() {
        let roof = roof_with(&[
            (8, 0),
            (0x30, f32::to_bits(0.05)), // flat
            (0x14, f32::to_bits(3.0)),
            (0x24, f32::to_bits(0.5)),
            (0x28, f32::to_bits(0.5)),
        ]);
        let dims = RoofRidgeDims::new([0.1, 0.1]);
        match dims.to_world_space(&roof) {
            Ok(RidgeGeometry::Point(_)) => {}
            other => panic!("flat circle should yield a point ridge, got {other:?}"),
        }
    }
}
