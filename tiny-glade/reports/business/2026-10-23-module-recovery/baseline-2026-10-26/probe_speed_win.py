"""原过山车1ms速度/末端刹车/t夹具；唯一业务外桥为Darwin libm expf。"""
from pathlib import Path
import ctypes, hashlib, itertools, json, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
ROOT = Path(r'D:/game/tiny-glade/work/m01-scratch/out'); ROOT.mkdir(parents=True, exist_ok=True)
raw = Path(r'D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True); base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image(); u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(image)+4095)&~4095); u.mem_write(base, image)
data, stack = 0x20000000, 0x30000000
u.mem_map(data, 0x10000); u.mem_map(stack, 0x10000); bp = stack+0x8000
expf = ctypes.CDLL('ucrtbase.dll').expf; expf.restype=ctypes.c_float; expf.argtypes=[ctypes.c_float]
def put(addr, fmt, *xs): u.mem_write(addr, struct.pack('<'+fmt, *xs))
def get(addr, fmt): return struct.unpack('<'+fmt, u.mem_read(addr, struct.calcsize('<'+fmt)))
def bits(x): return struct.unpack('<I', struct.pack('<f', x))[0]
def xmm(reg, value): u.reg_write(reg, bits(value))
def readxmm(reg): return struct.unpack('<f', struct.pack('<I', u.reg_read(reg)&0xffffffff))[0]
exp_calls = 0
def bridge(uc, address, size, opaque):
    global exp_calls
    if address == 0x142923ec0:
        exp_calls += 1
        xmm(UC_X86_REG_XMM0, expf(readxmm(UC_X86_REG_XMM0)))
        sp=uc.reg_read(UC_X86_REG_RSP); ret=get(sp,'Q')[0]
        uc.reg_write(UC_X86_REG_RSP, sp+8); uc.reg_write(UC_X86_REG_RIP,ret)
hook = u.hook_add(UC_HOOK_CODE, bridge)
def run(start,end):
    def stop(uc,address,size,opaque):
        if address == end: uc.emu_stop()
    h=u.hook_add(UC_HOOK_CODE,stop)
    try: u.emu_start(start,base+len(image),count=2000)
    finally: u.hook_del(h)
    assert u.reg_read(UC_X86_REG_RIP)==end,hex(u.reg_read(UC_X86_REG_RIP))
rows=[]
cases=itertools.product([-4., 0., 2., 15., 49., 80.],[-1., -.2, 0., .2, .8],[(0.,0.),(.5,0.),(1.,0.),(0.,.5),(0.,1.)],[0,1],[0.,20.,42.,46.,49.,50.],[1,3,12])
for old_speed, tangent_y, (drive,brake), flag,t,count in cases:
    u.mem_write(bp,bytes(0x800)); u.mem_write(data,bytes(0x200))
    u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,stack+0x7000)
    xmm(UC_X86_REG_XMM6,brake);xmm(UC_X86_REG_XMM9,1.)
    run(0x1403b841e,0x1403b844e)
    # 原844e后只是接线，原855a才写min_speed；保留原算术结果。
    put(bp+0x5e0,'I',u.reg_read(UC_X86_REG_XMM1)&0xffffffff)
    put(bp+0x780,'Q',data);put(bp+0x2b0,'Q',data+0x100)
    put(data+0x10,'Q',count);put(data+0x30,'2f',0.,50.)
    put(data+0x40,'2f',t,old_speed)
    put(data+0x100,'B',flag);put(bp+0x384,'f',tangent_y);put(bp+0x50,'f',drive)
    xmm(UC_X86_REG_XMM9,1.);xmm(UC_X86_REG_XMM13,0.)
    run(0x1403b861b,0x1403b883a)
    rows.append([bits(old_speed),bits(tangent_y),bits(drive),bits(brake),flag,bits(t),count,get(data+0x44,'I')[0],get(data+0x40,'I')[0],get(bp+0x610,'Q')[0]&0xff])
u.hook_del(hook)
fixture=ROOT/'toolbar-rollercoaster-speed.windows-ucrt.ron'
fixture.write_text('[\n'+''.join('('+','.join(map(str,row))+'),\n' for row in rows)+']\n')
report={'binary_sha256':sha,'cases':len(rows),'fixture':str(fixture.relative_to(ROOT)),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'ranges':[['0x1403b841e','0x1403b844e'],['0x1403b861b','0x1403b883a']],'native_helpers':['0x1408c5680 raw byte load','0x140c904d0 lerp','0x140c904e0 time_continuous_lerp'],'bridges':[{'address':'0x142923ec0','kind':'import expf','replacement':'host Darwin libm expf(c_float)->c_float','calls':exp_calls,'limitation':'不能宣称UCRT expf逐位相等；候选差分应明确最多ULP与exp库边界，不遮掩其他差异'}],'supplied_boundaries':['已归一化的drive/brake输入，原native阻尼与min-speed算术仍执行','Spline当前切线Y与t_min=0/t_max=50，车数；不含Spline求值','原raw byte flag从单字节地址读入，语义类型需调用方确认'],'columns':['old_speed_bits','tangent_y_bits','drive_bits','brake_bits','raw_flag','old_t_bits','cart_count','speed_bits','t_bits','braking_flag_u8'],'scope':'速度、重力、加速、阻尼、上限/下限、尾车末端smoothstep减速、t clamp以及触底布尔；不含accumulator与每车动态。'}
(ROOT/'toolbar-rollercoaster-speed-native.windows.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'cases':len(rows),'expf_calls':exp_calls,'fixture_sha256':report['fixture_sha256']}))
