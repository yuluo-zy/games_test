"""以原始 RNE 指令验证 Roof 状态函数；不运行游戏，不推测字段名。"""
from pathlib import Path
import struct, json, hashlib, random, subprocess
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *

root = Path('D:/game/reverse-engineering/tiny-glade')
raw = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
pe = struct.unpack_from('<I', raw, 0x3c)[0]
section_count = struct.unpack_from('<H', raw, pe+6)[0]
optional_size = struct.unpack_from('<H', raw, pe+20)[0]
image_base = struct.unpack_from('<Q', raw, pe+48)[0]
sections = [struct.unpack_from('<IIII', raw, pe+24+optional_size+40*i+8) for i in range(section_count)]
def offset(va):
    rva = va-image_base
    for virtual_size, virtual_address, raw_size, raw_offset in sections:
        if virtual_address <= rva < virtual_address+max(virtual_size,raw_size):
            return raw_offset+rva-virtual_address
    raise ValueError(hex(va))
def bytes_at(va, n):
    pos = offset(va)
    return raw[pos:pos+n]

constants = {va: bytes_at(va,4) for va in [0x142925984,0x142925904]}
report = {'source_sha256':hashlib.sha256(raw).hexdigest(), 'full_game_executed':False,
          'oracle':'Unicorn x64 模拟原作 RNE 中指令，比较编译后的 Rust 候选', 'mxcsr':'0x1f80',
          'constants':{hex(va):{'little_endian_bytes':data.hex(),'u32_bits':hex(struct.unpack('<I',data)[0]),'f32':struct.unpack('<f',data)[0]} for va,data in constants.items()},
          'functions':[], 'limitations':['字段原名和完整 Rust 类型未知','仅验证局部状态函数的返回布尔值和写入字节，不验证浮点异常状态位','构造器输入使用互不别名的有效内存','仅测试 MXCSR 0x1f80','候选实现采用显式字节接口，不宣称还原完整 Rust ABI']}
cpu = Uc(UC_ARCH_X86,UC_MODE_64)
code_page = 0x1408e3000
cpu.mem_map(code_page,0x1000)
cpu.mem_write(code_page,bytes_at(code_page,0x1000))
constant_page = 0x142925000
cpu.mem_map(constant_page,0x1000)
cpu.mem_write(constant_page,bytes_at(constant_page,0x1000))
data, stack = 0x20000000,0x30000000
cpu.mem_map(data,0x2000)
cpu.mem_map(stack,0x1000)
cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
cpu.reg_write(UC_X86_REG_RSP,stack+0x100)
rng = random.Random(20261010)
edges = [0,0x80000000,1,0x80000001,0x3f800000,0xbf800000,0x7f800000,0xff800000,
         0x7fc00000,0xffc00000,0x7f800001,0x7fffffff]
threshold = struct.unpack('<I',constants[0x142925984])[0]
equal = struct.unpack('<I',constants[0x142925904])[0]
edges += [threshold-1,threshold,threshold+1,equal-1 if equal else 0,equal,equal+1]
cases = [(a,b) for a in edges for b in edges]+[(rng.getrandbits(32),rng.getrandbits(32)) for _ in range(1024)]
lines, expected = [], []
for a,b in cases:
    cpu.mem_write(data+0x30,struct.pack('<II',a,b))
    cpu.reg_write(UC_X86_REG_RCX,data)
    cpu.reg_write(UC_X86_REG_RAX,0)
    cpu.emu_start(0x1408e3250,0x1408e325f,count=3)
    ty = cpu.reg_read(UC_X86_REG_RAX)&255
    cpu.reg_write(UC_X86_REG_RAX,0)
    cpu.emu_start(0x1408e33b0,0x1408e33db,count=9)
    gable = cpu.reg_read(UC_X86_REG_RAX)&255
    lines.append(f'pred {a:08x} {b:08x} {threshold:08x} {equal:08x}')
    expected.append(f'{ty} {gable}')

new_cases = 256
return_pointer_ok = True
for _ in range(new_cases):
    pieces = [rng.randbytes(n) for n in [0x58,8,28,28,4,4,1,12]]
    output,arg2,arg3,arg4,arg5,arg6,arg7,arg8 = pieces
    cpu.mem_write(data,output)
    cpu.mem_write(data+0x100,arg3)
    cpu.mem_write(data+0x200,arg4)
    cpu.mem_write(data+0x300,arg8)
    cpu.reg_write(UC_X86_REG_RCX,data)
    cpu.reg_write(UC_X86_REG_RDX,int.from_bytes(arg2,'little'))
    cpu.reg_write(UC_X86_REG_R8,data+0x100)
    cpu.reg_write(UC_X86_REG_R9,data+0x200)
    cpu.mem_write(stack+0x128,arg5)
    cpu.mem_write(stack+0x130,arg6)
    cpu.mem_write(stack+0x138,arg7)
    cpu.mem_write(stack+0x140,struct.pack('<Q',data+0x300))
    cpu.emu_start(0x1408e31c0,0x1408e3218,count=25)
    return_pointer_ok &= cpu.reg_read(UC_X86_REG_RAX)==data
    lines.append('new '+' '.join(p.hex() for p in pieces))
    expected.append(bytes(cpu.mem_read(data,0x58)).hex())

binary = root/'workspace/roof-state-reconstructed.exe'
subprocess.run(['C:/Users/liyu/.cargo/bin/rustc.exe','-O',str(root/'reconstruction/roof/roof_state.rs'),'-o',str(binary)],check=True)
result = subprocess.run([str(binary)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
actual = result.stdout.splitlines()
failures = [{'case':i,'input':lines[i],'original':a,'candidate':b} for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
for name,va,size,count in [('Roof::new',0x1408e31c0,89,new_cases),('Roof::ty',0x1408e3250,16,len(cases)),('Roof::is_gable',0x1408e33b0,44,len(cases))]:
    report['functions'].append({'name':name,'va':hex(va),'size':size,'machine_code_hex':bytes_at(va,size).hex(),'cases':count})
report.update(ok=len(actual)==len(expected) and not failures and return_pointer_ok, failures=failures,
              constructor_return_pointer_ok=return_pointer_ok,predicate_pair_cases=len(cases),constructor_cases=new_cases,
              nan_behavior={'ty':'unordered sets CF, so NaN returns true','is_gable':'NaN in +0x30 passes CMPNLE; NaN in +0x34 fails equality'})
(root/'evidence/native/roof-state-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'ok':report['ok'],'predicate_pairs':len(cases),'constructor_cases':new_cases,'failures':len(failures),'constants':report['constants']}))
if not report['ok']:
    raise SystemExit(1)
