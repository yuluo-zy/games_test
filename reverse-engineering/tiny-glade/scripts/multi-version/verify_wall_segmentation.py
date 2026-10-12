"""Complete native B calc -> mask flags/coords/range/length differential."""
import sys,json,math,struct,subprocess,random,hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_segmentation import produce

def bits(f):return struct.pack('<f',f).hex()
def word(f):return f'{struct.unpack("<I",struct.pack("<f",f))[0]:08x}'
def main():
 rng=random.Random(20261024);cases=[]
 for n in [3,5,9,17,33]:
  for scale in [.25,1,2,4]:
   for pattern in range(8):
    pts=[[j*scale,(j%2)*(.0 if pattern<4 else 1.5),(j%2)*(.0 if pattern%4<2 else .75)]for j in range(n)]
    vals=[0 if j in [0,n-1] else [.3,1,3,.3,1,3,1,.3][pattern] for j in range(n)]
    if pattern==7:vals=[0 if j%4==0 else 1.0 for j in range(n)]
    cases.append((pts,vals,2,pattern%2))
 for n in [5,9,17,33]:
  for radius in [1.,3.,6.]:
   pts=[[radius*math.cos(j*2*math.pi/(n-1)),.15*(j%3),radius*math.sin(j*2*math.pi/(n-1))]for j in range(n)]
   pts[-1]=pts[0].copy()
   for pattern in range(4):
    vals=[[0.,.7,2.,.2][(j+pattern)%4]for j in range(n)];vals[-1]=vals[0]
    cases.append((pts,vals,pattern%2,pattern%2))
 for i in range(384):
  n=rng.randrange(3,80);pts=[[j*rng.choice([.125,.4,1.,2.]),rng.uniform(-4,4),rng.uniform(-3,3)]for j in range(n)];vals=[rng.choice([0.,.1,.5,2.])for _ in range(n)];cases.append((pts,vals,2,i%2))
 # Real upstream selected4rasters -> prepared original producer -> masks.
 sys.path.insert(0,str(ROOT/'evidence/building-production-authority'))
 from native_path_empty import prepare
 from verify_wall_path_nonempty import raster
 from verify_wall_path_preparation import f32
 prepared_cases=0;prepared_points=0
 for ci in range(64):
  n=[3,5,8,17][ci%4];pts=[[f32(-4+j*8/(n-1)),f32((j%3)*.2),f32((j%3)*.1)]for j in range(n)]
  if ci%7==0:pts.append(pts[0])
  rel=[.8 if ci%4==0 and j%3==1 else 0. for j in range(len(pts))]
  pattern=[(ci%4,252+ci%3,260-ci%2,250,260,[1,5,10,16,24,32,128,255][ci%8])]
  out=prepare(pts,rel,[.6,1.,2.,-.6][ci%4],bool(ci%2),raster(pattern));raw=bytes.fromhex(out['points']);vals=list(struct.unpack('<'+'f'*out['count'],bytes.fromhex(out['segments'])));pts=[list(struct.unpack_from('<3f',raw,j*12))for j in range(out['count'])]
  cases.append((pts,vals,2,ci%2));prepared_cases+=1;prepared_points+=len(pts)
 lines=[];expected=[]
 for ci,(pts,vals,shape,special) in enumerate(cases):
  fields=rng.choice([(.2,2.,3.),(1.6,1.5,3.),(2.2,2.5,3.),(3.,4.,6.)]);out=produce(pts,vals,shape,special,fields)
  for m in out:assert m['lazy_headers']==['000000000000000004000000000000000000000000000000']*2+['000000000000000008000000000000000000000000000000']
  expected.append(';'.join(','.join([m['flags'],m['coords'],m['range'],f'{struct.unpack("<I",bytes.fromhex(m["length"]))[0]:08x}'])for m in out))
  lines.append(f'full {shape} {special} {len(pts)} '+ ' '.join(word(v)for v in fields)+' '+' '.join(word(v)for p,h in zip(pts,vals)for v in [*p,h]))
 probe=ROOT/'reconstruction/target/release/wall-segmentation-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines()
 fail=[{'case':i,'input':lines[i],'original':want,'rust':got}for i,(want,got)in enumerate(zip(expected,actual))if want!=got]
 report={'ok':not fail,'cases':len(cases),'actual_prepared_chain_cases':prepared_cases,'actual_prepared_chain_points':prepared_points,'mask_count':sum(len(v.split(';'))if v else 0 for v in expected),'failure_count':len(fail),'failures':fail[:30],'scope':'Complete original calc14074f7a0 -> 136B safeprojection flags/coords/range/length; lazycache Vec headers originalempty. Preparedinput arrays explicit.','oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-production-authority/native_path_empty.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'scripts/multi-version/verify_wall_segmentation.py']},'binary_sha256':__import__('native_segmentation').b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'native_entry':'0x14074f7a0','excluded':['Original outerEventReader/PublicWalls/HashMap insertion','MaterialStats opaque tail schema','A/C execution','Nonfiniteinput or mismatched publiccounts','Originalforkschedule and GPU'], 'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'reconstruction/building/wall_segmentation.rs',ROOT/'reconstruction/src/bin/wall-segmentation-probe.rs',ROOT/'reconstruction/roof/surface.rs',ROOT/'reconstruction/Cargo.lock',ROOT/'reconstruction/Cargo.toml']}}
 (ROOT/'evidence/building-segmentation/B-masks-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'first':fail[:2]}));assert not fail
if __name__=='__main__':main()
