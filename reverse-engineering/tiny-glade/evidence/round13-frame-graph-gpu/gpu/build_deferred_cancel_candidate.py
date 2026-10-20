from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
base_root = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/temporal/candidate'
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu/deferred-cancel'
files = []

def change(relative, transform):
    base = (base_root / relative).read_bytes()
    text = base.decode('utf-8')
    candidate = transform(text.replace('\r\n', '\n'))
    newline = '\r\n' if '\r\n' in text else '\n'
    candidate = candidate.replace('\n', newline)
    assert candidate != text
    target = out / 'candidate' / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(candidate.encode('utf-8'))
    files.append({'path': relative, 'base_sha256': hashlib.sha256(base).hexdigest(),
                  'candidate_sha256': hashlib.sha256(candidate.encode('utf-8')).hexdigest()})
    return ''.join(difflib.unified_diff(text.splitlines(keepends=True), candidate.splitlines(keepends=True),
                                      fromfile='a/' + relative, tofile='b/' + relative))

def bridge(text):
    text = text.replace('    pending_build: Rc<Cell<Option<GraphId>>>,\n',
                        '    pending_build: Rc<Cell<Option<GraphId>>>,\n    deferred_cancellations: DeferredCoreCancellations,\n')
    assert text.count('    deferred_cancellations: DeferredCoreCancellations,') == 2
    text = text.replace('            pending_build: Rc::new(Cell::new(None)),\n',
                        '            pending_build: Rc::new(Cell::new(None)),\n            deferred_cancellations: new_deferred_cancellations()?,\n')
    assert text.count('            deferred_cancellations: new_deferred_cancellations()?,') == 2
    text = text.replace('                pending_build: self.pending_build.clone(),\n',
                        '                pending_build: self.pending_build.clone(),\n                deferred_cancellations: self.deferred_cancellations.clone(),\n')
    text = text.replace('''        Ok(CompiledCoreGraph {
            session: self.session.clone(),
            frame: Some(frame),
        })''', '''        Ok(CompiledCoreGraph {
            session: self.session.clone(),
            pending_build: self.pending_build.clone(),
            deferred_cancellations: self.deferred_cancellations.clone(),
            frame: Some(frame),
        })''')
    for signature in [
        '    pub fn warmup_owners(&self) -> rhapsody::Result<CoreRenderOwners> {\n',
        '''    pub fn complete_gameplay_warmup(
        &mut self,
        timeout_ns: u64,
    ) -> rhapsody::Result<CompletedGameplayWarmup> {
''',
        '''    pub fn accept_gameplay_warmup(
        &mut self,
        completed: &CompletedGameplayWarmup,
    ) -> rhapsody::Result<()> {
''',
        '''    pub fn build_frame<T>(
        &mut self,
        build: impl FnOnce(&mut CoreGraphFrame<'_>) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<(T, BuiltCoreGraph)> {
''',
    ]:
        assert text.count(signature) == 1, signature
        text = text.replace(signature, signature + '        self.drain_deferred_cancellations();\n')

    replacements = [
        ('self.session.borrow_mut().execute_frame(graph)', 'self.with_session(|session| session.execute_frame(graph))'),
        ('let frame = self.session.borrow_mut().compile_frame(graph)?;', 'let frame = self.with_session(|session| session.compile_frame(graph))?;'),
        ('''        self.session
            .borrow_mut()
            .execute_prepared(compiled.frame.take().expect("compiled graph consumed once"))''', '''        self.with_session(|session| {
            session.execute_prepared(compiled.frame.take().expect("compiled graph consumed once"))
        })'''),
        ('self.session.borrow_mut().present(target)', 'self.with_session(|session| session.present(target))'),
        ('self.session.borrow_mut().cancel_presentation()', 'self.with_session(|session| session.cancel_presentation())'),
        ('''        let graph = self.claim_built(graph)?;
        self.session.borrow_mut().cancel_frame(graph)''', '''        let graph = self.claim_built(graph)?;
        self.with_session(|session| session.cancel_frame(graph))'''),
        ('self.session.borrow_mut().collect_completed_frames()', 'self.with_session(|session| session.collect_completed_frames())'),
        ('self.session.borrow_mut().wait_submits_done(timeout_ns)', 'self.with_session(|session| session.wait_submits_done(timeout_ns))'),
    ]
    for old, new in replacements:
        assert text.count(old) == 1, old
        text = text.replace(old, new)

    old = '''    pub fn with_session<T>(&mut self, body: impl FnOnce(&mut VulkanGraphSession) -> T) -> T {
        body(&mut self.session.borrow_mut())
    }'''
    new = '''    /// Native session access. A closure may drop a graph owned by this bridge:
    /// its cancellation runs after this borrow, also on Err or unwinding.
    pub fn with_session<T>(&mut self, body: impl FnOnce(&mut VulkanGraphSession) -> T) -> T {
        let cleanup = DeferredCancellationDrain {
            session: &self.session,
            pending_build: &self.pending_build,
            deferred: &self.deferred_cancellations,
        };
        cleanup.drain();
        let value = {
            // This RefMut is dropped before cleanup, including during unwind.
            let mut session = self.session.borrow_mut();
            body(&mut session)
        };
        drop(cleanup);
        value
    }

    fn drain_deferred_cancellations(&self) {
        drain_deferred_cancellations(
            &self.session,
            &self.pending_build,
            &self.deferred_cancellations,
        );
    }'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    old = '''            let id = graph.graph_id();
            if let Err(error) = self.session.borrow_mut().cancel_frame(graph) {
                log::warn!("cancelling uncompiled core frame: {error}");
            }
            if self.pending_build.get() == Some(id) {
                self.pending_build.set(None);
            }'''
    new = '''            cancel_or_defer(
                &self.session,
                &self.pending_build,
                &self.deferred_cancellations,
                DeferredCoreCancellation::Built(graph),
            );'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    old = '''pub struct CompiledCoreGraph {
    session: Rc<RefCell<VulkanGraphSession>>,
    frame: Option<rhapsody::rg::PreparedSessionFrame>,
}'''
    new = '''pub struct CompiledCoreGraph {
    session: Rc<RefCell<VulkanGraphSession>>,
    pending_build: Rc<Cell<Option<GraphId>>>,
    deferred_cancellations: DeferredCoreCancellations,
    frame: Option<rhapsody::rg::PreparedSessionFrame>,
}'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    old = '''            if let Err(error) = self.session.borrow_mut().cancel_prepared(frame) {
                log::warn!("cancelling unsubmitted compiled core frame: {error}");
            }'''
    new = '''            cancel_or_defer(
                &self.session,
                &self.pending_build,
                &self.deferred_cancellations,
                DeferredCoreCancellation::Prepared(frame),
            );'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    old = '''        if self.bridge.pending_build.get() == Some(graph.graph_id()) {
            self.bridge.pending_build.set(None);
        }
        if let Err(error) = self.bridge.session.borrow_mut().cancel_frame(graph.build()) {
            log::warn!("cancelling unsubmitted core frame: {error}");
        }'''
    new = '''        cancel_or_defer(
            &self.bridge.session,
            &self.bridge.pending_build,
            &self.bridge.deferred_cancellations,
            DeferredCoreCancellation::Built(graph.build()),
        );'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    marker = '/// An uncompiled declaration tied to its originating App/session.'
    helpers = '''// One outstanding built/prepared bridge lease bounds this queue to one
// cancellation at a time. Reserve its slot per App, never allocate in Drop.
type DeferredCoreCancellations = Rc<RefCell<Vec<DeferredCoreCancellation>>>;

enum DeferredCoreCancellation {
    Built(rhapsody::rg::BuiltGraph),
    Prepared(rhapsody::rg::PreparedSessionFrame),
}

fn new_deferred_cancellations() -> rhapsody::Result<DeferredCoreCancellations> {
    let mut pending = Vec::new();
    pending
        .try_reserve_exact(1)
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    Ok(Rc::new(RefCell::new(pending)))
}

fn cancel_core_graph(
    session: &mut VulkanGraphSession,
    pending_build: &Cell<Option<GraphId>>,
    work: DeferredCoreCancellation,
) {
    let result = match work {
        DeferredCoreCancellation::Built(graph) => {
            let id = graph.graph_id();
            let result = session.cancel_frame(graph);
            if pending_build.get() == Some(id) {
                pending_build.set(None);
            }
            result
        }
        DeferredCoreCancellation::Prepared(frame) => session.cancel_prepared(frame),
    };
    if let Err(error) = result {
        log::warn!("cancelling abandoned core graph: {error}");
    }
}

fn cancel_or_defer(
    session: &Rc<RefCell<VulkanGraphSession>>,
    pending_build: &Cell<Option<GraphId>>,
    deferred: &DeferredCoreCancellations,
    work: DeferredCoreCancellation,
) {
    if let Ok(mut session) = session.try_borrow_mut() {
        cancel_core_graph(&mut session, pending_build, work);
    } else {
        // Retain the complete native declaration/prepared owner. Its lease
        // remains busy until cancellation runs through this exact session.
        deferred.borrow_mut().push(work);
    }
}

fn drain_deferred_cancellations(
    session: &Rc<RefCell<VulkanGraphSession>>,
    pending_build: &Cell<Option<GraphId>>,
    deferred: &DeferredCoreCancellations,
) {
    if deferred.borrow().is_empty() {
        return;
    }
    let Ok(mut session) = session.try_borrow_mut() else {
        return;
    };
    loop {
        // Release the queue borrow before callbacks run. A callback may drop
        // another graph, which safely queues behind this cancellation.
        let work = deferred.borrow_mut().pop();
        let Some(work) = work else { break };
        cancel_core_graph(&mut session, pending_build, work);
    }
}

struct DeferredCancellationDrain<'a> {
    session: &'a Rc<RefCell<VulkanGraphSession>>,
    pending_build: &'a Cell<Option<GraphId>>,
    deferred: &'a DeferredCoreCancellations,
}
impl DeferredCancellationDrain<'_> {
    fn drain(&self) {
        drain_deferred_cancellations(self.session, self.pending_build, self.deferred);
    }
}
impl Drop for DeferredCancellationDrain<'_> {
    fn drop(&mut self) {
        self.drain();
    }
}
impl Drop for CoreRenderBridge {
    fn drop(&mut self) {
        self.drain_deferred_cancellations();
    }
}

'''
    assert text.count(marker) == 1
    return text.replace(marker, helpers + marker)

