from pathlib import Path
r=Path('D:/game');b=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep';src=r/'tiny-glade'
def rd(p):return (src/p).read_text(encoding='utf-8')
def wr(p,s):p=b/p;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding='utf-8')
p=Path('crates/systems/critters/src/startup_sheep.rs');s=(b/p).read_text(encoding='utf-8').replace('shaders.raster.get_handle_by_name("vertex_color_shader")','shaders.get_raster_handle_by_name("vertex_color_shader").expect("vertex_color_shader must already be registered")');wr(p,s)
p=Path('crates/systems/critters/src/lib.rs');s=rd(p).replace('pub mod sheep;','pub mod sheep;\npub mod startup_sheep;');wr(p,s)
p=Path('crates/country-core/src/startup/startup_sheep.rs');wr(p,'''//! B startup_sheep140a093e0 asset half. Complete ECS birth and audio are\n//! system_critters::startup_sheep::spawn_startup_sheep (cross-crate owner).\n//! Do not pre-register a bare frame1 mesh: the original mesh cache hit skips\n//! animation buffer creation, so a mesh-only entry poisons the real startup.\npub use super::sheep_assets::{SheepStartupAssets, build_sheep_animation, load_sheep_startup_assets};\npub const SHEEP_BAAH_EVENT: &str = "event:/SFX/Sheep/Baah";\npub const SHEEP_PITCH_PARAMETER: &str = "Pitch";\n''')
p=Path('crates/country-core/src/startup/mod.rs');s=rd(p).replace('pub mod startup_sheep;','pub mod startup_sheep;\npub mod sheep_assets;');start=s.index('    // `startup_sheep` 在契约');end=s.index('\n}',start);s=s[:start]+'''    // Sheep requires the typed animation library and its full ECS birth. The
    // device host registers it; a bare mesh here would poison its cache gate.
'''+s[end:];wr(p,s)
p=Path('crates/country-core/src/resources/critters/sheep/control.rs');s=rd(p);s+='''
/// Loading before OnEnter(Game) can precede the original control component's
/// birth. Resource presence means one pending transfer, including a None clear.
/// This DTO is not a live SheepControl and never owns an applied snapshot.
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct PendingSheepSnapshot(pub Option<SheepControlSnapshot>);

/// Query the unique real owner, or retain a pre-birth DTO until startup consumes
/// it. A duplicate owner is an error rather than selecting an arbitrary entity.
pub fn install_loaded_sheep_snapshot(
    world: &mut bevy_ecs::world::World,
    snapshot: Option<SheepControlSnapshot>,
) -> Result<(), &'static str> {
    use crate::systems::debug::sheep_controls::SheepControl;
    use bevy_ecs::{entity::Entity, query::With};
    let owners: Vec<_> = world.query_filtered::<Entity, With<SheepControl>>().iter(world).collect();
    match owners.as_slice() {
        [] => { world.insert_resource(PendingSheepSnapshot(snapshot)); }
        [entity] => {
            world.remove_resource::<PendingSheepSnapshot>();
            world.get_mut::<SheepControl>(*entity).expect("validated live sheep control").snapshot=snapshot;
        }
        _ => return Err("multiple live SheepControl components"),
    }
    Ok(())
}
''';wr(p,s)
p=Path('crates/country-core/src/systems/save/load_game.rs');s=rd(p);start=s.index('    // Existing canonical SheepControl is the deferred snapshot owner.');end=s.index('    {\n        let mut walls',start);s=s[:start]+'''    crate::resources::critters::sheep::control::install_loaded_sheep_snapshot(world, parsed.sheep)
        .map_err(anyhow::Error::msg)?;
'''+s[end:];wr(p,s)
p=Path('crates/country-core/src/startup/run_game.rs');s=rd(p).replace('debug::sheep_controls::{SheepControl, sheep_controls}', 'debug::sheep_controls::sheep_controls').replace('    world.init_resource::<SheepControl>();\n','');wr(p,s)
print('facade, canonical owners and snapshot transfer candidate prepared')
