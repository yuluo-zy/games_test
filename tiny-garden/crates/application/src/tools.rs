//! Mouse-independent build/move transactions. Screen rays and buttons stay outside.
use crate::{EditCommand, EditError, Editor, Preview};
use garden_domain::{Building, BuildingId, Placement, sample_building};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Tool {
    #[default]
    Select,
    Build,
    Move,
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
    Edit(EditError),
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
}

#[derive(Default)]
pub struct ToolController {
    tool: Tool,
    selected: Option<BuildingId>,
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
        self.selected = id;
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
            Tool::Select => None,
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
        };
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
            None => (),
        }
        Ok(())
    }
    pub fn valid(&self) -> bool {
        match &self.drag {
            Some(Drag::Build { candidate, .. }) => candidate.is_some(),
            Some(Drag::Move { valid, .. }) => *valid,
            None => false,
        }
    }
    pub fn candidate(&self) -> Option<&Building> {
        match &self.drag {
            Some(Drag::Build { candidate, .. }) => candidate.as_ref(),
            Some(Drag::Move { preview, .. }) => Some(preview.candidate()),
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
            Some(Drag::Move {
                preview,
                valid: true,
                ..
            }) => EditCommand::CommitPreview(preview),
            Some(_) => return Err(ToolError::InvalidDrag),
            None => return Ok(None),
        };
        Ok(Some(command))
    }
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
}
