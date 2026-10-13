"""原RNE指令oracle：私有binding名字规则与defines字符串对键；不恢复公共hash实现。"""
import importlib.util, json, struct
from pathlib import Path
from unicorn.x86_const import UC_X86_REG_RSP, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_RAX
root = Path('D:/game/reverse-engineering/tiny-glade')
spec = importlib.util.spec_from_file_location('pe',root/'reconstruction/rhapsody-rebuild/scripts/native_support/pe_fixture.py')
native=importlib.util.module_from_spec(spec); spec.loader.exec_module(native)
def run(entry,rcx,rdx=0):
    native.STATE.clear(); native.STATE.update(visited=set(),cut=native.STOP,cut_reached=False,panic='ok')
    sp=native.STACK+0x10008; native.write(sp,'<Q',native.STOP)
    for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,rcx),(UC_X86_REG_RDX,rdx),(UC_X86_REG_R8,0),(UC_X86_REG_R9,0)]: native.CPU.reg_write(reg,val)
    native.CPU.emu_start(entry,native.STOP,count=100000)
    assert native.CPU.reg_read(UC_X86_REG_RIP)==native.STOP
    return native.CPU.reg_read(UC_X86_REG_RAX)
from unicorn.x86_const import UC_X86_REG_RIP
names=['','output','output_dims','_dims','x_dims_dims','_DIMs','纹理_dims','a\0_dims']
name_results=[]
for name in names:
    b=name.encode(); native.CPU.mem_write(native.MEMORY+0x1000,b or b'\0')
    value=run(0x140c87660,native.MEMORY+0x1000,len(b))
    name_results.append(dict(name=name,hash=value))
pairs=[]
for value in ['0','1','']:
    name='SHADER_VARIANT_DEPTH_ONLY'; nb=name.encode(); vb=value.encode()
    native.CPU.mem_write(native.MEMORY+0x1000,nb); native.CPU.mem_write(native.MEMORY+0x2000,vb or b'\0')
    native.write(native.MEMORY,'<QQQQQQ',len(nb),native.MEMORY+0x1000,len(nb),len(vb),native.MEMORY+0x2000,len(vb))
    hashed=run(0x1420cfbf0,native.MEMORY)
    pairs.append(dict(name=name,value=value,hash=hashed,hex=format(hashed,'x')))
result=dict(original_sha=native.EXPECTED_SHA,names=name_results,pairs=pairs,with_suffix_dims_wrap=run(0x140c87650,2**64-1),private_replacements=[],public_hash_implementation_rebuilt=False)
Path(__file__).with_name('key-oracle.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8')
print(json.dumps(result,ensure_ascii=False))
