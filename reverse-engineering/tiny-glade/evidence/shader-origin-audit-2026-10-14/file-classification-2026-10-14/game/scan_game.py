import collections, hashlib, json, re, struct
from pathlib import Path

AUDIT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
RAW=Path('D:/game/ljxsj_92385/Tiny Glade')
INVENTORY=json.loads((AUDIT/'original-shader-inventory.json').read_text())
DEPENDENCIES=json.loads((AUDIT/'original-source-dependencies.json').read_text())
PATTERN=re.compile(r'^shaders/(wall/|roof/|buildings/|nani/|terrain_editing/|rocky_terrain/|garden/|grass/|ivy/|critters/|paths/|nani_|instanced_roof_|roof_|wall_|rasterize_buildings|gauss_blur_building_mask|terrain_|stairs_foliage|path_|reed|lilypad|sheep|solid_vertex_color_(wooden_gate|waterwheel|garden_stone|boat)|clearing/|water/(spawn_|draw_water_mask|read_water_mask|water_mask|waves|geometry)|painterly/|vermeer/)')
EXTRA_NAMES=['billboard_forest.raster.hlsl','billboard_plant.raster.hlsl','demo_terrain_overlay.raster.hlsl','flag_roof_cloth.raster.hlsl','flag_small.raster.hlsl','flag_wall_cloth.raster.hlsl','garden_lowpoly_flower.raster.hlsl','generate_grass.cs.hlsl','grass.raster.hlsl','grass_lod1.raster.hlsl','grass_object_push.raster.hlsl','leaf_particle.raster.hlsl','leaf_particles.cs.hlsl','mei_shadow.raster.hlsl','mouse_terrain_raycast.cs.hlsl','plaster.inc.hlsl','rain_particles.raster.hlsl','read_mouse_depth_value.cs.hlsl','read_terrain_stroke.cs.hlsl','smoke.inc.hlsl','snow_particles.raster.hlsl','sparkles.raster.hlsl','splash_particle.raster.hlsl','stairs_floor.inc.hlsl','star.raster.hlsl','timed_terrain_gizmo.raster.hlsl','wooden_plank.inc.hlsl']
EXTRA={'shaders/'+s for s in EXTRA_NAMES}
SOURCES={x['source']:x for x in DEPENDENCIES if PATTERN.search(x['source']) or x['source'] in EXTRA}
assert EXTRA<=SOURCES.keys(),EXTRA-SOURCES.keys()
rows=collections.defaultdict(list)
def string(words):return struct.pack('<'+'I'*len(words),*words).split(b'\0')[0].decode('utf-8','replace')
for file in INVENTORY:
    raw=(RAW/file['file']).read_bytes()
    assert hashlib.sha256(raw).hexdigest()==file['sha256']
    for stage in file['stages']:
        at=stage['module_offset'];count=stage['spirv_words'];part=raw[at:at+count*4]
        assert hashlib.sha256(part).hexdigest()==stage['spirv_sha256']
        words=struct.unpack('<'+'I'*count,part)
        names={};members=collections.defaultdict(dict);strings={}; constants={}; ptrroot={}; variables={}; types={}; strides={}; member_offsets=collections.defaultdict(dict);insts=[]
        i=5
        while i<count:
            wc=words[i]>>16;op=words[i]&65535;assert wc>0 and i+wc<=count
            args=words[i+1:i+wc]
            if op==5:names[args[0]]=string(args[1:])
            elif op==6:members[args[0]][args[1]]=string(args[2:])
            elif op==7:strings[args[0]]=string(args[1:])
            elif op==43 and len(args)==3:constants[args[1]]=args[2]
            elif op==59:variables[args[1]]=args[0];ptrroot[args[1]]=args[1]
            elif op in [28,29,30,32]:types[args[0]]=(op,list(args[1:]))
            elif op==71 and len(args)>=3 and args[1]==6:strides[args[0]]=args[2]
            elif op==72 and len(args)>=4 and args[2]==35:member_offsets[args[0]][args[1]]=args[3]
            if op in [65,66,67,70] and len(args)>=3:ptrroot[args[1]]=ptrroot.get(args[2],args[2])
            insts.append((at+i*4,op,args));i+=wc
        accum=collections.defaultdict(lambda:{'lines':set(),'opcodes':collections.Counter(),'resources':collections.Counter(),'line_examples':[]})
        source=None;line=None
        for pos,op,args in insts:
            if op==8:source=strings.get(args[0]);line=args[1]
            elif op==317:source=None;line=None
            elif source in SOURCES:
                row=accum[source];row['lines'].add(line);row['opcodes'][op]+=1
                pointers=[]
                if op in [65,66,67,70,61] and len(args)>=3:pointers.append(args[2])
                elif op==62 and args:pointers.append(args[0])
                for ptr in pointers:
                    root=ptrroot.get(ptr,ptr)
                    if root in variables:
                        name=names.get(root,str(root));row['resources'][name]+=1
                        if not name.isdecimal() and len(row['line_examples'])<40:
                            example={'raw_offset':pos,'source_line':line,'opcode':op,'resource':name}
                            if op in [65,66,67,70]:example['access_indices']=[{'id':x,'literal_value_if_constant':constants.get(x)} for x in args[3:]]
                            row['line_examples'].append(example)
        for source,row in accum.items():
            referenced=set(row['resources']);layouts=[]
            def walk(t,seen):
                if t in seen:return
                seen.add(t);op,args=types.get(t,(None,[]))
                if op==32:walk(args[1],seen)
                elif op in [28,29]:
                    if t in strides:layouts.append({'array_type_id':t,'stride':strides[t],'element_type_name':names.get(args[0])})
                    walk(args[0],seen)
                elif op==30:
                    layouts.append({'type_id':t,'type_name':names.get(t),'members':[{'index':ix,'name':members[t].get(ix),'offset':member_offsets[t].get(ix)} for ix in range(len(args))]})
                    for child in args:walk(child,seen)
            for vid,typ in variables.items():
                if names.get(vid,str(vid)) in referenced:walk(typ,set())
            rows[source].append({'raw_file':file['file'],'raw_sha256':file['sha256'],'stage':stage['stage'],'spirv_sha256':stage['spirv_sha256'],'source_line_min':min(row['lines']),'source_line_max':max(row['lines']),'line_count':len(row['lines']),'attributed_instruction_count':sum(row['opcodes'].values()),'opcodes':dict(row['opcodes']),'resources':dict(row['resources']),'access_examples':row['line_examples'],'resource_layouts':layouts})
