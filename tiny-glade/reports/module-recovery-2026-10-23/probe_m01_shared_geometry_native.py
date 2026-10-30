"""有界执行 B 共享几何完整函数；仅桥接四个原 CRT 数学导入，记录边界与原 panic。"""
from pathlib import Path
import ctypes, hashlib, itertools, json, math, random, struct, subprocess, tempfile
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path(__file__).resolve().parents[3]
raw = (ROOT / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(image)+4095)&~4095)
u.mem_write(base, image)
data, stack = 0x20000000, 0x30000000
u.mem_map(data, 0x10000)
u.mem_map(stack, 0x10000)
sp, sentinel = stack+0x8008, data+0xf000
libm = ctypes.CDLL(None)
imports = {}
for address, name, nargs in [(0x142923fd0, 'sinf', 1), (0x142923e80, 'cosf', 1), (0x142923f10, 'fmodf', 2), (0x142923e60, 'ceilf', 1)]:
    func = getattr(libm, name)
    func.restype = ctypes.c_float
    func.argtypes = [ctypes.c_float] * nargs
    imports[address] = (name, func, nargs)
panic_addresses = {0x1428d9430, 0x1428d9390, 0x140d8b480}
current, totals = {}, {name: 0 for name, _, _ in imports.values()}

def bits(x): return struct.unpack('<I', struct.pack('<f', x))[0]
def f32(x): return struct.unpack('<f', struct.pack('<f', x))[0]
def floats(xs): return [bits(x) for x in xs]
def put(addr, fmt, *xs): u.mem_write(addr, struct.pack('<'+fmt, *xs))
def xmm_bits(reg): return u.reg_read(reg) & 0xffffffff
def readx(reg): return struct.unpack('<f', struct.pack('<I', xmm_bits(reg)))[0]
def observer(uc, address, size, opaque):
    if address in panic_addresses:
        current['panic'] = hex(address)
        uc.emu_stop()
    elif address == 0x140ad2970:
        current['inside_calls'] += 1
    elif address in (0x140c9c730, 0x140c98170):
        current['factors'].append(xmm_bits(UC_X86_REG_XMM3))
    elif address in imports:
        name, func, nargs = imports[address]
        args = [readx(UC_X86_REG_XMM0), readx(UC_X86_REG_XMM1)][:nargs]
        result = func(*args)
        current['math'].append([name, *[bits(a) for a in args], bits(result)])
        totals[name] += 1
        uc.reg_write(UC_X86_REG_XMM0, bits(result))
        at = uc.reg_read(UC_X86_REG_RSP)
        ret = struct.unpack('<Q', uc.mem_read(at, 8))[0]
        uc.reg_write(UC_X86_REG_RSP, at+8)
        uc.reg_write(UC_X86_REG_RIP, ret)
u.hook_add(UC_HOOK_CODE, observer)

def setup():
    global current
    current = {'panic': None, 'inside_calls': 0, 'factors': [], 'math': []}
    u.mem_write(data, bytes([0xa5])*0x2000)
    u.mem_write(stack+0x1000, bytes([0xcd])*0x8000)
    put(sp, 'Q', sentinel)
    u.reg_write(UC_X86_REG_RSP, sp)
    for reg in [UC_X86_REG_RAX, UC_X86_REG_RBX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_RBP,
                UC_X86_REG_RSI, UC_X86_REG_RDI, UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_R10,
                UC_X86_REG_R11, UC_X86_REG_R12, UC_X86_REG_R13, UC_X86_REG_R14, UC_X86_REG_R15]:
        u.reg_write(reg, 0)
    for reg in range(UC_X86_REG_XMM0, UC_X86_REG_XMM15+1): u.reg_write(reg, 0)

