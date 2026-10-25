from pathlib import Path
import json,hashlib,difflib,collections
root=Path('/Users/yuluo/Desktop/game');out=Path(__file__).resolve().parent
base=root/'games_test/tiny-glade';reports=base/'reports/core-systems-resources-recovery-2026-10-18'
sha=lambda b:hashlib.sha256(b).hexdigest()
changed=['crates/country-core/src/startup/run_game.rs','crates/country-core/src/systems/terrain/mod.rs','crates/country-core/tests/core_editing_schedule.rs']
diffs=[];inputs=[]
for rel in changed:
 before=reports/'mode-registration-before'/rel;after=base/rel
 diffs.append(''.join(difflib.unified_diff(before.read_text().splitlines(True),after.read_text().splitlines(True),fromfile='root-before/'+rel,tofile='root-current/'+rel)))
 inputs +=[{'role':'before','path':str(before),'sha256':sha(before.read_bytes())},{'role':'current','path':str(after),'sha256':sha(after.read_bytes())}]
(out/'adapter.diff').write_text(''.join(diffs))
for rel in['crates/country-core/src/systems/mode_manager.rs','crates/country-core/src/systems/backend/input/mod.rs','crates/country-core/src/systems/history.rs','crates/country-core/src/resources/history/mod.rs','crates/country-core/src/resources/gui.rs','crates/country-core/src/systems/ui/resources/ui_state.rs','crates/tiny-glade/src/run_game.rs']:
 p=base/rel;inputs.append({'role':'read_dependency','path':str(p),'sha256':sha(p.read_bytes())})
# Reduced dependency projection of explicit current adapter edges. This is
# not a replacement for Bevy graph initialization, which root owns.
edges=[('Prepare','ControlEarlySend'),('ControlEarlySend','Clock'),('Clock','Translate'),('Translate','Resolve'),('Resolve','RecordReplay'),('RecordReplay','Derive'),('Derive','Consume'),('Consume','Finish'),('Derive','UiProjectionSync'),('UiProjectionSync','ModeManager'),('Derive','TerrainHover'),('TerrainHover','FocusProducer'),('FocusProducer','FocusMode'),('FocusProducer','ModeManager'),('ModeManager','LateSend'),('ControlEarlySend','FocusMode'),('LateSend','FocusMode'),('FocusMode','DrawUi'),('DrawUi','EditUi'),('EditUi','Arrow'),('Arrow','TerrainEdit'),('EditUi','TerrainEdit'),('Derive','HistoryCursor'),('ControlEarlySend','HistoryCursor'),('LateSend','HistoryCursor'),('HistoryCursor','HistoryReplay'),('ControlEarlySend','HistoryReplay'),('LateSend','HistoryReplay'),('HistoryReplay','HistoryApply'),('HistoryApply','HistoryPop'),('HistoryPop','HistoryRecord'),('HistoryRecord','Finish'),('HistoryReplay','TerrainEdit'),('TerrainEdit','TerrainChanged'),('TerrainChanged','TerrainReadback'),('TerrainChanged','HistoryPop')]
def topo(es):
 nodes=set(n for e in es for n in e);adj=collections.defaultdict(list);ind={n:0 for n in nodes}
 for a,b in es:adj[a].append(b);ind[b]+=1
 ready=collections.deque(sorted(n for n in nodes if not ind[n]));order=[]
 while ready:
  a=ready.popleft();order.append(a)
  for b in adj[a]:
   ind[b]-=1
   if not ind[b]:ready.append(b)
 return order,sorted(n for n in nodes if ind[n])
order,cyclic=topo(edges);assert not cyclic
_,hypothetical_cycle=topo(edges+[('ModeManager','ControlEarlySend')]);assert hypothetical_cycle
report={'scope':'current core startup adapter only; static reduced graph, no Cargo/Bevy execution','sources':inputs,'edges':edges,'topological_order':order,'cyclic_nodes':cyclic,'function_type_set_note':'send_mode_just_changed_event has an existing early InputControl instance and a late configure_mode_manager instance. Current chain uses instance-local edges; after(send function type) intentionally waits for both. A global ModeManager.before(send function type) would point back into InputControl and cycle; current code does not add it.','incorrect_extra_mode_before_early_send_would_cycle':hypothetical_cycle,'original_global_schedule_claimed':False,'current_host_calls_core_startup_found':False,'current_host_calls_terrain_helpers_found':False,'production_edited':False,'cargo_run':False}
(out/'review-evidence.json').write_text(json.dumps(report,indent=2)+'\n')
print({'source_inputs':len(inputs),'edges':len(edges),'reduced_graph_acyclic':not cyclic,'current_sources':[(x['path'],x['sha256'])for x in inputs if x['role']=='current']})
