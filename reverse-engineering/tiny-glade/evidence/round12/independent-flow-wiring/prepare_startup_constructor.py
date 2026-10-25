from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');here=Path(__file__).parent;out=here/'startup-constructor-candidate';rows={};patch=[]
def save(rel,new):
 p=root/rel;old=p.read_bytes() if p.exists() else b'';dst=out/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(new)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(new).hexdigest()}
 patch.append(''.join(difflib.unified_diff(old.decode().splitlines(True),new.decode().splitlines(True),'a/'+rel if old else '/dev/null','b/'+rel)))
def nl_at(raw,pos):
 end=raw.index(b'\n',pos);return b'\r\n' if raw[end-1:end]==b'\r' else b'\n'
rel='crates/tiny-glade/src/run_game.rs';raw=(root/rel).read_bytes();src=raw
needle=b'#[path = "history_runtime.rs"]';pos=src.index(needle);nl=nl_at(src,pos)
src=src[:pos]+b'#[path = "startup_construction_runtime.rs"]'+nl+b'mod startup_construction_runtime;'+nl+nl+src[pos:]
needle=b'    install_history_and_wall_runtime(&mut app, config.dev.demo);';assert src.count(needle)==1
src=src.replace(needle,needle+nl_at(src,src.index(needle))+b'    startup_construction_runtime::install(&mut app);',1)
stage19=src.index(b'    // add_wall_systems @140493197',src.index(b'fn install_history_and_wall_runtime'))
start=src.index(b'            publish_public_wall_state,',stage19)
end=src.index(b'            spatial_hash_update,',start)+len(b'            spatial_hash_update,')
nl=nl_at(src,start)
replacement='''            publish_public_wall_state,
            country_core::systems::wall::delete_wall::delete_wall,
            system_wall_constructor::gpu_output::delete_constructor_output,
            spatial_hash_update,
            country_core::systems::wall::calc_wall_bend::calc_wall_bend,
            system_wall_constructor::gpu_output::clear_constructor_planks,
            system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_wall_path_segmentation,
            system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height,'''.encode().replace(b'\n',nl)
src=src[:start]+replacement+src[end:];save(rel,src)

rel='crates/tiny-glade/src/history_runtime.rs';raw=(root/rel).read_bytes();nl=nl_at(raw,0)
# Replace only the obsolete CPU block. Keep replay40, color consumers and all
# actual command adapters; no editing/UI placeholder is added.
start=raw.index(b'    app.add_systems('+nl+b'        Update,'+nl+b'        ('+nl+b'            system_roof::create_roof::create_roof,')
end=raw.index(b'    app.add_systems('+nl+b'        Update,'+nl+b'        ('+nl+b'            system_color::write_roof_color_id_to_gpu',start)
contract=(here.parent/'roof-visual/roof-registration.integration.rs').read_text(encoding='utf-8')
group=contract[contract.index('app.add_systems(Update, ('):contract.index('app.add_systems(Update,\n    system_roof::deferred_record_roof_history')]
group+='''app.add_systems(Update,
    system_roof::gpu_flush_buffers.in_set(GameSystemSet(44)));
// Preserve already existing nonempty command adapters. Their individual native
// caller stages remain separately adjudicated; they are not native20 items.
app.add_systems(Update, (
    system_roof::edit_roof::switch_roof_direction,
    system_roof::ui_edit_roof_ridge_dir::switch_roof_ridge_direction,
    system_roof::edit_roof::set_flat_roof_rim_style,
    system_roof::update_roof_spatial::update_wall_gable_heights,
).chain().after(system_roof::replay_roof::replay_roof).in_set(GameSystemSet(40)));
'''
replacement='\n'.join('    '+line if line else '' for line in group.splitlines())+'\n'
src=raw[:start]+replacement.encode().replace(b'\n',nl)+raw[end:]
# This is an explicit current-owner adapter edge, not a native standalone item.
needle=b'.after(GameSystemSet(19))'+nl+b'            .before(GameSystemSet(35))'
assert needle in src
src=src.replace(needle,b'.after(GameSystemSet(19))'+nl+b'            .before(GameSystemSet(20))',1)
save(rel,src)
save('crates/tiny-glade/src/startup_construction_runtime.rs',(here/'startup_construction_runtime.rs').read_bytes())
rel='crates/country-core/src/systems/wall/construct_wall_bricks.rs';raw=(root/rel).read_bytes()
start=raw.index(b'#[derive(Event, Clone, Copy, Debug)]');end=raw.index(b'pub struct RecalculateWallBricks(pub WallId);',start)+len(b'pub struct RecalculateWallBricks(pub WallId);')
src=raw[:start]+b'pub use crate::resources::roofs::commands::RecalculateWallBricks;'+raw[end:]
save(rel,src)
(here/'startup-constructor-candidate.patch').write_text(''.join(patch),encoding='utf-8',newline='')
(here/'startup-constructor-candidate-manifest.json').write_text(json.dumps({'created_at':datetime.datetime.now().astimezone().isoformat(),'files':rows,'status':'Evidence only. Original numbered nonempty constructor slice, no edit/UI helper. Not Cargo verified; full omitted producers remain explicit.'},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
