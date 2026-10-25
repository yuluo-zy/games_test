"""B actual Mesh bounding-box/collision projection and WallSpace conversion.
The only bbox bridge supplies successful Vertex_Position attribute lookup;
original private reduction/extent/center and all Curve arithmetic execute.
"""
import hashlib,json,math,random,struct,subprocess
from pathlib import Path
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,setup,bits
from verify_building_state import reset
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
from verify_opening_rules import original as original_holes
active=False
def hook(c,pc,size,user):
    if active and pc==0x140a0788e:
        # After the original function prologue and before std string hash/probe.
        # Select a real float3 input slot, not precomputed AABB or dimensions.
        c.reg_write(UC_X86_REG_RCX,DATA+0x7000)
        c.reg_write(UC_X86_REG_RBX,0)
        c.reg_write(UC_X86_REG_RIP,0x140a07960)
cpu.hook_add(UC_HOOK_CODE,hook)
def pack_floats(values):return struct.pack('<'+'f'*len(values),*values)
def f(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def text_values(values):return ','.join(f'{bits(x):08x}' for x in values)
def original_bbox(points):
    global active
    reset();setup();cpu.mem_write(DATA,bytes(0x8000))
    cpu.mem_write(DATA+0x2000+0x50,struct.pack('<Q',1))
    cpu.mem_write(DATA+0x6fe0,struct.pack('<I',3))
    cpu.mem_write(DATA+0x6ff0,struct.pack('<QQ',DATA+0x3000,len(points)))
    cpu.mem_write(DATA+0x3000,pack_floats([x for p in points for x in p]))
    cpu.reg_write(UC_X86_REG_RCX,DATA+0x1000);cpu.reg_write(UC_X86_REG_RDX,DATA+0x2000)
    active=True
    try:cpu.emu_start(0x140a07a70,STACK,count=100000)
    finally:active=False
    assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    raw=bytes(cpu.mem_read(DATA+0x1000,32));assert raw[:8]==struct.pack('<2f',1.,0.)
    return ','.join(f'{x:08x}' for x in struct.unpack('<6I',raw[8:]))
def make_curve(points):
    points=[tuple(map(f,p)) for p in points];distance=0.;us=[0.]
    for a,b in zip(points,points[1:]):
        dx=f(b[0]-a[0]);dy=f(b[1]-a[1]);distance=f(distance+f(math.sqrt(f(f(dy*dy)+f(dx*dx)))))
        us.append(distance)
    return points,[f(x/distance) for x in us],distance

def original_coord(curve,values,op):
    points,us,length=curve;reset();setup();cpu.mem_write(DATA,bytes(0x8000))
    cpu.mem_write(DATA+0x2000,struct.pack('<QQQQQQf',len(points),DATA+0x3000,len(points),len(us),DATA+0x4000,len(us),length))
    cpu.mem_write(DATA+0x3000,pack_floats([x for p in points for x in p]))
    cpu.mem_write(DATA+0x4000,pack_floats(us));cpu.mem_write(DATA+0x1000,pack_floats(values))
    if op=='from':
        cpu.reg_write(UC_X86_REG_RCX,DATA+0x1000);cpu.reg_write(UC_X86_REG_RDX,DATA+0x2000)
        cpu.emu_start(0x140c91fb0,STACK,count=100000)
        assert cpu.reg_read(UC_X86_REG_RIP)==STACK
        return f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x},{cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff:08x}'
    cpu.reg_write(UC_X86_REG_RCX,DATA+0x5000);cpu.reg_write(UC_X86_REG_RDX,DATA+0x1000);cpu.reg_write(UC_X86_REG_R8,DATA+0x2000)
    cpu.emu_start(0x140c92090,STACK,count=100000);assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    return ','.join(f'{x:08x}' for x in struct.unpack('<3I',cpu.mem_read(DATA+0x5000,12)))
def original_key(subtype,rank):
    reset();setup();cpu.reg_write(UC_X86_REG_RCX,rank);cpu.reg_write(UC_X86_REG_RDX,subtype)
    cpu.emu_start(0x140b25c90,STACK,count=50)
    return f'{cpu.reg_read(UC_X86_REG_RAX)&255},{cpu.reg_read(UC_X86_REG_RDX)&255}'
