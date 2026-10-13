
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141206ed0(longlong *param_1,float param_2,undefined8 param_3,undefined *param_4)

{
  code *pcVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined4 uVar6;
  longlong *plVar8;
  undefined *puVar9;
  longlong *unaff_RSI;
  ulonglong unaff_RDI;
  ulonglong uVar10;
  longlong in_R10;
  ulonglong uVar11;
  float fVar12;
  float fVar13;
  float unaff_XMM6_Da;
  float unaff_XMM7_Da;
  float fVar14;
  undefined1 uStack_129;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  ulonglong uStack_110;
  undefined8 uStack_108;
  longlong *plStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  longlong lStack_e0;
  longlong lStack_d8;
  ulonglong uStack_d0;
  undefined1 *puStack_c8;
  ulonglong uStack_c0;
  ulonglong uStack_b8;
  char cStack_b0;
  char cStack_a8;
  undefined8 uStack_a0;
  ulonglong uStack_98;
  float fStack_90;
  float fStack_8c;
  longlong **pplStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  ulonglong uStack_68;
  undefined8 uStack_60;
  longlong lVar7;
  
  if (param_2 < 0.0) {
    puVar9 = &UNK_142c1a090;
    lVar7 = 0x21;
    plVar8 = (longlong *)FUN_1428d9430();
  }
  else {
    puVar9 = (undefined *)param_1[1];
    if (puVar9 == (undefined *)0x0) {
      return;
    }
    lStack_e0 = *param_1;
    unaff_XMM7_Da = *(float *)((longlong)param_1 + 0x1c);
    unaff_RDI = (ulonglong)*(uint *)(param_1 + 3);
    lVar7 = lStack_e0 + (longlong)puVar9 * 0xc;
    in_R10 = (longlong)puVar9 - unaff_RDI;
    plVar8 = param_1 + 2;
    uStack_f8 = 0;
    uStack_f0 = 0;
    uStack_e8 = 0;
    uStack_128 = 0;
    param_4 = (undefined *)(in_R10 + -2);
    puStack_c8 = &uStack_129;
    uStack_c0 = 0;
    cStack_b0 = '\0';
    cStack_a8 = '\0';
    unaff_RSI = param_1;
    unaff_XMM6_Da = param_2;
    plStack_100 = plVar8;
    uStack_b8 = unaff_RDI;
    if (param_4 == (undefined *)0x0) goto LAB_141206fa4;
  }
  lVar7 = lVar7 + (2 - in_R10) * 0xc;
  param_1 = unaff_RSI;
  if (puVar9 <= param_4) {
    return;
  }
LAB_141206fa4:
  uStack_d0 = 0;
  lStack_d8 = lVar7 + -0xc;
  fStack_90 = (float)(*(code *)*plVar8)(unaff_RDI + 1);
  uStack_a0 = 1;
  uStack_98 = unaff_RDI + 1;
  fStack_8c = param_2;
  pplStack_88 = &plStack_100;
  FUN_141fcf5b0(&uStack_80,&uStack_a0);
  uStack_108 = uStack_60;
  uVar4 = uStack_108;
  uStack_118 = uStack_70;
  uVar2 = uStack_118;
  uStack_110 = uStack_68;
  uStack_128 = CONCAT44(uStack_7c,uStack_80);
  uStack_120 = CONCAT44(uStack_74,uStack_78);
  if (uStack_128 != 0) {
    lVar7 = uStack_120 - 1;
    uVar6 = (undefined4)lVar7;
    if (uStack_120 == 0) {
LAB_1412072fc:
      FUN_1428d9310(&UNK_142c1a0d0);
      pcVar1 = (code *)swi(3);
      (*pcVar1)();
      return;
    }
    uStack_108._0_4_ = (float)uStack_60;
    uStack_118._0_4_ = (float)uStack_70;
    uStack_108._4_4_ = (float)((ulonglong)uStack_60 >> 0x20);
    uStack_118._4_4_ = (float)((ulonglong)uStack_70 >> 0x20);
    fVar13 = (uStack_108._4_4_ - uStack_118._4_4_) * (uStack_108._4_4_ - uStack_118._4_4_) +
             ((float)uStack_108 - (float)uStack_118) * ((float)uStack_108 - (float)uStack_118);
    fVar12 = SQRT(fVar13);
    fVar14 = (unaff_XMM7_Da + _DAT_14295a0e4 + _DAT_142925904) * fVar12 - unaff_XMM6_Da;
    if (fVar14 < 0.0) {
      uVar11 = uStack_128;
      uStack_118 = uVar2;
      uStack_108 = uVar4;
      do {
        uVar5 = uStack_b8;
        uVar3 = uStack_d0;
        *(int *)(param_1 + 3) = (int)lVar7;
        *(undefined4 *)((longlong)param_1 + 0x1c) = 0;
        if (cStack_a8 != '\0') {
          return;
        }
        if (cStack_b0 != '\0') {
          return;
        }
        if (uStack_b8 < uStack_c0) {
          return;
        }
        if (uStack_c0 < uStack_b8) {
          uStack_b8 = uStack_b8 - 1;
        }
        else {
          cStack_b0 = '\x01';
        }
        if (uStack_d0 == 0) {
          if (lStack_e0 == lStack_d8) {
            return;
          }
        }
        else {
          uStack_d0 = 0;
          uVar10 = ((ulonglong)(lStack_d8 - lStack_e0) >> 2) * -0x5555555555555555;
          lStack_d8 = lStack_d8 + uVar3 * -0xc;
          if (uVar10 < uVar3 || uVar10 - uVar3 == 0) {
            return;
          }
        }
        lStack_d8 = lStack_d8 + -0xc;
        fVar12 = (float)(*(code *)*plStack_100)(uVar5);
        uVar2 = uStack_108;
        uVar3 = uStack_110;
        if ((uVar11 & 1) == 0) {
          uStack_98 = uVar5;
          uStack_a0 = 1;
          fStack_90 = fVar12;
          fStack_8c = fVar13;
          pplStack_88 = &plStack_100;
          FUN_141fcf5b0(&uStack_80,&uStack_a0);
          uStack_108 = uStack_60;
          uStack_120 = CONCAT44(uStack_74,uStack_78);
          uStack_110 = uStack_68;
          if (CONCAT44(uStack_7c,uStack_80) != 1) {
            return;
          }
          fVar12 = (float)uStack_60;
          fVar13 = (float)((ulonglong)uStack_60 >> 0x20);
          uVar11 = 1;
          uStack_128 = 1;
          uStack_118 = uStack_70;
        }
        else {
          uStack_110 = uVar5;
          uStack_108 = CONCAT44(fVar13,fVar12);
          uStack_120 = uVar3;
          uStack_118 = uVar2;
        }
        if (uStack_120 == 0) goto LAB_1412072fc;
        lVar7 = uStack_120 - 1;
        uVar6 = (undefined4)lVar7;
        fVar12 = fVar12 - (float)uStack_118;
        fVar13 = fVar13 - (float)((ulonglong)uStack_118 >> 0x20);
        fVar13 = fVar13 * fVar13 + fVar12 * fVar12;
        fVar12 = SQRT(fVar13);
        fVar14 = fVar14 + fVar12;
      } while (fVar14 < 0.0);
    }
    *(undefined4 *)(param_1 + 3) = uVar6;
    *(float *)((longlong)param_1 + 0x1c) = fVar14 / fVar12;
  }
  return;
}

