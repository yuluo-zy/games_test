from pathlib import Path
out = Path(__file__).parent
path = out/'candidate/tiny-glade/crates/rhapsody/src/rg/vulkan.rs'
source = path.read_text(encoding='utf-8')
old = '''        } else {
            named.state.abandon_frame();
            self.result_cache.abort(graph_id);
            self.cancel_presentation_of(graph_id);
        }
        self.named = Some(named);'''
new = '''        } else {
            named.state.abandon_frame();
            if named.graph_id == Some(graph_id) {
                named.graph_id = None;
            }
            self.result_cache.abort(graph_id);
            self.cancel_presentation_of(graph_id);
        }
        self.named = Some(named);'''
assert source.count(old) == 1
source = source.replace(old, new)
old = '''        let result = run_retire_callbacks(frame.retire);
        if let Some(named) = &mut self.named {
            named.state.abandon_frame();
        }
        self.collect_frame_imports();'''
new = '''        let result = run_retire_callbacks(frame.retire);
        if let Some(named) = &mut self.named {
            named.state.abandon_frame();
            named.graph_id = None;
        }
        self.collect_frame_imports();'''
assert source.count(old) == 1
source = source.replace(old, new)
old = '''        self.collect_frame_imports();
        result
    }

    fn recycle_completed_commands'''
new = '''        // The declaration was consumed regardless of submit success. Pending
        // GPU work and committed history have their own independent owners.
        if let Some(named) = &mut self.named {
            named.graph_id = None;
        }
        self.collect_frame_imports();
        result
    }

    fn recycle_completed_commands'''
assert source.count(old) == 1
source = source.replace(old, new)
path.write_text(source, encoding='utf-8', newline='\n')

path = out/'candidate/tiny-glade/crates/tiny-glade/tests/original_scene_compute_gpu.rs'
source = path.read_text(encoding='utf-8')
old = '''    fixture.bridge.cancel_built(unsubmitted).unwrap();
    fixture.bridge.accept_gameplay_warmup(&completed).unwrap();'''
new = '''    fixture.bridge.cancel_built(unsubmitted).unwrap();
    let (_, unsubmitted) = fixture.bridge.build_frame(|_| Ok(())).unwrap();
    let compiled = fixture.bridge.compile_built(unsubmitted).unwrap();
    assert!(fixture.bridge.accept_gameplay_warmup(&completed).is_err());
    drop(compiled);
    fixture.bridge.accept_gameplay_warmup(&completed).unwrap();'''
assert source.count(old) == 1
source = source.replace(old,new)
path.write_text(source, encoding='utf-8', newline='\n')
print('Graph declaration identities close on compile failure, prepared cancellation, and execution terminal paths.')