def run(address):
    try:
        u.emu_start(address, sentinel, count=2_000_000)
    except Exception:
        print(json.dumps({'entry':hex(address),'rip':hex(u.reg_read(UC_X86_REG_RIP)), 'rsp':hex(u.reg_read(UC_X86_REG_RSP)), 'return':u.mem_read(u.reg_read(UC_X86_REG_RSP),8).hex(),'trace':current}))
        raise
    assert current['panic'] or u.reg_read(UC_X86_REG_RIP) == sentinel, hex(u.reg_read(UC_X86_REG_RIP))

def rectangle(axis=(1.,0.), center=(0.,0.), size=(4.,6.)):
    return floats([*axis, *center, *size])
axes = [(1.,0.), (1.,-0.), (0.,1.), (-1.,0.), (0.,-1.), (.6,.8), (.8,-.6),
        (math.cos(.37),math.sin(.37)), (math.cos(2.7),math.sin(2.7))]
special = [bits(x) for x in [-float('inf'), -20., -1., -0., 0., .1, 1., 20., float('inf')]] + [0x7fc12345]
expand = []
expand_cases = []
for axis, edge, delta, limits in itertools.product(axes, range(4), special, [(-2., 10.), (0., 100.), (3.,3.), (-float('inf'),float('inf'))]):
    expand_cases.append((rectangle(axis,(-0.,-0.)), edge, delta, *floats(limits)))
for edge, bounds in itertools.product([0,1,2,3,4,7], [(1.,0.), (float('nan'),1.), (0.,float('nan'))]):
    expand_cases.append((rectangle(), edge, bits(2.), *floats(bounds)))
for edge in [4,7]:
    expand_cases.append((rectangle(),edge,bits(2.),bits(0.),bits(10.)))
for field, value, edge, delta in itertools.product(range(6), [0x7fc12345,0xffc54321,0x7fa12345,0xffa54321,0x7f800000,0xff800000], range(4), [bits(0.),bits(2.),0x7fc45678,0xffc87654,0x7fa45678,0xffa87654,0x7f800000,0xff800000]):
    record = rectangle(); record[field] = value
    expand_cases.append((record,edge,delta,bits(-float('inf')),bits(float('inf'))))
for record, edge, delta, lower, upper in expand_cases:
    setup()
    put(data, '6I', *record)
    u.reg_write(UC_X86_REG_RCX, data)
    u.reg_write(UC_X86_REG_RDX, edge)
    u.reg_write(UC_X86_REG_XMM2, delta)
    u.reg_write(UC_X86_REG_XMM3, lower)
    put(sp+0x28, 'I', upper)
    run(0x140c9c620)
    expand.append({'before':record, 'edge':edge, 'delta':delta, 'lower':lower, 'upper':upper,
                   'panic':bool(current['panic']), 'after':list(struct.unpack('<6I',u.mem_read(data,24)))})

# Unicorn SoftFloat 在两个 NaN 同时参与运算时选更大 payload，与真实 x86 操作数顺序不符。
# 直接在 Rosetta x86_64 映射原 RNE，完整执行不调用 CRT 的 expand，作为非 panic 输出的权威交叉验证。
rosetta_differences = []
with tempfile.TemporaryDirectory(prefix='m01-original-expand-') as temporary:
    temporary = Path(temporary)
    image_path, input_path, output_path, executable = [temporary / name for name in ['original-image.bin','input.bin','output.bin','expand-x86']]
    image_path.write_bytes(image)
    with input_path.open('wb') as stream:
        for index, row in enumerate(expand):
            if not row['panic']:
                stream.write(struct.pack('<I6IQ3I',index,*row['before'],row['edge'],row['delta'],row['lower'],row['upper']))
    source = Path(__file__).with_name('probe_m01_expand_x86.c')
    subprocess.run(['/usr/bin/clang','-arch','x86_64','-O2',str(source),'-o',str(executable)],check=True)
    result = subprocess.run(['arch','-x86_64',str(executable),str(image_path),str(input_path),str(output_path)],check=True,capture_output=True,text=True)
    output = output_path.read_bytes()
    assert len(output) == sum(not row['panic'] for row in expand)*28
    for at in range(0,len(output),28):
        index,*values = struct.unpack_from('<7I',output,at)
        row=expand[index]
        if values != row['after']:
            rosetta_differences.append({'index':index,'rosetta':values,'unicorn':row['after'],'input':dict(row)})
            row['unicorn_after']=row['after']
        row['after']=values
    rosetta = {'cases':len(output)//28,'stdout':result.stdout,'stderr':result.stderr.strip(),
               'mapped_image_sha256':hashlib.sha256(image_path.read_bytes()).hexdigest(),
               'executable_sha256':hashlib.sha256(executable.read_bytes()).hexdigest(),
               'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
               'input_sha256':hashlib.sha256(input_path.read_bytes()).hexdigest(),
               'output_sha256':hashlib.sha256(output).hexdigest(),'bridges':[],
               'scope':'Rosetta x86_64 直接调用原RNE完整expand；不含panic实现与Windows CRT，原函数不需要CRT'}
