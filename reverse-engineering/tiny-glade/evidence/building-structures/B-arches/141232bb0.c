
/* WARNING: Removing unreachable block (ram,0x000141232db1) */

void __cdecl
system_wall_constructor::arch_walker::ArchWalker::
walk<system_wall_constructor::construct_arches::construct_arches::closure_env_3>(void)

{
  longlong lVar1;
  ulonglong uVar2;
  longlong in_RCX;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong in_R8;
  float fVar5;
  float fVar6;
  float extraout_XMM0_Da;
  float fVar7;
  float fVar8;
  float fVar9;
  ulonglong in_stack_00000028;
  ulonglong in_stack_00000038;
  float local_1c8;
  float fStack_1c4;
  float fStack_1c0;
  float local_1b8;
  float fStack_1b4;
  float local_1b0;
  float local_184;
  float fStack_180;
  float local_17c;
  undefined1 local_158 [4];
  float local_154;
  float fStack_150;
  float fStack_14c;
  ulonglong local_148;
  ulonglong local_140;
  undefined1 *local_138;
  code *local_130;
  undefined8 uStack_120;
  undefined8 uStack_110;
  undefined1 local_108 [16];
  undefined1 local_f8 [16];
  
  if (in_stack_00000028 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (in_R8 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  local_140 = in_R8;
  ::utils::swizzlers::impl$1::xvy();
  uVar4 = local_140;
  if (in_stack_00000028 < local_140) {
    uVar4 = in_stack_00000028;
  }
  fVar8 = 0.0;
  uVar3 = (ulonglong)(uVar4 != 0);
  if (uVar3 < uVar4) {
    lVar1 = 0;
LAB_141232dce:
    if (uVar3 - 1 != lVar1) goto code_r0x000141232dd3;
    uVar3 = uVar3 - lVar1;
    do {
      uVar3 = uVar3 + 1;
      ::utils::swizzlers::impl$1::xvy();
      fVar8 = fVar8 + SQRT((local_17c - local_1b0) * (local_17c - local_1b0) +
                           (fStack_180 - fStack_1b4) * (fStack_180 - fStack_1b4) +
                           (local_184 - local_1b8) * (local_184 - local_1b8));
      local_1b0 = local_17c;
      local_1b8 = local_184;
      fStack_1b4 = fStack_180;
    } while (uVar3 < uVar4);
  }
LAB_141232dec:
  fVar5 = floorf(fVar8 / *(float *)(in_RCX + 8) + __real_3f000000);
  if (fVar5 <= __real_3f800000) {
    fVar5 = __real_3f800000;
  }
  uVar3 = 0;
  if (0.0 <= fVar5) {
    uVar3 = (longlong)(fVar5 - __real_5f000000) & (longlong)fVar5 >> 0x3f | (longlong)fVar5;
  }
  uVar2 = 0xffffffffffffffff;
  if (fVar5 <= __real_5f7fffff) {
    uVar2 = uVar3;
  }
  if (in_stack_00000038 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  fVar8 = fVar8 / (float)uVar2;
  ::utils::swizzlers::impl$1::xvy();
  if (in_stack_00000038 < uVar4) {
    uVar4 = in_stack_00000038;
  }
  lVar1 = 1;
  uVar3 = 0;
  fVar5 = 0.0;
  do {
    if (uVar4 <= uVar3) goto LAB_1412331ae;
    uVar3 = uVar3 + 1;
    lVar1 = lVar1 + -1;
  } while (lVar1 != 0);
  while (uVar3 < uVar4) {
    local_148 = uVar3 + 1;
    ::utils::swizzlers::impl$1::xvy();
    local_108 = ZEXT416((uint)local_154);
    local_f8 = ZEXT416((uint)local_1c8);
    uStack_120 = 0;
    uStack_110 = 0;
    fVar9 = SQRT((fStack_14c - fStack_1c0) * (fStack_14c - fStack_1c0) +
                 (fStack_150 - fStack_1c4) * (fStack_150 - fStack_1c4) +
                 (local_154 - local_1c8) * (local_154 - local_1c8));
    fVar6 = fVar5 + fVar9;
    if (fVar8 < fVar6) {
      fVar7 = 0.0;
      do {
        fVar6 = fVar8 - fVar5;
        if (fVar8 - fVar5 <= 0.0) {
          fVar6 = 0.0;
        }
        fVar7 = fVar6 + fVar7;
        ::utils::wall_space::impl$1::subtract();
        ::utils::wall_space::impl$1::project_clamp();
        if ((extraout_XMM0_Da < 0.0) || (__real_3f800000 < extraout_XMM0_Da)) {
          local_138 = local_158;
          local_130 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        if (NAN(extraout_XMM0_Da)) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        fVar5 = fVar5 - fVar8;
        if (fVar5 <= 0.0) {
          fVar5 = 0.0;
        }
        construct_arches::construct_arches::closure_3();
        fVar6 = (fVar9 - fVar7) + fVar5;
      } while (fVar8 < fVar6);
    }
    if (fVar6 < 0.0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    fStack_1c0 = fStack_14c;
    local_1c8 = local_154;
    fStack_1c4 = fStack_150;
    uVar3 = local_148;
    fVar5 = fVar6;
  }
LAB_1412331ae:
  if (fVar8 * *(float *)(in_RCX + 0xc) < fVar5) {
    if (in_stack_00000038 <= in_stack_00000028 - 1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    ::utils::swizzlers::impl$1::xvy();
    construct_arches::construct_arches::closure_3();
  }
  return;
code_r0x000141232dd3:
  lVar1 = lVar1 + -1;
  if (uVar3 - uVar4 == lVar1) goto LAB_141232dec;
  goto LAB_141232dce;
}

