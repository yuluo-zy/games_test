from pathlib import Path
import hashlib,json,difflib,subprocess
ROOT=Path('D:/game/tiny-glade'); OUT=Path(__file__).resolve().parent
FILES={}
def read(rel):
    data=(ROOT/rel).read_bytes();FILES[rel]=[data,None];return data.decode('utf-8')
def write(rel,text):
    if rel not in FILES:FILES[rel]=[(ROOT/rel).read_bytes() if (ROOT/rel).is_file() else b'',None]
    base=FILES[rel][0]; ending='\r\n' if b'\r\n' in base and b'\n' not in base.replace(b'\r\n',b'') else '\n'
    text=text.replace('\r\n','\n')
    FILES[rel][1]=text.replace('\n',ending).encode()
def change(text,old,new,n=1):
    assert text.count(old)==n,(old[:120],text.count(old),n)
    return text.replace(old,new)
rel='crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs'
read(rel);write(rel,(OUT/'controller.rs').read_text())
rel='crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs'
s=read(rel).replace('\r\n','\n')
s=change(s,'        let ranges = steps.segments.entry(wall_id).or_default();','        // Native751f..7520 queues this wall before clearing its32B ranges.\n        steps.pending.push(wall_id);\n        let ranges = steps.segments.entry(wall_id).or_default();')
write(rel,s)
rel='crates/systems/wall-constructor/src/gpu_output.rs';s=read(rel).replace('\r\n','\n')
s=change(s,'    Gates,\n    Lintels,','    Gates,\n    PlatformStairs,\n    Lintels,')
s=change(s,'ConstructorPart::Arches | ConstructorPart::Gates)','ConstructorPart::Arches | ConstructorPart::Gates | ConstructorPart::PlatformStairs)')
write(rel,s)
rel='crates/systems/wall-constructor/Cargo.toml';s=read(rel).replace('\r\n','\n')
s=change(s,'country-core.workspace = true','country-core.workspace = true\nbytemuck.workspace = true');write(rel,s)
rel='crates/country-core/src/systems/collision/world.rs';s=read(rel).replace('\r\n','\n')
s=change(s,'fn registry_handle_region(world: &RaycastWorld, handle: ColliderHandle) -> Option<[u64; 3]> {\n    let (index, generation) = handle.into_raw_parts();','fn registry_handle_region(world: &RaycastWorld, slot: u8, handle: ColliderHandle) -> Option<[u64; 3]> {\n    // Native user_data identifies a global source, not a ColliderSet local row.\n    let collider=world.slots[slot as usize].colliders.get(handle)?;\n    let (index,generation)=if collider.user_data!=0 {\n        (collider.user_data as u32,(collider.user_data>>32) as u32)\n    } else { handle.into_raw_parts() };')
s=change(s,'registry_handle_region(world, handle)','registry_handle_region(world, slot_index as u8, handle)',4)
s=change(s,'    pub source: RaycastGeoSource,\n}','    pub source: RaycastGeoSource,\n    /// Original source+8 payload; platform tag7 carries its full WallId.\n    pub source_payload: Option<u64>,\n}')
s=change(s,'        Self { data, source }','        Self { data, source, source_payload: None }')
s=change(s,'    let slot = source.subworld() as usize;\n    // O：先取旧值；实时 collider 的 Isometry 与入参逐位比较。','''    upsert_collider_with_source_payload(world,map,key,source,None,mesh,position,rotation)
}

/// Platform's actual source payload entry. Other unknown payload callers remain
/// explicit gaps; their existing signature remains available.
#[cfg(feature = "native-physics")]
pub fn upsert_collider_with_source_payload<K: std::hash::Hash + Eq>(
    world: &mut RaycastWorld,
    map: &mut std::collections::HashMap<K, ColliderMeshInstance>,
    key: K,
    source: RaycastGeoSource,
    source_payload: Option<u64>,
    mesh: Arc<RaycastMeshData>,
    position: glam::Vec3,
    rotation: glam::Quat,
) -> ColliderUpsert {
    use slotmap::Key;
    let slot = source.subworld() as usize;
    // O：先取旧值；实时 collider 的 Isometry 与入参逐位比较。''')
