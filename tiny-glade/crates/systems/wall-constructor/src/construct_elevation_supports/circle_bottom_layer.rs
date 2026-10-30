//! 圆形底部砖层（construct_elevation_supports/circle_bottom_layer.rs）。
//!
//! `construct_circle_bottom_layer`（0x14124b2c0）+ `add_bottom_brick`
//! （0x141fe1de0，11 参；circle/rect 两闭包共用同一体）。
//!
//! 2026-10-17 restorer-remaining-wallconstructor fresh 结论（证据
//! `evidence/elev-0x14124b2c0.c`、`evidence/abb-dis.result.json`）：
//! 旧实现（TAU 均分 + 仅写 position 的 96B 记录）为**近似体**，已移除。
//!
//! `construct_circle_bottom_layer` 已解结构：
//! - 入参 circle：center Vec2@0、`radius+0.28`@8、@0xc；
//! - `Circle2d::perimeter()` → `ceil(perimeter / 0.69)` 段数；
//! - `WallStyle::is_halftimbered()` 早退；
//! - `AssetSsboLibrary` FrozenMap 取 `BrickTransformSsbo` 缓冲（0x3a 字节
//!   类型名 memcmp），`BufferState` 加锁（+0x18 自旋位 / parking_lot
//!   lock_slow / release_resource）；
//! - `utils::random_splits_into` 生成 u 序列，逐对 `Circle2d::get_pos_at_u`
//!   → `add_bottom_brick`（循环按首/次点交替推进）；
//! - 解锁后释放 BufferState。
//!
//! `add_bottom_brick` 已解结构（105 条指令）：`fastrand::Rng::f32` 抖动、
//! `x0y`×2、`xvy`、`render::impl$1::from`（0x60B 记录装配）、
//! `instanced_wall::impl$1::from`/`impl$3::default`、
//! `SparseInstanceBufferWriter::push<BrickTransformSsbo>`，失败置脏字节 1。
//!
//! 单砖已按原指令生成 typed 载荷；底层循环为候选，尚未接实际 owner 或整函数原侧差分。
use glam::Vec2;
use country_core::geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo};
use country_core::resources::walls::WallId;

/// 原 0x141fe1de0 的单砖几何及96字节载荷；调用方负责给出原随机值。
fn bottom_brick_record(
    sample: f32, start: Vec2, end: Vec2, base_y: f32, available_height: f32,
    wall: WallId, seed: u32, wall_distance: [f32; 2],
) -> BrickTransformSsbo {
    let half = 0.5;
    let jitter = (sample * half - (1.0 - sample) * half) * 0.2 + 0.4;
    let height = (available_height - 0.6).min(jitter);
    let delta = end - start;
    // 原长度和方向分别求平方和，保留原 SSE 的运算次序。
    let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
    let inverse = 1.0 / (delta.x * delta.x + delta.y * delta.y).sqrt();
    let x = delta.x * inverse;
    let z = delta.y * inverse;
    let center = (start + end) * half;
    BrickTransformSsbo {
        affine_columns: [
            [length * x, 0.0, length * z],
            [0.0, height, 0.0],
            [-z * 0.2, 0.0, x * 0.2],
            [center.x, base_y + height * half, center.y],
        ],
        seed,
        material: BrickSourceId::from_wall_id(country_core::resources::walls::publication_fields::WallId(wall.0)).0,
        wall_distance,
        ..Default::default()
    }
}

/// B 0x141fe1de0：消费一次原随机流，生成一砖并执行一次写入。
/// 返回 true 表示原 writer 失败；调用者据此保留脏标记并继续其原循环。
pub fn add_bottom_brick<E>(
    rng: &fastrand::Rng, start: Vec2, end: Vec2, base_y: f32,
    available_height: f32, wall: WallId, seed: u32, wall_distance: [f32; 2],
    mut push: impl FnMut(&BrickTransformSsbo) -> Result<(), E>,
) -> bool {
    let record = bottom_brick_record(rng.f32(), start, end, base_y, available_height, wall, seed, wall_distance);
    push(&record).is_err()
}

