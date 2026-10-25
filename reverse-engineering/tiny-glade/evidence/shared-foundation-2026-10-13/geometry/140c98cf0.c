utils::geometry::rectangle::Rectangle2d::as_curve2

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140c98cf0(longlong *param_1,undefined8 *param_2)

{
  code *pcVar1;
  undefined8 *puVar2;
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
  longlong lStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong lStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
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
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e700);
  }
  else {
    fVar10 = (float)*param_2;
    fVar11 = (float)((ulonglong)*param_2 >> 0x20);
    fVar7 = (float)param_2[1];
    fVar8 = (float)((ulonglong)param_2[1] >> 0x20);
    puVar2 = (undefined8 *)func_0x000140613c10(0x20,4);
    if (puVar2 != (undefined8 *)0x0) {
      fVar3 = (float)((uint)fVar11 ^ _DAT_1429258f0);
      fVar5 = fVar6 * _DAT_14295a0e8;
      fVar4 = _DAT_14295a0e8 * fVar9;
      fVar12 = fVar4 * fVar3 + fVar5 * fVar10 + fVar7;
      fVar13 = fVar4 * fVar10 + fVar5 * fVar11 + fVar8;
      fVar6 = fVar6 * _DAT_142925870;
      fVar9 = fVar9 * _DAT_142925870;
      *puVar2 = CONCAT44(fVar13,fVar12);
      *(float *)(puVar2 + 1) = fVar5 * fVar10 + fVar9 * fVar3 + fVar7;
      *(float *)((longlong)puVar2 + 0xc) = fVar5 * fVar11 + fVar9 * fVar10 + fVar8;
      *(float *)(puVar2 + 2) = fVar6 * fVar10 + fVar9 * fVar3 + fVar7;
      *(float *)((longlong)puVar2 + 0x14) = fVar6 * fVar11 + fVar9 * fVar10 + fVar8;
      puVar2[3] = CONCAT44(fVar4 * fVar10 + fVar6 * fVar11 + fVar8,
                           fVar4 * fVar3 + fVar6 * fVar10 + fVar7);
      uStack_90 = 4;
      uStack_80 = 4;
      uStack_71 = 1;
      uStack_88 = puVar2;
      FUN_140562e70(&uStack_90,&UNK_142b2e718);
      uStack_88[4] = CONCAT44(fVar13,fVar12);
      uStack_80 = 5;
      uStack_a8 = (undefined4)uStack_90;
      uStack_a4 = uStack_90._4_4_;
      uStack_a0 = (undefined4)uStack_88;
      uStack_9c = uStack_88._4_4_;
      uStack_98 = 5;
      uStack_71 = 0;
      FUN_1419401b0(&lStack_e0,&uStack_a8,0);
      if (!SBORROW8(0,lStack_e0)) {
        param_1[6] = lStack_b0;
        param_1[4] = lStack_c0;
        param_1[5] = lStack_b8;
        param_1[2] = CONCAT44(uStack_cc,uStack_d0);
        param_1[3] = lStack_c8;
        *param_1 = lStack_e0;
        param_1[1] = CONCAT44(uStack_d4,uStack_d8);
        return param_1;
      }
      goto LAB_140c98ee1;
    }
  }
  FUN_1428d9000(4,0x20);
LAB_140c98ee1:
  uStack_a8 = uStack_d8;
  uStack_a4 = uStack_d4;
  uStack_a0 = uStack_d0;
  uStack_9c = uStack_cc;
  uStack_71 = 0;
  FUN_1428d9760(&UNK_142e714b0,0x2b,&uStack_a8,&UNK_142e71490,&UNK_142b2e730);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

