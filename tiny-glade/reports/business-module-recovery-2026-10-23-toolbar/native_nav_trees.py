"""Run unchanged B navigation over nonempty native Bevy query storage.

Only allocator/TLS/memory and Taffy dependency access are bridged. Navigation,
query iteration/get, maps/sets, anchor traversal and nearest run original bytes.
Run with Python dependencies unicorn and pefile. No game process is launched.
"""
from pathlib import Path
import hashlib, json, struct, sys
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path(__file__).resolve().parents[2]
REPO = ROOT.parent
OLD = REPO / 'reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-focus/native_focus_world.py'
source = OLD.read_text().replace("ROOT=Path('D:/game/reverse-engineering/tiny-glade')", f"ROOT=Path({str(REPO / 'reverse-engineering/tiny-glade')!r})")
source = source.replace("ROOT/'workspace/input/tiny-glade.exe'", f"Path({str(REPO / 'tiny-glade.exe')!r})")
scope = {'__name__': 'reused_native_focus_world'}
exec(compile(source, str(OLD), 'exec'), scope)
Native = scope['Native']

def fixture(name, positions, children, anchors, current, direction, expected, visible=None, vertical=None):
    return dict(name=name, positions=positions, children=children, anchors=anchors,
                current=current, direction=direction, expected=expected,
                visible=visible or {}, vertical=vertical or [])

def run(case):
    n = Native([])
    # All fixtures use one real dense native table with four component columns.
    rows = [(int(node), pos) for node, pos in case['positions'].items()]
    table = 0x140000
    entities = n.alloc(8 * len(rows)); columns = n.alloc(0xc0); mapping = n.alloc(32)
    extent = n.alloc(16 * len(rows)); state = n.alloc(3 * len(rows))
    navs = n.alloc(64 * len(rows)); nodes = n.alloc(20 * len(rows))
    n.wq(table + 8, entities); n.wq(table + 16, len(rows)); n.wq(table + 24, columns)
    n.wq(table + 56, mapping); n.wq(table + 64, 4)
    for i, address in enumerate([extent, state, navs, nodes]):
        n.wq(mapping + 8*i, (-(i+1)) & ((1<<64)-1))
        n.wq(columns + 48*i + 16, address)
    n.wq(n.world + 0x10, len(rows) + 1)
    archetype = n.alloc(0xa0); component_presence = n.alloc(32)
    n.wq(n.world + 0x100, archetype); n.wq(n.world + 0x108, 1)
    n.wq(archetype + 0x80, component_presence); n.wq(archetype + 0x88, 4)
    for i in range(4): n.wq(component_presence + 8*i, 1)
    entity_by_node = {}
    for i, (node, pos) in enumerate(rows):
        entity = (1<<32) | i
        entity_by_node[node] = entity
        n.wq(entities + 8*i, entity)
        n.w(0x130000 + i*20, struct.pack('<IIIII', 1, 0, i, 0, i))
        n.w(extent + 16*i, struct.pack('<ffff', *pos, 10., 10.))
        n.w(state + 3*i, bytes([case['visible'].get(str(node), 1), 0, 0]))
        n.wq(nodes + 20*i, node)
    n.tables = [rows, [], []]
    first = n.query('all', [0, 1, 2]); second = n.query('all', [3, 2])
    layout = n.alloc(0xc0); layoutarg = n.alloc(32); n.wq(layoutarg, layout)
    anchor_data = n.alloc(40*len(case['anchors']))
    n.wq(layout + 0xa0, anchor_data); n.wq(layout + 0xa8, len(case['anchors']))
    for i, (node, parent, side) in enumerate(case['anchors']):
        n.wq(anchor_data + i*40 + 16, node)
        n.wq(anchor_data + i*40 + 24, parent)
        n.w(anchor_data + i*40 + 36, bytes([side]))
    tls = n.alloc(24); n.w(tls,struct.pack('<QQQ',1,1234,5678))
    def dependency_hook(u, address, size, unused):
        if address == 0x140fe7950:  # Taffy::children -> Result<Vec<Node>>
            out = u.reg_read(UC_X86_REG_RCX)
            node = u.reg_read(UC_X86_REG_R8) | (u.reg_read(UC_X86_REG_R9)<<32)
            values = case['children'].get(str(node), [])
            data = n.alloc(8*len(values))
            for i,v in enumerate(values): n.wq(data + 8*i, v)
            n.w(out, struct.pack('<QQQQ', 4, len(values), data, len(values)))
            n.ret(out)
        elif address == 0x141e4c650:  # Taffy::style -> &Style
            node = u.reg_read(UC_X86_REG_RDX) | (u.reg_read(UC_X86_REG_R8)<<32)
            result = n.alloc(0x160)
            n.w(result + 0x14b, bytes([int(node in case['vertical'])]))
            n.ret(result)
        elif address == 0x140fcfbe0:  # RandomState::new dependency
            u.reg_write(UC_X86_REG_RDX, 5678); n.ret(1234)
        elif address == 0x140d4d740:  # Windows TLS RandomState seed owner
            n.ret(tls)
    n.u.hook_add(UC_HOOK_CODE, dependency_hook)
    actions = n.alloc(320); actionarg = n.alloc(32); n.wq(actionarg, actions)
    for i in range(40): n.w(actions+8*i, struct.pack('<If', 4, 0.))
    n.w(actions+8*(20+case['direction']), struct.pack('<I', 0))
    navigation = n.alloc(56); navarg = n.alloc(32)
    n.w(navigation, struct.pack('<IIQIIQ', 0,0,0,1,0,entity_by_node[case['current']]))
    n.wq(navarg, navigation); n.wq(navarg+16, n.alloc(4)); n.w(navarg+28,struct.pack('<I',42))
    sp=0x2ffff8; n.wq(sp,0x100000); n.wq(sp+0x28,navarg); n.wq(sp+0x30,n.eventarg)
    for register,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,first),(UC_X86_REG_RDX,second),(UC_X86_REG_R8,actionarg),(UC_X86_REG_R9,layoutarg)]: n.u.reg_write(register,value)
    try:
        n.u.emu_start(0x14233eed0,0x100000,count=2000000)
        if n.u.reg_read(UC_X86_REG_RIP)!=0x100000: raise RuntimeError('instruction bound')
    except Exception as error:
        raise RuntimeError(f"{case['name']}: {error}; rip={n.u.reg_read(UC_X86_REG_RIP):x}; trace={[hex(a) for a in n.trace]}") from error
    raw=bytes(n.u.mem_read(navigation,32)); entity=struct.unpack_from('<Q',raw,24)[0]
    result=next((node for node,e in entity_by_node.items() if e==entity),None)
    return dict(case=case,native_selected=result,focus_tag=struct.unpack_from('<I',raw,16)[0],matched=result==case['expected'])

