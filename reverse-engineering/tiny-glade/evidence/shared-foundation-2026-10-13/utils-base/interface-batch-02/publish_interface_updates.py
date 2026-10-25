"""Publish actual per-original-interface status changes, not added Rust declarations."""
from pathlib import Path
import hashlib,json
OUT=Path(__file__).parent;PARENT=OUT.parent
baseline_path=OUT/'before-interface-coverage.json'
if not baseline_path.exists():baseline_path.write_bytes((PARENT/'interface-coverage.json').read_bytes())
baseline=json.loads(baseline_path.read_text(encoding='utf8'))
updates=[]
for r in baseline['interfaces']:
    key=r['canonical_key']
    if r['module']=='swizzlers':
        method=key.rsplit('::',1)[-1];transport=method=='as_vec2'
        updates.append({'canonical_key':key,'analysis_status':'analyzed_partial','candidate_implementation':{'file':'reconstruction/shared-foundation/crates/utils/src/swizzlers.rs','method':key.split(' as ')[-1].replace('>',''),'scope':'borrowed pointer/length transport only; projected consumer API remains unknown' if transport else 'original coordinate lane policy'},'business_analysis_evidence':['interface-batch-02/targets.json','interface-batch-02/query-bounded.json','interface-batch-02/B-module-757.txt','interface-batch-02/swizzler-native-fixtures.json'],'static_comparison':True,'original_instruction_differential':True,'original_instruction_differential_scope':'6 pointer/length identity cases, not element projection' if transport else '64 payload-bit patterns per original coordinate method','consumer_wired':False,'gaps':['Original return wrapper type, projected axis/element stride/borrow aliases and actual consumer interpretation unresolved; safe borrowed transport does not cast &[Vec3] to packed &[Vec2].'] if transport else ['Original Rust receiver-by-value/reference ABI and full game consumer wiring unverified; coordinate values/lane rules verified.']})
    elif key in ['utils::index_condenser::IndexCondenser::get','utils::index_condenser::IndexCondenser::clear','utils::index_condenser::Condex::last_known_index']:
        method=key.rsplit('::',1)[-1]
        updates.append({'canonical_key':key,'analysis_status':'analyzed_partial','candidate_implementation':{'file':'reconstruction/shared-foundation/crates/utils/src/index_condenser.rs','method':key},'business_analysis_evidence':['interface-batch-02/inline-bindings.json','interface-batch-02/B-module-540.txt','interface-batch-02/B-module-523.txt','interface-batch-02/B-module-554.txt','interface-batch-02/index-get-native-fixtures.json','interface-batch-02/index_tests.rs'],'static_comparison':True,'original_instruction_differential':False,'original_inline_differential':method=='get','original_inline_differential_scope':'72 exact original get-prefix success/missing branches, cache updates; std hash/probe original instructions execute' if method=='get' else None,'consumer_wired':method=='get','gaps':['No standalone original get entry: original inlined code compared at precise IPI0x74E3 boundaries. Recovered remove now calls get; full GPUWorld update consumer remains unwired.'] if method=='get' else ['Original clear confirmed inside uninit_clearing, calling original Vec/HashMap clear; candidate capacity/reset tests only, full original startup consumer not executed.'] if method=='clear' else ['Original IPI0x74F0 exposes last cached index in producer register; safe Cell getter restores value policy, no independent native method entry/ABI.']})
assert len(updates)==8
(OUT/'interface-state-updates.json').write_text(json.dumps({'baseline':'B','baseline_interface_count':baseline['statistics']['observed_private_interfaces'],'baseline_sha256':hashlib.sha256(baseline_path.read_bytes()).hexdigest(),'updates':updates,'states_are_partial_not_complete':True},ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('original interfaces updated',len(updates))
