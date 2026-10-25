utils::geometry::rectangle::Rectangle2d::try_as_curve2

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c98fa0(undefined8 param_1,undefined8 *param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  fVar6 = *(float *)(param_2 + 2);
  fVar9 = *(float *)((longlong)param_2 + 0x14);
  if (fVar6 * fVar9 <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e748);
  }
  else {
    fVar10 = (float)*param_2;
    fVar11 = (float)((ulonglong)*param_2 >> 0x20);
    fVar7 = (float)param_2[1];
    fVar8 = (float)((ulonglong)param_2[1] >> 0x20);
    uStack_88 = (undefined8 *)func_0x000140613c10(0x20,4);
    if (uStack_88 != (undefined8 *)0x0) {
      fVar3 = (float)((uint)fVar11 ^ _DAT_1429258f0);
      fVar5 = fVar6 * _DAT_14295a0e8;
      fVar4 = _DAT_14295a0e8 * fVar9;
      fVar12 = fVar4 * fVar3 + fVar5 * fVar10 + fVar7;
      fVar13 = fVar4 * fVar10 + fVar5 * fVar11 + fVar8;
      fVar6 = fVar6 * _DAT_142925870;
      fVar9 = fVar9 * _DAT_142925870;
      *uStack_88 = CONCAT44(fVar13,fVar12);
      *(float *)(uStack_88 + 1) = fVar5 * fVar10 + fVar9 * fVar3 + fVar7;
      *(float *)((longlong)uStack_88 + 0xc) = fVar5 * fVar11 + fVar9 * fVar10 + fVar8;
      *(float *)(uStack_88 + 2) = fVar6 * fVar10 + fVar9 * fVar3 + fVar7;
      *(float *)((longlong)uStack_88 + 0x14) = fVar6 * fVar11 + fVar9 * fVar10 + fVar8;
      uStack_88[3] = CONCAT44(fVar4 * fVar10 + fVar6 * fVar11 + fVar8,
                              fVar4 * fVar3 + fVar6 * fVar10 + fVar7);
      uStack_90 = 4;
      uStack_80 = 4;
      uStack_71 = 1;
      FUN_140562e70(&uStack_90,&UNK_142b2e760);
      uStack_88[4] = CONCAT44(fVar13,fVar12);
      uStack_80 = 5;
      uStack_a8 = (undefined4)uStack_90;
      uStack_a4 = uStack_90._4_4_;
      uStack_a0 = (undefined4)uStack_88;
      uStack_9c = uStack_88._4_4_;
      uStack_98 = 5;
      uStack_71 = 0;
      FUN_1419401b0(param_1,&uStack_a8,0);
      return param_1;
    }
  }
  FUN_1428d9000(4,0x20);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