def tests(text):
    return text + '''

#[derive(Clone, Copy)]
enum SessionClosureExit {
    Ok,
    Err,
    Panic,
}

fn reentrant_graph_drop_retires_on_session_exit(prepared: bool, exit: SessionClosureExit) {
    let Some(mut bridge) = bridge() else { return };
    let buffers = bridge.hosts().buffers.clone();
    let target = buffers
        .create_buffer(&BufferDesc::new(8, 0x22, 2), "reentrant cancellation", None)
        .unwrap();
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = bridge.build_frame(|frame| {
        frame.import_buffer(&target, 0)?;
        frame.with_graph(|graph| {
            graph.add_pass("must never record abandoned work")
                .with_side_effect()
                .render(|_| panic!("cancelled graph must not record"));
            graph.on_retire(move || flag.set(flag.get() + 1));
            Ok(())
        })
    }).unwrap();
    // This box is only test dispatch over the two public guard types.
    let owned: Box<dyn std::any::Any> = if prepared {
        Box::new(bridge.compile_built(built).unwrap())
    } else {
        Box::new(built)
    };
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        bridge.with_session(move |_| -> Result<(), &'static str> {
            let _owned = owned;
            match exit {
                SessionClosureExit::Ok => Ok(()),
                SessionClosureExit::Err => Err("body error"),
                SessionClosureExit::Panic => panic!("body unwind"),
            }
        })
    }));
    match exit {
        SessionClosureExit::Ok => assert_eq!(outcome.unwrap(), Ok(())),
        SessionClosureExit::Err => assert_eq!(outcome.unwrap(), Err("body error")),
        SessionClosureExit::Panic => {
            let payload = outcome.unwrap_err();
            assert_eq!(payload.downcast_ref::<&str>(), Some(&"body unwind"));
        }
    }
    // Assert before any later session call: with_session's unwind/tail cleanup
    // itself must have run the original retirement, not merely a later query.
    assert_eq!(retired.get(), 1);
    assert_eq!(bridge.with_session(|session| session.frame_import_count()), 0);
    assert_eq!(bridge.with_session(|session| session.pending_frame_count()), 0);
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
    assert_eq!(retired.get(), 1);
    assert!(buffers.release(&target));
}

#[test]
fn built_drop_inside_session_retires_on_normal_return() {
    reentrant_graph_drop_retires_on_session_exit(false, SessionClosureExit::Ok);
}
#[test]
fn built_drop_inside_session_retires_on_body_error() {
    reentrant_graph_drop_retires_on_session_exit(false, SessionClosureExit::Err);
}
#[test]
fn built_drop_inside_session_retires_during_body_unwind() {
    reentrant_graph_drop_retires_on_session_exit(false, SessionClosureExit::Panic);
}
#[test]
fn compiled_drop_inside_session_retires_on_normal_return() {
    reentrant_graph_drop_retires_on_session_exit(true, SessionClosureExit::Ok);
}
#[test]
fn compiled_drop_inside_session_retires_on_body_error() {
    reentrant_graph_drop_retires_on_session_exit(true, SessionClosureExit::Err);
}
#[test]
fn compiled_drop_inside_session_retires_during_body_unwind() {
    reentrant_graph_drop_retires_on_session_exit(true, SessionClosureExit::Panic);
}

#[test]
fn rejected_foreign_graph_drop_defers_until_the_origin_session_borrow_ends() {
    let Some(mut origin) = bridge() else { return };
    let mut other = CoreRenderBridge::from_owners(origin.warmup_owners().unwrap()).unwrap();
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, foreign) = origin.build_frame(|frame| frame.with_graph(|graph| {
        graph.add_pass("foreign graph must never submit")
            .with_side_effect()
            .render(|_| panic!("foreign graph must not record"));
        graph.on_retire(move || flag.set(flag.get() + 1));
        Ok(())
    })).unwrap();
    origin.with_session(|_| {
        assert!(other.execute_built(foreign).is_err());
        assert_eq!(retired.get(), 0, "origin borrow still owns session access");
    });
    assert_eq!(retired.get(), 1);
    assert_eq!(other.with_session(|session| session.pending_frame_count()), 0);
    let (_, retry) = origin.build_frame(|_| Ok(())).unwrap();
    origin.cancel_built(retry).unwrap();
    let (_, retry) = other.build_frame(|_| Ok(())).unwrap();
    other.cancel_built(retry).unwrap();
}

#[test]
fn rejected_foreign_compiled_drop_defers_until_the_origin_session_borrow_ends() {
    let Some(mut origin) = bridge() else { return };
    let mut other = CoreRenderBridge::from_owners(origin.warmup_owners().unwrap()).unwrap();
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = origin.build_frame(|frame| frame.with_graph(|graph| {
        graph.add_pass("foreign compiled graph must never submit")
            .with_side_effect()
            .render(|_| panic!("foreign compiled graph must not record"));
        graph.on_retire(move || flag.set(flag.get() + 1));
        Ok(())
    })).unwrap();
    let foreign = origin.compile_built(built).unwrap();
    origin.with_session(|_| {
        assert!(other.execute_compiled(foreign).is_err());
        assert_eq!(retired.get(), 0, "origin borrow still owns session access");
    });
    assert_eq!(retired.get(), 1);
    assert_eq!(other.with_session(|session| session.pending_frame_count()), 0);
    let (_, retry) = origin.build_frame(|_| Ok(())).unwrap();
    origin.cancel_built(retry).unwrap();
    let (_, retry) = other.build_frame(|_| Ok(())).unwrap();
    other.cancel_built(retry).unwrap();
}

#[test]
fn deferred_retirement_panic_does_not_replace_the_session_body_unwind() {
    let Some(mut bridge) = bridge() else { return };
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = bridge.build_frame(|frame| frame.with_graph(|graph| {
        graph.on_retire(move || {
            flag.set(flag.get() + 1);
            panic!("retirement panic");
        });
        Ok(())
    })).unwrap();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        bridge.with_session(move |_| {
            let _built = built;
            panic!("body unwind");
        });
    }));
    let payload = outcome.unwrap_err();
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"body unwind"));
    assert_eq!(retired.get(), 1);
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
}
'''

patch = change('tiny-glade/crates/tiny-glade/src/render_bridge.rs', bridge)
patch += change('tiny-glade/crates/tiny-glade/tests/core_render_bridge.rs', tests)
out.mkdir(parents=True, exist_ok=True)
(out / 'candidate.patch').write_text(patch, encoding='utf-8', newline='')
(out / 'manifest.json').write_text(json.dumps({
    'base': 'frozen temporal v2 candidate (not production at generation time)',
    'scope': 'defer originating-session cancellation across legal with_session guard Drop, including unwind',
    'production_written': False,
    'files': files,
    'validation': 'root owns Cargo and actual GPU acceptance',
}, indent=2), encoding='utf-8')
print('deferred-cancel/candidate.patch generated')