def original_asset_name(subtype,rank):
    # B load_decorators inner rank dispatch starts after feature availability,
    # resource acquisition and native key construction. Stop at first lookup;
    # no name/path, collision size or lookup result is injected.
    start,lookup={0:(0x1407b84e2,0x1407bb4b0),4:(0x1407b7af2,0x1407bb4b0),6:(0x1407b726d,0x1407bb160),9:(0x1407b8c28,0x1407bb160)}[subtype]
    reset();setup();cpu.mem_write(DATA,bytes(0x8000))
    cpu.reg_write(UC_X86_REG_RBP,DATA+0x2000);cpu.reg_write(UC_X86_REG_RSI,DATA+0x4000)
    cpu.reg_write(UC_X86_REG_RDI,rank);cpu.mem_write(DATA+0x28b0,struct.pack('<Q',DATA+0x4000))
    cpu.emu_start(start,lookup,count=100)
    assert cpu.reg_read(UC_X86_REG_RIP)==lookup
    return bytes(cpu.mem_read(cpu.reg_read(UC_X86_REG_R8),cpu.reg_read(UC_X86_REG_R9))).decode()
def main():
    rng=random.Random(20261013);inputs=[];expected=[];kinds=[];assets={}
    asset_root=Path(baseline['binary']['path']).parent/'assets/meshes/decorators'
    names=[f'window_{family}_{size}x1_collision.json' for family in ['cottage','gothic'] for size in [1,2,3]]
    names += [f'balcony_door{family}_rank{size}_collision.json' for family in ['', '_gothic'] for size in [1,2,3]]
    for name in names:
        p=asset_root/name;d=json.loads(p.read_text());points=d['Vertex_Position']['buffer'];assets[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
        inputs.append('bbox '+text_values([x for point in points for x in point]));expected.append(original_bbox(points));kinds.append('actual_asset_bbox')
    for i in range(512):
        points=[[f(rng.uniform(-20,20)) for _ in range(3)] for _ in range(i%32+1)]
        if i<32:points[0][i%3]=f(-0. if i%2 else 0.)
        inputs.append('bbox '+text_values([x for point in points for x in point]));expected.append(original_bbox(points));kinds.append('bbox')
    for i in range(512):
        angle=rng.uniform(-math.pi,math.pi);s=math.sin(angle);c=math.cos(angle);w=rng.uniform(.5,15);h=rng.uniform(.5,15);cx=rng.uniform(-30,30);cz=rng.uniform(-30,30)
        if i%2:
            local=[[-w,-h],[w,-h],[w,h],[-w,h],[-w,-h]]
        else:
            local=[[math.cos(j*math.tau/16)*w,math.sin(j*math.tau/16)*w] for j in range(16)];local.append(local[0])
        points=[[cx+x*c-z*s,cz+x*s+z*c] for x,z in local];curve=make_curve(points)
        point=[f(rng.uniform(cx-30,cx+30)),f(rng.uniform(-10,20)),f(rng.uniform(cz-30,cz+30))]
        header=f'{bits(curve[2]):08x} '+text_values([x for p in curve[0] for x in p])+' '+text_values(curve[1])
        inputs.append('from '+header+' '+text_values(point));expected.append(original_coord(curve,point,'from'));kinds.append('from_world')
        distance=f(curve[2]*[-.25,0.,.1,.5,1.,1.25][i%6]);coord=[distance,point[1]]
        inputs.append('to '+header+' '+text_values(coord));expected.append(original_coord(curve,coord,'to'));kinds.append('to_world')
    for family,subtype in [('cw',0),('gw',6),('cd',4),('gd',9)]:
        for rank in [1,2,3]:
            inputs.append(f'key {family} {rank}');expected.append(original_key(subtype,rank));kinds.append('key_bytes')
            inputs.append(f'asset {family} {rank}');expected.append(original_asset_name(subtype,rank));kinds.append('asset_name_selection')
    for family,subtype,prefix in [('cw',0,'window_cottage_{}x1_collision.json'),('gw',6,'window_gothic_{}x1_collision.json'),('cd',4,'balcony_door_rank{}_collision.json'),('gd',9,'balcony_door_gothic_rank{}_collision.json')]:
        for rank in [1,2,3]:
            vertices=json.loads((asset_root/prefix.format(rank)).read_text())['Vertex_Position']['buffer']
            bbox=[struct.unpack('<f',struct.pack('<I',int(x,16)))[0] for x in original_bbox(vertices).split(',')]
            for shape in [0,1]:
                points=([[4.,0.],[0.,4.],[-4.,0.],[0.,-4.],[4.,0.]] if shape==0 else [[0.,0.],[8.,0.],[8.,4.],[0.,4.],[0.,0.]])
                curve=make_curve(points);header=f'{bits(curve[2]):08x} '+text_values([x for p in curve[0] for x in p])+' '+text_values(curve[1])
                for point in [[.1,0.,0.],[4.,1.2,4.],[-3.,2.,-4.],[8.1,-1.,.01]]:
                    point=list(map(f,point));center_text=original_coord(curve,point,'from');center=[struct.unpack('<f',struct.pack('<I',int(x,16)))[0] for x in center_text.split(',')]
                    world_text=original_coord(curve,center,'to')
                    holes=original_holes(family,rank,shape,[*center,*bbox[2:]])
                    inputs.append(f'place {family} {rank} {shape} '+header+' '+text_values(point)+' '+text_values([x for p in vertices for x in p]))
                    expected.append(original_key(subtype,rank)+'|'+center_text+'|'+world_text+'|'+holes);kinds.append('asset_world_to_actual_holes')
    probe=ROOT/'reconstruction/target/release/decorator-placement-probe'
    actual=subprocess.run([str(probe)],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(expected)
    failures=[{'case':i,'kind':kinds[i],'input':inputs[i],'original':b,'rust':a} for i,(a,b) in enumerate(zip(actual,expected)) if a!=b]
    paths=['reconstruction/building/decorator_placement.rs','reconstruction/src/bin/decorator-placement-probe.rs','reconstruction/building/opening_rules.rs','reconstruction/roof/surface.rs','reconstruction/building/wall_state.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock']
    report={'ok':not failures,'cases':len(inputs),'case_counts':{kind:kinds.count(kind) for kind in sorted(set(kinds))},'failure_count':len(failures),'failures':failures[:20],
      'source_sha256':baseline['binary']['sha256'],'seed':20261013,'asset_sha256':assets,
      'candidate_sources':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths},'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
      'entries':{'collision_bbx':'140a07a70','aabb_bounds_reduction':'140a07960','from_world_space':'140c91fb0','get_approx_u_from_pos':'141940a40','to_world_space':'140c92090','DecoratorLibKey_new':'140b25c90','load_decorators':'1407b66e0','load_decorators_rank_dispatch':{'0':'1407b84e2','4':'1407b7af2','6':'1407b726d','9':'1407b8c28'}},
      'bridges':['Successful Vertex_Position float3 attribute slot; std string hash/probe skipped before original private AABB arithmetic','Curve2 points/normalized parameters/source length supplied as authoritative input; private nearest segment/search/interpolation execute'],
      'scope':'B original collision_bbx and private aabb reduction from raw vertices, including 12 actual assets; complete finite WallSpace from/to conversion over circle/rectangle source curves; original key byte order; original bounded rank-to-asset-name dispatch for 12 plain assets (availability and IO excluded); composed asset vertices + world position through original bbox/WallSpace to original four-family raw-hole production.',
      'excluded':['Complete DerivedDecoratorSubtypes roof/corner/half-dormer decisions','Original asset loader IO and serde internals','A/C dynamic behavior','Full original raycast/trigger mesh and UI command creation','Nonfinite/invalid curve panic and exception behavior','GPU visuals'],}
    out=ROOT/'evidence/building-placement/B-placement-verification.json';out.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:report[k] for k in ['ok','cases','case_counts','failure_count']}))
    if failures:print(json.dumps(failures[:3]));raise SystemExit(1)
if __name__=='__main__':main()
