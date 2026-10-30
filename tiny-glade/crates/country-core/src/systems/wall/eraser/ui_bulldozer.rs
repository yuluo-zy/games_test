//! B `systems/wall/eraser/ui_bulldozer.rs`: bulldozer UI entry.
//!
//! Evidence: `chapters/systems/wall/eraser/ui_bulldozer.rs.md` (O,
//! 0x1409de190 / condition 0x1409de170). The condition is
//! `AppMode::Eraser(EraserLayer::Bulldoze)` (native tag 9, sub-discriminant
//! 3). **归属核对（fresh 2026-10-17）**：0x1409de190 本体是 **UI 生产者**——
//! `UiState::add` + `Rapier::raycast_simple`（0x1408ce180，collision 域）→
//! 命中类型跳表（case 0x11 demo / 0-2,6-9 墙 / 4 / 5 / 13 / 14...）→ 逐墙记录
//! 距离 → "delete_tree"/"cursor_hammer" signifier 写 + UiState::add<String>
//! → 末 jumptable 动作分派。Rust 此前把 **事件→缓冲桥**（AnimateWallBulldozeCmd
//! 读 → AnimateWallBulldozeBuffer 入队）误挂到本 VA——该缓冲入队是 native 跳表
//! 目标 handler（独立 VA，不在 0x1409de190 体内），已拆为
//! [`enqueue_bulldoze_command`]。
//!
//! The live wall branch uses the collision owner and UI state directly.
//! Other target branches and the original competing-target scan remain open.

use bevy_ecs::prelude::*;

use super::animate_wall_bulldoze::AnimateWallBulldozeBuffer;
use super::AnimateWallBulldozeCmd;
use crate::systems::wall::eraser::bulldoze::WallBulldozeCommand;
use crate::resources::walls::{InnerWalls, WallId};
use crate::systems::mode_manager::{AppMode, EraserLayer};

/// B 1409e0df5: the new sorted segment and the removed point.
#[derive(Event, Clone, Copy, Debug)]
pub struct HandleStairAttachedDecoratorsOnStairPointDeletion {
    pub retained: (u32, u32),
    pub removed: u32,
}

/// B 0x1409de170: `(*app).tag == 9 && (*app).byte@+4 == 3` = bulldozer layer.
pub fn is_bulldozer_layer(mode: &AppMode) -> bool {
    matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze))
}

/// B 0x1409de170 as a run condition.
pub fn wall_eraser_bulldoze_condition(mode: Res<AppMode>) -> bool {
    is_bulldozer_layer(&mode)
}


/// The original first dispatch preserves all 15 collider tags; only the
/// seven wall tags share the WallId and animation-queue branch. Other tags
/// retain their complete receipt for the subsequent roof/stair/decorator
/// resolution, rather than being silently treated as misses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BulldozerColliderHit {
    Miss,
    Wall { wall: WallId, position: glam::Vec3 },
    AnimatingWall(crate::systems::collision::world::RaycastSimpleResult),
    Other(crate::systems::collision::world::RaycastSimpleResult),
}

/// B 1409de443..4e8: max TOI f32::MAX, mask 0x77fb, then dispatch on
/// the first byte of the copied collider receipt. A queued wall cannot be
/// selected a second time.
pub fn bulldozer_raycast(
    physics: &crate::systems::collision::world::RaycastWorld,
    ray: &crate::resources::mouse_terrain_raycast::CameraCursorRay,
    buffer: &AnimateWallBulldozeBuffer,
) -> BulldozerColliderHit {
    let hit = physics.raycast_simple(ray.origin, ray.dir, 0x77fb);
    if hit.is_miss() {
        return BulldozerColliderHit::Miss;
    }
    match hit.handle[0] as u8 {
        0..=2 | 6..=9 => {
            let wall = WallId(hit.handle[1]);
            if buffer.contains(wall) {
                BulldozerColliderHit::AnimatingWall(hit)
            } else {
                BulldozerColliderHit::Wall { wall, position: hit.position }
            }
        }
        _ => BulldozerColliderHit::Other(hit),
    }
}

/// Register the original terrain canvas before querying collision. It is a
/// separate widget from the selected object's closer terrain canvas.
pub fn register_bulldozer_canvas(
    ui: &crate::systems::ui::resources::ui_state::UiState,
) {
    use crate::systems::ui::resources::widget_params::{UiSounds, WidgetGeometry, WidgetParams, WidgetVolumeShape};
    let _ = ui.add(
        "bulldoze_canvas",
        WidgetParams::new(WidgetVolumeShape::default(), [1, 0])
            .sounds(UiSounds::empty())
            .geometry(WidgetGeometry::TerrainCanvas { distance: f32::MAX, layer: 0 })
            .occlude_by_world(false),
    );
}

/// Wall case 1409deb49..cebd: its terrain widget is biased 0.1 toward
/// the camera so it wins over the underlying collision surface.
pub fn register_bulldozer_wall_widget<'a>(
    ui: &'a crate::systems::ui::resources::ui_state::UiState,
    wall: WallId,
    ray_origin: glam::Vec3,
    hit: glam::Vec3,
) -> crate::systems::ui::resources::ui_state::UiWidgetContext<'a> {
    use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams, WidgetVolumeShape};
    let distance = (ray_origin - hit).length() - 0.1;
    ui.add(
        &format!("bulldoze_wall_{wall}"),
        WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
            .geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 }),
    )
}

/// B 1409e133f..13a7: project the UI intersection onto the published
/// 3D outline in XZ, then take a 0.2-world-unit span on the authority curve.
pub fn bulldozer_wall_range(
    public: &crate::resources::walls::public_wall_state::PublicWallState,
    authority: &crate::resources::walls::WallState,
    hit: glam::Vec3,
) -> [f32; 2] {
    let u = utils::curve::approx_curve_u(
        &public.geometry.outline,
        &public.geometry.outline_u,
        glam::Vec2::new(hit.x, hit.z),
        |p| glam::Vec2::new(p[0], p[2]),
    );
    // MAXSS/MINSS return the second operand on equality or unordered input.
    let maxss = |a: f32, b: f32| if a > b { a } else { b };
    let minss = |a: f32, b: f32| if a < b { a } else { b };
    let u = minss(1.0, maxss(0.0, u));
    let half_width = (0.2 / authority.source().curve.length) * 0.5;
    [
        minss(1.0, maxss(0.0, u - half_width)),
        minss(1.0, maxss(0.0, u + half_width)),
    ]
}

