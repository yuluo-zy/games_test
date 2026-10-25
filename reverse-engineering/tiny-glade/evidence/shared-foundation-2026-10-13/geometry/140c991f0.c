utils::geometry::rectangle::Rectangle2d::as_curve3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140c991f0(longlong *param_1,undefined8 *param_2,float param_3)

{
  code *pcVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  longlong lStack_f0;
  float fStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  float fStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined1 uStack_81;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  fVar8 = *(float *)(param_2 + 2);
  fVar11 = *(float *)((longlong)param_2 + 0x14);
  if (fVar8 * fVar11 <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e778);
  }
  else {
    fVar9 = (float)*param_2;
    fVar10 = (float)((ulonglong)*param_2 >> 0x20);
    fVar6 = (float)param_2[1];
    fVar7 = (float)((ulonglong)param_2[1] >> 0x20);
    pfVar2 = (float *)func_0x000140613c10(0x30,4);
    if (pfVar2 != (float *)0x0) {
      fVar5 = fVar8 * _DAT_142925870;
      fVar4 = fVar11 * _DAT_14295a0e8;
      fVar3 = (float)((uint)fVar10 ^ _DAT_1429258f0);
      fVar11 = fVar11 * _DAT_142925870;
      fVar8 = fVar8 * _DAT_14295a0e8;
      *pfVar2 = fVar6 + fVar4 * fVar3 + fVar8 * fVar9;
      pfVar2[1] = param_3;
      *(ulonglong *)(pfVar2 + 2) =
           CONCAT44(fVar11 * fVar3 + fVar8 * fVar9 + fVar6,fVar4 * fVar9 + fVar8 * fVar10 + fVar7);
      pfVar2[4] = param_3;
      *(ulonglong *)(pfVar2 + 5) =
           CONCAT44(fVar11 * fVar3 + fVar5 * fVar9 + fVar6,fVar11 * fVar9 + fVar8 * fVar10 + fVar7);
      pfVar2[7] = param_3;
      *(ulonglong *)(pfVar2 + 8) =
           CONCAT44(fVar4 * fVar3 + fVar5 * fVar9 + fVar6,fVar11 * fVar9 + fVar5 * fVar10 + fVar7);
      pfVar2[10] = param_3;
      pfVar2[0xb] = fVar7 + fVar4 * fVar9 + fVar5 * fVar10;
      uStack_a0 = 4;
      uStack_90 = 4;
      fStack_e8 = pfVar2[2];
      lStack_f0 = *(longlong *)pfVar2;
      uStack_81 = 1;
      uStack_98 = pfVar2;
      FUN_140562cf0(&uStack_a0,&UNK_142b2e790);
      uStack_98[0xe] = fStack_e8;
      *(longlong *)(uStack_98 + 0xc) = lStack_f0;
      uStack_90 = 5;
      fStack_b8 = (float)uStack_a0;
      uStack_b4 = uStack_a0._4_4_;
      uStack_b0 = (undefined4)uStack_98;
      uStack_ac = uStack_98._4_4_;
      uStack_a8 = 5;
      uStack_81 = 0;
      FUN_141940470(&lStack_f0,&fStack_b8,0);
      if (!SBORROW8(0,lStack_f0)) {
        param_1[6] = lStack_c0;
        param_1[4] = lStack_d0;
        param_1[5] = lStack_c8;
        param_1[2] = CONCAT44(uStack_dc,uStack_e0);
        param_1[3] = lStack_d8;
        *param_1 = lStack_f0;
        param_1[1] = CONCAT44(uStack_e4,fStack_e8);
        return param_1;
      }
      goto LAB_140c9945a;
    }
  }
  FUN_1428d9000(4,0x30);
LAB_140c9945a:
  fStack_b8 = fStack_e8;
  uStack_b4 = uStack_e4;
  uStack_b0 = uStack_e0;
  uStack_ac = uStack_dc;
  uStack_81 = 0;
  FUN_1428d9760(&UNK_142e714b0,0x2b,&fStack_b8,&UNK_142e71490,&UNK_142b2e7a8);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

