"""原dispatcher有界FP块与原Rectangle叶执行；不桥接算术或contains逻辑。"""
from pathlib import Path
import json,struct,itertools
source=Path(__file__).with_name('m01_dispatch_start_place_native.py').read_text()
exec(compile(source[:source.index('\nrows=[]')],'retained-pe-setup','exec'))
bp=S+0x80000
def xmm(reg,raw):u.reg_write(reg,int.from_bytes(raw.ljust(16,b'\0'),'little'))
def fraw(v):return struct.pack('<f',v)
def reset():
 u.mem_write(S,bytes(0x100000));u.reg_write(UC_X86_REG_RSP,S+0x70000);u.reg_write(UC_X86_REG_RBP,bp)
 for reg,addr,size in [(UC_X86_REG_XMM7,0x142a16cb8,4),(UC_X86_REG_XMM8,0x142925984,4),(UC_X86_REG_XMM10,0x142a81ca0,4),(UC_X86_REG_XMM11,0x142930d60,16)]:xmm(reg,bytes(u.mem_read(addr,size)))
rows=[]
for position,axis,offset in itertools.product([(0.,0.,0.),(2.3,-4.7,5.1),(-1000.25,4096.5,0.125)],[(1.,0.),(0.,1.),(.6,.8)],[(0.,-.5,0.),(0.,0.,0.),(0.,.1,0.),(0.,-1.1,0.),(.775,0.,0.),(-.575,0.,0.),(.776,0.,0.),(0.,0.,.775),(0.,0.,-.575),(2.,2.,2.)]):
 reset();params=D+0x80000;words=[0.,0.,position[0],position[2],0.,position[1],*axis]
 u.mem_write(params,struct.pack('<8f',*words));u.reg_write(UC_X86_REG_R14,params)
 point=tuple(a+b for a,b in zip(position,offset));u.mem_write(bp+0x448,struct.pack('<3f',*point))
 u.emu_start(0x1407b08a5,0x1407b097a,count=10000)
 rows.append(['C',struct.pack('<8f',*words).hex(),struct.pack('<3f',*point).hex(),bytes(u.mem_read(bp+0x370,32)).hex(),str(uq(bp+0x508)&1)])
values=[0.,-0.,1.,-1.,1e-10,1e10,1e19,3.1415927]
for delta in itertools.product(values,repeat=3):
 reset()
 for reg,v in zip([UC_X86_REG_XMM12,UC_X86_REG_XMM6,UC_X86_REG_XMM13],delta):xmm(reg,fraw(v))
 u.emu_start(0x1407b0d1d,0x1407b0d3d,count=30)
 rows.append(['D',struct.pack('<3f',*delta).hex(),struct.pack('<I',r(UC_X86_REG_XMM3)&0xffffffff).hex()])
for terrain,base,extra,hit in itertools.product([False,True],[0.,-3.5,4096.25],[0.,-2.,.125],[0.,.9,1.,1.00000012,2.,-2.,4096.25,float('nan'),float('inf')]):
 reset();xmm(UC_X86_REG_XMM6,fraw(base));p=D+0x81000;u.mem_write(p,fraw(extra));u.reg_write(UC_X86_REG_RAX,p);u.mem_write(bp+0x44c,fraw(hit))
 u.emu_start(0x1407b0c8b if terrain else 0x1407b09fe,0x1407b0cbb,count=30)
 rows.append(['S',str(int(terrain)),fraw(base).hex(),fraw(extra).hex(),fraw(hit).hex(),str(uq(bp+0x508)&1)])
out=Path(__file__).with_name('m01-dispatch-geometry-native.tsv');out.write_text('# Original dispatcher blocks + original Rectangle constructor/contains; C=chimney,D=distance,S=stair\n'+'\n'.join('\t'.join(row)for row in rows)+'\n')
print(json.dumps({'binary_sha256':sha,'cases':len(rows),'chimney':sum(r[0]=='C'for r in rows),'distance':sum(r[0]=='D'for r in rows),'stair':sum(r[0]=='S'for r in rows),'constants':{hex(a):bytes(u.mem_read(a,n)).hex()for a,n in [(0x142a16cb8,4),(0x142925984,4),(0x142a81ca0,4),(0x142930d60,16)]}}))