#[derive(bevy_ecs::system::SystemParam)]
pub struct BulldozerParams<'w, 's, S: Send + Sync + 'static> {
    mode: Res<'w, AppMode>,
    demo: Res<'w, crate::DevOverrideIsDemoMode>,
    cursor: Res<'w, crate::resources::mouse_terrain_raycast::CursorPositionSS>,
    ray: Res<'w, crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    ui: Res<'w, crate::systems::ui::resources::ui_state::UiState>,
    archival: crate::resources::walls::decorator_archivist::DecoratorArchivalResources<'w>,
    public: Res<'w, crate::resources::walls::public_walls::PublicWalls>,
    inner: Res<'w, InnerWalls>,
    buffer: ResMut<'w, AnimateWallBulldozeBuffer>,
    storage: ResMut<'w, crate::resources::walls::decorator_storage::DecoratorStorage>,
    decorators: ResMut<'w, crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    wall_colors: Res<'w, crate::resources::color_id::WallColorIds>,
    timber_colors: Res<'w, crate::resources::color_id::HalftimberColorIds>,
    history: ResMut<'w, crate::resources::history::History<S>>,
    roofs: Query<'w, 's, &'static crate::resources::roofs::Roof>,
    roof_delete: EventWriter<'w, crate::resources::roofs::commands::DeleteRoofCmd>,
    roof_highlight: EventWriter<'w, crate::resources::roofs::commands::HighlightRoofCmd>,
    wall_highlight: EventWriter<'w, crate::systems::wall::wall_highlight::HighlightWallCmd>,
    tool_used: EventWriter<'w, crate::resources::onboarding::NotifyHintSystemToolWasJustUsed>,
    erased: EventWriter<'w, crate::resources::onboarding::NotifyHintSystemSomethingWasErased>,
    wall_audio: EventWriter<'w, crate::systems::audio::PlayWallUiSound>,
    trees: Res<'w, crate::resources::tree_storage::TreeStorage>,
    tree_query: Query<'w, 's, (&'static crate::resources::prefab::PrefabInstance,
        &'static crate::systems::prefab::PrefabInstanceData, &'static crate::components::TreeComponent)>,
    tree_delete: EventWriter<'w, crate::resources::history::tree_edit::DeleteTreeCmd>,
    removed_supports: ResMut<'w, crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,
    cached: Res<'w, crate::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    decorator_delete: EventWriter<'w, crate::resources::walls::decorator::ExternalDecoratorCmd>,
    subtypes: Res<'w, crate::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,
    meshes: Res<'w, crate::resources::walls::decorator_subtype::DecoratorMeshLibrary>,
    signifiers: ResMut<'w, crate::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    stairs_audio: EventWriter<'w, crate::systems::audio::PlayStairsAudioCmd>,
    stairs: ResMut<'w, crate::resources::stairs::stairs_state::StairsState>,
    stair_reanchor: EventWriter<'w, HandleStairAttachedDecoratorsOnStairPointDeletion>,
    stair_outline: EventWriter<'w, crate::systems::wall::structure_outline::OutlineStairsCmd>,
}

/// Original wall branch of 1409de190. Non-wall dispatch is still being
/// recovered; keep its receipt intact in `bulldozer_raycast`.
pub fn wall_eraser_bulldoze<S: Send + Sync + 'static>(mut p: BulldozerParams<'_, '_, S>) {
    if !is_bulldozer_layer(&p.mode) { return; }
    let mut active = false;
    wall_eraser_bulldoze_impl(&mut p, &mut active);
    let cursor = p.cursor.0;
    hammer_cursor_icon(&mut p.signifiers, cursor, active);
}

