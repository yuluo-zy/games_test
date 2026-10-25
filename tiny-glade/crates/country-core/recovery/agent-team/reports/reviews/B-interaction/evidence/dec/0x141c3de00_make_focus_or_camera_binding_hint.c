
void __cdecl system_onboarding::hint_clutter_focus_edit::make_focus_or_camera_binding_hint(void)

{
  undefined2 extraout_AX;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined4 local_58;
  undefined2 uStack_54;
  undefined2 uStack_52;
  undefined6 uStack_50;
  undefined2 uStack_4a;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  
  country_core::resources::steam::impl$31::from();
  local_58 = 0xc9;
  if (*(short *)(in_RDX + 0x1c4) == 1) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_00 = &anon_5217b8c3a73ed4d6f627c21e189f8089_4_llvm_12955430627034200511;
    extraout_RAX_00[1] = 10;
    extraout_RAX_00[2] = &anon_5217b8c3a73ed4d6f627c21e189f8089_5_llvm_12955430627034200511;
    extraout_RAX_00[3] = 8;
    hint_orchestrator::BindingHintDisplay::
    alternate<system_onboarding::hint_orchestrator::HintGraphicParams>();
  }
  else {
    if (*(short *)(in_RDX + 0x1c4) != 2) {
      hint_orchestrator::BindingHintDisplay::alternate<enum2_<bevy_input::mouse::MouseButton>_>();
      uStack_54 = extraout_AX;
      goto LAB_141c3dfd5;
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = &anon_5217b8c3a73ed4d6f627c21e189f8089_4_llvm_12955430627034200511;
    extraout_RAX[1] = 10;
    extraout_RAX[2] = "hint/mmb";
    extraout_RAX[3] = 8;
    hint_orchestrator::BindingHintDisplay::
    alternate<system_onboarding::hint_orchestrator::HintGraphicParams>();
  }
  local_58 = (undefined4)local_f0;
  uStack_54 = (undefined2)((ulonglong)local_f0 >> 0x20);
  uStack_52 = (undefined2)((ulonglong)local_f0 >> 0x30);
  uStack_50 = (undefined6)uStack_e8;
  uStack_4a = (undefined2)((ulonglong)uStack_e8 >> 0x30);
  local_48 = local_e0;
  uStack_40 = uStack_d8;
  local_38 = local_d0;
  uStack_30 = uStack_c8;
LAB_141c3dfd5:
  in_RCX[4] = local_38;
  in_RCX[5] = uStack_30;
  in_RCX[2] = local_48;
  in_RCX[3] = uStack_40;
  *in_RCX = CONCAT26(uStack_52,CONCAT24(uStack_54,local_58));
  in_RCX[1] = CONCAT26(uStack_4a,uStack_50);
  return;
}

