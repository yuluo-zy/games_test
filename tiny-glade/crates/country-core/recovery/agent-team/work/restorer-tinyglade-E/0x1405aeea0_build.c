
void __cdecl tiny_glade::run_game::impl$2::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong in_RDX;
  ulonglong uVar1;
  
  uVar1 = 0x500f3099f5bff7ba;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::register_event<bevy_input::keyboard::KeyboardInput>();
  }
  uVar1 = 0x5818bdd4cf5f5554;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<bevy_input::keyboard::KeyboardFocusLost>();
  }
  bevy_app::app::App::
  init_resource<bevy_input::button_input::ButtonInput<enum2_<bevy_input::keyboard::KeyCode>_>_>();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2_<bevy_input::keyboard::KeyCode>_>_>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardFocusLost>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2_<bevy_input::keyboard::KeyCode>_>_>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardFocusLost>)>_>,bevy_input::InputSystem>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::PreUpdate,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  uVar1 = 0xe71eaa88577fee7a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 == 0) || (*(ulonglong *)(extraout_RAX_01 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_01 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_01 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::register_event<bevy_input::mouse::MouseButtonInput>();
  }
  uVar1 = 0x2cae11c106c4365a;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_03 == 0) || (*(ulonglong *)(extraout_RAX_01 + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_01 + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX_01 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::register_event<bevy_input::mouse::MouseMotion>();
  }
  uVar1 = 0x2288607094c8a33c;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(extraout_RAX_01 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_01 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_01 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::register_event<bevy_input::mouse::MouseWheel>();
  }
  bevy_app::app::App::
  init_resource<bevy_input::button_input::ButtonInput<enum2_<bevy_input::mouse::MouseButton>_>_>();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<void_(*)(bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2_<bevy_input::mouse::MouseButton>_>_>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseButtonInput>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2_<bevy_input::mouse::MouseButton>_>_>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseButtonInput>)>_>,bevy_input::InputSystem>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::PreUpdate,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  return;
}

