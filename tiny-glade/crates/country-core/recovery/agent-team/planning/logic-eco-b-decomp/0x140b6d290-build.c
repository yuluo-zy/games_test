
void __cdecl country_core::systems::garden::impl$0::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong uVar1;
  
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17h064e61c0de6b4877E();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>,bevy_app::main_schedule::Update,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>
            ();
  resources::world_raster::WorldRaster::new<512>();
  bevy_app::app::App::insert_resource<country_core::resources::world_raster_defs::GardenRaster>();
  uVar1 = 0x1dbf78e0d9cd349f;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>
              ();
  }
  uVar1 = 0x30caeddf11f1ff28;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::garden::GardenDrawEvent>();
  }
  uVar1 = 0xfbbe0392355af491;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 != 0) && (uVar1 < *(ulonglong *)(extraout_RAX + 0x218))) &&
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 != 0 &&
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
    return;
  }
  bevy_ecs::event::registry::EventRegistry::
  register_event<country_core::systems::garden::GardenEraseEvent>();
  return;
}

