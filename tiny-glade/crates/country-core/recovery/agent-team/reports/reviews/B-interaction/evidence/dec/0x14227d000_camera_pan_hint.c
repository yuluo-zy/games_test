
void __cdecl system_onboarding::hint_camera_controls::camera_pan_hint(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  
  country_core::resources::steam::impl$30::from();
  hint_orchestrator::BindingHintDisplay::
  alternate<array_<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>,4>_>();
  if (*(short *)(in_RDX + 0x1c0) == 1) {
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
  else {
    if (*(short *)(in_RDX + 0x1c0) != 2) {
      hint_orchestrator::BindingHintDisplay::alternate<enum2_<bevy_input::mouse::MouseButton>_>();
      goto LAB_14227d23a;
    }
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
  local_98 = local_f8;
  uStack_90 = uStack_f0;
  local_88 = local_e8;
  uStack_80 = uStack_e0;
  local_78 = local_d8;
  uStack_70 = uStack_d0;
LAB_14227d23a:
  in_RCX[4] = local_78;
  in_RCX[5] = uStack_70;
  in_RCX[2] = local_88;
  in_RCX[3] = uStack_80;
  *in_RCX = local_98;
  in_RCX[1] = uStack_90;
  return;
}

