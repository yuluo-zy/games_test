#!/usr/bin/env python3
"""B resolved bulldoze history segment and full real animation differential."""
import argparse,hashlib,json,random,struct,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_wall_delete as n
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--cases',type=int,default=128);ap.add_argument('--report',default='evidence/building-wall-history/B-bulldoze-verification.json');a=ap.parse_args();rng=random.Random(1409);cases=[];lines=[];dts=[0.,.025,.025,.05,.1,.025]
 for i in range(a.cases):
  shape=struct.pack('<I6f',0,rng.uniform(-5,5),rng.uniform(-5,5),rng.uniform(.5,5),rng.uniform(-3,3),0,0)if i%2==0 else struct.pack('<I6f',1,1,0,rng.uniform(-5,5),rng.uniform(-5,5),rng.uniform(.5,5),rng.uniform(.5,5))
  tail=bytearray(56);tail[:28]=shape;struct.pack_into('<3fI',tail,28,rng.uniform(.1,5),rng.uniform(-1,2),2.8,rng.getrandbits(32));tail[44:50]=bytes([i%256,(i//2)%2,i%3,(i*7)%256,i%4,(i*13)%256]);wall=9;color=i%17;low=struct.unpack('<f',struct.pack('<f',rng.uniform(0.,.4)))[0];high=struct.unpack('<f',struct.pack('<f',rng.uniform(.6,1.)))[0];anchor=struct.pack('<3f',rng.uniform(-5,5),rng.uniform(-1,2),rng.uniform(-5,5));roof=None
  if i&4:
   roof=bytearray(88);struct.pack_into('<Q',roof,0,wall+(99 if i&32 else 0));roof[0x24:0x54]=bytes(rng.randrange(256)for _ in range(48));roof=bytes(roof)
  present=bool(i&8);phase=[0.,.1,.5,1.][(i//16)%4];cases.append((shape,bytes(tail),wall,color,low,high,anchor,roof,present,phase))
  prefix=f'{shape.hex()} {tail.hex()} {wall} {color} {bits(low):08x} {bits(high):08x}'
  lines.append(f'record {prefix} {roof.hex()if roof else"-"}');lines.append(f'animate {prefix} {anchor.hex()} {int(present)} {bits(phase):08x} '+','.join(f'{bits(dt):08x}'for dt in dts))
 probe=ROOT/'reconstruction/target/release/wall-bulldoze-probe';binary=sha(probe);rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(rows)==len(lines);failures=[];summaries=[]
 for i,(shape,tail,wall,color,low,high,anchor,roof,present,phase)in enumerate(cases):
  ctor=n.new_wall_owner(shape,tail,wall,color);r=n.bulldoze_record_segment(ctor['source_pointer'],wall,low,high,color,roof);got=json.loads(rows[i*2]);want=r['owners']
  assert [o['tag']for o in want]==([8,7]if roof else[7]);assert [[e['kind']for e in g]for g in want[-1]['groups']]==[['state_change'],['delete']]
  if got!=want:failures.append({'case':i,'kind':'record_segment','rust':got,'native':want,'input':lines[i*2]})
  ctor=n.new_wall_owner(shape,tail,wall,color);command=n.data+0x16000;source=ctor['source_pointer'];n.cpu.mem_write(command,bytes(n.cpu.mem_read(source,112))+struct.pack('<Q2f',wall,low,high)+anchor+struct.pack('<fB7x',phase,0));expected=[];raw=[]
  for step,dt in enumerate(dts):
   x=n.animate_bulldoze(command,wall,source,present,dt,initialize=step==0);raw.append(x);audio=None
   assert x['audio_count']<=1
   if x['audio_count']:
    b=bytes.fromhex(x['audio']);audio={'wall':struct.unpack_from('<Q',b,8)[0],'anchor':b[16:28].hex(),'kind':b[28],'flag':b[29]}
   names={n.data+0x60:'updated',n.data+0x90:'removed'};assert all(w==wall for p,w in x['notifications']);expected.append({k:x[k]for k in['source_count','source','phase','buffer_count']}|{'finished':bool(x['finished']),'notifications':[names[p]for p,w in x['notifications']],'audio':audio})
  got=json.loads(rows[i*2+1])
  if got!=expected:failures.append({'case':i,'kind':'animation','rust':got,'native':expected,'input':lines[i*2+1]})
  summaries.append({'case':i,'actual_history_segment_stop':r['actual_segment_stop'],'recording':r,'actual_animation_returns':len(raw),'steps':raw})
 paths=[ROOT/'reconstruction/building/wall_bulldoze.rs',ROOT/'reconstruction/building/wall_history_edit.rs',ROOT/'reconstruction/src/bin/wall-bulldoze-probe.rs',ROOT/'reconstruction/building/wall_authoring.rs',ROOT/'reconstruction/building/wall_collection.rs',ROOT/'reconstruction/building/wall_freeform_cache.rs',ROOT/'reconstruction/building/wall_state.rs',ROOT/'reconstruction/roof/surface.rs',ROOT/'reconstruction/history/building_transactions.rs',ROOT/'reconstruction/Cargo.lock'];oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_wall_delete.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py'];assert sha(probe)==binary
 report={'ok':not failures,'history_segment_cases':len(cases),'full_animate_returns':len(cases)*len(dts),'failure_count':len(failures),'failures':failures[:8],'candidate_binary_sha256':binary,'candidate_sources':{str(p.relative_to(ROOT)):sha(p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'original_binary_sha256':n.nf.b['binary']['sha256'],'inputs':lines,'native':summaries,'scope':'Resolved actual wall_eraser_bulldoze recording business segment1409e1a11->1409df738 asserts real stop, not a whole UI entry RET claim. All private roofquery/snapshot/slice/Compound/History instructions original; std/ECS carriers explicit. Full animate_wall_bulldoze140ac27e0 entry->RET compares source/metadata/phase/buffer removal/audio. Upstream UI/hit selection and resolved range/anchor choice remain outside this proof.'}
 (ROOT/a.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','history_segment_cases','full_animate_returns','failure_count']}));sys.exit(bool(failures))
if __name__=='__main__':main()
