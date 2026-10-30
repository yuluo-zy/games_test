//! 支柱构造工具（util_pillar_construction.rs / util_cross_brick_pillar.rs）。
//!
//! `construct_wooden_pillar_w_stone_base`（0x141fe2150/0x141fe3240）：
//! 木柱 + 石基（0.2/1.2/0.01/0.5/±0.2/2.1/±0.25/0.56/1.26 常量族）。
//! `construct_crossbrick_pillar`（0x142418f50，15.9KB fresh）：十字砖柱
//! （closure env<construct_wooden_pillar_w_stone_base>）。
//!
//! 2026-10-17 复核：旧实现（0.56 等分 + ±0.2 十字）为**近似体**，已移除。
//! 真实逻辑（fresh 已取）含：高度钳 1.1、`len>1.25` 石基、0.3 斜撑、
//! 0x18B 记录二路；`construct_brick_columns` 走 FrozenMap(0x3a)→writer→
//! 0x40B 表；`construct_crossbrick_pillar` 的 count=`ceil((top-bottom)/
//! (size*0.4))` + `random_splits` + 四边 + `xz_rotation_bad`。
use glam::Vec3;

/// 原构造器内部柱提案的安全业务投影；不复制 64 字节私有 ABI。
#[derive(Clone, Debug)]
pub struct PillarProposal {
    pub center: glam::Vec2,
    pub index: usize,
    pub axis: glam::Vec2,
    pub adjacent_directions: [glam::Vec2; 2],
    pub bottom: f32,
    pub top: f32,
    pub size: f32,
    pub adjacent_distances: [f32; 2],
}

impl PillarProposal {
    /// 原矩形柱把提案转换为碰撞矩形：水平尺寸为柱宽的一半，
    /// 竖直中心由上下端各乘一半相加后交给 Rectangle3d 构造。
    pub(crate) fn stone_column_bounds(&self) -> utils::geometry::Rectangle3d {
        utils::geometry::Rectangle3d::new(
            Vec3::new(self.center.x, self.top * 0.5 + self.bottom * 0.5, self.center.y),
            Vec3::new(self.size * 0.5, self.top - self.bottom, self.size * 0.5), self.axis,
        )
    }
}

/// 木柱主体的原仿射记录；斜撑与石基另由同一原调用处理。
pub(crate) fn wooden_pillar_trunk(
    proposal: &PillarProposal, wall: country_core::resources::walls::WallId, index: usize,
) -> country_core::resources::roofs::roof_buffers::WoodenPlankSsbo {
    use country_core::resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo};
    let q = utils::xz_rotation_bad(proposal.axis);
    let zz = q.z + q.z; let yy = q.y + q.y; let xx = q.x + q.x;
    let width = if 1.1 <= proposal.size { 1.1 } else { proposal.size } * 0.5;
    let height = (proposal.top - proposal.bottom) + 0.2;
    let columns = [
        Vec3::new(width * (1.0 - (q.y * yy + q.z * zz)), width * (yy * q.x + q.w * zz), width * (zz * q.x - yy * q.w)),
        Vec3::new(height * (yy * q.x - q.w * zz), height * (1.0 - (q.z * zz + q.x * xx)), height * (xx * q.w + zz * q.y)),
        Vec3::new(width * (zz * q.x + yy * q.w), width * (zz * q.y - xx * q.w), width * (1.0 - (q.x * xx + q.y * yy))),
    ];
    WoodenPlankSsbo { transform: Affine3Packed::from_cols(columns[0], columns[1], columns[2],
        Vec3::new(proposal.center.x, proposal.bottom, proposal.center.y)), gate_seed: 0,
        piece_seed: (wall.0 as u32).wrapping_add(index as u32) as i32, wall_id: wall.0 as u32, flags: 0 }
}

