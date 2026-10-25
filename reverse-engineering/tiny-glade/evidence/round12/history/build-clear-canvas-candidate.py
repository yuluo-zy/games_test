"""Canvas consumer candidate only. Depends on the separately frozen water owner candidate."""
from pathlib import Path
import difflib,hashlib,json,subprocess
HERE=Path(__file__).resolve().parent;WORK=Path('D:/game');OUT=HERE/'clear-canvas-candidate'
changes={}
def add(relative,new):
    path=WORK/relative;old=path.read_text(encoding='utf8') if path.exists() else ''
    assert old!=new,relative
    changes[relative]=(old,new)

new='''//! Original canvas reset140a7abd0/6525B. Last unread command selects whether
//! the existing history/ID/terrain editing owner is retained for replay.
//! GPU operations use the same installed texture host and SSBO device.
use std::sync::Arc;
use bevy_ecs::prelude::*;
use crate::{
    asset_libraries::{ssbo_library::{AssetSsboLibrary,SsboType},
        texture_library::{AssetTextureLibrary,TextureLibraryDevice}},
    render::{device::GraphicsDeviceResource,ssbo::rhapsody_boundary::GraphicsDevice},
    resources::{color_id::*,history::{History,ReplayBuffer},terrain_editing::TerrainEditingData,
        texture_presets::TexturePreset,world_raster_defs::*,
        walls::{InnerWalls,WallHighlight,decorator_archivist::LiveDecoratorArchivist,
            curved_halftimber::CurvedHalfTimberDrawables,plaster::AnimatedPlasterPeel}},
    systems::{mode_manager::ClearCanvasCmd,wall::wall_state::wall_spatial_hash::WallSpatialHash,
        audio::water_ambience_audio::WaterAudioGrid},
};

/// Adapter to the actual existing TextureHost allocation owner. GraphicsDevice's
/// legacy TextureLibraryDevice impl is unsupported; never invoke that fallback.
#[derive(Resource,Clone)]
pub struct ClearCanvasTextureDevice(pub Arc<dyn TextureLibraryDevice+Send+Sync>);
impl ClearCanvasTextureDevice {
    pub fn new<T:TextureLibraryDevice+Send+Sync+'static>(host:Arc<T>)->Self {Self(host)}
}

/// Install exact empty-map startup owners without replacing any live state.
/// The caller supplies its existing real TextureHost Arc; no second device,
/// empty texture, fake handle or unsupported legacy fallback is constructed.
pub fn register_clear_canvas_resources<T:TextureLibraryDevice+Send+Sync+'static>(
    world:&mut World,texture_host:Arc<T>,
) {
    world.insert_resource(ClearCanvasTextureDevice::new(texture_host));
    world.init_resource::<CurvedHalfTimberDrawables>();
    world.init_resource::<AnimatedPlasterPeel>();
}

/// Native original parameter1 is one device; the current split host stores its
/// two existing interfaces as a tuple so the system keeps16 parameter groups.
#[allow(clippy::too_many_arguments,clippy::type_complexity)]
pub fn clear_canvas(
    devices:(Res<GraphicsDeviceResource>,Res<ClearCanvasTextureDevice>),
    mut walls:ResMut<InnerWalls>,mut spatial:ResMut<WallSpatialHash>,
    mut clear:EventReader<ClearCanvasCmd>,ssbos:Res<AssetSsboLibrary>,
    mut textures:ResMut<AssetTextureLibrary>,mut terrain:ResMut<TerrainEditingData>,
    (mut path,mut water,mut garden,mut cobble1,mut cobble2,mut cobble3):(
        ResMut<PathRaster>,ResMut<WaterRaster>,ResMut<GardenRaster>,
        ResMut<Cobblestone1Raster>,ResMut<Cobblestone2Raster>,ResMut<Cobblestone3Raster>),
    (mut wall_colors,mut gate_colors,mut timber_colors):(
        ResMut<WallColorIds>,ResMut<GateColorIds>,ResMut<HalftimberColorIds>),
    mut history:ResMut<History>,mut replay:ResMut<ReplayBuffer>,
    mut archive:ResMut<LiveDecoratorArchivist>,mut highlight:ResMut<WallHighlight>,
    mut water_audio:Option<ResMut<WaterAudioGrid>>,
    mut curved:ResMut<CurvedHalfTimberDrawables>,mut plaster:ResMut<AnimatedPlasterPeel>,
) {
    // The original drain selects the last unread event (new buffer preferred).
    let Some(preserve_history)=clear.read().last().map(|command|command.0) else {return};
    log::info!(target:"country_core::systems::clear_canvas","Clearing the canvas.");
    *archive=Default::default(); // native drop complete collision/archive owners, rebuild empty maps
    highlight.ty=0;             // retain the original id/entity payload
    plaster.clear();
    walls.remove_all();        // publish real removed IDs; do not discard notices
    if !preserve_history {walls.reset_max_id();}
    let texture_device=devices.1.0.as_ref();
    // Native interleaving matters: each CPU publication follows its GPU clear.
    textures.clear_preset_texture(texture_device,TexturePreset(0x13));water.clear_all();
    textures.clear_preset_texture(texture_device,TexturePreset(0x14));garden.clear_all();
    for preset in [6,5,7,1,2,3,4] {textures.clear_preset_texture(texture_device,TexturePreset(preset));}
    path.clear_all();cobble1.clear_all();cobble2.clear_all();cobble3.clear_all();
    curved.clear();
    *spatial=Default::default(); // native constructs new then drops/replaces old spatial owner
    if !preserve_history {*history=History::empty_glade();}
    *replay=ReplayBuffer::new(1.0); // drops every actual command/aux Arc, resets original counter
    *wall_colors=WallColorIds::new();
    reset_gpu_color_table::<WallColorIdSsbo>(devices.0.device(),&ssbos);
    *gate_colors=GateColorIds::new();
    reset_gpu_color_table::<GateColorIdSsbo>(devices.0.device(),&ssbos);
    *timber_colors=HalftimberColorIds::new();
    reset_gpu_color_table::<HalftimberColorIdSsbo>(devices.0.device(),&ssbos);
    if !preserve_history {
        terrain.strokes.clear();terrain.next_index=0;terrain.rasterized=false;
        // Native preserves cursor height override/unsubmitted UI state.
    }
    // Earlier texture operations use a shared borrow of ResMut; the native
    // explicitly writes this change tick only here, before the last four clears.
    textures.set_changed();
    for preset in [9,0xb,0xd,0xe] {textures.clear_preset_texture(texture_device,TexturePreset(preset));}
    if let Some(grid)=water_audio.as_deref_mut() {grid.clear_activity();}
}

fn reset_gpu_color_table<T:SsboType>(device:&dyn GraphicsDevice,ssbos:&AssetSsboLibrary) {
    let buffer=ssbos.get_buffer::<T>().expect("original color SSBO is required when clearing the canvas");
    let values=vec![ColorId::default_value().0;MAX_NUMBER_OF_WALLS];
    let bytes=bytemuck::cast_slice(&values);
    assert!(buffer.byte_size>=bytes.len() as u64,"original canvas color SSBO is too short");
    device.update_buffer_bytes(buffer,bytes,&[0..bytes.len()],0)
        .expect("original canvas color SSBO reset failed");
}
'''
add('tiny-glade/crates/country-core/src/systems/clear_canvas.rs',new)
relative='tiny-glade/crates/country-core/src/lib.rs';old=(WORK/relative).read_text(encoding='utf8')
add(relative,old.replace('    pub mod collision;','    pub mod collision;\n    pub mod clear_canvas;',1))

