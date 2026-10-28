"""原摄影尾部提示判定：执行原camera/modifier访问器及事件写入，跳过已选中的ghost UI构造。"""
from pathlib import Path
import hashlib,json,struct,itertools
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RBP, UC_X86_REG_RSP, UC_X86_REG_RIP
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack=0x20000000,0x30000000
u.mem_map(data,0x6000);u.mem_map(stack,0x10000)
rbp,rsp=stack+0x8000,stack+0x7008
end,ghost_entry=0x14234c0fe,0x14234bfda
state={}
def hook(uc,address,size,user):
 if address==ghost_entry:
  state['ghost']=True
  uc.reg_write(UC_X86_REG_RIP,end)
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for mode,modifier,prompt in itertools.product(range(6),[0,1,2,255],[False,True]):
 state['ghost']=False
 u.mem_write(data,b'\0'*0x6000)
 u.mem_write(rbp+0x258,struct.pack('<Q',data+0x1000))
 u.mem_write(data+0x14d0,bytes([mode]))
 u.mem_write(rbp+0x1c8,struct.pack('<Q',data+0x1800))
 u.mem_write(data+0x1800,bytes([modifier]))
 u.mem_write(0x143634cd9,bytes([prompt]))
 u.mem_write(rbp+0x368,struct.pack('<Q',data+0x2000))
 u.mem_write(data+0x2000,struct.pack('<QQQI',data+0x3000,0,data+0x4000,0)+struct.pack('<I',42))
 u.mem_write(data+0x3020,struct.pack('<QQQQQ',1,data+0x5000,0,0,7))
 u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,rsp)
 u.emu_start(0x14234bfb2,end,count=1000)
 assert u.reg_read(UC_X86_REG_RIP)==end
 count=struct.unpack('<Q',u.mem_read(data+0x3030,8))[0]
 if count:assert struct.unpack('<Q',u.mem_read(data+0x5000,8))[0]==7
 rows.append(dict(mode=mode,modifier=modifier,requested=prompt,ghost=state['ghost'],events=count))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-tail.ron'
fixture.write_text('[\n'+''.join(f"({r['mode']},{r['modifier']},{str(r['requested']).lower()},{r['events']}),\n" for r in rows)+']\n')
report=dict(binary_sha256=sha,entry='0x14234bfb2',exit='0x14234c0fe',cases=len(rows),rows=rows,fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),bridges=[{'address':hex(ghost_entry),'skip_to':hex(end),'scope':'walk+非零Modifiers已由原条件选中；仅跳过ghost模板构造，不改提示判定或事件输出'}],scope='仅摄影已enabled的原尾部；实际调用两条原只读访问器，真实写Event数/ID。不验证coaster上游/摄影screen门控/MEI绘制，也不发送OS输入。')
Path(__file__).with_name('toolbar-rollercoaster-tail-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['cases','fixture_sha256','scope']},ensure_ascii=False))
