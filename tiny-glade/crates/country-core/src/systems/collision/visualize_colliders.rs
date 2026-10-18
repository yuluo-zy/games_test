//! `systems/collision/visualize_colliders.rs` — collider 调试可视化（B `visualize_colliders`，
//! LOGIC.md 未实现登记段：6 函数，业务 2）。
//!
//! 证据：`planning/impl-collision-remainder/results/b8-closures-visualize-rt.result.json`
//! fresh 反编译（ghidra shared 只读，2026-10-17）。
//!
//! ## 分类（协议 §2026-10-17）
//!
//! | VA | 原函数 | 分类 | verdict | 分歧/依赖 |
//! |---|---|---|---|---|
//! | 0x141326370 | `visualize_colliders`（1460B） | 真业务 | 未实现-登记 | 遍历 `RaycastWorld` 全 collider + puffin scope + `show`；`show` 属 debug_viz ECS 胶水（debug_viz.rs 登记） |
//! | 0x141326930 | `visualize_shape`（2258B） | 真业务 | 未实现-登记 | 按形状类型分派：`as_compound/as_trimesh/as_convex_polyhedron/as_cuboid/as_ball` + `to_trimesh` + `from_iter<Vec3>` 构线；依赖 `RaycastMeshData` 顶点/索引布局 |
//! | (closure$0/$1 ×2、`next`/`from_iter` 等 4) | `visualize_shape::closure$0/$1` | 宏胶水/closure | 标记 | 顶点→三角迭代 glue；随父函数覆盖 |
//!
//! ## 已锚定证据
//!
//! - `visualize_shape` 的形状分派顺序（O：0x141326930 fresh）：compound → trimesh →
//!   convex_polyhedron → cuboid → ball → 不支持（`"not_yet_implemented__Unsupported"` 串 +
//!   shape 的 Debug fmt）。
//! - `from_iter<Vec3, FlatMap<Copied<Iter<[u32;3]>>, Map<IntoIter<u32,4>, closure>>>`：
//!   由三角索引（每三角 3×u32）经 closure 展开为顶点线框（quad 分片）。
//!
//! ## 待接（依赖落地后实现）
//!
//! 输入：`RaycastWorld` collider 枚举 + 每 collider 的 `RaycastMeshData`（顶点/索引）；
//! 输出：`utils::debug_viz::DebugLine`（颜色/线宽按形状类型）或 `show` 闭包（ECS 胶水）。
//! `box_trigger_volume`（触发体积盒）的展开属登记项。

// ---------------------------------------------------------------------------
// visualize_colliders / visualize_shape 实现（impl-collision-types，2026-10-17）。
//
// 证据：ghidra shared fresh（0x141326370 / 0x141326930，b8-closures-visualize-rt）：
// - `visualize_colliders`：17 槽（0x390 步进）collider arena 遍历（条目 `*==1` 占用），
//   注册表 (index,gen) 校验 → 按 kind 字节 + `DebugGui +0x104` 过滤 → `visualize_shape`；
// - `visualize_shape`：形状分派 compound → trimesh → convex_polyhedron → cuboid → ball
//   → 不支持（panic "not_yet_implemented__Unsupported"）；三角线框
//   `from_iter<Vec3, FlatMap<Copied<Iter<[u32;3]>>, Map<IntoIter<u32,4>, closure>>>`
//   （每三角 [a,b,c,a] 4 点闭环）；compound 递归带嵌套变换。
// 原版直接调 `DebugLine::show`/`DebugSphere::show`（ECS 胶水，host 待接线）。
// 本实现产生 `DebugDrawRequest` 清单（DebugLine/DebugSphere），`show` 胶水登记。
// ---------------------------------------------------------------------------

use crate::resources::gui::DebugGui;
use crate::systems::collision::world::{RaycastWorld, SubworldColliderHandle};
use crate::utils::debug_viz::{DebugColor, DebugLine, DebugSphere};
use glam::Vec3;

/// 可视化输出（O：`DebugLine::show`/`DebugSphere::show` 的请求清单；host 侧按
/// AssetMeshLibrary/AssetShaderLibrary 落屏，ECS 胶水登记）。
#[derive(Clone, Debug, PartialEq)]
pub enum DebugDrawRequest {
    /// 折线（O：`DebugLine`）。
    Line(DebugLine),
    /// 球体（O：`DebugSphere`）。
    Sphere(DebugSphere),
}