fn wall_eraser_bulldoze_impl<S: Send + Sync + 'static>(p: &mut BulldozerParams<'_, '_, S>, active: &mut bool) {
    use crate::resources::history::EditType;
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    use std::sync::Arc;

    if !is_bulldozer_layer(&p.mode) { return; }
    register_bulldozer_canvas(&p.ui);
    let hit = bulldozer_raycast(&p.archival.raycast_world, &p.ray, &p.buffer);
    let best = match hit {
        BulldozerColliderHit::Miss => f32::MAX,
        BulldozerColliderHit::Wall { position, .. } => (position - p.ray.origin).length(),
        BulldozerColliderHit::AnimatingWall(hit) | BulldozerColliderHit::Other(hit) =>
            (hit.position - p.ray.origin).length(),
    };
    let mut selected_tree = None;
    if !p.demo.0 { for (&index, entry) in p.trees.iter() {
        let (data, prefab, tree) = p.tree_query.get(entry.entity)
            .expect("TreeStorage entry missing tree placement components");
        let center = prefab.0.position + glam::Vec3::Y * 0.5;
        let mut sounds = crate::systems::ui::resources::widget_params::UiSounds::empty();
        sounds.slots[0][..4].copy_from_slice(&23u32.to_le_bytes());
        sounds.slots[1][..4].copy_from_slice(&24u32.to_le_bytes());
        let widget = p.ui.add(&format!("delete_tree_{index:?}"),
            crate::systems::ui::resources::widget_params::WidgetParams::new(
                crate::systems::ui::resources::widget_params::WidgetVolumeShape::sphere(center, 1.0), [0; 2])
                .sounds(sounds));
        if widget.is_active() {
            let intersection = widget.intersecting();
            let distance = f32::from_le_bytes(intersection[32..36].try_into().unwrap());
            if best > distance {
                selected_tree = Some((index, widget.clicked(), center, prefab.0.clone(),
                    data.prefab_name.as_name_ref().to_string(), tree.is_preplaced));
            }
        }
    }}
    if let Some((index, clicked, center, placement, name, preplaced)) = selected_tree {
        *active = true;
        write_tree_bulldoze_signifier(&mut p.signifiers, center);
        if clicked {
            p.tree_delete.write(crate::resources::history::tree_edit::DeleteTreeCmd { id: index.0 });
            let edit = crate::resources::history::tree_edit::TreeEdit::Delete(
                crate::resources::history::tree_edit::TreeEditData::from_placement(index, &placement, name, preplaced));
            p.history.add(EditType::Tree(Arc::new(edit))).expect("bulldozer tree history requires recording");
        }
        return;
    }
    if let BulldozerColliderHit::Other(other) = hit {
        if matches!(other.handle[0] as u8, 5 | 13) {
            use crate::resources::walls::decorator::DecoratorId;
            use crate::resources::stairs::stairs_state::StairSegment;
            use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams, WidgetVolumeShape};
            let node = (other.handle[0] >> 32) as u32;
            let second = if other.handle[0] as u8 == 5 { node } else { other.handle[1] as u32 };
            let neighbors: Vec<_> = p.stairs.graph().neighbors(node).collect();
            let midpoint = node == second && neighbors.len() == 2;
            let distance = (other.position - p.ray.origin).length();
            let name = format!("bulldoze_stair_segment_StairGraphEdge(DecoratorId({node}), DecoratorId({second}))");
            let widget = p.ui.add(&name, WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
                .geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 }));
            *active = widget.is_active();
            let midpoint_position = if midpoint {
                let Some(position) = p.cached.try_position(node) else { return };
                Some(position)
            } else { None };
            let edges: Vec<(u32, u32)> = if node == second {
                neighbors.iter().map(|&n| (node.min(n), node.max(n))).collect()
            } else { vec![(node, second)] };
            let mut removed = std::collections::BTreeSet::new();
            if midpoint || node == second { removed.insert(node); }
            if !midpoint {
                for &(a, b) in &edges {
                    for id in [a, b] {
                        if p.stairs.graph().neighbors(id).count() == 1 { removed.insert(id); }
                    }
                }
            }
            for &id in &removed {
                if let Some(center) = p.cached.try_position(id) {
                    crate::utils::debug_viz::DebugSphere::new(center)
                        .color(crate::utils::debug_viz::DebugColor::Red).radius(0.3).solid().show(&mut p.signifiers);
                }
            }
            for &(a, b) in &edges {
                p.stair_outline.write(crate::systems::wall::structure_outline::OutlineStairsCmd {
                    segment: StairSegment::new(DecoratorId(a), DecoratorId(b)),
                    strength: if midpoint { 2.0 } else { 4.0 }, kind: if midpoint { 2 } else { 1 }, include_connected: false,
                });
                if !midpoint {
                    p.stair_outline.write(crate::systems::wall::structure_outline::OutlineStairsCmd {
                        segment: StairSegment::new(DecoratorId(node), DecoratorId(second)),
                        strength: 0.0, kind: 3, include_connected: false,
                    });
                }
            }
            if widget.is_active() && widget.clicked() {
                // The stair-attached culler runs after the graph mutation and
                // requires permission for each affected old segment.
                if let Some(bucket) = p.storage.get(DecoratorStorageAddr::Stair) {
                    let ids: Vec<_> = bucket.decorators.iter().filter_map(|(&id, info)| {
                        if info.dst_tag() != 4 { return None; }
                        let crate::resources::walls::decorator::DecoratorDst::Stair(dst) = info.destination() else { return None };
                        edges.iter().any(|&(a, b)| (dst.first_id, dst.second_id) == (a.min(b), a.max(b))).then_some(id)
                    }).collect();
                    p.decorators.culling.allowed_ids.extend(ids);
                    p.decorators.archive_current_decorators_at_with_resources(DecoratorStorageAddr::Stair, &p.storage, &p.archival, "bulldozer stair");
                }
                if !midpoint {
                    if removed.is_empty() && !edges.is_empty() {
                        p.history.add(EditType::DecoratorV5(Arc::new(crate::resources::history::decorator_edit::DecoratorV5Edit {
                            kind: 5, fields: [0; 3],
                        }))).expect("bulldozer stair history requires recording");
                        let id = edges[0].0;
                        let payload = p.storage.lookup_decorator_info(DecoratorId(id))
                            .filter(|info| (info.extra_style() | 0xf8) < 0xfa).map(|info| info.u30).unwrap_or(0);
                        p.stairs_audio.write(crate::systems::audio::PlayStairsAudioCmd {
                            kind: 4, position: p.cached.try_position(id).unwrap_or(glam::Vec3::ZERO),
                            values: [0.0; 4], tail: [f32::from_bits((payload >> 32) as u32), f32::from_bits(payload as u32)], bytes: [0; 3],
                        });
                    }
                    for (a, b) in edges { p.stairs.disconnect_edge(a, b); }
                    for id in removed {
                        p.stairs.remove_node(id);
                        let position = p.cached.position(id);
                        let address = p.storage.lookup_storage(DecoratorId(id)).expect("stair node decorator missing");
                        p.decorators.archive_current_decorators_at_with_resources(address, &p.storage, &p.archival, "bulldozer stair point");
                        let mut payload = [0; 12];
                        for (i, value) in position.to_array().into_iter().enumerate() {
                            payload[i * 4..i * 4 + 4].copy_from_slice(&value.to_le_bytes());
                        }
                        p.decorator_delete.write(crate::resources::walls::decorator::ExternalDecoratorCmd {
                            wall: 0, decorator_id: id, dst: address, payload, stream: 0,
                        });
                    }
                    return;
                }
                p.stairs.remove_node(node);
                p.stairs.add_edge(neighbors[0], neighbors[1]);
                let (a, b) = if neighbors[0] < neighbors[1] { (neighbors[0], neighbors[1]) } else { (neighbors[1], neighbors[0]) };
                p.stair_reanchor.write(HandleStairAttachedDecoratorsOnStairPointDeletion { retained: (a, b), removed: node });
                let address = p.storage.lookup_storage(DecoratorId(node)).expect("stair node decorator missing");
                let position = midpoint_position.expect("midpoint position was checked before mutation");
                p.decorators.archive_current_decorators_at_with_resources(address, &p.storage, &p.archival, "bulldozer stair point");
                let mut payload = [0; 12];
                for (i, value) in position.to_array().into_iter().enumerate() {
                    payload[i * 4..i * 4 + 4].copy_from_slice(&value.to_le_bytes());
                }
                p.decorator_delete.write(crate::resources::walls::decorator::ExternalDecoratorCmd {
                    wall: 0, decorator_id: node, dst: address, payload, stream: 0,
                });
            }
            return;
        }
        if other.handle[0] as u8 == 4 {
            use crate::resources::walls::decorator::DecoratorId;
            use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams, WidgetVolumeShape};
            let id = DecoratorId((other.handle[0] >> 32) as u32);
            let address = match other.handle[1] {
                0 => DecoratorStorageAddr::Wall(WallId(other.handle[2])),
                1 => DecoratorStorageAddr::Terrain,
                2 => DecoratorStorageAddr::Stair,
                _ => panic!("invalid bulldozer decorator storage address"),
            };
            let bucket = p.storage.get(address).expect("bulldozer decorator storage is missing");
            let info = *bucket.get(id).expect("bulldozer decorator is missing");
            let transform = p.cached.transform(id.0);
            let position = transform.translation;
            let distance = (other.position - p.ray.origin).length();
            // Native DecoratorId Display includes the wrapper, unlike the
            // current shared Rust Display implementation.
            let widget = p.ui.add(&format!("bulldoze_decorator_DecoratorId({})", id.0),
                WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
                    .geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 }));
            *active = widget.is_active();
            let forward = info.extra_style() < 2 && match address {
                DecoratorStorageAddr::Wall(wall) => p.public.get(wall).is_some_and(|wall| wall.shape().tag() < 2),
                _ => false,
            };
            let subtype = p.subtypes.get(id.0);
            let outline = p.meshes.get_outline(crate::resources::walls::decorator_subtype::DecoratorLibKey::new(subtype.0, info.rank));
            let affine = glam::Affine3A::from_scale_rotation_translation(transform.scale, transform.rotation, position);
            outline.display(&affine, &mut p.signifiers, id,
                crate::resources::walls::decorator::DecoratorRank(info.rank), forward, true);
            let start = widget.dragged() == 3;
            let clicked = widget.clicked();
            // Native 1409e1017 archives and grants culling on drag start;
            // the release/click branch only emits the removal command.
            if start {
                let ids: Vec<_> = bucket.decorators().keys().copied().collect();
                p.decorators.culling.allowed_ids.extend(ids);
                p.decorators.archive_current_decorators_at_with_resources(address, &p.storage, &p.archival, "bulldozer decorator");
            }
            if clicked {
                let mut payload = [0; 12];
                for (i, value) in position.to_array().into_iter().enumerate() {
                    payload[i * 4..i * 4 + 4].copy_from_slice(&value.to_le_bytes());
                }
                p.decorator_delete.write(crate::resources::walls::decorator::ExternalDecoratorCmd {
                    wall: other.handle[2], decorator_id: id.0, dst: address, payload, stream: 0,
                });
            }
            return;
        }
        if other.handle[0] as u8 == 14 {
            use crate::resources::stairs::stairs_state::StairGraphEdge;
            use crate::resources::history::stairs_history_edit::StairsHistoryEdit;
            use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams, WidgetVolumeShape};
            let edge = StairGraphEdge::new((other.handle[0] >> 32) as u32, other.handle[1] as u32);
            if p.removed_supports.contains(&edge) { return; }
            let distance = (other.position - p.ray.origin).length();
            let (a, b) = edge.nodes();
            // Original 1409e38f0 formats a tuple of two DecoratorId wrappers;
            // this crate's StairGraphEdge Debug has named Rust fields instead.
            let widget = p.ui.add(&format!("bulldoze_stair_support_StairGraphEdge(DecoratorId({a}), DecoratorId({b}))"),
                WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
                    .geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 }));
            *active = widget.is_active();
            if widget.is_active() {
                p.stair_outline.write(crate::systems::wall::structure_outline::OutlineStairsCmd {
                    segment: crate::resources::stairs::stairs_state::StairSegment::new(
                        crate::resources::walls::decorator::DecoratorId(a), crate::resources::walls::decorator::DecoratorId(b)),
                    strength: 0.0, kind: 3, include_connected: false,
                });
            }
            if widget.is_active() && widget.clicked() {
                p.removed_supports.record(edge);
                p.history.add(EditType::Stairs(Arc::new(StairsHistoryEdit::SupportsDelete(edge))))
                    .expect("bulldozer stair support history requires recording");
                // B1409e0887..08b5/205d..20ef looks up the first node's
                // eligible extra payload, then emits kind 0x11 at the UI hit.
                let payload = p.storage.lookup_decorator_info(
                    crate::resources::walls::decorator::DecoratorId(a))
                    .filter(|info| (info.extra_style() | 0xf8) < 0xfa)
                    .map(|info| info.u30).unwrap_or(0);
                p.stairs_audio.write(crate::systems::audio::PlayStairsAudioCmd {
                    kind: 0x11,
                    position: p.ui.intersection_point(),
                    values: [0.0; 4],
                    tail: [f32::from_bits((payload >> 32) as u32), f32::from_bits(payload as u32)],
                    bytes: [0; 3],
                });
            }
            return;
        }
        return;
    }
    let BulldozerColliderHit::Wall { wall, position } = hit else { return; };
    let widget = register_bulldozer_wall_widget(&p.ui, wall, p.ray.origin, position);
    *active = widget.is_active();
    let interaction = p.ui.interaction();
    if widget.is_active() && (interaction.native_tag == 1 || interaction.native_phase >= 2) {
        if let Some(public) = p.public.get(wall) {
            if public.authority.source_flag_65() == 0 {
                p.wall_highlight.write(crate::systems::wall::wall_highlight::HighlightWallCmd { wall, ty: 1 });
                if p.public.roof_entity(wall).is_some() {
                    p.roof_highlight.write(crate::resources::roofs::commands::HighlightRoofCmd::new(wall, crate::resources::walls::HighlightColor(1)));
                }
            } else {
                // B 1409e0922..a07 and iterator fold 14157f730: every
                // third mapped point produces a base/top pair.
                let points = wall_hover_preview_points(public);
                crate::utils::debug_viz::DebugLine::new(points)
                    .color(crate::utils::debug_viz::DebugColor::Custom(glam::Vec3::new(0.8, 0.0, 0.0)))
                    .line_width(3.0).dashed().show(&mut p.signifiers);
            }
        }
    }
    if !widget.clicked() { return; }
    let Some(public) = p.public.get(wall) else { return; };
    if !widget.is_active() { return; }
    let state = p.inner.get_unwrapped(wall);
    let mut lo = glam::Vec2::splat(f32::INFINITY);
    let mut hi = glam::Vec2::splat(f32::NEG_INFINITY);
    for point in public.geometry.outline.iter() {
        let point = glam::Vec2::new(point[0], point[2]);
        lo = lo.min(point);
        hi = hi.max(point);
    }
    let size = hi - lo;
    let area = size.x * size.y;
    let amount = utils::scalar::inverse_lerp(10.0, 250.0, area).clamp(0.0, 1.0);
    p.wall_audio.write(crate::systems::audio::PlayWallUiSound {
        kind: 0, selector: 0, value: amount, position: p.ui.intersection_point(),
    });
    let [low, high] = bulldozer_wall_range(public, state, p.ui.intersection_point());
    let original = state.snapshot().clone();
    let color = if state.source_flag_65() != 0 { p.timber_colors.0[wall.0 as usize] }
        else { p.wall_colors.0[wall.0 as usize] };
    let roof = p.public.roof_entity(wall).map(|entity| p.roofs.get(entity).expect("bulldozer roof entity missing Roof"));
    let roof_row = roof.map(|roof| *roof.observed());
    p.buffer.push(WallBulldozeCommand {
        original: original.clone(), wall: crate::resources::walls::publication_fields::WallId(wall.0),
        low, high, anchor: p.ui.intersection_point().to_array(), phase: 0.0, finished: false,
    });
    p.tool_used.write(crate::resources::onboarding::NotifyHintSystemToolWasJustUsed);
    p.erased.write(crate::resources::onboarding::NotifyHintSystemSomethingWasErased);
    p.decorators.archive_current_decorators_at_with_resources(DecoratorStorageAddr::Wall(wall), &p.storage, &p.archival, "bulldozer");
    p.decorators.decorator_cull_allow_wall(wall, &p.storage);
    // B 1409e197e..19bc archives the wall bucket before remove_storage.
    p.storage.remove_storage(DecoratorStorageAddr::Wall(wall));
    let history = super::bulldoze::record_wall_bulldoze(
        crate::resources::walls::publication_fields::WallId(wall.0), &original,
        low, high, color, roof_row.as_ref(),
    ).expect("invalid wall bulldoze history");
    if let Some(roof) = history.roof {
        p.roof_delete.write(crate::resources::roofs::commands::DeleteRoofCmd(wall));
        p.history.add(EditType::Roof(Arc::new(roof))).expect("bulldozer roof history requires recording");
    }
    p.history.add(EditType::Wall(Arc::new(history.wall))).expect("bulldozer wall history requires recording");
}

