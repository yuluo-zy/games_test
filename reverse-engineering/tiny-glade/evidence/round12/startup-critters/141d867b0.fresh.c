
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141d867b0(longlong *param_1)

{
  uint *puVar1;
  undefined4 *puVar2;
  uint *puVar3;
  undefined4 uVar4;
  float fVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong *puVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  bool bVar15;
  bool bVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar21;
  undefined1 auVar20 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  longlong lStack_1a8;
  longlong lStack_1a0;
  longlong lStack_198;
  longlong lStack_188;
  longlong lStack_180;
  longlong lStack_168;
  uint *puStack_138;
  uint *puStack_130;
  longlong lStack_118;
  longlong lStack_108;
  longlong lStack_f8;
  undefined4 uStack_ec;
  longlong lStack_e8;
  longlong lStack_e0;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_c0;
  undefined4 uStack_b4;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  byte bStack_a0;
  
  lVar12 = *param_1;
  lVar6 = param_1[1];
  func_0x00014230ebf0(&puStack_138,lVar6,lVar12,(int)param_1[2],
                      *(undefined4 *)((longlong)param_1 + 0x14));
  fVar5 = _DAT_142925904;
  uVar4 = _DAT_142925870;
  lStack_168 = lStack_118;
  lStack_180 = lStack_108;
  lStack_188 = lStack_f8;
  lStack_198 = lStack_e8;
  lStack_1a0 = lStack_e0;
  lStack_1a8 = lStack_d8;
  if ((bStack_a0 & 1) == 0) {
    if (uStack_a8 != uStack_b0) {
LAB_141d86c6d:
      uVar7 = (ulonglong)*(uint *)(lStack_168 + 8 + uStack_a8 * 0x10);
      goto LAB_141d86c7e;
    }
    if (puStack_138 != puStack_130) {
      do {
        if (*(longlong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)*puStack_138 * 0xa0) != 0
           ) {
          lVar11 = *(longlong *)(lVar6 + 0x100) + (ulonglong)*puStack_138 * 0xa0;
          uVar7 = (ulonglong)*(uint *)(lVar11 + 0x94);
          lVar13 = *(longlong *)(lVar6 + 0x1a8) + uVar7 * 0x48;
          uVar7 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x40 + uVar7 * 0x48);
          if ((*(ulonglong *)(lVar12 + 0x110) < uVar7) &&
             (uVar10 = *(ulonglong *)
                        (*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
             uVar10 != 0)) {
            lStack_198 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar7 <= uVar10) goto LAB_141d86bcd;
LAB_141d86b91:
            uVar10 = *(ulonglong *)(*(longlong *)(lVar13 + 0x38) + uVar10 * 8);
            if (uVar10 == 0) goto LAB_141d86bcd;
            lStack_1a0 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar6 = *(longlong *)(lVar13 + 0x38);
            if (uVar7 <= uVar10) goto LAB_141d86bc7;
LAB_141d86be6:
            uVar7 = *(ulonglong *)(lVar6 + uVar10 * 8);
            lVar13 = *(longlong *)(lVar13 + 0x18);
            if (uVar7 == 0) goto LAB_141d86c0a;
            lStack_1a8 = *(longlong *)(lVar13 + 0x10 + ~uVar7 * 0x30);
          }
          else {
            lStack_198 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar10 < uVar7) goto LAB_141d86b91;
LAB_141d86bcd:
            lStack_1a0 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar6 = *(longlong *)(lVar13 + 0x38);
            if (uVar10 < uVar7) goto LAB_141d86be6;
LAB_141d86bc7:
            lVar13 = *(longlong *)(lVar13 + 0x18);
LAB_141d86c0a:
            lStack_1a8 = 0;
          }
          lVar14 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar12 + 0x128) * 8) * 0x30;
          lStack_180 = *(longlong *)(lVar13 + 0x10 + lVar14);
          lStack_188 = *(longlong *)(lVar13 + 0x28 + lVar14);
          lVar12 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar12 + 0x130) * 8) * 0x30;
          lStack_d0 = *(longlong *)(lVar13 + 0x10 + lVar12);
          lStack_c0 = *(longlong *)(lVar13 + 0x28 + lVar12);
          lStack_168 = *(longlong *)(lVar11 + 0x50);
          uStack_a8 = 0;
          goto LAB_141d86c6d;
        }
        puStack_138 = puStack_138 + 1;
      } while (puStack_138 != puStack_130);
    }
  }
  else {
    if (uStack_a8 != uStack_b0) {
LAB_141d86b74:
      uVar7 = uStack_a8 & 0xffffffff;
LAB_141d86c7e:
      puVar1 = (uint *)(lStack_1a0 + uVar7 * 0x18);
      lStack_198 = uVar7 * 0x30 + lStack_198;
      puVar2 = (undefined4 *)(lStack_c0 + uVar7 * 4);
      puVar3 = (uint *)(lStack_1a8 + uVar7 * 0x10);
      puVar8 = (ulonglong *)(uVar7 * 0x10 + lStack_180);
      uVar9 = *puVar3;
      fVar18 = *(float *)(lStack_198 + 0x18);
      fVar17 = fVar18;
      if ((uVar9 & 1) != 0) {
        fVar17 = (float)puVar3[3];
      }
      fVar17 = fVar17 - fVar18;
      uVar10 = *(ulonglong *)(lStack_198 + 0x10);
      auVar23._8_8_ = 0;
      auVar23._0_8_ = uVar10;
      auVar20._0_8_ = CONCAT44((int)(uVar9 << 0x1f) >> 0x1f,(int)(uVar9 << 0x1f) >> 0x1f);
      auVar20._8_8_ = 0;
      auVar22._8_8_ = 0;
      auVar22._0_8_ = *(ulonglong *)(puVar3 + 1) & auVar20._0_8_;
      auVar22 = ~auVar20 & auVar23 | auVar22;
      fVar19 = auVar22._0_4_ - (float)uVar10;
      fVar21 = auVar22._4_4_ - (float)(uVar10 >> 0x20);
      fVar18 = _DAT_142925904 / SQRT(fVar21 * fVar21 + fVar19 * fVar19 + fVar17 * fVar17);
      bVar15 = (int)ABS(fVar18) - 0x800000U < 0x7f000000;
      bVar16 = (int)fVar18 - 1U < 0x7fffff;
      fVar17 = fVar18 * fVar17 * _DAT_142f67d30;
      fVar19 = fVar18 * fVar19 * _DAT_142f67d40;
      fVar21 = fVar18 * fVar21 * _UNK_142f67d44;
      if (!bVar16 && (!bVar15 || -1 >= (int)fVar18)) {
        fVar17 = 0.0;
      }
      uVar9 = (uint)(bVar16 || bVar15 && -1 < (int)fVar18);
      *(undefined4 *)(lStack_188 + uVar7 * 4) = uStack_ec;
      *puVar8 = CONCAT44((int)(uVar9 << 0x1f) >> 0x1f,(int)(uVar9 << 0x1f) >> 0x1f) &
                CONCAT44(fVar21,fVar19);
      *(float *)(puVar8 + 1) = fVar17;
      fVar18 = (float)puVar1[4];
      *puVar2 = uStack_b4;
      *(float *)(lStack_d0 + uVar7 * 4) = fVar18 + fVar5;
                    /* WARNING: Could not recover jumptable at 0x000141d86de3. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&UNK_142f68018 + *(int *)(&UNK_142f68018 + (ulonglong)*puVar1 * 4)))
                (puStack_130,puVar2,uStack_a8 + 1,uVar4);
      return;
    }
    if (puStack_138 != puStack_130) {
      do {
        if (*(longlong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)*puStack_138 * 0x48) != 0
           ) {
          lVar6 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*puStack_138 * 0x48;
          uVar7 = *(ulonglong *)(lVar6 + 0x40);
          if ((*(ulonglong *)(lVar12 + 0x110) < uVar7) &&
             (uVar10 = *(ulonglong *)
                        (*(longlong *)(lVar6 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
             uVar10 != 0)) {
            lStack_198 = *(longlong *)(*(longlong *)(lVar6 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar7 <= uVar10) goto LAB_141d86add;
LAB_141d86aa1:
            uVar10 = *(ulonglong *)(*(longlong *)(lVar6 + 0x38) + uVar10 * 8);
            if (uVar10 == 0) goto LAB_141d86add;
            lStack_1a0 = *(longlong *)(*(longlong *)(lVar6 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar13 = *(longlong *)(lVar6 + 0x38);
            if (uVar7 <= uVar10) goto LAB_141d86ad7;
LAB_141d86af6:
            uVar7 = *(ulonglong *)(lVar13 + uVar10 * 8);
            lVar6 = *(longlong *)(lVar6 + 0x18);
            if (uVar7 == 0) goto LAB_141d86b1a;
            lStack_1a8 = *(longlong *)(lVar6 + 0x10 + ~uVar7 * 0x30);
          }
          else {
            lStack_198 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar10 < uVar7) goto LAB_141d86aa1;
LAB_141d86add:
            lStack_1a0 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar13 = *(longlong *)(lVar6 + 0x38);
            if (uVar10 < uVar7) goto LAB_141d86af6;
LAB_141d86ad7:
            lVar6 = *(longlong *)(lVar6 + 0x18);
LAB_141d86b1a:
            lStack_1a8 = 0;
          }
          lVar11 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar12 + 0x128) * 8) * 0x30;
          lStack_180 = *(longlong *)(lVar6 + 0x10 + lVar11);
          lStack_188 = *(longlong *)(lVar6 + 0x28 + lVar11);
          lVar12 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar12 + 0x130) * 8) * 0x30;
          lStack_d0 = *(longlong *)(lVar6 + 0x10 + lVar12);
          lStack_c0 = *(longlong *)(lVar6 + 0x28 + lVar12);
          uStack_a8 = 0;
          goto LAB_141d86b74;
        }
        puStack_138 = puStack_138 + 1;
      } while (puStack_138 != puStack_130);
    }
  }
  return;
}

