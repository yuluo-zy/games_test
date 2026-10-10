//! Desktop shell: rays/UI -> pure tool transactions -> command inbox.
use crate::{
    camera::{CameraAction, OrbitCamera, PanMode},
    controls::{self, ControlKind, DragPlane},
    picking::hit_part,
    pointer::{DesktopInputSet, PointerFrame, PointerRouterPlugin},
    renderer::DisplayedBuilding,
    ui,
};
use bevy::prelude::*;
use garden_application::{
    EditCommand,
    tools::{GroundPoint, ResizeAxes, Tool, ToolController},
};
use garden_bevy::{
    CommandInbox, EditFeedback, EditorState, GardenSet, GenerationFailures, Operation,
    PendingTargets,
};
use garden_domain::{Building, BuildingEdit, Facade, Roof};

pub struct DesktopEditorPlugin;
impl Plugin for DesktopEditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<PointerRouterPlugin>() {
            app.add_plugins(PointerRouterPlugin);
        }
        app.init_resource::<PanMode>()
            .init_gizmo_group::<controls::ControlGizmos>()
            .add_systems(Startup, controls::setup_gizmos)
            .init_resource::<DesktopEditor>()
            .configure_sets(Update, DesktopInputSet::Tools.before(GardenSet::Commit))
            .add_systems(
                Update,
                (ui_actions, world_tools)
                    .chain()
                    .in_set(DesktopInputSet::Tools),
            )
            .add_systems(
                Update,
                (feedback, status_text, button_color)
                    .chain()
                    .after(GardenSet::Commit),
            )
            .add_systems(Update, draw_overlay.after(GardenSet::Collect));
    }
}
#[derive(Component, Clone, Copy)]
pub enum EditorAction {
    Select,
    Build,
    Move,
    MoveBlock,
    Resize,
    Height,
    Rotate,
    NextBlock,
    AddUpper,
    RemoveUpper,
    Facade,
    Pan,
    Cancel,
    Retry,
    Delete,
    Roof,
    HeightUp,
    HeightDown,
    Undo,
    Redo,
    Wall,
    Path,
    StrokeSelect,
}
#[derive(Component)]
pub struct EditorStatus;

