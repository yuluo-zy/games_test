from pathlib import Path
import json,hashlib,subprocess
O=Path(__file__).resolve().parent;R=Path('D:/game');C=O/'runtime-candidate';manifest=json.loads((C/'manifest.json').read_text())
for species,element,count,projection in [
 ('butterfly','Butterfly',24,'ButterflySsbo::from_components(transform.translation, transform.rotation, butterfly.phase_ref)'),
 ('firefly','Firefly',96,'FireflySsbo::from_translation(transform.translation)')]:
 rel=f'tiny-glade/crates/systems/critters/src/{species}.rs';before=(R/rel).read_bytes();s=before.decode('utf8').replace('\r','')
 a=s.index('pub fn '+species+('ies' if False else '')) if False else s.index('pub fn '+('butterflies' if species=='butterfly' else 'fireflies')+'_send_to_gpu(')
 b=s.index('\n}',a)+2
 body=f'''pub fn {"butterflies" if species=="butterfly" else "fireflies"}_send_to_gpu(
    mut buffers: ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    query: Query<(&Transform, &{element})>,
) {{
    let instances: Vec<{element}Ssbo> = query.iter().map(|(transform, {species if species=='butterfly' else '_firefly'})| {projection}).collect();
    if instances.is_empty() {{ return; }}
    // Original author asserts the complete fixed population before looking up its SSBO.
    assert_eq!(instances.len(), {count}, "Unexpected {species} population for GPU upload");
    if let Some(buffer) = buffers.get_mut::<{element}Ssbo>() {{
        buffer.cpu_write(&instances);
    }}
}}'''
 s=s[:a]+body+s[b:]
 # Replace stale API documentation immediately preceding the actual function.
 d=s.rfind('///',0,a); first=s.rfind('\n}\n',0,a)+3
 if first>0: s=s[:first]+'\n/// Original full population projection, count validation, and mapped SSBO write.\n'+s[a:]
 p=C/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding='utf8')
 subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',str(p)],check=True)
 manifest.append({'path':rel,'base_sha256':hashlib.sha256(before).hexdigest(),'proposed_sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(C/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print('prepared actual mapped upload2 leafs')
