from pathlib import Path
import json,hashlib,struct,subprocess,math,difflib
ROOT=Path('D:/game'); BASE=ROOT/'reverse-engineering/tiny-glade/evidence/round12'; OUT=BASE/'history/clearing-independent-review';OUT.mkdir(exist_ok=True)
image=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=image.read_bytes();sha=lambda b:hashlib.sha256(b).hexdigest()
pe=struct.unpack_from('<I',raw,0x3c)[0];opt=pe+24;imagebase=struct.unpack_from('<Q',raw,opt+24)[0];sections=[]
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
    off=opt+struct.unpack_from('<H',raw,pe+20)[0]+i*40
    vs,va,rs,rp=struct.unpack_from('<IIII',raw,off+8);sections.append((va,vs,rs,rp))
def read(va,size):
    rva=va-imagebase
    for start,vs,rs,rp in sections:
        if start<=rva<start+max(vs,rs):return raw[rp+rva-start:rp+rva-start+size]
    raise ValueError(hex(va))
ranges={0x140650c50:243,0x140ea68d0:1079,0x140ea6d10:39,0x140ea74a0:1881,0x140ea7c00:138,0x140ea7c90:850,0x140ea7ff0:1869}
aliases={a:[] for a in ranges}
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8'):
    p=line.rstrip('\n').split('\t',4)
    if len(p)==5 and int(p[0],16) in aliases:aliases[int(p[0],16)].append({'size':int(p[1]),'module':p[3],'name':p[4]})
reg={'original_sha256':sha(raw),'functions':{hex(a):{'size':n,'sha256':sha(read(a,n)),'raw_hex':read(a,n).hex(),'pdb_aliases':aliases[a]} for a,n in ranges.items()},
     'system_set_vtable':{'address':'0x142b8fc58','raw_hex':read(0x142b8fc58,0x48).hex()},
     'observed_contract':{'schedule':'Update','set':'GameSystemSet(36)','systems':['manage_tree_prefabs','forest_tree_animation','handplaced_tree_animation'],'ordering':'chain_ignore_deferred','run_condition':None,'resource':'InteractableForestPlacements default: empty records, processed=false'}}
(OUT/'forest-registration-original.json').write_text(json.dumps(reg,indent=2),encoding='utf8')
objdump='C:/Program Files/LLVM/bin/llvm-objdump.exe'
asm=subprocess.run([objdump,'-d','--start-address=0x140650c50','--stop-address=0x140650d43',str(image)],capture_output=True,text=True,check=True).stdout
(OUT/'forest-registration-original.asm').write_text(asm,encoding='utf8')
f32=lambda v:struct.unpack('<f',struct.pack('<f',v))[0]
assets=ROOT/'ljxsj_92385/Tiny Glade/assets'; inventory=[]
helpers={'olden':['trees/rocks.json','trees/clearing_edge_rocks.json','trees/stumps.json','trees/clearing_edge_tree_stumps.json','trees/logs.json','trees/clearing_edge_tree_logs.json'],'flowery':['plants/lavender.json']}
for theme in ['summer','olden','flowery','autumn','winter']:
    folder=assets/'glade'/theme; config=folder/'trees.json';groups=json.loads(config.read_text()); tree_rows=[];used=[]
    for g in groups:
        skipped=g['prefab']=='clearing/tree/bush';path=folder/g['placements'];data=json.loads(path.read_text())
        outside=sum(not(abs(p['position'][0])<65 and abs(p['position'][2])<65) for p in data)
        eligible=[]
        for p in data:
            x,y,z=map(f32,p['position']);radius=f32(math.sqrt(f32(f32(z*z)+f32(f32(x*x)+f32(y*y)))))
            if radius<45 and f32(p['scale'])>f32(.7):eligible.append(p)
        row={'prefab':g['prefab'],'placements':g['placements'],'sha256':sha(path.read_bytes()),'records':len(data),'skipped':skipped,'exterior_records':outside,'particle_eligible_placements':len(eligible)}
        if not skipped:
            used.append(g['placements'])
            if g.get('particle_emitter'):
                mp=assets/'meshes'/g['particle_emitter'];mesh=json.loads(mp.read_text());row['particle_mesh']=g['particle_emitter'];row['particle_mesh_sha256']=sha(mp.read_bytes());row['particle_mesh_top_keys']=list(mesh)
                row['particle_vertices']=len(mesh['Vertex_Position']['buffer']);row['particle_index_count']=len(mesh['indices']['buffer']);row['particle_emitter_records']=len(eligible)*row['particle_vertices']
        tree_rows.append(row)
    for helper in helpers.get(theme,[]):used.append(helper)
    helper_records=[{'path':h,'sha256':sha((folder/h).read_bytes()),'records':len(json.loads((folder/h).read_text()))} for h in helpers.get(theme,[])]
    emitters=sum(r.get('particle_emitter_records',0) for r in tree_rows)
    inventory.append({'theme':theme,'trees_config_sha256':sha(config.read_bytes()),'groups':tree_rows,'forest_record_count':sum(r['exterior_records'] for r in tree_rows if not r['skipped']),'particle_emitters_complete_vector':emitters,'particle_emitters_published_count':min(emitters,1100),'particle_upload_bytes':emitters*16,'helper_records':helper_records,'placement_load_duplicate_paths_in_clearing_and_theme_helper':[p for p in set(used) if used.count(p)>1]})
(OUT/'five-theme-independent-inventory.json').write_text(json.dumps(inventory,indent=2),encoding='utf8')
policy=BASE/'clearing/native_placement_policy.rs';before=policy.read_text();after=before.replace('Quat::from_rotation_y(rotation)','Quat::from_axis_angle(Vec3::Y, rotation)')
assert before!=after
(OUT/'native_placement_policy.axis-angle-candidate.rs').write_text(after,encoding='utf8')
(OUT/'particle-axis-angle-erratum.patch').write_text(''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/tiny-glade/crates/country-core/src/startup/native_placement_policy.rs',tofile='b/tiny-glade/crates/country-core/src/startup/native_placement_policy.rs')),encoding='utf8')
fixtures=json.loads((OUT/'particle-extra-original.json').read_text())['cases']
probe='extern crate glam;\nmod native_placement_policy;\nuse glam::Vec3;\nfn v(a:[u32;3])->Vec3{Vec3::from_array(a.map(f32::from_bits))}\nfn main(){let mut failures=0;\n'
for i,r in enumerate(fixtures):
    probe+=f'if native_placement_policy::particle_emitter_for_vertex(v({r["position_bits"]}),f32::from_bits({r["scale_bits"]}),f32::from_bits({r["angle_bits"]}),v({r["vertex_bits"]}))!=Some({r["native_bits"]}){{failures+=1;println!("mismatch {i}");}}\n'
probe+='println!("cases=360 failures={failures}");assert_eq!(failures,0);}\n'
(OUT/'particle_extra_comparison.rs').write_text(probe,encoding='utf8')
print(json.dumps({'themes':[{k:r[k] for k in ['theme','forest_record_count','placement_load_duplicate_paths_in_clearing_and_theme_helper']} for r in inventory],'registration_source_sha':sha(raw),'policy_before_sha256':sha(before.encode()),'policy_candidate_sha256':sha(after.encode()),'scope':'original raw/PDB plus independent shipped assets, candidate Rust probe NOT run'},indent=2))
