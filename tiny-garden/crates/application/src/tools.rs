//! Mouse-independent building/block transactions. Screen rays stay outside.
use crate::{EditCommand, EditError, Editor, Preview};
use garden_domain::{
    BlockDraft, BlockId, Building, BuildingEdit, BuildingId, Placement, Stories, sample_building,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Tool {
    #[default]
    Select,
    Build,
    Move,
    MoveBlock,
    Resize,
    Height,
    Rotate,
    Pan,
    Wall,
    Path,
    StrokeSelect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroundPoint {
    pub x: f64,
    pub z: f64,
}
impl GroundPoint {
    pub fn valid(self) -> bool {
        self.x.is_finite() && self.z.is_finite()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolError {
    InvalidDrag,
    OutsideGarden,
    NoSelection,
    RootBlock,
    Edit(EditError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeAxes {
    Both,
    Width,
    Depth,
}

enum Drag {
    Build {
        id: BuildingId,
        anchor: GroundPoint,
        end: GroundPoint,
        candidate: Option<Building>,
    },
    Move {
        anchor: GroundPoint,
        origin: Placement,
        preview: Preview,
        valid: bool,
    },
    Resize {
        anchor: GroundPoint,
        placement: Placement,
        original: BlockDraft,
        preview: Preview,
        valid: bool,
        axes: ResizeAxes,
    },
    MoveBlock {
        anchor: GroundPoint,
        placement: Placement,
        original: BlockDraft,
        preview: Preview,
        valid: bool,
    },
    Height {
        original: BlockDraft,
        preview: Preview,
        valid: bool,
    },
    Rotate {
        center: GroundPoint,
        local_center: GroundPoint,
        origin: Placement,
        last_angle: f64,
        delta: f64,
        preview: Preview,
        valid: bool,
    },
}

#[derive(Default)]
pub struct ToolController {
    tool: Tool,
    selected: Option<BuildingId>,
    block: Option<BlockId>,
    drag: Option<Drag>,
}
impl ToolController {
    pub fn tool(&self) -> Tool {
        self.tool
    }
    pub fn selected(&self) -> Option<BuildingId> {
        self.selected
    }
    pub fn dragging(&self) -> bool {
        self.drag.is_some()
    }
    pub fn set_tool(&mut self, tool: Tool) {
        self.cancel();
        self.tool = tool;
    }
    pub fn select(&mut self, id: Option<BuildingId>) {
        self.cancel();
        if self.selected != id {
            self.block = None;
        }
        self.selected = id;
    }
    pub fn select_block(&mut self, block: Option<BlockId>) {
        self.cancel();
        self.block = block;
    }
    pub fn active_block<'a>(&self, building: &'a Building) -> &'a BlockDraft {
        self.block
            .and_then(|id| building.block(id))
            .unwrap_or_else(|| {
                building
                    .blocks()
                    .iter()
                    .find(|b| b.parent.is_none())
                    .unwrap()
            })
    }
    pub fn repair_selection(&mut self, editor: &Editor) {
        if let Some(id) = self.selected {
            if let Some(building) = editor.get(id) {
                if self.block.is_some_and(|id| building.block(id).is_none()) {
                    self.select_block(None);
                }
            } else {
                self.select(None);
            }
        }
    }
    pub fn add_upper(&self, editor: &Editor) -> Result<EditCommand, ToolError> {
        let id = self.selected.ok_or(ToolError::NoSelection)?;
        let building = editor.get(id).ok_or(ToolError::Edit(EditError::Missing))?;
        let parent = self.active_block(building);
        let r = parent.footprint;
        let width = (r.width * 0.7).max(1.2);
        let depth = (r.depth * 0.7).max(1.2);
        let mut footprint = r;
        footprint.x += (r.width - width) * 0.5;
        footprint.z += (r.depth - depth) * 0.5;
        footprint.width = width;
        footprint.depth = depth;
        let block = BlockDraft {
            id: editor.next_block_id(id).map_err(ToolError::Edit)?,
            parent: Some(parent.id),
            footprint,
            height: 3.,
            stories: Stories::Locked(1),
            roof_intent: parent.roof_intent,
            facade: parent.facade,
        };
        building
            .edited(BuildingEdit::AddBlock(block.clone()))
            .map_err(|e| ToolError::Edit(EditError::Domain(e)))?;
        Ok(EditCommand::Edit {
            building: id,
            edit: BuildingEdit::AddBlock(block),
        })
    }
    pub fn remove_upper(&self, editor: &Editor) -> Result<EditCommand, ToolError> {
        let id = self.selected.ok_or(ToolError::NoSelection)?;
        let building = editor.get(id).ok_or(ToolError::Edit(EditError::Missing))?;
        let block = self.active_block(building);
        if block.parent.is_none() {
            return Err(ToolError::RootBlock);
        }
        Ok(EditCommand::Edit {
            building: id,
            edit: BuildingEdit::RemoveBlock(block.id),
        })
    }
    pub fn cancel(&mut self) {
        self.drag = None;
    }
    pub fn begin(&mut self, editor: &Editor, anchor: GroundPoint) -> Result<(), ToolError> {
        self.cancel();
        if !anchor.valid() {
            return Err(ToolError::InvalidDrag);
        }
        self.drag = match self.tool {
            Tool::Select | Tool::Pan | Tool::Wall | Tool::Path | Tool::StrokeSelect => None,
            Tool::Build => Some(Drag::Build {
                id: editor.next_building_id().map_err(ToolError::Edit)?,
                anchor,
                end: anchor,
                candidate: None,
            }),
            Tool::Move => {
                let id = self.selected.ok_or(ToolError::NoSelection)?;
                let preview = editor.begin_preview(id).map_err(ToolError::Edit)?;
                let origin = preview.candidate().placement();
                Some(Drag::Move {
                    anchor,
                    origin,
                    preview,
                    valid: true,
                })
            }
            Tool::Resize => {
                let id = self.selected.ok_or(ToolError::NoSelection)?;
                let preview = editor.begin_preview(id).map_err(ToolError::Edit)?;
                let original = self.active_block(preview.candidate()).clone();
                Some(Drag::Resize {
                    anchor,
                    placement: preview.candidate().placement(),
                    original,
                    preview,
                    valid: true,
                    axes: ResizeAxes::Both,
                })
            }
            Tool::MoveBlock => {
                let preview = editor
                    .begin_preview(self.selected.ok_or(ToolError::NoSelection)?)
                    .map_err(ToolError::Edit)?;
                let original = self.active_block(preview.candidate()).clone();
                if original.parent.is_none() {
                    return Err(ToolError::RootBlock);
                }
                Some(Drag::MoveBlock {
                    anchor,
                    placement: preview.candidate().placement(),
                    original,
                    preview,
                    valid: true,
                })
            }
            Tool::Height => {
                let preview = editor
                    .begin_preview(self.selected.ok_or(ToolError::NoSelection)?)
                    .map_err(ToolError::Edit)?;
                Some(Drag::Height {
                    original: self.active_block(preview.candidate()).clone(),
                    preview,
                    valid: true,
                })
            }
            Tool::Rotate => {
                let preview = editor
                    .begin_preview(self.selected.ok_or(ToolError::NoSelection)?)
                    .map_err(ToolError::Edit)?;
                let building = preview.candidate();
                let root = building
                    .blocks()
                    .iter()
                    .find(|b| b.parent.is_none())
                    .unwrap()
                    .footprint;
                let local_center = GroundPoint {
                    x: root.x + root.width * 0.5,
                    z: root.z + root.depth * 0.5,
                };
                let origin = building.placement();
                let (sin, cos) = origin.yaw.sin_cos();
                let center = GroundPoint {
                    x: origin.x + cos * local_center.x + sin * local_center.z,
                    z: origin.z - sin * local_center.x + cos * local_center.z,
                };
                let angle = drag_angle(center, anchor).ok_or(ToolError::InvalidDrag)?;
                Some(Drag::Rotate {
                    center,
                    local_center,
                    origin,
                    last_angle: angle,
                    delta: 0.,
                    preview,
                    valid: true,
                })
            }
        };
        Ok(())
    }
    pub fn begin_resize(
        &mut self,
        editor: &Editor,
        anchor: GroundPoint,
        axes: ResizeAxes,
    ) -> Result<(), ToolError> {
        self.set_tool(Tool::Resize);
        self.begin(editor, anchor)?;
        if let Some(Drag::Resize { axes: current, .. }) = &mut self.drag {
            *current = axes;
        }
        Ok(())
    }
    /// Absolute meters from the initial gesture, not accumulated pointer motion.
    pub fn update_height(&mut self, delta: f64) -> Result<(), ToolError> {
        let Some(Drag::Height {
            original,
            preview,
            valid,
        }) = &mut self.drag
        else {
            return Err(ToolError::InvalidDrag);
        };
        *valid = false;
        if !delta.is_finite() {
            return Err(ToolError::InvalidDrag);
        }
        preview
            .update(BuildingEdit::Resize {
                block: original.id,
                footprint: original.footprint,
                height: original.height + delta,
            })
            .map_err(|e| ToolError::Edit(EditError::Domain(e)))?;
        *valid = true;
        Ok(())
    }
    pub fn update(&mut self, point: GroundPoint) -> Result<(), ToolError> {
        match &mut self.drag {
            Some(Drag::Build {
                id,
                anchor,
                end,
                candidate,
            }) => {
                *end = point;
                *candidate = None;
                if !point.valid() {
                    return Err(ToolError::InvalidDrag);
                }
                let mut draft = sample_building(*id, (point.x - anchor.x).abs(), 3.0);
                draft.placement.x = point.x.min(anchor.x);
                draft.placement.z = point.z.min(anchor.z);
                draft.blocks[0].footprint.depth = (point.z - anchor.z).abs();
                let building = Building::try_new(draft).map_err(|_| ToolError::InvalidDrag)?;
                if !inside_garden(&building) {
                    return Err(ToolError::OutsideGarden);
                }
                *candidate = Some(building);
            }
            Some(Drag::Move {
                anchor,
                origin,
                preview,
                valid,
            }) => {
                *valid = false;
                if !point.valid() {
                    return Err(ToolError::InvalidDrag);
                }
                preview
                    .update(garden_domain::BuildingEdit::Move(Placement {
                        x: origin.x + point.x - anchor.x,
                        z: origin.z + point.z - anchor.z,
                        ..*origin
                    }))
                    .map_err(|e| ToolError::Edit(EditError::Domain(e)))?;
                if !inside_garden(preview.candidate()) {
                    return Err(ToolError::OutsideGarden);
                }
                *valid = true;
            }
            Some(Drag::Resize {
                anchor,
                placement,
                original,
                preview,
                valid,
                axes,
            }) => {
                *valid = false;
                if !point.valid() {
                    return Err(ToolError::InvalidDrag);
                }
                let dx = point.x - anchor.x;
                let dz = point.z - anchor.z;
                let (sin, cos) = placement.yaw.sin_cos();
                let mut footprint = original.footprint;
                if *axes != ResizeAxes::Depth {
                    footprint.width += cos * dx - sin * dz;
                }
                if *axes != ResizeAxes::Width {
                    footprint.depth += sin * dx + cos * dz;
                }
                preview
                    .update(garden_domain::BuildingEdit::Resize {
                        block: original.id,
                        footprint,
                        height: original.height,
                    })
                    .map_err(|e| ToolError::Edit(EditError::Domain(e)))?;
                if !inside_garden(preview.candidate()) {
                    return Err(ToolError::OutsideGarden);
                }
                *valid = true;
            }
            Some(Drag::MoveBlock {
                anchor,
                placement,
                original,
                preview,
                valid,
            }) => {
                *valid = false;
                if !point.valid() {
                    return Err(ToolError::InvalidDrag);
                }
                let (sin, cos) = placement.yaw.sin_cos();
                let dx = point.x - anchor.x;
                let dz = point.z - anchor.z;
                let mut footprint = original.footprint;
                footprint.x += cos * dx - sin * dz;
                footprint.z += sin * dx + cos * dz;
                preview
                    .update(BuildingEdit::TranslateBlock {
                        block: original.id,
                        x: footprint.x,
                        z: footprint.z,
                    })
                    .map_err(|e| ToolError::Edit(EditError::Domain(e)))?;
                *valid = true;
            }
            Some(Drag::Rotate {
                center,
                local_center,
                origin,
                last_angle,
                delta,
                preview,
                valid,
            }) => {
                *valid = false;
                let angle = drag_angle(*center, point).ok_or(ToolError::InvalidDrag)?;
                // Unwrap across +/-pi so the cursor can cross the seam smoothly.
                let step = angle - *last_angle;
                *delta -= step.sin().atan2(step.cos());
                *last_angle = angle;
                let yaw = (origin.yaw + *delta).rem_euclid(std::f64::consts::TAU);
                let (sin, cos) = yaw.sin_cos();
                preview
                    .update(BuildingEdit::Move(Placement {
                        x: center.x - cos * local_center.x - sin * local_center.z,
                        z: center.z + sin * local_center.x - cos * local_center.z,
                        yaw,
                        ..*origin
                    }))
                    .map_err(|e| ToolError::Edit(EditError::Domain(e)))?;
                if !inside_garden(preview.candidate()) {
                    return Err(ToolError::OutsideGarden);
                }
                *valid = true;
            }
            Some(Drag::Height { valid, .. }) => {
                *valid = false;
                return Err(ToolError::InvalidDrag);
            }
            None => (),
        }
        Ok(())
    }
    pub fn valid(&self) -> bool {
        match &self.drag {
            Some(Drag::Build { candidate, .. }) => candidate.is_some(),
            Some(
                Drag::Move { valid, .. }
                | Drag::Resize { valid, .. }
                | Drag::Height { valid, .. }
                | Drag::Rotate { valid, .. }
                | Drag::MoveBlock { valid, .. },
            ) => *valid,
            None => false,
        }
    }
    pub fn candidate(&self) -> Option<&Building> {
        match &self.drag {
            Some(Drag::Build { candidate, .. }) => candidate.as_ref(),
            Some(
                Drag::Move { preview, .. }
                | Drag::Resize { preview, .. }
                | Drag::Height { preview, .. }
                | Drag::Rotate { preview, .. }
                | Drag::MoveBlock { preview, .. },
            ) => Some(preview.candidate()),
            None => None,
        }
    }
    pub fn build_outline(&self) -> Option<(GroundPoint, GroundPoint)> {
        match &self.drag {
            Some(Drag::Build { anchor, end, .. }) if end.valid() => Some((*anchor, *end)),
            _ => None,
        }
    }
    pub fn finish(&mut self) -> Result<Option<EditCommand>, ToolError> {
        let command = match self.drag.take() {
            Some(Drag::Build {
                candidate: Some(candidate),
                ..
            }) => EditCommand::Create(candidate.to_draft()),
            Some(
                Drag::Move {
                    preview,
                    valid: true,
                    ..
                }
                | Drag::Resize {
                    preview,
                    valid: true,
                    ..
                }
                | Drag::Height {
                    preview,
                    valid: true,
                    ..
                }
                | Drag::Rotate {
                    preview,
                    valid: true,
                    ..
                }
                | Drag::MoveBlock {
                    preview,
                    valid: true,
                    ..
                },
            ) => EditCommand::CommitPreview(preview),
            Some(_) => return Err(ToolError::InvalidDrag),
            None => return Ok(None),
        };
        Ok(Some(command))
    }
}

fn drag_angle(center: GroundPoint, point: GroundPoint) -> Option<f64> {
    let x = point.x - center.x;
    let z = point.z - center.z;
    (point.valid() && x.hypot(z) > 0.05).then(|| z.atan2(x))
}

/// Desktop greybox bounds, not a general domain restriction. All corners matter
/// for rotated buildings. Mobile / larger worlds can replace this workspace rule.
pub fn inside_garden(building: &Building) -> bool {
    let p = building.placement();
    let (sin, cos) = p.yaw.sin_cos();
    building.blocks().iter().all(|b| {
        let r = b.footprint;
        [
            (r.x, r.z),
            (r.x + r.width, r.z),
            (r.x + r.width, r.z + r.depth),
            (r.x, r.z + r.depth),
        ]
        .iter()
        .all(|&(x, z)| {
            let world_x = p.x + cos * x + sin * z;
            let world_z = p.z - sin * x + cos * z;
            (-25.5..=25.5).contains(&world_x) && (-16.0..=16.0).contains(&world_z)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn point(x: f64, z: f64) -> GroundPoint {
        GroundPoint { x, z }
    }
    #[test]
    fn repeated_build_updates_make_one_undo_step_and_cancel_makes_none() {
        let mut editor = Editor::new(32);
        let mut tools = ToolController::default();
        tools.set_tool(Tool::Build);
        tools.begin(&editor, point(4.0, 5.0)).unwrap();
        for x in 6..15 {
            tools.update(point(x as f64, 10.0)).unwrap();
        }
        assert_eq!(editor.undo_count(), 0);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        assert_eq!(editor.undo_count(), 1);
        let id = editor.objects().next().unwrap().id();
        editor.execute(EditCommand::Undo).unwrap();
        assert!(editor.get(id).is_none());
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.update(point(3.0, 3.0)).unwrap();
        tools.cancel();
        assert!(tools.finish().unwrap().is_none());
        assert_eq!(editor.undo_count(), 0);
    }
    #[test]
    fn invalid_last_build_position_never_commits_the_last_valid_candidate() {
        let editor = Editor::new(32);
        let mut tools = ToolController::default();
        tools.set_tool(Tool::Build);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.update(point(4.0, 5.0)).unwrap();
        assert!(tools.update(point(0.1, 0.1)).is_err());
        assert!(tools.finish().is_err());
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        assert_eq!(
            tools.update(point(26.0, 5.0)),
            Err(ToolError::OutsideGarden)
        );
        assert!(!tools.valid());
    }
    #[test]
    fn backwards_rectangle_is_normalized_and_ids_are_not_recycled() {
        let mut editor = Editor::new(32);
        let mut tools = ToolController::default();
        tools.set_tool(Tool::Build);
        tools.begin(&editor, point(8.0, 8.0)).unwrap();
        tools.update(point(3.0, 4.0)).unwrap();
        let b = tools.candidate().unwrap();
        assert_eq!(b.placement().x, 3.0);
        assert_eq!(b.blocks()[0].footprint.depth, 4.0);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        let id = editor.objects().next().unwrap().id();
        editor.execute(EditCommand::Delete(id)).unwrap();
        assert_eq!(editor.next_building_id().unwrap().get(), 2);
    }
    #[test]
    fn move_is_absolute_from_anchor_and_undo_restores_placement() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 3.0, 3.0)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Move);
        tools.begin(&editor, point(1.0, 1.0)).unwrap();
        tools.update(point(3.0, 3.0)).unwrap();
        tools.update(point(4.0, 5.0)).unwrap();
        assert_eq!(tools.candidate().unwrap().placement().x, 3.0);
        assert_eq!(editor.get(id).unwrap().placement().x, 0.0);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        assert_eq!(editor.undo_count(), 2);
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(editor.get(id).unwrap().placement().x, 0.0);
    }
    #[test]
    fn move_rejects_stale_preview_and_tool_switch_cancels_it() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 3.0, 3.0)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Move);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.update(point(2.0, 2.0)).unwrap();
        editor.execute(EditCommand::Delete(id)).unwrap();
        assert_eq!(
            editor
                .execute(tools.finish().unwrap().unwrap())
                .unwrap_err(),
            EditError::StalePreview
        );
        tools.set_tool(Tool::Build);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.set_tool(Tool::Select);
        assert!(!tools.dragging());
    }
    #[test]
    fn rotated_resize_uses_local_axes_and_commits_one_history_entry() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        let mut draft = sample_building(id, 3.0, 3.0);
        draft.placement.yaw = std::f64::consts::FRAC_PI_2;
        editor.execute(EditCommand::Create(draft)).unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Resize);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.update(point(1.0, -1.0)).unwrap();
        tools.update(point(2.0, -4.0)).unwrap();
        let r = tools.candidate().unwrap().blocks()[0].footprint;
        assert!((r.width - 7.0).abs() < 1e-8 && (r.depth - 8.0).abs() < 1e-8);
        assert_eq!(editor.get(id).unwrap().blocks()[0].footprint.width, 3.0);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        assert_eq!(editor.undo_count(), 2);
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(editor.get(id).unwrap().blocks()[0].footprint.width, 3.0);
    }
    #[test]
    fn invalid_last_resize_and_cancel_never_commit_previous_valid_size() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 3.0, 3.0)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Resize);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.update(point(3.0, 2.0)).unwrap();
        assert!(tools.update(point(-2.5, 0.0)).is_err());
        assert!(tools.finish().is_err());
        assert_eq!(editor.undo_count(), 1);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        tools.update(point(3.0, 2.0)).unwrap();
        tools.cancel();
        assert!(tools.finish().unwrap().is_none());
        assert_eq!(editor.get(id).unwrap().blocks()[0].footprint.width, 3.0);
    }
    #[test]
    fn shrinking_support_cannot_leave_an_upper_block_overhanging() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        let mut draft = sample_building(id, 10.0, 3.0);
        let mut child = draft.blocks[0].clone();
        child.id = garden_domain::BlockId::new(2).unwrap();
        child.parent = Some(draft.blocks[0].id);
        child.footprint.x = 2.0;
        child.footprint.z = 1.0;
        child.footprint.width = 5.0;
        child.footprint.depth = 3.0;
        draft.blocks.push(child);
        editor.execute(EditCommand::Create(draft)).unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Resize);
        tools.begin(&editor, point(0.0, 0.0)).unwrap();
        assert!(tools.update(point(-5.0, 0.0)).is_err());
        assert!(tools.finish().is_err());
        assert_eq!(editor.get(id).unwrap().blocks()[0].footprint.width, 10.0);
    }
    #[test]
    fn exhausted_identity_space_fails_explicitly() {
        let mut editor = Editor::new(32);
        editor
            .execute(EditCommand::Create(sample_building(
                BuildingId::new(u64::MAX).unwrap(),
                3.0,
                3.0,
            )))
            .unwrap();
        assert_eq!(editor.next_building_id(), Err(EditError::CounterExhausted));
    }
    #[test]
    fn height_drag_is_absolute_cancellable_and_invalid_final_sample_never_commits() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 8., 3.)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Height);
        tools.begin(&editor, point(0., 0.)).unwrap();
        for delta in [1., 4., 3.] {
            tools.update_height(delta).unwrap();
        }
        assert_eq!(tools.candidate().unwrap().blocks()[0].height, 6.);
        assert_eq!(editor.get(id).unwrap().blocks()[0].height, 3.);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        assert_eq!(editor.undo_count(), 2);
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(editor.get(id).unwrap().blocks()[0].height, 3.);
        tools.begin(&editor, point(0., 0.)).unwrap();
        tools.update_height(2.).unwrap();
        assert!(tools.update_height(f64::NAN).is_err());
        assert!(tools.finish().is_err());
        tools.begin(&editor, point(0., 0.)).unwrap();
        tools.update_height(2.).unwrap();
        tools.cancel();
        assert!(tools.finish().unwrap().is_none());
        assert_eq!(editor.undo_count(), 1);
    }
    #[test]
    fn rotation_keeps_base_center_fixed_unwraps_angle_and_undo_restores() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        let mut draft = sample_building(id, 8., 3.);
        draft.placement.x = -4.;
        draft.placement.z = -3.;
        let original = Building::try_new(draft.clone()).unwrap();
        editor.execute(EditCommand::Create(draft)).unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        tools.set_tool(Tool::Rotate);
        let p = |angle: f64| point(7. * angle.cos(), 7. * angle.sin());
        tools.begin(&editor, p(3.10)).unwrap();
        tools.update(p(-3.10)).unwrap();
        let b = tools.candidate().unwrap();
        let placement = b.placement();
        let (sin, cos) = placement.yaw.sin_cos();
        assert!((placement.x + cos * 4. + sin * 3.).abs() < 1e-8);
        assert!((placement.z - sin * 4. + cos * 3.).abs() < 1e-8);
        assert!((placement.yaw - (6.20f64).rem_euclid(std::f64::consts::TAU)).abs() < 1e-8);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(**editor.get(id).unwrap(), original);
        tools.begin(&editor, p(0.)).unwrap();
        assert!(tools.update(point(0., 0.)).is_err());
        assert!(tools.finish().is_err());
    }
    #[test]
    fn upper_creation_removal_and_undo_preserve_identity_and_support() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 10., 3.)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        assert!(matches!(
            tools.remove_upper(&editor),
            Err(ToolError::RootBlock)
        ));
        editor.execute(tools.add_upper(&editor).unwrap()).unwrap();
        let child = BlockId::new(2).unwrap();
        tools.select_block(Some(child));
        assert_eq!(
            tools.active_block(editor.get(id).unwrap()).parent,
            Some(BlockId::new(1).unwrap())
        );
        editor.execute(tools.add_upper(&editor).unwrap()).unwrap();
        assert_eq!(editor.get(id).unwrap().blocks().len(), 3);
        editor
            .execute(tools.remove_upper(&editor).unwrap())
            .unwrap();
        assert_eq!(editor.get(id).unwrap().blocks().len(), 1);
        assert_eq!(editor.next_block_id(id).unwrap().get(), 4);
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(editor.get(id).unwrap().blocks().len(), 3);
        assert_eq!(tools.active_block(editor.get(id).unwrap()).id, child);
        editor.execute(EditCommand::Redo).unwrap();
        tools.repair_selection(&editor);
        assert_eq!(tools.active_block(editor.get(id).unwrap()).id.get(), 1);
        editor.execute(tools.add_upper(&editor).unwrap()).unwrap();
        assert!(
            editor
                .get(id)
                .unwrap()
                .block(BlockId::new(4).unwrap())
                .is_some()
        );
    }
    #[test]
    fn upper_block_axes_are_independent_and_support_rejection_is_transactional() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 10., 3.)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        editor.execute(tools.add_upper(&editor).unwrap()).unwrap();
        tools.select_block(Some(BlockId::new(2).unwrap()));
        let old = tools.active_block(editor.get(id).unwrap()).footprint;
        tools
            .begin_resize(&editor, point(0., 0.), ResizeAxes::Width)
            .unwrap();
        tools.update(point(0.5, 100.)).unwrap();
        let new = tools.active_block(tools.candidate().unwrap()).footprint;
        assert_eq!(new.depth, old.depth);
        assert_eq!(new.width, old.width + 0.5);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        assert_eq!(editor.get(id).unwrap().blocks()[0].footprint.width, 10.);
        tools
            .begin_resize(&editor, point(0., 0.), ResizeAxes::Depth)
            .unwrap();
        assert!(tools.update(point(0., 100.)).is_err());
        assert!(tools.finish().is_err());
        assert_eq!(
            editor
                .get(id)
                .unwrap()
                .block(BlockId::new(2).unwrap())
                .unwrap()
                .footprint
                .depth,
            old.depth
        );
    }
    #[test]
    fn moving_upper_block_uses_local_axes_and_never_moves_the_aggregate() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        let mut draft = sample_building(id, 10., 3.);
        draft.placement.yaw = std::f64::consts::FRAC_PI_2;
        editor.execute(EditCommand::Create(draft)).unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        editor.execute(tools.add_upper(&editor).unwrap()).unwrap();
        tools.select_block(Some(BlockId::new(2).unwrap()));
        let original = tools.active_block(editor.get(id).unwrap()).footprint;
        let placement = editor.get(id).unwrap().placement();
        tools.set_tool(Tool::MoveBlock);
        tools.begin(&editor, point(0., 0.)).unwrap();
        tools.update(point(0., -0.5)).unwrap();
        let moved = tools.active_block(tools.candidate().unwrap()).footprint;
        assert!((moved.x - original.x - 0.5).abs() < 1e-8);
        assert!((moved.z - original.z).abs() < 1e-8);
        editor.execute(tools.finish().unwrap().unwrap()).unwrap();
        assert_eq!(editor.get(id).unwrap().placement(), placement);
        tools.begin(&editor, point(0., 0.)).unwrap();
        assert!(tools.update(point(0., -10.)).is_err());
        assert!(tools.finish().is_err());
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(
            tools.active_block(editor.get(id).unwrap()).footprint,
            original
        );
    }
    #[test]
    fn upper_limit_and_block_id_exhaustion_are_explicit_errors() {
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 10., 12.)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        assert!(tools.add_upper(&editor).is_err());
        assert_eq!(editor.undo_count(), 1);
        let mut draft = sample_building(BuildingId::new(2).unwrap(), 10., 3.);
        draft.blocks[0].id = BlockId::new(u64::MAX).unwrap();
        editor.execute(EditCommand::Create(draft)).unwrap();
        assert_eq!(
            editor.next_block_id(BuildingId::new(2).unwrap()),
            Err(EditError::CounterExhausted)
        );
    }
}
