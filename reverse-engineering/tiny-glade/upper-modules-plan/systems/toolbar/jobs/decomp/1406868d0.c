// 0x1406868d0 build
// signature: void __cdecl build(void)

void __cdecl toolbar::mei::core::impl$23::build(void)

{
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  byte *extraout_RAX_01;
  longlong extraout_RAX_02;
  byte *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong lVar1;
  byte *extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong lVar2;
  longlong in_RDX;
  ulonglong uVar3;
  
  lVar2 = in_RDX;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(longlong *)(extraout_RAX + 0x10) = lVar2;
    lVar1 = extraout_RAX_00;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX + 8);
    lVar2 = *(longlong *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar1 + 1;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_01 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_01[0] = 1;
    extraout_RAX_01[1] = 0;
    extraout_RAX_01[2] = 0;
    extraout_RAX_01[3] = 0;
    extraout_RAX_01[4] = 0;
    extraout_RAX_01[5] = 0;
    extraout_RAX_01[6] = 0;
    extraout_RAX_01[7] = 0;
    *(longlong *)(extraout_RAX_01 + 8) = extraout_RAX_02;
    *(longlong *)(extraout_RAX_01 + 0x10) = lVar2;
    lVar1 = extraout_RAX_02;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX_01 + 8);
    lVar2 = *(longlong *)(extraout_RAX_01 + 0x10);
  }
  *(longlong *)(extraout_RAX_01 + 8) = lVar1 + 1;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_03 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_03[0] = 1;
    extraout_RAX_03[1] = 0;
    extraout_RAX_03[2] = 0;
    extraout_RAX_03[3] = 0;
    extraout_RAX_03[4] = 0;
    extraout_RAX_03[5] = 0;
    extraout_RAX_03[6] = 0;
    extraout_RAX_03[7] = 0;
    *(longlong *)(extraout_RAX_03 + 8) = extraout_RAX_04;
    *(longlong *)(extraout_RAX_03 + 0x10) = lVar2;
    lVar1 = extraout_RAX_04;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX_03 + 8);
    lVar2 = *(longlong *)(extraout_RAX_03 + 0x10);
  }
  *(longlong *)(extraout_RAX_03 + 8) = lVar1 + 1;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_05 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_05[0] = 1;
    extraout_RAX_05[1] = 0;
    extraout_RAX_05[2] = 0;
    extraout_RAX_05[3] = 0;
    extraout_RAX_05[4] = 0;
    extraout_RAX_05[5] = 0;
    extraout_RAX_05[6] = 0;
    extraout_RAX_05[7] = 0;
    *(longlong *)(extraout_RAX_05 + 8) = extraout_RAX_06;
    *(longlong *)(extraout_RAX_05 + 0x10) = lVar2;
    lVar2 = extraout_RAX_06;
  }
  else {
    lVar2 = *(longlong *)(extraout_RAX_05 + 8);
  }
  *(longlong *)(extraout_RAX_05 + 8) = lVar2 + 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_07 != (undefined8 *)0x0) {
    ::core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_08 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    lVar2 = *extraout_RAX_08;
    lVar1 = extraout_RAX_08[1];
    *extraout_RAX_08 = *extraout_RAX_08 + 1;
    *extraout_RAX_07 = 1;
    extraout_RAX_07[1] = 1;
    extraout_RAX_07[2] = &anon_701f5fe71f189fe5a87bf7990fc988a4_3_llvm_7721286249138486557;
    extraout_RAX_07[3] = 0;
    extraout_RAX_07[4] = 0;
    extraout_RAX_07[5] = 0;
    extraout_RAX_07[6] = lVar2;
    extraout_RAX_07[7] = lVar1;
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    country_core::input::action_set::impl$0::default();
    bevy_app::app::App::insert_resource<toolbar::mei::core::MeiContext>();
    uVar3 = 0x8e1bd251962ba784;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_09 != 0) && (uVar3 < *(ulonglong *)(in_RDX + 0x218))) &&
       (uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar3 * 8), uVar3 != 0)) {
      lVar2 = ~uVar3 * 0x60;
      if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar2) != 0) {
        *(undefined4 *)(*(longlong *)(in_RDX + 0x1e0) + lVar2 + 0x5c) =
             *(undefined4 *)(in_RDX + 0x5b0);
        bevy_ecs::schedule::schedule::Schedules::
        add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)>_>,bevy_app::main_schedule::PreStartup,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)>
                  ();
        bevy_app::sub_app::SubApp::
        add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)>_>,bevy_app::main_schedule::Startup,void_(*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)>
                  ();
        _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hc1f044b998f0ad5cE
                  ();
        bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
        bevy_ecs::schedule::config::ScheduleConfigs::
        after_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                  ();
        bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
        bevy_ecs::schedule::config::ScheduleConfigs::
        in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                  ();
        bevy_app::sub_app::SubApp::
        add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
                  ();
        _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs5chain17hd5dd4cd817d58f42E();
        bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
        bevy_ecs::schedule::config::ScheduleConfigs::
        in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                  ();
        bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
        bevy_ecs::schedule::config::ScheduleConfigs::
        after_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                  ();
        bevy_app::sub_app::SubApp::
        add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
                  ();
        return;
      }
    }
                    // WARNING: Subroutine does not return
    ::core::panicking::panic_fmt();
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

