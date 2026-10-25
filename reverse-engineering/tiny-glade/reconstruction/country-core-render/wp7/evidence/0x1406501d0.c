
void __cdecl system_water::impl$0::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong uVar1;
  
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17h31980271add316ceE();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>,bevy_app::main_schedule::Update,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  run_if<void_(*)(bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)>_>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(enum2_<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>_>_>)>,bool_(*)(enum2_<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>_>_>)>
            ();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  country_core::resources::world_raster::DirtyTilePublisher::unique();
  country_core::resources::world_raster_defs::PathRaster::new();
  bevy_app::app::App::insert_resource<country_core::resources::world_raster_defs::WaterRaster>();
  uVar1 = 0xcc3c0165e2715d9d;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_water::record_water_history::RecordWaterEditToHistoryCmd>();
  }
  uVar1 = 0xb050d5756cb9fc8f;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::water::WaterDrawEvent>();
  }
  uVar1 = 0x494426e4a24afc26;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 != 0) && (uVar1 < *(ulonglong *)(extraout_RAX + 0x218))) &&
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 != 0 &&
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
    return;
  }
  bevy_ecs::event::registry::EventRegistry::
  register_event<country_core::resources::water::WaterEraseEvent>();
  return;
}

