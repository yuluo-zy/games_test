// name=add_roof_systems
// sig=void __cdecl add_roof_systems(void)
// size=1878

void __cdecl system_roof::impl$0::add_roof_systems(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RCX;
  ulonglong uVar1;
  
  uVar1 = 0x44a43d15e25ff344;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_roof::ui_edit_roof::DoUiEditRoof>();
  }
  uVar1 = 0x9891b2b949bee27b;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>();
  }
  uVar1 = 0x5579454b4f99baa1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>();
  }
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17hf4310f9d2856bb43E();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hc932a18ae45b00f2E();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
            ();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hc46b5c7bf02ecd8bE();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs5chain17h772a1a7c57f8f319E();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs5chain17h07f5b3d4e7739780E();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
            ();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17hb5afb6add6220049E();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::system::query::Query<tuple_<bevy_ecs::entity::Entity,ref_<country_core::resources::roofs::roof_shape::Roof>,enum2_<core::option::Option<ref_<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>_>_>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,tuple_<>_>,bevy_ecs::system::commands::Commands),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::system::query::Query<tuple_<bevy_ecs::entity::Entity,ref_<country_core::resources::roofs::roof_shape::Roof>,enum2_<core::option::Option<ref_<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>_>_>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,tuple_<>_>,bevy_ecs::system::commands::Commands)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>_>,country_core::systems::GameSystemSet>
            ();
  _ZN8bevy_app7sub_app6SubApp11add_systems17h13c01fab9669cc98E();
  return;
}