/// Public +0x88 is the prepared path; +0xe0 is its arch-height vector.
fn wall_hover_preview_points(public: &crate::resources::walls::public_wall_state::PublicWallState) -> Vec<glam::Vec3> {
    let heights = public.arch_segment_heights().unwrap_or(&[]);
    public.height_outline().iter().zip(heights).step_by(3)
        .flat_map(|(point, &height)| [glam::Vec3::from_array(*point), glam::Vec3::new(point[0], height, point[2])])
        .collect()
}

/// B 1409df103..265 / callback 1409e3340.
fn write_tree_bulldoze_signifier(stream: &mut crate::systems::ui::signifiers::signifier_create::UiSignifierStream,
    center: glam::Vec3) {
    use crate::systems::ui::signifiers::signifier_factory::{SignifierEntityBuilder, SignifierFactory};
    let name = format!("tree_bulldoze_{center:?}");
    stream.write_factory(&SignifierFactory::from_closure(&name, std::sync::Arc::new(move |meshes, shaders, commands| {
        vec![SignifierEntityBuilder::from_mesh_name("temp_sphere")
            .custom_shader("vertex_color_shader").position(center).scale(0.3)
            .animation_duration(0.1).build(meshes, shaders, commands)]
    })));
}

/// B 1409df738 and 1409e35a0: one owned cursor factory per frame.
pub fn hammer_cursor_icon(stream: &mut crate::systems::ui::signifiers::signifier_create::UiSignifierStream,
    cursor: glam::Vec2, active: bool) {
    use crate::systems::ui::signifiers::signifier_factory::{SignifierEntityBuilder, SignifierFactory, SignifierMesh};
    let name = format!("cursor_hammer_{cursor:?}");
    stream.write_factory(&SignifierFactory::from_closure(&name, std::sync::Arc::new(move |meshes, shaders, commands| {
        let entity = SignifierEntityBuilder::new(SignifierMesh::Empty).custom_shader("vertex_color_shader")
            .build(meshes, shaders, commands);
        commands.entity(entity).insert((crate::components::Ui2dExtent { pos: cursor, size: glam::Vec2::splat(60.0) },
            crate::components::Ui2dImage::new("hammer_cursor")));
        // B 1409e36ae conditionally queues the UiInactive bundle; the
        // unconditional 1409e37a5 bundle is Ui2DFollowMouse. The two bundle
        // registration type strings are at 143071aff and 143072c3a.
        if !active { commands.entity(entity).insert(crate::components::UiInactive); }
        commands.entity(entity).insert(crate::components::ui::Ui2DFollowMouse);
        vec![entity]
    })));
}

