from pathlib import Path
import hashlib, json
root = Path('D:/game')
out = Path(__file__).parent
path = out/'candidate/tiny-glade/crates/tiny-glade/src/render_bridge.rs'
source = path.read_text(encoding='utf-8')
source = source.replace('    /// The retirement callback also clears the lease if another bridge rejects it.\n', '    /// A BuiltCoreGraph guard releases it on cancellation or abandonment.\n')
source = source.replace('    ) -> rhapsody::Result<(T, rhapsody::rg::BuiltGraph)> {', '    ) -> rhapsody::Result<(T, BuiltCoreGraph)> {')
old = '''        let pending = self.pending_build.clone();
        graph.on_retire(move || {
            if pending.get() == Some(graph_id) {
                pending.set(None);
            }
        });
'''
assert old in source
source = source.replace(old, '')
source = source.replace('        let mut graph = self.session.borrow_mut().begin_frame()?;', '        let graph = self.session.borrow_mut().begin_frame()?;')
source = source.replace('        Ok((value, graph.build()))', '''        Ok((value, BuiltCoreGraph {
            session: self.session.clone(),
            pending_build: self.pending_build.clone(),
            graph: Some(graph.build()),
        }))''')
start = source.index('    /// Preserve the graph/session identity')
end = source.index('    pub fn compile_built', start)
source = source[:start] + '''    /// Reject a different App's graph before consuming its declaration.
    /// A rejected guard cancels through its originating session when dropped.
    fn claim_built(
        &self,
        mut graph: BuiltCoreGraph,
    ) -> rhapsody::Result<rhapsody::rg::BuiltGraph> {
        if !Rc::ptr_eq(&self.session, &graph.session)
            || self.pending_build.get() != Some(graph.graph_id())
        {
            return Err(rhapsody::Error::invalid_usage(
                "built graph belongs to another bridge or declaration",
            ));
        }
        self.pending_build.set(None);
        Ok(graph.graph.take().expect("built graph consumed once"))
    }

''' + source[end:]
# Only public bridge consumers change; the helper still yields native BuiltGraph.
source = source.replace('        graph: rhapsody::rg::BuiltGraph,', '        graph: BuiltCoreGraph,')
source = source.replace('    pub fn cancel_built(&mut self, graph: rhapsody::rg::BuiltGraph)', '    pub fn cancel_built(&mut self, graph: BuiltCoreGraph)')
needle = '/// A compiled, allocated frame. Dropping it before execution follows the\n'
assert needle in source
source = source.replace(needle, '''/// An uncompiled declaration tied to its originating App/session. Dropping it
/// cancels through that owner, retiring callbacks and frame imports exactly once.
#[must_use = "execute, compile or cancel this graph; dropping cancels its declaration"]
pub struct BuiltCoreGraph {
    session: Rc<RefCell<VulkanGraphSession>>,
    pending_build: Rc<Cell<Option<GraphId>>>,
    graph: Option<rhapsody::rg::BuiltGraph>,
}
impl BuiltCoreGraph {
    pub fn graph_id(&self) -> GraphId {
        self.graph.as_ref().expect("built graph consumed once").graph_id()
    }
}
impl Drop for BuiltCoreGraph {
    fn drop(&mut self) {
        if let Some(graph) = self.graph.take() {
            let id = graph.graph_id();
            if let Err(error) = self.session.borrow_mut().cancel_frame(graph) {
                log::warn!("cancelling uncompiled core frame: {error}");
            }
            if self.pending_build.get() == Some(id) {
                self.pending_build.set(None);
            }
        }
    }
}

''' + needle)
needle = '        if let Err(error) = self.bridge.session.borrow_mut().cancel_frame(graph.build()) {'
assert needle in source
source = source.replace(needle, '''        if self.bridge.pending_build.get() == Some(graph.graph_id()) {
            self.bridge.pending_build.set(None);
        }
''' + needle)
path.write_text(source, encoding='utf-8', newline='\n')

test = out/'candidate/tiny-glade/crates/tiny-glade/tests/core_render_bridge.rs'
source = test.read_text(encoding='utf-8')
source += '''

#[test]
fn dropped_uncompiled_graph_cancels_imports_and_leaves_the_session_reusable() {
    let Some(mut bridge) = bridge() else { return };
    let buffers = bridge.hosts().buffers.clone();
    let target = buffers.create_buffer(&BufferDesc::new(8, 0x22, 2), "abandoned declaration", None).unwrap();
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = bridge.build_frame(|frame| {
        frame.import_buffer(&target, 0)?;
        frame.with_graph(|graph| {
            graph.on_retire(move || flag.set(flag.get() + 1));
            Ok(())
        })
    }).unwrap();
    assert_eq!(bridge.with_session(|session| session.frame_import_count()), 1);
    assert_eq!(retired.get(), 0);
    drop(built);
    assert_eq!(retired.get(), 1);
    assert_eq!(bridge.with_session(|session| session.frame_import_count()), 0);
    assert_eq!(bridge.with_session(|session| session.pending_frame_count()), 0);
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
    assert!(buffers.release(&target));
}
'''
test.write_text(source, encoding='utf-8', newline='\n')

relative = 'tiny-glade/crates/tiny-glade/tests/original_raster_gpu.rs'
path = out/'candidate'/relative
source = (root/relative).read_text(encoding='utf-8')
assert source.count('    built: rhapsody::rg::BuiltGraph,') == 1
source = source.replace('    built: rhapsody::rg::BuiltGraph,', '    built: tiny_glade::render_bridge::BuiltCoreGraph,')
path.write_text(source, encoding='utf-8', newline='\n')
manifest = json.loads((out/'base.json').read_text())
manifest['files'].append({'path': relative, 'base_sha256': hashlib.sha256((root/relative).read_bytes()).hexdigest()})
(out/'base.json').write_text(json.dumps(manifest, indent=2)+'\n', encoding='utf-8')
print('v2 uses App-owned BuiltCoreGraph RAII; native rhapsody BuiltGraph/API unchanged.')