/// kind 过滤（O：0x141326370 switch + `DebugGui +0x104` 门控；kind 字节 = 注册表条目
/// @+0x00）。Rust `ShowDebugRaycastGeo` 判别经 `as u8` 参与（原枚举变体多于 5，登记）。
fn kind_filtered(kind: u8, gui_byte: u8) -> bool {
    match kind {
        0 => gui_byte == 1,
        2 => gui_byte == 2,
        3 => (gui_byte | 4) == 7,
        4 | 5 => gui_byte == 4,
        6 => (gui_byte | 2) == 7,
        8 => (gui_byte & 0xe) == 6,
        0xd | 0xe => (gui_byte.wrapping_sub(7)) <= 1,
        0xf => gui_byte == 9,
        0x10 => gui_byte == 10,
        // kind 1/7/9/0xb/0xc/0x11+：不显示（O：default 分支跳过）。
        _ => false,
    }
}

/// `visualize_colliders`（0x141326370，O）：遍历 `RaycastWorld` 全 collider，
/// 按注册表 kind + DebugGui 过滤后逐 collider 调 `visualize_shape`。
///
/// fresh 反编译：槽 0x20..0x3cb0 步进 0x390；每槽 collider arena 数据@+8/len@+0x10，
/// 条目 0x30 步进、`*==1` 占用；占用者 (index,gen) 校验（失败 panic）+ kind 过滤 +
/// `visualize_shape`。Rust 以 `ColliderSet` 迭代等价（arena 字节布局登记）。
#[cfg(feature = "native-physics")]
pub fn visualize_colliders(
    world: &RaycastWorld,
    gui: &DebugGui,
    requests: &mut Vec<DebugDrawRequest>,
) {
    let gui_byte = gui.show_raycast_geometry as u8;
    world.for_each_collider(|slot_index, handle, collider| {
        let (index, generation) = handle.into_raw_parts();
        // O（0x141326370）：注册表校验 `entry.generation@+0x20 == (generation | 1)`，
        // 失败 `begin_panic`；kind 读 `entry@+0x00`（即遍历 `get_collider` 的注册表面）。
        let entry = world
            .collider_registry()
            .get(index as usize)
            .filter(|entry| entry.generation | 1 == generation | 1)
            .unwrap_or_else(|| panic!("visualize_colliders: collider handle invalid"));
        if !kind_filtered(entry.kind(), gui_byte) {
            return;
        }
        let sub_handle = SubworldColliderHandle {
            index,
            generation,
            slot: slot_index,
        };
        visualize_shape(
            collider,
            world,
            sub_handle,
            &utils::iso_transform::IsoTransform::identity(),
            requests,
        );
    });
}

/// `visualize_shape`（0x141326930，O）：按形状类型分派，产生线框/球体请求。
///
/// fresh 反编译：compound → 递归（子形状 + 复合变换，`in_RCX` 嵌套变换）；
/// trimesh / convex_polyhedron → 三角索引线框（`[a,b,c,a]` 4 点闭环）+ `DebugLine`；
/// cuboid → `box_trigger_volume` 展开 + 逐 quad `DebugLine`（+0x1c/+0x24 参数登记）；
/// ball → `DebugSphere::show`；其余 panic "not_yet_implemented__Unsupported"。
#[cfg(feature = "native-physics")]
pub fn visualize_shape(
    collider: &rapier3d::geometry::Collider,
    _world: &RaycastWorld,
    _handle: SubworldColliderHandle,
    transform: &utils::iso_transform::IsoTransform,
    requests: &mut Vec<DebugDrawRequest>,
) {
    let shape = collider.shape();
    if let Some(compound) = shape.as_compound() {
        // O：compound 递归 + 复合变换（`nested_in` 语义，同 visualize_shape 自身递归）。
        for (_pos, child) in compound.shapes().iter() {
            let _ = child;
            let _ = transform;
            // 登记：子形状递归的复合变换与顶点投影在 Rust 侧按 parry 布局等价，
            // 当前为结构占位（子 compound/trimesh 线框登记）。
        }
        return;
    }
    if let Some(trimesh) = shape.as_trimesh() {
        let points = trimesh_wireframe(trimesh);
        requests.push(DebugDrawRequest::Line(DebugLine::new(points)
            .color(DebugColor::Green)
            .line_width(1.0)));
        return;
    }
    if let Some(convex) = shape.as_convex_polyhedron() {
        let (v, idx) = convex.to_trimesh();
        let trimesh = rapier3d::parry::shape::TriMesh::new(v, idx);
        let points = trimesh_wireframe(&trimesh);
        requests.push(DebugDrawRequest::Line(DebugLine::new(points)
            .color(DebugColor::Green)
            .line_width(1.0)));
        return;
    }
    if let Some(cuboid) = shape.as_cuboid() {
        let half = cuboid.half_extents;
        let points = cuboid_edges(&glam::Vec3::new(half.x, half.y, half.z));
        requests.push(DebugDrawRequest::Line(DebugLine::new(points)
            .color(DebugColor::Blue)
            .line_width(1.0)));
        return;
    }
    if let Some(ball) = shape.as_ball() {
        let pos = collider.position().translation;
        requests.push(DebugDrawRequest::Sphere(
            DebugSphere::new(Vec3::new(pos.x, pos.y, pos.z))
                .radius(ball.radius)
                .color(DebugColor::Red),
        ));
        return;
    }
    panic!("not_yet_implemented__Unsupported shape in visualize_shape");
}

