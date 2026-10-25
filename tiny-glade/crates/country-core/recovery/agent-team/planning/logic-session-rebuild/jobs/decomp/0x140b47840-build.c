// 0x140b47840 build
// signature: void __cdecl build(void)
// callees: [{"name": "add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>)>_>,bevy_app::main_schedule::Startup,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>)>", "address": "140b66eb0"}, {"name": "add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<>,tuple$<>_>,bevy_app::main_schedule::Update,tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<>_>,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<>_>,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>", "address": "140b679c0"}, {"name": "get_resource_id", "address": "1400dd6f0"}, {"name": "insert_resource<country_core::resources::session::NextSession>", "address": "141543b60"}, {"name": "insert_resource<country_core::resources::session::SavedSessionList>", "address": "14154ad00"}, {"name": "into_configs<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>)>_>", "address": "1413280a0"}, {"name": "memcpy", "address": "142923c00"}, {"name": "or<bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>)>,tuple$<>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>", "address": "1415650e0"}, {"name": "register_event<country_core::resources::session::NewSessionStartedCmd>", "address": "141516b30"}, {"name": "register_event<country_core::resources::session::SessionLoadedCmd>", "address": "141510e60"}, {"name": "register_event<country_core::systems::sav


void __cdecl country_core::resources::session::impl$10::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong uVar1;
  undefined1 local_930 [1096];
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined8 uStack_4d0;
  undefined8 local_4c8;
  undefined8 uStack_4c0;
  undefined8 local_4b8;
  undefined8 uStack_4b0;
  undefined8 local_4a8;
  undefined8 uStack_4a0;
  undefined8 local_498;
  undefined8 uStack_490;
  undefined8 local_488;
  undefined8 uStack_480;
  undefined8 local_478;
  undefined8 uStack_470;
  undefined8 local_468;
  undefined8 uStack_460;
  undefined8 local_458;
  ulonglong uStack_450;
  undefined8 local_448;
  undefined8 uStack_440;
  undefined8 local_438;
  undefined8 uStack_430;
  undefined8 local_428;
  undefined8 uStack_420;
  undefined8 local_418;
  undefined8 uStack_410;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_4e8 = 0;
  uStack_4e0 = 4;
  local_4d8 = 0;
  bevy_app::app::App::insert_resource<country_core::resources::session::SavedSessionList>();
  uStack_4d0 = 0x8000000000000001;
  local_4e8 = 0;
  uStack_4e0 = 8;
  local_4d8 = 0;
  uStack_450 = uStack_450 & 0xffffffffffffff00;
  bevy_app::app::App::insert_resource<country_core::resources::session::NextSession>();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2_<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>)>_>,bevy_app::main_schedule::Startup,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2_<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>)>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  run_if<void_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>_>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>
            ();
  bevy_ecs::schedule::condition::Condition::
  or<bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>),tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>)>,tuple_<>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>
            ();
  local_21 = 1;
  bevy_ecs::schedule::config::impl$4::
  into_configs<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2_<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>),tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2_<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>_>_>)>_>
            ();
  memcpy(&local_4e8,local_930,0x3d8);
  local_21 = 0;
  bevy_ecs::schedule::config::impl$3::
  run_if<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<>,bevy_ecs::system::combinator::CombinatorSystem<bevy_ecs::schedule::condition::OrMarker,bevy_ecs::system::function_system::FunctionSystem<bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>),bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>)>,bevy_ecs::system::function_system::FunctionSystem<bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>),bool_(*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)>_>_>
            ();
  local_4e8 = local_98;
  uStack_4e0 = uStack_90;
  local_4d8 = local_88;
  uStack_4d0 = uStack_80;
  local_4c8 = local_78;
  uStack_4c0 = uStack_70;
  local_4b8 = local_68;
  uStack_4b0 = uStack_60;
  local_4a8 = local_58;
  uStack_4a0 = uStack_50;
  local_498 = local_48;
  uStack_490 = uStack_40;
  local_488 = local_38;
  uStack_480 = uStack_30;
  local_478 = local_108;
  uStack_470 = uStack_100;
  local_468 = local_f8;
  uStack_460 = uStack_f0;
  local_458 = local_e8;
  uStack_450 = uStack_e0;
  local_448 = local_d8;
  uStack_440 = uStack_d0;
  local_438 = local_c8;
  uStack_430 = uStack_c0;
  local_428 = local_b8;
  uStack_420 = uStack_b0;
  local_418 = local_a8;
  uStack_410 = uStack_a0;
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>,bevy_app::main_schedule::Update,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>
            ();
  uVar1 = 0x2e360f7300b870d3;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::session::NewSessionStartedCmd>();
  }
  uVar1 = 0x8f52d704f62ff44b;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::session::SessionLoadedCmd>();
  }
  uVar1 = 0xc91955e8a4b3eed9;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 != 0) && (uVar1 < *(ulonglong *)(extraout_RAX + 0x218))) &&
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 != 0 &&
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
    return;
  }
  bevy_ecs::event::registry::EventRegistry::
  register_event<country_core::systems::save::save_game::DeleteSessionCmd>();
  return;
}

