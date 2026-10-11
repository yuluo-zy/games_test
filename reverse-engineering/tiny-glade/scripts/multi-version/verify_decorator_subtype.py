"""Untouched B wall destination subtype + actual half-dormer consumer.
Bridges only resolved PublicWalls/Roof access and std collision map lookup.
"""
import hashlib,json,math,random,struct,subprocess
from pathlib import Path
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,setup,ret,bits
from verify_building_state import reset
from verify_decorator_placement import make_curve,text_values,pack_floats,f,original_bbox
from verify_opening_rules import original as original_holes
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
active=False;current_roof=0

def hook(c,pc,sz,user):
    if not active:return
    if pc==0x140a91ef0:
        c.reg_write(UC_X86_REG_RAX,DATA+0x3000);ret(c)
    elif pc==0x140813dd6:
        # Actual resolved optional Roof component after PublicWalls roof_entity
        # and Bevy query access. The original gameplay branch resumes unchanged.
        c.reg_write(UC_X86_REG_RDI,current_roof);c.reg_write(UC_X86_REG_RIP,0x140814265)
    elif pc==0x1408145e1:
        # Successful plain-window key lookup. Real collision record, not output
        # half flag, interpolated height, segment or local fraction.
        c.reg_write(UC_X86_REG_RCX,DATA+0x9020);c.reg_write(UC_X86_REG_RSI,0)
        c.reg_write(UC_X86_REG_RIP,0x14081470f)
cpu.hook_add(UC_HOOK_CODE,hook)
def original(kind,door,corner,coord,curve,outline,heights,shape,roof,bounds):
    global active,current_roof
    reset();setup();cpu.mem_write(DATA,bytes(0x12000))
    block=bytearray(0x48);struct.pack_into('<2f',block,0,*coord);struct.pack_into('<Q',block,8,door)
    block[0x20]=corner;block[0x22]=0;block[0x44]=1;block[0x46]=0x38 if kind=='cw' else 0x39
    cpu.mem_write(DATA+0x2000,bytes(block));cpu.mem_write(DATA+0x1000,struct.pack('<QQQ',DATA+0x2000,0,1))
    points,us,length=curve
    cpu.mem_write(DATA+0x3000,struct.pack('<QQQQQQf',len(points),DATA+0x4000,len(points),len(us),DATA+0x5000,len(us),length))
    cpu.mem_write(DATA+0x4000,pack_floats([x for p in points for x in p]));cpu.mem_write(DATA+0x5000,pack_floats(us))
    cpu.mem_write(DATA+0x3088,struct.pack('<QQQ',len(outline),DATA+0x6000,len(outline)))
    cpu.mem_write(DATA+0x6000,pack_floats([x for p in outline for x in p]));cpu.mem_write(DATA+0x30e0,struct.pack('<QQ',DATA+0x8000,len(heights)))
    cpu.mem_write(DATA+0x8000,pack_floats(heights));cpu.mem_write(DATA+0x3258,struct.pack('<I',shape))
    current_roof=DATA+0xa000 if roof is not None else 0
    if roof is not None:cpu.mem_write(current_roof,roof)
    cpu.mem_write(DATA+0x9000,pack_floats([1.,0.,0.,0.,1.,1.,*bounds]))
    cpu.reg_write(UC_X86_REG_RCX,DATA+0x1000);cpu.reg_write(UC_X86_REG_RDX,DATA+0xb000)
    cpu.reg_write(UC_X86_REG_R8,DATA+0xc000);cpu.reg_write(UC_X86_REG_R9,DATA+0xd000)
    active=True
    try:cpu.emu_start(0x140813d00,STACK,count=200000)
    finally:active=False
    assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    return str(cpu.reg_read(UC_X86_REG_RAX)&255)
def make_roof(height,ridge,direction,w=8.,l=4.,angle=0.,cx=0.,cz=0.):
    b=bytearray(88);struct.pack_into('<I6f',b,8,1,math.cos(angle),math.sin(angle),cx,cz,w,l)
    struct.pack_into('<ff',b,0x30,height,ridge);b[0x3c]=direction;return bytes(b)
def original_half_asset_name(subtype,rank):
    start,lookup=(0x1407b87f2,0x1407bb4b0)if subtype==1 else (0x1407b726d,0x1407bb160)
    reset();setup();cpu.mem_write(DATA,bytes(0x8000));cpu.reg_write(UC_X86_REG_RBP,DATA+0x2000)
    cpu.reg_write(UC_X86_REG_RSI,DATA+0x4000);cpu.reg_write(UC_X86_REG_RDI,rank)
    cpu.mem_write(DATA+0x28b0,struct.pack('<Q',DATA+0x4000));cpu.emu_start(start,lookup,count=100)
    assert cpu.reg_read(UC_X86_REG_RIP)==lookup
    return bytes(cpu.mem_read(cpu.reg_read(UC_X86_REG_R8),cpu.reg_read(UC_X86_REG_R9))).decode()
