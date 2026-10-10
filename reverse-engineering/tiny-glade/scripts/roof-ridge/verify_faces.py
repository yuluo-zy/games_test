import capture_faces as orig
from pathlib import Path
import json,struct,subprocess,random,math
ROOT=orig.orig.ROOT
reference=json.loads((ROOT/'evidence/roof-ridge/rectangular-reference.json').read_text(encoding='utf8'))
shape=bytes.fromhex(reference['rectangle_bytes']);roof=bytes.fromhex(reference['roof_bytes'])
lines=[];expected=[];fixtures=[]
rng=random.Random(20261012)
cases=[(shape,roof,0)]
for ci in range(12):
 b=bytearray(roof);ang=rng.uniform(-3.14,3.14);shapevals=[math.cos(ang),math.sin(ang),rng.uniform(-10,10),rng.uniform(-10,10),rng.uniform(2,9),rng.uniform(2,9)]
 for offset,value in [(0xc,shapevals[0]),(0x10,shapevals[1]),(0x14,shapevals[2]),(0x18,shapevals[3]),(0x1c,shapevals[4]),(0x20,shapevals[5]),(0x2c,rng.uniform(.1,1)),(0x30,rng.uniform(.2,.8)),(0x34,1.0 if ci%4==0 else rng.uniform(.05,.95)),(0x38,rng.uniform(.05,.8))]:struct.pack_into('<f',b,offset,value)
 b[0x3c]=ci%2;b[0x54]=(ci//2)%2
 cases.append((struct.pack('<6f',*shapevals),bytes(b),ci%2))
for ci,(shape,roof,flag) in enumerate(cases):
 result,faces=orig.run_case(shape,roof,flag)
 assert result.get('completed') and not result.get('panic'),result
 for face in faces:
  c=face['world_profile'];roofid=struct.unpack_from('<I',roof)[0]
  lines.append(' '.join([face['rows'],c['points'],c['u'],c['length_bits'],face['row_count_bits'],shape.hex(),face['planes'],str(roofid),str(flag),str(roof[0x54]&1),str(face['ordinal_before']),str(face['rng_before'])]))
  regular=sum(x=='0x1421b1bf2' for x in face['callsites'])
  expected.append(str(face['rng_after'])+' '+str(face['ordinal_before']+regular)+' '+''.join(face['tile_records']))
  fixtures.append(face)
 print('captured',ci,len(faces),flush=True)
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
  fails.append(dict(face=i,expected_state=aa[:2],actual_state=bb[:2],expected_records=len(aa[2])//128,actual_records=len(bb[2])//128,first_byte=None if pos is None else pos//2,expected_tile=None if pos is None else aa[2][(pos//128)*128:(pos//128+1)*128],actual_tile=None if pos is None else bb[2][(pos//128)*128:(pos//128+1)*128]))
report=dict(ok=not fails,cases=len(lines),tiles=sum(len(x['tile_records']) for x in fixtures),failures=fails,oracle='Complete original rectangular entry captures face-loop prepared inputs and output records; numeric dependencies run actual RNE')
(ROOT/'evidence/roof-ridge/faces-verification.json').write_text(json.dumps(report,indent=2),encoding='utf8');print(json.dumps(report,indent=2))
if fails:raise SystemExit(1)
