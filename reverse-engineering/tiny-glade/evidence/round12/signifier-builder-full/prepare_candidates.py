from pathlib import Path
import json,hashlib,difflib,subprocess
ROOT=Path('D:/game');OUT=Path(__file__).resolve().parent
rows=[];changes=[]
def change(path,transform):
    before=(ROOT/path).read_text(encoding='utf8');after=transform(before)
    base=OUT/'base'/path;base.parent.mkdir(parents=True,exist_ok=True)
    if base.exists():assert base.read_text(encoding='utf8')==before,'baseline changed: '+path
    else:base.write_text(before,encoding='utf8',newline='\n')
    target=OUT/'candidate'/path;target.parent.mkdir(parents=True,exist_ok=True);target.write_text(after,encoding='utf8',newline='\n')
    changes.append((path,before,target))
core='tiny-glade/crates/country-core/src/'
def factory(s):
    s=s.replace('        let payload = self.payload();\n        let mut cloned = Self::new(payload, self.inner, self.marker);',
        '''        if self.name_ptr.is_null() {
            return Self { name_len: self.name_len, name_ptr: std::ptr::null_mut(),
                name_len_dup: self.name_len_dup, inner: self.inner, marker: self.marker,
                closure: self.closure.clone() };
        }
        let payload = self.payload();
        let mut cloned = Self::new(payload, self.inner, self.marker);''')
    start=s.index('/// `SignifierEntityBuilder` —')
    s=s[:start]+(OUT/'builder.rs').read_text(encoding='utf8')+(OUT/'builder_tests.rs').read_text(encoding='utf8')
    s=s.replace('use super::signifier_create::SignifierId;\n','')
    s=s.replace('        std::str::from_utf8(self.payload()).ok()',
        '        if self.name_ptr.is_null() { None } else { Some(std::str::from_utf8(self.payload()).expect("valid original name string")) }')
    a=s.index('//! `build` 完整入队需要');b=s.index('\n\nuse ',a)
    s=s[:a]+'//! Complete typed builder below resolves all five mesh branches and preserves\n//! Drawable/animation/pass/lifespan/transient-mesh lifecycle. Native evidence\n//! and independent verification are retained in round12/signifier-builder-full.'+s[b:]
    return s
change(core+'systems/ui/signifiers/signifier_factory.rs',factory)
def animation(s):
    a=s.index('    /// crate 附加：原版把 tick');b=s.index('\n}',a);s=s[:a]+s[b:]
    s=s.replace('            tick_acc: 0.0,\n','')
    a=s.index('pub fn signifier_animation_t(');b=s.index('\n/// `signifier_animation`',a)
    s=s[:a]+(OUT/'animation_impl.rs').read_text(encoding='utf8')+'\n'+s[b:]
    s=s.replace('std::mem::size_of::<SignifierAnimation>() - std::mem::size_of::<f32>()','std::mem::size_of::<SignifierAnimation>()')
    s=s.replace('    /// +0x00..+0x08：未取证（保持 0）。','    /// +0x00..+0x08：EasingFunction tag/payload, read through the existing utils enum.')
    fixture=json.loads((OUT.parent/'history/signifier-animation-independent-original.json').read_text())['native_t_cases']
    source='\n#[cfg(test)]\nmod original_t_oracle {\n use super::*;\n #[test]\n fn all_1008_independent_original_sse_vectors() {\n'
    source+='  // Original140a69998..9fe, SHA256 '+fixture['segment_sha256']+'\n'
    source+='  let cases: &[(u32,u32,u32,bool,u32)] = &[\n'
    for duration,t,delta,marked,result in fixture['cases']:
        source+=f'   (0x{duration:08x},0x{t:08x},0x{delta:08x},{str(marked).lower()},0x{result:08x}),\n'
    source+='  ];\n  for &(duration,t,delta,marked,result) in cases {\n   let mut anim=SignifierAnimation { duration:f32::from_bits(duration),t:f32::from_bits(t),..Default::default() };\n   anim.advance(f32::from_bits(delta),marked);\n   assert_eq!(anim.t.to_bits(),result,"duration={duration:08x}, t={t:08x}, delta={delta:08x}, marked={marked}");\n  }\n }\n}\n'
    return s+source
change(core+'systems/ui/signifiers/animation.rs',animation)
def water(s):
    s=s.replace('_mesh_library: &AssetMeshLibrary','mesh_library: &AssetMeshLibrary').replace('_shader_library: &AssetShaderLibrary','shader_library: &AssetShaderLibrary')
    s=s.replace('closure_0(point, scale, commands)','closure_0(point, scale, mesh_library, shader_library, commands)')
    a=s.index('pub fn closure_0(');s=s[:a]+'''pub fn closure_0(position: Vec3, scale: f32, meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary, commands: &mut Commands) -> Vec<Entity> {
    vec![SignifierEntityBuilder::from_mesh_name(BRUSH_CIRCLE_SCRIBBLE)
        .animation_duration(0.0).position(position).scale(scale)
        .set_ui_pass(0).build(meshes, shaders, commands)]
}
'''
    a=s.index('/// GAP（既有，core 侧登记）');b=s.index('pub fn closure_0(',a)
    s=s[:a]+'/// Both actual libraries are passed through the original factory invocation.\n'+s[b:]
    return s
