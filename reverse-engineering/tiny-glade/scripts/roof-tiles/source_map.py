from pathlib import Path
import json,subprocess,re
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles';pdb='D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb';llvm='C:/Program Files/LLVM/bin/llvm-pdbutil.exe'
records=json.loads((O/'procedures.json').read_text(encoding='utf-8'));modules={}
for mod in sorted({x['module'] for x in records}):
 text=subprocess.run([llvm,'dump','--modules','--files',f'--modi={mod}',pdb],capture_output=True,text=True,encoding='utf-8',check=True).stdout
 (O/f'module{mod}-files.txt').write_text(text,encoding='utf-8')
 modules[mod]=[dict(source_sha256=m[1].lower(),original_path=m[2]) for l in text.splitlines() if (m:=re.search(r'\(SHA-256: ([0-9A-F]+)\) (.*)',l))]
mapping=[]
for x in records:
 name=x['name'];files=modules[x['module']]
 keys=[]
 if 'assemble_circular_roof' in name:keys=['/crates/systems/roof/src/utils/assemble_circular_roof.rs']
 elif 'assemble_rectangular_roof' in name:keys=['/crates/systems/roof/src/utils/assemble_rectangular_roof.rs']
 elif 'assemble_roof_tiles' in name:keys=['/crates/systems/roof/src/visual/assemble_roof_tiles.rs']
 elif 'RoofBoundsDims' in name or 'get_roof_expanded' in name:keys=['roof_visual_shape.rs']
 elif 'Roof::' in name:keys=['roof_shape.rs']
 elif 'random_splits' in name:keys=['/crates/utils/src/lib.rs']
 elif 'Rng::f32' in name:keys=['/fastrand-']
 elif 'Circle2d' in name:keys=['/crates/utils/src/geometry/circle.rs']
 elif 'Rectangle2d' in name:keys=['/crates/utils/src/geometry/rectangle.rs']
 elif 'Curve::' in name:keys=['/crates/utils/src/curve/']
 elif 'SparseInstanceBufferWriter' in name:keys=['sparse_instance_buffer.rs']
 candidates=[f for f in files if any(k in f['original_path'] for k in keys)]
 mapping.append(dict(**x,source_candidates=candidates,source_attribution='namespace+matching module source-file metadata; not a recovered source body or line map'))
result=dict(reconstructed_source='reconstruction/roof/tiles.rs',procedures=mapping,known_layouts={
 'RoofObserved':dict(bytes=88,fields=[dict(offset=0,kind='u32',meaning='roof_id carried to instance'),dict(offset=8,kind='u32',meaning='shape enum; bit0 drives assembly'),dict(offset='0x0c..0x24',kind='shape payload',meaning='circle frame/radius/rotation or rectangle unit-basis/center/dimensions'),dict(offset='0x24..0x40',kind='RoofShapeParams 28 bytes',meaning='tip/profile/height/ridge/eave/direction'),dict(offset='0x54',kind='u8',meaning='gable assembly modifier flag; original field name unknown')],limitation='Observed representation; complete original Rust field names, padding and enum ABI not claimed'),
 'Rectangle2dObserved':dict(bytes=24,fields=['unit-basis cos f32@0','unit-basis sin f32@4','center x f32@8','center z f32@12','width/full dimension f32@16','depth/full dimension f32@20']),
 'RoofBoundsDimsObserved':dict(max_observed_bytes=20,fields=['discriminant u32@0','bottom dims/radius f32@4/8','rectangle top dims f32@12/16 or circle top radius f32@8'],limitation='Circle only writes first12 bytes, rectangle20; these are observed output stores, not an ABI size assertion'),
 'Curve2Observed':dict(bytes=56,fields=['points Vec capacity/pointer/length @0/8/16','points_u Vec capacity/pointer/length @24/32/40','cumulative length f32@48'],reference='../roof-surface/findings.md'),
 'TileRecord':dict(bytes=64,output='RoofTileRecord([u8;64])',limitation='All emitted bytes recovered; higher-level source struct spelling depends on retained layout evidence'),
 'TileRng':dict(bytes=8,kind='u64 state',original='fastrand::Rng::f32',math='state+=0xa0761d6478bd642f, xor/multiply fold, mantissa generation')},
api=['TileRng::next_f32','random_splits','circular_row_plan','circular_row_coordinates','circular_column_count','circular_boundaries','assemble_circular_tiles','assemble_circular_from_observed','expanded_bounds_observed','rectangular_context_from_observed','ObservedRectangle::{from_bytes,bytes,basis,corners}','half','record','matrix_quat'],scope='Rust source reconstruction and metadata index only; no rendering or scene replay work',limitations=['PDB records metadata/checksums/source paths; original Rust source text is not included','Generated Rust is behaviorally reconstructed code, not a bit-for-bit original source recovery'])
(O/'source-map.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf-8');print(json.dumps(dict(functions=len(mapping),source_mapped=sum(bool(x['source_candidates']) for x in mapping)),indent=2))