/// 按原顺序先写全部主体及斜撑，再写全部石基；两个 writer 的持锁由调用方负责。
pub(crate) fn construct_wooden_planks<P>(
    wall: country_core::resources::walls::WallId, proposals: &[PillarProposal], count: &mut u64,
    mut push_plank: impl FnMut(&country_core::resources::roofs::roof_buffers::WoodenPlankSsbo) -> Result<(), P>,
) -> usize {
    use country_core::resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo};
    let mut plank_failures = 0;
    for (index, p) in proposals.iter().enumerate() {
        if push_plank(&wooden_pillar_trunk(p, wall, index)).is_err() { plank_failures += 1; }
        *count = count.wrapping_add(1);
        let height = p.top - p.bottom;
        if 1.25 < height {
            for side in 0..2 {
                let raw = (height - 1.25) / (3.0 - 1.25);
                let t = if 0.0 <= raw { raw } else { 0.0 };
                let t = if t <= 1.0 { t } else { 1.0 };
                let low = p.top - ((1.0 - t) * 0.5 + t);
                let low = if low.is_nan() { p.bottom } else if p.bottom <= low { low } else { p.bottom };
                let distance = p.adjacent_distances[side] * 0.25;
                let distance = if p.size.is_nan() { distance } else if p.size <= distance { p.size } else { distance };
                let begin = Vec3::new(p.center.x, low, p.center.y);
                let end = Vec3::new(p.center.x + p.adjacent_directions[side].x * distance,
                    p.top + 0.2, p.center.y + p.adjacent_directions[side].y * distance);
                let delta = end - begin;
                let length = (delta.y * delta.y + delta.x * delta.x + delta.z * delta.z).sqrt();
                let inverse = 1.0 / length;
                let q = utils::rotation_from_normal(Vec3::new(delta.x * inverse,delta.y * inverse,delta.z * inverse));
                let zz = q.z + q.z; let yy = q.y + q.y; let xx = q.x + q.x;
                let columns = [
                    Vec3::new(0.3 * (1.0 - (q.y * yy + q.z * zz)), 0.3 * (yy * q.x + q.w * zz), 0.3 * (zz * q.x - yy * q.w)),
                    Vec3::new(0.3 * (yy * q.x - q.w * zz), 0.3 * (1.0 - (q.z * zz + q.x * xx)), 0.3 * (xx * q.w + zz * q.y)),
                    Vec3::new(length * (zz * q.x + yy * q.w), length * (zz * q.y - xx * q.w), length * (1.0 - (q.x * xx + q.y * yy))),
                ];
                let center = Vec3::new((begin.x + end.x) * 0.5, (begin.y + end.y) * 0.5, (begin.z + end.z) * 0.5);
                let plank = WoodenPlankSsbo { transform: Affine3Packed::from_cols(columns[0],columns[1],columns[2],center),
                    gate_seed: 0, piece_seed: (wall.0 as u32).wrapping_add(index as u32) as i32, wall_id: wall.0 as u32, flags: 0 };
                if push_plank(&plank).is_err() { plank_failures += 1; }
            }
        }
    }
    plank_failures
}

pub(crate) fn construct_wooden_stone_bases<B>(wall: country_core::resources::walls::WallId, proposals: &[PillarProposal],
    rng: &fastrand::Rng, mut push_brick: impl FnMut(&country_core::geometry::instanced_wall::BrickTransformSsbo) -> Result<(), B>,
) -> usize {
    let mut brick_failures = 0;
    for p in proposals {
        let width = if p.size <= 1.3 { p.size } else { 1.3 } - 0.5;
        let top = ((p.size * p.size) * p.size) * 0.5 + p.bottom;
        brick_failures += construct_crossbrick_pillar(width, p.center, top, p.bottom, rng, p.axis, wall, true, &mut push_brick);
    }
    brick_failures
}

/// 原木柱先持木板锁完成所有木柱，再释放并持砖锁生成各石基。
pub(crate) fn construct_wooden_pillar_w_stone_base(
    wall: country_core::resources::walls::WallId, proposals: &[PillarProposal], rng: &fastrand::Rng,
    count: &mut u64, library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
) {
    use country_core::{geometry::instanced_wall::BrickTransformSsbo, resources::roofs::roof_buffers::WoodenPlankSsbo};
    let plank_failures = {
        let storage = library.get_mut::<WoodenPlankSsbo>().expect("原木柱要求 WoodenPlankSsbo");
        let mut writer = output.gpu.plank_writer(storage, wall.0);
        construct_wooden_planks(wall, proposals, count, |plank| writer.push(plank))
    };
    let key = crate::gpu_output::ConstructorKey { wall: wall.0, part: crate::gpu_output::ConstructorPart::ElevationSupports };
    output.gpu.record_plank_failures(key, plank_failures);
    if proposals.is_empty() { return; }
    let brick_failures = {
        let storage = library.get_mut::<BrickTransformSsbo>().expect("原木柱石基要求 BrickTransformSsbo");
        let mut writer = output.gpu.brick_writer(storage, wall.0);
        construct_wooden_stone_bases(wall, proposals, rng, |brick| writer.push(brick))
    };
    output.gpu.record_brick_failures(key, brick_failures);
}

