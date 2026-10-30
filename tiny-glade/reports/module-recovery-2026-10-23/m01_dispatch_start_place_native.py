"""执行完整原 dispatcher 的StartPlace优先路径，UI库叶只桥接注册，不复制事件选择算法。"""
from pathlib import Path
import pefile,struct,hashlib,json,itertools
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;blob=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
w=lambda a,v:u.mem_write(a,struct.pack('<I',v))
uq=lambda a:struct.unpack('<Q',u.mem_read(a,8))[0]
uw=lambda a:struct.unpack('<I',u.mem_read(a,4))[0]
r=lambda reg:u.reg_read(reg)
def ret(v=0):
 sp=r(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RAX,v)
def hook(uc,a,size,_):
 rcx,rdx,r8,r9=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 if a==0x14093fbc0:ret(int(esc))
 elif a==0x1409a8e10:
  calls.append({'call':'widget_new','volume_tag':u.mem_read(rdx,1)[0],'distance_bits':uw(rdx+4)});u.mem_write(rcx,bytes(176));ret(rcx)
 elif a==0x1409a8fb0:u.mem_write(rcx,bytes(96));ret(rcx)
 elif a==0x1409a8e80:u.mem_write(rcx,bytes(u.mem_read(rdx,176)));ret(rcx)
 elif a==0x1409a8ee0:
  calls.append({'call':'occlude','enabled':r8&255});u.mem_write(rcx,bytes(u.mem_read(rdx,176)));ret(rcx)
 elif a==0x1409236d0:
  calls.append({'call':'add','name':bytes(u.mem_read(rdx,r8)).decode()});ret(D+0x10000);u.reg_write(UC_X86_REG_RDX,0x123)
u.hook_add(UC_HOOK_CODE,hook)
def payload(i,ty):
 b=bytearray((x*13+i*29)&255 for x in range(96))
 # 原生产者合法非空源；仍使用非单位、非共轴及NaN Quat位型检验reader不重算。
 struct.pack_into('<4I',b,0,0x3e800000+i,0x3f000000,0xbf400000,0x7fc12345 if i==2 else 0x3f600000)
 struct.pack_into('<Q',b,16,0xfedcba9876540000+i);b[24]=11;struct.pack_into('<I',b,24+48,0);b[88]=ty
 return bytes(b)
