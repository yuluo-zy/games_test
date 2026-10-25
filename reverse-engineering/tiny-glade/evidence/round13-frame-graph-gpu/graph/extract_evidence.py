from pathlib import Path
import hashlib, json, struct, subprocess, bisect
R=Path('D:/game');O=R/'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/graph';E=R/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe';raw=E.read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',raw,60)[0];n=struct.unpack_from('<H',raw,nt+6)[0];opt=struct.unpack_from('<H',raw,nt+20)[0];secs=[]
for i in range(n):
 a=nt+24+opt+i*40;vs,va,rs,rp=struct.unpack_from('<IIII',raw,a+8);secs.append((raw[a:a+8].split(b'\0')[0].decode(),va,vs,rs,rp))
def at(addr,size):
 rva=addr-0x140000000
 for name,va,vs,rs,rp in secs:
  if va<=rva and rva+size<=va+rs:return raw[rp+rva-va:rp+rva-va+size]
  if va<=rva and rva+size<=va+vs:return bytes(size)
 raise ValueError(hex(addr))
ps=[]
for l in (R/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
 p=l.split('\t')
 try:ps.append((int(p[0],16),int(p[1]),p[-1]))
 except ValueError:pass
ps.sort();keys=[p[0] for p in ps]
proof=[]
for value in [0x48b321ef779dc301,0x35385c45b6f578e9,0xd38dae617fb79c7a,0xc2b8e941c91a6428,0xa4c02661e67b048b]:
 needle=struct.pack('<Q',value);start=0
 while True:
  start=raw.find(needle,start)
  if start<0:break
  for _,va,vs,rs,rp in secs:
   if rp<=start<rp+rs:
    addr=start-rp+va+0x140000000;p=ps[bisect.bisect_right(keys,addr)-1]
    if p[0]+p[1]>addr:proof.append(dict(typeid_first=hex(value),instruction=hex(addr),function=hex(p[0]),size=p[1],symbol=p[2]))
  start+=8
(O/'resource-typeid-registration-proof.json').write_text(json.dumps(proof,indent=2))
ranges={'debug-default':(0x14093fce0,363),'painterly-entry':(0x1408a6410,0x325),'final-post-entry':(0x14090d000,0x430),'render-debug-control':(0x1400b5c30,1808)}
identity={}
for name,(a,size) in ranges.items():
 identity[name]=dict(start=hex(a),size=size,bytes_sha256=hashlib.sha256(at(a,size)).hexdigest(),scope='bounded entry fragment' if name.endswith('entry') else 'PDB function interval including known unwind funclets')
 p=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn','--start-address='+hex(a),'--stop-address='+hex(a+size),str(E)],capture_output=True,text=True,check=True)
 (O/(name+'.asm')).write_text(p.stdout)
identity['render_debug_static']=dict(address='0x143634b90',initial_byte=at(0x143634b90,1).hex(),symbol='tiny_glade::game_loop::RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW')
(O/'original-evidence.json').write_text(json.dumps(dict(image=str(E),sha256=hashlib.sha256(raw).hexdigest(),ranges=identity),indent=2))
print(json.dumps(identity,indent=2))