#[cfg(test)]
fn construct_wooden_pillars<P,B>(wall: country_core::resources::walls::WallId, proposals: &[PillarProposal],
    rng: &fastrand::Rng, count: &mut u64, push_plank: impl FnMut(&country_core::resources::roofs::roof_buffers::WoodenPlankSsbo) -> Result<(),P>,
    push_brick: impl FnMut(&country_core::geometry::instanced_wall::BrickTransformSsbo) -> Result<(),B>) -> (usize,usize) {
    (construct_wooden_planks(wall,proposals,count,push_plank), construct_wooden_stone_bases(wall,proposals,rng,push_brick))
}

#[cfg(test)]
mod wooden_trunk_tests {
    fn check_full(top: f32, plank_fixture: &str, brick_fixture: &str) {
        use super::*;
        let raw = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-pillar-rotated-native-output.json");
        let words: Vec<u32> = raw.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        let f = |i| f32::from_bits(words[i]);
        let p = PillarProposal { center: glam::Vec2::new(f(0),f(1)), index: 0, axis: glam::Vec2::new(f(4),f(5)),
            adjacent_directions: [glam::Vec2::new(f(6),f(7)),glam::Vec2::new(f(8),f(9))],
            bottom: f(10), top, size: f(12), adjacent_distances: [f(13),f(14)] };
        let mut planks = Vec::new(); let mut bricks = Vec::new(); let mut count = 0;
        let failures = construct_wooden_pillars(country_core::resources::walls::WallId(123), &[p], &fastrand::Rng::with_seed(77), &mut count,
            |p| -> Result<(), ()> { for c in p.transform.cols { planks.extend([c.x.to_bits(),c.y.to_bits(),c.z.to_bits()]); }
                planks.extend([p.gate_seed as u32,p.piece_seed as u32,p.wall_id,p.flags]); Ok(()) },
            |b| -> Result<(), ()> { bricks.extend(b.observed_bytes().chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap()))); Ok(()) });
        assert_eq!(failures, (0,0)); assert_eq!(count, 1);
        for (actual, fixture, stride) in [(&planks,plank_fixture,16),(&bricks,brick_fixture,24)] {
            let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v|v.parse().unwrap()).collect();
            assert_eq!(actual.len(),expected.len());
            for (i,(a,b)) in actual.iter().zip(&expected).enumerate() { assert_eq!(a,b,"record {} field {}",i/stride,i%stride); }
        }
    }
    #[test]
    fn wood_pillar_and_stone_base_match_native() {
        check_full(4.0, include_str!("../../../../../reports/module-recovery-2026-10-23/m06-wood-pillar-native-output.json"),
            include_str!("../../../../../reports/module-recovery-2026-10-23/m06-wood-pillar-bricks-native-output.json"));
    }
    #[test]
    fn short_wood_pillar_and_stone_base_match_native() {
        check_full(1.0, include_str!("../../../../../reports/module-recovery-2026-10-23/m06-wood-pillar-short-native-output.json"),
            include_str!("../../../../../reports/module-recovery-2026-10-23/m06-wood-pillar-short-bricks-native-output.json"));
    }
    #[test]
    fn short_wooden_trunk_matches_native() {
        use super::*;
        let raw = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-pillar-rotated-native-output.json");
        let words: Vec<u32> = raw.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        let f = |i| f32::from_bits(words[i]);
        let p = PillarProposal { center: glam::Vec2::new(f(0),f(1)), index: 0, axis: glam::Vec2::new(f(4),f(5)),
            adjacent_directions: [glam::Vec2::new(f(6),f(7)),glam::Vec2::new(f(8),f(9))],
            bottom: f(10), top: 1.0, size: f(12), adjacent_distances: [f(13),f(14)] };
        let plank = wooden_pillar_trunk(&p, country_core::resources::walls::WallId(123), 0);
        let mut actual = Vec::new();
        for c in plank.transform.cols { actual.extend([c.x.to_bits(),c.y.to_bits(),c.z.to_bits()]); }
        actual.extend([plank.gate_seed as u32,plank.piece_seed as u32,plank.wall_id,plank.flags]);
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-wood-pillar-short-native-output.json");
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        for (i,(a,b)) in actual.iter().zip(&expected).enumerate() { assert_eq!(a,b,"field {i}"); }
        assert_eq!(actual.len(), expected.len());
    }
}

