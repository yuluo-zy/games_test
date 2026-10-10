
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1421a6d40(undefined8 *param_1,longlong *param_2,longlong param_3)

{
  undefined4 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  code *pcVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  ulonglong uVar16;
  longlong *plVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  float fVar21;
  float fVar22;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined1 uStack_a8;
  undefined7 uStack_a7;
  ulonglong *puStack_a0;
  
  puStack_a0 = (ulonglong *)*param_1;
  lVar3 = param_1[1];
  uVar13 = *puStack_a0;
  uVar18 = 0;
  uVar19 = uVar13 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar13 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar19 = uVar18;
  }
  uVar16 = uVar13 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar13 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar16 = uVar18;
  }
  lVar14 = uVar19 * 0x20 + *(longlong *)(lVar3 + 8);
  uVar13 = *(ulonglong *)(lVar3 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar3 + 0x10) < uVar19) {
    lVar14 = 8;
    uVar13 = uVar18;
  }
  lVar20 = uVar16 * 0x20 + *(longlong *)(lVar3 + 0x28);
  uVar19 = *(ulonglong *)(lVar3 + 0x30) - uVar16;
  if (*(ulonglong *)(lVar3 + 0x30) < uVar16) {
    lVar20 = 8;
    uVar19 = uVar18;
  }
  *puStack_a0 = (*(longlong *)(lVar3 + 0x40) - uVar19) - uVar13;
  fVar10 = _DAT_1429a07d0;
  fVar9 = _DAT_142925988;
  fVar8 = _DAT_142925904;
  plVar17 = *(longlong **)(param_3 + 8);
  if (plVar17 == (longlong *)0x0) {
    plVar17 = *(longlong **)(param_3 + 0x10);
  }
  lVar3 = uVar13 * 0x20 + lVar14;
  lVar2 = uVar19 * 0x20 + lVar20;
  lVar4 = *param_2;
  lVar5 = param_2[1];
  lVar15 = lVar20;
  if (lVar14 == 0) goto LAB_1421a6e50;
LAB_1421a6e40:
  lVar15 = lVar20;
  if (lVar14 == lVar3) goto LAB_1421a6e50;
  uVar13 = uVar18;
  lVar15 = lVar14;
  lVar14 = lVar14 + 0x20;
  do {
    *puStack_a0 = *puStack_a0 + 1;
    uVar19 = *(ulonglong *)(lVar15 + 8);
    uVar16 = uVar19 & 0xffffffff;
    if (*(ulonglong *)(lVar5 + 0x10) <= uVar16) {
LAB_1421a703e:
      uStack_b8 = 1;
LAB_1421a704d:
      uStack_b8 = uVar13 << 0x20 | uStack_b8;
      uStack_b0 = uVar19;
      FUN_1428d9760(&UNK_142ff6b30,0x2b,&uStack_b8,&UNK_142ff6b10,&UNK_142ff6c70);
      pcVar7 = (code *)swi(3);
      (*pcVar7)();
      return;
    }
    uVar18 = uVar19 >> 0x20;
    if (*(int *)(*(longlong *)(lVar5 + 8) + uVar16 * 0x14) != (int)(uVar19 >> 0x20))
    goto LAB_1421a703e;
    lVar15 = *(longlong *)(lVar5 + 8) + uVar16 * 0x14;
    uVar16 = (ulonglong)*(uint *)(lVar15 + 4);
    if (uVar16 == 0xffffffff) goto LAB_1421a703e;
    if ((*(ulonglong *)(lVar4 + 0x38) <= uVar16) ||
       ((*(ulonglong *)(*(longlong *)(lVar4 + 0x28) + (ulonglong)(*(uint *)(lVar15 + 4) >> 6) * 8)
         >> (uVar16 & 0x3f) & 1) == 0)) {
      uStack_b8 = 0;
      uVar13 = uVar16;
      goto LAB_1421a704d;
    }
    uVar12 = 0;
    if ((*(byte *)(*(longlong *)
                    (*(longlong *)
                      (*(longlong *)(lVar5 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar15 + 0xc) * 0x48) + 0x10 +
                    ~*(ulonglong *)
                      (*(longlong *)
                        (*(longlong *)(lVar5 + 0x1a8) + 0x38 +
                        (ulonglong)*(uint *)(lVar15 + 0xc) * 0x48) +
                      *(longlong *)(lVar4 + 0x110) * 8) * 0x30) + 8 +
                  (ulonglong)*(uint *)(lVar15 + 0x10) * 0x58) & 1) == 0) {
      fVar21 = (float)func_0x000142923f50(0);
      fVar22 = fVar8;
      if (fVar8 <= fVar21) {
        fVar22 = fVar21;
      }
      fVar21 = fVar9;
      if (fVar22 <= fVar9) {
        fVar21 = fVar22;
      }
      uVar11 = 0;
      if (0.0 <= fVar21) {
        uVar11 = (undefined4)(longlong)fVar21;
      }
      uVar12 = 0xffffffff;
      if (fVar21 <= fVar10) {
        uVar12 = uVar11;
      }
    }
    uVar12 = func_0x000140833250(uVar12);
    func_0x00014082bd60(&uStack_b8,uVar12);
    uStack_a8 = 9;
    lVar15 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar15) < 0x38) {
      FUN_1428dacd0(plVar17,lVar15,0x38,1,1);
    }
    lVar6 = plVar17[1];
    *(code **)(lVar6 + lVar15) = FUN_140dceea0;
    puVar1 = (undefined4 *)(lVar6 + 8 + lVar15);
    *puVar1 = (undefined4)uStack_b8;
    puVar1[1] = uStack_b8._4_4_;
    puVar1[2] = (undefined4)uStack_b0;
    puVar1[3] = uStack_b0._4_4_;
    *(ulonglong *)(lVar6 + 0x18 + lVar15) = CONCAT71(uStack_a7,uStack_a8);
    *(undefined1 *)(lVar6 + 0x20 + lVar15) = 0;
    *(ulonglong *)(lVar6 + 0x28 + lVar15) = uVar19;
    *(code **)(lVar6 + 0x30 + lVar15) = FUN_140dcfa80;
    plVar17[2] = lVar15 + 0x38;
    lVar15 = lVar20;
    if (lVar14 != 0) goto LAB_1421a6e40;
LAB_1421a6e50:
    if ((lVar15 == 0) || (lVar15 == lVar2)) {
      return;
    }
    lVar20 = lVar15 + 0x20;
    lVar14 = 0;
    uVar13 = uVar18;
  } while( true );
}

