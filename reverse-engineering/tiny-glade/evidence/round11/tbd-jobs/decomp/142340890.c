// 0x142340890 undefined FUN_142340890(void) body_bytes=1152

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142340890(longlong *param_1,undefined8 *param_2,longlong *param_3,undefined8 *param_4)

{
  uint *puVar1;
  ulonglong uVar2;
  char cVar3;
  uint uVar4;
  longlong lVar5;
  float fVar6;
  longlong lVar7;
  ulonglong *puVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 *puVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong lVar15;
  bool bVar16;
  bool bVar17;
  longlong lVar18;
  uint *puVar19;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  float fVar22;
  float fVar23;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  longlong lStack_98;
  longlong lStack_90;
  longlong lStack_88;
  undefined4 *puStack_80;
  undefined8 uStack_78;
  float *pfStack_70;
  
  fVar6 = _DAT_142925904;
  lStack_98 = *param_1;
  lVar5 = param_1[1];
  uStack_9c = *(undefined4 *)((longlong)param_1 + 0x14);
  puVar19 = *(uint **)(lStack_98 + 0xf8);
  puVar1 = puVar19 + *(longlong *)(lStack_98 + 0x100);
  cVar3 = *(char *)(lStack_98 + 0x108);
  lStack_88 = *param_3;
  pfStack_70 = (float *)*param_4;
  uStack_a0 = *(undefined4 *)((longlong)param_2 + 0x1c);
  uStack_78 = *param_2;
  puStack_80 = (undefined4 *)param_2[2];
  lStack_90 = 8;
  bVar17 = false;
  lStack_b8 = 0;
  uVar13 = 0;
  uVar14 = 0;
  lVar18 = 0;
  lStack_c0 = 0;
  bVar16 = false;
  do {
    if (cVar3 == '\0') {
      if (uVar13 == uVar14) {
        if (puVar19 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0);
          if (uVar14 != 0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == puVar1) {
            return;
          }
        }
        lVar10 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar19 * 0xa0;
        uVar9 = (ulonglong)*(uint *)(lVar10 + 0x94);
        lVar18 = *(longlong *)(lVar5 + 0x1a8);
        uVar13 = *(ulonglong *)(lVar18 + 0x40 + uVar9 * 0x48);
        lVar7 = *(longlong *)(lVar18 + 0x18 + uVar9 * 0x48);
        lVar18 = *(longlong *)(lVar18 + 0x38 + uVar9 * 0x48);
        uVar9 = *(ulonglong *)(lStack_98 + 0x118);
        uVar2 = *(ulonglong *)(lVar10 + 0x88);
        if ((uVar9 < uVar2) && (*(longlong *)(*(longlong *)(lVar10 + 0x80) + uVar9 * 8) != 0)) {
          bVar17 = true;
          if ((uVar13 <= uVar9) || (uVar9 = *(ulonglong *)(lVar18 + uVar9 * 8), uVar9 == 0)) {
            lStack_b8 = 0;
            uVar9 = *(ulonglong *)(lStack_98 + 0x120);
            goto joined_r0x000142340b90;
          }
          lStack_b8 = *(longlong *)(lVar7 + 0x10 + ~uVar9 * 0x30);
          uVar9 = *(ulonglong *)(lStack_98 + 0x120);
          if (uVar9 < uVar2) goto LAB_142340ba9;
LAB_142340be4:
          bVar16 = false;
        }
        else {
          bVar17 = false;
          uVar9 = *(ulonglong *)(lStack_98 + 0x120);
joined_r0x000142340b90:
          if (uVar2 <= uVar9) goto LAB_142340be4;
LAB_142340ba9:
          if (*(longlong *)(*(longlong *)(lVar10 + 0x80) + uVar9 * 8) == 0) goto LAB_142340be4;
          bVar16 = true;
          if ((uVar9 < uVar13) && (uVar13 = *(ulonglong *)(lVar18 + uVar9 * 8), uVar13 != 0)) {
            lStack_c0 = *(longlong *)(lVar7 + 0x10 + ~uVar13 * 0x30);
          }
          else {
            lStack_c0 = 0;
          }
        }
        lVar11 = ~*(ulonglong *)(lVar18 + *(longlong *)(lStack_98 + 0x110) * 8) * 0x30;
        lVar18 = *(longlong *)(lVar7 + 0x10 + lVar11);
        lStack_b0 = *(longlong *)(lVar7 + 0x28 + lVar11);
        puVar19 = puVar19 + 1;
        lStack_90 = *(longlong *)(lVar10 + 0x50);
        uVar13 = 0;
      }
      uVar9 = (ulonglong)*(uint *)(lStack_90 + 8 + uVar13 * 0x10);
    }
    else {
      if (uVar13 == uVar14) {
        do {
          if (puVar19 == puVar1) {
            return;
          }
          uVar4 = *puVar19;
          puVar19 = puVar19 + 1;
          uVar14 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)uVar4 * 0x48);
        } while (uVar14 == 0);
        lVar7 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)uVar4 * 0x48;
        lVar10 = *(longlong *)(lVar7 + 0x18);
        lVar11 = *(longlong *)(lVar7 + 0x38);
        if ((*(ulonglong *)(lStack_98 + 0x118) < *(ulonglong *)(lVar7 + 0x40)) &&
           (uVar13 = *(ulonglong *)(lVar11 + *(ulonglong *)(lStack_98 + 0x118) * 8), uVar13 != 0)) {
          lStack_b8 = *(longlong *)(lVar10 + 0x10 + ~uVar13 * 0x30);
          bVar17 = true;
        }
        else {
          bVar17 = false;
        }
        lVar15 = ~*(ulonglong *)(lVar11 + *(longlong *)(lStack_98 + 0x110) * 8) * 0x30;
        lVar18 = *(longlong *)(lVar10 + 0x10 + lVar15);
        lStack_b0 = *(longlong *)(lVar10 + 0x28 + lVar15);
        if ((*(ulonglong *)(lStack_98 + 0x120) < *(ulonglong *)(lVar7 + 0x40)) &&
           (uVar13 = *(ulonglong *)(lVar11 + *(ulonglong *)(lStack_98 + 0x120) * 8), uVar13 != 0)) {
          lStack_c0 = *(longlong *)(lVar10 + 0x10 + ~uVar13 * 0x30);
          bVar16 = true;
          uVar13 = 0;
        }
        else {
          uVar13 = 0;
          bVar16 = false;
        }
      }
      uVar9 = uVar13 & 0xffffffff;
    }
    puVar12 = (undefined4 *)(uVar9 * 0x14 + lVar18);
    lVar7 = 0;
    if (bVar17) {
      lVar7 = lStack_b8;
    }
    lVar10 = uVar9 * 600 + lStack_c0;
    if (!bVar16) {
      lVar10 = 0;
    }
    fVar23 = fVar6;
    if (lVar7 != 0) {
      fVar23 = *(float *)(lStack_88 + 0xe8);
    }
    fVar22 = *pfStack_70 * fVar23 * (float)puVar12[4] * (float)*(undefined8 *)(puVar12 + 2);
    fVar23 = *pfStack_70 *
             fVar23 * (float)puVar12[4] * (float)((ulonglong)*(undefined8 *)(puVar12 + 2) >> 0x20);
    uStack_a8 = CONCAT44(fVar23,fVar22);
    if (lVar10 != 0) {
      puVar8 = (ulonglong *)func_0x000140b90f50();
      uVar2 = *puVar8;
      auVar20._8_8_ = 0;
      auVar20._0_8_ = uVar2;
      auVar21._4_4_ = fVar23;
      auVar21._0_4_ = fVar22;
      auVar21._8_8_ = 0;
      auVar21 = maxps(auVar20,auVar21);
      uStack_a8 = CONCAT44(~-(uint)NAN(fVar23) & auVar21._4_4_,~-(uint)NAN(fVar22) & auVar21._0_4_)
                  | CONCAT44((uint)(uVar2 >> 0x20) & -(uint)NAN(fVar23),
                             (uint)uVar2 & -(uint)NAN(fVar22));
    }
    uVar13 = uVar13 + 1;
    *puStack_80 = uStack_a0;
    FUN_142376120(uStack_78,*puVar12,puVar12[1],&uStack_a8,(longlong)&uStack_a8 + 4);
    *(undefined4 *)(lStack_b0 + uVar9 * 4) = uStack_9c;
    puVar12[4] = 0x3f800000;
  } while( true );
}