#[derive(Resource)]
pub struct DesktopEditor {
    pub tools: ToolController,
    pub status: String,
    pub(crate) pending: Option<EditCommand>,
    drag_plane: Option<DragPlane>,
    observed_session: Option<u64>,
}
impl Default for DesktopEditor {
    fn default() -> Self {
        Self {
            tools: ToolController::default(),
            status: "点击房屋进行选择，或使用「建造」在草地上拖出一栋新房屋。".into(),
            pending: None,
            drag_plane: None,
            observed_session: None,
        }
    }
}
impl DesktopEditor {
    pub fn submit(&mut self, command: EditCommand, inbox: &mut CommandInbox) {
        self.drag_plane = None;
        if self.pending.is_some() {
            self.status = "请先重试或取消待提交的操作；原操作仍已保留。".into();
            return;
        }
        match inbox.submit(Operation::Edit(command.clone())) {
            Ok(()) => {
                self.pending = None;
                self.status = "操作已排队，等待提交。".into();
            }
            Err(_) => {
                self.pending = Some(command);
                self.status = "操作队列已满，请重试或取消；本次修改已保留。".into();
            }
        }
    }
}
type ActionButton<'a> = (
    &'a Interaction,
    Option<&'a EditorAction>,
    Option<&'a CameraAction>,
);
fn ui_actions(
    buttons: Query<ActionButton<'_>, Changed<Interaction>>,
    editor: Res<EditorState>,
    mut state: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
    mut pan: ResMut<PanMode>,
) {
    for (interaction, action, camera) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if camera.is_some() {
            state.tools.cancel();
            state.drag_plane = None;
        }
        let Some(action) = action else {
            continue;
        };
        state.tools.cancel();
        state.drag_plane = None;
        if matches!(action, EditorAction::Cancel) {
            state.pending = None;
            state.status = "已取消，未新增操作记录。".into();
            continue;
        }
        if matches!(action, EditorAction::Retry) {
            if let Some(command) = state.pending.take() {
                state.submit(command, &mut inbox);
            }
            continue;
        }
        if state.pending.is_some() {
            state.status = "请先重试或取消待提交的操作。".into();
            continue;
        }
        let tool = match action {
            EditorAction::Select => Some(Tool::Select),
            EditorAction::Build => Some(Tool::Build),
            EditorAction::Move => Some(Tool::Move),
            EditorAction::MoveBlock => Some(Tool::MoveBlock),
            EditorAction::Resize => Some(Tool::Resize),
            EditorAction::Height => Some(Tool::Height),
            EditorAction::Rotate => Some(Tool::Rotate),
            EditorAction::Pan => Some(Tool::Pan),
            EditorAction::Wall => Some(Tool::Wall),
            EditorAction::Path => Some(Tool::Path),
            EditorAction::StrokeSelect => Some(Tool::StrokeSelect),
            _ => None,
        };
        if let Some(tool) = tool {
            state.tools.set_tool(tool);
            pan.0 = tool == Tool::Pan;
            state.status = match tool {
                Tool::Select => "点击一栋房屋进行选择。",
                Tool::Build => "在草地上按住左键拖动，松开建造；宽度和进深至少为 1.2 米。",
                Tool::Move => "按住左键拖动房屋进行移动，右键单击取消。",
                Tool::MoveBlock => {
                    "拖动上层体块调整位置，不能超出支撑范围；移动整栋请使用「移动」。"
                }
                Tool::Resize => "按住左键拖动房屋调整宽度和进深，松开提交；右键单击取消。",
                Tool::Height => "拖动绿色高度点或房屋上下调整高度，松开提交。",
                Tool::Rotate => "拖动紫色旋转点围绕底层中心旋转整栋房屋，松开提交。",
                Tool::Pan => "按住左键拖动地面平移视角，不修改房屋或操作记录。",
                Tool::Wall => "在草地上拖动画石墙；松开提交，右键或取消放弃。",
                Tool::Path => "拖动画路，经过石墙会自动生成拱口；路面重叠自动连成一片。",
                Tool::StrokeSelect => "点击墙砖或道路选择；可拖动整条轨迹，按钮调宽/高度或删除。",
            }
            .into();
            continue;
        }
        if matches!(action, EditorAction::AddUpper | EditorAction::RemoveUpper) {
            let result = if matches!(action, EditorAction::AddUpper) {
                state.tools.add_upper(editor.editor())
            } else {
                state.tools.remove_upper(editor.editor())
            };
            match result {
                Ok(command) => state.submit(command, &mut inbox),
                Err(error) => state.status = format!("无法修改体块：{}", ui::tool_error(&error)),
            }
            continue;
        }
        let command = match action {
            EditorAction::Undo => EditCommand::Undo,
            EditorAction::Redo => EditCommand::Redo,
            _ => {
                let Some(id) = state.tools.selected() else {
                    state.status = "请先选择一栋房屋。".into();
                    continue;
                };
                let Some(building) = editor.editor().get(id) else {
                    state.status = "所选房屋已不存在。".into();
                    continue;
                };
                let block = state.tools.active_block(building);
                match action {
                    EditorAction::NextBlock => {
                        let index = building
                            .blocks()
                            .iter()
                            .position(|b| b.id == block.id)
                            .unwrap();
                        state.tools.select_block(Some(
                            building.blocks()[(index + 1) % building.blocks().len()].id,
                        ));
                        state.status = "已切换体块；尺寸、高度、屋顶和立面只修改当前体块。".into();
                        continue;
                    }
                    EditorAction::Facade => EditCommand::Edit {
                        building: id,
                        edit: BuildingEdit::SetFacade {
                            block: block.id,
                            facade: match block.facade {
                                Facade::Stone => Facade::Plaster,
                                Facade::Plaster => Facade::Timber,
                                Facade::Timber => Facade::Stone,
                            },
                        },
                    },
                    EditorAction::Delete => EditCommand::Delete(id),
                    EditorAction::Roof => EditCommand::Edit {
                        building: id,
                        edit: BuildingEdit::SetRoof {
                            block: block.id,
                            roof: match block.roof_intent {
                                Roof::Gabled => Roof::Hipped,
                                Roof::Hipped => Roof::Flat,
                                Roof::Flat => Roof::Gabled,
                            },
                        },
                    },
                    EditorAction::HeightUp | EditorAction::HeightDown => EditCommand::Edit {
                        building: id,
                        edit: BuildingEdit::Resize {
                            block: block.id,
                            footprint: block.footprint,
                            height: block.height
                                + if matches!(action, EditorAction::HeightUp) {
                                    3.0
                                } else {
                                    -3.0
                                },
                        },
                    },
                    _ => continue,
                }
            }
        };
        state.submit(command, &mut inbox);
    }
}
fn ground(ray: Ray3d) -> Option<GroundPoint> {
    if ray.direction.y.abs() < 1e-6 {
        return None;
    }
    let distance = -ray.origin.y / ray.direction.y;
    if distance < 0.0 || !distance.is_finite() {
        return None;
    }
    let point = ray.get_point(distance);
    Some(GroundPoint {
        x: f64::from(point.x),
        z: f64::from(point.z),
    })
}
fn world_tools(
    context_tools: Option<Res<crate::context_tools::ContextTools>>,
    frame: Res<PointerFrame>,
    editor: Res<EditorState>,
    cameras: Query<(&Camera, &Transform), With<OrbitCamera>>,
    displayed: Query<(&DisplayedBuilding, &Transform)>,
    mut state: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    if context_tools.as_ref().is_some_and(|c| c.owns_pointer()) {
        return;
    }
    if frame.cancel_world {
        state.tools.cancel();
        state.drag_plane = None;
        state.status = "已取消拖动，操作记录未改变。".into();
        return;
    }
    if state.pending.is_some() {
        return;
    }
    if matches!(
        state.tools.tool(),
        Tool::Pan | Tool::Wall | Tool::Path | Tool::StrokeSelect
    ) {
        return;
    }
    if !(frame.world_begin || frame.world_held || frame.world_finish) {
        return;
    }
    let Some((camera, view)) = cameras.iter().next() else {
        return;
    };
    let ray = frame.cursor.and_then(|cursor| {
        camera
            .viewport_to_world(&GlobalTransform::from(*view), cursor)
            .ok()
    });
    let Some(ray) = ray else {
        if frame.world_finish {
            state.tools.cancel();
            state.drag_plane = None;
            state.status = "无法定位光标位置，已取消修改。".into();
        }
        return;
    };
    if frame.world_begin {
        state.drag_plane = None;
        // Projected handles win over triangles, but only for the selected building.
        let handle = state.tools.selected().and_then(|id| {
            let building = editor.editor().get(id)?;
            let block = state.tools.active_block(building).id;
            let (shown, transform) = displayed.iter().find(|(b, _)| b.target.building == id)?;
            let handles = controls::controls(&shown.target, transform, block, state.tools.tool());
            controls::hit_control(camera, view, frame.cursor?, &handles).map(|h| (h, shown.ticket))
        });
        let mut axes = ResizeAxes::Both;
        let mut plane_origin = Vec3::ZERO;
        if let Some((handle, ticket)) = handle {
            if !editor.editor().accepts(ticket) {
                state.status = "模型仍在更新，请等待后再拖动控制点。".into();
                return;
            }
            state.tools.set_tool(handle.kind.tool());
            axes = match handle.kind {
                ControlKind::Width => ResizeAxes::Width,
                ControlKind::Depth => ResizeAxes::Depth,
                _ => ResizeAxes::Both,
            };
            plane_origin = handle.position;
        } else if state.tools.tool() != Tool::Build {
            let picked = displayed
                .iter()
                .filter(|(b, _)| editor.editor().get(b.target.building).is_some())
                .filter_map(|(b, t)| {
                    hit_part(ray, b, t).map(|hit| (hit, b.target.building, b.ticket))
                })
                .min_by(|a, b| a.0.distance.total_cmp(&b.0.distance).then(a.1.cmp(&b.1)));
            state.tools.select(picked.map(|(_, id, _)| id));
            state
                .tools
                .select_block(picked.map(|(hit, _, _)| hit.block));
            if state.tools.tool() == Tool::Select {
                state.status = if picked.is_some() {
                    "已选中房屋体块；拖动彩色控制点进行修改。"
                } else {
                    "已取消选择。"
                }
                .into();
                return;
            }
            let Some((hit, _, _)) =
                picked.filter(|(_, _, ticket)| editor.editor().accepts(*ticket))
            else {
                state.status = "请选择已完成更新的房屋，或等待房屋生成完成。".into();
                return;
            };
            plane_origin = ray.get_point(hit.distance);
        }
        let tool = state.tools.tool();
        let normal = if tool == Tool::Height {
            Vec3::new(
                view.translation.x - plane_origin.x,
                0.,
                view.translation.z - plane_origin.z,
            )
        } else {
            Vec3::Y
        };
        if matches!(tool, Tool::Build | Tool::Move) {
            plane_origin = Vec3::ZERO;
        }
        let Some(plane) = DragPlane::begin(ray, plane_origin, normal) else {
            state.status = "当前视角无法拖动，请调整视角。".into();
            return;
        };
        let point = if tool == Tool::Build {
            let Some(point) = crate::context_tools::ground(ray, &editor.editor().context().terrain)
            else {
                return;
            };
            GroundPoint {
                x: point.x,
                z: point.z,
            }
        } else {
            plane.ground(ray).unwrap()
        };
        let result = if tool == Tool::Resize {
            state.tools.begin_resize(editor.editor(), point, axes)
        } else {
            state.tools.begin(editor.editor(), point)
        };
        if let Err(error) = result {
            state.status = format!("无法开始：{}", ui::tool_error(&error));
            return;
        }
        state.drag_plane = Some(plane);
    }
    if state.tools.dragging() {
        let plane = state.drag_plane;
        let result = if state.tools.tool() == Tool::Height {
            state
                .tools
                .update_height(plane.and_then(|p| p.height_delta(ray)).unwrap_or(f64::NAN))
        } else {
            let point = if state.tools.tool() == Tool::Build {
                crate::context_tools::ground(ray, &editor.editor().context().terrain)
                    .map(|p| GroundPoint { x: p.x, z: p.z })
            } else {
                plane.map_or_else(|| ground(ray), |p| p.ground(ray))
            }
            .unwrap_or(GroundPoint {
                x: f64::NAN,
                z: f64::NAN,
            });
            state.tools.update(point)
        };
        match result {
            Ok(()) => state.status = "当前为预览，松开左键提交；右键单击取消。".into(),
            Err(error) => {
                state.status = format!("预览无效：{} 松开不会提交。", ui::tool_error(&error))
            }
        }
        if frame.world_finish {
            state.drag_plane = None;
            let candidate_id = state.tools.candidate().map(Building::id);
            match state.tools.finish() {
                Ok(Some(command)) => {
                    if let Some(id) = candidate_id {
                        state.tools.select(Some(id));
                    }
                    state.submit(command, &mut inbox);
                }
                Ok(None) => (),
                Err(error) => state.status = format!("已取消修改：{}", ui::tool_error(&error)),
            }
        }
    }
}
fn feedback(
    context: Res<garden_bevy::context::ContextProjection>,
    mut results: ResMut<EditFeedback>,
    mut failures: ResMut<GenerationFailures>,
    editor: Res<EditorState>,
    mut state: ResMut<DesktopEditor>,
) {
    let session = editor.editor().session();
    if state.observed_session.is_some_and(|old| old != session) {
        // 新会话可以复用领域编号，旧拖动和排队重试不能因此指向新场景中的同号对象。
        state.tools.cancel();
        state.tools.select(None);
        state.pending = None;
        state.drag_plane = None;
    }
    state.observed_session = Some(session);
    let mut accepted = false;
    for outcome in results.drain() {
        accepted |= outcome.result.is_ok();
        if let Ok(Some(change)) = &outcome.result
            && state.tools.selected() == Some(change.building)
            && let Some(after) = &change.after
            && let Some(added) = after.blocks().iter().find(|b| {
                change
                    .before
                    .as_ref()
                    .is_some_and(|before| before.block(b.id).is_none())
            })
        {
            state.tools.select_block(Some(added.id));
        }
        state.status = match outcome.result {
            Ok(Some(change)) => format!(
                "房屋 #{} 的第 {} 次修改已提交，模型可能仍在生成中。",
                change.building.get(),
                change.revision
            ),
            Ok(None) => "操作已处理；墙路变化会自动重构相关结构。".into(),
            Err(error) => {
                warn!("Edit rejected: {error}");
                format!("修改未通过：{}", ui::edit_error(&error))
            }
        };
    }
    if accepted {
        use garden_generation::context::Diagnostic;
        let sleeping = context
            .semantic
            .diagnostics
            .iter()
            .filter(|d| matches!(d, Diagnostic::Sleeping(_)))
            .count();
        if sleeping > 0 {
            state.status =
                format!("修改已提交，{sleeping}个手工窗暂时休眠；原参数保留，冲突消失后恢复。");
        } else if context
            .semantic
            .diagnostics
            .iter()
            .any(|d| matches!(d, Diagnostic::Steep(_)))
        {
            state.status = "修改已提交；部分道路坡度过大，保留路面并停止生成自动通道。".into();
        } else if context
            .semantic
            .diagnostics
            .iter()
            .any(|d| matches!(d, Diagnostic::BlockedEntrance(_)))
        {
            state.status = "修改已提交；部分道路与墙面高度不匹配，未生成自动门。".into();
        }
    }
    for (ticket, error) in failures.drain() {
        warn!("Geometry failed for #{}: {error}", ticket.building.get());
        state.status = format!(
            "房屋 #{} 的模型生成失败，请调整房屋后重试。",
            ticket.building.get()
        );
    }
    if state.pending.is_some() {
        state.status = "操作队列已满，请重试或取消；本次修改已保留。".into();
    }
    if state.pending.is_none() {
        state.tools.repair_selection(editor.editor());
    }
}
fn status_text(
    state: Res<DesktopEditor>,
    pending: Res<PendingTargets>,
    editor: Res<EditorState>,
    stroke_tools: Option<Res<crate::strokes::StrokeTools>>,
    stroke_targets: Option<Res<garden_bevy::strokes::StrokeTargets>>,
    stroke_display: Option<Res<crate::strokes::StrokeDisplay>>,
    mut labels: Query<&mut Text, With<EditorStatus>>,
) {
    if !state.is_changed()
        && !pending.is_changed()
        && !editor.is_changed()
        && !stroke_tools.as_ref().is_some_and(|r| r.is_changed())
        && !stroke_targets.as_ref().is_some_and(|r| r.is_changed())
        && !stroke_display.as_ref().is_some_and(|r| r.is_changed())
    {
        return;
    }
    let selected = state
        .tools
        .selected()
        .map_or("无".into(), |id| format!("#{}", id.get()));
    for mut text in &mut labels {
        if matches!(
            state.tools.tool(),
            Tool::Wall | Tool::Path | Tool::StrokeSelect
        ) {
            let selected = stroke_tools
                .as_ref()
                .and_then(|t| t.selected)
                .and_then(|id| editor.editor().stroke(id))
                .map_or("无".into(), |s| {
                    format!("#{}（宽{:.1}米，高{:.1}米）", s.id.0, s.width, s.height)
                });
            let progress = if stroke_targets.as_ref().is_some_and(|t| t.error.is_some()) {
                "\n墙路生成失败，保留原模型。"
            } else if stroke_display
                .as_ref()
                .is_some_and(|d| d.revision != Some(editor.editor().world_revision()))
            {
                "\n墙路更新中；青色轨迹为编辑目标。"
            } else {
                ""
            };
            let next = format!(
                "工具：{} ｜ 已选墙路：{}\n{}{}",
                ui::tool_name(state.tools.tool()),
                selected,
                state.status,
                progress
            );
            if text.0 != next {
                text.0 = next;
            }
            continue;
        }
        let block_status = state
            .tools
            .selected()
            .and_then(|id| editor.editor().get(id))
            .map(|b| {
                let block = state.tools.active_block(b);
                format!(
                    " ｜ 体块 #{}（{}层，{:.1}米）",
                    block.id.get(),
                    block.stories.count(),
                    block.height
                )
            })
            .unwrap_or_default();
        let progress = state
            .tools
            .selected()
            .and_then(|id| pending.get(id))
            .map_or("", |p| {
                if p.failed {
                    "\n生成失败，保留原模型；红色轮廓为未显示目标。"
                } else {
                    "\n模型更新中；青色轮廓为最新目标。"
                }
            });
        let next = format!(
            "工具：{} ｜ 已选房屋：{}{}\n{}{}",
            ui::tool_name(state.tools.tool()),
            selected,
            block_status,
            state.status,
            progress
        );
        // Backend polling can mark resources changed without changing this
        // label. Avoid reshaping Chinese glyphs every frame in that case.
        if text.0 != next {
            text.0 = next;
        }
    }
}
fn button_color(
    state: Res<DesktopEditor>,
    mut buttons: Query<(&EditorAction, &Interaction, &mut BackgroundColor)>,
) {
    for (action, interaction, mut color) in &mut buttons {
        let selected = matches!(
            (action, state.tools.tool()),
            (EditorAction::Select, Tool::Select)
                | (EditorAction::Build, Tool::Build)
                | (EditorAction::Move, Tool::Move)
                | (EditorAction::MoveBlock, Tool::MoveBlock)
                | (EditorAction::Resize, Tool::Resize)
                | (EditorAction::Height, Tool::Height)
                | (EditorAction::Rotate, Tool::Rotate)
                | (EditorAction::Pan, Tool::Pan)
                | (EditorAction::Wall, Tool::Wall)
                | (EditorAction::Path, Tool::Path)
                | (EditorAction::StrokeSelect, Tool::StrokeSelect)
        );
        color.0 = if *interaction == Interaction::Pressed {
            Color::srgb_u8(73, 99, 70)
        } else if *interaction == Interaction::Hovered || selected {
            Color::srgb_u8(141, 161, 126)
        } else {
            Color::srgb_u8(108, 132, 99)
        };
    }
}
fn wire_box(gizmos: &mut Gizmos, min: Vec3, max: Vec3, transform: Transform, color: Color) {
    let matrix = transform.compute_affine();
    let corners: [Vec3; 8] = std::array::from_fn(|i| {
        matrix.transform_point3(Vec3::new(
            if i & 1 == 0 { min.x } else { max.x },
            if i & 2 == 0 { min.y } else { max.y },
            if i & 4 == 0 { min.z } else { max.z },
        ))
    });
    for i in 0..8 {
        for bit in [1, 2, 4] {
            if i & bit == 0 {
                gizmos.line(corners[i], corners[i | bit], color);
            }
        }
    }
}
fn wire_building(gizmos: &mut Gizmos, b: &Building, color: Color) {
    let p = b.placement();
    let transform = Transform::from_xyz(p.x as f32, p.elevation as f32, p.z as f32)
        .with_rotation(Quat::from_rotation_y(p.yaw as f32));
    for block in b.blocks() {
        let mut base = 0.0;
        let mut parent = block.parent;
        while let Some(id) = parent {
            let support = b.block(id).unwrap();
            base += support.height;
            parent = support.parent;
        }
        let r = block.footprint;
        wire_box(
            gizmos,
            Vec3::new(r.x as f32, base as f32 + 0.05, r.z as f32),
            Vec3::new(
                (r.x + r.width) as f32,
                (base + block.height) as f32,
                (r.z + r.depth) as f32,
            ),
            transform,
            color,
        );
    }
}
fn draw_overlay(
    state: Res<DesktopEditor>,
    editor: Res<EditorState>,
    cameras: Query<(&Camera, &Transform), With<OrbitCamera>>,
    displayed: Query<(&DisplayedBuilding, &Transform)>,
    mut gizmos: Gizmos,
    mut control_gizmos: Gizmos<controls::ControlGizmos>,
    pending: Res<PendingTargets>,
) {
    if let Some(id) = state.tools.selected() {
        if let Some(target) = pending.get(id) {
            wire_building(
                &mut gizmos,
                &target.building,
                if target.failed {
                    Color::srgb_u8(240, 91, 73)
                } else {
                    Color::srgb_u8(58, 213, 219)
                },
            );
        }
        for (shown, transform) in &displayed {
            if shown.target.building == id {
                wire_box(
                    &mut gizmos,
                    shown.min - Vec3::splat(0.04),
                    shown.max + Vec3::splat(0.04),
                    *transform,
                    Color::srgb_u8(246, 201, 86),
                );
                if let Some(building) = editor.editor().get(id) {
                    let block = state.tools.active_block(building).id;
                    if let Some(b) = shown.target.blocks.iter().find(|b| b.block == block) {
                        wire_box(
                            &mut gizmos,
                            Vec3::new(
                                b.footprint.x as f32,
                                b.base_elevation as f32,
                                b.footprint.z as f32,
                            ),
                            Vec3::new(
                                b.footprint.right() as f32,
                                (b.base_elevation + b.height) as f32,
                                b.footprint.back() as f32,
                            ),
                            *transform,
                            Color::srgb_u8(255, 223, 113),
                        );
                    }
                    if !state.tools.dragging()
                        && let Some((camera, view)) = cameras.iter().next()
                    {
                        let handles =
                            controls::controls(&shown.target, transform, block, state.tools.tool());
                        controls::draw_controls(&mut control_gizmos, camera, view, &handles);
                    }
                }
            }
        }
    }
    let color = if state.tools.valid() {
        Color::srgb_u8(58, 213, 219)
    } else {
        Color::srgb_u8(240, 91, 73)
    };
    if let Some(building) = state.tools.candidate() {
        wire_building(&mut gizmos, building, color);
    } else if let Some((a, b)) = state.tools.build_outline() {
        wire_box(
            &mut gizmos,
            Vec3::new(a.x.min(b.x) as f32, 0.05, a.z.min(b.z) as f32),
            Vec3::new(a.x.max(b.x) as f32, 3.0, a.z.max(b.z) as f32),
            Transform::default(),
            color,
        );
    }
}

