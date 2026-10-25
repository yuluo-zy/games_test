// widget  size=1445
// void __cdecl widget(void)

void __cdecl
country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget(void)

{
  undefined4 uVar1;
  longlong lVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar3;
  longlong in_RCX;
  ulonglong uVar4;
  longlong in_R8;
  longlong lVar5;
  ulonglong uVar6;
  undefined4 uVar7;
  float fVar8;
  char in_stack_00000028;
  
  lVar3 = *(longlong *)(in_R8 + 0xe0);
  uVar4 = *(ulonglong *)(in_R8 + 0xe8);
  if (*(char *)(in_RCX + 9) == '\0') {
    uVar6 = *(ulonglong *)(in_R8 + 0x98);
    if (uVar4 < *(ulonglong *)(in_R8 + 0x98)) {
      uVar6 = uVar4;
    }
    if (0xaaaaaaaaaaaaaaa < uVar6) {
LAB_140956c42:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (uVar6 != 0) {
      lVar2 = *(longlong *)(in_R8 + 0x90);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == 0) goto LAB_140956c42;
      if (uVar6 == 1) {
        uVar4 = 0;
      }
      else {
        lVar5 = 5;
        uVar4 = 0;
        do {
          uVar7 = *(undefined4 *)(lVar2 + -0xc + lVar5 * 4);
          fVar8 = *(float *)(lVar3 + uVar4 * 4);
          *(undefined4 *)(extraout_RAX_00 + -0x14 + lVar5 * 4) =
               *(undefined4 *)(lVar2 + -0x14 + lVar5 * 4);
          *(float *)(extraout_RAX_00 + -0x10 + lVar5 * 4) = fVar8 + 0.6;
          *(undefined4 *)(extraout_RAX_00 + -0xc + lVar5 * 4) = uVar7;
          uVar7 = *(undefined4 *)(lVar2 + lVar5 * 4);
          fVar8 = *(float *)(lVar3 + 4 + uVar4 * 4);
          uVar4 = uVar4 + 2;
          *(undefined4 *)(extraout_RAX_00 + -8 + lVar5 * 4) =
               *(undefined4 *)(lVar2 + -8 + lVar5 * 4);
          *(float *)(extraout_RAX_00 + -4 + lVar5 * 4) = fVar8 + 0.6;
          *(undefined4 *)(extraout_RAX_00 + lVar5 * 4) = uVar7;
          lVar5 = lVar5 + 6;
        } while ((uVar6 & 0xffffffffffffffe) != uVar4);
      }
      if ((uVar6 & 1) != 0) {
        lVar5 = uVar4 * 0xc;
        fVar8 = *(float *)(lVar3 + uVar4 * 4);
        uVar7 = *(undefined4 *)(lVar2 + 8 + lVar5);
        *(undefined4 *)(extraout_RAX_00 + lVar5) = *(undefined4 *)(lVar2 + lVar5);
        *(float *)(extraout_RAX_00 + 4 + lVar5) = fVar8 + 0.6;
        *(undefined4 *)(extraout_RAX_00 + 8 + lVar5) = uVar7;
      }
    }
  }
  else {
    lVar2 = *(longlong *)(in_R8 + 0x90);
    uVar6 = *(ulonglong *)(in_R8 + 0x98);
    if (uVar4 < *(ulonglong *)(in_R8 + 0x98)) {
      uVar6 = uVar4;
    }
    if (in_stack_00000028 == '\0') {
      if (0xaaaaaaaaaaaaaaa < uVar6) goto LAB_140956c5f;
      if (uVar6 == 0) goto LAB_140956b11;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == 0) goto LAB_140956c5f;
      if (uVar6 == 1) {
        uVar4 = 0;
      }
      else {
        lVar5 = 5;
        uVar4 = 0;
        do {
          uVar7 = *(undefined4 *)(lVar2 + -0xc + lVar5 * 4);
          fVar8 = *(float *)(lVar3 + uVar4 * 4);
          *(undefined4 *)(extraout_RAX_01 + -0x14 + lVar5 * 4) =
               *(undefined4 *)(lVar2 + -0x14 + lVar5 * 4);
          *(float *)(extraout_RAX_01 + -0x10 + lVar5 * 4) = fVar8 + 0.0;
          *(undefined4 *)(extraout_RAX_01 + -0xc + lVar5 * 4) = uVar7;
          uVar7 = *(undefined4 *)(lVar2 + lVar5 * 4);
          fVar8 = *(float *)(lVar3 + 4 + uVar4 * 4);
          uVar4 = uVar4 + 2;
          *(undefined4 *)(extraout_RAX_01 + -8 + lVar5 * 4) =
               *(undefined4 *)(lVar2 + -8 + lVar5 * 4);
          *(float *)(extraout_RAX_01 + -4 + lVar5 * 4) = fVar8 + 0.0;
          *(undefined4 *)(extraout_RAX_01 + lVar5 * 4) = uVar7;
          lVar5 = lVar5 + 6;
        } while ((uVar6 & 0xffffffffffffffe) != uVar4);
      }
      if ((uVar6 & 1) == 0) goto LAB_140956b11;
      lVar5 = uVar4 * 0xc;
      uVar7 = *(undefined4 *)(lVar2 + lVar5);
      fVar8 = *(float *)(lVar3 + uVar4 * 4) + 0.0;
      lVar3 = extraout_RAX_01;
    }
    else {
      if (0xaaaaaaaaaaaaaaa < uVar6) {
LAB_140956c5f:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (uVar6 == 0) goto LAB_140956b11;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == 0) goto LAB_140956c5f;
      if (uVar6 == 1) {
        uVar4 = 0;
      }
      else {
        lVar5 = 5;
        uVar4 = 0;
        do {
          uVar7 = *(undefined4 *)(lVar2 + -0xc + lVar5 * 4);
          fVar8 = *(float *)(lVar3 + uVar4 * 4);
          *(undefined4 *)(extraout_RAX + -0x14 + lVar5 * 4) =
               *(undefined4 *)(lVar2 + -0x14 + lVar5 * 4);
          *(float *)(extraout_RAX + -0x10 + lVar5 * 4) = fVar8 + -0.29000002;
          *(undefined4 *)(extraout_RAX + -0xc + lVar5 * 4) = uVar7;
          uVar7 = *(undefined4 *)(lVar2 + lVar5 * 4);
          fVar8 = *(float *)(lVar3 + 4 + uVar4 * 4);
          uVar4 = uVar4 + 2;
          *(undefined4 *)(extraout_RAX + -8 + lVar5 * 4) = *(undefined4 *)(lVar2 + -8 + lVar5 * 4);
          *(float *)(extraout_RAX + -4 + lVar5 * 4) = fVar8 + -0.29000002;
          *(undefined4 *)(extraout_RAX + lVar5 * 4) = uVar7;
          lVar5 = lVar5 + 6;
        } while ((uVar6 & 0xffffffffffffffe) != uVar4);
      }
      if ((uVar6 & 1) == 0) goto LAB_140956b11;
      lVar5 = uVar4 * 0xc;
      uVar7 = *(undefined4 *)(lVar2 + lVar5);
      fVar8 = *(float *)(lVar3 + uVar4 * 4) + -0.29000002;
      lVar3 = extraout_RAX;
    }
    uVar1 = *(undefined4 *)(lVar2 + 8 + lVar5);
    *(undefined4 *)(lVar3 + lVar5) = uVar7;
    *(float *)(lVar3 + 4 + lVar5) = fVar8;
    *(undefined4 *)(lVar3 + 8 + lVar5) = uVar1;
  }
LAB_140956b11:
  alloc::fmt::format::format_inner();
  systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
  return;
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': 'add<alloc::string::String>', 'address': '1409231f0'}, {'name': 'format_inner', 'address': '140d79f90'}, {'name': 'handle_error', 'address': '1428d8fe3'}]