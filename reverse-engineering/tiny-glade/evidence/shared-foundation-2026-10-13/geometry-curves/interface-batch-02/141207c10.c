
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141207c10(longlong *param_1,float param_2)

{
  float fVar1;
  ulonglong uVar2;
  undefined4 uVar3;
  longlong *plVar5;
  undefined *puVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong *unaff_RSI;
  undefined *unaff_RDI;
  ulonglong uVar9;
  float fVar10;
  float fVar11;
  undefined8 uVar12;
  float unaff_XMM6_Da;
  float fVar13;
  float unaff_XMM9_Da;
  undefined1 uStack_139;
  undefined8 uStack_138;
  undefined *puStack_130;
  float fStack_128;
  float fStack_124;
  longlong **pplStack_120;
  ulonglong uStack_118;
  undefined *puStack_110;
  undefined8 uStack_108;
  undefined *puStack_100;
  undefined8 uStack_f8;
  longlong *plStack_f0;
  longlong lStack_e8;
  longlong lStack_e0;
  ulonglong uStack_d8;
  undefined *puStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined1 *puStack_b0;
  char cStack_a8;
  ulonglong uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined *puVar4;
  
  if (param_2 < 0.0) {
    puVar6 = &UNK_142c1a090;
    lVar8 = 0x21;
    plVar5 = (longlong *)FUN_1428d9430(&UNK_142c1a090,0x21,&UNK_142c1a0e8);
  }
  else {
    unaff_XMM9_Da = *(float *)((longlong)param_1 + 0x24);
    unaff_RDI = (undefined *)(ulonglong)*(uint *)(param_1 + 4);
    plVar5 = param_1 + 2;
    puStack_d0 = unaff_RDI + 1;
    lVar8 = *param_1;
    puVar6 = (undefined *)param_1[1];
    lStack_e0 = lVar8 + (longlong)puVar6 * 0xc;
    uStack_c8 = 0;
    uStack_c0 = 0;
    uStack_b8 = 0;
    uStack_118 = 0;
    puStack_b0 = &uStack_139;
    cStack_a8 = '\0';
    unaff_RSI = param_1;
    unaff_XMM6_Da = param_2;
    plStack_f0 = plVar5;
    if (unaff_RDI == (undefined *)0x0) {
      if (puVar6 == (undefined *)0x0) {
        return;
      }
      goto LAB_141207cda;
    }
  }
  if (puVar6 <= unaff_RDI) {
    return;
  }
  lVar8 = lVar8 + (longlong)unaff_RDI * 0xc;
LAB_141207cda:
  uStack_d8 = 0;
  lStack_e8 = lVar8 + 0xc;
  fStack_128 = (float)(*(code *)*plVar5)(unaff_RDI);
  uStack_138 = 1;
  puStack_130 = unaff_RDI;
  fStack_124 = param_2;
  pplStack_120 = &plStack_f0;
  FUN_1412affb0(&uStack_a0,&uStack_138);
  fVar1 = _DAT_142925904;
  uStack_f8 = uStack_80;
  uStack_108 = uStack_90;
  puStack_100 = puStack_88;
  uStack_118 = uStack_a0;
  puStack_110 = puStack_98;
  if (uStack_a0 != 0) {
    uVar3 = SUB84(puStack_98,0);
    fVar10 = (float)uStack_80 - (float)uStack_90;
    fVar11 = (float)((ulonglong)uStack_80 >> 0x20) - (float)((ulonglong)uStack_90 >> 0x20);
    fVar10 = fVar10 * fVar10;
    fVar11 = SQRT(fVar11 * fVar11 + fVar10);
    fVar13 = (_DAT_142925904 - unaff_XMM9_Da) * fVar11 - unaff_XMM6_Da;
    if (fVar13 < 0.0) {
      uVar9 = uStack_a0;
      do {
        puVar6 = puStack_d0;
        uVar2 = uStack_d8;
        *(int *)(unaff_RSI + 4) = (int)puStack_110;
        *(undefined4 *)((longlong)unaff_RSI + 0x24) = 0x3f800000;
        if (cStack_a8 != '\0') {
          return;
        }
        puStack_d0 = puStack_d0 + 1;
        if (uStack_d8 == 0) {
          if (lStack_e8 == lStack_e0) {
            return;
          }
        }
        else {
          uStack_d8 = 0;
          uVar7 = ((ulonglong)(lStack_e0 - lStack_e8) >> 2) * -0x5555555555555555;
          if (uVar7 < uVar2 || uVar7 - uVar2 == 0) {
            return;
          }
          lStack_e8 = lStack_e8 + uVar2 * 0xc;
        }
        lStack_e8 = lStack_e8 + 0xc;
        fVar11 = (float)(*(code *)*plStack_f0)(puVar6);
        uVar12 = uStack_f8;
        puVar4 = puStack_100;
        if ((uVar9 & 1) == 0) {
          puStack_130 = puVar6;
          uStack_138 = 1;
          fStack_128 = fVar11;
          fStack_124 = fVar10;
          pplStack_120 = &plStack_f0;
          FUN_1412affb0(&uStack_a0,&uStack_138);
          uStack_f8 = uStack_80;
          uStack_118 = uStack_a0;
          puStack_100 = puStack_88;
          if (uStack_a0 != 1) {
            return;
          }
          fVar11 = (float)uStack_80;
          fVar10 = (float)((ulonglong)uStack_80 >> 0x20);
          uVar9 = 1;
          puVar4 = puStack_98;
          uVar12 = uStack_90;
        }
        else {
          puStack_100 = puVar6;
          uStack_f8 = CONCAT44(fVar10,fVar11);
        }
        uVar3 = SUB84(puVar4,0);
        fVar11 = fVar11 - (float)uVar12;
        fVar10 = fVar10 - (float)((ulonglong)uVar12 >> 0x20);
        fVar10 = fVar10 * fVar10 + fVar11 * fVar11;
        fVar11 = SQRT(fVar10);
        fVar13 = fVar13 + fVar11;
        puStack_110 = puVar4;
        uStack_108 = uVar12;
      } while (fVar13 < 0.0);
    }
    *(undefined4 *)(unaff_RSI + 4) = uVar3;
    *(float *)((longlong)unaff_RSI + 0x24) = fVar1 - fVar13 / fVar11;
  }
  return;
}

