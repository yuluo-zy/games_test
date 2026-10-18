"""Execute B InProgressCurve instructions; only Rust allocation/CRT are bridged."""
import json, math, struct, hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
source=recipe.read_text().split('def spline(')[0]
source=source.replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals())
OUT=Path(__file__).resolve().parent
RESOURCE=OUTPUT+0x400; RESULT=OUTPUT+0x600; HEADER_IN=INPUT+0x8000

def call(address,rcx,rdx=0,r8=0,r9=0):
    sp=STACK+0x10008;u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(sp,struct.pack('<Q',STOP)+bytes(128))
    for reg,value in [(UC_X86_REG_RCX,rcx),(UC_X86_REG_RDX,rdx),(UC_X86_REG_R8,r8),(UC_X86_REG_R9,r9)]:u.reg_write(reg,value)
    run(address)
def vec(addr,stride):
    cap,ptr,n=struct.unpack('<3Q',u.mem_read(addr,24))
    assert n<1000,(hex(addr),cap,ptr,n)
    return list(struct.unpack('<'+'I'*(n*stride//4),u.mem_read(ptr,n*stride))) if n else []
def curve(addr):
    cap=struct.unpack('<Q',u.mem_read(addr,8))[0]
    if cap==0x8000000000000000:return {'error':list(u.mem_read(addr+8,16))}
    return {'points':vec(addr,8),'points_u':vec(addr+24,4),'length':struct.unpack('<I',u.mem_read(addr+48,4))[0]}
def resource():
    return {'raw':curve(RESOURCE),'weights':read_usize_vec(RESOURCE+0x38),'indices':read_usize_vec(RESOURCE+0x50),'simplified_points':vec(RESOURCE+0x68,8)}
def read_usize_vec(addr):
    cap,ptr,n=struct.unpack('<3Q',u.mem_read(addr,24));assert n<1000
    return list(struct.unpack('<'+'Q'*n,u.mem_read(ptr,n*8))) if n else []
def create(points):
    reset();u.mem_write(RESOURCE,bytes(512));u.mem_write(INPUT,pack([v for p in points for v in p]))
    u.mem_write(HEADER_IN,struct.pack('<3Q',len(points),INPUT,len(points)))
    call(0x141469430,OUTPUT,HEADER_IN,0)
    assert 'error' not in curve(OUTPUT)
    call(0x140ad3070,RESOURCE,OUTPUT)
def compute(smooth):
    u.reg_write(UC_X86_REG_XMM2,bits(.1));call(0x140ad41a0,RESULT,RESOURCE,0,int(smooth));return curve(RESULT)
def add(points):
    ptr=INPUT+0x4000;u.mem_write(ptr,pack([v for p in points for v in p]));call(0x140ad3f50,RESULT,RESOURCE,ptr,len(points));return u.mem_read(RESULT,1)[0]
def weight(amount):
    n=len(read_usize_vec(RESOURCE+0x38));u.reg_write(UC_X86_REG_R14,RESOURCE);u.reg_write(UC_X86_REG_RSI,n);u.reg_write(UC_X86_REG_RCX,amount)
    u.emu_start(0x140a7d5fb,0x140a7d622,count=100)

def main():
    cases=[]
    inputs=[[[0,0],[1,0]],[[0,0],[1,1],[2,0]],[[0,0],[1,.2],[2,-.2],[3,0]],[[0,0],[.1,.4],[.2,-.4],[.3,.2],[.4,0]]]
    for n in [7,21,40]:inputs.append([[i*.1,math.sin(i*.7)*.3] for i in range(n)])
    inputs.append([[0,0],[2,1],[3,0],[1,-1],[0,0]])
    for points in inputs:
        for smooth in [False,True]:
            points=[[struct.unpack('<f',pack([v]))[0] for v in p] for p in points]
            create(points);initial=resource();first=compute(smooth);after_first=resource();last=points[-1];new=[[last[0]+.2,last[1]+.15],[last[0]+.4,last[1]-.05]]
            tag=add(new);weight(4);second=compute(smooth);after_second=resource()
            cases.append({'points':[[bits(v) for v in p] for p in points],'smooth':smooth,'initial':initial,'first':first,'after_first':after_first,'append':[[bits(v) for v in p] for p in new],'append_tag':tag,'second':second,'after_second':after_second})
    create([[0,0],[1,0]]);before=resource();tag=add([[1,0]]);failed={'before':before,'tag':tag,'after':resource()}
    report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'instruction_entries':{'new':'0x140ad3070','try_add':'0x140ad3f50','compute':'0x140ad41a0','weight':'0x140a7d5fb..0x140a7d622'},'method':'Original x86_64 code through Unicorn. Existing native_spline_rdp allocator/CRT bridges only; original Curve slicing/resampling/smoothing, RDP and std vector internals execute. No original OS process or ECS scheduler.','cases':cases,'failed_append':failed}
    (OUT/'native-fixtures.json').write_text(json.dumps(report,indent=2)+'\n')
    print({'native_cases':len(cases),'failed_append_tag':tag,'raw_before_after':(len(before['raw']['points'])//2,len(failed['after']['raw']['points'])//2)})
if __name__=='__main__':main()
