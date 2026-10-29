
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::resources::walls::decorator_outline::DecoratorOutline::display(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined4 uVar4;
  ulonglong uVar5;
  undefined1 auVar6 [16];
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  ulonglong *puVar11;
  code *pcVar12;
  undefined8 *puVar13;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  ulonglong _Size;
  undefined1 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  void *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  void *extraout_RAX_05;
  undefined4 *in_RCX;
  float *in_RDX;
  uint in_R9D;
  undefined1 *_Dst;
  void *pvVar14;
  ulonglong uVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  char in_stack_00000030;
  undefined1 in_stack_00000038;
  ulonglong *local_160;
  code *local_158;
  undefined8 *local_150;
  code *local_148;
  undefined1 *local_140;
  code *local_138;
  ulonglong local_130;
  longlong local_128;
  void *pvStack_120;
  ulonglong **ppuStack_118;
  code *pcStack_110;
  undefined8 local_108;
  float local_100;
  ulonglong **local_f8;
  undefined **local_f0;
  ulonglong **local_e8;
  undefined8 *local_e0;
  undefined **local_d8;
  ulonglong *puStack_c8;
  undefined8 local_c0;
  
  local_150 = &local_108;
  local_c0 = 0xfffffffffffffffe;
  local_108 = *(undefined8 *)(in_RDX + 0xc);
  local_100 = in_RDX[0xe];
  fVar7 = in_RDX[1];
  fVar8 = in_RDX[4];
  fVar9 = in_RDX[8];
  fVar10 = in_RDX[9];
  fVar1 = in_RDX[2];
  fVar2 = in_RDX[6];
  fVar3 = in_RDX[10];
  if (fVar3 <= 0.0) {
    fVar18 = in_RDX[5] - *in_RDX;
    if (fVar18 <= 0.0) {
      fVar18 = (__real_3f800000 - fVar3) - fVar18;
      fVar20 = __real_3f000000 / SQRT(fVar18);
      fVar17 = fVar18 * fVar20;
      fVar18 = (fVar7 + fVar8) * fVar20;
      fVar16 = (fVar1 + fVar9) * fVar20;
      fVar20 = (fVar2 - fVar10) * fVar20;
    }
    else {
      fVar18 = fVar18 + (__real_3f800000 - fVar3);
      fVar20 = __real_3f000000 / SQRT(fVar18);
      fVar17 = (fVar7 + fVar8) * fVar20;
      fVar18 = fVar18 * fVar20;
      fVar16 = (fVar2 + fVar10) * fVar20;
      fVar20 = (fVar9 - fVar1) * fVar20;
    }
  }
  else {
    fVar19 = in_RDX[5] + *in_RDX;
    if (fVar19 <= 0.0) {
      fVar19 = (__real_3f800000 + fVar3) - fVar19;
      fVar20 = __real_3f000000 / SQRT(fVar19);
      fVar17 = (fVar1 + fVar9) * fVar20;
      fVar18 = (fVar2 + fVar10) * fVar20;
      fVar16 = fVar19 * fVar20;
      fVar20 = (fVar7 - fVar8) * fVar20;
    }
    else {
      fVar19 = fVar19 + __real_3f800000 + fVar3;
      fVar20 = __real_3f000000 / SQRT(fVar19);
      fVar17 = (fVar2 - fVar10) * fVar20;
      fVar18 = (fVar9 - fVar1) * fVar20;
      fVar16 = (fVar7 - fVar8) * fVar20;
      fVar20 = fVar20 * fVar19;
    }
  }
  if (in_stack_00000030 != '\0') {
    local_108 = CONCAT44(in_RDX[0xd] + fVar10 * _UNK_142aecb34,
                         in_RDX[0xc] + fVar9 * ___xmm_00000000000000003e3851ec3e3851ec);
    local_100 = local_100 + fVar3 * __real_3e3851ec;
  }
  local_130 = (ulonglong)in_R9D;
  local_160 = &local_130;
  local_158 = core::fmt::num::impl$89::fmt;
  local_148 = glam::f32::vec3::impl$93::fmt;
  local_140 = &stack0x00000028;
  local_138 = decorator::impl$57::fmt;
  local_f8 = (ulonglong **)&PTR_DAT_142aed708;
  local_f0 = (void *)0x3;
  local_e8 = &local_160;
  local_e0 = (undefined8 *)0x3;
  local_d8 = (undefined **)0x0;
  alloc::fmt::format::format_inner();
                    /* WARNING: Switch is manually overridden */
  switch(*in_RCX) {
  case 0:
    alloc::string::impl$6::clone();
    puVar13 = local_150;
    pcVar12 = local_158;
    puVar11 = local_160;
    local_f8 = (ulonglong **)local_108;
    local_f0 = (undefined **)CONCAT44(local_f0._4_4_,local_100);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
      pcStack_110 = pcVar12;
      puStack_c8 = puVar11;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    *(float *)(extraout_RAX + 2) = fVar17;
    *(float *)((longlong)extraout_RAX + 0x14) = fVar18;
    *(float *)(extraout_RAX + 3) = fVar16;
    *(float *)((longlong)extraout_RAX + 0x1c) = fVar20;
    extraout_RAX[4] = puVar11;
    extraout_RAX[5] = pcVar12;
    extraout_RAX[6] = puVar13;
    extraout_RAX[7] = local_f8;
    *(undefined4 *)(extraout_RAX + 8) = local_f0._0_4_;
    local_f0 = (undefined **)&UNK_142aed738;
    if ((longlong)ppuStack_118 < 0) {
code_r0x000140b34218:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    if (ppuStack_118 == (ulonglong **)0x0) {
      pvVar14 = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      pvVar14 = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) goto code_r0x000140b34218;
    }
    memcpy(pvVar14,pvStack_120,(size_t)ppuStack_118);
    local_f8 = ppuStack_118;
    local_e8 = ppuStack_118;
    local_d8 = (undefined **)&UNK_142aed738;
    local_f0 = pvVar14;
    local_e0 = extraout_RAX;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    break;
  case 1:
    uVar5 = *(ulonglong *)(in_RCX + 6);
    auVar6._8_8_ = 0;
    auVar6._0_8_ = uVar5;
    _Size = SUB168(auVar6 * ZEXT816(0xc),0);
    if ((SUB168(auVar6 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < _Size)) {
code_r0x000140b3419d:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    pvVar14 = *(void **)(in_RCX + 4);
    if (_Size == 0) {
      _Dst = &DAT_00000004;
      uVar15 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_01;
      uVar15 = uVar5;
      if (extraout_RAX_01 == (undefined1 *)0x0) goto code_r0x000140b3419d;
    }
    memcpy(_Dst,pvVar14,_Size);
    local_f0 = (undefined **)CONCAT44(local_f0._4_4_,local_100);
    local_f8 = (ulonglong **)local_108;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_04 == (undefined8 *)0x0) {
      pcStack_110 = (code *)_Dst;
      puStack_c8 = (ulonglong *)uVar15;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_04 = 1;
    extraout_RAX_04[1] = 1;
    *(float *)(extraout_RAX_04 + 2) = fVar17;
    *(float *)((longlong)extraout_RAX_04 + 0x14) = fVar18;
    *(float *)(extraout_RAX_04 + 3) = fVar16;
    *(float *)((longlong)extraout_RAX_04 + 0x1c) = fVar20;
    extraout_RAX_04[4] = uVar15;
    extraout_RAX_04[5] = _Dst;
    extraout_RAX_04[6] = uVar5;
    extraout_RAX_04[7] = local_f8;
    *(undefined4 *)(extraout_RAX_04 + 8) = local_f0._0_4_;
    local_f0 = &
               PTR_drop_in_place<country_core::resources::walls::decorator_outline::impl_1::display::closure_env_1>_142aed768
    ;
    if ((longlong)ppuStack_118 < 0) {
code_r0x000140b34233:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    if (ppuStack_118 == (ulonglong **)0x0) {
      pvVar14 = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      pvVar14 = extraout_RAX_05;
      if (extraout_RAX_05 == (void *)0x0) goto code_r0x000140b34233;
    }
    memcpy(pvVar14,pvStack_120,(size_t)ppuStack_118);
    local_f8 = ppuStack_118;
    local_e8 = ppuStack_118;
    local_d8 = &
               PTR_drop_in_place<country_core::resources::walls::decorator_outline::impl_1::display::closure_env_1>_142aed768
    ;
    local_f0 = pvVar14;
    local_e0 = extraout_RAX_04;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    break;
  case 2:
    uVar4 = in_RCX[1];
    local_f0 = (undefined **)CONCAT44(local_f0._4_4_,local_100);
    local_f8 = (ulonglong **)local_108;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = 1;
    extraout_RAX_02[1] = 1;
    *(undefined4 *)(extraout_RAX_02 + 2) = uVar4;
    *(ulonglong ***)((longlong)extraout_RAX_02 + 0x14) = local_f8;
    *(undefined4 *)((longlong)extraout_RAX_02 + 0x1c) = local_f0._0_4_;
    *(undefined1 *)(extraout_RAX_02 + 4) = in_stack_00000038;
    local_f0 = (undefined **)&UNK_142aed798;
    if ((longlong)ppuStack_118 < 0) {
code_r0x000140b341cb:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    if (ppuStack_118 == (ulonglong **)0x0) {
      pvVar14 = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      pvVar14 = extraout_RAX_03;
      if (extraout_RAX_03 == (void *)0x0) goto code_r0x000140b341cb;
    }
    memcpy(pvVar14,pvStack_120,(size_t)ppuStack_118);
    local_f8 = ppuStack_118;
    local_e8 = ppuStack_118;
    local_d8 = (undefined **)&UNK_142aed798;
    local_f0 = pvVar14;
    local_e0 = extraout_RAX_02;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
  }
  if (local_128 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

