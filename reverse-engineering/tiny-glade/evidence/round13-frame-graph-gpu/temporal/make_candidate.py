from pathlib import Path
import hashlib, json, difflib

ROOT = Path('D:/game')
OUT = Path(__file__).parent
paths = [
    'tiny-glade/crates/tiny-glade/src/render_bridge.rs',
    'tiny-glade/crates/rhapsody/src/rg/vulkan.rs',
    'tiny-glade/crates/tiny-glade/tests/core_render_bridge.rs',
    'tiny-glade/crates/tiny-glade/tests/original_scene_compute_gpu.rs',
]
base = {p: (ROOT/p).read_text(encoding='utf-8') for p in paths}
candidate = dict(base)

def replace(path, old, new):
    assert candidate[path].count(old) == 1, (path, old[:120])
    candidate[path] = candidate[path].replace(old, new, 1)

bridge, vulkan, bridge_test, gpu_test = paths
replace(bridge, '    shaders: Arc<AssetShaderLibrary>,\n}\n\n/// This App',
'''    shaders: Arc<AssetShaderLibrary>,
    /// A built graph still owns this session's declaration/specification state.
    /// The retirement callback also clears the lease if another bridge rejects it.
    pending_build: Rc<Cell<Option<GraphId>>>,
}

/// This App''')
replace(bridge, '            shaders,\n        })\n    }\n\n    /// Captured',
'''            shaders,
            pending_build: Rc::new(Cell::new(None)),
        })
    }

    /// Captured''')
replace(bridge, '        if self.session.borrow().named_resource_count() != 0 {',
'''        if self.pending_build.get().is_some()
            || self.session.borrow().named_resource_count() != 0
        {''')
replace(bridge, '            shaders: owners.shaders,\n        })',
'''            shaders: owners.shaders,
            pending_build: Rc::new(Cell::new(None)),
        })''')
replace(bridge, '''        if let Some(named) = named.take() {
            self.session
                .borrow_mut()
                .adopt_completed_named_resources(named)?;
        }''', '''        if let Some(completed) = named.as_ref() {
            let mut session = self.session.borrow_mut();
            // Keep the actual owners available when activation must be retried.
            // No state can change between this borrowed preflight and adoption:
            // the session is thread local and this RefMut remains held.
            session.validate_completed_named_resources_adoption(completed)?;
            session.adopt_completed_named_resources(named.take().expect("validated warmup"))?;
        }''')
replace(bridge, '''        let graph = self.session.borrow_mut().begin_frame()?;
        let declarer''', '''        if self.pending_build.get().is_some() {
            return Err(rhapsody::Error::invalid_usage(
                "execute, compile or cancel the built core graph before beginning another frame",
            ));
        }
        let mut graph = self.session.borrow_mut().begin_frame()?;
        let graph_id = graph.graph_id();
        self.pending_build.set(Some(graph_id));
        let pending = self.pending_build.clone();
        graph.on_retire(move || {
            if pending.get() == Some(graph_id) {
                pending.set(None);
            }
        });
        let declarer''')
replace(bridge, '''        self.session.borrow_mut().execute_frame(graph)
    }

    pub fn compile_built''', '''        let graph = self.claim_built(graph)?;
        self.session.borrow_mut().execute_frame(graph)
    }

    /// Preserve the graph/session identity even for graphs without named resources.
    /// Session-level APIs also support independently constructed graphs, while
    /// this bridge only accepts the declaration it actually built.
    fn claim_built(
        &self,
        graph: rhapsody::rg::BuiltGraph,
    ) -> rhapsody::Result<rhapsody::rg::BuiltGraph> {
        if self.pending_build.get() != Some(graph.graph_id()) {
            // cancel_frame retires callbacks/leases before rejecting a foreign
            // graph. Its originating bridge's retirement lease is released too.
            if let Err(cleanup) = self.session.borrow_mut().cancel_frame(graph) {
                log::warn!("retiring rejected core graph: {cleanup}");
            }
            return Err(rhapsody::Error::invalid_usage(
                "built graph belongs to another bridge or declaration",
            ));
        }
        self.pending_build.set(None);
        Ok(graph)
    }

    pub fn compile_built''')
replace(bridge, '        let frame = self.session.borrow_mut().compile_frame(graph)?;',
'''        let graph = self.claim_built(graph)?;
        let frame = self.session.borrow_mut().compile_frame(graph)?;''')
replace(bridge, '''    pub fn cancel_built(&mut self, graph: rhapsody::rg::BuiltGraph) -> rhapsody::Result<()> {
        self.session.borrow_mut().cancel_frame(graph)
    }''', '''    pub fn cancel_built(&mut self, graph: rhapsody::rg::BuiltGraph) -> rhapsody::Result<()> {
        let graph = self.claim_built(graph)?;
        self.session.borrow_mut().cancel_frame(graph)
    }''')

