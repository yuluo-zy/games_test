"""完整矩形 CPU 源码组合与原入口至 RET 比较，不进行游戏场景回放。"""
from pathlib import Path
import importlib.util,subprocess,json,hashlib,collections
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RBP
import struct
root=Path('D:/game/reverse-engineering/tiny-glade')
spec=importlib.util.spec_from_file_location('original_rect',root/'scripts/roof-ridge/capture_rectangular.py')
original=importlib.util.module_from_spec(spec);spec.loader.exec_module(original)
states={}
def hook(cpu,pc,size,user):
 if pc==0x1421b2099:
  rbp=cpu.reg_read(UC_X86_REG_RBP);states['final_rng']=struct.unpack('<Q',cpu.mem_read(rbp+0x3a8,8))[0]
original.cpu.hook_add(UC_HOOK_CODE,hook)
# Bind only modules actually imported by this runner, plus its locked libraries.
# A glob also included unrelated edit rules and made their changes invalidate
# a geometry proof even though the edit module is not linked into this runner.
sources=[root/'reconstruction/roof'/name for name in ['surface.rs','ridge.rs','tiles.rs','rectangular_edges.rs','rectangular_caps.rs','rectangular_faces.rs','pipeline.rs']]
sources += [root/'scripts/roof-pipeline-runner.rs',root/'reconstruction/Cargo.toml',root/'reconstruction/Cargo.lock']
exe=root/'workspace/roof-pipeline-reconstructed.exe'
import sys
sys.path.insert(0,str(root/'scripts'))
from dependency_build import build_runner
build_runner(root/'scripts/roof-pipeline-runner.rs',exe)
case_records=json.loads((root/'evidence/rectangular-edge-verification.json').read_text(encoding='utf-8'))['case_inputs']
cases=[(original.shape,bytes(original.roof),0)]
cases.extend((bytes.fromhex(c['shape']),bytes.fromhex(c['roof']),c['case']%2) for c in case_records)
expected=[];inputs=[];metadata=[]
for i,(shape,roof,flag) in enumerate(cases):
 states.clear();result=original.case(shape,roof,flag)
 assert result.get('completed') and not result.get('panic'),result
 expected.append(result['tile_records']);inputs.append(shape.hex()+' '+roof.hex()+' '+str(flag))
 phases=collections.Counter(result['tile_callsites']);seed=max(int.from_bytes(bytes.fromhex(t)[52:56],'little') for t in result['tile_records'])+1
 metadata.append(dict(case=i,shape=shape.hex(),roof=roof.hex(),special_mode=flag,records=len(result['tile_records']),phases=dict(phases),next_seed=seed,final_rng=states.get('final_rng')))
 print(json.dumps({'captured':i,'records':len(result['tile_records'])}),flush=True)
got=subprocess.run([str(exe)],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines()
failures=[]
for i,(expect,line) in enumerate(zip(expected,got)):
 parts=line.split();rng,seed,edges,caps,faces=map(int,parts[:5]);data=parts[5] if len(parts)>5 else ''
 actual=[data[j:j+128] for j in range(0,len(data),128)]
 if actual!=expect or seed!=metadata[i]['next_seed'] or rng!=metadata[i]['final_rng']:
  bad=next((j for j,(a,b) in enumerate(zip(expect,actual)) if a!=b),None)
  failures.append(dict(case=i,expected_records=len(expect),actual_records=len(actual),expected_seed=metadata[i]['next_seed'],actual_seed=seed,expected_rng=metadata[i]['final_rng'],actual_rng=rng,
   first_tile=bad,first_original=None if bad is None else expect[bad],first_candidate=None if bad is None else actual[bad]))
 metadata[i].update(candidate_rng=rng,candidate_seed=seed,candidate_stages=dict(edges=edges,caps=caps,faces_and_fillers=faces))
report=dict(ok=len(got)==len(expected) and not failures,source_sha256=hashlib.sha256(original.raw).hexdigest(),
 original_entry='0x1421aeef0',cases=len(cases),records=sum(map(len,expected)),failures=failures,inputs=metadata,
 candidate_sources={p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sources},
 oracle='Entire original rectangular function entry to RET; all numeric helpers original; bridge allocator/free/realloc, current host CRT math and Writer output sink only',
 candidate='Pure Rust entire edges -> caps -> faces/fillers composition from Rectangle/Roof input; no prepared geometry/RNG/ordinal injection',
 full_game_executed=False,scene_replay_performed=False,gpu_executed=False,
 limitations=['Valid pitched rectangular generator inputs; ECS setup/flat roof paths not implemented','Circle has separate whole-function verification','Synthetic shape dimensions/transforms combined with original JSON parameter snapshots','Float exception flags/allocator failure/panic formatting unverified'])
(root/'evidence/roof-pipeline-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'ok':report['ok'],'cases':len(cases),'records':report['records'],'failures':failures[:2]}))
if not report['ok']:raise SystemExit(1)