Path(__file__).with_name('m01-expand-rosetta-unicorn-differences.json').write_text(json.dumps(rosetta_differences,indent=2)+'\n')

lerp = []
factors = floats([-1., -0., 0., .05, .25, .5, .95, 1., 2., float('inf')]) + [0x7fc12345]
lerp_cases = [(rectangle(a,(12.3,-7.1),(4.,6.)),rectangle(b,(-2.7,8.3),(10.,2.)),t)
              for a,b,t in itertools.product(axes,axes,factors)]
# 公开可变字段使有限度数值损坏进入 helper；原 guard 的 panic 同样属于结果。
for index,value in itertools.product(range(6),[0,0x80000000,0x7fc12345,0x7f800000]):
    before = rectangle(); before[index] = value
    lerp_cases.append((before,rectangle(),bits(.5)))
rng = random.Random(0x140c9c730)
for _ in range(512):
    a,b = rng.uniform(-math.pi,math.pi),rng.uniform(-math.pi,math.pi)
    lerp_cases.append((rectangle((math.cos(a),math.sin(a)),(rng.uniform(-20,20),rng.uniform(-20,20))),
                       rectangle((math.cos(b),math.sin(b)),(rng.uniform(-20,20),rng.uniform(-20,20))), bits(rng.uniform(-1,2))))
for before,after,factor in lerp_cases:
    setup()
    put(data+0x100,'6I',*before);put(data+0x200,'6I',*after)
    u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_RDX,data+0x100);u.reg_write(UC_X86_REG_R8,data+0x200)
    u.reg_write(UC_X86_REG_XMM3,factor)
    run(0x140c9c730)
    lerp.append({'before':before,'after':after,'factor':factor,'panic':bool(current['panic']),
                 'output':list(struct.unpack('<6I',u.mem_read(data,24))), 'math':current['math']})

def circle_record(center=(0.,0.),radius=1.,rotation=0.,tail=(0x12345678,0x9abcdef0)):
    return [0,*floats([*center,radius,rotation]),*tail]
def rect_record(axis=(1.,0.),center=(0.,0.),size=(4.,6.)):
    return [1,*rectangle(axis,center,size)]
search = []
search_cases = []
for border in [circle_record(radius=10.),rect_record(size=(20.,20.)),rect_record(axes[5],size=(20.,16.))]:
    for start,end,rotation in itertools.product([0.,9.,10.,11.],[-20.,0.,9.,10.,20.],[-0., .3, 6.2]):
        search_cases.append((border,circle_record((start,0.),1.,rotation),circle_record((end,0.),2.,-rotation)))
    for axis, start, end in itertools.product(axes,[0.,8.,12.],[-20.,0.,20.]):
        search_cases.append((border,rect_record(axis,(start,0.),(2.,3.)),rect_record(axes[5],(end,3.),(4.,6.))))
    for a,b in [(circle_record(),rect_record()),(rect_record(),circle_record())]:
        search_cases.append((border,a,b))