# The current consuming API remains source compatible. Its validation becomes
# borrowable so the App's one-shot transfer does not discard owners on rejection.
replace(vulkan, '''    pub fn adopt_completed_named_resources(
        &mut self,
        completed: CompletedNamedResources,
    ) -> crate::Result<()> {
        if self.active_prepared.is_some()''', '''    pub fn validate_completed_named_resources_adoption(
        &self,
        completed: &CompletedNamedResources,
    ) -> crate::Result<()> {
        if self.active_prepared.is_some()''')
replace(vulkan, '''        for (token, family) in completed.families {
            self.backend.import_families.insert(token, family);
        }
        self.named = Some(completed.named);
        Ok(())
    }

    pub fn configure_receipt_storage''', '''        Ok(())
    }

    /// Consume a completed owner's cache only after all adoption checks succeed.
    /// Call the borrowed validation first when retaining the transfer on failure
    /// is required by an application-owned one-shot channel.
    pub fn adopt_completed_named_resources(
        &mut self,
        completed: CompletedNamedResources,
    ) -> crate::Result<()> {
        self.validate_completed_named_resources_adoption(&completed)?;
        for (token, family) in completed.families {
            self.backend.import_families.insert(token, family);
        }
        self.named = Some(completed.named);
        Ok(())
    }

    pub fn configure_receipt_storage''')

candidate[bridge_test] += '''

#[test]
fn built_graph_lease_survives_rejected_rebuild_and_releases_on_compiled_drop() {
    let Some(mut bridge) = bridge() else { return };
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = bridge.build_frame(|frame| frame.with_graph(|graph| {
        graph.add_pass("original pending declaration").with_side_effect().render(|_| true);
        graph.on_retire(move || flag.set(flag.get() + 1));
        Ok(())
    })).unwrap();
    let invoked = Cell::new(false);
    assert!(bridge.build_frame(|_| { invoked.set(true); Ok(()) }).is_err());
    assert!(!invoked.get(), "rejected begin must not invoke another builder");
    assert_eq!(retired.get(), 0, "the first declaration must remain live");
    let compiled = bridge.compile_built(built).unwrap();
    assert_eq!(compiled.snapshot().pass_order, ["original pending declaration"]);
    assert!(bridge.build_frame(|_| Ok(())).is_err(), "prepared frame still owns its session");
    drop(compiled);
    assert_eq!(retired.get(), 1);
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
}

#[test]
fn foreign_built_graph_retires_the_originating_lease_without_submitting() {
    let Some(mut origin) = bridge() else { return };
    let mut other = CoreRenderBridge::from_owners(origin.warmup_owners().unwrap()).unwrap();
    let retired = Rc::new(Cell::new(false));
    let flag = retired.clone();
    let (_, foreign) = origin.build_frame(|frame| frame.with_graph(|graph| {
        graph.add_pass("foreign work").with_side_effect().render(|_| {
            panic!("foreign graph must never record");
        });
        graph.on_retire(move || flag.set(true));
        Ok(())
    })).unwrap();
    let error = other.execute_built(foreign).unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert!(retired.get());
    assert_eq!(other.with_session(|session| session.pending_frame_count()), 0);
    let (_, retry) = origin.build_frame(|_| Ok(())).unwrap();
    origin.cancel_built(retry).unwrap();
    let (_, retry) = other.build_frame(|_| Ok(())).unwrap();
    other.cancel_built(retry).unwrap();
}
'''
replace(gpu_test, '''    fixture.world = world;
    fixture.bridge.accept_gameplay_warmup(&completed).unwrap();''', '''    fixture.world = world;
    // An outstanding declaration makes activation temporarily inadmissible.
    // Rejecting it must preserve the worker's actual one-shot owners/parity,
    // so activation can be retried after this graph is cancelled.
    let (_, unsubmitted) = fixture.bridge.build_frame(|_| Ok(())).unwrap();
    assert!(fixture.bridge.accept_gameplay_warmup(&completed).is_err());
    fixture.bridge.cancel_built(unsubmitted).unwrap();
    fixture.bridge.accept_gameplay_warmup(&completed).unwrap();''')

manifest = {'base_commit': '1253c0ab7cc9706c58c4e57c3a61a0b17c7212f2', 'files': []}
patch = []
for path in paths:
    target = OUT/'candidate'/path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(candidate[path], encoding='utf-8', newline='\n')
    manifest['files'].append({'path': path, 'base_sha256': hashlib.sha256((ROOT/path).read_bytes()).hexdigest()})
    patch += difflib.unified_diff(base[path].splitlines(keepends=True), candidate[path].splitlines(keepends=True), fromfile='a/'+path, tofile='b/'+path)
(OUT/'candidate.patch').write_text(''.join(patch), encoding='utf-8', newline='\n')
(OUT/'base.json').write_text(json.dumps(manifest, indent=2)+'\n', encoding='utf-8')
print(json.dumps(manifest, indent=2))
