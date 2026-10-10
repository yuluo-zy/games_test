//! 墙路类型负责手势和整组安装；网格上传、预算与暂存回收复用公共流程。
//! 砖块按批次展示，不为每块砖创建实体。
use crate::{
    camera::OrbitCamera,
    editor::{DesktopEditor, EditorAction},
    pointer::{DesktopInputSet, PointerFrame},
    renderer::{BuildingInstall, MeshReadiness, Palette, UploadBudget, UploadLedger},
    upload::{UploadContext, UploadSource, UploadStage, release_mesh},
};
use bevy::{ecs::system::SystemParam, prelude::*};
use garden_application::{EditCommand, StrokeEdit, tools::Tool};
use garden_bevy::{
    CommandInbox, EditorState, GardenSet,
    strokes::{StrokeGenerationPlugin, StrokeTargets},
};
use garden_domain::strokes::{Point, Stroke, StrokeId, StrokeKind};
use garden_generation::{
    incremental::{PreparedBatch, PreparedBuilding},
    strokes::distance_to_segment,
};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Component, Clone, Copy)]
pub enum StrokeAction {
    HeightUp,
    HeightDown,
    Wider,
    Narrower,
    Delete,
}
#[derive(Resource, Default)]
pub struct StrokeTools {
    pub selected: Option<StrokeId>,
    pub auto_draw: bool,
    drag_revision: u64,
    drag: Option<Drag>,
    invalid: bool,
    truncated: bool,
}
enum Drag {
    Draw(Stroke),
    Move {
        original: Stroke,
        anchor: Point,
        revision: u64,
        candidate: Stroke,
    },
}
impl StrokeTools {
    pub fn cancel(&mut self) {
        self.drag = None;
    }
    fn candidate(&self) -> Option<&Stroke> {
        match &self.drag {
            Some(Drag::Draw(s)) => Some(s),
            Some(Drag::Move { candidate, .. }) => Some(candidate),
            None => None,
        }
    }
}
#[derive(Clone)]
struct Resident {
    mesh: Arc<PreparedBuilding>,
    handles: Vec<Handle<Mesh>>,
    entities: Vec<Entity>,
}
struct Stage {
    session: u64,
    handoff: crate::handoff::CoherentHandoff,
    fading: bool,
    next_materials:
        BTreeMap<(bool, garden_generation::mesh::MaterialKey), Handle<StandardMaterial>>,
    old_materials: BTreeMap<(bool, garden_generation::mesh::MaterialKey), Handle<StandardMaterial>>,
    old_bindings: Vec<(Entity, Handle<StandardMaterial>)>,
    warmups: Vec<Entity>,
    baseline: Option<BTreeMap<StrokeId, Resident>>,
    revision: u64,
    targets: BTreeMap<StrokeId, Arc<PreparedBuilding>>,
    upload: UploadStage<(StrokeId, usize)>,
    entities: BTreeMap<StrokeId, Vec<Entity>>,
    warm_frames: u8,
}
/// 道路统一路面和各段墙体采用复合索引；公共层不解释 StrokeId。
struct StrokeUploadSource<'a>(&'a BTreeMap<StrokeId, Arc<PreparedBuilding>>);
impl UploadSource for StrokeUploadSource<'_> {
    type Key = (StrokeId, usize);
    fn batches(&self) -> impl Iterator<Item = (Self::Key, &PreparedBatch)> {
        self.0.iter().flat_map(|(&id, mesh)| {
            mesh.batches
                .iter()
                .enumerate()
                .map(move |(i, b)| ((id, i), b))
        })
    }
    fn batch(&self, (id, i): Self::Key) -> &PreparedBatch {
        &self.0[&id].batches[i]
    }
}
#[derive(Resource, Default)]
pub struct StrokeDisplay {
    pub revision: Option<u64>,
    resident: BTreeMap<StrokeId, Resident>,
    stage: Option<Stage>,
    pub uploads: u64,
    pub bytes: u64,
    pub oversized_batches: u64,
}
impl StrokeDisplay {
    /// 其他结构的当前主导网格参与遮挡；地面另用同一高度场的 DDA 查询。
    pub fn nearest_structure_hit(&self, ray: Ray3d) -> Option<f32> {
        self.resident
            .iter()
            .filter(|(id, _)| !garden_generation::context_mesh::is_terrain(**id))
            .filter_map(|(_, r)| hit(ray, &r.mesh))
            .min_by(f32::total_cmp)
    }
    pub fn has_terrain(&self) -> bool {
        self.resident
            .keys()
            .any(|id| garden_generation::context_mesh::is_terrain(*id))
    }
}
pub struct StrokePlugin;
impl Plugin for StrokePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StrokeTools>()
            .init_resource::<StrokeDisplay>()
            .add_plugins((
                StrokeGenerationPlugin,
                crate::context_tools::ContextToolsPlugin,
            ))
            .add_systems(
                Update,
                (buttons, pointer)
                    .chain()
                    .in_set(DesktopInputSet::Tools)
                    .before(GardenSet::Commit),
            )
            .add_systems(
                Update,
                install.after(BuildingInstall).after(GardenSet::Commit),
            )
            .add_systems(Update, overlay.after(GardenSet::Commit));
    }
}
type Action<'a> = (
    &'a Interaction,
    Option<&'a StrokeAction>,
    Option<&'a EditorAction>,
    Option<&'a crate::camera::CameraAction>,
);
fn buttons(
    actions: Query<Action<'_>, Changed<Interaction>>,
    context: Res<garden_bevy::context::ContextProjection>,
    editor: Res<EditorState>,
    mut tool: ResMut<StrokeTools>,
    mut state: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    for (interaction, action, editor_action, camera_action) in &actions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if action.is_some() || editor_action.is_some() || camera_action.is_some() {
            tool.drag = None;
            if editor_action.is_some() || camera_action.is_some() {
                tool.auto_draw = false;
            }
        }
        let Some(action) = action else {
            continue;
        };
        state.tools.cancel();
        if context.busy {
            state.status = "上一笔修改正在解析，请稍候。".into();
            continue;
        }
        let Some(id) = tool.selected else {
            state.status = "请用「墙路编辑」先选择一条墙或路。".into();
            continue;
        };
        let Some(stroke) = editor.editor().stroke(id) else {
            tool.selected = None;
            continue;
        };
        let revision = editor.editor().stroke_revision();
        let mut stroke = stroke.as_ref().clone();
        let context_intent = editor.editor().context().linear.get(&id).cloned();
        let automatic = context_intent
            .as_ref()
            .is_some_and(|i| matches!(i.mode, garden_domain::context::StructureMode::Auto));
        if matches!(action, StrokeAction::Delete) {
            state.submit(
                EditCommand::Stroke(StrokeEdit::Delete {
                    id,
                    expected_revision: revision,
                }),
                &mut inbox,
            );
            continue;
        }
        let step = if automatic { 0.1 } else { 0.4 };
        match action {
            StrokeAction::HeightUp => stroke.height = (stroke.height + step).min(5.),
            StrokeAction::HeightDown => stroke.height = (stroke.height - step).max(0.),
            StrokeAction::Wider => stroke.width += 0.2,
            StrokeAction::Narrower => stroke.width -= 0.2,
            _ => {}
        }
        if let Some(intent) = context_intent {
            state.submit(
                EditCommand::Context(garden_application::context::ContextEdit::Guarded {
                    revision: editor.editor().world_revision(),
                    edit: Box::new(garden_application::context::ContextEdit::Stroke {
                        stroke,
                        intent,
                    }),
                }),
                &mut inbox,
            );
        } else {
            if matches!(action, StrokeAction::HeightUp | StrokeAction::HeightDown)
                && stroke.kind == StrokeKind::Path
            {
                state.status = "请先启用「自动三态」，再改变道路的结构高度。".into();
                continue;
            }
            state.submit(
                EditCommand::Stroke(StrokeEdit::Replace {
                    stroke,
                    expected_revision: revision,
                }),
                &mut inbox,
            );
        }
    }
}
fn hit(ray: Ray3d, mesh: &PreparedBuilding) -> Option<f32> {
    mesh.batches
        .iter()
        .flat_map(|b| {
            b.data.indices.chunks_exact(3).filter_map(|t| {
                let p = &b.data.positions;
                crate::picking::triangle_hit(
                    ray.origin,
                    *ray.direction,
                    Vec3::from_array(p[t[0] as usize]),
                    Vec3::from_array(p[t[1] as usize]),
                    Vec3::from_array(p[t[2] as usize]),
                )
            })
        })
        .min_by(f32::total_cmp)
}
#[derive(SystemParam)]
struct StrokeWorld<'w, 's> {
    frame: Res<'w, PointerFrame>,
    editor: Res<'w, EditorState>,
    semantic: Res<'w, garden_bevy::context::ContextProjection>,
    cameras: Query<'w, 's, (&'static Camera, &'static Transform), With<OrbitCamera>>,
    display: Res<'w, StrokeDisplay>,
}
fn pointer(
    world: StrokeWorld,
    mut tool: ResMut<StrokeTools>,
    mut context_tools: Option<ResMut<crate::context_tools::ContextTools>>,
    mut state: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    let StrokeWorld {
        frame,
        editor,
        semantic,
        cameras,
        display,
    } = world;
    if context_tools.as_ref().is_some_and(|c| c.owns_pointer()) {
        return;
    }
    if frame.cancel_world {
        tool.drag = None;
        return;
    }
    if !matches!(
        state.tools.tool(),
        Tool::Wall | Tool::Path | Tool::StrokeSelect
    ) || state.pending.is_some()
    {
        tool.drag = None;
        return;
    }
    if !(frame.world_begin || frame.world_held || frame.world_finish) {
        return;
    }
    let ray = frame.cursor.and_then(|p| {
        cameras
            .iter()
            .next()
            .and_then(|(c, t)| c.viewport_to_world(&GlobalTransform::from(*t), p).ok())
    });
    let Some(ray) = ray else {
        tool.invalid = true;
        if frame.world_finish {
            tool.drag = None;
        }
        return;
    };
    let Some(point) = crate::context_tools::ground(ray, &editor.editor().context().terrain) else {
        tool.invalid = true;
        if frame.world_finish {
            tool.drag = None;
        }
        return;
    };
    if frame.world_begin {
        if semantic.busy || display.revision != Some(editor.editor().world_revision()) {
            state.status = "结构正在重构，请稍候再开始拖动。".into();
            return;
        }
        tool.drag_revision = editor.editor().world_revision();
        tool.invalid = false;
        tool.truncated = false;
        if state.tools.tool() == Tool::StrokeSelect {
            let nearest = display
                .resident
                .iter()
                .filter(|(id, _)| {
                    id.0 < (1 << 60) || garden_generation::context_mesh::is_road(**id)
                })
                .filter_map(|(id, r)| hit(ray, &r.mesh).map(|d| (*id, d)))
                .min_by(|a, b| a.1.total_cmp(&b.1));
            let selected = nearest.and_then(|(id, distance)| {
                let hit = ray.get_point(distance);
                let picked = Point {
                    x: hit.x as f64,
                    z: hit.z as f64,
                };
                if !garden_generation::context_mesh::is_road(id) {
                    Some(id)
                } else {
                    editor
                        .editor()
                        .strokes()
                        .filter(|s| {
                            semantic
                                .semantic
                                .linear
                                .get(&s.id)
                                .map_or(s.kind == StrokeKind::Path, |l| l.kind == StrokeKind::Path)
                        })
                        .map(|s| {
                            let horizontal = s
                                .points
                                .windows(2)
                                .map(|p| distance_to_segment(picked, p[0], p[1]))
                                .fold(f64::INFINITY, f64::min);
                            let offset = semantic
                                .semantic
                                .linear
                                .get(&s.id)
                                .map_or(0., |l| l.elevation_offset);
                            // 使用实际可见路面的高度，避免二维交叉时选中下层道路。
                            let vertical = editor.editor().context().terrain.height(picked)
                                + offset
                                - hit.y as f64;
                            (s.id, horizontal.hypot(vertical))
                        })
                        .min_by(|a, b| a.1.total_cmp(&b.1))
                        .map(|(id, _)| id)
                }
            });
            tool.selected = selected;
            if display.revision != Some(editor.editor().world_revision()) {
                state.status = "墙路仍在重构，请等待后重新拖动。".into();
                return;
            }
            tool.drag = selected
                .and_then(|id| editor.editor().stroke(id))
                .map(|s| Drag::Move {
                    original: s.as_ref().clone(),
                    candidate: s.as_ref().clone(),
                    anchor: point,
                    revision: editor.editor().stroke_revision(),
                });
            state.status = selected.map_or("未选中墙或路。".into(), |id| {
                format!("已选墙路 #{}；拖动移动，按钮调整或删除。", id.0)
            });
        } else {
            let Ok(id) = editor.editor().next_stroke_id() else {
                state.status = "墙路编号已耗尽。".into();
                return;
            };
            let kind = if state.tools.tool() == Tool::Wall {
                StrokeKind::Wall
            } else {
                StrokeKind::Path
            };
            tool.drag = Some(Drag::Draw(Stroke {
                id,
                kind,
                points: vec![point],
                width: if kind == StrokeKind::Wall || tool.auto_draw {
                    0.4
                } else {
                    1.4
                },
                height: if kind == StrokeKind::Path { 0. } else { 2.4 },
            }));
        }
    }
    match &mut tool.drag {
        Some(Drag::Draw(s)) => {
            if s.points.last().unwrap().distance(point) >= 0.2
                || (frame.world_finish && s.points.last().unwrap().distance(point) >= 0.04)
            {
                // Bound gesture memory too, not only the committed model.
                if s.points.len() >= 128 {
                    tool.invalid = true;
                    tool.truncated = true;
                    if frame.world_finish {
                        tool.drag = None;
                        state.status = "轨迹采样已达128点，未提交；请分段绘制。".into();
                    }
                    return;
                }
                s.points.push(point);
            }
        }
        Some(Drag::Move {
            original,
            anchor,
            candidate,
            ..
        }) => {
            candidate.points = original
                .points
                .iter()
                .map(|p| Point {
                    x: p.x + point.x - anchor.x,
                    z: p.z + point.z - anchor.z,
                })
                .collect();
        }
        None => return,
    }
    tool.invalid = tool.truncated || tool.candidate().is_none_or(|s| s.validate().is_err());
    let context_edit = tool.candidate().map(|stroke| {
        let mut intent = editor
            .editor()
            .context()
            .linear
            .get(&stroke.id)
            .cloned()
            .unwrap_or_else(|| garden_domain::context::LinearIntent::legacy(stroke));
        if tool.auto_draw {
            intent.mode = garden_domain::context::StructureMode::Auto;
        }
        garden_application::context::ContextEdit::Stroke {
            stroke: stroke.clone(),
            intent,
        }
    });
    if !tool.invalid
        && let (Some(edit), Some(preview)) = (context_edit.clone(), context_tools.as_mut())
        && let Ok(candidate) = editor.editor().prepare(EditCommand::Context(edit))
    {
        preview.preview_input(
            garden_bevy::context::input(candidate.snapshot),
            tool.drag_revision,
        );
    }
    if frame.world_finish {
        let invalid = tool.invalid;
        let edit = match tool.drag.take() {
            Some(Drag::Draw(s)) => Some((s.id, StrokeEdit::Create(s))),
            Some(Drag::Move {
                candidate,
                revision,
                ..
            }) => Some((
                candidate.id,
                StrokeEdit::Replace {
                    stroke: candidate,
                    expected_revision: revision,
                },
            )),
            None => None,
        };
        if invalid {
            state.status = "轨迹无效，未提交；不能越出庭院，至少拖动0.04米。".into();
        } else if let Some((id, edit)) = edit {
            tool.selected = Some(id);
            if let (Some(context_edit), Some(preview)) = (context_edit, context_tools.as_mut()) {
                preview.finish_edit(context_edit);
                state.status = "候选已保留，等待上下文解析后提交。".into();
            } else {
                state.submit(EditCommand::Stroke(edit), &mut inbox);
            }
        }
    }
}

