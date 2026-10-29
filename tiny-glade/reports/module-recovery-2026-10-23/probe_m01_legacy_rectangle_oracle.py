"""只复核两个旧测试：按既有输入原位运行发行完整函数，不实现几何算法。"""
from pathlib import Path
import json,hashlib,struct,subprocess,tempfile
R=Path(__file__).resolve().parent;root=R.parents[2]
old=root/'reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/geometry/native-fixtures.json'
if not old.exists():old=root.parent/'reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/geometry/native-fixtures.json'
original=root/'B版本内容/tiny-glade.rne'
x=json.loads(old.read_text());assert hashlib.sha256(original.read_bytes()).hexdigest()==x['original_binary_sha256']
rows=[z for z in x['rows'] if z['group'] in ('original_completion_rectangle_deformation','original_completion_rectangle_intersections')]
inputs=[]
for i,z in enumerate(rows):
 if z['group'].endswith('deformation'):v=[0,i,*z['rectangle'],z['edge'],z['delta'],*z['limits'],*z['matrix']]
 else:v=[1,i,*z['rectangle'],*z['other'],*z['circle']]
 inputs.append(struct.pack('<24I',*(v+[0]*(24-len(v)))))
with tempfile.TemporaryDirectory(prefix='m01-legacy-rectangle-') as tmp:
 t=Path(tmp);(t/'input').write_bytes(b''.join(inputs));source=R/'probe_m01_legacy_rectangle_oracle_x86.c'
 subprocess.run(['clang','-arch','x86_64','-O2',str(source),'-o',str(t/'probe')],check=True)
 run=subprocess.run([str(t/'probe'),str(original),str(t/'input'),str(t/'output')],check=True,capture_output=True,text=True)
 output=(t/'output').read_bytes();assert len(output)==len(rows)*72
 results=[]
 for z,raw in zip(rows,struct.iter_unpack('<18I',output)):
  op,i,panic,*words=raw;assert not panic
  if op==0:actual={'expanded':words[:6],'rotated':words[6:14]};expected={'expanded':z['expanded'],'rotated':z['rotated']}
  else:actual={'rectangle_intersects':bool(words[0]),'circle_intersects':bool(words[1])};expected={'rectangle_intersects':z['rectangle_intersects'],'circle_intersects':z['circle_intersects']}
  results.append({'index':i,'group':z['group'],'input':{k:v for k,v in z.items() if k not in expected and k!='group'},'old_expected':expected,'original_direct':actual,'differs':actual!=expected})
 report={'original_sha256':x['original_binary_sha256'],'old_fixture_sha256':hashlib.sha256(old.read_bytes()).hexdigest(),'native_source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'native_executable_sha256':hashlib.sha256((t/'probe').read_bytes()).hexdigest(),'execution':'Rosetta x86_64, full RNE PE image, original complete functions and all normal callees; only panic exit bridge','input_sha256':hashlib.sha256(b''.join(inputs)).hexdigest(),'output_sha256':hashlib.sha256(output).hexdigest(),'native_panics':0,'rows':results,'stdout':run.stdout,'stderr':run.stderr}
 (R/'m01-legacy-rectangle-oracle-direct-replay.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'rows':len(results),'differing_old_rows':sum(z['differs'] for z in results),'examples':[z for z in results if z['differs']][:3]}))