#[cfg(test)]
mod brick_tests {
    use super::*;
    #[test]
    fn circle_bottom_layer_matches_native_brick_sequence() {
        let native = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-bottom-native-output.json");
        let expected: Vec<u32> = native.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        let mut actual = Vec::new();
        let failed = construct_circle_bottom_layer(WallId(77), utils::geometry::Circle2d { center: Vec2::ZERO, radius: 2.0, rotation: 0.0 }, 3.0, 14.0, false, |brick| -> Result<(), ()> {
            actual.extend(brick.observed_bytes().chunks_exact(4).map(|word| u32::from_le_bytes(word.try_into().unwrap())));
            Ok(())
        });
        assert!(!failed);
        assert_eq!(actual.len(), 21 * 24);
        assert_eq!(actual, expected);
    }
    #[test]
    fn bottom_brick_matches_native_payload() {
        let record = bottom_brick_record(0.25, Vec2::new(1.0,2.0), Vec2::new(4.0,6.0), 3.0, 4.0, WallId(77), 9, [0.25,0.75]);
        let expected: [u32; 24] = [0x40400000,0,0x40800000,0,0x3eb33333,0,0xbe23d70b,0,0x3df5c290,0x40200000,0x404b3333,0x40800000,0,0,0,0,0xc2c80000,0xc2c80000,0xc2c80000,9,77,0x3e800000,0x3f400000,0];
        let bytes=record.observed_bytes();
        let actual: Vec<_> = bytes.chunks_exact(4).map(|word| u32::from_le_bytes(word.try_into().unwrap())).collect();
        assert_eq!(actual, expected);
    }
}

/// 原 0x14124b2c0：在圆周外扩 0.28 后按随机边界铺设底砖。
/// `available_height` 是原 PublicWallState +0x284 的派生值；写入回调
/// 连接墙独占砖 owner，并返回原 push 失败。此函数不清空共享 owner。
pub fn construct_circle_bottom_layer<E>(
    wall: WallId,
    circle: utils::geometry::Circle2d,
    elevation: f32,
    available_height: f32,
    halftimbered: bool,
    mut push: impl FnMut(&BrickTransformSsbo) -> Result<(), E>,
) -> bool {
    let expanded = utils::geometry::Circle2d { radius: circle.radius + 0.28, ..circle };
    let perimeter = expanded.perimeter();
    let seed = wall.0.wrapping_add((perimeter * 1000.0) as u64);
    if halftimbered { return false; }
    let rng = fastrand::Rng::with_seed(seed);
    let count = ((perimeter / 0.69).ceil() as usize).wrapping_add(1);
    let mut splits = Vec::new();
    utils::random_splits_into(count, 0.32 / perimeter, &rng, &mut splits);
    let mut failed = false;
    for (index, pair) in splits.windows(2).enumerate() {
        let start_u = utils::curve::CurveU::<utils::curve::CurveCoordSemantic2D>::new(pair[0]).get();
        let end_u = utils::curve::CurveU::<utils::curve::CurveCoordSemantic2D>::new(pair[1]).get();
        let start = expanded.get_pos_at_u(start_u);
        let end = expanded.get_pos_at_u(end_u);
        let distance = start_u * perimeter;
        failed |= add_bottom_brick(&rng, start, end, elevation, available_height, wall,
            (wall.0 as u32).wrapping_add(index as u32), [distance, distance], &mut push);
    }
    failed
}

/// 在正式墙砖 owner 上执行原圆底层；同一 writer 持锁直到全部逐砖写完。
/// 调用前须由检测器清理并建立该墙原有分配。
pub fn construct_circle_bottom_layer_for_wall(
    id: WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
) {
    let circle = wall.is_circle().expect("圆形支撑事件对应的墙必须为圆形");
    let halftimbered = wall.building_projection().is_halftimbered();
    if halftimbered { return; }
    let elevation = wall.authority.elevation();
    let height = wall.building_projection().float_0x284();
    let storage = library.get_mut::<BrickTransformSsbo>()
        .expect("原圆底层要求注册 BrickTransformSsbo");
    let mut failures = 0;
    {
        let mut writer = output.gpu.brick_writer(storage, id.0);
        construct_circle_bottom_layer(id, circle, elevation, height, false, |record| {
            let result = writer.push(record);
            if result.is_err() { failures += 1; }
            result
        });
    }
    output.gpu.record_brick_failures(crate::gpu_output::ConstructorKey {
        wall: id.0,
        part: crate::gpu_output::ConstructorPart::ElevationSupports,
    }, failures);
}