#[derive(SystemParam)]
struct InstallAssets<'w> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    palette: Res<'w, Palette>,
    bridge: Res<'w, MeshReadiness>,
    budget: Res<'w, UploadBudget>,
    ledger: ResMut<'w, UploadLedger>,
    ground_material: Option<Res<'w, crate::context_tools::GroundMaterial>>,
}
fn release(resident: Resident, commands: &mut Commands, assets: &mut InstallAssets) {
    for e in resident.entities {
        assets.bridge.forget_draw(e);
        commands.entity(e).despawn();
    }
    for h in resident.handles {
        release_mesh(&mut assets.meshes, &assets.bridge, &h);
    }
}
fn discard(stage: Stage, commands: &mut Commands, assets: &mut InstallAssets) {
    for (e, h) in stage.old_bindings {
        commands.entity(e).insert(MeshMaterial3d(h));
    }
    for ghost in stage.warmups {
        assets.bridge.forget_draw(ghost);
        commands.entity(ghost).despawn();
    }
    for h in stage
        .next_materials
        .into_values()
        .chain(stage.old_materials.into_values())
    {
        assets.materials.remove(h.id());
    }
    for entities in stage.entities.into_values() {
        for e in entities {
            assets.bridge.forget_draw(e);
            commands.entity(e).despawn();
        }
    }
    stage.upload.discard(&mut assets.meshes, &assets.bridge);
}
fn material_for(
    id: StrokeId,
    key: garden_generation::mesh::MaterialKey,
    assets: &InstallAssets,
) -> Handle<StandardMaterial> {
    if garden_generation::context_mesh::is_terrain(id) {
        assets
            .ground_material
            .as_ref()
            .map_or_else(|| assets.palette.0[&key].clone(), |m| m.0.clone())
    } else {
        assets.palette.0[&key].clone()
    }
}
fn fade_material(
    id: StrokeId,
    key: garden_generation::mesh::MaterialKey,
    alpha: f32,
    cache: &mut BTreeMap<(bool, garden_generation::mesh::MaterialKey), Handle<StandardMaterial>>,
    assets: &mut InstallAssets,
) -> Handle<StandardMaterial> {
    cache
        .entry((garden_generation::context_mesh::is_terrain(id), key))
        .or_insert_with(|| {
            let mut material = assets
                .materials
                .get(&material_for(id, key, assets))
                .unwrap()
                .clone();
            material.alpha_mode = AlphaMode::Blend;
            material.base_color = material.base_color.with_alpha(alpha);
            assets.materials.add(material)
        })
        .clone()
}
fn publish_pick(stage: &mut Stage, display: &mut StrokeDisplay) {
    let mut next = BTreeMap::new();
    for (&id, mesh) in &stage.targets {
        if let Some(old) = display
            .resident
            .get(&id)
            .filter(|r| Arc::ptr_eq(&r.mesh, mesh))
        {
            next.insert(id, old.clone());
        } else {
            next.insert(
                id,
                Resident {
                    mesh: mesh.clone(),
                    handles: (0..mesh.batches.len())
                        .map(|i| stage.upload.handles[&(id, i)].clone())
                        .collect(),
                    entities: stage.entities.get(&id).cloned().unwrap_or_default(),
                },
            );
        }
    }
    stage.baseline = Some(std::mem::replace(&mut display.resident, next));
}
fn install(
    mut commands: Commands,
    editor: Res<EditorState>,
    targets: Res<StrokeTargets>,
    mut display: ResMut<StrokeDisplay>,
    mut assets: InstallAssets,
    time: Option<Res<Time>>,
    mut groups: Option<ResMut<garden_bevy::context::PublicationGroups>>,
) {
    let revision = editor.editor().world_revision();
    let session = editor.editor().session();
    let obsolete = display
        .stage
        .as_ref()
        .is_some_and(|s| s.session != session || (s.revision != revision && !s.handoff.started));
    if obsolete {
        let mut stage = display.stage.take().unwrap();
        if let Some(old) = stage.baseline.take() {
            display.resident = old;
        }
        discard(stage, &mut commands, &mut assets);
    }
    let active = display.stage.as_ref().is_some_and(|s| s.handoff.started);
    if !active
        && (!targets.ready
            || targets.revision != revision
            || targets.error.is_some()
            || display.revision == Some(revision))
    {
        return;
    }
    if display.stage.is_none() {
        let upload = UploadStage::new(&StrokeUploadSource(&targets.meshes), |(id, i), _| {
            display
                .resident
                .get(&id)
                .filter(|r| Arc::ptr_eq(&r.mesh, &targets.meshes[&id]))
                .map(|r| r.handles[i].clone())
        });
        let changes = !display.resident.is_empty()
            && (display.resident.iter().any(|(id, r)| {
                targets
                    .meshes
                    .get(id)
                    .is_none_or(|m| !Arc::ptr_eq(&r.mesh, m))
            }) || targets.meshes.iter().any(|(id, m)| {
                display
                    .resident
                    .get(id)
                    .is_none_or(|r| !Arc::ptr_eq(&r.mesh, m))
            }));
        display.stage = Some(Stage {
            revision,
            session,
            targets: targets.meshes.clone(),
            upload,
            entities: BTreeMap::new(),
            warm_frames: 0,
            handoff: crate::handoff::CoherentHandoff::default(),
            fading: time.is_some() && changes,
            next_materials: BTreeMap::new(),
            old_materials: BTreeMap::new(),
            old_bindings: Vec::new(),
            warmups: Vec::new(),
            baseline: None,
        });
    }
    let mut stage = display.stage.take().unwrap();
    let progress = {
        let InstallAssets {
            meshes,
            bridge,
            budget,
            ledger,
            ..
        } = &mut assets;
        stage.upload.advance(
            &StrokeUploadSource(&stage.targets),
            &mut UploadContext {
                meshes,
                bridge,
                budget: **budget,
                ledger,
            },
        )
    };
    display.bytes += progress.bytes as u64;
    display.oversized_batches += progress.oversized as u64;
    if !stage.upload.ready(&assets.bridge) {
        display.stage = Some(stage);
        return;
    }
    if stage.warm_frames == 0 {
        let entries = stage
            .upload
            .handles
            .iter()
            .map(|(&key, h)| (key, h.clone()))
            .collect::<Vec<_>>();
        for ((id, i), handle) in entries {
            if display
                .resident
                .get(&id)
                .is_some_and(|r| Arc::ptr_eq(&r.mesh, &stage.targets[&id]))
            {
                continue;
            }
            let key = stage.targets[&id].batches[i].material;
            let material = if stage.fading {
                fade_material(id, key, 0., &mut stage.next_materials, &mut assets)
            } else {
                material_for(id, key, &assets)
            };
            let e = commands
                .spawn((
                    Mesh3d(handle),
                    MeshMaterial3d(material),
                    Transform::IDENTITY,
                ))
                .id();
            assets.bridge.track_draw(e);
            stage.entities.entry(id).or_default().push(e);
        }
        if stage.fading {
            for (&id, resident) in &display.resident {
                if stage
                    .targets
                    .get(&id)
                    .is_some_and(|m| Arc::ptr_eq(m, &resident.mesh))
                {
                    continue;
                }
                for (i, &entity) in resident.entities.iter().enumerate() {
                    let key = resident.mesh.batches[i].material;
                    stage
                        .old_bindings
                        .push((entity, material_for(id, key, &assets)));
                    if !stage
                        .old_materials
                        .contains_key(&(garden_generation::context_mesh::is_terrain(id), key))
                    {
                        let material =
                            fade_material(id, key, 0., &mut stage.old_materials, &mut assets);
                        let ghost = commands
                            .spawn((
                                Mesh3d(resident.handles[i].clone()),
                                MeshMaterial3d(material),
                                Transform::IDENTITY,
                            ))
                            .id();
                        assets.bridge.track_draw(ghost);
                        stage.warmups.push(ghost);
                    }
                }
            }
        }
    }
    stage.warm_frames = stage.warm_frames.saturating_add(1);
    let dt = time.as_ref().map_or(1., |t| t.delta_secs().min(0.1));
    let ready = stage.warm_frames > 2
        && stage.handoff.pipeline_ready(
            dt,
            assets.bridge.enabled,
            assets.bridge.draws_ready(
                stage
                    .entities
                    .values()
                    .flatten()
                    .copied()
                    .chain(stage.warmups.iter().copied()),
            ),
        );
    if ready && let Some(groups) = &mut groups {
        groups.linear_ready(stage.revision);
    }
    let barrier = groups.as_ref().is_none_or(|g| g.ready(stage.revision));
    let elapsed = time.as_ref().and_then(|t| {
        groups
            .as_mut()
            .and_then(|g| g.elapsed(stage.revision, t.elapsed_secs_f64()))
    });
    let Some(step) = stage.handoff.advance(
        dt,
        if stage.fading { 0.16 } else { 0.001 },
        ready && barrier,
        elapsed,
    ) else {
        display.stage = Some(stage);
        return;
    };
    if step.started_now {
        for handle in stage.old_materials.values() {
            if let Some(mut m) = assets.materials.get_mut(handle) {
                m.base_color = m.base_color.with_alpha(1.);
            }
        }
        for (&id, resident) in &display.resident {
            if stage
                .targets
                .get(&id)
                .is_some_and(|m| Arc::ptr_eq(m, &resident.mesh))
            {
                continue;
            }
            for (i, &entity) in resident.entities.iter().enumerate() {
                if stage.fading {
                    let key = (
                        garden_generation::context_mesh::is_terrain(id),
                        resident.mesh.batches[i].material,
                    );
                    commands
                        .entity(entity)
                        .insert(MeshMaterial3d(stage.old_materials[&key].clone()));
                }
            }
        }
        for ghost in stage.warmups.drain(..) {
            assets.bridge.forget_draw(ghost);
            commands.entity(ghost).despawn();
        }
    }
    for h in stage.next_materials.values() {
        if let Some(mut m) = assets.materials.get_mut(h) {
            m.base_color = m.base_color.with_alpha(step.alpha);
        }
    }
    for h in stage.old_materials.values() {
        if let Some(mut m) = assets.materials.get_mut(h) {
            m.base_color = m.base_color.with_alpha(1. - step.alpha);
        }
    }
    if step.switched_now {
        publish_pick(&mut stage, &mut display);
    }
    if !step.complete {
        display.stage = Some(stage);
        return;
    }
    if stage.baseline.is_none() {
        publish_pick(&mut stage, &mut display);
    }
    for (id, old) in stage.baseline.take().unwrap() {
        if display
            .resident
            .get(&id)
            .is_some_and(|r| Arc::ptr_eq(&r.mesh, &old.mesh))
        {
            continue;
        }
        release(old, &mut commands, &mut assets);
    }
    for (&id, entities) in &stage.entities {
        for (i, &e) in entities.iter().enumerate() {
            let key = stage.targets[&id].batches[i].material;
            commands
                .entity(e)
                .insert(MeshMaterial3d(material_for(id, key, &assets)));
            assets.bridge.forget_draw(e);
        }
    }
    for h in stage
        .next_materials
        .into_values()
        .chain(stage.old_materials.into_values())
    {
        assets.materials.remove(h.id());
    }
    display.revision = Some(stage.revision);
    display.uploads += 1;
}
fn overlay(
    context_tools: Option<Res<crate::context_tools::ContextTools>>,
    mut tools: ResMut<StrokeTools>,
    editor: Res<EditorState>,
    semantic: Res<garden_bevy::context::ContextProjection>,
    state: Res<DesktopEditor>,
    mut gizmos: Gizmos,
) {
    if tools
        .selected
        .is_some_and(|id| editor.editor().stroke(id).is_none()
            && !context_tools.as_ref().is_some_and(|c|c.pending_stroke==Some(id))
            && !matches!(&state.pending, Some(EditCommand::Stroke(StrokeEdit::Create(s))) if s.id == id))
    {
        tools.selected = None;
    }
    if tools.drag.is_some() && tools.drag_revision != editor.editor().world_revision() {
        tools.drag = None;
    }
    let candidate = tools.candidate();
    let selected = tools.selected.and_then(|id| editor.editor().stroke(id));
    let stroke = candidate.or_else(|| selected.map(AsRef::as_ref));
    if let Some(s) = stroke {
        let color = if tools.invalid && candidate.is_some() {
            Color::srgb_u8(240, 91, 73)
        } else {
            Color::srgb_u8(58, 213, 219)
        };
        let parsed = context_tools
            .as_ref()
            .and_then(|t| t.preview.as_ref())
            .map_or(&semantic.semantic, |p| p);
        let line = parsed.linear.get(&s.id);
        let kind = line.map_or(s.kind, |l| l.kind);
        let offset = line.map_or(0., |l| l.elevation_offset);
        let terrain = &editor.editor().context().terrain;
        let points = line.map_or(s.points.as_slice(), |l| l.points.as_slice());
        for p in points.windows(2) {
            let a = Vec3::new(
                p[0].x as f32,
                (terrain.height(p[0]) + offset + 0.05) as f32,
                p[0].z as f32,
            );
            let b = Vec3::new(
                p[1].x as f32,
                (terrain.height(p[1]) + offset + 0.05) as f32,
                p[1].z as f32,
            );
            gizmos.line(a, b, color);
            if kind != StrokeKind::Path {
                let up = Vec3::Y * s.height as f32;
                gizmos.line(a + up, b + up, color);
                gizmos.line(a, a + up, color);
                gizmos.line(b, b + up, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{catalog::ArtCatalog, renderer::PresentationPlugin};
    use garden_bevy::{GardenPlugin, Operation};
    use std::time::{Duration, Instant};

    fn app() -> App {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<DesktopEditor>()
            .init_resource::<PointerFrame>()
            .init_resource::<StrokeTools>()
            .init_resource::<StrokeDisplay>()
            .add_plugins((
                GardenPlugin::default(),
                PresentationPlugin::new(ArtCatalog::warm_stone()),
                StrokeGenerationPlugin,
            ))
            .add_systems(Update, (buttons, pointer).chain().before(GardenSet::Commit))
            .add_systems(
                Update,
                install.after(BuildingInstall).after(GardenSet::Commit),
            );
        let orbit = OrbitCamera::from_view(Vec3::new(10., 15., 20.), Vec3::ZERO).unwrap();
        let mut camera = Camera::default();
        camera.computed.clip_from_view =
            Mat4::perspective_infinite_reverse_rh(std::f32::consts::PI / 3., 1.5, 0.1);
        camera.computed.target_info = Some(bevy::camera::RenderTargetInfo {
            physical_size: UVec2::new(1440, 960),
            scale_factor: 1.,
        });
        app.world_mut().spawn((camera, orbit.transform(), orbit));
        settle(&mut app);
        app
    }
    fn frame(app: &mut App, point: Vec3, begin: bool, finish: bool) {
        let mut q = app.world_mut().query::<(&Camera, &Transform)>();
        let (c, t) = q.iter(app.world()).next().unwrap();
        let cursor = c
            .world_to_viewport(&GlobalTransform::from(*t), point)
            .unwrap();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cursor: Some(cursor),
            world_begin: begin,
            world_held: !finish,
            world_finish: finish,
            ..default()
        };
        app.update();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame::default();
    }
    fn settle(app: &mut App) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            app.update();
            let world = app.world();
            let revision = world.resource::<EditorState>().editor().world_revision();
            let ledger = world.resource::<UploadLedger>();
            let budget = world.resource::<UploadBudget>();
            assert!(
                ledger.bytes <= budget.bytes_per_frame
                    && ledger.batches <= budget.batches_per_frame
            );
            if world.resource::<StrokeDisplay>().revision == Some(revision)
                && !world
                    .resource::<garden_bevy::context::ContextProjection>()
                    .busy
            {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "wall/path projection did not settle"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    fn draw(app: &mut App, tool: Tool, a: Vec3, b: Vec3) {
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(tool);
        frame(app, a, true, false);
        frame(app, b, false, true);
        settle(app);
    }
    fn operation(app: &mut App, op: Operation) {
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(op)
            .unwrap();
        app.update();
        settle(app);
    }
    #[test]
    fn tiny_shared_budget_still_publishes_wall_and_house_and_reclaims_wall() {
        use garden_domain::{BuildingId, sample_building};
        let mut app = app();
        app.insert_resource(UploadBudget {
            bytes_per_frame: 1,
            batches_per_frame: 1,
        });
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Edit(EditCommand::Create(sample_building(
                BuildingId::new(1).unwrap(),
                6.,
                3.,
            ))))
            .unwrap();
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Edit(EditCommand::Stroke(StrokeEdit::Create(
                Stroke {
                    id: StrokeId(1),
                    kind: StrokeKind::Wall,
                    points: vec![Point { x: -5., z: 0. }, Point { x: 5., z: 0. }],
                    width: 0.4,
                    height: 2.4,
                },
            ))))
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            app.update();
            let world = app.world();
            let ledger = world.resource::<UploadLedger>();
            assert!(ledger.batches <= 1, "不同类型不能各自获得一份预算");
            let revision = world.resource::<EditorState>().editor().world_revision();
            if world.resource::<StrokeDisplay>().revision == Some(revision)
                && world.resource::<crate::renderer::RenderStats>().buildings == 1
            {
                break;
            }
            assert!(Instant::now() < deadline, "超大批次必须能够最终推进");
            std::thread::sleep(Duration::from_millis(1));
        }
        let wall = app.world().resource::<StrokeDisplay>().resident[&StrokeId(1)].clone();
        assert!(app.world().resource::<StrokeDisplay>().oversized_batches > 0);
        assert!(
            app.world()
                .resource::<crate::renderer::RenderStats>()
                .oversized_batches
                > 0
        );
        let revision = app
            .world()
            .resource::<EditorState>()
            .editor()
            .stroke_revision();
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Edit(EditCommand::Stroke(StrokeEdit::Delete {
                id: StrokeId(1),
                expected_revision: revision,
            })))
            .unwrap();
        // 删除不上传新网格，恢复正常预算后沿用已有的收敛检查。
        app.insert_resource(UploadBudget::default());
        settle(&mut app);
        assert!(
            wall.handles
                .iter()
                .all(|h| app.world().resource::<Assets<Mesh>>().get(h).is_none())
        );
        assert_eq!(
            app.world()
                .resource::<crate::renderer::RenderStats>()
                .buildings,
            1
        );
    }
    #[test]
    fn gestures_holes_deletion_undo_and_remote_gpu_reuse_use_the_real_pipeline() {
        let mut app = app();
        let terrain_meshes = app.world().resource::<Assets<Mesh>>().len();
        draw(
            &mut app,
            Tool::Wall,
            Vec3::new(-5., 0., 0.),
            Vec3::new(5., 0., 0.),
        );
        draw(
            &mut app,
            Tool::Wall,
            Vec3::new(-5., 0., 9.),
            Vec3::new(5., 0., 9.),
        );
        let remote = app.world().resource::<StrokeDisplay>().resident[&StrokeId(2)].clone();
        let ray = Ray3d::new(Vec3::new(0.1, 0.7, 5.), Dir3::NEG_Z);
        assert!(
            hit(
                ray,
                &app.world().resource::<StrokeDisplay>().resident[&StrokeId(1)].mesh
            )
            .is_some()
        );
        draw(
            &mut app,
            Tool::Path,
            Vec3::new(0., 0., -3.),
            Vec3::new(0., 0., 3.),
        );
        let display = app.world().resource::<StrokeDisplay>();
        assert!(
            hit(ray, &display.resident[&StrokeId(1)].mesh).is_none(),
            "arch must be a real pickable hole"
        );
        assert!(Arc::ptr_eq(
            &remote.mesh,
            &display.resident[&StrokeId(2)].mesh
        ));
        assert_eq!(remote.handles, display.resident[&StrokeId(2)].handles);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            3
        );
        let revision = app
            .world()
            .resource::<EditorState>()
            .editor()
            .stroke_revision();
        operation(
            &mut app,
            Operation::Edit(EditCommand::Stroke(StrokeEdit::Delete {
                id: StrokeId(3),
                expected_revision: revision,
            })),
        );
        assert!(
            hit(
                ray,
                &app.world().resource::<StrokeDisplay>().resident[&StrokeId(1)].mesh
            )
            .is_some()
        );
        operation(&mut app, Operation::Edit(EditCommand::Undo));
        assert!(
            hit(
                ray,
                &app.world().resource::<StrokeDisplay>().resident[&StrokeId(1)].mesh
            )
            .is_none()
        );
        operation(&mut app, Operation::Edit(EditCommand::Redo));
        assert!(
            hit(
                ray,
                &app.world().resource::<StrokeDisplay>().resident[&StrokeId(1)].mesh
            )
            .is_some()
        );
        operation(&mut app, Operation::ReplaceScene(vec![]));
        assert!(
            app.world()
                .resource::<StrokeDisplay>()
                .resident
                .keys()
                .all(|id| garden_generation::context_mesh::is_terrain(*id))
        );
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), terrain_meshes);
        assert!(
            remote
                .handles
                .iter()
                .all(|h| app.world().resource::<Assets<Mesh>>().get(h).is_none())
        );
    }
    #[test]
    fn visible_upper_road_keeps_its_source_at_a_height_separated_crossing() {
        use garden_application::context::ContextEdit;
        use garden_domain::context::LinearIntent;
        let mut app = app();
        for (id, offset) in [(1, 0.), (2, 1.)] {
            let stroke = Stroke {
                id: StrokeId(id),
                kind: StrokeKind::Path,
                points: vec![Point { x: -4., z: 0. }, Point { x: 4., z: 0. }],
                width: 0.8,
                height: 0.,
            };
            let mut intent = LinearIntent::legacy(&stroke);
            intent.elevation_offset = offset;
            operation(
                &mut app,
                Operation::Edit(EditCommand::Context(ContextEdit::Stroke { stroke, intent })),
            );
        }
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::StrokeSelect);
        frame(&mut app, Vec3::new(0.1, 1.015, 0.), true, false);
        assert_eq!(
            app.world().resource::<StrokeTools>().selected,
            Some(StrokeId(2))
        );
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
    }
    #[test]
    fn cancelled_invalid_and_overlong_gestures_never_commit() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Wall);
        frame(&mut app, Vec3::ZERO, true, false);
        frame(&mut app, Vec3::new(5., 0., 0.), false, false);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cancel_world: true,
            ..default()
        };
        app.update();
        assert!(app.world().resource::<StrokeTools>().drag.is_none());
        frame(&mut app, Vec3::ZERO, true, false);
        frame(&mut app, Vec3::new(5., 0., 0.), false, false);
        frame(&mut app, Vec3::new(30., 0., 0.), false, true);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            0
        );
        frame(&mut app, Vec3::ZERO, true, false);
        for i in 1..130 {
            frame(
                &mut app,
                Vec3::new(if i % 2 == 0 { 0. } else { 0.3 }, 0., 0.),
                false,
                false,
            );
        }
        assert!(
            app.world()
                .resource::<StrokeTools>()
                .candidate()
                .unwrap()
                .points
                .len()
                <= 128
        );
        frame(&mut app, Vec3::new(0.3, 0., 0.), false, true);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            0
        );
    }
    #[test]
    fn height_buttons_and_tool_changes_cancel_uncommitted_stroke_preview() {
        let mut app = app();
        draw(
            &mut app,
            Tool::Wall,
            Vec3::new(-5., 0., 0.),
            Vec3::new(5., 0., 0.),
        );
        let button = app
            .world_mut()
            .spawn((Interaction::Pressed, StrokeAction::HeightUp))
            .id();
        app.update();
        settle(&mut app);
        assert!(
            (app.world()
                .resource::<EditorState>()
                .editor()
                .stroke(StrokeId(1))
                .unwrap()
                .height
                - 2.8)
                .abs()
                < 1e-8
        );
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
        app.world_mut().entity_mut(button).despawn();
        frame(&mut app, Vec3::ZERO, true, false);
        frame(&mut app, Vec3::new(2., 0., 0.), false, false);
        app.world_mut()
            .spawn((Interaction::Pressed, EditorAction::Cancel));
        app.update();
        assert!(app.world().resource::<StrokeTools>().drag.is_none());
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
    }
    #[test]
    fn selecting_and_moving_a_wall_keeps_domain_unchanged_until_release() {
        let mut app = app();
        draw(
            &mut app,
            Tool::Wall,
            Vec3::new(-5., 0., 0.),
            Vec3::new(5., 0., 0.),
        );
        let before = app
            .world()
            .resource::<EditorState>()
            .editor()
            .stroke(StrokeId(1))
            .unwrap()
            .clone();
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::StrokeSelect);
        let anchor = Vec3::new(0.1, 1., 0.);
        frame(&mut app, anchor, true, false);
        assert_eq!(
            app.world().resource::<StrokeTools>().selected,
            Some(StrokeId(1))
        );
        frame(&mut app, anchor + Vec3::X * 2., false, false);
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .stroke(StrokeId(1))
                .unwrap(),
            &before
        );
        frame(&mut app, anchor + Vec3::X * 2., false, true);
        settle(&mut app);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .stroke(StrokeId(1))
                .unwrap()
                .points[0]
                .x
                > before.points[0].x + 1.
        );
        operation(&mut app, Operation::Edit(EditCommand::Undo));
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .stroke(StrokeId(1))
                .unwrap(),
            &before
        );
    }
}
