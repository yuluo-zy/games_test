"""Read-only shipped resource baseline. No target process execution or modification."""
from pathlib import Path
import collections, csv, hashlib, json, re, tomllib

ROOT = Path('D:/game/ljxsj_92385/Tiny Glade')
OUT = Path('D:/game/reverse-engineering/tiny-glade/evidence/resources')
PRIOR = Path('D:/game/TinyGlade_逆向初查/initial-analysis.json')
OUT.mkdir(parents=True, exist_ok=True)
prior = json.loads(PRIOR.read_text(encoding='utf-8-sig'))
files = sorted(p for p in ROOT.rglob('*') if p.is_file())
def rel(p): return p.relative_to(ROOT).as_posix()
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def shape(v):
    if isinstance(v, dict): return {'type':'object','keys':list(v)}
    if isinstance(v, list):
        return {'type':'array','count':len(v),'first_item':shape(v[0]) if v else None}
    return {'type':type(v).__name__}
inventory = {}
for folder in ['assets','assets-src','compiled-assets','compiled-shaders','build-info']:
    group = [p for p in files if rel(p).startswith(folder+'/')]
    inventory[folder] = {'file_count':len(group),'total_bytes':sum(p.stat().st_size for p in group),
                         'extensions':dict(collections.Counter(p.suffix for p in group))}
    if folder == 'assets':
        inventory[folder]['subdirectories'] = {name: {'file_count':len(ps),'total_bytes':sum(p.stat().st_size for p in ps)}
            for name,ps in ((name,[p for p in group if p.relative_to(ROOT).parts[1]==name])
                            for name in sorted({p.relative_to(ROOT).parts[1] for p in group}))}
with (OUT/'file-inventory.tsv').open('w',encoding='utf-8',newline='') as f:
    w=csv.writer(f,delimiter='\t'); w.writerow(['relative_path','bytes','extension'])
    w.writerows((rel(p),p.stat().st_size,p.suffix) for p in files)
schemas=[]; failures=[]; mesh_count=0
for p in files:
    if p.suffix != '.json' or not rel(p).startswith('assets/'): continue
    try: v=json.loads(p.read_text(encoding='utf-8-sig'))
    except Exception as ex: failures.append({'path':rel(p),'error':str(ex)}); continue
    rec={'path':rel(p), **shape(v)}
    if isinstance(v,dict): rec['fields']={k:shape(x) for k,x in v.items()}
    if rel(p).startswith('assets/meshes/') and isinstance(v,dict) and 'attributes' in v:
        mesh_count+=1
        rec['attributes']=v['attributes']
        rec['buffers']={k:{'type':x.get('type'),'count':len(x.get('buffer',[]))}
                       for k,x in v.items() if isinstance(x,dict) and 'buffer' in x}
        pos=v.get('Vertex_Position',{}).get('buffer',[])
        if pos and isinstance(pos[0],list):
            rec['position_bounds']={'min':[min(x[i] for x in pos) for i in range(3)],
                                    'max':[max(x[i] for x in pos) for i in range(3)]}
    schemas.append(rec)
(OUT/'json-schemas.json').write_text(json.dumps({'parsed':len(schemas),'failed':failures,'mesh_count':mesh_count,'files':schemas},ensure_ascii=False,indent=2),encoding='utf-8')
lock=tomllib.loads((ROOT/'build-info/Cargo.lock').read_text())
pkgs=lock['package']; credits=set((ROOT/'credits-crates.txt').read_text().splitlines())
wanted={'country-core','country','tiny-glade','bevy_ecs','bevy_app','bevy_tasks','ash','ash-window','rhapsody','motomoto','shader-compiler','shader-pipeline','shader-variants','asset','asset-pipe','libfmod','rapier3d','parry3d','glam','egui','rkyv','ron','serde','wgpu','wgpu-types','naga','gltf','obvhs','fsr'}
deps=[{k:p[k] for k in ['name','version','source','dependencies'] if k in p} | {'listed_in_credits':p['name'] in credits}
      for p in pkgs if p['name'] in wanted or p['name'].startswith('system-')]
