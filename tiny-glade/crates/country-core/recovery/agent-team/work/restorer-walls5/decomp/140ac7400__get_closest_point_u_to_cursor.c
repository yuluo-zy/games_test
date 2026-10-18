// 0x140ac7400 get_closest_point_u_to_cursor size=1494 sig=void __cdecl get_closest_point_u_to_cursor(void)
void __cdecl
country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::
get_closest_point_u_to_cursor(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float *in_RCX;
  ulonglong uVar4;
  longlong in_R9;
  ulonglong uVar5;
  float *pfVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Db;
  undefined4 extraout_XMM0_Dc;
  undefined4 extraout_XMM0_Dd;
  float in_XMM1_Da;
  float in_XMM2_Da;
  undefined4 in_XMM2_Db;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
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
  undefined8 *in_stack_00000028;
  float in_stack_00000030;
  float local_1ec;
  float local_1e8;
  float fStack_1e4;
  undefined4 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  float *local_1a8;
  code *local_1a0;
  float local_198;
  float fStack_194;
  undefined8 uStack_190;
  float local_188;
  float fStack_184;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined **local_108;
  undefined8 local_100;
  undefined4 **local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  
  uVar7 = *(ulonglong *)(in_R9 + 0x10);
  if (uVar7 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  local_1e8 = in_XMM2_Da;
  fStack_1e4 = (float)in_XMM2_Db;
  ::utils::camera::CameraMatrices::aspect_ratio();
  uVar4 = *(ulonglong *)(in_R9 + 0x28);
  uVar8 = uVar7;
  if (uVar4 < uVar7) {
    uVar8 = uVar4;
  }
  uVar7 = uVar7 - 1;
  uVar5 = 0;
  if (uVar4 != 0) {
    uVar5 = uVar4 - 1;
  }
  if (uVar7 <= uVar5) {
    uVar5 = uVar7;
  }
  if (uVar7 == 0) {
    local_1c8._0_4_ = 0;
    local_1c8._4_4_ = 0;
    local_1d8._0_4_ = 0.0;
    local_1b8._0_4_ = 0.0;
  }
  else {
    local_1b8 = 0;
    uStack_1b0 = 0;
    fVar21 = in_stack_00000030 * 0.0;
    local_118 = *in_stack_00000028;
    uStack_110 = in_stack_00000028[1];
    fVar1 = *(float *)(in_stack_00000028 + 2);
    fVar2 = *(float *)((longlong)in_stack_00000028 + 0x14);
    fVar3 = *(float *)((longlong)in_stack_00000028 + 0x1c);
    local_128 = in_stack_00000028[4];
    uStack_120 = in_stack_00000028[5];
    local_138 = in_stack_00000028[6];
    uStack_130 = in_stack_00000028[7];
    local_148 = in_stack_00000028[0x10];
    uStack_140 = in_stack_00000028[0x11];
    local_158 = in_stack_00000028[0x12];
    uStack_150 = in_stack_00000028[0x13];
    local_168 = in_stack_00000028[0x14];
    uStack_160 = in_stack_00000028[0x15];
    local_178 = in_stack_00000028[0x16];
    uStack_170 = in_stack_00000028[0x17];
    uStack_180 = 0;
    fVar22 = 0.0;
    fVar23 = 0.0;
    uStack_1e0 = fStack_1e4;
    pfVar6 = (float *)(*(longlong *)(in_R9 + 8) + 0x14);
    local_1c8 = 0;
    uStack_1c0 = 0;
    fVar9 = 3.4028235e+38;
    uVar4 = 0;
    local_1d8 = 0;
    uStack_1d0 = 0;
    fStack_194 = in_stack_00000030;
    uStack_190 = 0;
    fStack_1e4 = in_XMM1_Da;
    local_188 = fVar21;
    fStack_184 = fVar21;
    local_198 = fVar21;
    do {
      if ((uVar8 == uVar4) || (uVar5 == uVar4)) break;
      fVar24 = (float)*(undefined8 *)(pfVar6 + -5) + fVar21;
      fVar25 = (float)((ulonglong)*(undefined8 *)(pfVar6 + -5) >> 0x20) + in_stack_00000030;
      fVar15 = (float)*(undefined8 *)(pfVar6 + -2) + fVar21;
      fVar16 = (float)((ulonglong)*(undefined8 *)(pfVar6 + -2) >> 0x20) + in_stack_00000030;
      fVar26 = *pfVar6 + local_188;
      fVar27 = pfVar6[-3] + fStack_184;
      fVar28 = uStack_180._4_4_ + 0.0;
      fVar10 = fVar27 * (float)local_168 + fVar25 * (float)local_158 + fVar24 * (float)local_148 +
               (float)local_178;
      fVar11 = fVar27 * local_168._4_4_ + fVar25 * local_158._4_4_ + fVar24 * local_148._4_4_ +
               local_178._4_4_;
      fVar12 = fVar27 * (float)uStack_160 + fVar25 * (float)uStack_150 + fVar24 * (float)uStack_140
               + (float)uStack_170;
      fVar13 = fVar12 * uStack_120._4_4_ + fVar11 * fVar3 + fVar10 * uStack_110._4_4_ +
               uStack_130._4_4_;
      fVar17 = fVar26 * (float)local_168 + fVar16 * (float)local_158 + fVar15 * (float)local_148 +
               (float)local_178;
      fVar18 = fVar26 * local_168._4_4_ + fVar16 * local_158._4_4_ + fVar15 * local_148._4_4_ +
               local_178._4_4_;
      fVar19 = fVar26 * (float)uStack_160 + fVar16 * (float)uStack_150 + fVar15 * (float)uStack_140
               + (float)uStack_170;
      fVar20 = fVar19 * uStack_120._4_4_ + fVar18 * fVar3 + fVar17 * uStack_110._4_4_ +
               uStack_130._4_4_;
      fVar14 = (fVar12 * local_128._4_4_ + fVar11 * fVar2 + fVar10 * local_118._4_4_ +
               local_138._4_4_) / fVar13;
      fVar11 = ((fVar12 * (float)local_128 + fVar11 * fVar1 + fVar10 * (float)local_118 +
                (float)local_138) / fVar13) * extraout_XMM0_Da;
      fVar10 = (fVar19 * local_128._4_4_ + fVar18 * fVar2 + fVar17 * local_118._4_4_ +
               local_138._4_4_) / fVar20;
      fVar12 = ((fVar19 * (float)local_128 + fVar18 * fVar1 + fVar17 * (float)local_118 +
                (float)local_138) / fVar20) * extraout_XMM0_Da;
      fVar17 = fVar10 - fVar14;
      fVar18 = fVar12 - fVar11;
      fVar13 = fVar18 * fVar18 + fVar17 * fVar17;
      if (fVar13 <= 1e-20) {
        fVar13 = 1e-20;
      }
      fVar13 = ((fStack_1e4 - fVar11) * fVar18 + (local_1e8 - fVar14) * fVar17) / fVar13;
      fVar17 = 0.0;
      if (0.0 <= fVar13) {
        fVar17 = fVar13;
      }
      fVar13 = 1.0;
      if (fVar17 <= 1.0) {
        fVar13 = fVar17;
      }
      fVar10 = (fVar13 * fVar10 + (1.0 - fVar13) * fVar14) - local_1e8;
      fVar11 = (fVar13 * fVar12 + fVar11 * (1.0 - fVar13)) - fStack_1e4;
      fVar10 = SQRT(fVar11 * fVar11 + fVar10 * fVar10);
      if (fVar10 < fVar9) {
        ::utils::lerp();
        ::utils::lerp();
        fVar9 = extraout_XMM0_Da_01;
        if (extraout_XMM0_Da_01 <= 1e-10) {
          fVar9 = 1e-10;
        }
        fVar21 = 0.0;
        if (0.0 <= extraout_XMM0_Da_00 / fVar9) {
          fVar21 = extraout_XMM0_Da_00 / fVar9;
        }
        fVar9 = 1.0;
        if (fVar21 <= 1.0) {
          fVar9 = fVar21;
        }
        ::utils::lerp();
        local_1b8 = CONCAT44(extraout_XMM0_Db,extraout_XMM0_Da_02);
        uStack_1b0 = CONCAT44(extraout_XMM0_Dd,extraout_XMM0_Dc);
        fVar21 = 1.0 - fVar9;
        fVar28 = fVar28 * 0.0;
        local_1d8 = CONCAT44(fVar27 * fVar21,fVar27 * fVar21 + fVar26 * fVar9);
        uStack_1d0 = CONCAT44(fVar28,fVar28);
        local_1c8 = CONCAT44(fVar16 * fVar9 + fVar25 * fVar21,fVar15 * fVar9 + fVar24 * fVar21);
        uStack_1c0 = CONCAT44((fVar23 + 0.0) * 0.0 + (fVar23 + 0.0) * 0.0,
                              (fVar22 + 0.0) * 0.0 + (fVar22 + 0.0) * 0.0);
        fVar9 = fVar10;
        fVar21 = local_198;
        in_stack_00000030 = fStack_194;
        fVar22 = (float)uStack_190;
        fVar23 = uStack_190._4_4_;
      }
      uVar4 = uVar4 + 1;
      pfVar6 = pfVar6 + 3;
    } while (uVar7 != uVar4);
    local_1ec = (float)local_1b8;
    if (((float)local_1b8 < 0.0) || (1.0 < (float)local_1b8)) {
      local_1a8 = &local_1ec;
      local_1a0 = core::fmt::float::impl$5::fmt;
      local_108 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
      local_100 = 1;
      local_e8 = 0;
      local_f8 = &local_1a8;
      local_f0 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    if (NAN((float)local_1b8)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
  }
  *in_RCX = (float)local_1b8;
  *(ulonglong *)(in_RCX + 1) = CONCAT44(local_1c8._4_4_,(undefined4)local_1c8);
  in_RCX[3] = (float)local_1d8;
  return;
}
