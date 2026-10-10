//! 上下文语义事务的后台协调：串行提交保留玩家操作顺序，生成结果与显示状态分离。
use super::*;
use garden_application::context::{AuthoritySnapshot, ContextCandidate};
use garden_generation::context::{ContextInput, ResolvedContext};
#[derive(Resource, Default)]
pub struct ContextProjection {
    pub revision: u64,
    pub busy: bool,
    pub semantic: Arc<ResolvedContext>,
    pub cache: garden_generation::context_cache::ContextCache,
    pub parts: Arc<garden_generation::context::OpeningParts>,
}
type SemanticResult = (
    ContextCandidate,
    ResolvedContext,
    garden_generation::context_cache::ContextCache,
);
pub(super) type SemanticJob = (Task<SemanticResult>, super::slots::GenerationSlot);
#[derive(Resource, Default)]
pub(super) struct ContextCommitWork {
    pub running: Option<SemanticJob>,
    /// 场景替换不等待旧解析；旧句柄保留到完成，避免错误提前释放真实 CPU 槽位。
    pub retired: Vec<SemanticJob>,
}
pub fn input(snapshot: AuthoritySnapshot) -> ContextInput {
    ContextInput {
        buildings: snapshot.buildings.into_values().collect(),
        strokes: snapshot.strokes.into_values().collect(),
        scene: snapshot.scene,
    }
}

/// 跨类型显示一致性组只协调就绪和共同起点，业务关系仍由语义解析器决定。
#[derive(Resource, Default)]
pub struct PublicationGroups {
    revision: u64,
    expected: std::collections::BTreeSet<garden_domain::BuildingId>,
    ready: std::collections::BTreeSet<garden_domain::BuildingId>,
    linear: bool,
    start: Option<f64>,
}
impl PublicationGroups {
    pub fn begin(
        &mut self,
        revision: u64,
        buildings: impl Iterator<Item = garden_domain::BuildingId>,
    ) {
        self.revision = revision;
        self.expected = buildings.collect();
        self.ready.clear();
        self.linear = false;
        self.start = None;
    }
    pub fn building_ready(&mut self, revision: u64, id: garden_domain::BuildingId) {
        if revision == self.revision {
            self.ready.insert(id);
        }
    }
    pub fn linear_ready(&mut self, revision: u64) {
        if revision == self.revision {
            self.linear = true;
        }
    }
    pub fn ready(&self, revision: u64) -> bool {
        revision != self.revision || (self.linear && self.expected.is_subset(&self.ready))
    }
    pub fn elapsed(&mut self, revision: u64, now: f64) -> Option<f32> {
        if revision != self.revision {
            return None;
        }
        if !self.ready(revision) {
            return None;
        }
        let start = *self.start.get_or_insert(now);
        Some((now - start).max(0.) as f32)
    }
}

impl ContextProjection {
    pub(super) fn refresh(&mut self, editor: &Editor) {
        self.revision = editor.world_revision();
        self.semantic = Arc::new(garden_generation::context::resolve_cached(
            &input(editor.snapshot()),
            &mut self.cache,
        ));
        Arc::make_mut(&mut self.parts).update(&self.semantic);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_application::context::ContextEdit;
    use garden_domain::{
        strokes::Point,
        terrain::{BrushKind, BrushSample},
    };
    use std::time::{Duration, Instant};
    #[test]
    fn replacement_retires_semantics_without_early_slot_release_or_stale_commit() {
        let mut app = App::new();
        app.add_plugins(GardenPlugin::default());
        app.update();
        let editor = app.world().resource::<EditorState>().editor();
        let original_session = editor.session();
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
        let candidate = editor
            .prepare(EditCommand::Context(ContextEdit::Terrain(Arc::new(
                terrain,
            ))))
            .unwrap();
        let slot = app
            .world()
            .resource::<GenerationSlots>()
            .try_acquire()
            .unwrap();
        let (send, receive) = std::sync::mpsc::channel();
        let task = app
            .world()
            .resource::<GenerationPool>()
            .0
            .spawn(async move {
                receive.recv().unwrap();
                let snapshot = input(candidate.snapshot.clone());
                let mut cache = garden_generation::context_cache::ContextCache::default();
                let semantic = garden_generation::context::resolve_cached(&snapshot, &mut cache);
                (candidate, semantic, cache)
            });
        app.world_mut().resource_mut::<ContextCommitWork>().running = Some((task, slot));
        app.world_mut().resource_mut::<ContextProjection>().busy = true;
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::ReplaceScene(Vec::new()))
            .unwrap();
        app.update();
        assert_ne!(
            app.world().resource::<EditorState>().editor().session(),
            original_session
        );
        assert_eq!(app.world().resource::<ContextCommitWork>().retired.len(), 1);
        assert_eq!(app.world().resource::<GenerationSlots>().active(), 1);
        assert!(!app.world().resource::<ContextProjection>().busy);
        send.send(()).unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        while !app
            .world()
            .resource::<ContextCommitWork>()
            .retired
            .is_empty()
        {
            app.update();
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .context()
                .terrain
                .tiles
                .is_empty()
        );
        assert_eq!(app.world().resource::<GenerationSlots>().active(), 0);
    }
    #[test]
    fn publication_waits_for_all_related_members_and_shares_start() {
        let id = garden_domain::BuildingId::new(1).unwrap();
        let mut group = PublicationGroups::default();
        group.begin(4, [id].into_iter());
        group.linear_ready(4);
        assert!(!group.ready(4));
        assert_eq!(group.elapsed(4, 1.), None);
        group.building_ready(3, id);
        assert!(!group.ready(4));
        group.building_ready(4, id);
        assert!(group.ready(4));
        assert_eq!(group.elapsed(4, 2.), Some(0.));
        assert_eq!(group.elapsed(4, 2.08), Some(0.08));
        group.begin(5, [id].into_iter());
        assert_eq!(group.elapsed(5, 3.), None);
    }
}
