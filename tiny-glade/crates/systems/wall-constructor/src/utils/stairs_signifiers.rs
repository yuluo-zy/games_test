//! 墙体阶梯指示砖（B 0x141223d10）。

use country_core::geometry::instanced_wall::BrickTransformSsbo;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::{WallId, public_wall_state::PublicWallState, wall_holes::FinalizedWallHole};
use glam::Vec2;
use utils::curve::{approx_curve_coord, approx_curve_u, coord_at_cached_u};

/// 使用墙的已准备曲线、当前地形和局部最终化洞口，直接追加主砖数组。
pub fn stairs_signifiers_new(
    wall: &PublicWallState,
    id: WallId,
    out: &mut Vec<BrickTransformSsbo>,
    terrain: &TerrainHeightsData,
    holes: &[FinalizedWallHole],
) {
    use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
    use country_core::systems::wall::construction::parameters::brick_source_id;
    use utils::subtractive_1d_boolean::{Interval, SubtractiveBoolean1D};
    if wall.try_get_wall_shape().is_none() { return; }
    let path = wall.prepared_path.as_ref().expect("墙路径尚未准备");
    let outline = wall.simplified_curve_2d();
    let end = path.positions.len().checked_sub(1).expect("墙路径为空");
    let mut boundaries = vec![0];
    if let Some(rect) = wall.is_rectangle() {
        for i in 1..=2 {
            boundaries.push(approx_curve_coord(&path.positions, rect.get_point(i),
                |p| Vec2::new(p[0], p[2])).segment);
        }
    }
    boundaries.push(end);
    let mut active = Vec::new();
    for pair in boundaries.windows(2) {
        let start = pair[0]; let end = pair[1];
        let exclusive_end = end.checked_add(1).expect("墙路径范围溢出");
        assert!(start <= exclusive_end && exclusive_end <= path.positions.len());
        let mut run = None;
        for index in start..exclusive_end {
            let p = path.positions[index];
            let valid = p[1] - terrain.sample_at_world_xz(p[0], p[2]) < 0.5
                && wall.max_y() - p[1].max(0.55) < 3.0;
            if valid && run.is_none() { run = Some(index.saturating_sub(1)); }
            if !valid { if let Some(first) = run.take() { active.push((first, index)); } }
        }
        if let Some(first) = run { active.push((first, end)); }
    }
    if active.is_empty() { return; }
    let top = wall.max_y() - 0.79 - 0.1 - 0.3;
    let cuts: Vec<_> = holes.iter().filter(|h| h.origin == 0 &&
        (matches!(h.kind, 1 | 3 | 4) || (h.kind == 0 && h.aabb[1] <= top && top < h.aabb[3])))
        .map(|h| Interval { start: h.aabb[0] / outline.length, end: h.aabb[2] / outline.length, upper_exclusive: false }).collect();
    for (first, last) in active {
        let start = path.points_u[first]; let end = path.points_u[last];
        if !((end - start) * path.length >= 0.5) { continue; }
        let convert = |u: f32| if u == 0.0 || u == 1.0 { u } else {
            let coord = coord_at_cached_u(path.positions.len(), &path.points_u, u);
            let a = path.positions[coord.segment];
            let b = path.positions[coord.segment + 1];
            let t = coord.t;
            let p = Vec2::new(t*b[0] + (1.0-t)*a[0], t*b[2] + (1.0-t)*a[2]);
            approx_curve_u(&outline.points, &outline.points_u, p, |point| Vec2::from_array(*point))
        };
        let mut ranges = SubtractiveBoolean1D::new_range(convert(start), convert(end));
        for cut in &cuts { ranges.subtract(*cut); }
        for range in ranges.result() {
            let length = (range.end - range.start) * outline.length;
            let count = ((length / 0.69).ceil() as usize).max(1).wrapping_add(1);
            let rng = fastrand::Rng::with_seed(((wall.max_y() + length) * 1000.0) as u64);
            let mut splits = Vec::new();
            utils::random_splits_into(count, 0.32 / length, &rng, &mut splits);
            let boundaries: Vec<_> = splits.iter().map(|t| t * range.end + (1.0 - t) * range.start).collect();
            for (ordinal, pair) in boundaries.windows(2).enumerate() {
                if !(path.length * (pair[1] - pair[0]) >= 0.1) { continue; }
                assert!((0.0..=1.0).contains(&pair[0]) && (0.0..=1.0).contains(&pair[1]));
                let a = Vec2::from_array(outline.pos_at_u(pair[0]));
                let b = Vec2::from_array(outline.pos_at_u(pair[1]));
                let diff = b - a;
                let inv = 1.0 / (diff.y * diff.y + diff.x * diff.x).sqrt();
                let normal = Vec2::new(-diff.y * inv, diff.x * inv);
                let rotation = utils::xz_rotation_bad(normal);
                let axes = glam::Mat3::from_quat(rotation);
                let middle = a * 0.5 + b * 0.5;
                let width = (diff.y * diff.y + diff.x * diff.x).sqrt();
                let half = (pair[1] - pair[0]) * 0.5;
                let center = pair[1] * 0.5 + pair[0] * 0.5;
                for (xscale, yscale, zscale, drop) in [(width,0.2,0.3,0.0),(width*0.4,0.3,0.2,-0.25)] {
                    out.push(BrickTransformSsbo {
                        affine_columns: [(axes.x_axis*xscale).to_array(),(axes.y_axis*yscale).to_array(),(axes.z_axis*zscale).to_array(),
                            [middle.x+normal.x*0.3, wall.max_y()-0.79+drop, middle.y+normal.y*0.3]],
                        deformation:[0.0;4],surface:[-100.0;3],seed:ordinal as u32,
                        material:brick_source_id(ObservedWallId(id.0)),wall_distance:[(center-half)*length,(center+half)*length],flags:4,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::{roofs::geometry::surface::Curve2, walls::{inner_wall_state::WallState,
        publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512}};
    use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;

    #[test]
    fn original_records_cover_rectangle_hole_and_slope() {
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut terrain = TerrainHeightsData::new();
        terrain.set_height_range(0.0, 0.0);
        for name in ["circle", "hole", "holes", "slope", "rectangle", "raised", "pattern", "nan-min", "nan-max", "reversed"] {
            if name == "raised" || name == "pattern" {
                terrain.set_height_range(0.0, 1.0);
                for (i, v) in terrain.heights_mut().iter_mut().enumerate() {
                    *v = if name == "raised" || i % 512 < 260 { 65535 } else { 0 };
                }
            } else {
                terrain.set_height_range(0.0, 0.0);
            }
            let rectangle = name == "rectangle";
            let points = if rectangle { vec![[0.,0.],[0.,3.],[3.,3.],[3.,0.],[0.,0.]] }
                else { vec![[0.,0.],[3.,0.],[3.,3.]] };
            let source = Curve2::try_new(points, false).unwrap();
            let mut snapshot = WallState::new(source,3.0,0,0,false).snapshot().clone();
            let mut shape = [0u8;28];
            if rectangle {
                shape[..4].copy_from_slice(&1u32.to_le_bytes());
                for (i,v) in [1f32,0.,1.5,1.5,3.,3.].iter().enumerate() {
                    shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
                }
            }
            snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
            let mut wall = PublicWallState::collect(&WallState::from_snapshot(snapshot,0),&grid).unwrap();
            let positions = if rectangle { vec![[0.,0.,0.],[0.,0.,3.],[3.,0.,3.],[3.,0.,0.],[0.,0.,0.]] }
                else if name == "raised" || name == "pattern" { vec![[0.,0.6,0.],[3.,0.6,0.],[3.,0.6,3.]] }
                else { vec![[0.,0.,0.],[3.,if name=="slope" {1.} else {0.},0.],[3.,0.,3.]] };
            let points_u = if rectangle {vec![0.,0.25,0.5,0.75,1.]} else {vec![0.,0.5,1.]};
            let length = if rectangle {12.} else if name=="slope" {10f32.sqrt()*2.} else {6.};
            wall.prepared_path = Some(PreparedWallPath { segment_values: vec![0.;positions.len()], positions, points_u, length });
            wall.regular_heights = Some(country_core::resources::walls::height_cache::RegularWallHeightCache {
                maximum_y:if name=="raised" || name=="pattern" {3.5} else {3.}, arch_segment_heights:vec![3.;3] });
            let holes = if name=="hole" { vec![FinalizedWallHole {aabb:[2.5,1.,3.5,2.5],kind:1,origin:0}] }
                else if name=="holes" { vec![FinalizedWallHole {aabb:[1.,1.,2.,2.5],kind:1,origin:0}, FinalizedWallHole {aabb:[4.,1.,5.,2.5],kind:3,origin:0}] }
                else if name=="nan-min" { vec![FinalizedWallHole {aabb:[f32::NAN,1.,3.5,2.5],kind:1,origin:0}] }
                else if name=="nan-max" { vec![FinalizedWallHole {aabb:[2.5,1.,f32::NAN,2.5],kind:1,origin:0}] }
                else if name=="reversed" { vec![FinalizedWallHole {aabb:[3.5,1.,2.5,2.5],kind:1,origin:0}] }
                else {vec![]};
            let mut out=Vec::new();
            stairs_signifiers_new(&wall,WallId(17),&mut out,&terrain,&holes);
            let observed: Vec<_> = out.into_iter().flat_map(|b| b.observed_bytes()).collect();
            let fixture = match name {
                "circle" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-circle.out")),
                "hole" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-hole.out")),
                "holes" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-holes.out")),
                "slope" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-slope.out")),
                "raised" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-terrain-raised.out")),
                "pattern" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-terrain-pattern.out")),
                "nan-min" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-nan-min.out")),
                "nan-max" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-nan-max.out")),
                "reversed" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-reversed.out")),
                _ => include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m06-stairs-entry-rectangle.out")),
            };
            let hex=fixture.lines().nth(1).unwrap();
            let expected: Vec<_> = hex.as_bytes().chunks_exact(2).map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect();
            assert_eq!(observed, expected, "原入口样本 {name}");
        }
    }
}