/// 原 0x141fe3240：同一墙砖 writer 锁内依次构造每一根十字砖柱。
/// 原计数在每根柱子完整返回后递增；容量失败保留成功前缀并继续。
pub fn construct_brick_columns(
    wall: country_core::resources::walls::WallId,
    proposals: &[PillarProposal],
    rng: &fastrand::Rng,
    count: &mut u64,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
) {
    use country_core::geometry::instanced_wall::BrickTransformSsbo;
    let storage = library.get_mut::<BrickTransformSsbo>().expect("原砖柱要求注册 BrickTransformSsbo");
    let mut failures = 0usize;
    {
        let mut writer = output.gpu.brick_writer(storage, wall.0);
        for proposal in proposals {
            failures += construct_crossbrick_pillar(proposal.size, proposal.center, proposal.top,
                proposal.bottom, rng, proposal.axis, wall, false, |brick| writer.push(brick));
            *count = count.wrapping_add(1);
        }
    }
    output.gpu.record_brick_failures(crate::gpu_output::ConstructorKey {
        wall: wall.0, part: crate::gpu_output::ConstructorPart::ElevationSupports,
    }, failures);
}

/// 原 0x142418f50：沿柱高随机分层，每层四边逐砖写入。
/// 调用方持有同一 writer，失败只计数，不跳过后续砖。
pub fn construct_crossbrick_pillar<E>(
    size: f32, center: glam::Vec2, top: f32, bottom: f32,
    rng: &fastrand::Rng, axis: glam::Vec2, wall: country_core::resources::walls::WallId,
    upper_only: bool, mut push: impl FnMut(&country_core::geometry::instanced_wall::BrickTransformSsbo) -> Result<(), E>,
) -> usize {
    use country_core::geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo};
    let height = top - bottom;
    let count = ((height / (0.4 * size)).ceil() as usize).max(3);
    let splits = utils::random_splits(count, (0.2 * size) / height, rng);
    let heights: Vec<_> = splits.iter().map(|&u| u * top + (1.0 - u) * bottom).collect();
    let steps = (heights.len() - 2) as f32;
    let smooth = |v: f32| {
        let v = if 0.0 <= v { v } else { 0.0 };
        let v = if v <= 1.0 { v } else { 1.0 };
        (3.0 - (v + v)) * (v * v)
    };
    let mut failures = 0;
    let mut distance = 0.0f32;
    for (layer, pair) in heights.windows(2).enumerate() {
        let progress = layer as f32 / steps;
        let bottom_weight = if upper_only { 0.0 } else { smooth((progress - 0.85) / f32::from_bits(0x3e199998)) };
        let upper = if upper_only { 1.0 - smooth((progress - 0.8) / f32::from_bits(0x3dccccc8)) }
                    else { 1.0 - smooth((progress - 0.1) / 0.1) };
        let weight = if bottom_weight.is_nan() { upper } else if upper <= bottom_weight { bottom_weight } else { upper };
        let width = (weight * 0.3 + 0.3) * size;
        let rectangle = utils::geometry::Rectangle2d::new(center, glam::Vec2::splat(width), axis);
        let start = pair[0]; let end = pair[1];
        let layer_mid = (start + end) * 0.5;
        let dx = center.x - center.x;
        let dz = center.y - center.y;
        let dy = end - start;
        let layer_height = (dz * dz + dy * dy + dx * dx).sqrt();
        let sign = if layer % 2 == 0 { -1.0 } else { 1.0 };
        for edge in 0..4 {
            let p0 = rectangle.get_point(edge);
            let p1 = rectangle.get_point((edge + 1) & 3);
            let delta = p1 - p0;
            let inverse = 1.0 / (delta.y * delta.y + delta.x * delta.x).sqrt();
            let shift = glam::Vec2::new(inverse * delta.x * 0.4 * 0.5 * sign,
                inverse * delta.y * 0.4 * 0.5 * sign);
            let a = p0 + shift; let b = p1 + shift;
            let middle = glam::Vec2::new(b.x * 0.5 + a.x * 0.5, b.y * 0.5 + a.y * 0.5);
            let segment = a - b;
            let length = (segment.y * segment.y + segment.x * segment.x).sqrt();
            let sample = rng.f32();
            let jitter = 0.05 * size;
            let depth = (sample * jitter - (1.0 - sample) * jitter) + 0.4;
            let q = utils::xz_rotation_bad(rectangle.get_edge_normal(edge));
            let x2 = q.x + q.x; let y2 = q.y + q.y; let z2 = q.z + q.z;
            let zz = q.z * z2; let xy = y2 * q.x; let xz = z2 * q.x;
            let yz = z2 * q.y; let xx = q.x * x2; let wx = x2 * q.w;
            let wy = y2 * q.w; let wz = q.w * z2;
            let new_distance = length + distance;
            let record = BrickTransformSsbo {
                affine_columns: [
                    [length * (1.0 - (q.y * y2 + zz)), length * (xy + wz), length * (xz - wy)],
                    [layer_height * (xy - wz), layer_height * (1.0 - (zz + xx)), layer_height * (wx + yz)],
                    [depth * (xz + wy), depth * (yz - wx), depth * (1.0 - (xx + q.y * y2))],
                    [middle.x, layer_mid, middle.y],
                ],
                seed: (wall.0 as u32).wrapping_add(edge as u32).wrapping_add(layer as u32),
                material: BrickSourceId::from_wall_id(country_core::resources::walls::publication_fields::WallId(wall.0)).0,
                wall_distance: [distance, new_distance],
                ..Default::default()
            };
            if push(&record).is_err() { failures += 1; }
            distance = new_distance;
        }
    }
    failures
}

