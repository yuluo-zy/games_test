use super::{Change, EditError, Editor, History};
use garden_domain::strokes::{Stroke, StrokeId};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub enum StrokeEdit {
    Create(Stroke),
    Replace {
        stroke: Stroke,
        expected_revision: u64,
    },
    Delete {
        id: StrokeId,
        expected_revision: u64,
    },
}
#[derive(Clone, Default)]
pub(super) struct StrokeState {
    pub objects: BTreeMap<StrokeId, Arc<Stroke>>,
    pub used: BTreeSet<StrokeId>,
    pub revision: u64,
}
#[derive(Debug, Clone)]
pub(super) struct StrokePatch {
    id: StrokeId,
    before: Option<Arc<Stroke>>,
    after: Option<Arc<Stroke>>,
}
impl Editor {
    pub fn strokes(&self) -> impl Iterator<Item = &Arc<Stroke>> {
        self.strokes.objects.values()
    }
    pub fn stroke(&self, id: StrokeId) -> Option<&Arc<Stroke>> {
        self.strokes.objects.get(&id)
    }
    pub fn stroke_revision(&self) -> u64 {
        self.strokes.revision
    }
    pub fn next_stroke_id(&self) -> Result<StrokeId, EditError> {
        self.strokes
            .used
            .last()
            .map_or(0, |id| id.0)
            .checked_add(1)
            .map(StrokeId)
            .ok_or(EditError::CounterExhausted)
    }
    pub(super) fn execute_stroke(&mut self, edit: StrokeEdit) -> Result<Option<Change>, EditError> {
        let patch = match edit {
            StrokeEdit::Create(stroke) => {
                stroke.validate()?;
                if self.strokes.used.contains(&stroke.id) {
                    return Err(EditError::IdAlreadyUsed);
                }
                if self.strokes.objects.len() >= 32 {
                    return Err(garden_domain::DomainError("too many strokes").into());
                }
                StrokePatch {
                    id: stroke.id,
                    before: None,
                    after: Some(Arc::new(stroke)),
                }
            }
            StrokeEdit::Replace {
                stroke,
                expected_revision,
            } => {
                if expected_revision != self.stroke_revision() {
                    return Err(EditError::StalePreview);
                }
                stroke.validate()?;
                if let Some(intent) = self.context.scene.linear.get(&stroke.id) {
                    // 改变采样点数必须经 ContextEdit 显式提供稳定控制点编号。
                    intent.validate(&stroke)?;
                }
                let before = self.stroke(stroke.id).ok_or(EditError::Missing)?.clone();
                if *before == stroke {
                    return Ok(None);
                }
                StrokePatch {
                    id: stroke.id,
                    before: Some(before),
                    after: Some(Arc::new(stroke)),
                }
            }
            StrokeEdit::Delete {
                id,
                expected_revision,
            } => {
                if expected_revision != self.stroke_revision() {
                    return Err(EditError::StalePreview);
                }
                StrokePatch {
                    id,
                    before: Some(self.stroke(id).ok_or(EditError::Missing)?.clone()),
                    after: None,
                }
            }
        };
        self.apply_stroke(&patch, true)?;
        self.redo.clear();
        if self.history_limit > 0 {
            if self.undo.len() == self.history_limit {
                self.undo.pop_front();
            }
            self.undo.push_back(History::Stroke(patch));
        }
        // Existing Change describes buildings only; stroke consumers use revision.
        Ok(None)
    }
    fn apply_stroke(&mut self, patch: &StrokePatch, forward: bool) -> Result<(), EditError> {
        let serial = self.next_serial()?;
        let before = self.strokes.objects.get(&patch.id).cloned();
        let after = if forward { &patch.after } else { &patch.before };
        if let Some(stroke) = after {
            self.strokes.objects.insert(patch.id, stroke.clone());
        } else {
            self.strokes.objects.remove(&patch.id);
            Arc::make_mut(&mut self.context.scene)
                .linear
                .remove(&patch.id);
            Arc::make_mut(&mut self.context.scene)
                .decisions
                .kinds
                .remove(&patch.id);
        }
        self.strokes.used.insert(patch.id);
        self.strokes.revision = serial;
        let regions = before
            .iter()
            .chain(after.iter())
            .map(|s| s.bounds())
            .collect();
        self.note_change(super::context::ObjectRef::Stroke(patch.id), regions, serial);
        Ok(())
    }
    pub(super) fn apply_history(
        &mut self,
        history: &History,
        forward: bool,
    ) -> Result<Option<Change>, EditError> {
        match history {
            History::Building(patch) => self.apply(patch, forward).map(Some),
            History::Stroke(patch) => self.apply_stroke(patch, forward).map(|()| None),
            History::Context(patch) => self.apply_context(patch, forward).map(|_| None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditCommand;
    use garden_domain::{
        BuildingId, sample_building,
        strokes::{Point, StrokeKind},
    };
    fn wall() -> Stroke {
        Stroke {
            id: StrokeId(1),
            kind: StrokeKind::Wall,
            points: vec![Point { x: 0., z: 0. }, Point { x: 4., z: 0. }],
            width: 0.4,
            height: 2.4,
        }
    }
    #[test]
    fn walls_and_houses_share_history_and_deleted_ids_are_not_reused() {
        let mut e = Editor::new(16);
        e.execute(EditCommand::Stroke(StrokeEdit::Create(wall())))
            .unwrap();
        e.execute(EditCommand::Create(sample_building(
            BuildingId::new(1).unwrap(),
            3.,
            3.,
        )))
        .unwrap();
        e.execute(EditCommand::Undo).unwrap();
        assert_eq!(e.objects().count(), 0);
        assert_eq!(e.strokes().count(), 1);
        e.execute(EditCommand::Undo).unwrap();
        assert_eq!(e.strokes().count(), 0);
        assert_eq!(e.next_stroke_id().unwrap(), StrokeId(2));
        e.execute(EditCommand::Redo).unwrap();
        assert_eq!(e.stroke(StrokeId(1)).unwrap().height, 2.4);
        let revision = e.stroke_revision();
        let mut changed = wall();
        changed.height = 3.;
        e.execute(EditCommand::Stroke(StrokeEdit::Replace {
            stroke: changed,
            expected_revision: revision,
        }))
        .unwrap();
        assert!(matches!(
            e.execute(EditCommand::Stroke(StrokeEdit::Delete {
                id: StrokeId(1),
                expected_revision: revision
            })),
            Err(EditError::StalePreview)
        ));
        e.replace_scene(vec![]).unwrap();
        assert_eq!(e.strokes().count(), 0);
        assert_eq!(e.undo_count(), 0);
    }
}
