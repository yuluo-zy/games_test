"""Archive exact inputs, deduplicate modeling data, and decode legacy textures."""
from pathlib import Path
import collections, hashlib, io, json, re, shutil, struct
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
GAME = ROOT.parent
VERSIONS = {'A': GAME/'Tiny Glade', 'B': GAME/'ljxsj_92385/Tiny Glade',
            'C': GAME/'tiny.glade/XJ07036/Tiny Glade v1.16.0'}

def signature(d):
    core = {k:v for k,v in d.items() if isinstance(v, dict) and k != 'indices'}
    ix = d['indices']['buffer']
    core['faces'] = sorted(tuple(sorted(ix[i:i+3])) for i in range(0,len(ix),3)) if len(ix)%3 == 0 else ix
    return hashlib.sha256(json.dumps(core, sort_keys=True, separators=(',',':')).encode()).hexdigest()

def category(name):
    s = name.lower()
    if s.startswith('sheep_animation/'): return '08_AnimationFrames'
    if any(t in s for t in ['collision','outline','interaction','trigger','spawner','particles','flowerbed_locations']): return '09_Helpers'
    if s.startswith('terrain_rt/') or s in ['terrain','forest_terrain','terrain_rocks','water_plane','water_stones','olden_glade_rock','path_pebble','garden_stone']: return '05_Terrain'
    if s.startswith('decorators/') or s.startswith('wooden_gate/'): return '03_Architecture'
    if s.startswith('clutter/'): return '04_Props'
    if any(t in s for t in ['tree','branch','bush']): return '01_Vegetation'
    if any(t in s for t in ['flower','plant','clover','lavender','reed','grass','ivy','lilypad','petal','leaves','thistle']): return '02_Plants'
    if any(t in s for t in ['sheep','duck','bird','butterfly','fish']): return '06_Animals'
    if any(t in s for t in ['brick','roof_tile','plank','beam','stair','sewage_gate']): return '03_Architecture'
    return '07_Other'

def decode_texture(p, target):
    b = p.read_bytes()
    if len(b)<36 or b[:4] != bytes.fromhex('83215048'): raise ValueError('unknown header')
    length,w,h,depth,ml,fmt = struct.unpack('<6I',b[-24:])
    mips,layers = ml & 65535, ml >> 16
    if depth != 1: raise ValueError('volume texture: archived, not converted')
    bc = {0x292:98,0x392:98,0x291:98,0x28b:80}
    raw = fmt in [0x225,0x22b]
    if not raw and fmt not in bc: raise ValueError(f'unknown format {fmt:#x}')
    sizes=[]
    for mip in range(mips):
        mw,mh=max(1,w>>mip),max(1,h>>mip)
        sizes.append(mw*mh*4 if raw else ((mw+3)//4)*((mh+3)//4)*(8 if fmt==0x28b else 16))
    if sum(sizes)*layers != length or len(b) != length+36: raise ValueError('payload size mismatch')
    outputs=[]
    for layer in range(layers):
        data=b[8+layer*sum(sizes):8+layer*sum(sizes)+sizes[0]]
        if raw: im=Image.frombytes('RGBA',(w,h),data)
        else:
            header=struct.pack('<31I',124,0x81007,h,w,len(data),0,1,*([0]*11),32,4,
                               struct.unpack('<I',b'DX10')[0],0,0,0,0,0,0x1000,0,0,0,0)
            dx10=struct.pack('<5I',bc[fmt],3,0,1,0)
            im=Image.open(io.BytesIO(b'DDS '+header+dx10+data)).convert('RGBA')
        dest=target.with_name(target.stem+(f'__layer{layer:02d}' if layers>1 else '')+'.png')
        dest.parent.mkdir(parents=True,exist_ok=True);im.save(dest)
        outputs.append(str(dest.relative_to(ROOT)))
    return {'width':w,'height':h,'layers':layers,'mips':mips,'format':hex(fmt),'outputs':outputs,
            'interpretation':'RGBA8 or BC7/BC4 inferred from format code and verified payload dimensions; game shader semantics not recovered'}

def main():
    inventory={}; grouped=collections.defaultdict(list)
    for version,base in VERSIONS.items():
        entries=[]
        for p in sorted((base/'assets/meshes').rglob('*.json')):
            raw=p.read_bytes(); sha=hashlib.sha256(raw).hexdigest();out=ROOT/'source/mesh-blobs'/f'{sha}.json'
            if not out.exists(): out.write_bytes(raw)
            d=json.loads(raw);name=p.relative_to(base/'assets/meshes').as_posix()[:-5]
            record={'version':version,'name':name,'sha256':sha,'blob':str(out.relative_to(ROOT)),
                    'signature':signature(d),'vertices':len(d['Vertex_Position']['buffer']),
                    'indices':len(d['indices']['buffer']),'attributes':d.get('attributes',[]),
                    'double_sided':d.get('double_sided')}
            entries.append(record);grouped[name].append(record)
        inventory[version]=entries
        for sub in ['assets/prefabs','assets-src','assets/glade']:
            source=base/sub
            if source.exists(): shutil.copytree(source,ROOT/'source/configs'/version/sub,dirs_exist_ok=True)
        for file in ['assets/nani_meshes.ron','assets/colors.json']:
            source=base/file
            if source.exists():
                dst=ROOT/'source/configs'/version/file;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(source,dst)
    assets=[]
    for name,entries in sorted(grouped.items()):
        chosen=sorted(entries,key=lambda e:{'B':0,'C':1,'A':2}[e['version']])[0]
        sigs={chosen['signature']}; selections=[chosen]
        for e in entries:
            if e['signature'] not in sigs: selections.append(e);sigs.add(e['signature'])
        for i,e in enumerate(selections):
            rec=dict(e);rec['id']=name+(f'__variant_{e["version"]}' if i else '')
            rec['category']=category(name);rec['is_variant']=bool(i)
            rec['available_versions']=[x['version'] for x in entries if x['signature']==e['signature']]
            newer=next((x for x in entries if x['version']=='C'),None)
            rec['double_sided_hint']=newer['double_sided'] if newer else None
            assets.append(rec)
    textures={}; failures=[]
    # Named B textures are the main material source; A-only names complement them.
    for version in ['B','A']:
        for p in sorted((VERSIONS[version]/'compiled-assets/textures').rglob('*.texture')):
            name=p.relative_to(VERSIONS[version]/'compiled-assets/textures').as_posix()[:-8]
            if name in textures: continue
            dst=ROOT/'source/textures'/version/p.relative_to(VERSIONS[version]/'compiled-assets/textures')
            dst.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dst)
            try: report=decode_texture(p,ROOT/'textures'/(name+'.png'))
            except Exception as ex: report={'error':str(ex)};failures.append({'name':name,'error':str(ex)})
            textures[name]={'version':version,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),**report}
    shutil.copytree(VERSIONS['C']/'compiled-assets/foliage-package',ROOT/'source/foliage-package-C',dirs_exist_ok=True)
    report={'versions':inventory,'assets':assets,'texture_index':textures,
            'summary':{'unique_names':len(grouped),'editable_mesh_variants':len(assets),
                       'categories':dict(collections.Counter(e['category'] for e in assets)),
                       'textures_decoded':sum('outputs' in e for e in textures.values()),'texture_failures':failures}}
    (ROOT/'reports/manifest.json').write_text(json.dumps(report,ensure_ascii=False,indent=2))
    print(json.dumps(report['summary'],ensure_ascii=False,indent=2),flush=True)

if __name__=='__main__':main()
