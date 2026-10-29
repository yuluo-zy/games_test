//! B 0x1408eae40/0x1408e9ed0: wall quads and cached default collision triangles.
//! Quad is 120 meaningful bytes, not the old report's guessed 96 bytes. The
//! corner order is [bottom/start, bottom/end, top/start, top/end]. Freehand
//! second faces reverse bottom/top so the two faces have opposite winding.
use super::public_wall_state::PublicWallState;
use crate::systems::wall::wall_state::pending_changed::approximate_curve_u;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::{curve::Curve3, geometry::intersect::Triangle};

#[derive(Clone, Debug)]
pub struct Quad {
    pub wall_uv: [Vec2; 4],
    pub positions: [Vec3; 4],
    pub prepared_u: [f32; 2],
    pub endpoint_normals: [Vec3; 2],
    pub segment_heights: [f32; 2],
}

/// 原 B `WallTriggerVolumeParams` 的拥有型参数；范围、法线偏移均有独立存在位。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallTriggerVolumeParams {
    pub u_range: Option<[f32; 2]>,
    pub normal_offset: Option<f32>,
    pub minimum_height: f32,
    pub offset_from_top: f32,
    pub apply: bool,
}
impl WallTriggerVolumeParams {
    /// B 14165de44..14165de90 的内联 edit_shape_default + 顶部偏移构造。
    pub const fn edit_shape_default() -> Self {
        Self {
            u_range: None,
            normal_offset: Some(f32::from_bits(0x3e93_3333)),
            minimum_height: 0.0,
            offset_from_top: -0.5,
            apply: true,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallTriggerError {
    MissingPreparedPath,
    MissingHeightCache,
    InvalidCurve,
    ArrayCount,
    Projection,
}

/// 原get_collision_geo返回的三组独立owned数组；法线与UV均逐三角、逐角点。
/// 这是现有Rust所有权表达，不把Vec布局当作原72B私有ABI。
#[derive(Debug)]
pub struct WallTriggerCollisionGeo {
    pub triangles: Vec<Triangle>,
    pub normals: Vec<[Vec3; 3]>,
    pub uvs: Vec<[Vec2; 3]>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct WallTriggerVolume {
    pub quads: Vec<Quad>,
    triangles: Vec<Triangle>,
    normals: Vec<[Vec3; 3]>,
    uvs: Vec<[Vec2; 3]>,
    special: u8,
}
impl WallTriggerVolume {
    pub fn is_valid(&self) -> bool {
        !self.quads.is_empty()
    }
    pub fn default_tris(&self) -> &[Triangle] {
        &self.triangles
    }

    /// B 1408e80f0：逐quad应用高度、法线偏移及u裁剪，按原绕序生成两三角形。
    /// 与default_tris不同，此入口为编辑手柄重新构造窄化触发体，不能返回缓存替代。
    pub fn get_tris(&self, params: WallTriggerVolumeParams) -> Vec<Triangle> {
        let mut triangles = Vec::with_capacity(self.quads.len() * 2);
        self.get_collision_geo_callback(params, |triangle, _, _| triangles.push(triangle));
        triangles
    }

    /// 原1408e8d70：每次从同一quads与参数重建碰撞面，不读取default_tris缓存。
    /// 原裁剪只改变顶点，逐角点法线/UV仍按原quad索引复制，不另做插值或归一化。
    pub fn get_collision_geo(&self, params: WallTriggerVolumeParams) -> WallTriggerCollisionGeo {
        let capacity = self.quads.len() * 2;
        let mut geo = WallTriggerCollisionGeo {
            triangles: Vec::with_capacity(capacity), normals: Vec::with_capacity(capacity), uvs: Vec::with_capacity(capacity),
        };
        self.get_collision_geo_callback(params, |triangle, normals, uv| {
            geo.triangles.push(triangle); geo.normals.push(normals); geo.uvs.push(uv);
        });
        geo
    }

    /// 原B get_collision_geo_callback共享几何分支；编辑三角面与物理面复用同一算法。
    fn get_collision_geo_callback(&self, params: WallTriggerVolumeParams,
        mut emit: impl FnMut(Triangle, [Vec3; 3], [Vec2; 3])) {
        assert!(!self.quads.is_empty(), "墙触发体的quad不能为空");
        let mut emitted = false;
        for quad in &self.quads {
            let Some(mut p) = collision_quad_positions::<false>(quad, self.special, params) else { continue; };
            // 常规有限坐标保留直接浮点快速路径。仅非有限结果重演原SSE指令语义；
            // 新生成的invalid NaN必须为原x86负quiet NaN，输入NaN的payload及顺序仍保留。
            if params.apply && p.iter().any(|point| !point.is_finite()) {
                p = collision_quad_positions::<true>(quad, self.special, params).unwrap();
            }
            for indices in [[0, 1, 2], [1, 3, 2]] {
                emit(Triangle::new(p[indices[0]], p[indices[1]], p[indices[2]]),
                    indices.map(|i| quad.endpoint_normals[i & 1]), indices.map(|i| quad.wall_uv[i]));
            }
            emitted = true;
        }
        assert!(emitted, "墙触发体三角形不能为空");
    }
    pub fn triangle_normals(&self) -> &[[Vec3; 3]] {
        &self.normals
    }
    pub fn triangle_uvs(&self) -> &[[Vec2; 3]] {
        &self.uvs
    }

    pub fn recompute(&mut self, wall: &PublicWallState) -> Result<(), WallTriggerError> {
        let prepared = wall
            .prepared_path
            .as_ref()
            .ok_or(WallTriggerError::MissingPreparedPath)?;
        let heights = wall
            .arch_segment_heights()
            .ok_or(WallTriggerError::MissingHeightCache)?;
        let count = prepared
            .positions
            .len()
            .min(prepared.points_u.len())
            .min(heights.len())
            .min(wall.segment_heights.len());
        if count < 2 {
            return Err(WallTriggerError::ArrayCount);
        }
        let mut curve = Curve3::try_new(
            prepared
                .positions
                .iter()
                .copied()
                .map(Vec3::from_array)
                .collect(),
            false,
        )
        .map_err(|_| WallTriggerError::InvalidCurve)?;
        curve.points_u.clone_from(&prepared.points_u);
        curve.length = prepared.length;
        let tag = wall.shape().tag();
        let endpoint = |i: usize| -> Result<Endpoint, WallTriggerError> {
            let u = prepared.points_u[i];
            let c = curve.get_coord_at_u(u);
            let d = curve.points[c.segment + 1] - curve.points[c.segment];
            // 1408e7800..784a normalizes full XYZ, then projects/rotates XZ.
            // It does not renormalize after discarding the slope's Y component.
            let inverse = 1.0 / ((d.y * d.y + d.x * d.x) + d.z * d.z).sqrt();
            let normal = Vec3::new(-d.z * inverse, 0.0, d.x * inverse);
            let source = curve.points[i];
            let segment_height = wall.segment_heights[i];
            let bottom = Vec3::new(
                source.x + segment_height * 0.0,
                source.y + segment_height,
                source.z + segment_height * 0.0,
            );
            let top = Vec3::new(bottom.x, heights[i], bottom.z);
            let raw = wall.simplified_curve_2d();
            let horizontal = approximate_curve_u(raw, [bottom.x, bottom.z])
                .map_err(|_| WallTriggerError::Projection)?
                * raw.length;
            Ok(Endpoint {
                bottom,
                top,
                normal,
                u,
                horizontal,
                source_y: source.y,
                segment_height,
            })
        };
        let mut endpoints: Vec<_> = (0..count).map(endpoint).collect::<Result<_, _>>()?;
        // The original chains the first item for every shape tag except open 2.
        if tag != 2 {
            endpoints.push(endpoints[0]);
        }
        let mut quads = Vec::new();
        for (i, pair) in endpoints.windows(2).enumerate() {
            let [a, b] = [pair[0], pair[1]];
            let uv = [
                Vec2::new(a.horizontal, a.source_y),
                Vec2::new(b.horizontal, a.source_y),
                Vec2::new(a.horizontal, a.top.y),
                Vec2::new(b.horizontal, b.top.y),
            ];
            let mut push = |positions, normals, segment_heights, wall_uv| {
                quads.push(Quad {
                    positions,
                    endpoint_normals: normals,
                    segment_heights,
                    wall_uv,
                    prepared_u: [a.u, b.u],
                });
            };
            if tag < 2 {
                push(
                    [a.bottom, b.bottom, a.top, b.top],
                    [a.normal, b.normal],
                    [a.segment_height, b.segment_height],
                    uv,
                );
                continue;
            }
            // Two multiplies are retained from the original constant loads.
            let oa = (a.normal * 0.63) * 0.5;
            let ob = (b.normal * 0.63) * 0.5;
            let top_uv = [uv[2], uv[3], uv[2], uv[3]];
            if wall.authority.source_flag_65() == 1
                && (a.segment_height > 0.0 || b.segment_height > 0.0)
            {
                if a.segment_height != 0.0 && b.segment_height != 0.0 {
                    push(
                        [a.bottom + oa, b.bottom + ob, a.bottom - oa, b.bottom - ob],
                        [Vec3::Y; 2],
                        [0.0; 2],
                        top_uv,
                    );
                }
                continue;
            }
            push(
                [a.bottom + oa, b.bottom + ob, a.top + oa, b.top + ob],
                [a.normal, b.normal],
                [a.segment_height, b.segment_height],
                uv,
            );
            push(
                [a.top - oa, b.top - ob, a.bottom - oa, b.bottom - ob],
                [a.normal, b.normal],
                [a.segment_height, b.segment_height],
                uv,
            );
            push(
                [a.top + oa, b.top + ob, a.top - oa, b.top - ob],
                [Vec3::Y; 2],
                [0.0; 2],
                top_uv,
            );
            if tag == 2 {
                if i == 0 {
                    push(
                        [a.bottom + oa, a.top + oa, a.bottom - oa, a.top - oa],
                        [a.normal; 2],
                        [0.0; 2],
                        [uv[0], uv[2], uv[0], uv[2]],
                    );
                }
                if i == prepared.positions.len() - 2 {
                    let end_bottom = Vec2::new(b.horizontal, a.source_y);
                    push(
                        [b.bottom - ob, b.top - ob, b.bottom + ob, b.top + ob],
                        [b.normal; 2],
                        [0.0; 2],
                        [end_bottom, uv[3], end_bottom, uv[3]],
                    );
                }
            }
        }
        self.quads = quads;
        self.recompute_tris(tag < 2);
        self.special = wall.authority.source_flag_65();
        Ok(())
    }

    fn recompute_tris(&mut self, regular: bool) {
        self.triangles.clear();
        self.normals.clear();
        self.uvs.clear();
        for q in &self.quads {
            let mut p = q.positions;
            if regular {
                if q.prepared_u[0] == q.prepared_u[1] {
                    continue;
                }
                for i in 0..2 {
                    let h = q.segment_heights[i];
                    let positive = if h > 0.0 { h } else { 0.0 };
                    let offset = q.endpoint_normals[i] * 0.315;
                    p[i] = (p[i] - Vec3::Y * h) + Vec3::Y * positive + offset;
                    p[i + 2] += offset;
                    // Source recompute uses the volume's previous special byte,
                    // then stores the newly published byte at function exit.
                    if self.special & 1 != 0 && q.segment_heights[0] > 0.0 {
                        p[i + 2] += Vec3::Y * (q.segment_heights[0] - q.positions[i + 2].y);
                    }
                }
            }
            for indices in [[0, 1, 2], [1, 3, 2]] {
                self.triangles
                    .push(Triangle::new(p[indices[0]], p[indices[1]], p[indices[2]]));
                self.normals
                    .push(indices.map(|i| q.endpoint_normals[i & 1]));
                self.uvs.push(indices.map(|i| q.wall_uv[i]));
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Endpoint {
    bottom: Vec3,
    top: Vec3,
    normal: Vec3,
    u: f32,
    horizontal: f32,
    source_y: f32,
    segment_height: f32,
}

/// 保持原1408e93b0..1408e961b逐操作次序；常规实例化由编译器消去所有SSE兼容分支。
fn collision_quad_positions<const SSE: bool>(quad: &Quad, special: u8, params: WallTriggerVolumeParams) -> Option<[Vec3; 4]> {
    #[inline]
    fn original<const SSE: bool>(a: f32, b: f32, result: f32) -> f32 {
        if !SSE { return result; }
        if a.is_nan() { f32::from_bits(a.to_bits() | 0x0040_0000) }
        else if b.is_nan() { f32::from_bits(b.to_bits() | 0x0040_0000) }
        else if result.is_nan() { f32::from_bits(0xffc0_0000) }
        else { result }
    }
    let add = |a: f32, b: f32| original::<SSE>(a, b, a + b);
    let sub = |a: f32, b: f32| original::<SSE>(a, b, a - b);
    let mul = |a: f32, b: f32| original::<SSE>(a, b, a * b);
    let div = |a: f32, b: f32| original::<SSE>(a, b, a / b);
    let mut p = quad.positions;
    if !params.apply { return Some(p); }
    let [original_begin, original_end] = quad.prepared_u;
    let [begin, end] = params.u_range.map_or(quad.prepared_u, |[minimum, maximum]| {
        [original_begin.clamp(minimum, maximum), original_end.clamp(minimum, maximum)]
    });
    if params.u_range.is_some() {
        assert!((0.0..=1.0).contains(&begin) && (0.0..=1.0).contains(&end), "墙触发体u超出范围");
    }
    if begin == end { return None; }
    let offset = params.normal_offset.unwrap_or(0.0);
    for i in 0..2 {
        let height = quad.segment_heights[i];
        // 原MAXPS后blend使NaN高度选择参数下限；相等时保留高度的有符号零。
        let minimum = if height.is_nan() || params.minimum_height > height { params.minimum_height } else { height };
        let normal = quad.endpoint_normals[i].map(|v| mul(v, offset));
        p[i] = Vec3::new(
            add(add(mul(minimum, 0.0), sub(p[i].x, mul(height, 0.0))), normal.x),
            add(add(minimum, sub(p[i].y, height)), normal.y),
            add(add(sub(p[i].z, mul(height, 0.0)), mul(minimum, 0.0)), normal.z),
        );
        // 原特殊分支统一读取首端高度，第二端不改为自身高度。
        let top = if special & 1 != 0 && quad.segment_heights[0] > 0.0 {
            sub(quad.segment_heights[0], p[i + 2].y)
        } else { params.offset_from_top };
        p[i + 2] = Vec3::new(
            add(add(mul(top, 0.0), p[i + 2].x), normal.x),
            add(add(top, p[i + 2].y), normal.y),
            add(add(p[i + 2].z, mul(top, 0.0)), normal.z),
        );
    }
    if begin != original_begin || end != original_end {
        let first = div(sub(begin, original_begin), sub(original_end, original_begin));
        let last = div(sub(end, original_begin), sub(original_end, original_begin));
        let before = p;
        let interpolate = |a: Vec3, b: Vec3, t| {
            Vec3::from_array(std::array::from_fn(|i| add(mul(b[i], t), mul(a[i], sub(1.0, t)))))
        };
        p[0] = interpolate(before[0], before[1], first);
        p[1] = interpolate(before[0], before[1], last);
        p[2] = interpolate(before[2], before[3], first);
        p[3] = interpolate(before[2], before[3], last);
    }
    Some(p)
}

#[cfg(test)]
mod native_edit_trigger_tests {
    use super::*;

    #[cfg(feature = "native-physics")]
    #[test]
    fn prepared_complex_wall_collision_outputs_survive_edit_failure_and_release_actual_mesh() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512};
        use crate::systems::wall::wall_state::path_preparation::PathRasters512;
        use crate::systems::collision::world::{RaycastWorld, RaycastMeshData, RaycastGeoSource, upsert_collider, remove_collider, ColliderUpsert};
        use std::{collections::HashMap, sync::Arc};
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let paths = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0; 512 * 512])), false).unwrap();
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new());
        let entity = world.spawn(WallTriggerVolume::default()).id();
        let mut instances = HashMap::new();
        let mut previous_hash = None;
        let mut previous_weak: Option<std::sync::Weak<RaycastMeshData>> = None;
        let params = WallTriggerVolumeParams { u_range: Some([0.15, 0.85]), normal_offset: Some(0.29), minimum_height: 0.0, offset_from_top: -0.5, apply: true };
        for bend in [3.0, 4.5] {
            // 使用真实发布/路径/高度算法生成折线墙多面，不能手填quad或用AABB替代。
            let authority = WallState::new(Curve2::try_new(vec![[-4.0, 0.0], [0.0, 0.0], [2.0, bend], [6.0, bend]], false).unwrap(), 3.0, 0, 2, false);
            let mut wall = PublicWallState::collect(&authority, &terrain).unwrap();
            wall.prepare_paths(&paths).unwrap();
            let mut volume = world.get_mut::<WallTriggerVolume>(entity).unwrap();
            volume.recompute(&wall).unwrap();
            assert!(volume.quads.len() > 4);
            let cache_before: Vec<_> = volume.default_tris().iter().flat_map(|t|t.points).flat_map(|v|v.to_array()).map(f32::to_bits).collect();
            let geo = volume.get_collision_geo(params);
            assert_eq!(geo.triangles.len(), geo.normals.len());
            assert_eq!(geo.triangles.len(), geo.uvs.len());
            assert!(geo.triangles.len() > 2);
            // 只读构造失败也不能破坏该组件的上次默认缓存和quad。
            assert!(std::panic::catch_unwind(|| volume.get_collision_geo(WallTriggerVolumeParams { u_range: Some([1.0, 0.0]), ..params })).is_err());
            assert_eq!(cache_before, volume.default_tris().iter().flat_map(|t|t.points).flat_map(|v|v.to_array()).map(f32::to_bits).collect::<Vec<_>>());
            let mesh = Arc::new(RaycastMeshData::new_unindexed(&geo.triangles, &geo.normals, Some(&geo.uvs)));
            assert_eq!(mesh.vertices(), geo.triangles.iter().flat_map(|t|t.points).collect::<Vec<_>>());
            assert_eq!(mesh.normals(), geo.normals.iter().flatten().copied().collect::<Vec<_>>());
            assert_eq!(mesh.uvs(), geo.uvs.iter().flatten().copied().collect::<Vec<_>>());
            if let Some(hash) = previous_hash { assert_ne!(mesh.hash, hash); }
            previous_hash = Some(mesh.hash);
            let weak = Arc::downgrade(&mesh);
            let result = upsert_collider(&mut world.resource_mut::<RaycastWorld>(), &mut instances, entity, RaycastGeoSource::Slot0, mesh, Vec3::ZERO, glam::Quat::IDENTITY);
            assert!(matches!(result, ColliderUpsert::Inserted { .. } | ColliderUpsert::Replaced { .. }));
            if let Some(old) = previous_weak.replace(weak) { assert!(old.upgrade().is_none()); }
            let instance = instances[&entity];
            assert!(world.resource_mut::<RaycastWorld>().get_rapier_collider(instance.slot, instance.collider).is_some());
        }
        let instance = instances[&entity];
        assert!(remove_collider(&mut world.resource_mut::<RaycastWorld>(), &mut instances, &entity).is_some());
        assert!(instances.is_empty());
        assert!(world.resource_mut::<RaycastWorld>().get_rapier_collider(instance.slot, instance.collider).is_none());
        assert!(previous_weak.unwrap().upgrade().is_none());
        assert!(world.despawn(entity));
        assert!(world.get::<WallTriggerVolume>(entity).is_none());
    }

    #[test]
    fn original_collision_geo_full_entry_keeps_all_three_output_arrays_and_failures() {
        for (json, count) in [
            (include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-wall-trigger-collision-rosetta.json")), 345),
            (include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-wall-trigger-collision-extra-rosetta.json")), 250),
        ] {
        let report:serde_json::Value=serde_json::from_str(json).unwrap();
        assert_eq!(report["rne_sha256"],"f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        let rows=report["rows"].as_array().unwrap();assert_eq!(rows.len(),count);
        let mut differences=Vec::new();
        for (case,row)in rows.iter().enumerate() {
            let input=&row["input"];let f=|v:&serde_json::Value|f32::from_bits(v.as_u64().unwrap()as u32);
            let volume=WallTriggerVolume {quads:input["quads"].as_array().unwrap().iter().map(|q|Quad {
                wall_uv:std::array::from_fn(|i|Vec2::new(f(&q["uv"][i][0]),f(&q["uv"][i][1]))),
                positions:std::array::from_fn(|i|Vec3::new(f(&q["points"][i][0]),f(&q["points"][i][1]),f(&q["points"][i][2]))),
                prepared_u:std::array::from_fn(|i|f(&q["us"][i])),
                endpoint_normals:std::array::from_fn(|i|Vec3::new(f(&q["normals"][i][0]),f(&q["normals"][i][1]),f(&q["normals"][i][2]))),
                segment_heights:std::array::from_fn(|i|f(&q["heights"][i])),
            }).collect(),special:input["special"].as_u64().unwrap()as u8,
                // 明确污染旧三组缓存，原get_collision_geo只能从quads+params重建。
                triangles:vec![Triangle::new(Vec3::splat(731.0),Vec3::splat(732.0),Vec3::splat(733.0))],
                normals:vec![[Vec3::splat(734.0);3]],uvs:vec![[Vec2::splat(735.0);3]]};
            let actual=std::panic::catch_unwind(||volume.get_collision_geo(WallTriggerVolumeParams {
                u_range:(!input["range"].is_null()).then(||std::array::from_fn(|i|f(&input["range"][i]))),
                normal_offset:(!input["offset"].is_null()).then(||f(&input["offset"])),
                minimum_height:f(&input["minimum"]),offset_from_top:f(&input["top"]),apply:input["apply"].as_bool().unwrap(),
            }));
            if row["panic"].as_bool().unwrap() {assert!(actual.is_err(),"原碰撞几何拒绝case{case}");continue;}
            let actual=actual.unwrap();
            let outputs=[("triangles",actual.triangles.iter().flat_map(|t|t.points).flat_map(|p|p.to_array()).map(f32::to_bits).collect::<Vec<_>>()),
                ("normals",actual.normals.iter().flatten().flat_map(|p|p.to_array()).map(f32::to_bits).collect()),
                ("uvs",actual.uvs.iter().flatten().flat_map(|p|p.to_array()).map(f32::to_bits).collect())];
            for (name,actual)in outputs {
                let expected:Vec<u32>=row[name].as_array().unwrap().iter().flat_map(|t|t.as_array().unwrap()).map(|x|x.as_u64().unwrap()as u32).collect();
                if actual!=expected {differences.push(format!("case{case} {name}: {:?}",actual.iter().zip(&expected).enumerate().filter(|(_, (a,b))|a!=b).take(4).collect::<Vec<_>>()));}
            }
            assert_eq!(actual.triangles.len(),actual.normals.len());assert_eq!(actual.triangles.len(),actual.uvs.len());
        }
        assert!(differences.is_empty(),"原全部三组输出严格字位差异：{differences:?}");
        }
    }

    #[test]
    fn circle_original_get_tris_instruction_cases_preserve_geometry_and_rejections() {
        let report: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../reports/module-recovery-2026-10-23/m01-circle-trigger-native.json"
        )).unwrap();
        assert_eq!(report["binary_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        let rows = report["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 288);
        for (case, row) in rows.iter().enumerate() {
            let input = &row["input"];
            let pair = |key: &str| -> [f32; 2] { serde_json::from_value(input[key].clone()).unwrap() };
            let volume = WallTriggerVolume {
                quads: vec![Quad {
                    wall_uv: serde_json::from_value::<[[f32; 2]; 4]>(input["uv"].clone()).unwrap().map(Vec2::from_array),
                    positions: serde_json::from_value::<[[f32; 3]; 4]>(input["points"].clone()).unwrap().map(Vec3::from_array),
                    prepared_u: pair("us"),
                    endpoint_normals: serde_json::from_value::<[[f32; 3]; 2]>(input["normals"].clone()).unwrap().map(Vec3::from_array),
                    segment_heights: pair("heights"),
                }],
                special: input["special"].as_u64().unwrap() as u8,
                ..Default::default()
            };
            let params = WallTriggerVolumeParams {
                u_range: serde_json::from_value(input["range"].clone()).unwrap(),
                normal_offset: serde_json::from_value(input["offset"].clone()).unwrap(),
                minimum_height: input["minimum"].as_f64().unwrap() as f32,
                offset_from_top: input["top"].as_f64().unwrap() as f32,
                apply: input["apply"].as_bool().unwrap(),
            };
            let actual = std::panic::catch_unwind(|| volume.get_tris(params));
            if !row["panic"].is_null() {
                assert!(actual.is_err(), "原版拒绝case{case}");
                continue;
            }
            let actual = actual.unwrap();
            let expected: Vec<[u32; 9]> = serde_json::from_value(row["triangles"].clone()).unwrap();
            for (triangle, expected) in actual.iter().zip(&expected) {
                assert_eq!(triangle.points.into_iter().flat_map(|p| p.to_array()).map(f32::to_bits).collect::<Vec<_>>(), expected, "原指令case{case}");
            }
            assert_eq!(actual.len(), expected.len());
        }
    }

    #[test]
    fn circle_original_trigger_optional_normal_and_nonfinite_endpoint_cases() {
        let report: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-circle-trigger-edge-native.json"
        ))).unwrap();
        let rows = report["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 24);
        for (case, row) in rows.iter().enumerate() {
            let input = &row["input"];
            let f = |value: &serde_json::Value| f32::from_bits(value.as_u64().unwrap() as u32);
            let volume = WallTriggerVolume {
                quads: vec![Quad {
                    wall_uv: std::array::from_fn(|i| Vec2::new(f(&input["uv"][i][0]), f(&input["uv"][i][1]))),
                    positions: std::array::from_fn(|i| Vec3::new(f(&input["points"][i][0]), f(&input["points"][i][1]), f(&input["points"][i][2]))),
                    prepared_u: std::array::from_fn(|i| f(&input["us"][i])),
                    endpoint_normals: std::array::from_fn(|i| Vec3::new(f(&input["normals"][i][0]), f(&input["normals"][i][1]), f(&input["normals"][i][2]))),
                    segment_heights: std::array::from_fn(|i| f(&input["heights"][i])),
                }], special: input["special"].as_u64().unwrap() as u8, ..Default::default()
            };
            let actual = volume.get_tris(WallTriggerVolumeParams {
                u_range: None,
                normal_offset: (!input["offset"].is_null()).then(|| f(&input["offset"])),
                minimum_height: f(&input["minimum"]), offset_from_top: f(&input["top"]), apply: true,
            });
            let expected: Vec<[u32; 9]> = serde_json::from_value(row["triangles"].clone()).unwrap();
            assert_eq!(actual.len(), expected.len());
            for (triangle, expected) in actual.iter().zip(expected) {
                for (actual, expected) in triangle.points.into_iter().flat_map(|p| p.to_array()).map(f32::to_bits).zip(expected) {
                    // x86的inf*0产生负indefinite NaN，ARM产生正NaN；只容许这一
                    // 不参与任何后续业务判定的符号差，不能放宽有限几何或NaN载荷。
                    let architecture_indefinite = cfg!(target_arch = "aarch64") && actual == 0x7fc0_0000 && expected == 0xffc0_0000;
                    assert!(actual == expected || architecture_indefinite, "原get_tris可选法线case{case}: {actual:08x} != {expected:08x}");
                }
            }
        }
    }
}