curved='''//! Original clear_canvas140a7b2da/2e6 clears two48B hash maps in order.
//! Callee140aaae80 and remove_wall_id140aa9380 prove both exact key/value types.
//! Field names describe their storage order; full geometry producer is separate.
use bevy_ecs::{resource::Resource,entity::Entity};
use std::collections::HashMap;
use super::WallId;

#[derive(Resource,Clone,Debug,Default)]
pub struct CurvedHalfTimberDrawables {
    pub primary:HashMap<WallId,Vec<Entity>>,
    pub secondary:HashMap<WallId,Vec<Entity>>,
}
impl CurvedHalfTimberDrawables {
    /// Retain table allocations/hash state; drop every actual owned entity list.
    /// This original clear does not queue despawns; entity lifecycle consumers
    /// remain responsible for the native remove_wall_id+Commands policy.
    pub fn clear(&mut self) {self.primary.clear();self.secondary.clear();}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canvas_clear_drops_nonempty_lists_keeps_table_capacity_and_never_despawns() {
        let mut world=bevy_ecs::world::World::new();
        let first=world.spawn_empty().id();let second=world.spawn_empty().id();
        let mut owner=CurvedHalfTimberDrawables::default();
        owner.primary.insert(WallId(7),vec![first,second]);
        owner.secondary.insert(WallId(19),vec![second]);
        let capacity=(owner.primary.capacity(),owner.secondary.capacity());
        owner.clear();
        assert!(owner.primary.is_empty());assert!(owner.secondary.is_empty());
        assert_eq!((owner.primary.capacity(),owner.secondary.capacity()),capacity);
        assert!(world.get_entity(first).is_ok());assert!(world.get_entity(second).is_ok());
    }
}
'''
add('tiny-glade/crates/country-core/src/resources/walls/curved_halftimber.rs',curved)