#[cfg(test)]
mod native_tests {
    use super::*;
    #[test]
    fn offset_circle_crossbrick_edge_matches_native() {
        let rect=utils::geometry::Rectangle2d::new(glam::Vec2::new(f32::from_bits(0x40cd8123),f32::from_bits(0xbf2c37cf)),
            glam::Vec2::splat(f32::from_bits(0x3f1c274f)),glam::Vec2::new(f32::from_bits(0xbf6ead00),f32::from_bits(0xbeb925aa)));
        let p0=rect.get_point(0);let p1=rect.get_point(1);let d=p1-p0;
        let inverse=1.0/(d.y*d.y+d.x*d.x).sqrt();
        let shift=glam::Vec2::new(inverse*d.x*0.4*0.5,inverse*d.y*0.4*0.5);
        let a=p0+shift;let b=p1+shift;
        assert_eq!([a.x.to_bits(),a.y.to_bits(),b.x.to_bits(),b.y.to_bits()],
            [0x40d56382,0xbeedda42,0x40dc7278,0xbf84419a]);
    }
    #[test]
    fn offset_circle_crossbrick_rotation_matches_native() {
        let q=utils::xz_rotation_bad(glam::Vec2::new(f32::from_bits(0x3f6ead00),f32::from_bits(0x3eb925aa)));
        assert_eq!(q.to_array().map(f32::to_bits),[0,0x3f10a1f2,0,0x3f533a76]);
    }
    #[test]
    fn rotated_upper_crossbrick_matches_original_full_sequence() {
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-crossbrick-rotated-native-output.json");
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        let mut actual = Vec::new();
        let rng = fastrand::Rng::with_seed(123);
        let failures = construct_crossbrick_pillar(1.0, glam::Vec2::new(2.0,-3.0), 5.0, 1.0, &rng, glam::Vec2::new(0.6,0.8),
            country_core::resources::walls::WallId(77), true, |brick| -> Result<(), ()> {
                actual.extend(brick.observed_bytes().chunks_exact(4).map(|word| u32::from_le_bytes(word.try_into().unwrap())));
                Ok(())
            });
        assert_eq!(failures, 0);
        assert_eq!(actual.len(), expected.len());
        for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
            assert_eq!(actual, expected, "brick {} word {}", index / 24, index % 24);
        }
    }
    #[test]
    fn crossbrick_matches_original_full_sequence() {
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-crossbrick-native-output.json");
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        let mut actual = Vec::new();
        let rng = fastrand::Rng::with_seed(123);
        let failures = construct_crossbrick_pillar(1.0, glam::Vec2::ZERO, 5.0, 1.0, &rng, glam::Vec2::X,
            country_core::resources::walls::WallId(77), false, |brick| -> Result<(), ()> {
                actual.extend(brick.observed_bytes().chunks_exact(4).map(|word| u32::from_le_bytes(word.try_into().unwrap())));
                Ok(())
            });
        assert_eq!(failures, 0);
        assert_eq!(actual.len(), expected.len());
        for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
            assert_eq!(actual, expected, "brick {} word {}", index / 24, index % 24);
        }
    }
}
