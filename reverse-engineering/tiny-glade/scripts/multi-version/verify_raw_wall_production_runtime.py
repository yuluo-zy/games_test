"""Actual Bevy source -> native path/cache/policy/fullfrom_curve outputs."""
import hashlib,json,struct,subprocess,sys
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
from verify_wall_construction_sources import original_seed,original_perturb,original_options,original_kind,bits
sys.path.insert(0,str(ROOT/'evidence/building-production'))
from native_production import produce
sys.path.insert(0,str(ROOT/'evidence/building-production-authority'))
from native_path_empty import prepare as original_prepare
import native_fixture as nf
from unicorn.x86_const import *

def packed(values):return b''.join(struct.pack('<f',v)for v in values)

def original_cache(s):
 cpu=nf.cpu;data=nf.data;frame=nf.stack+0x100080;points=s['prepared_positions'];n=len(points);count=len(s['prepared_segment_values'])
 cpu.mem_write(data,bytes(0x10000));cpu.mem_write(nf.stack,bytes(0x200000));nf.allocation[0]=nf.heap
 record=bytearray(672);struct.pack_into('<QQ',record,0x90,data+0x1000,n);struct.pack_into('<Q',record,0xd0,count);struct.pack_into('<QQQ',record,0xd8,max(n,1),data+0x5000,0)
 record[0x258:0x274]=bytes.fromhex(s['shape_hex']);extra=max(.3 if s['special']else .5,min(2.5 if s['special']else 14.,s['authored_height']));struct.pack_into('<f',record,0x284,extra);record[0x295]=s['clutter_version']
 cpu.mem_write(data,bytes(record));cpu.mem_write(data+0x1000,packed(v for p in points for v in p))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,frame-0x80);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for r,v in [(UC_X86_REG_XMM8,0xc0200000),(UC_X86_REG_XMM9,0xbeeb851f),(UC_X86_REG_XMM10,0x3f75c290),(UC_X86_REG_XMM11,0x3f000000),(UC_X86_REG_XMM12,0x3f0000003f0000003f0000003f000000)]:cpu.reg_write(r,v)
 if s['shape_tag']>=2:
  cpu.reg_write(UC_X86_REG_RSI,data+672);cpu.mem_write(frame+0x1e8,struct.pack('<Q',nf.stop));cpu.mem_write(frame+0xd0,struct.pack('<Q',data+0x8000));cpu.emu_start(0x14078b7e7,nf.stop,count=100000);assert cpu.reg_read(UC_X86_REG_RIP)==nf.stop
  assert packed(s.get('base_third',s['third']))==bytes(cpu.mem_read(nf.u64(data+0xf8),count*4))
 else:
  cpu.reg_write(UC_X86_REG_RBX,data+672);cpu.emu_start(0x14078b865,0x14078bc96,count=100000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14078bc96;assert all(bits(v)==0 for v in s.get('base_third',s['third']))
 assert packed([s['base_maximum_y']])==bytes(cpu.mem_read(data+0x10c,4))
 assert nf.u64(data+0xe8)==n;assert packed(s.get('base_arch_cache',s['arch']))==bytes(cpu.mem_read(nf.u64(data+0xe0),n*4))

def main():
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/raw-wall-production-cycle.json';binary=ROOT/'reconstruction/target/release/building-run'
 execution=subprocess.run([str(binary),str(fixture)],capture_output=True,text=True,check=True);frames=[json.loads(line)for line in execution.stdout.splitlines()];assert len(frames)==11
 assert all(not f['faults']and not f['pending_wall_bricks']for f in frames)
 by=lambda f:{e['wall_id']:e for e in f['wall_bricks']};first=by(frames[0]);assert set(first)=={'100000','100011'};assert by(frames[1])==first
 for a,z in [(0,2),(2,3),(3,4),(4,5)]:assert by(frames[a])['100000']['records_hex']!=by(frames[z])['100000']['records_hex']
 assert frames[5]['openings']==[];assert by(frames[6])['100011']['records_hex']!=by(frames[5])['100011']['records_hex']
 dedup=by(frames[7])['100000']['source'];assert len(dedup['mapped_positions'])>len(dedup['prepared_positions'])
 for i in [8,9]:assert frames[i]['walls']==[]and frames[i]['wall_bricks']==[]and frames[i]['openings']==[]
 assert by(frames[10])['100000']['records_hex']==by(frames[7])['100000']['records_hex']
 cases=records=0
 for f in frames:
  for wall in f['wall_bricks']:
   s=wall['source'];p=original_prepare(s['mapped_positions'],s['relative_heights'],s['observed_path_scalar'],True)
   assert p['points']==packed(v for pt in s['prepared_positions']for v in pt).hex();assert p['points_u']==packed(s['prepared_u']).hex();assert p['length']==packed([s['prepared_length']]).hex();assert p['segments']==packed(s['prepared_segment_values']).hex()
   original_cache(s);seed,owner=map(int,original_seed(int(wall['wall_id']),len(s['mapped_positions'])).split());assert int(s['seed'])==seed and s['material']==owner
   assert f'{bits(s["perturb"]):08x}'==original_perturb(2,s['clutter_version'],bits(s['holes_length']))
   radius=struct.unpack_from('<I',bytes.fromhex(s['shape_hex']),12)[0];assert original_options(s['shape_tag'],radius).split()==[('-'if s[k]is None else f'{bits(s[k]):08x}')for k in ('width_padding','thickness_scale')];assert original_kind(s['clutter_version'],s['style'],0)=='2'
   inputs={k:s[k]for k in ['points','us','length','arch','second','third','holes','material','closed','base_height','perturb','width_padding','thickness_scale','wall_float','holes_length']};inputs['seed']=seed
   expected=produce(**inputs);assert nf.cpu.reg_read(UC_X86_REG_RIP)==nf.stop;assert [record.hex()for record in expected]==wall['records_hex'];cases+=1;records+=len(expected)
 out=ROOT/'evidence/building-integrate/B-raw-wall-production-runtime-verification.json'
 oracle=[Path(__file__),ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-production-authority/native_path_empty.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'scripts/multi-version/verify_wall_construction_sources.py']
 report={'ok':True,'baseline':'B','frames':frames,'native_wall_frames':cases,'native_ordered_96byte_records':records,'scope':'Actual Bevy raw source + explicit4clear PathRasters -> actual native prepared curve/segment owner -> regular/free cache -> native source-derived seed/BrickSourceId/perturb/options -> full native from_curve ordered records. Shipped opening asset place/move/remove, terrain/height/duplicate-path publication, remove/recreate/nochange. No injected proposal/context/RNG draw.','excluded':['Nonempty raster and Roof top/kind Runtime acceptance: next stage','Original fork schedule/task graph','Native mask/arch consumers, subsequent third-height writers, corners/skirts/structural families/GPU/C migration/whole building acceptance'],'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'candidate_binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in oracle},**candidate_provenance()}
 out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':True,'frames':len(frames),'wall_frames':cases,'records':records}))
if __name__=='__main__':main()
