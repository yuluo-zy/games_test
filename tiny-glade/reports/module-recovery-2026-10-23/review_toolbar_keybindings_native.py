"""完整原assign_key有限差分：无依赖桥、无跳过原业务范围、无OS输入。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RSP,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_RIP
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest();assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack,stop=0x20000000,0x30000000,0x20005000;u.mem_map(data,0x6000);u.mem_map(stack,0x10000)
rows=[]
for action in range(37):
 for key in range(1,196):
  for pattern in range(2):
   before=[key if not pattern or i%3==0 else 195 if i%3==1 else (i*7)%194+1 for i in range(37)]
   state=bytearray(456)
   for i,v in enumerate(before):struct.pack_into('<III',state,i*12,v,0,0)
   u.mem_write(data,bytes(state));u.mem_write(data+0x1000,struct.pack('<III',key,0,0));rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
   for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,action),(UC_X86_REG_R8,data+0x1000)]:u.reg_write(reg,val)
   u.emu_start(0x14089b2a0,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop
   after=[struct.unpack('<I',u.mem_read(data+i*12,4))[0] for i in range(37)]
   rows.append((action,key,pattern,after))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-keybindings-assign.ron';fixture.write_text('[\n'+''.join(f'({a},{k},{p},'+str(v).replace(' ','')+'),\n' for a,k,p,v in rows)+']\n')
Path(__file__).with_name('toolbar-keybindings-assign-native.json').write_text(json.dumps({'binary_sha256':digest,'function':'0x14089b2a0','whole_function':True,'bridges':[],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'37动作、194命名键+None、全冲突/混合两种档案。精确比较37字段discriminant。不含Unidentified带平台payload键，不宣称全域。'},ensure_ascii=False,indent=2)+'\n')
print(len(rows),'cases')

labels=[]
from unicorn.x86_const import UC_X86_REG_RAX
for tag in range(1,195):
 u.mem_write(data+0x1000,struct.pack('<III',tag,0,0));rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data+0x1000)]:u.reg_write(reg,val)
 u.emu_start(0x14089d490,stop,count=1000);assert u.reg_read(UC_X86_REG_RIP)==stop
 pointer=u.reg_read(UC_X86_REG_RAX);length=u.reg_read(UC_X86_REG_RDX)
 label=bytes(u.mem_read(pointer,length)).decode() if pointer else None
 labels.append((tag,label))
label_fixture=root/'tiny-glade/tests/fixtures/native/toolbar-keybindings-labels.ron'
label_fixture.write_text('[\n'+''.join(f'({tag},'+('Some('+json.dumps(label,ensure_ascii=False)+')' if label is not None else 'None')+'),\n' for tag,label in labels)+']\n')
Path(__file__).with_name('toolbar-keybindings-labels-native.json').write_text(json.dumps({'binary_sha256':digest,'function':'0x14089d490','whole_function':True,'bridges':[],'cases':len(labels),'fixture_sha256':hashlib.sha256(label_fixture.read_bytes()).hexdigest(),'scope':'194个命名KeyCode的显示文字／None逐项比较；不传None或Unidentified载荷。'},ensure_ascii=False,indent=2)+'\n')
print(len(labels),'label cases')
# 原KeyCode::Unidentified = tag0 + NativeKeyCode枚举。只比较已初始化的语义投影。
def native_bytes(kind,payload):
 return struct.pack('<IHHI',0,kind,payload&65535 if kind in (2,3) else 0,payload if kind in (1,4) else 0)
def semantic(kind,payload):
 return ((kind+1)<<32)|(0 if kind==0 else payload&65535 if kind in (2,3) else payload)
native_rows=[]
for action in range(37):
 for kind in range(5):
  for payload in [0,1,42,65535]+([4294967295] if kind in (1,4) else []):
   for pattern in range(2):
    state=bytearray(456)
    for i in range(37):
     k,p=(kind,payload) if pattern==0 or i%3==0 else ((kind+1)%5,payload) if i%3==1 else (kind,(payload+1)&4294967295)
     state[i*12:i*12+12]=native_bytes(k,p)
    u.mem_write(data,bytes(state));u.mem_write(data+0x1000,native_bytes(kind,payload));rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
    for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,action),(UC_X86_REG_R8,data+0x1000)]:u.reg_write(reg,val)
    u.emu_start(0x14089b2a0,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop
    after=[]
    for i in range(37):
     tag,k,p16,p32=struct.unpack('<IHHI',u.mem_read(data+i*12,12));assert tag in [0,195]
     after.append(0 if tag==195 else semantic(k,p16 if k in (2,3) else p32))
    native_rows.append((action,kind,payload,pattern,after))
native_fixture=root/'tiny-glade/tests/fixtures/native/toolbar-keybindings-native-keys.ron'
native_fixture.write_text('[\n'+''.join(f'({a},{k},{v},{p},'+str(values).replace(' ','')+'),\n' for a,k,v,p,values in native_rows)+']\n')
Path(__file__).with_name('toolbar-keybindings-native-keys.json').write_text(json.dumps({'binary_sha256':digest,'function':'0x14089b2a0','whole_function':True,'bridges':[],'cases':len(native_rows),'fixture_sha256':hashlib.sha256(native_fixture.read_bytes()).hexdigest(),'scope':'37动作、NativeKeyCode五种variant、0/1/42/65535及32位variant的u32MAX、相同/跨平台/不同payload。逐字段Option/平台variant/payload比较；不比较padding。'},ensure_ascii=False,indent=2)+'\n')
print(len(native_rows),'native-payload cases')
