"""审查探针：完整执行原calculate_input_delta；不补写或替换原业务指令。
传入--fixture时生成有限输入的完整40槽差分夹具，保留原两例反证不覆盖。
"""
from pathlib import Path
import hashlib,json,struct,pefile,sys,itertools
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RSP,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_RIP
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes(); digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack,stop=0x20000000,0x30000000,0x20005000
u.mem_map(data,0x6000);u.mem_map(stack,0x10000)
u.mem_write(data,struct.pack('<Q',data+0x1000))
u.mem_write(data+0x80,struct.pack('<Q',data+0x2000))
u.mem_write(data+0x100,struct.pack('<QQQI',data+0x3000,0,data+0x400,0)+struct.pack('<I',42))
rows=[]
fixture_mode='--fixture' in sys.argv
cases=itertools.product(range(40),[False,True],[False,True],range(5),[-0.18,0.,0.19,0.2],[0.,0.02,0.2]) if fixture_mode else [(20,True,True,1,0.19,0.02),(20,True,True,2,-0.18,0.02)]
for slot,previous,current,phase,timer,delta in cases:
 values=bytearray(0xb0);values[0x30+slot]=int(previous);values[0x88+slot]=int(current);u.mem_write(data+0x1000,bytes(values))
 state=bytearray(0x170)
 for i in range(40):struct.pack_into('<If',state,8*i,4,0.)
 struct.pack_into('<If',state,8*slot,phase,timer);u.mem_write(data+0x3000,bytes(state));u.mem_write(data+0x2058,struct.pack('<f',delta))
 rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x80),(UC_X86_REG_R8,data+0x100)]:u.reg_write(reg,val)
 u.emu_start(0x1416feeb0,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop
 p,bits=struct.unpack('<II',u.mem_read(data+0x3000+8*slot,8));result=struct.unpack('<f',struct.pack('<I',bits))[0]
 row={'slot':slot,'previous':previous,'current':current,'state_before':[phase,timer],'delta':delta,'native_phase':p,'native_timer':result,'native_timer_bits':f'{bits:08x}','native_fires':p==0 or (p==2 and result<=0)}
 rows.append(row)
 if not fixture_mode:print(row)
out={'binary_sha256':digest,'function':'0x1416feeb0','size_bytes':5502,'whole_function':True,'bridges':[],'rows':rows,'scope':'只对两个合法有限输入例执行完整原函数；无OS输入；不宣称全域差分。当前Rust另由实际已构建country_core rlib探针执行。'}
if fixture_mode:
 fixture=root/'tiny-glade/tests/fixtures/native/toolbar-input-delta.ron'
 fixture.write_text('[\n'+''.join(f"({r['slot']},{str(r['previous']).lower()},{str(r['current']).lower()},{r['state_before'][0]},{struct.unpack('<I',struct.pack('<f',r['state_before'][1]))[0]},{struct.unpack('<I',struct.pack('<f',r['delta']))[0]},{r['native_phase']},{int(r['native_timer_bits'],16)}),\n" for r in rows)+']\n')
 out.update(cases=len(rows),scope='完整原函数40槽、四种设备边沿、五种输入相位、四种有限timer和三种有限delta。previous=false时timer是原非活动载荷，不比较其位值；其余phase/timer逐bit。无依赖桥；无OS输入。',fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest());out.pop('rows')
 name='toolbar-input-delta-native.json'
else:name='toolbar-deep-review-input-cases.json'
Path(__file__).with_name(name).write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
