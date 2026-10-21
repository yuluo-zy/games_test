
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::decorator_visual::stairs::find_alignment_preferences(void)

{
  undefined8 uVar1;
  byte bVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined2 *extraout_RAX_01;
  undefined2 *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined2 *extraout_RAX_04;
  undefined2 *puVar7;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong *in_R8;
  uint uVar8;
  float fVar9;
  float fVar10;
  float extraout_XMM0_Da;
  float in_XMM3_Da;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float in_stack_00000028;
  
  if ((*(byte *)(in_RDX + 2) & 1) != 0) {
    *in_RCX = 0;
    in_RCX[1] = 4;
    in_RCX[2] = 0;
    return;
  }
  uVar1 = *(undefined8 *)((longlong)in_RDX + 0x14);
  fVar14 = (float)uVar1;
  fVar10 = (float)((ulonglong)uVar1 >> 0x20);
  lVar3 = *in_RDX;
  lVar4 = in_RDX[1];
  country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  bVar2 = *(byte *)(extraout_RAX + 0x22);
  if (1 < bVar2) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  lVar5 = *in_R8;
  lVar6 = in_R8[1];
  country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info();
  if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (lVar3 != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (((lVar5 != 0) || (lVar4 != lVar6)) || (1 < *(byte *)(extraout_RAX_00 + 0x22))) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined2 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    fVar9 = 1.0 / SQRT(in_stack_00000028 * in_stack_00000028 + in_XMM3_Da * in_XMM3_Da);
    if ((int)fVar9 - 1U < 0x7fffff || (int)ABS(fVar9) - 0x800000U < 0x7f000000 && -1 < (int)fVar9) {
      fVar13 = in_XMM3_Da * fVar9;
      fVar9 = fVar9 * in_stack_00000028;
    }
    else {
      fVar13 = 0.0;
      fVar9 = 0.0;
    }
    fVar11 = fVar9 * fVar14 - fVar13 * fVar10;
    uVar8 = -(uint)NAN(fVar11);
    fVar11 = (float)(uVar8 & 0x7fc00000 |
                    ~uVar8 & ((uint)fVar11 & 0x80000000 |
                             (uint)__xmm_3f8000003f8000003f8000003f800000));
    fVar12 = -fVar10 * fVar11;
    fVar11 = fVar11 * fVar14;
    fVar10 = fVar9 * fVar10 + fVar13 * fVar14;
    fVar9 = fVar9 * fVar11 + fVar13 * fVar12;
    fVar14 = *(float *)(&__real_3ed413cd3f800000 + (ulonglong)(0.0 < fVar10) * 4);
    *extraout_RAX_01 = 0;
    *(undefined1 *)(extraout_RAX_01 + 1) = 0;
    *(undefined8 *)(extraout_RAX_01 + 2) = uVar1;
    *(float *)(extraout_RAX_01 + 10) = fVar10 * fVar14;
    extraout_RAX_01[0xc] = 1;
    *(byte *)(extraout_RAX_01 + 0xd) = bVar2 ^ 1;
    *(undefined8 *)(extraout_RAX_01 + 0xe) = uVar1;
    *(float *)(extraout_RAX_01 + 0x12) = fVar12;
    *(float *)(extraout_RAX_01 + 0x14) = fVar11;
    *(float *)(extraout_RAX_01 + 0x16) = fVar9;
    extraout_RAX_01[0x18] = 1;
    *(byte *)(extraout_RAX_01 + 0x19) = bVar2 ^ 1;
    *(undefined8 *)(extraout_RAX_01 + 0x1a) = uVar1;
    *(float *)(extraout_RAX_01 + 0x1e) = -fVar12;
    *(ulonglong *)(extraout_RAX_01 + 0x20) = CONCAT44(fVar9,fVar11) ^ 0x8000000080000000;
    *in_RCX = 3;
    in_RCX[1] = extraout_RAX_01;
    in_RCX[2] = 3;
                    // WARNING: Read-only address (ram,0x000142925930) is written
    return;
  }
  if (1e-06 <= in_stack_00000028 * in_stack_00000028 + in_XMM3_Da * in_XMM3_Da) {
    country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_03 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
    ::utils::wall_space::impl$1::subtract();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_04 == (undefined2 *)0x0) goto LAB_14080c340;
    fVar9 = (float)(~-(uint)NAN(extraout_XMM0_Da) &
                    ((uint)-extraout_XMM0_Da & 0x80000000 | 0x3f800000) |
                   -(uint)NAN(extraout_XMM0_Da) & 0xffc00000);
    fVar10 = -fVar10 * fVar9;
    fVar9 = fVar9 * fVar14;
    if (bVar2 == 0) {
      fVar10 = -fVar10;
      fVar9 = -fVar9;
    }
    *extraout_RAX_04 = 1;
    *(byte *)(extraout_RAX_04 + 1) = bVar2 ^ 1;
    *(undefined8 *)(extraout_RAX_04 + 2) = uVar1;
    *(float *)(extraout_RAX_04 + 6) = fVar10;
    *(float *)(extraout_RAX_04 + 8) = fVar9;
    puVar7 = extraout_RAX_04;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined2 *)0x0) {
LAB_14080c340:
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = 0;
    *(undefined1 *)(extraout_RAX_02 + 1) = 0;
    *(undefined8 *)(extraout_RAX_02 + 2) = uVar1;
    puVar7 = extraout_RAX_02;
  }
  *(undefined4 *)(puVar7 + 10) = 0x3f800000;
  *in_RCX = 1;
  in_RCX[1] = puVar7;
  in_RCX[2] = 1;
  return;
}