relative='tiny-glade/crates/country-core/src/resources/walls/plaster.rs';old=(WORK/relative).read_text(encoding='utf8')
before='''/// 石膏剥离动画资源（O：`Res<AnimatedPlasterPeel>`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimatedPlasterPeel(pub Gap);'''
after='''/// Native producer1411fcad0 inserts WallId→PlasterPeelAmount; Nani
/// consumer14207b290 reads its f32, falling back to the actual wall's peel.
#[repr(transparent)]
#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct PlasterPeelAmount(pub f32);

#[derive(Resource,Clone,Debug,Default)]
pub struct AnimatedPlasterPeel(pub std::collections::HashMap<WallId,PlasterPeelAmount>);
impl AnimatedPlasterPeel {
    /// Native clear_canvas140a7b102..157 clears the actual RawTable and keeps
    /// its allocation. No fabricated default wall/peel entry is inserted.
    pub fn clear(&mut self) {self.0.clear();}
    pub fn get(&self,wall:WallId)->Option<f32> {self.0.get(&wall).map(|amount|amount.0)}
}

#[cfg(test)]
mod animated_peel_owner_tests {
    use super::*;
    #[test]
    fn canvas_clear_uses_real_nonempty_amount_owner_and_keeps_allocation() {
        let mut peel=AnimatedPlasterPeel::default();
        peel.0.insert(WallId(7),PlasterPeelAmount(0.625));
        peel.0.insert(WallId(9),PlasterPeelAmount(f32::from_bits(0x7fc01234)));
        assert_eq!(peel.get(WallId(7)),Some(0.625));
        assert_eq!(peel.get(WallId(9)).unwrap().to_bits(),0x7fc01234);
        let capacity=peel.0.capacity();peel.clear();
        assert!(peel.0.is_empty());assert_eq!(peel.0.capacity(),capacity);
        assert_eq!(peel.get(WallId(7)),None);
    }
}'''
assert before in old
add(relative,old.replace(before,after))

manifest={}
for relative,(old,new) in changes.items():
    path=OUT/relative;path.parent.mkdir(parents=True,exist_ok=True);path.write_text(new,encoding='utf8')
    subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe',
        '--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],check=True,stdout=subprocess.DEVNULL)
    source=WORK/relative
    manifest[relative]={'base_sha256':hashlib.sha256(source.read_bytes()).hexdigest() if source.exists() else None,
        'candidate_sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
patch=''.join(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),
    fromfile='a/'+relative if old else '/dev/null',tofile='b/'+relative)) for relative,(old,new) in changes.items())
(HERE/'clear-canvas-candidate.patch').write_text(patch,encoding='utf8')
(HERE/'clear-canvas-candidate-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(json.dumps({'candidate_files':len(changes),'production_files_changed':0,'depends_on':'startup-unload-owner-candidate.patch water owner','execution':'rustfmt parsing only; no Cargo'},indent=2))
