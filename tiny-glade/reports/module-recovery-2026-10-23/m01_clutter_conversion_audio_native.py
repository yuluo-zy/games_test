"""执行原装饰物删除/出现/拆离派发块；仅桥接已独立验证的音频SDK入口。"""
from pathlib import Path
import struct,json,hashlib
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path("/Users/yuluo/Desktop/game")
raw=(root/"B版本内容/tiny-glade.rne").read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=="f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
blob=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64)
u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S=0x20000000,0x30000000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,v):u.mem_write(a,struct.pack("<Q",v))
def uq(a):return struct.unpack("<Q",u.mem_read(a,8))[0]
def hook(uc,pc,size,opaque):
 if pc==0x140aa3360:
  ptr=u.reg_read(UC_X86_REG_RDX);length=u.reg_read(UC_X86_REG_R8)
  calls.append({"name":bytes(u.mem_read(ptr,length)).decode(),"position":list(struct.unpack("<3I",u.mem_read(u.reg_read(UC_X86_REG_R9),12)))})
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
  u.reg_write(UC_X86_REG_RAX,0)
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for kind in [0x20,0x21,0x25]:
 for selector in (range(56) if kind!=0x25 else [0,55]):
  for position in [[0,0x80000000,0x3f800000],[0x7fc12345,0x7f800000,0xff800000]]:
   calls=[];u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x100000))
   bp=S+0x80000;state=D+0x1000;event=D+0x2000;out=D+0x3000
   q(bp+0x20,state);q(bp+0x168,D+0x5000);q(bp-0x18,D+0x6000)
   u.mem_write(bp+0xb4,struct.pack("<I",1234))
   q(state+0x20,128);q(state+0x28,out);q(state+0x30,0);q(state+0x40,77)
   u.mem_write(event+8,bytes([kind,selector]));u.mem_write(event+0x18,struct.pack("<3I",*position))
   for reg,value in [(UC_X86_REG_RBP,bp),(UC_X86_REG_RSP,bp-0x300),(UC_X86_REG_RDI,event),(UC_X86_REG_RAX,kind)]:u.reg_write(reg,value)
   u.emu_start(0x14145e3f8 if kind==0x25 else 0x14145e220,0x14145e1a0,count=10000)
   assert u.reg_read(UC_X86_REG_RIP)==0x14145e1a0
   count=uq(state+0x30);assert count in [0,1]
   forward=None if not count else {"event_id":uq(out),"position":list(struct.unpack("<3I",u.mem_read(out+8,12))),"kind":u.mem_read(out+20,1)[0],"context":u.mem_read(out+21,1)[0]}
   rows.append({"kind":kind,"selector":selector,"position":position,"one_shots":calls,"forward":forward})
report={"binary_sha256":sha,"dispatch_entry":"0x14145e220","detach_entry":"0x14145e3f8","stop":"0x14145e1a0","bridges":["instantiate_audio_w_position success boundary (separately recovered/tested)"],"rows":rows}
out=root/"tiny-glade/reports/module-recovery-2026-10-23"
with (out/"m01-clutter-conversion-audio-native.json").open("x") as f:json.dump(report,f,indent=2);f.write("\n")
lines=["const ORIGINAL_CONVERSION_AUDIO: &[(u8,u8,[u32;3],Option<(u8,u8)>,Option<&str>)] = &["]
for row in rows:
 fw=row["forward"];forward="None" if fw is None else "Some(("+str(fw["kind"])+","+str(fw["context"])+"))"
 name="None" if not row["one_shots"] else "Some("+json.dumps(row["one_shots"][0]["name"])+")"
 assert all(x["position"]==row["position"] for x in row["one_shots"])
 assert fw is None or fw["position"]==row["position"]
 lines.append("("+str(row["kind"])+","+str(row["selector"])+",["+",".join(hex(x) for x in row["position"])+"],"+forward+","+name+"),")
lines.append("];")
with (out/"m01-clutter-conversion-audio-native.rs").open("x") as f:f.write("\n".join(lines)+"\n")
print(json.dumps({"rows":len(rows),"forwarded":sum(r["forward"] is not None for r in rows),"one_shots":sum(len(r["one_shots"]) for r in rows)}))