# 起点越界且 after 含 NaN：区分返回 before 原字节与重新生成 t=0 candidate。
search_cases += [(rect_record(size=(2.,2.)),circle_record((3.,-0.),1.,-0.),circle_record((float('nan'),0.),1.,1.)),
                 (rect_record(size=(2.,2.)),rect_record(center=(3.,-0.)),rect_record(center=(float('nan'),0.))),
                 (circle_record(radius=10.),circle_record((0.,0.),10.),circle_record((0.,0.),10.))]
for field, value in itertools.product(range(1,5), [0,0x80000000,0x7fc12345,0x7f800000,0xff800000]):
    before = circle_record()
    before[field] = value
    search_cases.append((circle_record(radius=10.),before,circle_record((5.,2.),2.,.7)))
for field, value in itertools.product(range(1,7), [0x7fc12345,0x7f800000]):
    before = rect_record()
    before[field] = value
    search_cases.append((circle_record(radius=10.),before,rect_record(axes[5],(5.,2.))))
# 角点距离平方为 next_down(100)，sqrt 舍入成10；精确区分原 squared predicate 与错误 sqrt predicate。
near_eight = struct.unpack('<f', struct.pack('<I', bits(8.)-1))[0]
search_cases.append((circle_record(radius=10.),rect_record(center=(5.5,f32(near_eight-.5)),size=(1.,1.)),rect_record(size=(1.,1.))))
for border,before,after in search_cases:
    setup()
    put(data+0x100,'7I',*border);put(data+0x200,'7I',*before);put(data+0x300,'7I',*after)
    u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_RDX,data+0x100)
    u.reg_write(UC_X86_REG_R8,data+0x200);u.reg_write(UC_X86_REG_R9,data+0x300)
    run(0x140ad2ce0)
    output=list(struct.unpack('<7I',u.mem_read(data,28)))
    t0_failed=current['inside_calls']==1 and output==before
    search.append({'border':border,'before':before,'after':after,'panic':bool(current['panic']),
                   'output':output,'inside_calls':current['inside_calls'],'factors':current['factors'],
                   'preserved_before':t0_failed})

fixture=ROOT/'tiny-glade/tests/fixtures/native/m01-shared-geometry.json'
fixture.write_text(json.dumps({'expand':expand,'lerp':lerp,'search':search},separators=(',',':'))+'\n')
report={'binary_sha256':sha,'seed':'0x140c9c730','fixture':str(fixture.relative_to(ROOT)),
        'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),
        'counts':{'expand':len(expand),'lerp':len(lerp),'search':len(search),
                  'search_20_steps':sum(x['inside_calls']==20 for x in search),
                  'search_before_exact_bytes':sum(x['preserved_before'] for x in search)},
        'expand_rosetta_crosscheck':rosetta,'unicorn_double_nan_differences':len(rosetta_differences),
        'functions':['0x140c9c620 expand_along_edge complete','0x140c9c730 Rectangle2d::lerp complete',
                     '0x140ad2ce0 search_lerped_shape_that_fits_into_glade complete',
                     '0x140ad2970 is_shape_inside complete and original circle/rectangle callees'],
        'bridges':[{'address':hex(address),'kind':'original CRT import','replacement':'Darwin libm '+name,
                    'calls':totals[name],'limitation':'只验证相同宿主数学库边界的原算法；不是Windows UCRT与Darwin逐位等价证明'}
                   for address,(name,_,_) in imports.items()],
        'panic_observation':'在原 panic/panic_fmt/clamp panic 入口停止，不替换成功算法',
        'comparison':'输出定义字段逐位、panic一致；search t0失败额外完整28B before比较；circle成功尾8B原未初始化不作为语义字段',
        'supplied_boundaries':['原私有ABI的已解析形状记录，真实边界函数和插值完整执行','栈初始为0xcd，输出存储初始为0xa5；不注入输出或边界判定']}
Path(__file__).with_name('m01-shared-geometry-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(report['counts']))
