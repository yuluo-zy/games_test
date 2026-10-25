"""Original B private 64-byte OETF oracle; public powf is the host UCRT import."""
import ctypes,hashlib,json,pathlib,random,struct,subprocess
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RSP,UC_X86_REG_RIP,UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_MXCSR
OUT=pathlib.Path(__file__).resolve().parent;ROOT=OUT.parents[2]
raw=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];op=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0];size=struct.unpack_from('<I',raw,pe+24+56)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
    _,rva,n,off=struct.unpack_from('<IIII',raw,pe+24+op+40*i+8)
    cpu.mem_write(base+rva,raw[off:off+n])
stack=0x30000000;stop=0x40000000;cpu.mem_map(stack,0x10000);cpu.mem_map(stop,0x1000)
powf=ctypes.CDLL('ucrtbase.dll').powf;powf.argtypes=[ctypes.c_float,ctypes.c_float];powf.restype=ctypes.c_float
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def number(v):return struct.unpack('<f',struct.pack('<I',v))[0]
imports=[0]
def hook(c,a,n,u):
    if a==0x142923f70:
        imports[0]+=1
        x=number(c.reg_read(UC_X86_REG_XMM0)&0xffffffff);y=number(c.reg_read(UC_X86_REG_XMM1)&0xffffffff)
        c.reg_write(UC_X86_REG_XMM0,bits(powf(x,y)))
        sp=c.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',c.mem_read(sp,8))[0]
        c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,ra)
cpu.hook_add(UC_HOOK_CODE,hook)
edge=[0,0x80000000,0x7f800000,0xff800000,0x7fc00000,0xffc00000,1,0x007fffff,0x7f7fffff]
threshold=bits(0.0031308);edge.extend([threshold-1,threshold,threshold+1])
inputs=edge+[bits(n/255) for n in range(256)]
rng=random.Random(20261013);inputs.extend(rng.getrandbits(32) for _ in range(4096))
expected=[]
for value in inputs:
    sp=stack+0xff00;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp)
    cpu.reg_write(UC_X86_REG_XMM0,value);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.emu_start(0x1417bcd20,stop,count=100)
    assert cpu.reg_read(UC_X86_REG_RIP)==stop
    expected.append(cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff)
runner=ROOT/'reconstruction/shared-foundation/target/release/examples/gamma_runner.exe'
run=subprocess.run([str(runner)],input=''.join(f'{i:08x}\n' for i in inputs),text=True,capture_output=True,check=True)
actual=[int(line,16) for line in run.stdout.splitlines()]
diff=[{'input':f'{inputs[i]:08x}','original':f'{a:08x}','candidate':f'{b:08x}'} for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
report={'schema':1,'ok':len(actual)==len(expected) and not diff,'original_binary_sha256':sha,'entry':'0x1417bcd20',
    'private_body_size':64,'cases':len(inputs),'mismatches':diff,'powf_calls':imports[0],
    'oracle':'Original RNE private machine instructions; imported powf delegates to ucrtbase.dll',
    'limits':['OETF scalar only, not complete image pipeline','MXCSR 0x1f80','No full game executed','Public UCRT math implementation reused']}
(OUT/'gamma-verification.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'ok':report['ok'],'cases':len(inputs),'mismatches':len(diff),'first':diff[:2]}))
if not report['ok']:raise SystemExit(1)