report={'method':'Original distribution .bin SPIR-V only. Validate raw/module SHA-256 against strict parent inventory; map OpLine file IDs through original OpString, collect each source-attributed region and OpAccessChain/OpLoad/OpStore resource roots. Follow original pointer/runtime-array/struct definitions for member names/offsets and strides. Names/structs are observed shader interfaces, not original text.','sources':[{'source':s,'pipeline_count':SOURCES[s]['pipeline_count'],'consumers':SOURCES[s]['files'],'proofs':sorted(rows[s],key=lambda r:r['attributed_instruction_count'],reverse=True)[:3],'all_source_attributed_stage_count':len(rows[s])} for s in sorted(SOURCES)],'limitations':['All source text has been replaced by whitespace; names, layouts, OpLine attribution and instruction structure survive.','OpLine ownership establishes emitted source regions, not authorship of every formula within them.','Buffer layout is a consumer contract and can combine multiple includes.','No broad negative upstream search was performed; game_specific means game integration/contract evidence, never a claim that every line was invented by developers.']}
(OUT/'raw_game_evidence.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print('source_count',len(SOURCES),'sources_with_emitted_instruction_regions',sum(bool(rows[s]) for s in SOURCES))
for row in report['sources']:
    proofs=row['proofs'];p=proofs[0] if proofs else {};print(row['source'],p.get('attributed_instruction_count',0),','.join(list(p.get('resources',{}))[:12]))
