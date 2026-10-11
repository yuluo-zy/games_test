"""Complete resolved B tag>=2 cache branch through original return.
Only standard allocation/free/reallocation are bridged; no game result is injected.
"""
import hashlib,json,random,struct,subprocess,ctypes,ctypes.util,math
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from verification_provenance import ROOT

def bits(value):return struct.unpack('<I',struct.pack('<f',value))[0]

def main():
 out=ROOT/'evidence/building-freeform-cache';out.mkdir(parents=True,exist_ok=True)
 source_paths=['reconstruction/building/wall_freeform_cache.rs','reconstruction/src/bin/wall-freeform-cache-probe.rs','reconstruction/building/wall_state.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_publication.rs','reconstruction/roof/surface.rs','reconstruction/wall/height_rules.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock']
 source_snapshot={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in source_paths};oracle_snapshot=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
 baseline=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B']
 raw=Path(baseline['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==baseline['binary']['sha256']
 pe_header=struct.unpack_from('<I',raw,0x3c)[0];import_rva=struct.unpack_from('<I',raw,pe_header+24+112+8)[0]
 def file_offset(rva):
  s=next(s for s in baseline['pe']['sections']if s['rva']<=rva<s['rva']+s['raw_size']);return s['raw_pointer']+rva-s['rva']
 imports={};descriptor=file_offset(import_rva)
 while any(raw[descriptor:descriptor+20]):
  original,_,_,_,first=struct.unpack_from('<5I',raw,descriptor);lookup=file_offset(original or first);i=0
  while (name_rva:=struct.unpack_from('<Q',raw,lookup+i*8)[0]):
   if not name_rva>>63:
    start=file_offset(name_rva)+2;name=raw[start:raw.index(0,start)].decode();imports[baseline['pe']['image_base']+first+i*8]=name
   i+=1
  descriptor+=20
 pe=baseline['pe'];base=pe['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64)
 end=max(s['rva']+max(s['raw_size'],s['virtual_size'])for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
 for s in pe['sections']:
  if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
 data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x50000000
 cpu.mem_map(data,0x100000);cpu.mem_map(stack,0x10000);cpu.mem_map(heap,0x100000);cpu.mem_map(stop,0x1000);allocation=[heap];capture={}
 def u64(p):return struct.unpack('<Q',cpu.mem_read(p,8))[0]
 def ret(value=None):
  if value is not None:cpu.reg_write(UC_X86_REG_RAX,value)
  sp=cpu.reg_read(UC_X86_REG_RSP);cpu.reg_write(UC_X86_REG_RIP,u64(sp));cpu.reg_write(UC_X86_REG_RSP,sp+8)
 def alloc(n,zero=False):
  p=allocation[0];allocation[0]+=max(16,(n+15)&~15);assert allocation[0]<heap+0x100000
  if zero and n:cpu.mem_write(p,bytes(n))
  return p
 lib=ctypes.CDLL(ctypes.util.find_library('m')or ctypes.util.find_library('c'))
 crt={0x142923e60:('ceilf',1),0x142923e80:('cosf',1),0x142923f10:('fmodf',2),0x142923fd0:('sinf',1),0x142923fa0:('roundf',1),0x142923ec0:('expf',1)}
 verified_iat=[]
 for thunk,name in [(pc,value[0])for pc,value in crt.items()]+[(0x142923c00,'memcpy'),(0x142923c10,'memmove'),(0x142923c20,'memset')]:
  offset=file_offset(thunk-baseline['pe']['image_base']);assert raw[offset:offset+2]==b'\xff\x25';iat=thunk+6+struct.unpack_from('<i',raw,offset+2)[0];assert imports[iat]==name;verified_iat.append({'thunk':hex(thunk),'iat':hex(iat),'name':name})
 (out/'B-CRT-IAT.json').write_text(json.dumps({'binary_sha256':baseline['binary']['sha256'],'basis':'Original PE import names and JMP RIP-relative thunk targets, checked on each verification run','verified':verified_iat},indent=2)+'\n')
 for name,count in crt.values():fn=getattr(lib,name);fn.argtypes=[ctypes.c_float]*count;fn.restype=ctypes.c_float
 def curve_at(p,dimension):
  n=u64(p+16);ptr=u64(p+8);us=u64(p+32);length=struct.unpack('<I',cpu.mem_read(p+48,4))[0]
  points=[list(struct.unpack('<'+'I'*dimension,cpu.mem_read(ptr+j*dimension*4,dimension*4)))for j in range(n)]
  return points,list(struct.unpack('<'+'I'*n,cpu.mem_read(us,n*4))),length
 def hook(uc,pc,size,user):
  if pc in [0x140613c10,0x140613c40]:ret(alloc(cpu.reg_read(UC_X86_REG_RCX),pc==0x140613c40))
  elif pc==0x140613c20:ret()
  elif pc==0x140613c30:
   old,n,new=(cpu.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R9]);p=alloc(new)
   if min(n,new):cpu.mem_write(p,bytes(cpu.mem_read(old,min(n,new))))
   ret(p)
  elif pc in [0x142923c00,0x142923c10]:
   dst,src,n=(cpu.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8])
   if n:cpu.mem_write(dst,bytes(cpu.mem_read(src,n)))
   ret(dst)
  elif pc==0x142923c20:
   dst,value,n=(cpu.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8])
   if n:cpu.mem_write(dst,bytes([value&255])*n)
   ret(dst)
  elif pc in crt:
   name,count=crt[pc];args=[struct.unpack('<f',struct.pack('<I',cpu.reg_read(r)&0xffffffff))[0]for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1][:count]]
   cpu.reg_write(UC_X86_REG_XMM0,bits(getattr(lib,name)(*args)));ret()
  elif pc==0x140ac816c:capture['preview']=curve_at(cpu.reg_read(UC_X86_REG_RBP)-0x20,2)
  elif pc==0x140ac8189:capture['sampled']=curve_at(cpu.reg_read(UC_X86_REG_RBP)-0x58,3)
  elif pc==0x140ac82de:capture['target']=cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff
  elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError(f'Original panic {pc:x}')
 cpu.hook_add(UC_HOOK_CODE,hook)
 rng=random.Random(20261016);edges=[0,0x80000000,0x3f000000,0x3f75c28f,0x3f75c290,0x3f75c291,0xc0200000,0x7f800000,0xff800000,0x7fc01234,0xffc01234,0x7f800001]
 cases=[];lines=[];expected=[]
 for case in range(1536):
  n=[0,1,2,3,7,8,9,15,16,17,31][case%11];segments=[0,n,n+1,2][case%4];tag=[2,3,4,0xffffffff][case%4];clutter=[0,1,2,255][(case//4)%4]
  extra=rng.choice(edges)if case<512 else bits(rng.uniform(-4,10));positions=[(bits(float(i)),rng.choice(edges)if case<512 else bits(rng.uniform(-4,20)),bits(-float(i)))for i in range(n)]
  allocation[0]=heap;cpu.mem_write(data,bytes(0x20000));cpu.mem_write(stack,bytes(0x10000));cpu.mem_write(heap,bytes([0xa5])*0x100000)
  record=bytearray([0xa5]*0x2a0);struct.pack_into('<QQQ',record,0x38,1,data+0x2000,1);cpu.mem_write(data+0x2000,struct.pack('<fff',0.,1000000. if case%2 else -1000000.,0.));struct.pack_into('<QQ',record,0x90,data+0x1000,n);struct.pack_into('<Q',record,0xd0,segments)
  arch_capacity=0 if case%2 else n+3;arch_ptr=data+0x4000 if arch_capacity else 4
  struct.pack_into('<QQQ',record,0xd8,arch_capacity,arch_ptr,min(3,arch_capacity));struct.pack_into('<QQQ',record,0xf0,3,data+0x5000,3);cpu.mem_write(data+0x4000,bytes([0x5a])*256);cpu.mem_write(data+0x5000,struct.pack('<3f',123.,-456.,789.));struct.pack_into('<I',record,0x258,tag);struct.pack_into('<I',record,0x284,extra);record[0x295]=clutter
  cpu.mem_write(data,bytes(record));cpu.mem_write(data+0x1000,b''.join(struct.pack('<III',*p)for p in positions))
  for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
  rbp=stack+0x8000;cpu.reg_write(UC_X86_REG_RBP,rbp);cpu.reg_write(UC_X86_REG_RSP,rbp-0x80);cpu.reg_write(UC_X86_REG_RSI,data+0x2a0);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  cpu.mem_write(rbp+0x1e8,struct.pack('<Q',stop));cpu.mem_write(rbp+0xc0,struct.pack('<Q',0));cpu.mem_write(rbp+0xe8,struct.pack('<Q',0));cpu.mem_write(rbp+0xd0,struct.pack('<Q',data+0x8000))
  for reg,value in [(UC_X86_REG_XMM8,0xc0200000),(UC_X86_REG_XMM9,0xbeeb851f),(UC_X86_REG_XMM10,0x3f75c290),(UC_X86_REG_XMM11,0x3f000000),(UC_X86_REG_XMM12,0x3f0000003f0000003f0000003f000000)]:cpu.reg_write(reg,value)
  cpu.emu_start(0x14078b7e7,stop,count=100000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
  result=bytes(cpu.mem_read(data,0x2a0));arch_count=u64(data+0xe8);third_count=u64(data+0x100)
  assert arch_count==n and third_count==segments
  arch=struct.unpack('<'+'I'*n,cpu.mem_read(u64(data+0xe0),n*4))if n else [];third=struct.unpack('<'+'I'*segments,cpu.mem_read(u64(data+0xf8),segments*4))if segments else []
  assert not any(third)
  changed=set(range(0xd8,0x118))
  assert all(a==b for i,(a,b)in enumerate(zip(record,result))if i not in changed)
  scalar=struct.unpack_from('<IIII',result,0x108);expected.append(' '.join(f'{v:08x}'for v in scalar)+' '+','.join(f'{v:08x}'for v in arch)+' '+','.join(f'{v:08x}'for v in third))
  lines.append(f'cache {tag} {clutter} {extra:08x} {segments} '+' '.join(''.join(f'{v:08x}'for v in p)for p in positions));cases.append({'shape_tag':tag,'clutter_version':clutter,'extra_bits':f'{extra:08x}','prepared_position_bits':[[f'{v:08x}'for v in p]for p in positions],'segment_count':segments})
 probe=ROOT/'reconstruction/target/release/wall-freeform-cache-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);actual=run.stdout.splitlines();assert len(actual)==len(expected)
 failures=[{'case':i,'input':line,'original':want,'rust':got}for i,(line,want,got)in enumerate(zip(lines,expected,actual))if want.rstrip()!=got.rstrip()]
 shape_cases=[];shape_lines=[];shape_expected=[]
 terrain_bytes=b''.join(struct.pack('<H',(i*73+(i//512)*101)&65535)for i in range(512*512))
 def hs(v):return f'{v:08x}'
 for i in range(384):
  mode=6;subtype=1+i%2
  if i<32:mode=[0,6,7,0xffffffff][i%4];subtype=[0,3,255][i%3]
  angle=rng.uniform(-4,4);rect=[bits(math.cos(angle)),bits(math.sin(angle)),bits(rng.uniform(-60,60)),bits(rng.uniform(-60,60)),bits(rng.uniform(.2,6)),bits(rng.uniform(.2,6))]
  circle=[bits(rng.uniform(-60,60)),bits(rng.uniform(-60,60)),bits(rng.uniform(.15,3)),bits(rng.uniform(-25,25))]
  placement=bytearray([0xa5]*68);placement[0]=[0,1,255][i%3];struct.pack_into('<I',placement,4,bits(rng.uniform(-4,8)));struct.pack_into('<I',placement,8,[0,1,2,3,0xffffffff][i%5]);struct.pack_into('<I',placement,0x3c,[0,0x80000000,bits(-2.5),bits(.5),bits(4),bits(19)][i%6])
  parameters=bytearray([0xa5]*16);struct.pack_into('<I',parameters,0,i%3);struct.pack_into('<I',parameters,4,bits(rng.uniform(-10,30)))
  lo,hi=bits(-2.5),bits(20);fix=i%2;dt=bits([0.,.005,.03,.1,.5,-.1][i%6])
  allocation[0]=heap;capture.clear();cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x10000));cpu.mem_write(heap,bytes([0xa5])*0x100000)
  cpu.mem_write(data+0x1100,struct.pack('<IB',mode,subtype));cpu.mem_write(data+0x1000,struct.pack('<Q',data+0x1100))
  cpu.mem_write(data+0x1200,struct.pack('<Q',data+0x1300));cpu.mem_write(data+0x1310,struct.pack('<6I',*rect));cpu.mem_write(data+0x1500,struct.pack('<Q',data+0x1600));cpu.mem_write(data+0x1610,struct.pack('<4I',*circle))
  cpu.mem_write(data+0x1800,struct.pack('<Q',data+0x2000));cpu.mem_write(data+0x2008,struct.pack('<Q',data+0x30000));cpu.mem_write(data+0x2050,struct.pack('<II',lo,hi));cpu.mem_write(data+0x30000,terrain_bytes);cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x1900));cpu.mem_write(data+0x1900,bytes([fix]))
  cpu.mem_write(data+0x9000,bytes(placement));cpu.mem_write(data+0xa000,bytes(parameters));cpu.mem_write(data+0x7000,struct.pack('<QQQI',data+0x9000,0,data+0x6ff0,0)+struct.pack('<I',0x12345678));cpu.mem_write(data+0x8000,struct.pack('<QQQI',data+0xa000,0,data+0x7ff0,0)+struct.pack('<I',0x12345678));cpu.mem_write(data+0x6ff0,struct.pack('<I',0x11223344));cpu.mem_write(data+0x7ff0,struct.pack('<I',0x55667788))
  cpu.mem_write(data+0xa100,struct.pack('<Q',data+0xa200));cpu.mem_write(data+0xa228,struct.pack('<I',dt));sp=stack+0x8008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<QQQ',data+0x7000,data+0x8000,data+0xa100))
  for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
  cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data+0x1000);cpu.reg_write(UC_X86_REG_RDX,data+0x1200);cpu.reg_write(UC_X86_REG_R8,data+0x1500);cpu.reg_write(UC_X86_REG_R9,data+0x1800);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  cpu.emu_start(0x140ac80e0,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
  want={'applied':bool(capture),'placement':bytes(cpu.mem_read(data+0x9000,68)).hex(),'parameters':bytes(cpu.mem_read(data+0xa000,16)).hex(),'placement_tick':struct.unpack('<I',cpu.mem_read(data+0x6ff0,4))[0],'parameters_tick':struct.unpack('<I',cpu.mem_read(data+0x7ff0,4))[0]}
  if capture:
   pp,pu,pl=capture['preview'];pts,us,ll=capture['sampled'];want.update(preview_points=[[hs(v)for v in p]for p in pp],preview_u=list(map(hs,pu)),preview_length=hs(pl),positions=[[hs(v)for v in p]for p in pts],sampled_u=list(map(hs,us)),sampled_length=hs(ll),requested_extra=bytes(cpu.mem_read(data+0x9040,4))[::-1].hex(),target_world_height=hs(capture['target']))
  line=f'shape {mode} {subtype} {lo:08x} {hi:08x} {fix} {dt:08x} {placement.hex()} {parameters.hex()} '+''.join(map(hs,rect))+' '+''.join(map(hs,circle));shape_lines.append(line);shape_expected.append(want);shape_cases.append({'input':line,'original':want})
 shape_actual=subprocess.run([str(probe)],input='\n'.join(shape_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(shape_actual)==len(shape_expected)
 shape_failures=[{'case':i,'input':line,'different_fields':[key for key in set(want)|set(got)if want.get(key)!=got.get(key)],'original':want,'rust':got}for i,(line,want,got)in enumerate(zip(shape_lines,shape_expected,map(json.loads,shape_actual)))if want!=got]
 assert source_snapshot=={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in source_paths};assert oracle_snapshot==hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
 report={'ok':not failures,'baseline':'B','binary_sha256':baseline['binary']['sha256'],'cases':len(cases),'failure_count':len(failures),'failures':failures[:20],'sources_remained_frozen':True,'executed_range':{'entry':'0x14078b7e7','exit':'original ret to sentinel','branch_guard':'shape>=2 at0x14078b85b','actual_return_assertions':len(cases)},'scope':'Complete resolved tag>=2 cache branch: third array zero initialization at actual segment-owner count; uniform/nonuniform prepared height policies; arch count/values; both cacheflags/values. PublicWalls std lookup/event/scheduler and prepared segmentation producer excluded.','bridges':['PinnedB originalPE','Only generic allocator/free/realloc bridges; original Vec reserve executes; no private helpers/results are injected'],'input_roles':'Initial mapped Curve3+38 vector is independently set to one +/-1,000,000 Y point. Output uses only prepared+90/+98. Third length from+d0 is varied independently of prepared point count.','input_cases':cases,'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':source_snapshot,'oracle_sha256':oracle_snapshot,'rustc':subprocess.run(['rustc','-vV'],capture_output=True,text=True,check=True).stdout}
 (out/'B-freeform-cache-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(cases),'failures':len(failures),'first':failures[:2]}));assert not failures
 shape_report={'ok':not shape_failures,'baseline':'B','binary_sha256':baseline['binary']['sha256'],'cases':len(shape_cases),'guard_cases':32,'applied_cases':len(shape_cases)-32,'preview_point_records':sum(len(v.get('preview_points',[]))for v in shape_expected),'terrain_mapped_point_records':sum(len(v.get('positions',[]))for v in shape_expected),'failure_count':len(shape_failures),'failures':shape_failures[:8],'sources_remained_frozen':True,'input_cases':shape_cases,'scope':'Original calc_shape_height full entry->return, including actual preview geometry, own terrain remap, all Curve2/Curve3 points/U/length, ShapePlacement+40, TimberframeConstructionParamsCache16bytes and both change ticks; exact finite observed fields. Not freeform handler, UI tool owner/defaults or World registration.','executed_range':{'entry':'0x140ac80e0','exit':'original ret to sentinel','actual_return_assertions':len(shape_cases)},'bridges':['Only generic allocator/free/realloc/memory and IAT identified ceilf/cosf/fmodf/sinf/roundf/expf. No preview geometry, terrain sampler, curve or lerp output substituted.'],'dependency_call_boundary':'Original cosf then sinf is retained as separate Rust dependency calls. Previous LLVM sincos combination failure is retained separately. Host CRT bridges establish measured inputs, not general Windows/macOS CRT equivalence.','candidate_binary_sha256':report['candidate_binary_sha256'],'candidate_sources':report['candidate_sources'],'oracle_sha256':report['oracle_sha256'],'rustc':report['rustc']}
 (out/'B-shape-height-verification.json').write_text(json.dumps(shape_report,indent=2)+'\n');print(json.dumps({'ok':not shape_failures,'shape_cases':len(shape_cases),'failures':len(shape_failures),'first':[{k:v for k,v in f.items()if k not in ['original','rust']}for f in shape_failures[:3]]}));assert not shape_failures

if __name__=='__main__':main()
