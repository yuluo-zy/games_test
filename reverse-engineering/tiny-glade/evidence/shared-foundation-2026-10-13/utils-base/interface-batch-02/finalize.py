"""Snapshot the actual checked small-interface delivery and status delta."""
from pathlib import Path
import json,hashlib
OUT=Path(__file__).parent;PARENT=OUT.parent;ROOT=OUT.parents[3]
v=json.loads((PARENT/'verification.json').read_text(encoding='utf8'))
c=json.loads((PARENT/'interface-coverage.json').read_text(encoding='utf8'))
b=json.loads((OUT/'before-interface-coverage.json').read_text(encoding='utf8'))
updates=json.loads((OUT/'interface-state-updates.json').read_text(encoding='utf8'))
for filename in ['utils-test.log','utils-clippy.log','selective-policy.log']:
    (OUT/filename).write_bytes((PARENT/filename).read_bytes())
v['interface_statistics_before']=b['statistics'];v['interface_statistics_after']=c['statistics']
v['batch_interfaces_touched']=[x['canonical_key'] for x in updates['updates']]
v['new_original_cases']={'swizzler_coordinate':256,'swizzler_borrowed_transport_only':6,'index_get_inline_prefix':72}
v['new_candidate_contracts']=['clear preserves capacity and invalidates old keys','safe borrowed adapter preserves source storage']
v['modified_source_files']={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [ROOT/'reconstruction/shared-foundation/crates/utils/src/swizzlers.rs',ROOT/'reconstruction/shared-foundation/crates/utils/src/index_condenser.rs']}
v['worker']='foundation-utils-base';v['database']='D:/game/tools/reverse-runtime/foundation-utils-base/project/TinyGlade.gpr'
v['full_original_game_or_GPU_runtime']=False
(OUT/'verification.json').write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('whole checks',v['all_passed'],'untouched',b['statistics']['untouched'],'->',c['statistics']['untouched'])
