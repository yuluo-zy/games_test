//! Original canvas reset140a7abd0/6525B. Last unread command selects whether
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