def main():
    rng=random.Random(20261014);rows=[];expected=[];kinds=[];assets={}
    assets_root=Path(baseline['binary']['path']).parent/'assets/meshes/decorators'
    actual_bounds={}
    for kind,family in [('cw','cottage'),('gw','gothic')]:
        for rank in [1,2,3]:
            p=assets_root/f'window_{family}_{rank}x1_collision.json';assets[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
            bbx=original_bbox(json.loads(p.read_text())['Vertex_Position']['buffer'])
            values=[struct.unpack('<f',struct.pack('<I',int(v,16)))[0]for v in bbx.split(',')];actual_bounds[kind,rank]=values[-2:]
    for i in range(1152):
        kind=['cw','gw'][i%2];rank=i%3+1;bounds=actual_bounds[kind,rank]
        angle=f(rng.uniform(-math.pi,math.pi));c=f(math.cos(angle));s=f(math.sin(angle));w=f(rng.uniform(1.,20.));l=f(rng.uniform(1.,20.));cx=f(rng.uniform(-30,30));cz=f(rng.uniform(-30,30))
        local=[[-w*.5,-l*.5],[-w*.5,l*.5],[w*.5,l*.5],[w*.5,-l*.5],[-w*.5,-l*.5]]
        raw=[[f((-s*z+c*x)+cx),f((c*z+s*x)+cz)]for x,z in local];curve=make_curve(raw)
        outline=[]
        for a,b in zip(raw,raw[1:]):
            for k in range(4):
                t=f(k*.25);outline.append([f(t*b[0]+f(1-t)*a[0]),f(rng.uniform(-2.,2.)),f(t*b[1]+f(1-t)*a[1])])
        outline.append([raw[-1][0],0.,raw[-1][1]])
        heights=[f(rng.uniform(0.,10.))for _ in outline]
        # Covers published endpoints, interior fractions, source extension and
        # high/low/equal-near collision boundaries over changing terrain Y.
        distance=f(curve[2]*[-.1,0.,.125,.25,.5,.75,1.,1.1][i%8]);coord=[distance,f(rng.uniform(-2.,12.))]
        roof_kind=i%6
        roof=[None,make_roof(.05,1.,0,w,l,angle,cx,cz),make_roof(.8,.6,0,w,l,angle,cx,cz),make_roof(.8,1.,0,w,l,angle,cx,cz),make_roof(.8,1.,1,w,l,angle,cx,cz),make_roof(.1,.9,1,w,l,angle,cx,cz)][roof_kind]
        door=i%11==0;corner=i%17==0;shape=2 if i%29==0 else i%2
        args=(kind,door,corner,coord,curve,outline,heights,shape,roof,bounds)
        expected.append(original(*args));kinds.append('corner'if corner else 'door'if door else ['no_roof','flat','pitched','gable0','gable1','flat_threshold'][roof_kind])
        rows.append(f'{kind} {int(door)} {int(corner)} '+text_values(coord)+' '+text_values([curve[2]])+' '+text_values([x for p in curve[0]for x in p])+' '+text_values(curve[1])+' '+text_values([x for p in outline for x in p])+' '+text_values(heights)+f' {shape} '+('none'if roof is None else roof.hex())+' '+text_values(bounds))
    bindings=[]
    curve=make_curve([[-4.,-2.],[-4.,2.],[4.,2.],[4.,-2.],[-4.,-2.]])
    outline=[[p[0],0.,p[1]]for p in curve[0]]
    for kind in ['cw','gw']:
        for rank in [1,2,3]:
            bounds=actual_bounds[kind,rank];lo,hi=bounds
            for y in [0.,-.1,1.1,6.25]:
                y=f(y);top=f(f(f(hi-lo)+f(f(lo+hi)*.5))*.5+y)
                for top_bits in [bits(top)-1,bits(top),bits(top)+1]:
                    wall_height=struct.unpack('<f',struct.pack('<I',top_bits))[0]
                    coord=[f(curve[2]*.37),y];heights=[wall_height]*5
                    expected.append(original(kind,False,False,coord,curve,outline,heights,1,None,bounds));kinds.append('half_strict_comparison_boundary')
                    rows.append(f'{kind} 0 0 '+text_values(coord)+' '+text_values([curve[2]])+' '+text_values([x for p in curve[0]for x in p])+' '+text_values(curve[1])+' '+text_values([x for p in outline for x in p])+' '+text_values(heights)+' 1 none '+text_values(bounds))
    for kind,subtype in [('cw',1),('gw',7)]:
        for rank in [1,2,3]:
            name=original_half_asset_name(subtype,rank);bindings.append({'subtype':subtype,'rank':rank,'original_lookup_name':name})
            rows.append(f'asset {subtype} {rank}');expected.append(name);kinds.append('half_asset_name_selection')
            path=assets_root.parent/(name+'.json');assets[str(path)]=hashlib.sha256(path.read_bytes()).hexdigest()
            vertices=json.loads(path.read_text())['Vertex_Position']['buffer'];encoded_vertices=text_values([x for p in vertices for x in p])
            bbx=original_bbox(vertices);collision=[struct.unpack('<f',struct.pack('<I',int(v,16)))[0]for v in bbx.split(',')]
            curve=make_curve([[-4.,-2.],[-4.,2.],[4.,2.],[4.,-2.],[-4.,-2.]])
            outline=[[p[0],0.,p[1]]for p in curve[0]];heights=[1.5]*5
            for shape in [0,1]:
                for ratio in [0.,.25,.5,.75]:
                    coord=[f(curve[2]*ratio),4.];bounds=actual_bounds[kind,rank]
                    for roof in [None,make_roof(.8,.6,0)]:
                        derived=original(kind,False,False,coord,curve,outline,heights,shape,roof,bounds);assert derived==str(subtype)
                        holes=original_holes(kind,rank,shape,[*coord,*collision[2:]])
                        rows.append(f'{kind} 0 0 '+text_values(coord)+' '+text_values([curve[2]])+' '+text_values([x for p in curve[0]for x in p])+' '+text_values(curve[1])+' '+text_values([x for p in outline for x in p])+' '+text_values(heights)+f' {shape} '+('none'if roof is None else roof.hex())+' '+text_values(bounds)+' '+encoded_vertices+f' {rank}')
                        expected.append(derived+'|'+holes);kinds.append('actual_half_asset_to_holes')
    probe=ROOT/'reconstruction/target/release/decorator-subtype-probe';actual=subprocess.run([str(probe)],input='\n'.join(rows)+'\n',capture_output=True,text=True,check=True).stdout.splitlines()
    assert len(actual)==len(expected)
    failures=[{'case':i,'kind':kinds[i],'input':rows[i],'original':b,'rust':a}for i,(a,b)in enumerate(zip(actual,expected))if a!=b]
    paths=['reconstruction/building/decorator_subtype.rs','reconstruction/building/decorator_placement.rs','reconstruction/src/bin/decorator-subtype-probe.rs','reconstruction/roof/roof_state.rs','reconstruction/roof/surface.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock']
    report={'ok':not failures,'cases':len(rows),'case_counts':{k:kinds.count(k)for k in sorted(set(kinds))},'failure_count':len(failures),'failures':failures[:20],'source_sha256':baseline['binary']['sha256'],'seed':20261014,'asset_sha256':assets,'candidate_sources':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in paths},'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'entries':{'derive_decorator_subtype':'140813d00','is_half_dormer':'140814470','mapped_outline_XZ_segment':'14126a450','closest_roof_rectangle_edge':'140c9a9d0'},'bridges':['Successful original PublicWalls lookup returns supplied actual publication','Actual optional Roof component access bypasses Bevy query machinery; original roof shape and branch consumers execute','std collision HashMap lookup supplies actual plain-window collision record; original interpolation/comparison executes'],'scope':'B complete finite wall destination subtype consumer with actual source Curve2, published mapped Curve3, arch heights and optional real Roof. Corner/door precedence and half/plain selection execute in original instructions. Gothic/cottage actual asset vertical bounds used.','excluded':['Non-wall roof-UV destinations and roof trigger construction','Original storage iteration and Bevy change/removal traversal','GPU drawable updates','A/C runtime behavior','Invalid/nonfinite input panic/unwind identity']}
    report['half_asset_bindings']=bindings
    report['scope']+=' Original half rank-to-asset-name dispatch and 96 composed subtype + actual half mesh bbox + original raw-hole producer cases; subtype acceptance is saved as original CW/GW machine-code evidence.'
    out=ROOT/'evidence/building-placement/B-subtype-verification.json';out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in ['ok','cases','case_counts','failure_count']}))
    if failures:print(json.dumps(failures[:4]));raise SystemExit(1)
if __name__=='__main__':main()
