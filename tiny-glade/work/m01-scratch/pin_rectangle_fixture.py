exec(open('D:/game/tiny-glade/work/m01-scratch/rotate_fragment_fixture.py').read().split('cases=')[0])
def rect(axis,center,size):return struct.pack('<6f',*axis,*center,*size)
def test(a,b):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x100000,0x10000);u.reg_write(UC_X86_REG_RSP,0x108008);u.mem_write(0x108008,struct.pack('<Q',0x100200));u.mem_write(0x100000,rect(*a));u.mem_write(0x100100,rect(*b));u.reg_write(UC_X86_REG_RCX,0x100000);u.reg_write(UC_X86_REG_RDX,0x100100)
 from unicorn import UC_HOOK_CODE
 def observe(uc,address,size,data):
  if address==0x1428d9430:uc.emu_stop()
 u.hook_add(UC_HOOK_CODE,observe);u.emu_start(0x140c9b990,0x100200,count=100000)
 if u.reg_read(UC_X86_REG_RIP)==0x1428d9430:return 'panic_assert'
 assert u.reg_read(UC_X86_REG_RIP)==0x100200;return bool(u.reg_read(UC_X86_REG_RAX)&255)
if '--legacy-oracle' in sys.argv:
 import re
 source=Path('D:/game/tiny-glade/crates/utils/tests/oracle/foundation-completion-2026-10-13/geometry/native_tests.rs');text=source.read_text();begin=text.index('fn original_completion_rectangle_intersections()');end=text.find('#[test]',begin+5);body=text[begin:end if end>=0 else len(text)]
 pattern=r'let r=rect\(\[([^\]]+)\]\);assert_eq!\(r\.intersects\(rect\(\[([^\]]+)\]\)\),(true|false)\)'
 rows=[]
 def decode(bits):
  f=[struct.unpack('<f',struct.pack('<I',x))[0] for x in bits];return (f[:2],f[2:4],f[4:6])
 for m in re.finditer(pattern,body):
  a=[int(v) for v in m[1].split(',')];b=[int(v) for v in m[2].split(',')];old=m[3]=='true';native=test(decode(a),decode(b));rows.append({'line':text[:begin+m.start()].count('\n')+1,'a_bits':a,'b_bits':b,'old_oracle':old,'original_intersects':native,'old_differs':old!=native})
 out={'scope':'Every Rectangle2d::intersects input in the old original_completion_rectangle_intersections test, decoded from literal u32 bits; complete original PE function/helper execution without business or CRT substitutions','pe_sha256':hashlib.sha256(raw).hexdigest(),'oracle_source_path':str(source),'oracle_source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'cases':rows,'old_oracle_differences':sum(x['old_differs'] for x in rows)}
 Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-pin-rectangle-legacy-oracle-fixture.json').write_text(json.dumps(out,indent=2,allow_nan=False));print(json.dumps({'cases':len(rows),'old_oracle_differences':out['old_oracle_differences'],'first_difference':next((r for r in rows if r['old_differs']),None)},indent=2));raise SystemExit
cases=[('contained',((1,0),(0,0),(2,2)),((1,0),(0,0),(1,1))),('equal',((1,0),(0,0),(2,2)),((1,0),(0,0),(2,2))),('cross-axis',((1,0),(0,0),(2,2)),((1,0),(1,0),(2,2))),('cross-rotated',((1,0),(0,0),(2,2)),((.6,.8),(1,0),(2,2))),('equal-rotated',((.6,.8),(0,0),(2,2)),((.6,.8),(0,0),(2,2)))]
for n,a,b in [('self-zero-axis',((0,0),(0,0),(2,2)),((1,0),(0,0),(1,1))),('self-negative-size',((1,0),(0,0),(-2,-2)),((1,0),(0,0),(1,1))),('other-negative-pair',((1,0),(0,0),(2,2)),((1,0),(1,0),(-2,-2))),('other-negative-one',((1,0),(0,0),(2,2)),((1,0),(0,0),(-2,2))),('other-zero-size',((1,0),(0,0),(2,2)),((1,0),(0,0),(0,2))),('self-size-nan',((1,0),(0,0),(float('nan'),2)),((1,0),(0,0),(1,1)))]:cases.append((n,a,b))
import random,math
rng=random.Random(20261028)
for i in range(64):
 axis=lambda: (rng.uniform(-3,3),rng.uniform(-3,3))
 extent=lambda: (rng.uniform(.01,20),rng.uniform(.01,20))
 base_center=tuple(rng.uniform(-1e7,1e7) for _ in range(2)) if i%3==0 else tuple(rng.uniform(-20,20) for _ in range(2))
 offset=tuple(rng.uniform(-10,10) for _ in range(2))
 cases.append((f'random-{i}',(axis(),base_center,extent()),(axis(),tuple(base_center[j]+offset[j] for j in range(2)),extent())))
for n,axis,center,extent in [('negative-zero-axis',(-0.,-0.),(0.,0.),(2.,2.)),('infinite-axis',(math.inf,0.),(0.,0.),(2.,2.)),('infinite-center',(1.,0.),(math.inf,0.),(2.,2.)),('infinite-size',(1.,0.),(0.,0.),(math.inf,2.)),('axis-subnormal',(1e-30,1e-30),(0.,0.),(2.,2.)),('size-subnormal',(1.,0.),(0.,0.),(1e-30,1e-30))]:cases.append((n,(axis,center,extent),((1.,-0.),(1.,-0.),(2.,2.))))
out={'scope':'Original PE Rectangle2d::intersects 140c9b990 and its original helper executed in Unicorn without import bridges or algorithm substitutions; panic-entry observation stops invalid inputs before assertion machinery; not full ShowPinned consumer','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'name':n,'a':a,'b':b,'intersects':test(a,b)} for n,a,b in cases]}
import math
def safe(value):
 if isinstance(value,float) and not math.isfinite(value):return str(value)
 if isinstance(value,(tuple,list)):return [safe(v) for v in value]
 if isinstance(value,dict):return {k:safe(v) for k,v in value.items()}
 return value
out=safe(out)
Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-pin-rectangle-intersects-fixture.json').write_text(json.dumps(out,indent=2,allow_nan=False));print(json.dumps(out,indent=2,allow_nan=False))
def extent_bits(a,b):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.reg_write(UC_X86_REG_XMM15,a);u.reg_write(UC_X86_REG_XMM6,b);u.emu_start(0x141236bd9,0x141236c04);return f'{u.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}'
extent_cases=[(0,0x80000000),(0x80000000,0),(0x7fc12345,0x3f800000),(0x3f800000,0x7fc12345),(0x7fc12345,0x7fc54321),(0xbf800000,0x3f800000),(0x3f800000,0x40000000),(0x7f800000,0x7f800000)]
extent_out={'scope':'Unicorn original PE extent span141236bd9..c00, x in XMM15 and y in XMM6; no hooks or substitutions','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'x_bits':f'{a:08x}','y_bits':f'{b:08x}','extent_bits':extent_bits(a,b)} for a,b in extent_cases]}
Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-pin-extent-fragment-fixture.json').write_text(json.dumps(extent_out,indent=2,allow_nan=False))
def progress_case(bits,expiring):
 from unicorn import UC_HOOK_CODE
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x100000,4096);u.mem_write(0x100014,bytes([expiring]));u.reg_write(UC_X86_REG_RDI,0x100000);u.reg_write(UC_X86_REG_XMM0,bits);u.reg_write(UC_X86_REG_XMM9,0);u.reg_write(UC_X86_REG_XMM10,0x3f800000)
 def observe(uc,address,size,data):
  if address in [0x141235f11,0x141235f70]:uc.emu_stop()
 hook=u.hook_add(UC_HOOK_CODE,observe);u.emu_start(0x141235f05,0x141235f80);target=u.reg_read(UC_X86_REG_RIP);u.hook_del(hook)
 result={'progress_bits':f'{bits:08x}','expiring':bool(expiring),'branch_target':hex(target),'action':'retain' if target==0x141235f70 else 'despawn'}
 if target==0x141235f70:
  u.emu_start(0x141235f70,0x141235f7f);result['clamped_bits']=f'{u.reg_read(UC_X86_REG_XMM1)&0xffffffff:08x}'
 return result
progress_out={'scope':'Original PE branch141235f05..f0f stops at retain/despawn labels, retained clamp141235f70..f7b; no ECS executed or algorithm substitutions','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[progress_case(bits,flag) for bits in [0x7fc12345,0xffc12345,0,0x80000000,0xbf800000,0x3f000000,0x40000000] for flag in [0,1]]}
Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-pin-progress-fragment-fixture.json').write_text(json.dumps(progress_out,indent=2,allow_nan=False))
