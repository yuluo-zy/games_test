from pathlib import Path
import json,subprocess,importlib.util,struct,math,random,hashlib
root=Path('D:/game/reverse-engineering/tiny-glade')
spec=importlib.util.spec_from_file_location('edge_capture',root/'scripts/capture-rectangular-edges.py')
capture=importlib.util.module_from_spec(spec);spec.loader.exec_module(capture)
import sys
sys.path.insert(0,str(root/'scripts'))
from dependency_build import build_runner
build_runner(root/'scripts/edge-runner.rs',root/'workspace/edge-runner.exe')
fixtures=json.loads((root/'evidence/roof-fixtures.json').read_text(encoding='utf-8'))['samples']
fixtures=[s for s in fixtures if s['params'].get('height_01',0)>0.10001]
rng=random.Random(20261010)
cases=[]
for i in range(32):
    roof=bytearray(capture.oracle.roof)
    angle=[0.0,0.4,1.1,-0.8][i%4];cos,sin=math.cos(angle),math.sin(angle)
    x,z=rng.uniform(-4,4),rng.uniform(-4,4);w,l=rng.uniform(3,10),rng.uniform(3,12)
    params=fixtures[(i*7)%len(fixtures)]['params']
    for off,value in [(0xc,cos),(0x10,sin),(0x14,x),(0x18,z),(0x1c,w),(0x20,l),
      (0x24,params['tip_offset_01'][0]),(0x28,params['tip_offset_01'][1]),(0x2c,params['profile_01']),
      (0x30,params['height_01']),(0x34,params['ridge_length_01']),(0x38,params['eave_length_01'])]:
        struct.pack_into('<f',roof,off,value)
    if i>=24:
        struct.pack_into('<f',roof,0x2c,1.0)
    roof[0x3c]=i%2;roof[0x54]=(i//2)%2
    shape=struct.pack('<6f',cos,sin,x,z,w,l)
    cases.append((shape,bytes(roof),i%2,fixtures[(i*7)%len(fixtures)]['pointer']))
inputs=[];original=[];case_info=[];unsupported=[]
for i,(shape,roof,flag,pointer) in enumerate(cases):
    reference=capture.run(shape,roof,flag)
    if not reference['original_entry_completed']:
        unsupported.append(dict(case=i,panic=reference['panic'],error=reference['error']))
        continue
    inputs.append('full '+roof.hex()+' '+shape.hex())
    original.append(reference['edge_records'])
    case_info.append(dict(case=i,fixture_pointer=pointer,roof=roof.hex(),shape=shape.hex(),tiles=len(reference['edge_records']),edge_end=reference['edge_end']))
result=subprocess.run([str(root/'workspace/edge-runner.exe')],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True)
actual=result.stdout.splitlines();failures=[]
for i,(expected,line) in enumerate(zip(original,actual)):
    words=line.split();hexbytes=words[1] if len(words)>1 else ''
    got=[hexbytes[j:j+128] for j in range(0,len(hexbytes),128)]
    rng_ok=int(words[0],16)==case_info[i]['edge_end']['rng']
    if got!=expected or not rng_ok:
        mismatches=[dict(tile=j,offsets=[k for k,(a,b) in enumerate(zip(bytes.fromhex(e),bytes.fromhex(g))) if a!=b],original=e,candidate=g) for j,(e,g) in enumerate(zip(expected,got)) if e!=g]
        failures.append(dict(case=case_info[i],expected_count=len(expected),actual_count=len(got),rng_ok=rng_ok,mismatch_count=len(mismatches),first=mismatches[:4]))
report=dict(ok=len(actual)==len(original) and bool(original) and not failures,source_sha256=capture.oracle.hashlib.sha256(capture.oracle.raw).hexdigest(),
  original_entry='0x1421aeef0',core_range='0x1421af190..0x1421afbfb',oracle='Original complete rectangle entry through RET; compare edge Writer calls',
  candidate='Rust from roof/rectangle including original bounds/basis/corners/profile chain; no prepared geometry injection',
  cases=len(original),attempted_cases=len(cases),total_edge_records=sum(map(len,original)),failures=failures,unsupported=unsupported,case_inputs=case_info,
  full_game_executed=False,limitations=['Only rectangular edge records verified here; caps/face/fillers separate','Synthetic rectangle sizes/transform; actual JSON roof params plus explicit gable variants','CRT/allocator/Writer bridges documented in original capture harness','Floating exception flags/invalid shapes not covered'])
(root/'evidence/rectangular-edge-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print(json.dumps({'ok':report['ok'],'cases':report['cases'],'tiles':report['total_edge_records'],'unsupported':len(unsupported),'failures':len(failures),'first':failures[:1]}))
if not report['ok']:raise SystemExit(1)