CASES = [
    fixture('sibling_root_priority', {'10':[0,0],'11':[100,0],'12':[10,0]}, {'1':[10,11,12]}, [[1,0,4]],10,1,11),
    fixture('hidden_candidate_is_eligible', {'10':[0,0],'11':[5,0]}, {'1':[10,11]}, [[1,0,4]],10,1,11,{'11':0}),
    fixture('subtree_nearest', {'10':[0,0],'11':[100,0],'12':[10,0]}, {'1':[10,2],'2':[11,12]}, [[1,0,4]],10,1,12),
    fixture('subtree_tie_keeps_first', {'10':[0,0],'11':[10,1],'12':[10,-1]}, {'1':[10,2],'2':[11,12]}, [[1,0,4]],10,1,11),
    fixture('left_reverses_siblings', {'10':[0,0],'11':[5,0],'12':[100,0]}, {'1':[10,11,12]}, [[1,0,4]],12,0,11),
    fixture('no_forward_candidate_keeps_current', {'10':[0,0],'11':[-1,0]}, {'1':[10,11]}, [[1,0,4]],10,1,10),
    fixture('enter_above_anchor', {'10':[0,0],'11':[0,-10]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,0]],10,3,11),
    fixture('return_from_above_anchor', {'10':[0,0],'11':[0,-10]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,0]],11,2,10),
    fixture('enter_below_anchor', {'10':[0,0],'11':[0,10]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,1]],10,2,11),
    fixture('enter_left_anchor', {'10':[0,0],'11':[-10,0]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,2]],10,0,11),
    fixture('enter_right_anchor', {'10':[0,0],'11':[10,0]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,3]],10,1,11),
    fixture('vertical_sibling', {'10':[0,0],'11':[0,10]}, {'1':[10,11]}, [[1,0,4]],10,2,11,vertical=[1]),
    fixture('return_from_below_anchor', {'10':[0,0],'11':[0,10]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,1]],11,3,10),
    fixture('return_from_left_anchor', {'10':[0,0],'11':[-10,0]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,2]],11,1,10),
    fixture('return_from_right_anchor', {'10':[0,0],'11':[10,0]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,3]],11,0,10),
    fixture('wrong_anchor_return_direction', {'10':[0,0],'11':[0,-10]}, {'1':[10],'2':[11]}, [[1,0,4],[2,10,0]],11,3,11),
    fixture('sibling_precedes_closer_anchor', {'10':[0,0],'11':[100,0],'12':[1,0]}, {'1':[10,11],'2':[12]}, [[1,0,4],[2,10,3]],10,1,11),
    fixture('nested_ascent_finds_outer_sibling', {'10':[0,0],'11':[5,0]}, {'1':[2,11],'2':[10]}, [[1,0,4]],10,1,11),
    fixture('vertical_parent_blocks_horizontal_sibling', {'10':[0,0],'11':[5,0]}, {'1':[10,11]}, [[1,0,4]],10,1,10,vertical=[1]),
    fixture('visited_anchor_cycle_terminates', {'10':[0,0]}, {'1':[10]}, [[1,10,4],[1,10,0]],10,0,10),
]

if __name__=='__main__':
    exe=REPO/'tiny-glade.exe'
    digest=hashlib.sha256(exe.read_bytes()).hexdigest()
    bodies={}
    for address,size,wanted in [(0x14233eed0,6017,'da693f822586b2665e73bc88ad68d96fbe9f1b0ac1cb669f14c3c76600840082'),(0x141ea8c60,1128,'83cc51bb90300bb80cb798f1f684f058daa66abe947755dfd7dd8c2261b3e3e9')]:
        actual=hashlib.sha256(scope['IM'][address-scope['IB']:address-scope['IB']+size]).hexdigest()
        assert actual==wanted,(hex(address),actual)
        bodies[hex(address)]=actual
    results=[run(case) for case in CASES]
    output={'exe_sha256':digest,'body_sha256':bodies,'scope':'unchanged native business; native Bevy query storage; bridged Taffy access/allocator/TLS/memory dependencies','results':results}
    Path(__file__).with_suffix('.json').write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps({'cases':len(results),'matched':sum(r['matched'] for r in results)}))
    if not all(r['matched'] for r in results): sys.exit(1)
