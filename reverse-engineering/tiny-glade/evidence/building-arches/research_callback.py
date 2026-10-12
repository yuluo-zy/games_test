import sys,random,struct,subprocess,json
from native_arches import *
rng=__import__('random').Random(20261012);cases=[]
for i in range(200):
 a=[rng.uniform(-10,10),rng.uniform(-2,10),rng.uniform(-10,10)];b=[a[0]+rng.uniform(-1.,1.),a[1]+rng.uniform(-.5,.5),a[2]+rng.uniform(-1.,1.)];us=[rng.random(),rng.random()];cases.append((a,b,us))
def hx(v):return struct.pack('<'+'f'*len(v),*v).hex()
lines=['callback 100000 '+hx([10.])+' 5 7 '+hx(a+b)+' '+hx(us)for a,b,us in cases]
got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-arches-probe')],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(got)==len(cases)
fail=[]
for i,((a,b,u),line)in enumerate(zip(cases,got)):
 native=observed_arch_callback(a,b,u,100000,10.,5,7)[0];rust=bytes.fromhex(line)
 if native!=rust:fail.append(dict(case=i,input=[a,b,u],offsets=[j for j,(x,y)in enumerate(zip(native,rust))if x!=y],native=list(struct.unpack('<19f2I2fI',native)),rust=list(struct.unpack('<19f2I2fI',rust))))
r=dict(cases=len(cases),failures=len(fail),first=fail[:4]);(ROOT/'evidence/building-arches/B-callback-research.json').write_text(json.dumps(r,indent=2));print(json.dumps(r))
