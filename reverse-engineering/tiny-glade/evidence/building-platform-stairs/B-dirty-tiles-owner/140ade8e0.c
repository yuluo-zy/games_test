
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar12;
  float fVar15;
  float fVar16;
  undefined1 extraout_var [12];
  undefined1 extraout_var_00 [12];
  undefined1 auVar13 [16];
  undefined1 extraout_var_01 [12];
  undefined1 extraout_var_02 [12];
  undefined1 auVar14 [16];
  int iVar17;
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar18 [16];
  
  fVar10 = (float)fmaf();
  fVar11 = (float)fmaf();
  fVar11 = floorf(fVar11);
  fVar15 = extraout_var._0_4_;
  auVar21._4_4_ = fVar15;
  auVar21._0_4_ = fVar11;
  fVar12 = floorf(fVar10);
  auVar14 = ___xmm_0000000000000000427c0000427c0000;
  fVar10 = __real_4f7fffff;
  auVar21._8_4_ = fVar15;
  fVar16 = extraout_var_00._0_4_;
  auVar21._12_4_ = fVar16;
  auVar13._8_8_ = auVar21._8_8_;
  auVar13._4_4_ = fVar12;
  auVar13._0_4_ = fVar11;
  auVar13 = maxps(ZEXT416(0),auVar13);
  auVar22._0_4_ = (float)(-(uint)(!NAN(fVar11) && !NAN(fVar11)) & auVar13._0_4_);
  auVar22._4_4_ = (float)(-(uint)(!NAN(fVar12) && !NAN(fVar12)) & auVar13._4_4_);
  auVar22._8_4_ = -(uint)(!NAN(fVar15) && !NAN(fVar15)) & auVar13._8_4_;
  auVar22._12_4_ = -(uint)(!NAN(fVar16) && !NAN(fVar16)) & auVar13._12_4_;
  uVar7 = (uint)__xmm_0000000000000000427c0000427c0000;
  uVar8 = __xmm_0000000000000000427c0000427c0000._4_4_;
  auVar13 = minps(___xmm_0000000000000000427c0000427c0000,auVar22);
  fVar11 = (float)(~-(uint)NAN(auVar22._0_4_) & auVar13._0_4_ | uVar7 & -(uint)NAN(auVar22._0_4_));
  fVar12 = (float)(~-(uint)NAN(auVar22._4_4_) & auVar13._4_4_ | uVar8 & -(uint)NAN(auVar22._4_4_));
  uVar3 = (uint)(longlong)fVar11;
  if (fVar11 < 0.0) {
    uVar3 = 0;
  }
  if (__real_4f7fffff < fVar11) {
    uVar3 = 0xffffffff;
  }
  uVar5 = (longlong)fVar12 & 0xffffffff;
  if (fVar12 < 0.0) {
    uVar5 = 0;
  }
  if (__real_4f7fffff < fVar12) {
    uVar5 = 0xffffffff;
  }
  fVar11 = (float)fmaf();
  fVar12 = (float)fmaf();
  fVar12 = ceilf(fVar12);
  fVar15 = extraout_var_01._0_4_;
  auVar19._4_4_ = fVar15;
  auVar19._0_4_ = fVar12;
  fVar11 = ceilf(fVar11);
  auVar19._8_4_ = fVar15;
  fVar16 = extraout_var_02._0_4_;
  auVar19._12_4_ = fVar16;
  auVar18._8_8_ = auVar19._8_8_;
  auVar18._4_4_ = fVar11;
  auVar18._0_4_ = fVar12;
  auVar13 = maxps(ZEXT816(0),auVar18);
  auVar20._0_4_ = (float)(-(uint)(!NAN(fVar12) && !NAN(fVar12)) & auVar13._0_4_);
  auVar20._4_4_ = (float)(-(uint)(!NAN(fVar11) && !NAN(fVar11)) & auVar13._4_4_);
  auVar20._8_4_ = -(uint)(!NAN(fVar15) && !NAN(fVar15)) & auVar13._8_4_;
  auVar20._12_4_ = -(uint)(!NAN(fVar16) && !NAN(fVar16)) & auVar13._12_4_;
  auVar14 = minps(auVar14,auVar20);
  fVar11 = (float)(~-(uint)NAN(auVar20._0_4_) & auVar14._0_4_ | uVar7 & -(uint)NAN(auVar20._0_4_));
  fVar12 = (float)(~-(uint)NAN(auVar20._4_4_) & auVar14._4_4_ | uVar8 & -(uint)NAN(auVar20._4_4_));
  uVar7 = (uint)(longlong)fVar11;
  if (fVar11 < 0.0) {
    uVar7 = 0;
  }
  if (fVar10 < fVar11) {
    uVar7 = 0xffffffff;
  }
  uVar8 = (uint)(longlong)fVar12;
  if (fVar12 < 0.0) {
    uVar8 = 0;
  }
  if (fVar10 < fVar12) {
    uVar8 = 0xffffffff;
  }
  auVar14._0_4_ =
       -(uint)((int)(uVar3 ^ ___xmm_80000000800000008000000080000000) <
              (int)(___xmm_80000000800000008000000080000000 ^ uVar7));
  iVar17 = -(uint)((int)((uint)uVar5 ^ _UNK_1429258f4) < (int)(_UNK_1429258f4 ^ uVar8));
  auVar14._4_4_ = auVar14._0_4_;
  auVar14._8_4_ = iVar17;
  auVar14._12_4_ = iVar17;
  iVar17 = movmskpd(uVar7,auVar14);
  if (iVar17 == 3) {
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
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      uVar1 = uVar5 + 1;
      uVar6 = (ulonglong)uVar3;
      iVar17 = uVar7 - uVar3;
      do {
        if (uVar4 == uVar6) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        puVar2 = (ulonglong *)(in_RDX + uVar5 * 8);
        *puVar2 = *puVar2 | 1L << ((byte)uVar6 & 0x3f);
        uVar6 = uVar6 + 1;
        iVar17 = iVar17 + -1;
      } while (iVar17 != 0);
      uVar5 = uVar1;
    } while (uVar1 != uVar8);
  }
  return;
}

