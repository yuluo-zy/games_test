"""Independent fresh-byte review of four runtime candidates (not author models)."""
from pathlib import Path
import json,hashlib,struct
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import *
HERE=Path(__file__).resolve().parent
OUT=HERE.parent/'independent-ui-camera';OUT.mkdir(exist_ok=True)
repo=HERE.parents[4]
raw=(repo/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
def ins(a,n):return list(md.disasm(image[a-base:a-base+n],a))
imports={entry.address:(dll.dll.decode(),entry.name.decode() if entry.name else None) for dll in pe.DIRECTORY_ENTRY_IMPORT for entry in dll.imports}
math=[]
for a,expected in [(0x142923fd0,'sinf'),(0x142923eb0,'exp2f'),(0x142924060,'truncf')]:
    i=ins(a,6)[0];assert i.mnemonic=='jmp' and i.operands[0].type==X86_OP_MEM and i.operands[0].mem.base==X86_REG_RIP
    addr=i.address+i.size+i.operands[0].mem.disp
    assert imports[addr][1]==expected,(hex(a),imports[addr])
    math.append({'thunk':hex(a),'iat':hex(addr),'dll':imports[addr][0],'name':expected,'original_6bytes':i.bytes.hex()})
table=0x142a9f3b0
jumps=[table+x for x in struct.unpack('<5i',image[table-base:table-base+20])]
assert jumps==[0x1408b2ea0,0x1408b2e07,0x1408b2e90,0x1408b2e07,0x1408b2e07]
phase2=ins(0x1408b2e90,16)
assert any(i.mnemonic=='ucomiss' for i in phase2)
assert phase2[-1].mnemonic=='jb' # unordered/positive timer rejects trigger
ranges=[('human',0x1408b2b30,2516),('ui2d',0x14092a100,1265),('mesh',0x140a68430,1556),('replay',0x140b1b860,1959),('mesh_indexed_collect',0x1412fd550,496)]
spans=[]
for name,a,n in ranges:
    body=image[a-base:a-base+n]
    spans.append({'name':name,'va':hex(a),'size':n,'sha256':hashlib.sha256(body).hexdigest()})
    (OUT/(name+'.fresh.asm')).write_text('\n'.join(f'{i.address:x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ins(a,n))+'\n',encoding='utf-8')
replay=ins(0x140b1b860,1959)
actual=[i.operands[0].imm for i in replay if i.mnemonic=='call' and i.operands[0].type==X86_OP_IMM]
assert actual.count(0x142923fd0)==2 and actual.count(0x142923eb0)==2 and actual.count(0x142924060)==1
mesh=ins(0x1412fd550,496)
assert any(i.mnemonic=='call' and i.op_str=='0x1409e9640' for i in mesh)
paths=[
'tiny-glade/crates/country-core/src/systems/debug/human_scale_ref.rs',
'tiny-glade/crates/country-core/src/systems/ui_2d_follow_mouse.rs',
'tiny-glade/crates/country-core/src/systems/mesh_follow_terrain.rs',
'tiny-glade/crates/country-core/src/systems/history/signifier_replay_active.rs',
'tiny-glade/crates/tiny-glade/src/run_game.rs',
'tiny-glade/crates/tiny-glade/src/audio_host.rs',
'tiny-glade/crates/country-core/src/systems/main_camera.rs',
'tiny-glade/crates/country-core/src/systems/camera_rig/main_camera.rs',
'tiny-glade/crates/country-core/src/systems/camera_rig/free_camera.rs',
'tiny-glade/crates/country-core/src/systems/camera_rig/walk.rs',
'tiny-glade/crates/country-core/src/systems/backend/photomode.rs',
'tiny-glade/crates/systems/toolbar/src/photomode.rs']
(OUT/'independent-original-evidence.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'method':'independent PE import parsing and original Capstone CFG; not author fixture/model execution','math_imports':math,'human_phase_jump_targets':[hex(x) for x in jumps],'replay_math_call_counts':{'sinf':2,'exp2f':2,'truncf':1},'ranges':spans,'reviewed_source_sha256':{p:hashlib.sha256((repo/p).read_bytes()).hexdigest() for p in paths}},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'independent_imports':3,'phase_jump_entries':5,'full_original_spans':5,'source_files_read':len(paths),'static_only':True}))
