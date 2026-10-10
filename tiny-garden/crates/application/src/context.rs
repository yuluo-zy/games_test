//! 上下文事务、依赖读集与历史。纯解析由外层协调，不引入生成或引擎依赖。
//! 候选可以交给后台解析；提交验证会话及版本，意图与最小决策一次发布。
use super::*;
use garden_domain::{
    context::*,
    strokes::{Stroke, StrokeId},
    terrain::{TerrainDocument, TileId},
};
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub enum ContextEdit {
    Decisions(DecisionState),
    Guarded {
        revision: u64,
        edit: Box<ContextEdit>,
    },
    Stroke {
        stroke: Stroke,
        intent: LinearIntent,
    },
    PutOpening(OpeningIntent),
    DeleteOpening(OpeningId),
    Terrain(Arc<TerrainDocument>),
    Foundation {
        building: BuildingId,
        intent: FoundationIntent,
    },
    Batch(Vec<ContextEdit>),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectRef {
    Building(BuildingId),
    Stroke(StrokeId),
    Opening(OpeningId),
    Terrain(TileId),
}
#[derive(Debug, Clone, Default)]
pub struct ChangeSet {
    pub revision: u64,
    pub objects: Vec<ObjectRef>,
    pub regions: Vec<[f64; 4]>,
}
#[derive(Debug, Clone, Default)]
pub struct DependencyStamp {
    pub session: u64,
    pub rules: u64,
    pub objects: BTreeMap<ObjectRef, u64>,
    pub buckets: BTreeMap<(i32, i32), u64>,
}
#[derive(Clone, Default)]
pub(super) struct ContextState {
    pub scene: Arc<SceneContext>,
    pub revision: u64,
    pub revisions: BTreeMap<ObjectRef, u64>,
    pub buckets: BTreeMap<(i32, i32), u64>,
    pub last: ChangeSet,
    pub used_openings: BTreeSet<OpeningId>,
}
#[derive(Debug, Clone)]
pub struct AuthoritySnapshot {
    pub buildings: BTreeMap<BuildingId, Arc<Building>>,
    pub strokes: BTreeMap<StrokeId, Arc<Stroke>>,
    pub scene: Arc<SceneContext>,
}
#[derive(Debug, Clone)]
pub(super) struct ContextPatch {
    before: AuthoritySnapshot,
    after: AuthoritySnapshot,
}
pub struct ContextCandidate {
    pub snapshot: AuthoritySnapshot,
    pub expected_clock: u64,
    pub session: u64,
    pub dependencies: DependencyStamp,
    before: AuthoritySnapshot,
    history: Option<(VecDeque<History>, Vec<History>)>,
}
impl Editor {
    pub fn context(&self) -> &Arc<SceneContext> {
        &self.context.scene
    }
    pub fn context_revision(&self) -> u64 {
        self.context.revision
    }
    pub fn world_revision(&self) -> u64 {
        self.clock
    }
    pub fn session(&self) -> u64 {
        self.session
    }
    pub fn next_opening_id(&self) -> Result<OpeningId, EditError> {
        let n = self
            .context
            .used_openings
            .last()
            .map_or(0, |id| id.0)
            .checked_add(1)
            .ok_or(EditError::CounterExhausted)?;
        if n >= 0x4000 {
            return Err(EditError::CounterExhausted);
        }
        Ok(OpeningId(n))
    }
    pub fn snapshot(&self) -> AuthoritySnapshot {
        AuthoritySnapshot {
            buildings: self.objects.clone(),
            strokes: self.strokes.objects.clone(),
            scene: self.context.scene.clone(),
        }
    }
    /// 即使撤销到没有上下文的场景，下一次重做仍须走同一语义事务。
    pub fn has_context_history(&self, command: &EditCommand) -> bool {
        match command {
            EditCommand::Undo => matches!(self.undo.back(), Some(History::Context(_))),
            EditCommand::Redo => matches!(self.redo.last(), Some(History::Context(_))),
            _ => false,
        }
    }
    pub fn last_change_set(&self) -> &ChangeSet {
        &self.context.last
    }
    /// 兼容旧用例的准备入口；候选编辑只发生在隔离副本中，不污染正式历史或编号。
    pub fn prepare(&self, command: EditCommand) -> Result<ContextCandidate, EditError> {
        let history_edit = matches!(command, EditCommand::Undo | EditCommand::Redo);
        let mut fork = self.clone();
        match command {
            EditCommand::Context(edit) => fork.edit_context(edit)?,
            command => {
                fork.execute(command)?;
            }
        }
        Ok(ContextCandidate {
            snapshot: fork.snapshot(),
            before: self.snapshot(),
            expected_clock: self.clock,
            session: self.session,
            dependencies: self.dependency_stamp([-25.5, -16., 25.5, 16.]),
            history: history_edit.then_some((fork.undo, fork.redo)),
        })
    }
    fn edit_context(&mut self, edit: ContextEdit) -> Result<(), EditError> {
        match edit {
            ContextEdit::Decisions(decisions) => {
                Arc::make_mut(&mut self.context.scene).decisions = decisions;
            }
            ContextEdit::Guarded { revision, edit } => {
                if self.clock != revision {
                    return Err(EditError::StalePreview);
                }
                self.edit_context(*edit)?;
            }
            ContextEdit::Stroke { stroke, intent } => {
                stroke.validate()?;
                intent.validate(&stroke)?;
                if !self.strokes.objects.contains_key(&stroke.id) {
                    if self.strokes.used.contains(&stroke.id) {
                        return Err(EditError::IdAlreadyUsed);
                    }
                    if self.strokes.objects.len() >= 32 {
                        return Err(garden_domain::DomainError("too many strokes").into());
                    }
                }
                Arc::make_mut(&mut self.context.scene)
                    .linear
                    .insert(stroke.id, intent);
                self.strokes.objects.insert(stroke.id, Arc::new(stroke));
            }
            ContextEdit::PutOpening(window) => {
                window.validate()?;
                if !self.context.scene.openings.contains_key(&window.id)
                    && self
                        .objects
                        .get(&window.host.building)
                        .is_none_or(|b| b.block(window.host.block).is_none())
                {
                    return Err(EditError::Missing);
                }
                // 已有窗的宿主可以消失并休眠，新增窗则必须锚定当前合法体块。
                if !self.context.scene.openings.contains_key(&window.id)
                    && self.context.used_openings.contains(&window.id)
                {
                    return Err(EditError::IdAlreadyUsed);
                }
                if window.id.0 >= 0x4000 {
                    return Err(EditError::CounterExhausted);
                }
                Arc::make_mut(&mut self.context.scene)
                    .openings
                    .insert(window.id, window);
            }
            ContextEdit::DeleteOpening(id) => {
                if Arc::make_mut(&mut self.context.scene)
                    .openings
                    .remove(&id)
                    .is_none()
                {
                    return Err(EditError::Missing);
                }
            }
            ContextEdit::Foundation { building, intent } => {
                intent.validate()?;
                if !self.objects.contains_key(&building) {
                    return Err(EditError::Missing);
                }
                Arc::make_mut(&mut self.context.scene)
                    .foundations
                    .insert(building, intent);
            }
            ContextEdit::Terrain(terrain) => {
                terrain.validate()?;
                Arc::make_mut(&mut self.context.scene).terrain = terrain.as_ref().clone();
            }
            ContextEdit::Batch(edits) => {
                for edit in edits {
                    self.edit_context(edit)?;
                }
            }
        }
        self.context.scene.validate()?;
        Ok(())
    }
    /// 决策和意图同一历史操作；解析输出不携带网格或其他运行时数据。
    pub fn commit_prepared(
        &mut self,
        mut candidate: ContextCandidate,
        decisions: DecisionState,
    ) -> Result<ChangeSet, EditError> {
        if candidate.session != self.session
            || candidate.expected_clock != self.clock
            || !self.accepts_dependencies(&candidate.dependencies)
        {
            return Err(EditError::StalePreview);
        }
        Arc::make_mut(&mut candidate.snapshot.scene).decisions = decisions;
        candidate.snapshot.scene.validate()?;
        if candidate.before.buildings == candidate.snapshot.buildings
            && candidate.before.strokes == candidate.snapshot.strokes
            && candidate.before.scene == candidate.snapshot.scene
        {
            if let Some((undo, redo)) = candidate.history.take() {
                self.undo = undo;
                self.redo = redo;
            }
            self.context.last = ChangeSet::default();
            return Ok(self.context.last.clone());
        }
        let patch = ContextPatch {
            before: candidate.before,
            after: candidate.snapshot,
        };
        let change = self.apply_context(&patch, true)?;
        if let Some((undo, redo)) = candidate.history {
            self.undo = undo;
            self.redo = redo;
            return Ok(change);
        }
        self.redo.clear();
        if self.history_limit > 0 {
            if self.undo.len() == self.history_limit {
                self.undo.pop_front();
            }
            self.undo.push_back(History::Context(Box::new(patch)));
        }
        Ok(change)
    }
    pub(super) fn execute_context(
        &mut self,
        edit: ContextEdit,
    ) -> Result<Option<Change>, EditError> {
        let candidate = self.prepare(EditCommand::Context(edit))?;
        let decisions = candidate.snapshot.scene.decisions.clone();
        self.commit_prepared(candidate, decisions)?;
        Ok(None)
    }
    /// 旧用例同样记录前后影响区域，空查询也能察觉新对象进入。
    pub(super) fn note_change(&mut self, object: ObjectRef, regions: Vec<[f64; 4]>, revision: u64) {
        self.context.revision = revision;
        self.context.revisions.insert(object, revision);
        for bounds in &regions {
            for cell in cells(*bounds) {
                self.context.buckets.insert(cell, revision);
            }
        }
        self.context.last = ChangeSet {
            revision,
            objects: vec![object],
            regions,
        };
    }

    pub(super) fn building_bounds(b: &Building) -> [f64; 4] {
        let p = b.placement();
        let mut bounds = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for block in b.blocks() {
            let r = block.footprint;
            for q in [
                garden_domain::strokes::Point { x: r.x, z: r.z },
                garden_domain::strokes::Point {
                    x: r.right(),
                    z: r.z,
                },
                garden_domain::strokes::Point {
                    x: r.right(),
                    z: r.back(),
                },
                garden_domain::strokes::Point {
                    x: r.x,
                    z: r.back(),
                },
            ] {
                let q = world_point(q, p.x, p.z, p.yaw);
                bounds = [
                    bounds[0].min(q.x),
                    bounds[1].min(q.z),
                    bounds[2].max(q.x),
                    bounds[3].max(q.z),
                ];
            }
        }
        bounds
    }
    pub(super) fn apply_context(
        &mut self,
        patch: &ContextPatch,
        forward: bool,
    ) -> Result<ChangeSet, EditError> {
        let target = if forward { &patch.after } else { &patch.before };
        let previous = self.snapshot();
        let serial = self.next_serial()?;
        let mut change = ChangeSet {
            revision: serial,
            ..Default::default()
        };
        let ids = previous
            .strokes
            .keys()
            .chain(target.strokes.keys())
            .copied()
            .collect::<BTreeSet<_>>();
        for id in ids {
            if previous.strokes.get(&id) != target.strokes.get(&id)
                || previous.scene.linear.get(&id) != target.scene.linear.get(&id)
                || previous.scene.decisions.kinds.get(&id) != target.scene.decisions.kinds.get(&id)
            {
                change.objects.push(ObjectRef::Stroke(id));
                for stroke in [previous.strokes.get(&id), target.strokes.get(&id)]
                    .into_iter()
                    .flatten()
                {
                    change.regions.push(stroke.bounds());
                }
            }
        }
        for id in previous
            .scene
            .terrain
            .tiles
            .keys()
            .chain(target.scene.terrain.tiles.keys())
            .copied()
            .collect::<BTreeSet<_>>()
        {
            if previous.scene.terrain.tiles.get(&id) != target.scene.terrain.tiles.get(&id) {
                change.objects.push(ObjectRef::Terrain(id));
                change.regions.push([
                    id.0 as f64 * 4.,
                    id.1 as f64 * 4.,
                    (id.0 + 1) as f64 * 4.,
                    (id.1 + 1) as f64 * 4.,
                ]);
            }
        }
        let mut hosts = BTreeSet::new();
        if previous.scene.decisions.window_groups != target.scene.decisions.window_groups {
            hosts.extend(
                previous
                    .scene
                    .openings
                    .values()
                    .chain(target.scene.openings.values())
                    .map(|w| w.host.building),
            );
        }
        for id in previous
            .scene
            .openings
            .keys()
            .chain(target.scene.openings.keys())
            .copied()
            .collect::<BTreeSet<_>>()
        {
            if previous.scene.openings.get(&id) != target.scene.openings.get(&id) {
                change.objects.push(ObjectRef::Opening(id));
                for w in [
                    previous.scene.openings.get(&id),
                    target.scene.openings.get(&id),
                ]
                .into_iter()
                .flatten()
                {
                    hosts.insert(w.host.building);
                }
            }
        }
        let buildings = previous
            .buildings
            .keys()
            .chain(target.buildings.keys())
            .copied()
            .collect::<BTreeSet<_>>();
        for id in buildings {
            let changed = previous.buildings.get(&id) != target.buildings.get(&id)
                || previous.scene.foundations.get(&id) != target.scene.foundations.get(&id);
            let affected = hosts.contains(&id)
                || [previous.buildings.get(&id), target.buildings.get(&id)]
                    .into_iter()
                    .flatten()
                    .any(|b| {
                        change
                            .regions
                            .iter()
                            .any(|r| bounds_overlap(*r, Self::building_bounds(b)))
                    });
            if changed || affected {
                change.objects.push(ObjectRef::Building(id));
                self.revisions.insert(id, serial);
                if changed {
                    for b in [previous.buildings.get(&id), target.buildings.get(&id)]
                        .into_iter()
                        .flatten()
                    {
                        change.regions.push(Self::building_bounds(b));
                    }
                }
            }
        }
        self.objects = target.buildings.clone();
        self.strokes.objects = target.strokes.clone();
        self.strokes.used.extend(target.strokes.keys().copied());
        self.strokes.revision = serial;
        self.context
            .used_openings
            .extend(target.scene.openings.keys().copied());
        self.context.scene = target.scene.clone();
        self.context.revision = serial;
        for object in &change.objects {
            self.context.revisions.insert(*object, serial);
            if let ObjectRef::Building(id) = object {
                if let Some(b) = self.objects.get(id) {
                    self.used_blocks
                        .entry(*id)
                        .or_default()
                        .extend(b.blocks().iter().map(|b| b.id));
                    self.queue(*id, serial, serial);
                } else {
                    self.latest.remove(id);
                    if let Some(old) = self.pending.remove(id) {
                        self.pending_order.remove(&(old.ticket.request_serial, *id));
                    }
                }
            }
        }
        for region in &change.regions {
            for cell in cells(*region) {
                self.context.buckets.insert(cell, serial);
            }
        }
        self.context.last = change.clone();
        Ok(change)
    }
    /// 网格桶修订号也捕获空候选，防止后来进入区域的新对象漏掉依赖校验。
    pub fn dependency_stamp(&self, bounds: [f64; 4]) -> DependencyStamp {
        let buckets = cells(bounds)
            .into_iter()
            .map(|cell| (cell, self.context.buckets.get(&cell).copied().unwrap_or(0)))
            .collect();
        let mut objects = BTreeMap::new();
        for (id, b) in &self.objects {
            if bounds_overlap(bounds, Self::building_bounds(b)) {
                objects.insert(
                    ObjectRef::Building(*id),
                    self.context
                        .revisions
                        .get(&ObjectRef::Building(*id))
                        .copied()
                        .unwrap_or(0),
                );
            }
        }
        for (id, s) in &self.strokes.objects {
            if bounds_overlap(bounds, s.bounds()) {
                objects.insert(
                    ObjectRef::Stroke(*id),
                    self.context
                        .revisions
                        .get(&ObjectRef::Stroke(*id))
                        .copied()
                        .unwrap_or(0),
                );
            }
        }
        for (id, w) in &self.context.scene.openings {
            if self
                .objects
                .get(&w.host.building)
                .is_some_and(|b| bounds_overlap(bounds, Self::building_bounds(b)))
            {
                objects.insert(
                    ObjectRef::Opening(*id),
                    self.context
                        .revisions
                        .get(&ObjectRef::Opening(*id))
                        .copied()
                        .unwrap_or(0),
                );
            }
        }
        for id in TerrainDocument::tiles_in(bounds) {
            let key = ObjectRef::Terrain(id);
            objects.insert(key, self.context.revisions.get(&key).copied().unwrap_or(0));
        }
        DependencyStamp {
            session: self.session,
            rules: self.context.scene.rules.version,
            objects,
            buckets,
        }
    }
    pub fn accepts_dependencies(&self, stamp: &DependencyStamp) -> bool {
        stamp.session == self.session
            && stamp.rules == self.context.scene.rules.version
            && stamp
                .objects
                .iter()
                .all(|(id, r)| self.context.revisions.get(id).copied().unwrap_or(0) == *r)
            && stamp
                .buckets
                .iter()
                .all(|(id, r)| self.context.buckets.get(id).copied().unwrap_or(0) == *r)
    }
}
fn cells(bounds: [f64; 4]) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for z in (bounds[1] / 4.).floor() as i32..=(bounds[3] / 4.).floor() as i32 {
        for x in (bounds[0] / 4.).floor() as i32..=(bounds[2] / 4.).floor() as i32 {
            out.push((x, z));
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{
        strokes::Point,
        terrain::{BrushKind, BrushSample},
    };
    #[test]
    fn prepared_creation_undo_redo_preserve_history_and_id_tombstones() {
        let mut editor = Editor::new(8);
        let id = BuildingId::new(1).unwrap();
        let create = editor
            .prepare(EditCommand::Create(garden_domain::sample_building(
                id, 3., 3.,
            )))
            .unwrap();
        editor
            .commit_prepared(create, DecisionState::default())
            .unwrap();
        assert_eq!(editor.undo_count(), 1);
        let undo = editor.prepare(EditCommand::Undo).unwrap();
        editor
            .commit_prepared(undo, DecisionState::default())
            .unwrap();
        assert!(editor.get(id).is_none());
        assert_eq!(editor.undo_count(), 0);
        assert_eq!(
            editor.next_building_id().unwrap(),
            BuildingId::new(2).unwrap()
        );
        let redo = editor.prepare(EditCommand::Redo).unwrap();
        editor
            .commit_prepared(redo, DecisionState::default())
            .unwrap();
        assert!(editor.get(id).is_some());
        assert_eq!(editor.undo_count(), 1);
        editor.execute(EditCommand::Undo).unwrap();
        assert!(matches!(
            editor.execute(EditCommand::Create(garden_domain::sample_building(
                id, 3., 3.
            ))),
            Err(EditError::IdAlreadyUsed)
        ));
    }
    #[test]
    fn negative_queries_invalidate_only_when_objects_enter_the_region() {
        let mut editor = Editor::new(8);
        let stamp = editor.dependency_stamp([-2., -2., 2., 2.]);
        let stroke = |id, x| garden_domain::strokes::Stroke {
            id: garden_domain::strokes::StrokeId(id),
            kind: garden_domain::strokes::StrokeKind::Path,
            points: vec![Point { x, z: 0. }, Point { x: x + 1., z: 0. }],
            width: 0.4,
            height: 0.,
        };
        editor
            .execute(EditCommand::Stroke(crate::StrokeEdit::Create(stroke(
                1, 12.,
            ))))
            .unwrap();
        assert!(editor.accepts_dependencies(&stamp));
        editor
            .execute(EditCommand::Stroke(crate::StrokeEdit::Create(stroke(
                2, 0.,
            ))))
            .unwrap();
        assert!(!editor.accepts_dependencies(&stamp));
        let fresh = editor.dependency_stamp([-2., -2., 2., 2.]);
        editor.execute(EditCommand::Undo).unwrap();
        assert!(!editor.accepts_dependencies(&fresh));
    }
    #[test]
    fn invalid_batch_preserves_authority_history_and_opening_numbers() {
        let mut editor = Editor::new(8);
        let before = editor.world_revision();
        let terrain = editor
            .context()
            .terrain
            .brushed(BrushSample {
                center: Point { x: 0., z: 0. },
                radius: 1.,
                amount: 0.2,
                kind: BrushKind::Raise,
            })
            .unwrap();
        let result = editor.execute(EditCommand::Context(ContextEdit::Batch(vec![
            ContextEdit::Terrain(Arc::new(terrain)),
            ContextEdit::DeleteOpening(OpeningId(999)),
        ])));
        assert!(result.is_err());
        assert_eq!(editor.world_revision(), before);
        assert!(editor.context().terrain.tiles.is_empty());
        assert_eq!(editor.undo_count(), 0);
        assert_eq!(editor.next_opening_id().unwrap(), OpeningId(1));
    }
    #[test]
    fn terrain_transaction_is_atomic_undoable_and_old_candidates_are_rejected() {
        let mut e = Editor::new(8);
        let terrain = e
            .context()
            .terrain
            .brushed(BrushSample {
                center: Point { x: 0., z: 0. },
                radius: 1.,
                amount: 0.2,
                kind: BrushKind::Raise,
            })
            .unwrap();
        let candidate = e
            .prepare(EditCommand::Context(ContextEdit::Terrain(Arc::new(
                terrain,
            ))))
            .unwrap();
        let duplicate = e
            .prepare(EditCommand::Context(ContextEdit::Terrain(Arc::default())))
            .unwrap();
        e.commit_prepared(candidate, DecisionState::default())
            .unwrap();
        assert!(e.context().terrain.height(Point { x: 0., z: 0. }) > 0.);
        assert_eq!(e.undo_count(), 1);
        assert_eq!(
            e.commit_prepared(duplicate, DecisionState::default())
                .unwrap_err(),
            EditError::StalePreview
        );
        e.execute(EditCommand::Undo).unwrap();
        assert_eq!(e.context().terrain.height(Point { x: 0., z: 0. }), 0.);
        e.execute(EditCommand::Redo).unwrap();
        assert!(e.context().terrain.height(Point { x: 0., z: 0. }) > 0.);
    }
}
