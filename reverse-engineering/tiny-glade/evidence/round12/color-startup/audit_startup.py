"""Read-only original PE/source/packaged-asset audit. Does not run the game."""
from pathlib import Path
import hashlib, json, struct, subprocess
import re

ROOT = Path(__file__).resolve().parents[5]
OUT = Path(__file__).resolve().parent
INSTALL = ROOT / 'ljxsj_92385/Tiny Glade'
EXE = ROOT / 'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw = EXE.read_bytes()
assert hashlib.sha256(raw).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', raw, 0x3c)[0]
opt = pe + 24
base = struct.unpack_from('<Q', raw, opt + 24)[0]
section_header = opt + struct.unpack_from('<H', raw, pe + 20)[0]
sections = []
for i in range(struct.unpack_from('<H', raw, pe + 6)[0]):
    off = section_header + i * 40
    size, rva, raw_size, ptr = struct.unpack_from('<IIII', raw, off + 8)
    sections.append((raw[off:off+8].rstrip(b'\0').decode(), rva, raw_size, ptr))
def read(va, size):
    for _, rva, length, ptr in sections:
        if rva <= va-base and va-base+size <= rva+length:
            start = ptr + va-base-rva
            return raw[start:start+size]
    raise ValueError(hex(va), size)

targets = {
    'ImageFormationConfigs_default': (0x1408ae7f0, 607),
    'load_glade_grading': (0x1408bf9c0, 2346),
    'GCC_default': (0x1408c5600, 82),
    'GradingLuts_load': (0x140b01df0, None),
}
procedures=[]
procs=ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
with procs.open(encoding='utf8') as f:
    for line in f:
        parts=line.rstrip('\n').split('\t',4)
        if len(parts)==5:
            procedures.append((int(parts[0],16),int(parts[1]),parts[4]))
for name,(va,size) in list(targets.items()):
    exact=[row for row in procedures if row[0]==va]
    if size is None:
        assert exact
        size=exact[0][1]
        targets[name]=(va,size)
calls=[]
for secname,rva,size,ptr in sections:
    if secname!='.text':continue
    data=raw[ptr:ptr+size]
    for i in range(len(data)-4):
        if data[i]!=0xe8:continue
        site=base+rva+i
        dest=site+5+struct.unpack_from('<i',data,i+1)[0]
        for name,(target,_) in targets.items():
            if dest==target:
                owners=[row for row in procedures if row[0]<=site<row[0]+row[1]]
                calls.append({'target':name,'call_va':hex(site),'bytes':data[i:i+5].hex(),
                    'pdb_owners':[{'va':hex(a),'size':b,'name':c} for a,b,c in owners]})

assets=[]
for path in sorted((INSTALL/'assets/luts').iterdir()):
    blob=path.read_bytes()
    row={'name':path.stem,'file':path.name,'size':len(blob),'sha256':hashlib.sha256(blob).hexdigest()}
    if path.suffix=='.png':
        assert blob[:8]==b'\x89PNG\r\n\x1a\n'
        row['png_extent']=list(struct.unpack_from('>II',blob,16))
        row['rgba8_texel_bytes']=row['png_extent'][0]*row['png_extent'][1]*4
        row['expected_native_16_cubed_texel_bytes']=16**3*4
        assert row['rgba8_texel_bytes']==16**3*4
    assets.append(row)
themes=[]
for path in sorted((INSTALL/'assets/glade').glob('*/grading.json')):
    config=json.loads(path.read_text(encoding='utf8'))
    assert config['current_grading_lut'] in [a['name'] for a in assets]
    themes.append({'theme':path.parent.name,'grading':config,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
source_names=[
 'tiny-glade/crates/tiny-glade/src/gameplay_startup.rs',
 'tiny-glade/crates/tiny-glade/src/loading_graph.rs',
 'tiny-glade/crates/tiny-glade/src/run_game.rs',
 'tiny-glade/crates/tiny-glade/src/world_postprocess.rs',
 'tiny-glade/crates/tiny-glade/src/app_systems.rs',
 'tiny-glade/crates/country-core/src/startup/load_glade_grading.rs',
 'tiny-glade/crates/country-core/src/resources/grading_lut.rs',
 'tiny-glade/crates/country-core/src/resources/image_formation.rs',
 'tiny-glade/crates/country-core/src/resources/glade_color_customization.rs',
 'tiny-glade/crates/country-core/src/systems/save/load_game.rs',
]
report={'original_pe_sha256':hashlib.sha256(raw).hexdigest(),
 'native_ranges':{name:{'va':hex(va),'size':size,'sha256':hashlib.sha256(read(va,size)).hexdigest()} for name,(va,size) in targets.items()},
 'rel32_call_candidates':calls,'call_scope':'Raw rel32 scan; each reported site must be confirmed at instruction boundary with LLVM before being treated as a call.',
 'packaged_luts':assets,'theme_grades':themes,
 'source_snapshot_sha256':{name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in source_names},
 'execution_scope':'No game/GPU/Rust execution. Existing original execution GCC fixtures and prior grading byte verification are separate evidence.'}
(OUT/'startup-color-audit.json').write_text(json.dumps(report,indent=2,ensure_ascii=False),encoding='utf8')
for name,(va,size) in targets.items():
    if name not in ['load_glade_grading','ImageFormationConfigs_default']:continue
    result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={hex(va)}',f'--stop-address={hex(va+size)}',str(EXE)],capture_output=True,check=True)
    (OUT/(name+'-original.asm')).write_bytes(result.stdout)
for call in calls:
    assert len(call['pdb_owners'])==1
    owner=call['pdb_owners'][0]
    if owner['name']=='tiny_glade::run_game::run_game_thread':
        asm_path=ROOT/'reverse-engineering/tiny-glade/skeleton/task-index/work/TG-c459c7f73345e24709/main-chain-boot-20261018/run-game-thread-rne.asm'
    else:
        asm_path=OUT/('owner-'+owner['va']+'-original.asm')
        start=int(owner['va'],16)
        result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={hex(start)}',f'--stop-address={hex(start+owner["size"])}',str(EXE)],capture_output=True,check=True)
        asm_path.write_bytes(result.stdout)
    lines=asm_path.read_text(encoding='utf8').splitlines()
    matched=[s for s in lines if s.lstrip().startswith(call['call_va'][2:]+':')]
    assert len(matched)==1 and re.search(r'\bcallq?\s+0x'+format(targets[call['target']][0],'x')+r'\b',matched[0])
    call['confirmed_llvm_instruction']=matched[0]
    call['owner_disassembly']=str(asm_path.relative_to(ROOT)).replace('\\','/')
report['call_scope']='Every rel32 candidate confirmed as an actual call instruction in complete PDB-owned LLVM disassembly.'
(OUT/'startup-color-audit.json').write_text(json.dumps(report,indent=2,ensure_ascii=False),encoding='utf8')
print(json.dumps({'luts':len(assets),'themes':len(themes),'calls':calls,'native_ranges':report['native_ranges']},ensure_ascii=False))