#[cfg(test)]
pub(crate) fn setup_context_test_input(app: &mut App) {
    app.init_resource::<DesktopEditor>()
        .init_resource::<PointerFrame>()
        .init_resource::<PanMode>()
        .add_systems(
            Update,
            (ui_actions, world_tools)
                .chain()
                .in_set(DesktopInputSet::Tools)
                .before(GardenSet::Commit),
        )
        .add_systems(Update, feedback.after(GardenSet::Commit));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        catalog::ArtCatalog,
        renderer::{PresentationPlugin, RenderStats},
    };
    use garden_bevy::{GardenPlugin, Settings};
    use garden_domain::{BuildingId, sample_building};
    use std::time::{Duration, Instant};

    fn app(settings: Settings) -> App {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<DesktopEditor>()
            .init_resource::<PointerFrame>()
            .init_resource::<PanMode>()
            .add_plugins((
                GardenPlugin::new(settings)
                    .unwrap()
                    .with_compiler(std::sync::Arc::new(
                        crate::building_kit::BuildingKit::warm_stone(),
                    )),
                PresentationPlugin::new(ArtCatalog::warm_stone()),
            ))
            .add_systems(
                Update,
                (ui_actions, world_tools).chain().before(GardenSet::Commit),
            )
            .add_systems(Update, feedback.after(GardenSet::Commit));
        let orbit = OrbitCamera::from_view(Vec3::new(10.0, 15.0, 20.0), Vec3::ZERO).unwrap();
        let mut camera = Camera::default();
        camera.computed.clip_from_view =
            Mat4::perspective_infinite_reverse_rh(std::f32::consts::PI / 3.0, 1.5, 0.1);
        camera.computed.target_info = Some(bevy::camera::RenderTargetInfo {
            physical_size: UVec2::new(1440, 960),
            scale_factor: 1.0,
        });
        app.world_mut().spawn((camera, orbit.transform(), orbit));
        app
    }
    fn cursor(app: &mut App, point: Vec3) -> Vec2 {
        let mut q = app.world_mut().query::<(&Camera, &Transform)>();
        let (camera, transform) = q.iter(app.world()).next().unwrap();
        camera
            .world_to_viewport(&GlobalTransform::from(*transform), point)
            .unwrap()
    }
    fn frame(app: &mut App, point: Vec3, begin: bool, held: bool, finish: bool) {
        let cursor = cursor(app, point);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cursor: Some(cursor),
            world_begin: begin,
            world_held: held,
            world_finish: finish,
            ..default()
        };
        app.update();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame::default();
    }
    fn await_upload(app: &mut App, uploads: u64) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            app.update();
            if app.world().resource::<RenderStats>().uploads >= uploads {
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    #[test]
    fn status_and_rejected_undo_are_chinese_through_the_real_ui_pipeline() {
        let mut app = app(Settings::default());
        app.add_systems(Update, status_text.after(feedback));
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Resize);
        let label = app.world_mut().spawn((EditorStatus, Text::new(""))).id();
        app.world_mut()
            .spawn((Interaction::Pressed, EditorAction::Undo));
        app.update();
        let text = &app.world().get::<Text>(label).unwrap().0;
        assert!(text.starts_with("工具：尺寸 ｜ 已选房屋：无"));
        assert!(text.contains("没有可撤销或重做的操作"));
        assert!(!text.contains("Resize") && !text.contains("EmptyHistory"));
    }
    fn press(app: &mut App, action: EditorAction) {
        let button = app.world_mut().spawn((Interaction::Pressed, action)).id();
        app.update();
        app.world_mut().despawn(button);
    }
    fn fixture() -> (App, BuildingId) {
        let mut app = app(Settings::default());
        let id = BuildingId::new(1).unwrap();
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Edit(EditCommand::Create(sample_building(
                id, 10., 3.,
            ))))
            .unwrap();
        await_upload(&mut app, 1);
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .select(Some(id));
        (app, id)
    }
    fn handle(app: &mut App, kind: ControlKind) -> controls::Control {
        let state = app.world().resource::<DesktopEditor>();
        let id = state.tools.selected().unwrap();
        let block = state
            .tools
            .active_block(
                app.world()
                    .resource::<EditorState>()
                    .editor()
                    .get(id)
                    .unwrap(),
            )
            .id;
        let world = app.world_mut();
        let mut query = world.query::<(&DisplayedBuilding, &Transform)>();
        let (shown, pose) = query
            .iter(world)
            .find(|(b, _)| b.target.building == id)
            .unwrap();
        controls::controls(&shown.target, pose, block, Tool::Select)
            .into_iter()
            .find(|h| h.kind == kind)
            .unwrap()
    }
    #[test]
    fn ui_upper_add_pick_style_remove_and_undo_use_the_selected_block() {
        let (mut app, id) = fixture();
        press(&mut app, EditorAction::AddUpper);
        await_upload(&mut app, 2);
        let child = garden_domain::BlockId::new(2).unwrap();
        let editor = app.world().resource::<EditorState>().editor();
        assert_eq!(editor.get(id).unwrap().blocks().len(), 2);
        assert_eq!(
            app.world()
                .resource::<DesktopEditor>()
                .tools
                .active_block(editor.get(id).unwrap())
                .id,
            child
        );
        frame(&mut app, Vec3::new(5., 7.5, 3.), true, false, true);
        let editor = app.world().resource::<EditorState>().editor();
        assert_eq!(
            app.world()
                .resource::<DesktopEditor>()
                .tools
                .active_block(editor.get(id).unwrap())
                .id,
            child
        );
        press(&mut app, EditorAction::Facade);
        let editor = app.world().resource::<EditorState>().editor();
        assert_eq!(
            editor.get(id).unwrap().block(child).unwrap().facade,
            Facade::Timber
        );
        assert_eq!(editor.get(id).unwrap().blocks()[0].facade, Facade::Plaster);
        press(&mut app, EditorAction::Roof);
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .block(child)
                .unwrap()
                .roof_intent,
            Roof::Hipped
        );
        press(&mut app, EditorAction::RemoveUpper);
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .blocks()
                .len(),
            1
        );
        press(&mut app, EditorAction::Undo);
        let editor = app.world().resource::<EditorState>().editor();
        assert_eq!(
            editor.get(id).unwrap().block(child).unwrap().roof_intent,
            Roof::Hipped
        );
        assert_eq!(editor.undo_count(), 4);
    }
    #[test]
    fn projected_width_and_height_handles_win_over_selection_and_commit_once() {
        let (mut app, id) = fixture();
        press(&mut app, EditorAction::AddUpper);
        await_upload(&mut app, 2);
        let child = garden_domain::BlockId::new(2).unwrap();
        let width = handle(&mut app, ControlKind::Width);
        frame(&mut app, width.position, true, true, false);
        assert_eq!(
            app.world().resource::<DesktopEditor>().tools.tool(),
            Tool::Resize
        );
        frame(&mut app, width.position + Vec3::X * 0.5, false, true, false);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
        frame(&mut app, width.position + Vec3::X * 0.5, false, false, true);
        let editor = app.world().resource::<EditorState>().editor();
        let block = editor.get(id).unwrap().block(child).unwrap();
        assert!((block.footprint.width - 7.5).abs() < 0.001);
        assert!((block.footprint.depth - 4.2).abs() < 0.001);
        assert_eq!(editor.undo_count(), 3);
        await_upload(&mut app, 3);
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Select);
        let height = handle(&mut app, ControlKind::Height);
        frame(&mut app, height.position, true, true, false);
        assert_eq!(
            app.world().resource::<DesktopEditor>().tools.tool(),
            Tool::Height
        );
        frame(&mut app, height.position + Vec3::Y * 2., false, true, false);
        frame(&mut app, height.position + Vec3::Y * 2., false, false, true);
        let editor = app.world().resource::<EditorState>().editor();
        assert!((editor.get(id).unwrap().block(child).unwrap().height - 5.).abs() < 0.001);
        assert_eq!(editor.get(id).unwrap().blocks()[0].height, 3.);
        assert_eq!(editor.undo_count(), 4);
        press(&mut app, EditorAction::Undo);
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .block(child)
                .unwrap()
                .height,
            3.
        );
    }
    #[test]
    fn projected_rotation_handle_rotates_aggregate_and_cancel_changes_no_history() {
        let (mut app, id) = fixture();
        let rotate = handle(&mut app, ControlKind::Rotate);
        frame(&mut app, rotate.position, true, true, false);
        assert_eq!(
            app.world().resource::<DesktopEditor>().tools.tool(),
            Tool::Rotate
        );
        let radius = rotate.position.distance(rotate.anchor);
        let end = rotate.anchor + Vec3::X * radius;
        frame(&mut app, end, false, true, false);
        frame(&mut app, end, false, false, true);
        let editor = app.world().resource::<EditorState>().editor();
        let p = editor.get(id).unwrap().placement();
        assert!((p.yaw - std::f64::consts::PI * 1.5).abs() < 0.001);
        assert_eq!(editor.undo_count(), 2);
        press(&mut app, EditorAction::Undo);
        await_upload(&mut app, 2);
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Select);
        let height = handle(&mut app, ControlKind::Height);
        frame(&mut app, height.position, true, true, false);
        frame(&mut app, height.position + Vec3::Y, false, true, false);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cancel_world: true,
            ..default()
        };
        app.update();
        let editor = app.world().resource::<EditorState>().editor();
        assert_eq!(editor.undo_count(), 1);
        assert_eq!(editor.get(id).unwrap().blocks()[0].height, 3.);
        assert!(!app.world().resource::<DesktopEditor>().tools.dragging());
    }
    #[test]
    fn stale_display_control_cannot_start_a_new_edit() {
        let (mut app, id) = fixture();
        press(&mut app, EditorAction::HeightUp);
        // Explicitly stale even if a very fast worker completed between frames.
        let world = app.world_mut();
        let mut query = world.query::<&mut DisplayedBuilding>();
        query.iter_mut(world).next().unwrap().ticket.request_serial = 0;
        let height = handle(&mut app, ControlKind::Height);
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Select);
        frame(&mut app, height.position, true, true, false);
        assert!(!app.world().resource::<DesktopEditor>().tools.dragging());
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .blocks()[0]
                .height,
            6.
        );
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
    }
    #[test]
    fn camera_rays_build_pick_move_and_undo_through_real_pipeline() {
        let mut app = app(Settings::default());
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Build);
        frame(&mut app, Vec3::ZERO, true, true, false);
        frame(&mut app, Vec3::new(4.0, 0.0, 5.0), false, true, false);
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .objects()
                .count(),
            0
        );
        frame(&mut app, Vec3::new(4.0, 0.0, 5.0), false, false, true);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            1
        );
        await_upload(&mut app, 1);
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Select);
        frame(&mut app, Vec3::new(2.0, 1.0, 2.0), true, false, true);
        let id = app
            .world()
            .resource::<DesktopEditor>()
            .tools
            .selected()
            .unwrap();
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Move);
        frame(&mut app, Vec3::new(2.0, 1.0, 2.0), true, true, false);
        frame(&mut app, Vec3::new(5.0, 0.0, 6.0), false, true, false);
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .placement()
                .x
                .abs()
                < 0.001
        );
        frame(&mut app, Vec3::new(5.0, 0.0, 6.0), false, false, true);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .placement()
                .x
                > 1.0
        );
        app.world_mut()
            .spawn((Interaction::Pressed, EditorAction::Undo));
        app.update();
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .placement()
                .x
                .abs()
                < 0.001
        );
    }
    #[test]
    fn cancelled_pointer_preview_never_creates_or_changes_history() {
        let mut app = app(Settings::default());
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Build);
        frame(&mut app, Vec3::ZERO, true, true, false);
        frame(&mut app, Vec3::new(4.0, 0.0, 5.0), false, true, false);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cancel_world: true,
            ..default()
        };
        app.update();
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            0
        );
        assert!(!app.world().resource::<DesktopEditor>().tools.dragging());
    }
    #[test]
    fn full_inbox_retains_command_until_visible_retry_accepts_it() {
        let mut app = app(Settings {
            command_capacity: 1,
            ..default()
        });
        let id = BuildingId::new(1).unwrap();
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Invalidate(id))
            .unwrap();
        app.world_mut()
            .resource_scope(|world, mut state: Mut<DesktopEditor>| {
                state.submit(
                    EditCommand::Create(sample_building(id, 3.0, 3.0)),
                    &mut world.resource_mut::<CommandInbox>(),
                );
                assert!(state.pending.is_some());
                state.submit(
                    EditCommand::Create(sample_building(BuildingId::new(2).unwrap(), 3.0, 3.0)),
                    &mut world.resource_mut::<CommandInbox>(),
                );
            });
        app.update();
        assert!(
            app.world()
                .resource::<DesktopEditor>()
                .status
                .contains("重试或取消")
        );
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .is_none()
        );
        app.world_mut()
            .spawn((Interaction::Pressed, EditorAction::Retry));
        app.update();
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .is_some()
        );
        assert!(app.world().resource::<DesktopEditor>().pending.is_none());
    }
}