/// native 跳表目标 handler（缓冲入队，非 0x1409de190）：enqueue the bulldoze
/// command, skipping already-animating walls (the original scans
/// AnimateWallBulldozeBuffer by WallId).
pub fn enqueue_bulldoze_command(
    mut commands: EventReader<AnimateWallBulldozeCmd>,
    mut buffer: ResMut<AnimateWallBulldozeBuffer>,
    inner: Res<InnerWalls>,
) {
    for command in commands.read() {
        if buffer.contains(command.wall) {
            continue;
        }
        let Some(state) = inner.get(command.wall).cloned() else {
            continue;
        };
        buffer.push(WallBulldozeCommand {
            original: state.snapshot().clone(),
            wall: crate::resources::walls::publication_fields::WallId(command.wall.0),
            low: command.low,
            high: command.high,
            anchor: command.anchor,
            phase: 0.0,
            finished: false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_preview_uses_prepared_curve_and_nonuniform_published_heights() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{WallState, public_wall_state::PublicWallState,
            publication_source::TerrainHeightGrid512};
        use crate::systems::wall::wall_state::path_preparation::PathRasters512;
        use std::sync::Arc;
        let terrain = TerrainHeightGrid512 {
            encoded: (0..512 * 512).map(|i| ((i % 512) * 256).min(65535) as u16).collect::<Vec<_>>().into(),
            minimum_y: 0.0, maximum_y: 10.0, sampling_fix: false,
        };
        let authority = WallState::new(Curve2::try_new(vec![[-4.0, 0.0], [4.0, 0.0]], false).unwrap(),
            2.0, 0, 1, false);
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::<[u8]>::from(vec![255; 512 * 512])), false).unwrap();
        let mut public = PublicWallState::collect(&authority, &terrain).unwrap();
        let original = public.geometry.outline.clone();
        public.prepare_paths(&paths).unwrap();
        let prepared = public.height_outline();
        let heights = public.arch_segment_heights().unwrap();
        assert_ne!(prepared.len(), original.len(), "path threshold refinement must change the owner");
        assert_eq!(prepared.len(), heights.len());
        assert!(heights.windows(2).any(|h| h[0] != h[1]));
        let points = wall_hover_preview_points(&public);
        let indices: Vec<_> = (0..prepared.len()).step_by(3).collect();
        assert_eq!(points.len(), indices.len() * 2);
        for (pair, index) in points.chunks_exact(2).zip(indices) {
            assert_eq!(pair[0], glam::Vec3::from_array(prepared[index]));
            assert_eq!(pair[1], glam::Vec3::new(prepared[index][0], heights[index], prepared[index][2]));
        }
        assert_ne!(points[2], glam::Vec3::from_array(original[3]), "sampling must use prepared points");
    }

    #[test]
    fn hammer_factory_builds_real_follow_mouse_image_and_inactive_state() {
        use crate::asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary};
        use crate::render::{shader::RasterShaderProgram, test_device::TestDevice};
        use crate::systems::ui::signifiers::signifier_create::{UiSignifierEntry, UiSignifierStream};
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(RasterShaderProgram::new_draw("vertex_color_shader", Default::default(), &TestDevice(1)), false);
        for active in [false, true] {
            let mut stream = UiSignifierStream::default();
            hammer_cursor_icon(&mut stream, glam::Vec2::new(20.0, 30.0), active);
            let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else { panic!("missing cursor factory") };
            let mut world = World::new();
            let mut queue = bevy_ecs::world::CommandQueue::default();
            let entities = factory.wake(&meshes, &shaders, &mut bevy_ecs::system::Commands::new(&mut queue, &world));
            queue.apply(&mut world);
            let entity = entities[0];
            let extent = world.get::<crate::components::Ui2dExtent>(entity).unwrap();
            assert_eq!(extent.pos, glam::Vec2::new(20.0, 30.0));
            assert_eq!(extent.size, glam::Vec2::splat(60.0));
            assert_eq!(world.get::<crate::components::Ui2dImage>(entity).unwrap().name, "hammer_cursor");
            assert!(world.get::<crate::components::ui::Ui2DFollowMouse>(entity).is_some());
            assert_eq!(world.get::<crate::components::UiInactive>(entity).is_some(), !active);
        }
    }

    #[test]
    fn native_wall_widget_uses_hit_distance_and_real_click_state() {
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        use crate::systems::ui::resources::widget_params::WidgetGeometry;
        let wall = WallId(7);
        let id = WidgetId::from_name(&format!("bulldoze_wall_{wall}"));
        let ui = UiState::new(UiInteraction { native_tag: 2, native_phase: 1, widget: id });
        register_bulldozer_canvas(&ui);
        let context = register_bulldozer_wall_widget(&ui, wall, glam::Vec3::ZERO, glam::Vec3::Z * 4.0);
        assert!(context.clicked());
        let params = ui.widget(id).unwrap();
        assert!(matches!(params.resolved_geometry().as_ref(), WidgetGeometry::TerrainCanvas { distance, layer: 0 } if (*distance - 3.9).abs() < 0.00001));
        assert!(ui.widget(WidgetId::from_name("bulldoze_canvas")).is_some());
    }

    #[test]
    fn wall_range_tracks_hit_and_clamps_at_ends() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512};
        use crate::resources::walls::public_wall_state::PublicWallState;
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = WallState::new(curve, 2.0, 0, 0, false);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 4.0, sampling_fix: true,
        };
        let public = PublicWallState::collect(&wall, &terrain).unwrap();
        let middle = bulldozer_wall_range(&public, &wall, glam::Vec3::new(2.0, 0.0, 0.0));
        assert!((middle[0] - 0.475).abs() < 0.00001);
        assert!((middle[1] - 0.525).abs() < 0.00001);
        let start = bulldozer_wall_range(&public, &wall, glam::Vec3::ZERO);
        assert_eq!(start[0], 0.0);
        assert!((start[1] - 0.025).abs() < 0.00001);
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn real_rapier_wall_receipt_drives_bulldozer_pick() {
        use crate::systems::collision::world::{upsert_collider, ColliderMeshInstance, ColliderUpsert, RaycastMeshData, RaycastWorld};
        use crate::resources::mouse_terrain_raycast::CameraCursorRay;
        use std::{collections::HashMap, sync::Arc};
        let mut physics = RaycastWorld::new();
        let mut instances: HashMap<u32, ColliderMeshInstance> = HashMap::new();
        let mut bytes = [0u8; 24];
        bytes[8..16].copy_from_slice(&7u64.to_le_bytes());
        let source = crate::systems::collision::query_policy::RaycastGeoSourceProjection(bytes);
        assert!(matches!(upsert_collider(
            &mut physics, &mut instances, 1, source,
            Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE, glam::Vec3::ZERO)),
            glam::Vec3::Z * 4.0, glam::Quat::IDENTITY,
        ), ColliderUpsert::Inserted { .. }));
        physics.flush_colliders();
        let ray = CameraCursorRay { origin: glam::Vec3::ZERO, dir: glam::Vec3::Z };
        assert!(matches!(
            bulldozer_raycast(&physics, &ray, &AnimateWallBulldozeBuffer::new()),
            BulldozerColliderHit::Wall { wall: WallId(7), position } if position == glam::Vec3::Z * 3.5
        ));
    }

    #[test]
    fn bulldoze_layer_condition() {
        assert!(is_bulldozer_layer(&AppMode::Eraser(EraserLayer::Bulldoze)));
        assert!(!is_bulldozer_layer(&AppMode::Eraser(EraserLayer::Wall)));
        assert!(!is_bulldozer_layer(&AppMode::Wall(
            crate::systems::mode_manager::WallShape::Freeform
        )));
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn real_picker_click_records_wall_history_and_enqueues_animation() {
        use crate::resources::history::{EditType, History};
        use crate::resources::mouse_terrain_raycast::CameraCursorRay;
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512,
            public_walls::{PublicWalls, WallPublicationDirty}};
        use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
        use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
        use crate::systems::collision::world::{upsert_collider, ColliderMeshInstance, RaycastMeshData};
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        use std::{collections::HashMap, sync::Arc};

        let mut world = World::new();
        crate::systems::wall::registration::register_wall_cpu_resources(&mut world);
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(glam::Vec2::ZERO));
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall(WallState::new(curve, 2.0, 0, 0, false));
        let decorator = crate::resources::walls::decorator::DecoratorId(77);
        let info = crate::resources::walls::decorator::DecoratorInfo { decorator_id: 77, ty: 1, ..Default::default() };
        world.resource_mut::<crate::resources::walls::decorator_storage::DecoratorStorage>()
            .apply_decorator_place(DecoratorStorageAddr::Wall(wall), decorator, &info);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 4.0, sampling_fix: true };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.insert_resource(inner);
        world.insert_resource(public);
        let mut instances: HashMap<u32, ColliderMeshInstance> = HashMap::new();
        let mut receipt = [0u8; 24];
        receipt[8..16].copy_from_slice(&wall.0.to_le_bytes());
        {
            let mut physics = world.resource_mut::<crate::systems::collision::world::RaycastWorld>();
            upsert_collider(&mut physics, &mut instances, 1,
                crate::systems::collision::query_policy::RaycastGeoSourceProjection(receipt),
                Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE, glam::Vec3::ZERO)),
                glam::Vec3::new(2.0, 1.0, 0.0), glam::Quat::IDENTITY);
            physics.flush_colliders();
        }
        world.insert_resource(AppMode::Eraser(EraserLayer::Bulldoze));
        world.insert_resource(CameraCursorRay { origin: glam::Vec3::new(2.0, 1.0, -4.0), dir: glam::Vec3::Z });
        let id = WidgetId::from_name(&format!("bulldoze_wall_{wall}"));
        let mut ui = UiState::new(UiInteraction { native_tag: 2, native_phase: 1, widget: id });
        ui.set_intersection_point(glam::Vec3::new(2.0, 1.0, 0.0));
        world.insert_resource(ui);
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(9).unwrap();
        world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems(wall_eraser_bulldoze::<()>);
        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 1, native_phase: 0, widget: id });
        schedule.run(&mut world);
        let highlight = world.resource_mut::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>().drain().next().unwrap();
        assert_eq!((highlight.wall, highlight.ty), (wall, 1));
        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 1, widget: id });
        schedule.run(&mut world);
        assert!(world.resource::<AnimateWallBulldozeBuffer>().contains(wall));
        assert!(world.resource::<crate::resources::walls::decorator_storage::DecoratorStorage>().lookup_storage(decorator).is_none());
        assert!(world.resource::<crate::resources::walls::decorator_storage::DecoratorStorage>()
            .removed[&DecoratorStorageAddr::Wall(wall)].contains(&decorator.0));
        assert_eq!(world.resource::<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>()
            .archived_wall_decorator(wall, decorator.0), Some(info));
        assert!(world.resource::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>().is_empty());
        let audio = world.resource_mut::<Events<crate::systems::audio::PlayWallUiSound>>().drain().next().unwrap();
        assert_eq!((audio.kind, audio.position), (0, glam::Vec3::new(2.0, 1.0, 0.0)));
        assert!(matches!(world.resource::<History<()>>().pending().unwrap().edits.as_slice(), [EditType::Wall(_)]));
        schedule.run(&mut world);
        assert_eq!(world.resource::<AnimateWallBulldozeBuffer>().0.len(), 1);
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn real_picker_disconnects_internal_stair_segment_and_records_marker() {
        use crate::resources::history::{EditType, History};
        use crate::resources::mouse_terrain_raycast::CameraCursorRay;
        use crate::resources::stairs::stairs_state::StairsState;
        use crate::systems::collision::world::{upsert_collider, ColliderMeshInstance, RaycastMeshData};
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        use std::{collections::HashMap, sync::Arc};
        let mut world = World::new();
        crate::systems::wall::registration::register_wall_cpu_resources(&mut world);
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(glam::Vec2::ZERO));
        {
            let mut stairs = world.resource_mut::<StairsState>();
            for (a, b) in [(1, 2), (2, 3), (3, 4)] { stairs.add_edge(a, b); }
        }
        let mut bytes = [0u8; 24];
        bytes[0] = 13;
        bytes[4..8].copy_from_slice(&2u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&3u32.to_le_bytes());
        let mut instances: HashMap<u32, ColliderMeshInstance> = HashMap::new();
        {
            let mut physics = world.resource_mut::<crate::systems::collision::world::RaycastWorld>();
            upsert_collider(&mut physics, &mut instances, 1,
                crate::systems::collision::query_policy::RaycastGeoSourceProjection(bytes),
                Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE, glam::Vec3::ZERO)),
                glam::Vec3::Z * 4.0, glam::Quat::IDENTITY);
            physics.flush_colliders();
        }
        world.insert_resource(AppMode::Eraser(EraserLayer::Bulldoze));
        world.insert_resource(CameraCursorRay { origin: glam::Vec3::ZERO, dir: glam::Vec3::Z });
        let widget = WidgetId::from_name("bulldoze_stair_segment_StairGraphEdge(DecoratorId(2), DecoratorId(3))");
        world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: 1, widget }));
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(9).unwrap();
        world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems(wall_eraser_bulldoze::<()>);
        schedule.run(&mut world);
        let graph = world.resource::<StairsState>().graph();
        assert!(!graph.contains_edge(2, 3));
        assert!(graph.contains_edge(1, 2) && graph.contains_edge(3, 4));
        assert!(matches!(world.resource::<History<()>>().pending().unwrap().edits.as_slice(),
            [EditType::DecoratorV5(edit)] if edit.kind == 5));
        assert_eq!(world.resource_mut::<Events<crate::systems::audio::PlayStairsAudioCmd>>().drain().next().unwrap().kind, 4);
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn real_picker_removes_stair_support_and_records_replay_edit() {
        use crate::resources::history::{EditType, History, stairs_history_edit::StairsHistoryEdit};
        use crate::resources::mouse_terrain_raycast::CameraCursorRay;
        use crate::resources::stairs::{stairs_removed_supports::StairsRemovedSupports, stairs_state::StairGraphEdge};
        use crate::systems::collision::world::{upsert_collider, ColliderMeshInstance, RaycastMeshData};
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        use std::{collections::HashMap, sync::Arc};
        let mut world = World::new();
        crate::systems::wall::registration::register_wall_cpu_resources(&mut world);
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(glam::Vec2::ZERO));
        let edge = StairGraphEdge::new(12, 15);
        let mut bytes = [0u8; 24];
        bytes[0] = 14;
        bytes[4..8].copy_from_slice(&12u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&15u32.to_le_bytes());
        let mut instances: HashMap<u32, ColliderMeshInstance> = HashMap::new();
        {
            let mut physics = world.resource_mut::<crate::systems::collision::world::RaycastWorld>();
            upsert_collider(&mut physics, &mut instances, 1,
                crate::systems::collision::query_policy::RaycastGeoSourceProjection(bytes),
                Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE, glam::Vec3::ZERO)),
                glam::Vec3::Z * 4.0, glam::Quat::IDENTITY);
            physics.flush_colliders();
        }
        world.insert_resource(AppMode::Eraser(EraserLayer::Bulldoze));
        world.insert_resource(CameraCursorRay { origin: glam::Vec3::ZERO, dir: glam::Vec3::Z });
        let id = WidgetId::from_name("bulldoze_stair_support_StairGraphEdge(DecoratorId(12), DecoratorId(15))");
        world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: 1, widget: id }));
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(9).unwrap();
        world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems(wall_eraser_bulldoze::<()>);
        schedule.run(&mut world);
        assert!(world.resource::<StairsRemovedSupports>().contains(&edge));
        assert!(matches!(world.resource::<History<()>>().pending().unwrap().edits.as_slice(),
            [EditType::Stairs(edit)] if matches!(&**edit, StairsHistoryEdit::SupportsDelete(e) if *e == edge)));
        let audio = world.resource_mut::<Events<crate::systems::audio::PlayStairsAudioCmd>>()
            .drain().next().unwrap();
        assert_eq!(audio.kind, 0x11);
        assert_eq!(audio.position, glam::Vec3::ZERO);
        world.resource_mut::<History<()>>().finish_input_recording().unwrap();
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.insert_resource(crate::resources::time::GameTime::default());
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.init_resource::<crate::resources::walls::decorator::DecoratorIdGen>();
        world.init_resource::<crate::resources::walls::decorator::DeferredDecoratorOpEvents>();
        world.init_resource::<Events<crate::systems::history::HistoryReplayFailed>>();
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        world.init_resource::<Events<crate::systems::history::history_process::NewReplayCommandEvent>>();
        let mut replay = Schedule::default();
        replay.add_systems((crate::systems::history::process_history_cursor_commands::<()>,
            crate::systems::history::replay_stairs::replay_stairs,
            crate::systems::history::replay_pop_bucket_system).chain());
        for undo in [true, false] {
            world.resource_mut::<crate::resources::walls::decorator_storage::DecoratorStorage>().mark_mutable();
            world.resource_mut::<crate::systems::history::PendingHistoryCommands>().0.push_back(
                crate::systems::mode_manager::HistoryCommand { bytes: [0, u8::from(!undo), 1, 0] });
            replay.run(&mut world);
            assert!(world.resource::<crate::resources::history::ReplayBuffer>().is_empty());
            assert_eq!(world.resource::<StairsRemovedSupports>().contains(&edge), !undo);
        }
    }

    #[test]
    fn tree_widget_records_real_storage_identity_and_deletion() {
        use crate::components::TreeComponent;
        use crate::resources::history::{EditType, History, tree_edit::{DeleteTreeCmd, TreeEdit}};
        use crate::resources::mouse_terrain_raycast::CameraCursorRay;
        use crate::resources::placement_data::PlacementDataSsbo;
        use crate::resources::prefab::PrefabInstance;
        use crate::resources::tree_storage::{TreeStorage, TreeStorageEntry, TreeStorageIndex};
        use crate::systems::prefab::PrefabInstanceData;
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        let mut world = World::new();
        crate::systems::wall::registration::register_wall_cpu_resources(&mut world);
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(glam::Vec2::ZERO));
        let mut placement = PlacementDataSsbo::default();
        placement.position = glam::Vec3::new(2.0, 0.0, 3.0);
        placement.scale = 1.0;
        let entity = world.spawn((PrefabInstance::default(), PrefabInstanceData(placement),
            TreeComponent { id: 31, is_preplaced: true })).id();
        let index = TreeStorageIndex(31);
        world.resource_mut::<TreeStorage>().insert(index, TreeStorageEntry { entity, position: placement.position });
        world.insert_resource(AppMode::Eraser(EraserLayer::Bulldoze));
        world.insert_resource(CameraCursorRay { origin: glam::Vec3::ZERO, dir: glam::Vec3::Z });
        let id = WidgetId::from_name("delete_tree_TreeStorageIndex(31)");
        world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: 1, widget: id }));
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(9).unwrap();
        world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems(wall_eraser_bulldoze::<()>);
        schedule.run(&mut world);
        assert_eq!(world.resource_mut::<Events<DeleteTreeCmd>>().drain().next().unwrap().id, index.0);
        assert!(matches!(world.resource::<History<()>>().pending().unwrap().edits.as_slice(),
            [EditType::Tree(edit)] if matches!(&**edit, TreeEdit::Delete(data) if data.storage_index == Some(index) && data.is_preplaced)));
        world.resource_mut::<crate::DevOverrideIsDemoMode>().0 = true;
        schedule.run(&mut world);
        assert!(world.resource::<Events<DeleteTreeCmd>>().is_empty());
    }
}
