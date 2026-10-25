// addr=0x140ade8e0 name=iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>
// signature=void __cdecl iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>(void)


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
country_core::utils::dirty_tiles::DirtyTiles::
iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl_0::rasterize_world_space_rect::closure_env_0>
          (void)

{
  ulonglong uVar1;
  ulonglong *puVar2;
  uint uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  uint uVar7;
  longlong in_RDX;
  uint uVar8;
  ulonglong uVar9;
  float fVar10;
  float fVar11;
  float fVar14;
  float fVar15;
  undefined1 extraout_var [12];
  undefined1 extraout_var_00 [12];
  undefined1 auVar12 [16];
  undefined1 extraout_var_01 [12];
  undefined1 extraout_var_02 [12];
  undefined1 auVar13 [16];
  int iVar16;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar17 [16];
  
  fVar10 = (float)fmaf();
  fVar11 = (float)fmaf();
  fVar11 = floorf(fVar11);
  fVar14 = extraout_var._0_4_;
  auVar20._4_4_ = fVar14;
  auVar20._0_4_ = fVar11;
  fVar10 = floorf(fVar10);
  auVar13 = ___xmm_0000000000000000427c0000427c0000;
  auVar20._8_4_ = fVar14;
  fVar15 = extraout_var_00._0_4_;
  auVar20._12_4_ = fVar15;
  auVar12._8_8_ = auVar20._8_8_;
  auVar12._4_4_ = fVar10;
  auVar12._0_4_ = fVar11;
  auVar12 = maxps(ZEXT416(0),auVar12);
  auVar21._0_4_ = (float)(-(uint)(!NAN(fVar11) && !NAN(fVar11)) & auVar12._0_4_);
  auVar21._4_4_ = (float)(-(uint)(!NAN(fVar10) && !NAN(fVar10)) & auVar12._4_4_);
  auVar21._8_4_ = -(uint)(!NAN(fVar14) && !NAN(fVar14)) & auVar12._8_4_;
  auVar21._12_4_ = -(uint)(!NAN(fVar15) && !NAN(fVar15)) & auVar12._12_4_;
  uVar7 = (uint)__xmm_0000000000000000427c0000427c0000;
  uVar8 = __xmm_0000000000000000427c0000427c0000._4_4_;
  auVar12 = minps(___xmm_0000000000000000427c0000427c0000,auVar21);
  fVar10 = (float)(~-(uint)NAN(auVar21._0_4_) & auVar12._0_4_ | uVar7 & -(uint)NAN(auVar21._0_4_));
  fVar11 = (float)(~-(uint)NAN(auVar21._4_4_) & auVar12._4_4_ | uVar8 & -(uint)NAN(auVar21._4_4_));
  uVar3 = (uint)(longlong)fVar10;
  if (fVar10 < 0.0) {
    uVar3 = 0;
  }
  if (4.294967e+09 < fVar10) {
    uVar3 = 0xffffffff;
  }
  uVar5 = (longlong)fVar11 & 0xffffffff;
  if (fVar11 < 0.0) {
    uVar5 = 0;
  }
  if (4.294967e+09 < fVar11) {
    uVar5 = 0xffffffff;
  }
  fVar10 = (float)fmaf();
  fVar11 = (float)fmaf();
  fVar11 = ceilf(fVar11);
  fVar14 = extraout_var_01._0_4_;
  auVar18._4_4_ = fVar14;
  auVar18._0_4_ = fVar11;
  fVar10 = ceilf(fVar10);
  auVar18._8_4_ = fVar14;
  fVar15 = extraout_var_02._0_4_;
  auVar18._12_4_ = fVar15;
  auVar17._8_8_ = auVar18._8_8_;
  auVar17._4_4_ = fVar10;
  auVar17._0_4_ = fVar11;
  auVar12 = maxps(ZEXT816(0),auVar17);
  auVar19._0_4_ = (float)(-(uint)(!NAN(fVar11) && !NAN(fVar11)) & auVar12._0_4_);
  auVar19._4_4_ = (float)(-(uint)(!NAN(fVar10) && !NAN(fVar10)) & auVar12._4_4_);
  auVar19._8_4_ = -(uint)(!NAN(fVar14) && !NAN(fVar14)) & auVar12._8_4_;
  auVar19._12_4_ = -(uint)(!NAN(fVar15) && !NAN(fVar15)) & auVar12._12_4_;
  auVar13 = minps(auVar13,auVar19);
  fVar10 = (float)(~-(uint)NAN(auVar19._0_4_) & auVar13._0_4_ | uVar7 & -(uint)NAN(auVar19._0_4_));
  fVar11 = (float)(~-(uint)NAN(auVar19._4_4_) & auVar13._4_4_ | uVar8 & -(uint)NAN(auVar19._4_4_));
  uVar7 = (uint)(longlong)fVar10;
  if (fVar10 < 0.0) {
    uVar7 = 0;
  }
  if (4.294967e+09 < fVar10) {
    uVar7 = 0xffffffff;
  }
  uVar8 = (uint)(longlong)fVar11;
  if (fVar11 < 0.0) {
    uVar8 = 0;
  }
  if (4.294967e+09 < fVar11) {
    uVar8 = 0xffffffff;
  }
  auVar13._0_4_ = -(uint)((int)(uVar3 ^ 0x80000000) < (int)(uVar7 ^ 0x80000000));
  iVar16 = -(uint)((int)((uint)uVar5 ^ 0x80000000) < (int)(uVar8 ^ 0x80000000));
  auVar13._4_4_ = auVar13._0_4_;
  auVar13._8_4_ = iVar16;
  auVar13._12_4_ = iVar16;
  iVar16 = movmskpd(uVar7,auVar13);
  if (iVar16 == 3) {
    uVar4 = 0x40;
    if (0x40 < uVar3) {
      uVar4 = (ulonglong)uVar3;
    }
    uVar9 = 0x40;
    if (0x40 < (uint)uVar5) {
      uVar9 = uVar5;
    }
    do {
      if (uVar5 == uVar9) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar1 = uVar5 + 1;
      uVar6 = (ulonglong)uVar3;
      iVar16 = uVar7 - uVar3;
      do {
        if (uVar4 == uVar6) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        puVar2 = (ulonglong *)(in_RDX + uVar5 * 8);
        *puVar2 = *puVar2 | 1L << ((byte)uVar6 & 0x3f);
        uVar6 = uVar6 + 1;
        iVar16 = iVar16 + -1;
      } while (iVar16 != 0);
      uVar5 = uVar1;
    } while (uVar1 != uVar8);
  }
  return;
}