s=change(s,'        if pose_matches {\n            return ColliderUpsert::Kept {','''        let old_key=RaycastMeshKey::from(slotmap::KeyData::from_ffi(
            u64::from(existing.registry_index)|(u64::from(existing.generation)<<32)));
        let old_mesh=world.mesh_sources.get(old_key)
            .expect("upsert_collider: old mesh-source key invalid");
        let mesh_matches=old_mesh.data.hash==mesh.hash && old_mesh.source==source
            && old_mesh.source_payload==source_payload;
        if pose_matches && mesh_matches {
            return ColliderUpsert::Kept {''')
s=change(s,'    // O：`to_offset_collider_builder` + `ColliderBuilder::build`（sensor 字节 true）。','''    let mesh_key=world.mesh_sources.insert(RaycastMesh{data:mesh.clone(),source,source_payload});
    let mesh_identity=mesh_key.data().as_ffi();
    // O：`to_offset_collider_builder` + `ColliderBuilder::build`（sensor 字节 true）。''')
s=change(s,'        .sensor(true)\n        .build();','        .sensor(true)\n        .user_data(u128::from(mesh_identity))\n        .build();')
s=change(s,'    let (index, generation) = collider_handle.into_raw_parts();\n    world.ensure_registry_entry(index, generation, 2);','''    let (index,generation)=(mesh_identity as u32,(mesh_identity>>32) as u32);
    // SlotMap owns allocation and generations. This registry is its query
    // projection, never a ColliderSet index alias across17 independent slots.
    if world.registry_free_head==index {
        world.registry_free_head=world.colliders[index as usize].free_next();
    }
    world.ensure_registry_entry(index,generation,source.tag());
    let entry=&mut world.colliders[index as usize];
    entry.first=u32::from(source.tag()); entry.meta=[0;0x1c];
    if let Some(payload)=source_payload {entry.meta[4..12].copy_from_slice(&payload.to_le_bytes());}''')
s=change(s,'    let kind = entry.kind();\n    let old_head = world.registry_free_head;','''    let kind = entry.kind();
    let mesh_key=RaycastMeshKey::from(slotmap::KeyData::from_ffi(
        u64::from(instance.registry_index)|(u64::from(instance.generation)<<32)));
    // Removing the actual source releases its mesh Arc and invalidates its key.
    world.mesh_sources.remove(mesh_key).expect("dispose_collider_mesh_instance: mesh source missing");
    let old_head = world.registry_free_head;''')
s+='\n#[path = "platform_stair_owner.rs"]\nmod platform_stair_owner;\npub use platform_stair_owner::PlatformStairColliderMap;\n'
write(rel,s)
write('crates/country-core/src/systems/collision/platform_stair_owner.rs',(OUT/'platform_stair_owner.rs').read_text())
for rel,(base,candidate) in FILES.items():
    path=OUT/'candidate'/rel;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(candidate)
fmt=Path('C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe')
for rel in FILES:
    if rel.endswith('.rs'):
        path=OUT/'candidate'/rel
        r=subprocess.run([str(fmt),'--edition','2024','--config','skip_children=true',str(path)],capture_output=True,text=True)
        if r.returncode:raise RuntimeError((rel,r.stderr))
        FILES[rel][1]=path.read_bytes()
patch=[];manifest=[]
for rel,(base,candidate) in FILES.items():
    a=base.decode('utf-8').replace('\r\n','\n').splitlines(True)
    b=candidate.decode('utf-8').replace('\r\n','\n').splitlines(True)
    patch.extend(difflib.unified_diff(a,b,fromfile='a/'+rel,tofile='b/'+rel))
    manifest.append({'path':rel,'candidateRootBase':hashlib.sha256(base).hexdigest() if base else None,'candidateSHA256':hashlib.sha256(candidate).hexdigest(),'sourceBytes':len(base),'candidateBytes':len(candidate),'baseLines':len(a),'candidateLines':len(b)})
(OUT/'candidate.patch').write_text(''.join(patch),encoding='utf-8',newline='\n')
(OUT/'manifest.json').write_text(json.dumps({'productionWritten':False,'cargoRun':False,'rustfmtFullSourceParsed':True,'files':manifest},indent=2),encoding='utf-8')
print(json.dumps(manifest,indent=2))