/// 三角线框顶点（O：`from_iter<Vec3, FlatMap<Copied<Iter<[u32;3]>>, Map<IntoIter<u32,4>, closure>>>`，
/// 每三角 `[a,b,c,a]` 4 点闭环）。
#[cfg(feature = "native-physics")]
fn trimesh_wireframe(trimesh: &rapier3d::parry::shape::TriMesh) -> Vec<Vec3> {
    let vertices = trimesh.vertices();
    let mut out = Vec::new();
    for tri in trimesh.indices() {
        let mut quad = [0u32; 4];
        quad[0] = tri[0];
        quad[1] = tri[1];
        quad[2] = tri[2];
        quad[3] = tri[0];
        for idx in quad {
            if let Some(v) = vertices.get(idx as usize) {
                out.push(Vec3::new(v.coords.x, v.coords.y, v.coords.z));
            }
        }
    }
    out
}

/// cuboid 12 边线框（O：`box_trigger_volume` 展开；Rust 以 8 角 12 边等价，登记）。
#[cfg(feature = "native-physics")]
fn cuboid_edges(half: &glam::Vec3) -> Vec<Vec3> {
    let (x, y, z) = (half.x, half.y, half.z);
    let c = [
        Vec3::new(-x, -y, -z),
        Vec3::new(x, -y, -z),
        Vec3::new(x, -y, z),
        Vec3::new(-x, -y, z),
        Vec3::new(-x, y, -z),
        Vec3::new(x, y, -z),
        Vec3::new(x, y, z),
        Vec3::new(-x, y, z),
    ];
    let edges = [
        (0, 1), (1, 2), (2, 3), (3, 0),
        (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];
    edges.iter().flat_map(|&(a, b)| [c[a], c[b]]).collect()
}

/// 非 native-physics 壳：无可视化（查询不落地）。
#[cfg(not(feature = "native-physics"))]
pub fn visualize_colliders(
    _world: &RaycastWorld,
    _gui: &DebugGui,
    _requests: &mut Vec<DebugDrawRequest>,
) {
}

#[cfg(all(test, feature = "native-physics"))]
mod tests {
    use super::*;

    #[test]
    fn kind_filters_match_decompiled_gui_byte_conditions() {
        // O：kind 0 需 +0x104==1；kind 2 需 ==2；kind 4/5 需 ==4。
        assert!(kind_filtered(0, 1));
        assert!(!kind_filtered(0, 2));
        assert!(kind_filtered(2, 2));
        assert!(!kind_filtered(2, 1));
        assert!(kind_filtered(4, 4));
        assert!(kind_filtered(5, 4));
        assert!(!kind_filtered(5, 3));
        // kind 1 不显示。
        assert!(!kind_filtered(1, 1));
    }

    #[test]
    fn trimesh_wireframe_closes_each_triangle() {
        // 1 三角 (0,1,2) → 4 点 [p0,p1,p2,p0]。
        let v = [
            rapier3d::parry::math::Point::new(0.0, 0.0, 0.0),
            rapier3d::parry::math::Point::new(1.0, 0.0, 0.0),
            rapier3d::parry::math::Point::new(0.0, 1.0, 0.0),
        ];
        let trimesh = rapier3d::parry::shape::TriMesh::new(v.to_vec(), vec![[0u32, 1, 2]]);
        let pts = trimesh_wireframe(&trimesh);
        assert_eq!(pts.len(), 4);
        assert_eq!(pts[0], pts[3]); // 闭环
    }
}
