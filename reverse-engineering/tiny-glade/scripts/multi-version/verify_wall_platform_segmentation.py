"""Original calc -> actualplatform helpers -> processed masks+steps."""
import sys,random,json,struct,math,subprocess,hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_platform_segmentation import process
from native_fixture import b

def word(f):return f'{struct.unpack("<I",struct.pack("<f",f))[0]:08x}'
def main():
 rng=random.Random(20261026);cases=[]
 for i in range(256):
  n=[3,5,9,17,33][i%5];shape=i%4;special=i%2;maximum=[1.,3.55,4.,5.,8.][i%5];roof=[None,0,0x3dcccccd,0x3f800000,0x7fc00000,0xbdcccccd][i%6]
  if shape<2:
   pts=[[3*math.cos(j*2*math.pi/(n-1)),[0.,.55,1.5,3.,4.][(j+i)%5],3*math.sin(j*2*math.pi/(n-1))]for j in range(n)];pts[-1]=pts[0].copy()
  else:pts=[[j*.6,[0.,.55,1.5,3.,4.][(j+i)%5],.2*(j%3)]for j in range(n)]
  vals=[rng.choice([0.,.3,1.,2.])for j in range(n)]
  if shape<2:vals[-1]=vals[0]
  cases.append((pts,vals,shape,special,maximum,roof))
 for i in range(128):
  n=rng.randrange(3,40);maximum=rng.uniform(1,8);pts=[[j*.5,rng.uniform(-1,5),rng.uniform(-.5,.5)]for j in range(n)];vals=[rng.choice([0.,.5,1.])for j in range(n)];cases.append((pts,vals,i%2,i%2,maximum,0))
 lines=[];expected=[];maskcount=0;stepcount=0
 for pts,vals,shape,special,maximum,roof in cases:
  masks,steps=process(pts,vals,shape,special,maximum,roof);maskcount+=len(masks);stepcount+=len(steps)
  out=';'.join(','.join([m['flags'],m['coords'],m['range'],f'{struct.unpack("<I",bytes.fromhex(m["length"]))[0]:08x}'])for m in masks)+'|'+ ';'.join(f'{s["start"]},{s["end"]},{s["range"]}'for s in steps);expected.append(out)
  lines.append(f'{shape} {special} {word(maximum)} 1 '+('none'if roof is None else f'{roof:08x}')+f' {len(pts)} '+' '.join(word(v)for p,h in zip(pts,vals)for v in [*p,h]))
 probe=ROOT/'reconstruction/target/release/wall-platform-segmentation-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True)
 if run.returncode:raise RuntimeError(run.stderr[:2500])
 actual=run.stdout.splitlines();assert len(actual)==len(expected)
 fail=[{'case':i,'input':lines[i],'original':want,'rust':got}for i,(want,got)in enumerate(zip(expected,actual))if want!=got]
 files=[ROOT/'reconstruction/building/wall_platform_segmentation.rs',ROOT/'reconstruction/building/wall_segmentation.rs',ROOT/'reconstruction/src/bin/wall-platform-segmentation-probe.rs',ROOT/'reconstruction/building/wall_state.rs',ROOT/'reconstruction/roof/roof_state.rs',ROOT/'reconstruction/roof/surface.rs',ROOT/'reconstruction/wall/height_rules.rs',ROOT/'reconstruction/Cargo.lock',ROOT/'reconstruction/Cargo.toml']
 report={'ok':not fail,'cases':len(cases),'processed_mask_count':maskcount,'step_count':stepcount,'failure_count':len(fail),'failures':fail[:20],'native_entries':['0x14074f7a0','0x1408e3380','0x141202370','0x141205db0','0x1411ffbd0'],'scope':'Original wholecalc produces real136B rawmasks; actualplatform privateheight,shift andthreshold helpers run without privatehelper bridges. Compare all processedmask flags/coords/range/length and step active28B start/end/sourceLoopRange; last4B originalpadding excluded. B observed finite arrays; resolved gate priorrelationship validated externally.','excluded':['OriginalEventReader/ECS relationshiplookup/HashMap storage','Originalforkschedule','A/C behavior','Opaque lazycache schema and32Bstep padding','Thirdconstructionheight writers'], 'binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in files},'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'evidence/building-segmentation/native_platform_segmentation.py',ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'scripts/multi-version/verify_wall_platform_segmentation.py']}}
 name='B-platform-masks-verification.json'if not fail else 'B-platform-masks-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'first':fail[:2]}));assert not fail
if __name__=='__main__':main()
