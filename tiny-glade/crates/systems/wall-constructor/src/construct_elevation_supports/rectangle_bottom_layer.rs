//! 矩形底部砖层（construct_elevation_supports/rectangle_bottom_layer.rs）。
//!
//! `construct_rect_bottom_layer`（0x14124eb80）；`add_bottom_brick`
//! （0x141fe1de0）与圆形版共用同一体。
//!
//! 2026-10-17 复核：旧实现（宽度/深度各按 1.2 均分网格）为**近似体**，
//! 已移除。fresh（`evidence/elev-0x14124eb80.c`）已解结构：halftimber 早退
//! → halftimber 四角 plank 与普通 5 段边二路 → `ceil(len / 0.69)` 计数 +
//! `jitter=0.32` 宽/长 → `random_splits_into` 逐对起止点 → `add_bottom_brick`。

/// 普通石墙底砖的原四边随机分段；调用方负责保持同一墙砖 writer。
/// 半木底层另有独立木板分支，不由此函数替代。
pub(crate) fn construct_rect_stone_bottom_layer<E>(
    wall: country_core::resources::walls::WallId,
    mut rectangle: utils::geometry::Rectangle2d,
    elevation: f32, available_height: f32,
    mut push: impl FnMut(&country_core::geometry::instanced_wall::BrickTransformSsbo) -> Result<(), E>,
) -> bool {
    rectangle.size += glam::Vec2::splat(0.56);
    let corners = rectangle.corner_us();
    let perimeter = rectangle.perimeter();
    let mut failed = false;
    for edge in 0..4 {
        let start = rectangle.get_point(edge);
        let end = rectangle.get_point(edge + 1);
        let delta = start - end;
        let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
        let count = ((length / 0.69).ceil() as usize).wrapping_add(1);
        let rng = fastrand::Rng::with_seed((length * 1000.0) as u64 + edge as u64 + (edge + 1) as u64);
        let mut splits = Vec::new();
        utils::random_splits_into(count, 0.32 / length, &rng, &mut splits);
        for (index, pair) in splits.windows(2).enumerate() {
            let a = pair[0]; let b = pair[1];
            let p = start * (1.0 - a) + end * a;
            let q = start * (1.0 - b) + end * b;
            let da = (corners[edge] + (corners[edge + 1] - corners[edge]) * a) * perimeter;
            let db = (corners[edge] + (corners[edge + 1] - corners[edge]) * b) * perimeter;
            let seed = (wall.0 as u32).wrapping_add(index as u32);
            failed |= super::circle_bottom_layer::add_bottom_brick(&rng, p, q, elevation,
                available_height, wall, seed, [da, db], &mut push);
        }
    }
    failed
}

/// 普通石墙底层持有正式墙砖 owner，一次锁内按原顺序追加全部砖。
pub(crate) fn construct_rect_stone_bottom_for_wall(
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    rectangle: utils::geometry::Rectangle2d,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
) {
    use country_core::geometry::instanced_wall::BrickTransformSsbo;
    let storage = library.get_mut::<BrickTransformSsbo>().expect("原矩形底层要求注册 BrickTransformSsbo");
    let mut failures = 0;
    {
        let mut writer = output.gpu.brick_writer(storage, id.0);
        construct_rect_stone_bottom_layer(id, rectangle, wall.authority.elevation(),
            wall.building_projection().float_0x284(), |brick| {
                let result = writer.push(brick);
                if result.is_err() { failures += 1; }
                result
            });
    }
    output.gpu.record_brick_failures(crate::gpu_output::ConstructorKey {
        wall: id.0, part: crate::gpu_output::ConstructorPart::ElevationSupports,
    }, failures);
}

