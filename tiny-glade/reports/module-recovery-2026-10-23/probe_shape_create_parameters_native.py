"""执行原形状创建屋顶参数指令及原Easing tag2，仅cosf跨到本机数学库。"""
from pathlib import Path
import ctypes,hashlib,itertools,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[4]
raw=(ROOT/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
cosf=ctypes.CDLL(None).cosf;cosf.restype=ctypes.c_float;cosf.argtypes=[ctypes.c_float]
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def readx(reg):return struct.unpack('<f',struct.pack('<I',u.reg_read(reg)&0xffffffff))[0]
def xmm(reg,x):u.reg_write(reg,bits(x))
calls=0
def bridge(uc,address,size,opaque):
    global calls
    if address==0x142923e80:
        calls+=1;xmm(UC_X86_REG_XMM0,cosf(readx(UC_X86_REG_XMM0)))
        sp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(sp,8))[0]
        uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
h=u.hook_add(UC_HOOK_CODE,bridge)
def run(start,end):
    def stop(uc,address,size,opaque):
        if address==end:uc.emu_stop()
    hh=u.hook_add(UC_HOOK_CODE,stop)
    try:u.emu_start(start,base+len(image),count=1000)
    finally:u.hook_del(hh)
    assert u.reg_read(UC_X86_REG_RIP)==end,hex(u.reg_read(UC_X86_REG_RIP))
def setup():
    u.reg_write(UC_X86_REG_RBP,stack+0x8000);u.reg_write(UC_X86_REG_RSP,stack+0x7000)
    u.reg_write(UC_X86_REG_RDI,data)
    for reg in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):u.reg_write(reg,0)
finite=[-100.,-1.,-0.,0.,.1,.9,1.,2.,2.25,3.,4.,5.25,6.,7.5,8.,12.75,15.,20.,1000.]
values=[bits(x) for x in finite]+[0x7fc01234,0x7f800000,0xff800000]
circle=[]
for radius in values:
    setup();u.reg_write(UC_X86_REG_XMM7,radius)
    for reg,value in [(UC_X86_REG_XMM10,7.1),(UC_X86_REG_XMM11,1.),(UC_X86_REG_XMM12,4.),(UC_X86_REG_XMM13,10.),(UC_X86_REG_XMM14,12.)]:xmm(reg,value)
    run(0x14097ebf7,0x14097ec3d)
    circle.append((radius,u.reg_read(UC_X86_REG_XMM7)&0xffffffff))
rectangle=[]
for x,y in itertools.product(values,repeat=2):
    setup();u.mem_write(data+0x20,struct.pack('<II',x,y))
    for reg,value in [(UC_X86_REG_XMM6,3.),(UC_X86_REG_XMM7,5.25),(UC_X86_REG_XMM10,1.5),(UC_X86_REG_XMM11,-1.5),(UC_X86_REG_XMM12,12.75),(UC_X86_REG_XMM13,6.),(UC_X86_REG_XMM14,12.),(UC_X86_REG_XMM15,1.)]:xmm(reg,value)
    run(0x140a93df7,0x140a93ec7)
    height=u.reg_read(UC_X86_REG_XMM8)&0xffffffff;direction=u.reg_read(UC_X86_REG_R12)&255
    # 原中间块仅尝试继承旧屋顶；此数值夹具明确取无previous分支。
    run(0x140a940d1,0x140a94106)
    rectangle.append((x,y,height,u.reg_read(UC_X86_REG_XMM9)&0xffffffff,direction))
u.hook_del(h)
fixtures={}
for name,rows in [('shape-create-circle-roof-parameters.ron',circle),('shape-create-rectangle-roof-parameters.ron',rectangle)]:
    p=ROOT/'tiny-glade/tests/fixtures/native'/name
    p.write_text('[\n'+''.join('('+','.join(map(str,row))+'),\n' for row in rows)+']\n')
    fixtures[name]={'cases':len(rows),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
report={'binary_sha256':sha,'pdb_sha256':'2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911','fixtures':fixtures,'ranges':[['0x14097ebf7','0x14097ec3d'],['0x140a93df7','0x140a93ec7'],['0x140a940d1','0x140a94106']],'executed_helper':'0x140c95ef0 EasingFunction.apply，原静态tag2与相对跳表','bridges':[{'address':'0x142923e80','kind':'import cosf','replacement':'Darwin libm cosf(c_float)','calls':calls,'scope':'数学库跨平台边界，非UCRT逐位实机证明'}],'columns':{'circle':['radius_bits','height_bits'],'rectangle':['x_bits','y_bits','height_bits','ridge_bits','direction_u8']},'supplied_boundaries':['ABI私有寄存器和shape payload；原调用者载入常量单独明确设置','无previous roof继承，继承分支由真实World集成测试验证'],'corrections':['circle ridge固定0.4、eave0.2：原142ab6600字节cdcccc3ecdcc4c3e','rectangle height easing使用未clamp的(max_dim-2.25)/12.75；ridge独立5.25','ridge保留smoothstep乘加分组与NaN传播，不擅自重排'],'scope':'仅原roof参数数学，不宣称墙创建/历史/消费者链整体通过'}
Path(__file__).with_name('shape-create-roof-parameters-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'fixtures':fixtures,'cosf_calls':calls}))
