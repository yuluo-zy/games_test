import capture_faces as orig
from pathlib import Path
import json,struct,subprocess,random,math
ROOT=orig.orig.ROOT
ref=json.loads((ROOT/'evidence/roof-ridge/rectangular-reference.json').read_text(encoding='utf8'))
base_shape=bytes.fromhex(ref['rectangle_bytes']);base_roof=bytes.fromhex(ref['roof_bytes'])
lines=[];expected=[];meta=[];rng=random.Random(20261012)
cases=[(base_shape,base_roof,0)]
for ci in range(12):
 b=bytearray(base_roof);ang=rng.uniform(-3.14,3.14);v=[math.cos(ang),math.sin(ang),rng.uniform(-10,10),rng.uniform(-10,10),rng.uniform(2,9),rng.uniform(2,9)]
 for offset,value in [(0xc,v[0]),(0x10,v[1]),(0x14,v[2]),(0x18,v[3]),(0x1c,v[4]),(0x20,v[5]),(0x2c,rng.uniform(.1,1)),(0x30,rng.uniform(.2,.8)),(0x34,1.0 if ci%4==0 else rng.uniform(.05,.95)),(0x38,rng.uniform(.05,.8))]:struct.pack_into('<f',b,offset,value)
 b[0x3c]=ci%2;b[0x54]=(ci//2)%2
 cases.append((struct.pack('<6f',*v),bytes(b),ci%2))
fixtures=json.loads((ROOT/'evidence/roof-fixtures.json').read_text(encoding='utf8'))['samples']
selected=[];seen=set()
for source in fixtures:
 p=source['params'];key=json.dumps(p,sort_keys=True)
 if key in seen or p.get('height_01',0)<=.1:continue
 seen.add(key);selected.append(source)
 if len(selected)==16:break
for ci,source in enumerate(selected):
 p=source['params'];b=bytearray(base_roof);ang=(ci*.235)-1.0
 v=[math.cos(ang),math.sin(ang),float(ci%3),float(-(ci%5)),4.75,6.25]
 for offset,value in [(0xc,v[0]),(0x10,v[1]),(0x14,v[2]),(0x18,v[3]),(0x1c,v[4]),(0x20,v[5])]:struct.pack_into('<f',b,offset,value)
 for name,offset in [('profile_01',0x2c),('height_01',0x30),('ridge_length_01',0x34),('eave_length_01',0x38)]:
  if name in p:struct.pack_into('<f',b,offset,p[name])
 if 'tip_offset_01' in p:
  for offset,value in zip([0x24,0x28],p['tip_offset_01']):struct.pack_into('<f',b,offset,value)
 b[0x3c]=ci%2;b[0x54]=(ci//2)%2
 cases.append((struct.pack('<6f',*v),bytes(b),ci%2))
for ci,(shape,roof,flag) in enumerate(cases):
 result,faces=orig.run_case(shape,roof,flag)
 assert result.get('completed') and not result.get('panic'),result
 regular=sum(x=='0x1421b1bf2' for f in faces for x in f['callsites'])
 line=' '.join(['full',shape.hex(),roof.hex(),str(flag),str(faces[0]['ordinal_before']),str(faces[0]['rng_before'])])
 lines.append(line);expected.append(str(faces[-1]['rng_after'])+' '+str(faces[0]['ordinal_before']+regular)+' '+''.join(t for f in faces for t in f['tile_records']))
 meta.append(dict(case=ci,faces=len(faces),records=sum(len(f['tile_records']) for f in faces)))
 print('captured',ci,flush=True)
exe=ROOT/'workspace/faces-reconstructed.exe'
import sys
sys.path.insert(0,str(ROOT/'scripts'))
from dependency_build import build_runner
build_runner(ROOT/'scripts/roof-ridge/faces_runner.rs',exe)
got=subprocess.run([str(exe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
fails=[]
for i,(a,b) in enumerate(zip(expected,got)):
 if a!=b:
  aa=a.split();bb=b.split();pos=next((j for j,(x,y) in enumerate(zip(aa[2],bb[2])) if x!=y),None)
  fails.append(dict(case=i,expected_state=aa[:2],actual_state=bb[:2],expected_records=len(aa[2])//128,actual_records=len(bb[2])//128,first_byte=None if pos is None else pos//2,expected_tile=None if pos is None else aa[2][(pos//128)*128:(pos//128+1)*128],actual_tile=None if pos is None else bb[2][(pos//128)*128:(pos//128+1)*128]))
report=dict(ok=not fails,cases=len(lines),records=sum(m['records'] for m in meta),failures=fails,oracle='Entire original rectangular entry captured actual face/filler subsequence; Rust reconstructs context, profile, rowcount, row interpolation, planes, loop, random stream and packed records from original Rectangle/Roof bytes. Only starting RNG/ordinal supplied from original edge+cap result',cases_metadata=meta,original_parameter_snapshots=selected,limitations=['Geometry tests include the exact f32 height threshold and ordinary positive dimensions; actual ECS caller coverage is not asserted','Shape dimensions/transform synthesized because histories reference separate Wall geometry','RNG/ordinal start supplied by observed preceding stages, to be composed into total rectangle pipeline'])
(ROOT/'evidence/roof-ridge/composed-faces-verification.json').write_text(json.dumps(report,indent=2),encoding='utf8')
(ROOT/'evidence/roof-ridge/composed-faces-cases.json').write_text(json.dumps(dict(input_lines=lines,expected_lines=expected,metadata=meta)),encoding='utf8')
print(json.dumps({k:v for k,v in report.items() if k!='cases_metadata'},indent=2))
if fails:raise SystemExit(1)
