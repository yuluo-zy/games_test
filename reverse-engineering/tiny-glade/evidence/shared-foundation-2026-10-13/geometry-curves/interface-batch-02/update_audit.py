"""Apply reviewed batch evidence to the original fixed 326-interface inventory."""
from pathlib import Path
import json,collections
OUT=Path(__file__).resolve().parent
BASE=OUT.parent
path=BASE/'interface-coverage.json'
d=json.loads(path.read_text(encoding='utf-8'))
fixtures=json.loads((OUT/'circular-native-fixtures.json').read_text())
methods={'wrap_around':'wrap','wrapping_lerp':'lerp','wrapping_contains':'contains'}
for r in d['interfaces']:
 if r['original_source']!='crates/utils/src/curve/curve_u_circular.rs':continue
 method=r['method'];k=methods.get(method,method)
 r.update(status='native_compared',business_analysis_started=True,has_recovered_implementation=True,has_original_comparison=True,candidate_crate_exported=True,original_consumer_chain_verified=False,integrated=False)
 r['ownership']='private_game_business_interface'
 r['ownership_evidence'].append('Original private inline definition + explicit scalar operations; From checks CurveU guard and eq compares the private value.')
 r['restored_file']='reconstruction/shared-foundation/crates/utils/src/curve/curve_u_circular.rs'
 r['original_comparison']=[{'artifact':str((OUT/'circular-native-fixtures.json').relative_to(BASE.parents[2])).replace('\\','/'),'cases':fixtures['counts'][k],'comparison':'bits_except_NaN_payload_boolean_and_panic','scope':'unmodified inline computation with caller registers, checked control-flow exits; not a standalone Rust ABI','direct_original_entry':False,'inline_leaf_projection':True}]
 r['business_rules']=['NaN input assertion precedes remainder; infinity can project to NaN.','Add/subtract project without new assertion.','Forward distance and unbounded lerp choose end+1 when end<start; original lerp dependency is reused.','Contains inclusive forward interval, same endpoints single point.','From<Circular> for CurveU rechecks inclusive 0..1; equality follows scalar f32 rules.']
 r['gaps']=['Original consumers located as donor evidence, but full stairs/dormer/half-timber consumer state is not replayed.','Typed ABI/layout and exact panic messages are not reconstruction targets; NaN payload is not compared.']
 r['all_observed_instantiations_native_compared']=False
 r['restored_declarations']=[{'declaration':'Original semantic method recovered; trait mappings reviewed explicitly','owner_context':'CurveUCircular<S>'}]
 r['batch_evidence']='geometry-curves/interface-batch-02'
d['status_counts']=dict(collections.Counter(r['status']for r in d['interfaces']))
d['ownership_counts']=dict(collections.Counter(r['ownership']for r in d['interfaces']))
d['status_counts_by_ownership']={o:dict(collections.Counter(r['status']for r in d['interfaces']if r['ownership']==o))for o in d['ownership_counts']}
for field in ['business_analysis_started','has_recovered_implementation','has_original_comparison','candidate_consumer_wired','original_consumer_chain_verified','integrated','decompiled_evidence_present']:
 if field in d['independent_counts']:d['independent_counts'][field]=sum(r[field]for r in d['interfaces'])
for f in d['files']:
 rr=[r for r in d['interfaces']if r['original_source']==f['original_source']];f['status_counts']=dict(collections.Counter(r['status']for r in rr))
d['strategy_variants']['CurveUCircular']={'status':'native_compared_inline_projection','generic_semantic_marker':'Scalar rules do not branch on S; emitted donor instances are actual observed specializations only.','forward_distance':True,'forward_lerp':True,'shortest_arc':False,'nan_payload_compared':False,'original_consumer_chain_verified':False}
assert len(d['interfaces'])==326
path.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
(OUT/'coverage-delta.json').write_text(json.dumps({'original_known_denominator':326,'newly_business_analyzed':9,'newly_implemented':9,'newly_native_compared':9,'resolved_pending_ownership':2,'status_counts':d['status_counts'],'status_counts_by_ownership':d['status_counts_by_ownership'],'whole_original_consumer_chains_newly_verified':0},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'status_counts':d['status_counts'],'independent_counts':d['independent_counts'],'ownership_counts':d['ownership_counts']}))
