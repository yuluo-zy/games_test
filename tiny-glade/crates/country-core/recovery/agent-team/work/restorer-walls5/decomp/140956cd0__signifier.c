// 0x140956cd0 signifier size=1234 sig=void __cdecl signifier(void)
void __cdecl
country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier(void)

{
  float fVar1;
  char cVar2;
  undefined8 uVar3;
  longlong lVar4;
  longlong lVar5;
  code *_Src;
  undefined1 *_Size;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong uVar6;
  undefined8 *extraout_RAX_02;
  void *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong *in_R8;
  longlong lVar7;
  ulonglong uVar8;
  void *pvVar9;
  undefined4 uVar10;
  longlong local_100;
  void *local_f8;
  undefined1 *local_f0;
  undefined1 local_e8 [8];
  code *local_e0;
  longlong local_d8;
  code *local_d0;
  undefined1 *local_c8;
  code *local_c0;
  longlong local_b0;
  undefined1 local_a4 [4];
  ulonglong local_a0;
  longlong local_98;
  undefined **local_88;
  undefined **local_80;
  longlong *local_78;
  undefined8 local_70;
  undefined **local_68;
  longlong local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  if (*(char *)((longlong)in_RCX + 9) == '\x01') {
    cVar2 = *(char *)(in_RCX + 1);
    if (cVar2 == '\0') {
      lVar5 = *in_R8;
      if (*(char *)(lVar5 + 0x18) == '\0') {
        return;
      }
      if (*(longlong *)(lVar5 + 0x38) != in_R8[1]) {
        return;
      }
      if (*(int *)(lVar5 + 0x28) == 2) {
        return;
      }
    }
    uVar3 = *in_RCX;
    local_e0 = resources::walls::impl$0::fmt;
    local_88 = &PTR_s_wall_height_edit__142ab4820;
    local_80 = (undefined **)0x1;
    local_78 = (longlong *)local_e8;
    local_70 = 1;
    local_68 = (undefined **)0x0;
    alloc::fmt::format::format_inner();
    _Size = local_c8;
    _Src = local_d0;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    uVar10 = 0;
    if (cVar2 == '\0') {
      uVar10 = 0x3e4ccccd;
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    extraout_RAX[2] = uVar3;
    *(undefined4 *)(extraout_RAX + 3) = uVar10;
    local_80 = (undefined **)&DAT_142ab4830;
    if ((longlong)_Size < 0) {
LAB_140957192:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (_Size == (undefined1 *)0x0) {
      pvVar9 = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      pvVar9 = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) goto LAB_140957192;
    }
    memcpy(pvVar9,_Src,(size_t)_Size);
    local_88 = (undefined **)_Size;
    local_78 = (longlong *)_Size;
    local_68 = (undefined **)&DAT_142ab4830;
    local_80 = pvVar9;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    local_98 = local_d8;
  }
  else {
    uVar8 = *(ulonglong *)(in_RDX + 0x98);
    if (*(ulonglong *)(in_RDX + 0xe8) < *(ulonglong *)(in_RDX + 0x98)) {
      uVar8 = *(ulonglong *)(in_RDX + 0xe8);
    }
    if (0xaaaaaaaaaaaaaaa < uVar8) {
LAB_140957140:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_b0 = uVar8 * 0xc;
    local_a0 = uVar8;
    if (uVar8 == 0) {
      local_58 = 4;
    }
    else {
      lVar5 = *(longlong *)(in_RDX + 0x90);
      lVar4 = *(longlong *)(in_RDX + 0xe0);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == 0) goto LAB_140957140;
      if (uVar8 == 1) {
        uVar6 = 0;
      }
      else {
        lVar7 = 5;
        uVar6 = 0;
        do {
          uVar10 = *(undefined4 *)(lVar5 + -0xc + lVar7 * 4);
          fVar1 = *(float *)(lVar4 + uVar6 * 4);
          *(undefined4 *)(extraout_RAX_01 + -0x14 + lVar7 * 4) =
               *(undefined4 *)(lVar5 + -0x14 + lVar7 * 4);
          *(float *)(extraout_RAX_01 + -0x10 + lVar7 * 4) = fVar1 + 0.6;
          *(undefined4 *)(extraout_RAX_01 + -0xc + lVar7 * 4) = uVar10;
          uVar10 = *(undefined4 *)(lVar5 + lVar7 * 4);
          fVar1 = *(float *)(lVar4 + 4 + uVar6 * 4);
          uVar6 = uVar6 + 2;
          *(undefined4 *)(extraout_RAX_01 + -8 + lVar7 * 4) =
               *(undefined4 *)(lVar5 + -8 + lVar7 * 4);
          *(float *)(extraout_RAX_01 + -4 + lVar7 * 4) = fVar1 + 0.6;
          *(undefined4 *)(extraout_RAX_01 + lVar7 * 4) = uVar10;
          lVar7 = lVar7 + 6;
        } while ((uVar8 & 0xffffffffffffffe) != uVar6);
      }
      local_58 = extraout_RAX_01;
      if ((uVar8 & 1) != 0) {
        lVar7 = uVar6 * 0xc;
        fVar1 = *(float *)(lVar4 + uVar6 * 4);
        uVar10 = *(undefined4 *)(lVar5 + 8 + lVar7);
        *(undefined4 *)(extraout_RAX_01 + lVar7) = *(undefined4 *)(lVar5 + lVar7);
        *(float *)(extraout_RAX_01 + 4 + lVar7) = fVar1 + 0.6;
        *(undefined4 *)(extraout_RAX_01 + 8 + lVar7) = uVar10;
      }
    }
    local_d0 = resources::walls::impl$0::fmt;
    local_c8 = local_a4;
    local_c0 = core::fmt::num::imp::impl$6::fmt;
    local_88 = &PTR_s_wall_height_edit__142ab47d0;
    local_80 = (undefined **)0x2;
    local_78 = &local_d8;
    local_70 = 2;
    local_68 = (undefined **)0x0;
    local_49 = 1;
    alloc::fmt::format::format_inner();
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_98 = local_100;
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = 1;
    extraout_RAX_02[1] = 1;
    extraout_RAX_02[2] = local_a0;
    extraout_RAX_02[3] = local_58;
    extraout_RAX_02[4] = local_a0;
    local_80 = &PTR_drop_ptr<country_core::components::WoodenGate>_142ab47f0;
    if ((longlong)local_f0 < 0) {
LAB_14095716a:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (local_f0 == (undefined1 *)0x0) {
      pvVar9 = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      pvVar9 = extraout_RAX_03;
      if (extraout_RAX_03 == (void *)0x0) goto LAB_14095716a;
    }
    memcpy(pvVar9,local_f8,(size_t)local_f0);
    local_88 = (undefined **)local_f0;
    local_78 = (longlong *)local_f0;
    local_68 = &PTR_drop_ptr<country_core::components::WoodenGate>_142ab47f0;
    local_80 = pvVar9;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
  }
  if (local_98 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}