change('tiny-glade/crates/systems/water/src/draw_water.rs',water)
def border(s):
    s=s.replace('_mesh_library: &AssetMeshLibrary','mesh_library: &AssetMeshLibrary').replace('_shader_library: &AssetShaderLibrary','shader_library: &AssetShaderLibrary')
    s=s.replace('closure_1(&env, commands)','closure_1(&env, mesh_library, shader_library, commands)')
    s=s.replace('pub fn closure_1(env: &GladeBorderSignifierEnv, commands: &mut Commands) -> Vec<Entity> {',
        'pub fn closure_1(env: &GladeBorderSignifierEnv, meshes: &AssetMeshLibrary, shaders: &AssetShaderLibrary, commands: &mut Commands) -> Vec<Entity> {')
    a=s.index('    // 2. O：SignifierEntityBuilder 链');b=s.index('    // 3. O：2× Commands insert',a)
    s=s[:a]+'''    let entity = SignifierEntityBuilder::from_points(points)
        .set_ui_pass(0).animation_duration(0.0).custom_shader("dashed_line")
        .build(meshes, shaders, commands);
'''+s[b:]
    s=s.replace('        let mut world = World::new();\n        let points = vec!', '''        let mut world = World::new();
        let meshes=AssetMeshLibrary::new();
        let shaders=AssetShaderLibrary::new();
        shaders.add_raster(country_core::render::shader::RasterShaderProgram::new_draw(
            "dashed_line", Default::default(), &CpuShaders), false);
        let points = vec!''')
    s=s.replace('closure_1(&env, &mut commands)','closure_1(&env, &meshes, &shaders, &mut commands)')
    at=s.index('    /// 接线验证：perimeter')
    s=s[:at]+(OUT/'border_test_backend.rs').read_text(encoding='utf8')+'\n'+s[at:]
    return s
change('tiny-glade/crates/systems/glade-border/src/display_glade_border.rs',border)
change(core+'systems/ui/signifiers/mod.rs',lambda s:s.replace('SignifierEntityBuilder, SignifierFactory','SignifierEntityBuilder, SignifierFactory, SignifierMesh'))
def delete(s):
    s=s.replace('lifespan.min_lifespan <= lifespan.t','!(lifespan.min_lifespan > lifespan.t)')
    s=s.replace('// O：`RDI == None`（无最短寿命组件）→ 命中；否则 `min_lifespan <= t` 才命中。',
        '// Original UCOMISS/JA: a positive ordered lifespan comparison skips; unordered is expired.')
    s=s.replace('/// `despawn_marked` @0x140929a90（O）：销毁所有带删除标记的指示符实体。',
        '/// Original140929a90: no animation, or ordered t==±0 after reverse, permits deletion.')
    a=s.index('pub fn despawn_marked(')
    s=s[:a]+'''pub fn despawn_marked(query: Query<(Entity, &SignifierMarkedForDeletion,
    Option<&super::animation::SignifierAnimation>)>, mut commands: Commands) {
    for (entity, _, animation) in &query {
        if animation.is_none_or(|animation| animation.t==0.0) {
            commands.entity(entity).despawn();
        }
    }
}
'''
    return s
change(core+'systems/ui/signifiers/signifier_mark_for_delete.rs',delete)
def lifecycle_tests(s):
    a=s.index('    submit(&mut world, &second);\n    schedule.run(&mut world);\n    assert!(world.get_entity(first_entity).is_err());')
    s=s[:a]+s[a:].replace('    submit(&mut world, &second);\n    schedule.run(&mut world);', '''    submit(&mut world, &second);
    schedule.run(&mut world);
    assert!(world.get::<SignifierMarkedForDeletion>(first_entity).is_some());
    assert!(registry.lock().unwrap().is_empty());
    submit(&mut world, &second);
    schedule.run(&mut world);''',1)
    s=s.replace('    schedule.run(&mut world);\n    assert!(entities(&mut world).is_empty());\n    assert_eq!(*registry.lock().unwrap(), vec![first_mesh, second_mesh]);', '''    schedule.run(&mut world);
    assert!(world.get::<SignifierMarkedForDeletion>(second_entity).is_some());
    schedule.run(&mut world);
    assert!(entities(&mut world).is_empty());
    assert_eq!(*registry.lock().unwrap(), vec![first_mesh, second_mesh]);''')
    s=s.replace('    submit(&mut world, &line);\n    schedule.run(&mut world);\n    assert!(entities(&mut world).is_empty());', '''    submit(&mut world, &line);
    schedule.run(&mut world);
    schedule.run(&mut world);
    assert!(entities(&mut world).is_empty());''')
    s=s.replace('    schedule.run(&mut world);\n    assert!(world.get_entity(entity).is_err());', '''    schedule.run(&mut world);
    assert!(world.get::<SignifierMarkedForDeletion>(entity).is_some());
    schedule.run(&mut world);
    assert!(world.get_entity(entity).is_err());''')
    return s
change(core+'systems/ui/signifiers/tests.rs',lifecycle_tests)
subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',*[str(p) for _,_,p in changes]],check=True)
patch=''
for path,before,target in changes:
    after=target.read_text(encoding='utf8');patch+=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+path,tofile='b/'+path))
    rows.append({'path':path,'base_sha256':hashlib.sha256((ROOT/path).read_bytes()).hexdigest(),'proposed_sha256':hashlib.sha256(target.read_bytes()).hexdigest()})
(OUT/'production-review.patch').write_text(patch,encoding='utf8',newline='\n')
(OUT/'candidate-checkpoint.json').write_text(json.dumps({'status':'candidate only; no production writes/Cargo; awaiting independent lifecycle review','files':rows},indent=2)+'\n')
print(json.dumps({'candidate_files':len(rows)}))
