
void __cdecl country_core::systems::main_camera::impl$3::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong uVar1;
  undefined1 local_288 [272];
  undefined1 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined1 local_108 [120];
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
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::event::writer::EventWriter<country_core::systems::main_camera::MainCameraMoved>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::event::writer::EventWriter<country_core::systems::main_camera::MainCameraMoved>)>_>,country_core::systems::GameSystemSet>
            ();
  bevy_ecs::schedule::config::impl$3::
  run_if<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>)>,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>)>
            ();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  before_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  run_if<void_(*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)>_>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>
            ();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6run_if17h3960b262e123b10eE();
  bevy_ecs::schedule::condition::common_conditions::
  not<tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>,bool,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  run_if<void_(*)(bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>_>,tuple_<>,bevy_ecs::system::adapter_system::AdapterSystem<bevy_ecs::schedule::condition::NotMarker,bevy_ecs::system::function_system::FunctionSystem<bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>),bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>_>_>
            ();
  uStack_90 = uStack_170;
  local_88 = local_168;
  uStack_80 = uStack_160;
  local_78 = local_158;
  uStack_70 = uStack_150;
  local_68 = local_148;
  uStack_60 = uStack_140;
  local_58 = local_138;
  uStack_50 = uStack_130;
  local_48 = local_128;
  uStack_40 = uStack_120;
  local_38 = local_118;
  uStack_30 = uStack_110;
  memcpy(local_288,local_108,0xe0);
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  chain_ignore_deferred<tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>,alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>_>_>
            ();
  local_178 = 9;
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  bevy_app::app::App::insert_resource<country_core::systems::main_camera::CameraSnapshot>();
  uVar1 = 0x80c70da6101ec1a;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 != 0) && (uVar1 < *(ulonglong *)(extraout_RAX + 0x218))) &&
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 != 0)) &&
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)) {
    return;
  }
  bevy_ecs::event::registry::EventRegistry::
  register_event<country_core::systems::main_camera::MainCameraMoved>();
  return;
}

