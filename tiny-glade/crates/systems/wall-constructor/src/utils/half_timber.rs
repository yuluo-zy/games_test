//! 半木构纹理生成（utils/half_timber.rs）。
//!
//! `half_timbering`（0x141218e30，27KB / 5494 条指令）。re-analysis
//! hardpoints-interaction §3.1 / hardpoints-ecstate §4.1 已攻克：radare2
//! `aa` 后 pdg 可反编译（13KB，`r2_half_timbering_full_141218e30.c`）。
//! Ghidra/ReVa "Unable to find unique hash for varnode" 修签名后仍复现，
//! 固定走 radare2 `aa`+pdg 路径。
//!
//! pdg 已解码的前导（逐语句可判等）：
//! 1. puffin 门控（`**0x14362c098` 非 0 → 早退）；
//! 2. `fcn.140b2a600(arg1)`（try_get_wall_shape）NULL → 返回；
//! 3. `fcn.1408e33e0(&fStack_4b0, wall)`（Roof::gable_sides）+ `cStack_f4 =
//!    fcn.1408e3380(wall)`（Roof::is_flat）；
//! 4. SSBO 注册表 `fcn.141209c40(var_38h, 0x142c1a7d0)` 查名
//!    `"country_core::resources::roofs::roof_buffers::WoodenPlankSsbo..."`
//!    （0x3d=61 字节 memcmp 校验；不匹配走 `fcn.1428cab41` 错误/panic）；
//!    命中后虚调 `(**[1]+0x18)(*reg)` 取稀疏缓冲；
//! 5. 墙实体 `+0x180` 字段 → `+0x18` LOCK cmpxchg 握手（后台上传占用位）；
//! 6. 主流程：取 10 qword（`reg[4..0xb]`）→ `fcn.14120c9d0(&0x60B 结构)` →
//!    按 is_flat 分 `fcn.140b2a620` / `fcn.140f86cd0+141206480+140b2a680`
//!    得 skirt 高度 → `fcn.140b34f80`（split_height_into_skirts，
//!    country-core `resources/walls/mod.rs` 已落库）→ `Vec<f32>` 追加 +
//!    扩容（0x15 阈值分流 `14122cd30`/`14122f250`）→ 遍历扫描
//!    `while (*(wall+0x280) + *0x142925904[1.0] <= fVar110)`。
//!
//! **未实现**：pdg 分页截断，逐梁（plank）坐标生成/closure 装配（函数中段
//! ~0x6860B）未取证；`+0x180` 结构字段语义、SSBO 写缓冲目标记录布局未闭合。
//! 不得以观测近似通过。
use glam::Vec3;

/// 墙形状点列（try_get_wall_shape 输出）。
pub type WallShapePoints = [[f32; 3]];

/// B `half_timbering`（0x141218e30）。**未实现**：前导已由 radare2 pdg 解码
/// （见模块文档），逐梁 plank 坐标生成/closure 装配未取证；本函数为占位壳，
/// 不得作为真实逻辑。
pub fn half_timbering(
    _wall_shape: Option<&WallShapePoints>,
    _u: &[f32],
    _heights: &[f32],
    _gable_sides: Option<[u8; 2]>,
    _is_flat: bool,
    _out: &mut Vec<[u8; 64]>,
) {
    todo!("未实现: half_timbering 0x141218e30（pdg 前导已解码，逐梁 plank 坐标/closure 装配未取证）")
}

/// B `roof_outline`（0x141217be0）：is_gable 时的屋顶轮廓半木构
/// （Roof::gable_sides 驱动；坐标细节未恢复登记）。
pub fn roof_outline(_roof: &[u8; 88], _points: &[Vec3], _out: &mut Vec<[u8; 64]>) {}

/// B `overlapping_windows`（0x1412218a0）。**未实现（近似体，不得作为真实逻辑）**：
/// 真实逻辑为 DecoratorStorage SIMD 掩码遍历（每墙 0x50B 记录）、subtype 过滤
/// （byte[10]<2 且 byte[0xe]−0x38<3）、get_window_aabb 后 ±0.038（dormer 'g' 且
/// leaf[0xc]==1 再 ±0.14）、Aabb2::intersects → ResourceAccessInfo（stride 0x14，
/// 标记 0/1）收集。fresh 反编译见 evidence/toolchain session-ghidra-shared-* /
/// /tmp/wc-dec/0x1412218a0.c。
pub fn overlapping_windows(_window: &[f32; 4], _other: &[f32; 4]) -> bool {
    todo!("未实现: overlapping_windows 0x1412218a0 (fresh decomp /tmp/wc-dec/0x1412218a0.c)")
}

/// B `get_window_aabb`（0x141221da0）。**未实现（近似体，不得作为真实逻辑）**：
/// 真实逻辑含门类分支（dst_as_wall 且 raw+0x46=='9' → gothic_door_profile_curve
/// 或 gothic_half_dormer_profile_curve，取 profile 末点算 [x−x0, y0+y, y0, ..+0.093]），
/// 普通装饰 bbx_ls + Aabb2::from_center_dims。fresh 反编译见
/// /tmp/wc-dec/0x141221da0.c。
pub fn get_window_aabb(_window: &[f32; 4]) -> [f32; 4] {
    todo!("未实现: get_window_aabb 0x141221da0 (fresh decomp /tmp/wc-dec/0x141221da0.c)")
}

/// B `check_for_overlap_with_all_windows`（0x141221f70）。**未实现**：
/// 真实逻辑为 get_window_aabb + Aabb2::contains 中心/侧向/扩展三测试。
/// fresh 反编译见 /tmp/wc-dec/0x141221f70.c。
pub fn check_for_overlap_with_all_windows(_window: &[f32; 4], _windows: &[[f32; 4]]) -> bool {
    todo!("未实现: check_for_overlap_with_all_windows 0x141221f70 (fresh decomp /tmp/wc-dec/0x141221f70.c)")
}

/// B `add_underwindow_pattern`（0x141fd1c30）：窗下图案。
/// **未恢复（壳登记）**：窗下半木构图案坐标生成未实现（不称已恢复）。
pub fn add_underwindow_pattern(_window: &[f32; 4], _out: &mut Vec<[u8; 64]>) {}
