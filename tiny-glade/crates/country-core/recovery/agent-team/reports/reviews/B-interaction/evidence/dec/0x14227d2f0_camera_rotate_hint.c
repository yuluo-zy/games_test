
void __cdecl system_onboarding::hint_camera_controls::camera_rotate_hint(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong in_RDX;
  
  country_core::resources::steam::impl$31::from();
  country_core::resources::steam::impl$30::from();
  hint_orchestrator::BindingHintDisplay::
  together<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
  if (*(short *)(in_RDX + 0x1bc) == 1) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_00 = &DAT_1430385c4;
    extraout_RAX_00[1] = 10;
    extraout_RAX_00[2] = &DAT_1430385ce;
    extraout_RAX_00[3] = 8;
    extraout_RAX_00[4] = &DAT_1430385d6;
    extraout_RAX_00[5] = 0xd;
    hint_orchestrator::BindingHintDisplay::
    alternate<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
  }
  else if (*(short *)(in_RDX + 0x1bc) == 2) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = &DAT_1430385c4;
    extraout_RAX[1] = 10;
    extraout_RAX[2] = &DAT_1430385e3;
    extraout_RAX[3] = 8;
    extraout_RAX[4] = &DAT_1430385eb;
    extraout_RAX[5] = 0xd;
    hint_orchestrator::BindingHintDisplay::
    alternate<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
  }
  else {
    hint_orchestrator::BindingHintDisplay::alternate<enum2_<bevy_input::mouse::MouseButton>_>();
  }
  hint_orchestrator::BindingHintDisplay::
  adjacent<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
  hint_orchestrator::BindingHintDisplay::
  alternate<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
  return;
}

