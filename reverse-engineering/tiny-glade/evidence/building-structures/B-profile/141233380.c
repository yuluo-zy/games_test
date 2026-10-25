
/* WARNING: Removing unreachable block (ram,0x00014123356f) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_wall_constructor::arch_walker::ArchWalker::
walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env_0<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>_>
          (void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  int iVar3;
  undefined1 *puVar4;
  char extraout_AL;
  char extraout_AL_00;
  int iVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong in_RCX;
  ulonglong uVar8;
  ulonglong in_R8;
  undefined4 *in_R9;
  int iVar9;
  ulonglong uVar10;
  float fVar11;
  float fVar12;
  float extraout_XMM0_Da;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  ulonglong in_stack_00000028;
  ulonglong in_stack_00000038;
  longlong *in_stack_00000040;
  float local_218;
  float fStack_214;
  float fStack_210;
  float local_208;
  float fStack_204;
  float local_200;
  undefined1 local_1fc [4];
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined8 *local_1e8;
  undefined8 local_1e0;
  undefined8 local_1d8;
  undefined4 local_1c8;
  int local_1c4;
  undefined4 local_1bc;
  float local_1b4;
  float fStack_1b0;
  float fStack_1ac;
  ulonglong local_1a8;
  longlong local_1a0;
  undefined4 *local_198;
  int *local_190;
  longlong local_188;
  ulonglong local_180;
  float local_174;
  float local_170;
  float local_16c;
  undefined8 local_168;
  undefined8 uStack_160;
  float local_158;
  float fStack_154;
  float fStack_150;
  float fStack_14c;
  float local_148;
  float fStack_144;
  float fStack_140;
  float fStack_13c;
  float local_138;
  float fStack_134;
  float fStack_130;
  float fStack_12c;
  float local_128;
  float fStack_124;
  undefined8 uStack_120;
  float local_118;
  float fStack_114;
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
  uVar1 = *in_R9;
  uVar2 = in_R9[2];
  local_168 = (undefined1 *)CONCAT44(uVar2,uVar1);
  local_180 = in_R8;
  ::utils::swizzlers::impl$1::xvy();
  uVar8 = local_180;
  if (in_stack_00000028 < local_180) {
    uVar8 = in_stack_00000028;
  }
  fVar26 = 0.0;
  uVar7 = (ulonglong)(uVar8 != 0);
  if (uVar7 < uVar8) {
    lVar6 = 0;
LAB_14123358c:
    if (uVar7 - 1 != lVar6) goto code_r0x000141233591;
    uVar7 = uVar7 - lVar6;
    do {
      uVar10 = uVar7 + 1;
      local_168 = (undefined1 *)CONCAT44(in_R9[uVar7 * 3 + 2],in_R9[uVar7 * 3]);
      ::utils::swizzlers::impl$1::xvy();
      fVar26 = fVar26 + SQRT((local_200 - (float)local_1f0) * (local_200 - (float)local_1f0) +
                             (fStack_204 - local_1f8._4_4_) * (fStack_204 - local_1f8._4_4_) +
                             (local_208 - (float)local_1f8) * (local_208 - (float)local_1f8));
      local_1f0 = CONCAT44(local_1f0._4_4_,local_200);
      local_1f8 = (undefined **)CONCAT44(fStack_204,local_208);
      uVar7 = uVar10;
    } while (uVar10 < uVar8);
  }
LAB_1412335aa:
  fVar11 = floorf(fVar26 / *(float *)(in_RCX + 8) + __real_3f000000);
  if (fVar11 <= __real_3f800000) {
    fVar11 = __real_3f800000;
  }
  uVar7 = 0;
  if (0.0 <= fVar11) {
    uVar7 = (longlong)(fVar11 - __real_5f000000) & (longlong)fVar11 >> 0x3f | (longlong)fVar11;
  }
  uVar10 = 0xffffffffffffffff;
  if (fVar11 <= __real_5f7fffff) {
    uVar10 = uVar7;
  }
  if (in_stack_00000038 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  fVar26 = fVar26 / (float)uVar10;
  local_1f8 = (undefined **)CONCAT44(uVar2,uVar1);
  ::utils::swizzlers::impl$1::xvy();
  fVar11 = __real_3f800000;
  if (in_stack_00000038 < uVar8) {
    uVar8 = in_stack_00000038;
  }
  local_1a0 = *in_stack_00000040;
  local_198 = (undefined4 *)in_stack_00000040[1];
  local_190 = (int *)in_stack_00000040[2];
  local_188 = in_stack_00000040[3];
  lVar6 = 1;
  fVar32 = 0.0;
  uVar7 = 0;
  do {
    fVar31 = local_218;
    fVar28 = fStack_214;
    fVar27 = fStack_210;
    if (uVar8 <= uVar7) goto LAB_141233b07;
    uVar7 = uVar7 + 1;
    lVar6 = lVar6 + -1;
  } while (lVar6 != 0);
  while (uVar7 < uVar8) {
    local_1a8 = uVar7 + 1;
    local_1f8 = (undefined **)CONCAT44(in_R9[uVar7 * 3 + 2],in_R9[uVar7 * 3]);
    ::utils::swizzlers::impl$1::xvy();
    local_f8 = ZEXT416((uint)local_218);
    local_118 = fStack_214;
    fStack_114 = fStack_210;
    local_128 = fStack_1b0;
    fStack_124 = fStack_1ac;
    local_108 = ZEXT416((uint)local_1b4);
    uStack_120 = 0;
    uStack_110 = 0;
    fVar29 = SQRT((fStack_1ac - fStack_210) * (fStack_1ac - fStack_210) +
                  (fStack_1b0 - fStack_214) * (fStack_1b0 - fStack_214) +
                  (local_1b4 - local_218) * (local_1b4 - local_218));
    fVar12 = fVar32 + fVar29;
    if (fVar26 < fVar12) {
      fVar30 = 0.0;
      fVar25 = fVar31;
      fVar18 = fVar28;
      fVar13 = fVar27;
      do {
        fVar31 = fVar26 - fVar32;
        if (fVar31 <= 0.0) {
          fVar31 = 0.0;
        }
        fVar30 = fVar31 + fVar30;
        fVar27 = fVar30 / fVar29;
        ::utils::wall_space::impl$1::subtract();
        ::utils::wall_space::impl$1::project_clamp();
        if ((extraout_XMM0_Da < 0.0) || (fVar11 < extraout_XMM0_Da)) {
          local_168 = local_1fc;
          uStack_160 = core::fmt::float::impl$5::fmt;
          local_1f8 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
          local_1f0 = 1;
          local_1d8 = 0;
          local_1e8 = &local_168;
          local_1e0 = 1;
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        if (NAN(extraout_XMM0_Da)) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        fVar12 = fVar11 - fVar27;
        fVar31 = (float)local_108._0_4_ * fVar27 + (float)local_f8._0_4_ * fVar12;
        fVar28 = fVar27 * local_128 + fVar12 * local_118;
        fVar27 = fVar27 * fStack_124 + fVar12 * fStack_114;
        fVar12 = fVar31 - fVar25;
        fVar22 = (fVar28 + __real_be666667) - (fVar18 + __real_be666667);
        fVar23 = fVar27 - fVar13;
        fVar24 = SQRT(fVar23 * fVar23 + fVar12 * fVar12 + fVar22 * fVar22);
        fVar16 = fVar11 / fVar24;
        fVar12 = fVar12 * fVar16;
        fVar22 = fVar16 * fVar22;
        fVar16 = fVar16 * fVar23;
        fVar17 = fVar16 * __real_3f800000 - fVar22 * 0.0;
        fVar19 = fVar12 * 0.0 - fVar16 * 0.0;
        fVar20 = fVar22 * 0.0 - fVar12 * __real_3f800000;
        fVar21 = fVar22 * 0.0 - fVar12 * __real_3f800000;
        local_158 = fVar24 * fVar12 + 0.0 + fVar17 * 0.0;
        fStack_154 = fVar24 * fVar22 + 0.0 + fVar19 * 0.0;
        fStack_150 = fVar24 * fVar16 + 0.0 + fVar20 * 0.0;
        fStack_14c = fVar16 * 0.0 + 0.0 + fVar21 * 0.0;
        fVar23 = fVar12 * 0.0 + 0.0;
        fVar24 = fVar22 * 0.0 + 0.0;
        fVar14 = fVar16 * 0.0 + 0.0;
        fVar15 = fVar16 * 0.0 + 0.0;
        local_148 = fVar17 * ___xmm_000000003eaccccc3eaccccc3eaccccc + fVar23;
        fStack_144 = fVar19 * _UNK_142c1e464 + fVar24;
        fStack_140 = fVar20 * _UNK_142c1e468 + fVar14;
        fStack_13c = fVar21 * _UNK_142c1e46c + fVar15;
        local_138 = fVar23 + fVar17 * 0.0 + fVar25;
        fStack_134 = fVar24 + fVar19 * 0.0 + fVar18 + __real_be666667;
        fStack_130 = fVar14 + fVar20 * 0.0 + fVar13;
        fStack_12c = fVar15 + fVar21 * 0.0 + fVar13;
        local_168 = (undefined1 *)
                    CONCAT44(fVar19 * _UNK_142c1e444 + fVar22 * _UNK_142c1e444 + _UNK_142c1e454,
                             fVar17 * ___xmm_3e666667000000000000000000000000 +
                             fVar12 * ___xmm_3e666667000000000000000000000000 +
                             ___xmm_00000000000000003e66666700000000);
        uStack_160 = (code *)CONCAT44(fVar21 * _UNK_142c1e44c +
                                      fVar16 * _UNK_142c1e44c + _UNK_142c1e45c,
                                      fVar20 * _UNK_142c1e448 +
                                      fVar16 * _UNK_142c1e448 + _UNK_142c1e458);
        country_core::resources::render::impl$1::from();
        fVar25 = fVar25 * __real_447a0000;
        iVar5 = (int)fVar25;
        if (__real_4effffff < fVar25) {
          iVar5 = 0x7fffffff;
        }
        if (NAN(fVar25)) {
          iVar5 = 0;
        }
        fVar13 = fVar13 * __real_447a0000;
        iVar9 = (int)fVar13;
        if (__real_4effffff < fVar13) {
          iVar9 = 0x7fffffff;
        }
        if (NAN(fVar13)) {
          iVar9 = 0;
        }
        uVar1 = *local_198;
        iVar3 = *local_190;
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        local_1bc = 0;
        puVar4 = *(undefined1 **)(local_1a0 + 8);
        local_1c8 = uVar1;
        local_1c4 = iVar9 + iVar5 + iVar3;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        if (extraout_AL != '\0') {
          *puVar4 = 1;
        }
        fVar32 = fVar32 - fVar26;
        if (fVar32 <= 0.0) {
          fVar32 = 0.0;
        }
        fVar12 = (fVar29 - fVar30) + fVar32;
        fVar25 = fVar31;
        fVar18 = fVar28;
        fVar13 = fVar27;
      } while (fVar26 < fVar12);
    }
    if (fVar12 < 0.0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    fStack_210 = fStack_1ac;
    local_218 = local_1b4;
    fStack_214 = fStack_1b0;
    uVar7 = local_1a8;
    fVar32 = fVar12;
  }
LAB_141233b07:
  if (fVar26 * *(float *)(in_RCX + 0xc) < fVar32) {
    uVar8 = in_stack_00000028 - 1;
    if (in_stack_00000038 <= uVar8) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    local_1f8 = (undefined **)CONCAT44(in_R9[uVar8 * 3 + 2],in_R9[uVar8 * 3]);
    ::utils::swizzlers::impl$1::xvy();
    local_174 = local_174 - fVar31;
    fVar25 = (local_170 + __real_be666667) - (fVar28 + __real_be666667);
    local_16c = local_16c - fVar27;
    fVar26 = SQRT(local_16c * local_16c + local_174 * local_174 + fVar25 * fVar25);
    fVar11 = fVar11 / fVar26;
    local_174 = local_174 * fVar11;
    fVar25 = fVar11 * fVar25;
    fVar11 = fVar11 * local_16c;
    fVar18 = fVar11 * __real_3f800000 - fVar25 * 0.0;
    fVar13 = local_174 * 0.0 - fVar11 * 0.0;
    fVar30 = fVar25 * 0.0 - local_174 * __real_3f800000;
    fVar23 = fVar25 * 0.0 - local_174 * __real_3f800000;
    local_158 = fVar26 * local_174 + 0.0 + fVar18 * 0.0;
    fStack_154 = fVar26 * fVar25 + 0.0 + fVar13 * 0.0;
    fStack_150 = fVar26 * fVar11 + 0.0 + fVar30 * 0.0;
    fStack_14c = fVar11 * 0.0 + 0.0 + fVar23 * 0.0;
    fVar26 = local_174 * 0.0 + 0.0;
    fVar32 = fVar25 * 0.0 + 0.0;
    fVar12 = fVar11 * 0.0 + 0.0;
    fVar29 = fVar11 * 0.0 + 0.0;
    local_148 = fVar18 * ___xmm_000000003eaccccc3eaccccc3eaccccc + fVar26;
    fStack_144 = fVar13 * _UNK_142c1e464 + fVar32;
    fStack_140 = fVar30 * _UNK_142c1e468 + fVar12;
    fStack_13c = fVar23 * _UNK_142c1e46c + fVar29;
    local_138 = fVar26 + fVar18 * 0.0 + fVar31;
    fStack_134 = fVar32 + fVar13 * 0.0 + fVar28 + __real_be666667;
    fStack_130 = fVar12 + fVar30 * 0.0 + fVar27;
    fStack_12c = fVar29 + fVar23 * 0.0 + fVar27;
    local_168 = (undefined1 *)
                CONCAT44(_UNK_142c1e444 * fVar13 + fVar25 * _UNK_142c1e444 + _UNK_142c1e454,
                         ___xmm_3e666667000000000000000000000000 * fVar18 +
                         local_174 * ___xmm_3e666667000000000000000000000000 +
                         ___xmm_00000000000000003e66666700000000);
    uStack_160 = (code *)CONCAT44(_UNK_142c1e44c * fVar23 + fVar11 * _UNK_142c1e44c + _UNK_142c1e45c
                                  ,_UNK_142c1e448 * fVar30 +
                                   fVar11 * _UNK_142c1e448 + _UNK_142c1e458);
    country_core::resources::render::impl$1::from();
    fVar31 = fVar31 * __real_447a0000;
    iVar5 = (int)fVar31;
    if (__real_4effffff < fVar31) {
      iVar5 = 0x7fffffff;
    }
    if (NAN(fVar31)) {
      iVar5 = 0;
    }
    fVar27 = fVar27 * __real_447a0000;
    iVar9 = (int)fVar27;
    if (__real_4effffff < fVar27) {
      iVar9 = 0x7fffffff;
    }
    if (NAN(fVar27)) {
      iVar9 = 0;
    }
    uVar1 = *local_198;
    iVar3 = *local_190;
    minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
    local_1bc = 0;
    puVar4 = *(undefined1 **)(local_1a0 + 8);
    local_1c8 = uVar1;
    local_1c4 = iVar9 + iVar5 + iVar3;
    country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
    push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
    if (extraout_AL_00 != '\0') {
      *puVar4 = 1;
    }
  }
  return;
code_r0x000141233591:
  lVar6 = lVar6 + -1;
  if (uVar7 - uVar8 == lVar6) goto LAB_1412335aa;
  goto LAB_14123358c;
}