(OUT/'dependency-evidence.json').write_text(json.dumps({'source':'build-info/Cargo.lock','credits_source':'credits-crates.txt','packages':deps,'limitations':'Lock/credits entries establish listed dependencies; runtime use and algorithm responsibility require symbols/call sites.'},ensure_ascii=False,indent=2),encoding='utf-8')
sample_paths=['assets-src/textures/roof/assets.ron','assets-src/textures/wood_detail/assets.ron',
 'assets/prefabs/clearing/tree/default.ron','assets/glade/summer/trees.json','assets/glade/summer/plants.json',
 'assets/glade/summer/settings.json','assets/glade/summer/textures.json','assets/glade/summer/grading.json',
 'assets/tod/default.json','assets/data/rocky_terrain.json','assets/meshes/roof_tile.json',
 'assets/meshes/roof_tile_lod1.json','assets/meshes/brick.json','assets/meshes/window_cottage_1x1.json',
 'assets/meshes/decorators/door.json','assets/meshes/path_pebble.json','compiled-assets/textures/list.ron',
 'build-info/Cargo.lock','build-info/toolchain-info.json']
samples={}
by_path={r['path']:r for r in schemas}
for s in sample_paths:
    p=ROOT/s
    if not p.exists(): continue
    samples[s]={'bytes':p.stat().st_size,'sha256':digest(p)}
    if s in by_path: samples[s]['schema']=by_path[s]
    elif p.suffix=='.ron' and p.stat().st_size<3000: samples[s]['text']=p.read_text()
    if s=='assets/tod/default.json':
        v=json.loads(p.read_text()); samples[s]['first_entry']=v['entries'][0]; samples[s]['entry_count']=len(v['entries'])
    if s=='assets/glade/summer/trees.json' or s=='assets/glade/summer/plants.json':
        samples[s]['data']=json.loads(p.read_text())
signatures=[]
for ext in ['.texture','.snapshot','.bank','.glb']:
    candidates=[p for p in files if p.suffix==ext]
    for p in candidates[:2]:
        with p.open('rb') as f: head=f.read(48)
        signatures.append({'path':rel(p),'bytes':p.stat().st_size,'first48_hex':head.hex(),'format_status':'unparsed header; extension alone is not proof' if ext!='.glb' else 'glTF binary magic candidate; no runtime inference'})
summary={
 'evidence_id':'TG-RES-BASELINE-20261010','target':str(ROOT),'target_execution':False,
 'search_boundary':'Complete on-disk recursive file inventory; all assets/*.json parsed; representative text configuration samples; shader format reused from existing static evidence.',
 'prior_evidence':{'path':str(PRIOR),'sha256':digest(PRIOR),'reused_sections':['E05_resource_inventory','E06_json_structure_samples','E07_shader_format']},
 'inventory':inventory,'inventory_matches_prior':{k:inventory[k]['file_count']==v['file_count'] and inventory[k]['total_bytes']==v['total_bytes'] for k,v in prior['E05_resource_inventory'].items()},
 'source_availability':{'assets_src_extensions':inventory['assets-src']['extensions'],
   'rust_source_files':[rel(p) for p in files if p.suffix=='.rs'],
   'shader_source_files':[rel(p) for p in files if p.suffix.lower() in {'.hlsl','.glsl','.wgsl','.vert','.frag','.comp'}],
   'conclusion':'assets-src consists of 16 texture build descriptions; this supplied directory is a shipped distribution with symbols/build metadata, not a complete Rust/HLSL source repository.'},
 'json_parsing':{'parsed_file_count':len(schemas),'parse_failures':failures,'mesh_attribute_schema_count':mesh_count},
 'shader_format_reused':{k:v for k,v in prior['E07_shader_format'].items() if k not in {'samples','feature_samples'}},
 'shader_feature_samples_reused':prior['E07_shader_format'].get('feature_samples',[]),
 'toolchain_direct_record':json.loads((ROOT/'build-info/toolchain-info.json').read_text()),
 'samples':samples,'binary_format_candidates':signatures,
 'current_scope':'Only original Tiny Glade implementation reconstruction; no other-engine/project mapping.',
 'history_reconstruction':'../../reconstruction/history/observed-history-schema.json',
 'findings':'findings.md',
 'unknowns':['Compiled texture binary container/format and compression metadata are not decoded.','Snapshot serialization, schema version, and object graph are not decoded.','No behavior, update ordering, attachment thresholds, generation equations, or transition timing can be concluded from resource names alone.','Prior SPIR-V parsing covers first embedded module boundaries, not all stages/full semantic validation/GPU execution.'],
 'artifacts':['file-inventory.tsv','json-schemas.json','dependency-evidence.json','resource-analysis-questions.json','command-schemas.json']
}
(OUT/'resources-summary.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'inventory':inventory,'parsed_jsons':len(schemas),'mesh_count':mesh_count,'failures':failures,'dependency_names':[p['name'] for p in deps]},ensure_ascii=False))
