//! Complete rotate consumer of the canonical affordance payload.
//! Original B1407a8580 (3596B, including cold tails), table142a82be8.
//! Native instruction fixtures retain the original112B requests/audio payloads.
use bevy_ecs::prelude::*;
use country_core::{
    resources::{mouse_terrain_raycast::CameraCursorRay, read_mouse_depth_value::MouseDepthValue,
        terrain_height::{TerrainHeightSsbo,TerrainHeightsData}, time::GameTime, window::WindowSize,
        walls::{cached_decorator_transforms::CachedDecoratorTransforms,
            decorator::{DecoratorDst,DecoratorId,DecoratorInfo,DecoratorRotationXZ,DecoratorType,DeferredDecoratorOpEvents},
            decorator_archivist::DecoratorArchivist, decorator_storage::DecoratorStorage}},
    systems::{audio::PlayDecoratorSoundCmd, camera_rig::MainCamera,
        ui::{resources::ui_state::UiState,signifiers::signifier_create::UiSignifierStream}},
    utils::debug_viz::{DebugLine,DebugSphere},
};
use glam::{Vec2,Vec3};
use crate::{apply_decorator_op::{apply_decorator_op,DecoratorChangePatch,DecoratorOp},
    backup::DecoratorBackup,deferred_decorator_history::DeferredDecoratorEditHistory};
use super::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;

/// Original seven f32 Local fields. All persist across frames of a drag.
#[derive(Default,Debug)]
pub struct EditingState {
    pub initial_cursor: Vec3,
    pub plane_height: f32,
    pub initial_angle: f32,
    pub initial_yaw: f32,
    pub latest_yaw: f32,
}

/// Native phase2..4 maps to0..2; both cancel0 and commit1 use phase3.
/// A continuous drag is raw4, not a release (the old reconstruction reversed it).
pub fn rotation_phase(raw: u8) -> u8 {
    if raw.wrapping_sub(2) < 3 {raw.wrapping_sub(2)} else {3}
}

/// Preserve the complete canonical destination and replace only its yaw.
pub fn rotated_destination(destination: DecoratorDst, yaw: DecoratorRotationXZ) -> DecoratorDst {
    match destination {
        DecoratorDst::Roof(mut value) => {value.yaw=yaw;DecoratorDst::Roof(value)}
        DecoratorDst::Terrain(mut value) => {value.yaw=yaw;DecoratorDst::Terrain(value)}
        DecoratorDst::Stair(mut value) => {value.yaw=yaw;DecoratorDst::Stair(value)}
        DecoratorDst::Wall(_) | DecoratorDst::WallCompound(_) => unreachable!("cannot rotate a wall decorator"),
    }
}

pub fn rotation_audio_kind(ty: DecoratorType) -> u8 {
    let family=if ty.0.wrapping_sub(0x38)<8 {ty.0.wrapping_sub(0x38)} else {6};
    match family {4=>0xd,6=>0x22,_=>0x18}
}