/// 半木矩形底层：每条边从起点外移后生成一条 64 字节木板。
pub(crate) fn construct_rect_wood_bottom_layer<E>(
    wall: country_core::resources::walls::WallId,
    mut rectangle: utils::geometry::Rectangle2d,
    elevation: f32,
    mut push: impl FnMut(&country_core::resources::roofs::roof_buffers::WoodenPlankSsbo) -> Result<(), E>,
) -> usize {
    use country_core::resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo};
    use glam::{Vec2, Vec3};
    rectangle.size += Vec2::splat(0.56);
    let mut failures = 0;
    let c = f32::from_bits(0x3f3504f3);
    for edge in 0..4 {
        let a = rectangle.get_point(edge);
        let b = rectangle.get_point(edge + 1);
        let delta = a - b;
        let inverse = 1.0 / (delta.y * delta.y + delta.x * delta.x).sqrt();
        let direction = Vec2::new(delta.x * inverse, delta.y * inverse);
        let position = Vec3::new(a.x - direction.y * 0.1, elevation + 0.2, direction.x * 0.1 + a.y);
        let q = utils::xz_rotation_bad(direction);
        // 原四元数相乘逐分量及列矩阵运算顺序；负零参与原浮点写面。
        let x = ((q.z * -0.0 * -1.0 + q.y * -0.0 * 1.0) + q.x * c * 1.0) + q.w * -c;
        let y = ((q.z * -c * 1.0 + q.y * c * 1.0) + q.x * -0.0 * -1.0) + q.w * -0.0;
        let z = ((q.z * c * 1.0 + q.y * -c * -1.0) + q.x * -0.0 * 1.0) + q.w * -0.0;
        let w = ((q.z * -0.0 * -1.0 + q.y * -0.0 * -1.0) + q.x * -c * -1.0) + q.w * c;
        let zz = z + z; let yy = y + y; let xx = x + x;
        let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
        let columns = [
            Vec3::new((1.0 - (y * yy + z * zz)) * 0.4, (yy * x + w * zz) * 0.4, (zz * x - yy * w) * 0.4),
            Vec3::new(length * (yy * x - w * zz), length * (1.0 - (z * zz + x * xx)), length * (xx * w + zz * y)),
            Vec3::new((yy * w + zz * x) * 0.4, (zz * y - xx * w) * 0.4, (1.0 - (x * xx + y * yy)) * 0.4),
        ];
        let plank = WoodenPlankSsbo { transform: Affine3Packed::from_cols(columns[0], columns[1], columns[2], position),
            gate_seed: 0, piece_seed: (wall.0 as u32).wrapping_add(edge as u32) as i32,
            wall_id: wall.0 as u32, flags: 0 };
        if push(&plank).is_err() { failures += 1; }
    }
    failures
}

/// 半木底层写入墙的原 +0x140 木板 owner。
pub(crate) fn construct_rect_wood_bottom_for_wall(
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    rectangle: utils::geometry::Rectangle2d,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    let storage = library.get_mut::<WoodenPlankSsbo>().expect("原矩形底层要求注册 WoodenPlankSsbo");
    let failures = {
        let mut writer = output.gpu.plank_writer(storage, id.0);
        construct_rect_wood_bottom_layer(id, rectangle, wall.authority.elevation(), |plank| writer.push(plank))
    };
    output.gpu.record_plank_failures(crate::gpu_output::ConstructorKey {
        wall: id.0, part: crate::gpu_output::ConstructorPart::ElevationSupports,
    }, failures);
}

/// 原矩形底层按墙材质分派到同一墙的木板或砖 owner。
pub(crate) fn construct_rect_bottom_layer(
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    rectangle: utils::geometry::Rectangle2d,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
) {
    if wall.building_projection().is_halftimbered() {
        construct_rect_wood_bottom_for_wall(id, wall, rectangle, library, output);
    } else {
        construct_rect_stone_bottom_for_wall(id, wall, rectangle, library, output);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rectangle_wood_bottom_matches_native_rotated_sequence() {
        let mut actual = Vec::new();
        super::construct_rect_wood_bottom_layer(country_core::resources::walls::WallId(77),
            utils::geometry::Rectangle2d::new(glam::Vec2::new(3.0,-2.0), glam::Vec2::new(4.0,6.0), glam::Vec2::new(0.6,0.8)),
            3.0, |plank| -> Result<(), ()> {
                for c in plank.transform.cols { actual.extend([c.x.to_bits(), c.y.to_bits(), c.z.to_bits()]); }
                actual.extend([plank.gate_seed as u32, plank.piece_seed as u32, plank.wall_id, plank.flags]); Ok(())
            });
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-wood-bottom-native-output.json");
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        assert_eq!(actual.len(), expected.len());
        for (i, (a,b)) in actual.iter().zip(&expected).enumerate() { assert_eq!(a,b,"plank {} field {}",i/16,i%16); }
    }
    #[test]
    fn rectangle_stone_bottom_matches_native_rotated_sequence() {
        let mut actual = Vec::new();
        super::construct_rect_stone_bottom_layer(country_core::resources::walls::WallId(77),
            utils::geometry::Rectangle2d::new(glam::Vec2::new(3.0,-2.0), glam::Vec2::new(4.0,6.0), glam::Vec2::new(0.6,0.8)),
            3.0, 14.0, |brick| -> Result<(), ()> {
                actual.extend(brick.observed_bytes().chunks_exact(4).map(|w| u32::from_le_bytes(w.try_into().unwrap()))); Ok(())
            });
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-bottom-native-output.json");
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        assert_eq!(actual.len(), expected.len());
        for (i, (a,b)) in actual.iter().zip(&expected).enumerate() { assert_eq!(a,b,"brick {} field {}",i/24,i%24); }
    }
}
