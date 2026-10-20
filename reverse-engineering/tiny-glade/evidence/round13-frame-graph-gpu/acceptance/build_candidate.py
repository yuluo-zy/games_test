from pathlib import Path
import difflib, hashlib, json

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/acceptance'
rel = Path('tiny-glade/crates/tiny-glade/tests/original_gameplay_chain.rs')
base = (root / rel).read_text(encoding='utf-8')
candidate = base.replace('assert_nonempty_original_walls(world, shaders.as_ref());', 'assert_nonempty_original_walls(world, shaders.as_ref(), &bridge);')
candidate = candidate.replace('fn assert_nonempty_original_walls(\n    world: &mut bevy_ecs::world::World,\n    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,\n)', 'fn assert_nonempty_original_walls(\n    world: &mut bevy_ecs::world::World,\n    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,\n    bridge: &CoreRenderBridge,\n)')
needle = '    assert!(nonempty_allocations > 0, "no loaded wall produced real allocated brick chunks");'
assert needle in candidate
addition = '''
    // An allocation/pending=false records the CPU write; a later device flush
    // may still have failed. Require the production publisher's outcome and
    // the exact physical namespace used by this App's bridge.
    let status = world.resource::<country_core::geometry::runtime::WallBrickUploadStatus>();
    assert!(status.gpu_ready, "brick publisher failed: {:?}", status.last_error);
    assert_eq!(status.capacity_failures, 0, "brick allocation capacity failure");
    assert!(status.last_error.is_none(), "brick upload error was retained");
    let storage = world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>()
        .get::<WallBrickInstance>().expect("no canonical brick sparse SSBO");
    let ranges = storage.to_draw_multi_instanced();
    let count: u64 = ranges.iter().map(|(_, count)| u64::from(*count)).sum();
    assert!(count > 0, "allocated brick chunks contain no drawable instances");
    assert!(storage.dirty_bitset.iter().all(|word| word.load(std::sync::atomic::Ordering::Relaxed) == 0),
        "brick upload left dirty chunks");
    assert!(bridge.hosts().buffers.managed_buffer(&storage.buffer.buffer).is_some(),
        "brick SSBO does not resolve on the production App device");
    eprintln!("actual uploaded wall brick instances: {count}, draw ranges: {}", ranges.len());
'''
candidate = candidate.replace(needle, needle + '\n' + addition)
needle2 = '    assert_eq!(ids.len(), 7, "original seven global wall drawables were not produced exactly once");'
assert needle2 in candidate
addition2 = '''
    // A draw-list item only proves ECS admission. Its indexed mesh must also
    // resolve to a nonempty VAO and retained physical index-buffer owner.
    for entity in &ids {
        let handle = world.get::<country_core::asset_libraries::Handle<country_core::render::vao::Vao>>(*entity)
            .expect("wall drawable has no published VAO");
        let vao = world.resource::<country_core::asset_libraries::vao_library::AssetVAOLibrary>()
            .get_by_handle(*handle).expect("wall VAO handle is stale");
        assert!(vao.index_count > 0, "wall drawable has no mesh indices");
        assert!(bridge.hosts().buffers.managed_buffer(&vao.vao.index_buffer).is_some(),
            "wall VAO index buffer does not resolve on the production App device");
    }
'''
candidate = candidate.replace(needle2, needle2 + '\n' + addition2)
assert candidate != base
for folder, text in [('base', base), ('candidate', candidate)]:
    target = out / folder / rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(text, encoding='utf-8', newline='\n')
patch = ''.join(difflib.unified_diff(base.splitlines(True), candidate.splitlines(True), fromfile='a/'+str(rel).replace('\\','/'), tofile='b/'+str(rel).replace('\\','/')))
(out/'nonempty-uploads.patch').write_text(patch, encoding='utf-8', newline='\n')
(out/'manifest.json').write_text(json.dumps({'scope':'CPU allocation, successful device publisher, nonzero actual ranges and physical owner admission; not GPU readback/pixel correctness/frame-loop acceptance', 'production_modified':False, 'cargo_executed':False, 'file':str(rel).replace('\\','/'), 'base_sha256':hashlib.sha256(base.encode()).hexdigest(), 'candidate_sha256':hashlib.sha256(candidate.encode()).hexdigest()}, indent=2), encoding='utf-8')
print(out/'nonempty-uploads.patch')
