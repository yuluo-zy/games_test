"""Retain original PE-only raster registration proof; no DB/settings mutation."""
from pathlib import Path
import hashlib,json,struct
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64

ROOT=Path(__file__).resolve().parents[4]
OUT=Path(__file__).resolve().parent
EXE=ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw=EXE.read_bytes(); sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
p.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_EXCEPTION']])
cs=Cs(CS_ARCH_X86,CS_MODE_64)
rows=[]
for name,va in [('parent_run_game_thread',0x14059d450),('upload_converter',0x1405c0400),
    ('run_without_applying',0x14045dc60),('run_unsafe',0x1405bef90),
    ('upload_leaf',0x140957bf0),('app_add_systems_tuple',0x140327490)]:
    e=next(e.struct for e in p.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress==va-base)
    data=p.get_data(e.BeginAddress,e.EndAddress-e.BeginAddress)
    output=OUT/(name+'.asm')
    output.write_text('\n'.join(f'{i.address:x}: {i.mnemonic} {i.op_str}'
        for i in cs.disasm(data,va)),encoding='utf-8')
    rows.append({'name':name,'address':hex(va),'size':len(data),'sha256':hashlib.sha256(data).hexdigest(),
        'range_basis':'original PE exception directory BeginAddress/EndAddress','assembly':output.name})
vtable=0x142a05448
system_vtable=[{'slot':hex(off),'value':hex(struct.unpack('<Q',p.get_data(vtable-base+off,8))[0])}
    for off in range(0,0xb8,8)]
set_vtable=0x14292af98
set_slots=[{'slot':hex(off),'value':hex(struct.unpack('<Q',p.get_data(set_vtable-base+off,8))[0])}
    for off in range(0,0x40,8)]
lengths=0x14292b070;offsets=0x14292b1d8
labels=[]
for tag in range(45):
    length=struct.unpack('<Q',p.get_data(lengths-base+tag*8,8))[0]
    off=struct.unpack('<i',p.get_data(offsets-base+tag*4,4))[0]
    labels.append({'byte':tag,'name':p.get_data(offsets+off-base,length).decode('utf-8')})
assert labels[14]['name']=='HistoryStart'
manifest={'original_image':str(EXE),'sha256':sha,'ranges':rows,
    'system_vtable':{'address':hex(vtable),'slots':system_vtable},
    'set_vtable':{'address':hex(set_vtable),'slots':set_slots},
    'label_decode':{'fmt':'140037760','lengths':hex(lengths),'relative_offsets':hex(offsets),'labels':labels},
    'pdb_identity_source':{'parent':'skeleton/B/crates/tiny-glade/src/run_game.rs:2450 run_game_thread; module240/71628',
        'set':'skeleton/B/crates/country-core/src/systems/mod.rs:47 dyn_clone140a90e20, :158 fmt140037760, named GameSystemSet'},
    'proved_edges':['parent14059fc85 DL14 -> call14059fc87 converter1405c0400',
        'converter1405c056e -> system vtable142a05448 -> run1405bef90/14045dc60 -> leaf140957bf0',
        'converter1405c0584 stores incoming14;1405c058e interns set vtable14292af98;1405c05ab adds that set to returned config',
        'same set vtable14292af98 used for knownGameSystemSet22 in parent14059fb68..14059fb96',
        'parent14059ff83 passes tuple including rbX+440 upload config to140327490'],
    'remaining_boundary':'GameSystemSet::HistoryStart14 is directly proved. Direct Update schedule identity for tuple140327490 is under independent review; do not infer it solely from current host.'}
(OUT/'raster-registration-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
print(json.dumps({'manifest':str(OUT/'raster-registration-manifest.json'),'set':labels[14],'ranges':len(rows)}))
