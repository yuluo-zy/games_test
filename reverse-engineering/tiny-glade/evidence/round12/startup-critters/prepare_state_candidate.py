from pathlib import Path
import json,hashlib,subprocess
O=Path(__file__).resolve().parent;R=Path('D:/game');C=O/'butterfly-candidate';C.mkdir(exist_ok=True);entries=[];S='tiny-glade/crates/systems/critters/src/'
def write(rel,s,base=None):
 p=C/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding='utf8');subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',str(p)],check=True);entries.append({'path':rel,'base_sha256':base,'proposed_sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
rel=S+'butterfly.rs';raw=(R/rel).read_bytes();s=raw.decode().replace('\r','');start=s.index('/// update_butterfly_states（');end=s.index('/// update_butterfly_goals（',start)
query='''/// Original exact query and mandatory canonical mapped-rock/water owners.
#[allow(clippy::too_many_arguments)]
pub fn update_butterfly_states(
    time: Res<GameTime>, camera: Res<MainCamera>, cursor: Res<CursorTerrainRaycast>,
    rng: Res<CritterRng>, water: Res<WaterRaster>, buffers: Res<AssetSsboLibrary>,
    mut query: Query<(&Transform,&Velocity,&mut Butterfly,&mut ButterflyAudio,&CritterGoal)>,
    sheep_query: Query<(&Transform,&Sheep)>,
) {
    let mut rng = rng.0.lock().unwrap();
    // Original lookup occurs even when the species query is empty.
    let rock = buffers.get::<TerrainRockMaskSsbo>().expect("update_butterfly_states requires TerrainRockMaskSsbo");
    let environment = LiveButterflyEnvironment {water:water.0.as_ref(),rock:rock.mapped};
    let sheep:Vec<_> = sheep_query.iter().map(|(t,_)|Vec2::new(t.translation.x,t.translation.z)).collect();
    let mode = camera.mode_byte();
    let eye = if mode==3 {camera.eye_position()} else {Vec3::ZERO};
    for (transform,velocity,mut butterfly,_audio,goal) in &mut query {
        crate::butterfly_state::update_state(&mut butterfly,transform.translation,velocity.0,goal.goal_tag&1!=0,
            time.delta,mode,eye,cursor.position(),&sheep,&environment,&mut || rng.uniform_f32());
    }
}
struct LiveButterflyEnvironment<'a> {
    water:country_core::resources::world_raster::RasterBufferRef<'a,512>,
    rock:&'a [TerrainRockMaskSsbo],
}
impl crate::butterfly_state::ButterflyEnvironment for LiveButterflyEnvironment<'_> {
    fn rock(&self,p:Vec2)->u32 {TerrainRockMaskSsbo::sample_mapped_at_world_xz(self.rock,p.x,p.y)}
    fn water(&self,p:Vec2)->f32 {self.water.bilinear_sample_at_world_xz(p.x,p.y,130.0,130.0)}
}

'''
s=s[:start]+query+s[end:];s=s.replace('use country_core::resources::terrain_rock_mask;','use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary, resources::{terrain_rock_mask::TerrainRockMaskSsbo,world_raster_defs::WaterRaster}};').replace('use country_core::resources::world_raster::WorldRaster;\n','').replace('use crate::shared::TerrainRockMask;\n','')
# Replace the stale historical header; exact recovered evidence lives with the candidate.
a=s.index('use bevy_ecs::prelude::*;');s='//! Butterfly policy from original author bytes; actual resource owners and all4 state handlers.\n//! State/velocity original fixture evidence: round12/startup-critters.\n'+s[a:]
write(rel,s,hashlib.sha256(raw).hexdigest())
rel=S+'lib.rs';raw=(R/rel).read_bytes();s=raw.decode().replace('\r','').replace('pub mod butterfly;','pub mod butterfly;\npub mod butterfly_state;');write(rel,s,hashlib.sha256(raw).hexdigest())
write(S+'butterfly_state.rs',(O/'butterfly_state_full.rs').read_text())
write(S+'tests/fixtures/butterfly_state_original_b.rs',(O/'butterfly_state_original_b.rs').read_text())
(C/'manifest.json').write_text(json.dumps(entries,indent=2),encoding='utf8');print('prepared canonical state owner candidate',len(entries))