#[allow(clippy::too_many_arguments)]
pub fn ui_rotate_decorator<C: Send+Sync+'static>(
    ui: Res<UiState>, mut stream: ResMut<UiSignifierStream>, mut local: Local<EditingState>,
    ray: Res<CameraCursorRay>, terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    transforms: Res<CachedDecoratorTransforms>, dispatcher: Res<DecoratorAffordanceDispatcher>,
    mut audio: EventWriter<PlayDecoratorSoundCmd>, time: Res<GameTime>,
    (mut storage,mut archivist,events): (ResMut<DecoratorStorage>,ResMut<DecoratorArchivist<C>>,Res<DeferredDecoratorOpEvents>),
    (camera,window,depth): (Res<MainCamera>,Res<WindowSize>,Res<MouseDepthValue>),
    mut history: ResMut<DeferredDecoratorEditHistory>,mut backup: ResMut<DecoratorBackup>,
) {
    let Some(request)=dispatcher.rotation() else {return};
    let context=ui.widget_context(request.widget);
    if !context.is_active() {return}
    let raw=context.dragged();
    if raw<2 {history.pending=true;}
    let addr=storage.lookup_storage(request.decorator_id).expect("rotate decorator storage missing");
    let info=*storage.get(addr).and_then(|bucket|bucket.get(request.decorator_id)).expect("rotate decorator info missing");
    let destination=info.destination();
    let (mut center,current_yaw)=match destination {
        DecoratorDst::Roof(value)=>(transforms.transform(request.decorator_id.0).translation,value.yaw),
        DecoratorDst::Terrain(value)=>(Vec3::new(value.position_xz.x,terrain.sample_at_world_xz(value.position_xz.x,value.position_xz.y),value.position_xz.y),value.yaw),
        DecoratorDst::Stair(value)=>(transforms.transform(request.decorator_id.0).translation,value.yaw),
        DecoratorDst::Wall(_) | DecoratorDst::WallCompound(_)=>unreachable!("cannot rotate a wall decorator"),
    };
    if request.ty.0&0x3e==0x3c {DebugSphere::new(center).solid().show(&mut stream);}
    center.y+=request.amount;
    match rotation_phase(raw) {
        0=>{local.latest_yaw=current_yaw.0;}
        1=>{
            let cached_y=transforms.position(request.decorator_id.0).y;
            local.plane_height=if cached_y<=0.0 {0.0} else {cached_y};
            local.initial_angle=DecoratorRotationXZ::from_xz_forward_vector(Vec2::new(request.hit.position.x-center.x,request.hit.position.z-center.z)).0;
            local.initial_cursor=ray.intersect_with_xz_plane(local.plane_height);
            local.initial_yaw=current_yaw.0;
            local.latest_yaw=current_yaw.0;
        }
        2|3=>{
            if raw<2 {backup.reset_after=true;}
            let cursor=ray.intersect_with_xz_plane(local.plane_height);
            let dx=local.initial_cursor.x-cursor.x;
            let dz=local.initial_cursor.z-cursor.z;
            let scale=utils::geometry::ws_to_ss(depth.view_depth,camera.read_matrix(0),window.height as f32);
            if scale*(dz*dz+dx*dx).sqrt()<2.0 && raw<2 && request.ty.can_change_seed() {
                let mut patch=DecoratorChangePatch::rotation(request.decorator_id,addr,destination,transforms.position(request.decorator_id.0));
                patch.destination=None;
                patch.word_2c_delta=Some(1);
                apply_decorator_op(&DecoratorOp::from_change(addr,patch),u8::from(raw>1),addr,&mut storage,Some(&mut *archivist),Some(&mut *history),&events);
            } else {
                let angle=DecoratorRotationXZ::from_xz_forward_vector(Vec2::new(cursor.x-center.x,cursor.z-center.z));
                DebugLine::new(vec![cursor,Vec3::new(center.x,local.plane_height,center.z)]).show(&mut stream);
                let yaw=angle.offset(local.initial_yaw-local.initial_angle);
                let patch=DecoratorChangePatch::rotation(request.decorator_id,addr,rotated_destination(destination,yaw),center);
                apply_decorator_op(&DecoratorOp::from_change(addr,patch),u8::from(raw>1),addr,&mut storage,Some(&mut *archivist),Some(&mut *history),&events);
                local.latest_yaw=yaw.0;
                let position_context=match addr {country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(_)=>0,country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Terrain=>1,country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Stair=>2};
                audio.write(PlayDecoratorSoundCmd {kind:rotation_audio_kind(request.ty),ratio:(current_yaw.0-yaw.0).abs()/time.delta,position_context,position:cursor,flag:false});
            }
        }
        _=>unreachable!(),
    }
    show_rotation_signifiers(request.decorator_id,center,local.latest_yaw,&mut stream);
}

/// Keep the two original meshes in one owned factory (the original names are
/// identical), so keyed factory ownership cannot replace the first mesh with
/// the outline closure. Both drawables retain their own pass selection.
fn show_rotation_signifiers(id: DecoratorId,position:Vec3,yaw:f32,stream:&mut UiSignifierStream) {
    use country_core::{components::drawable::DrawableMeshBundle,
        systems::{render_loop::draw_passes::draw_lists::{DontCastShadow,UiPass},
            ui::signifiers::{animation::SignifierAnimation,SignifierFactory}},
        utils::components::transform::Transform};
    let angle=(yaw+std::f32::consts::PI)*0.5;
    let sine=angle.sin();
    let rotation=glam::Quat::from_xyzw(sine*0.0,sine,sine*0.0,angle.cos());
    let name=format!("lantern_rotate_{id}{position:?}{rotation:?}");
    let factory=SignifierFactory::from_closure(&name,std::sync::Arc::new(move|meshes,shaders,commands| {
        let shader=shaders.get_raster_handle_by_name("vertex_color_shader").expect("rotation signifier shader missing");
        [("corner_arrow",false),("corner_arrow_outline",true)].into_iter().map(|(mesh_name,ui_pass)| {
            let mesh=meshes.get_handle_by_name(mesh_name);
            let mut entity=commands.spawn((DrawableMeshBundle{mesh,shader,transform:Transform{rotation,translation:position,scale:Vec3::ZERO}},DontCastShadow,SignifierAnimation{flag_8:0,flag_9:1,value:0.3,duration:0.0,..Default::default()}));
            if ui_pass {entity.insert(UiPass);}
            entity.id()
        }).collect()
    }));
    stream.write_factory(&factory);
}