def run(mode,esc_open,buffer_lengths,cursor_before,ty):
 global calls,esc
 calls=[];esc=esc_open;u.mem_write(D,bytes(0x20000));u.mem_write(S,bytes([0xa5])*0x100000)
 mode_ref,mode_data,dispatch_ref,dispatch_data,tuple_sources,tuple_events=D+0x100,D+0x200,D+0x300,D+0x400,D+0x1000,D+0x2000
 q(mode_ref,mode_data);w(mode_data,mode);w(mode_data+4,0xdeadbe3f)
 q(dispatch_ref,dispatch_data);q(dispatch_ref+0x10,D+0x800);w(dispatch_ref+0x1c,0x1234);w(D+0x800,0xabcd)
 u.mem_write(dispatch_data,bytes([0x91])*112);q(dispatch_data,6);q(dispatch_data+8,0)
 for i in range(6):q(tuple_sources+i*32,D+0x12000+i*0x100)
 for off in [0,40,80]:q(tuple_events+off,D+0x2500+off);q(tuple_events+off+8,D+0x2700+off)
 q(tuple_events+120,D+0x2900);q(tuple_events+136,D+0x2a00)
 q(D+0x2500,cursor_before);events=D+0x2700;a,b=buffer_lengths
 q(events+8,D+0x4000);q(events+0x10,a);q(events+0x18,0);q(events+0x28,D+0x6000);q(events+0x30,b);q(events+0x38,a);q(events+0x40,a+b)
 all_payloads=[]
 for i in range(a+b):
  data=payload(i,ty);all_payloads.append(data);at=(D+0x4000+i*112)if i<a else(D+0x6000+(i-a)*112);u.mem_write(at,data);q(at+96,i)
 sp=S+0xf0008;q(sp,STOP)
 refs=[D+0x900,D+0xa00,D+0xb00,tuple_sources,D+0xc00,D+0xd00,D+0xe00,tuple_events,D+0xf00,D+0x1100]
 for i,p in enumerate(refs):
  q(sp+0x28+i*8,p)
  if p not in [tuple_sources,tuple_events]:q(p,D+0x10000+i*0x100)
 for reg,value in [(UC_X86_REG_RCX,mode_ref),(UC_X86_REG_RDX,D+0x11500),(UC_X86_REG_R8,D+0x11600),(UC_X86_REG_R9,dispatch_ref),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 u.emu_start(0x1407af340,STOP,count=20000);assert r(UC_X86_REG_RIP)==STOP
 out=bytes(u.mem_read(dispatch_data,112));accepted=not esc and mode in [0,3,4,11,20]
 if accepted:
  latest=all_payloads[-1];assert out[:16]==struct.pack('<QQ',1,0)
  assert out[16:32]==latest[:16]and out[32:92]==latest[24:84]and out[96:104]==latest[16:24]and out[104:106]==bytes([1,ty])
  assert uq(D+0x2500)==a+b and uw(D+0x800)==0x1234
  assert calls==[{'call':'widget_new','volume_tag':5,'distance_bits':0x7f7fffff},{'call':'occlude','enabled':0},{'call':'add','name':'ui_decide_decorator_interaction'}]
 else:
  assert uq(D+0x2500)==cursor_before and not calls
  if not esc:assert out[:16]==struct.pack('<QQ',6,0)and uw(D+0x800)==0x1234
  else:assert uw(D+0x800)==0xabcd
 return {'mode':mode,'esc':esc,'buffers':buffer_lengths,'cursor_before':cursor_before,'ty':ty,'payloads_hex':[p.hex()for p in all_payloads],'output_header':list(struct.unpack('<QQ',out[:16])),'output_active_hex':out[16:106].hex()if accepted else None,'cursor_after':uq(D+0x2500),'changed_tick':uw(D+0x800),'calls':calls}
rows=[]
for mode,esc_open,buffers,ty in itertools.product([0,1,2,3,4,5,6,10,11,12,19,20,21,0xffffffff],[False,True],[(1,0),(0,2),(2,2)],[0,9,27,55]):
 rows.append(run(mode,esc_open,buffers,0,ty))
for buffers,cursor in [((3,2),1),((3,2),3),((3,0),2)]:rows.append(run(0,False,buffers,cursor,9))
report={'binary_sha256':sha,'entry':'0x1407af340','payload_bytes':96,'event_stride':112,'native_writes':'header1/0, quat+10, hit60activebytes+20 (last4B inactive padding), widget+60, flag+68=1, ty+69','bridges':['EscMenu read-only view','WidgetParams constructors/empty-sounds/copy; constructor arithmetic separately verified','UiState global registration; widget hash is dependency output, not reimplemented'],'boundary':'this probe executes complete original entry-to-return on nonempty StartPlace path plus Esc/mode rejection; does not claim later hit discovery recovered','rows':rows}
Path(__file__).with_name('m01-dispatch-start-place-native.json').write_text(json.dumps(report,indent=2)+'\n')
lines=['# mode\tesc\tbuffer_a_len\tbuffer_b_len\tcursor_before\tty\tpayloads96_hex_semicolon\toutput_state\toutput_payload\tactive90_hex_or_dash\tchanged_tick']
for row in rows:
 lines.append('\t'.join(map(str,[row['mode'],int(row['esc']),*row['buffers'],row['cursor_before'],row['ty'],';'.join(row['payloads_hex']),*row['output_header'],row['output_active_hex']or '-',row['changed_tick']])))
Path(__file__).with_name('m01-dispatch-start-place-native.tsv').write_text('\n'.join(lines)+'\n')
print(json.dumps({'cases':len(rows),'accepted':sum(r['output_active_hex']is not None for r in rows),'mode_or_esc_rejected':sum(r['output_active_hex']is None for r in rows)}))
