"""Original B borrowed LoopSlice entries and typed RDP producer fixtures."""
import importlib.util,json,random,struct
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).parent;ROOT=OUT.parents[2]
prior=ROOT/'evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
spec=importlib.util.spec_from_file_location('original',prior)
o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
back_stop=False
stitch_stop=False
def bound(uc,address,size,data):
    if back_stop and address in [0x141237cf7,0x141237d07]:uc.emu_stop()
    if stitch_stop and address==0x140ad4bab:uc.emu_stop()
o.u.hook_add(UC_HOOK_CODE,bound)
def loop(start,end,period,count,ops):
    global back_stop
    o.reset();view=o.INPUT+0x1000;source=o.INPUT
    o.u.mem_write(source,o.pack([float(v) for i in range(count) for v in [i,100+i,200+i]]))
    o.u.mem_write(view,struct.pack('<QQiii',source,count,start,end,period))
    records=[]
    for operation in ops:
        sp=o.STACK+0x10008;o.u.reg_write(UC_X86_REG_RSP,sp);o.u.mem_write(sp,struct.pack('<Q',o.STOP))
        if operation=='f':
            o.u.reg_write(UC_X86_REG_RCX,view);o.run(0x141208e10)
            pointer=o.u.reg_read(UC_X86_REG_RAX)
        else:
            o.u.reg_write(UC_X86_REG_RDX,view-0x58);o.u.reg_write(UC_X86_REG_R15,view-0x58)
            back_stop=True
            o.u.emu_start(0x141237cbc,o.STOP,count=10000)
            assert o.u.reg_read(UC_X86_REG_RIP) in [0x141237cf7,0x141237d07]
            pointer=o.u.reg_read(UC_X86_REG_R8);back_stop=False
        state=struct.unpack('<iii',o.u.mem_read(view+16,12))
        records.append({'operation':operation,'index':None if not pointer else (pointer-source)//12,'state':state})
    return {'range':[start,end,period],'source_count':count,'steps':records}
def rdp2(points):
    o.reset();o.u.mem_write(o.INPUT,o.pack([v for p in points for v in p]));o.u.reg_write(UC_X86_REG_RCX,o.OUTPUT);o.u.reg_write(UC_X86_REG_RDX,o.INPUT);o.u.reg_write(UC_X86_REG_R8,len(points))
    o.run(0x140ad1fe0)
    cap,pointer,n,pcap,pptr,pn=struct.unpack('<6Q',o.u.mem_read(o.OUTPUT,48))
    indices=list(struct.unpack('<'+'Q'*n,o.u.mem_read(pointer,8*n))) if n else []
    return {'points':[[o.bits(v) for v in p] for p in points],'indices':indices}
def stitch(old_indices,new_indices,at):
    global stitch_stop
    o.reset();body=o.INPUT+0x1000;old_i=o.INPUT;old_p=o.INPUT+0x2000;new_i=o.INPUT+0x4000;new_p=o.INPUT+0x6000
    old_points=[[float(i),float(i+10)] for i in old_indices]
    new_points=[[float(i+old_indices[at]),float(i+old_indices[at]+10)] for i in new_indices]
    o.u.mem_write(old_i,struct.pack('<'+'Q'*len(old_indices),*old_indices));o.u.mem_write(old_p,o.pack([v for p in old_points for v in p]))
    o.u.mem_write(new_i,struct.pack('<'+'Q'*len(new_indices),*new_indices));o.u.mem_write(new_p,o.pack([v for p in new_points for v in p]))
    o.u.mem_write(body+0x50,struct.pack('<6Q',len(old_indices),old_i,len(old_indices),len(old_points),old_p,len(old_points)))
    bp=o.STACK+0x8000;sp=bp-0x80
    o.u.mem_write(bp+0xc8,struct.pack('<Q',body+0x50));o.u.mem_write(bp+0x118,struct.pack('<Q',len(old_indices)))
    o.u.mem_write(bp+0xe0,struct.pack('<6Q',len(new_indices),new_i,len(new_indices),len(new_points),new_p,len(new_points)))
    for register,value in [(UC_X86_REG_RDI,body),(UC_X86_REG_RBX,old_i),(UC_X86_REG_R12,at),(UC_X86_REG_R14,old_indices[at]),(UC_X86_REG_R15,at*8),(UC_X86_REG_RBP,bp),(UC_X86_REG_RSP,sp)]:o.u.reg_write(register,value)
    stitch_stop=True;o.u.emu_start(0x140ad47ab,o.STOP,count=3000000);stitch_stop=False
    assert o.u.reg_read(UC_X86_REG_RIP)==0x140ad4bab,hex(o.u.reg_read(UC_X86_REG_RIP))
    cap,ip,n,pcap,pp,pn=struct.unpack('<6Q',o.u.mem_read(body+0x50,48))
    return {'old_indices':old_indices,'new_indices':new_indices,'at':at,'old_points':old_points,'new_points':new_points,'result_indices':list(struct.unpack('<'+'Q'*n,o.u.mem_read(ip,n*8))),'result_points':[list(struct.unpack('<2f',o.u.mem_read(pp+i*8,8))) for i in range(pn)]}
if __name__=='__main__':
    fixtures=[]
    for n in [1,2,3,7]:
        for start,end in [(0,0),(0,n),(0,n+1),(-1,n),(n,n+1)]:
            for operations in ['f'*(end-start+2),'b'*(end-start+2),('fb'*(end-start+2))]:
                fixtures.append(loop(start,end,n,n,operations))
    rng=random.Random(130137)
    rdp=[rdp2([[float(i),rng.uniform(-3,3)] for i in range(n)]) for n in [0,1,2,3,4,8,16,31] for _ in range(6)]
    stitched=[stitch([0,2,7,12],[0,3,6],at) for at in range(4)]
    report={'original_sha256':o.hashlib.sha256(o.raw).hexdigest(),'loop_method':'Forward original complete LoopSlice::next entry 0x141208e10; backward bounded original consumer prefix 0x141237cbc with its LoopRange::next_back / LoopIdx::index callees unchanged. Stop before unrelated consumer state mutation.','loop':fixtures,'rdp2':rdp,'stitch':stitched}
    (OUT/'native-base-fixtures.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    test='use super::*;\n#[test]\nfn original_b_loop_slice_state_transitions() {\n'
    for row in fixtures:
        a,b,period=row['range'];n=row['source_count']
        test+=f'{{let source:Vec<_>=(0..{n}).collect();let mut view=LoopRange{{start_idx:{a},end_idx:{b},period:{period}}}.map_slice(&source);\n'
        for step in row['steps']:
            function='next' if step['operation']=='f' else 'next_back'
            expected='None' if step['index'] is None else 'Some(&'+str(step['index'])+')'
            test+=f'assert_eq!(view.{function}(),{expected});assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],{list(step["state"])});\n'
        test+='}\n'
    test+='}\n'
    (OUT/'loop_slice_tests.rs').write_text(test,encoding='utf-8')
    print('loop traces',len(fixtures),'rdp2',len(rdp))
