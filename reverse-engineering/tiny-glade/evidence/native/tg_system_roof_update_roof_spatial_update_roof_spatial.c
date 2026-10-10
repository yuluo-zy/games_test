
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1421a7710(longlong *param_1,undefined8 *param_2,undefined8 *param_3,longlong *param_4)

{
  longlong lVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  uint uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  undefined8 uVar7;
  longlong lVar8;
  longlong lVar9;
  undefined4 *puVar10;
  longlong lVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  longlong lVar14;
  code *pcVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  char cVar21;
  undefined1 uVar22;
  ulonglong uVar23;
  longlong lVar24;
  longlong lVar25;
  longlong lVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  longlong lVar30;
  undefined8 *puVar31;
  longlong lVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  longlong lVar35;
  longlong lVar36;
  undefined4 uVar37;
  float fVar38;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 **ppuStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 *puStack_68;
  code *pcStack_60;
  
  puVar5 = (ulonglong *)*param_3;
  lVar6 = param_3[1];
  uVar23 = *puVar5;
  uVar33 = 0;
  uVar27 = uVar23 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar23 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar27 = uVar33;
  }
  uVar34 = uVar23 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar23 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar34 = uVar33;
  }
  lVar32 = uVar27 * 0x10 + *(longlong *)(lVar6 + 8);
  uVar23 = *(ulonglong *)(lVar6 + 0x10) - uVar27;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar27) {
    lVar32 = 8;
    uVar23 = uVar33;
  }
  lVar35 = uVar34 * 0x10 + *(longlong *)(lVar6 + 0x28);
  uVar27 = *(ulonglong *)(lVar6 + 0x30) - uVar34;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar34) {
    lVar35 = 8;
    uVar27 = uVar33;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar27) - uVar23;
  fVar20 = _DAT_142a7c878;
  lVar36 = uVar23 * 0x10 + lVar32;
  lVar30 = uVar27 * 0x10 + lVar35;
  uVar7 = *param_2;
  lVar6 = *param_1;
  lVar8 = param_1[1];
  uVar2 = *(undefined4 *)((longlong)param_1 + 0x14);
  uVar3 = *(undefined4 *)((longlong)param_4 + 0x1c);
  lVar9 = *param_4;
  puVar10 = (undefined4 *)param_4[2];
  do {
    do {
      do {
        if ((lVar32 == 0) || (lVar32 == lVar36)) {
          if ((lVar35 == 0) || (lVar35 == lVar30)) {
            return;
          }
          lVar32 = 0;
          lVar25 = lVar35;
          lVar35 = lVar35 + 0x10;
        }
        else {
          lVar25 = lVar32;
          lVar32 = lVar32 + 0x10;
        }
        *puVar5 = *puVar5 + 1;
        uVar23 = FUN_140a922b0(uVar7,*(undefined8 *)(lVar25 + 8));
      } while (((uVar23 >> 0x20 == 0) ||
               (uVar27 = uVar23 & 0xffffffff, *(ulonglong *)(lVar8 + 0x10) <= uVar27)) ||
              (*(int *)(*(longlong *)(lVar8 + 8) + uVar27 * 0x14) != (int)(uVar23 >> 0x20)));
      lVar1 = *(longlong *)(lVar8 + 8) + uVar27 * 0x14;
      uVar27 = (ulonglong)*(uint *)(lVar1 + 4);
    } while (((uVar27 == 0xffffffff) || (*(ulonglong *)(lVar6 + 0x38) <= uVar27)) ||
            ((*(ulonglong *)
               (*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8) >>
              (uVar27 & 0x3f) & 1) == 0));
    puVar31 = (undefined8 *)(lVar25 + 8);
    uVar27 = (ulonglong)*(uint *)(lVar1 + 0x10);
    lVar25 = *(longlong *)
              (*(longlong *)(lVar8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar1 = *(longlong *)
             (*(longlong *)(lVar8 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar26 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
    lVar11 = *(longlong *)(lVar25 + 0x28 + lVar26);
    lVar24 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
    lVar1 = *(longlong *)(lVar25 + 0x28 + lVar24);
    lVar26 = uVar27 * 0x58 + *(longlong *)(lVar25 + 0x10 + lVar26);
    lVar24 = uVar27 * 0x30 + *(longlong *)(lVar25 + 0x10 + lVar24);
    lVar25 = FUN_140a91ef0(uVar7,*puVar31,&UNK_142ff6d30);
    if (lVar25 == 0) {
      pcStack_60 = FUN_140b34ec0;
      uStack_98 = &UNK_142ff6d98;
      uStack_90 = 2;
      uStack_78 = 0;
      ppuStack_88 = &puStack_68;
      uStack_80 = 1;
      puStack_68 = puVar31;
      FUN_1428d9390(&uStack_98,&UNK_142ff6db8);
      pcVar15 = (code *)swi(3);
      (*pcVar15)();
      return;
    }
    uVar37 = FUN_140b2a620(lVar25);
    *(undefined4 *)(lVar11 + uVar27 * 4) = uVar2;
    *(undefined4 *)(lVar26 + 0x4c) = uVar37;
    cVar21 = func_0x0001408c5670(lVar25 + 0x299);
    if (*(char *)(lVar26 + 0x54) != cVar21) {
      uVar22 = func_0x0001408c5670(lVar25 + 0x299);
      *(undefined4 *)(lVar11 + uVar27 * 4) = uVar2;
      *(undefined1 *)(lVar26 + 0x54) = uVar22;
      uVar12 = *puVar31;
      *puVar10 = uVar3;
      lVar29 = *(longlong *)(lVar9 + 0x30);
      uVar13 = *(undefined8 *)(lVar9 + 0x40);
      if (lVar29 == *(longlong *)(lVar9 + 0x20)) {
        FUN_140562c30(lVar9 + 0x20,&UNK_142b72d70);
      }
      lVar14 = *(longlong *)(lVar9 + 0x28);
      lVar28 = lVar29 * 0x20;
      *(undefined8 *)(lVar14 + lVar28) = uVar13;
      *(ulonglong *)(lVar14 + 8 + lVar28) = uVar23;
      *(undefined8 *)(lVar14 + 0x10 + lVar28) = uVar12;
      *(undefined1 *)(lVar14 + 0x18 + lVar28) = 2;
      *(longlong *)(lVar9 + 0x30) = lVar29 + 1;
      *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    }
    uVar4 = *(uint *)(lVar25 + 600);
    if (uVar4 < 2) {
      fVar16 = *(float *)(lVar25 + 0x25c);
      fVar17 = *(float *)(lVar25 + 0x260);
      fVar18 = *(float *)(lVar25 + 0x264);
      fVar19 = *(float *)(lVar25 + 0x268);
      uVar12 = *(undefined8 *)(lVar25 + 0x26c);
      if (uVar4 == *(uint *)(lVar26 + 8)) {
        if ((uVar4 & 1) == 0) {
          if ((fVar16 == *(float *)(lVar26 + 0xc)) &&
             (!NAN(fVar16) && !NAN(*(float *)(lVar26 + 0xc)))) {
            if ((fVar17 == *(float *)(lVar26 + 0x10)) &&
               (!NAN(fVar17) && !NAN(*(float *)(lVar26 + 0x10)))) {
LAB_1421a7ae8:
              if ((fVar18 == *(float *)(lVar26 + 0x14)) &&
                 (!NAN(fVar18) && !NAN(*(float *)(lVar26 + 0x14)))) {
                if ((fVar19 == *(float *)(lVar26 + 0x18)) &&
                   (!NAN(fVar19) && !NAN(*(float *)(lVar26 + 0x18)))) goto LAB_1421a7b9c;
              }
            }
          }
        }
        else if ((fVar16 == *(float *)(lVar26 + 0xc)) &&
                (!NAN(fVar16) && !NAN(*(float *)(lVar26 + 0xc)))) {
          if ((fVar17 == *(float *)(lVar26 + 0x10)) &&
             (!NAN(fVar17) && !NAN(*(float *)(lVar26 + 0x10)))) {
            if (((float)uVar12 == *(float *)(lVar26 + 0x1c)) &&
               (!NAN((float)uVar12) && !NAN(*(float *)(lVar26 + 0x1c)))) {
              fVar38 = (float)((ulonglong)uVar12 >> 0x20);
              if ((fVar38 == *(float *)(lVar26 + 0x20)) &&
                 (!NAN(fVar38) && !NAN(*(float *)(lVar26 + 0x20)))) goto LAB_1421a7ae8;
            }
          }
        }
      }
      *(undefined4 *)(lVar11 + uVar27 * 4) = uVar2;
      *(uint *)(lVar26 + 8) = uVar4;
      *(float *)(lVar26 + 0xc) = fVar16;
      *(float *)(lVar26 + 0x10) = fVar17;
      *(float *)(lVar26 + 0x14) = fVar18;
      *(float *)(lVar26 + 0x18) = fVar19;
      *(undefined8 *)(lVar26 + 0x1c) = uVar12;
      uVar12 = *puVar31;
      *puVar10 = uVar3;
      lVar25 = *(longlong *)(lVar9 + 0x30);
      uVar13 = *(undefined8 *)(lVar9 + 0x40);
      if (lVar25 == *(longlong *)(lVar9 + 0x20)) {
        FUN_140562c30(lVar9 + 0x20,&UNK_142b72d70);
      }
      lVar11 = *(longlong *)(lVar9 + 0x28);
      lVar29 = lVar25 * 0x20;
      *(undefined8 *)(lVar11 + lVar29) = uVar13;
      *(ulonglong *)(lVar11 + 8 + lVar29) = uVar23;
      *(undefined8 *)(lVar11 + 0x10 + lVar29) = uVar12;
      *(undefined1 *)(lVar11 + 0x18 + lVar29) = 2;
      *(longlong *)(lVar9 + 0x30) = lVar25 + 1;
      *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    }
LAB_1421a7b9c:
    cVar21 = func_0x0001408e3250(lVar26);
    if (cVar21 == '\0') {
      func_0x0001408e3220(&uStack_98,lVar26);
      *(undefined4 *)(lVar1 + uVar27 * 4) = uVar2;
      *(undefined4 *)(lVar24 + 0x10) = (undefined4)uStack_98;
      *(float *)(lVar24 + 0x14) = uStack_98._4_4_ + fVar20;
      *(undefined4 *)(lVar24 + 0x18) = (undefined4)uStack_90;
    }
    else {
      *(undefined4 *)(lVar1 + uVar27 * 4) = uVar2;
      *(undefined4 *)(lVar24 + 0x18) = 0;
      *(undefined8 *)(lVar24 + 0x10) = 0;
    }
  } while( true );
}

