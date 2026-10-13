
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_1411f97b0(longlong *param_1,longlong param_2,float param_3)

{
  code *pcVar1;
  longlong *plVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  float fVar5;
  float fVar6;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  longlong lStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong lStack_20;
  
  if (*(ulonglong *)(param_2 + 0x10) < 2) {
    FUN_1428d9430(&UNK_142c18e90,0x28,&UNK_142c18eb8);
LAB_1411f98b2:
    FUN_1428d9430(&UNK_142c18ed0,0x27,&UNK_142c18ef8);
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_1411f98b2;
    fVar5 = *(float *)(param_2 + 0x30);
    if ((fVar5 != 0.0) || (NAN(fVar5))) {
      if (fVar5 <= _DAT_14295a0d8) {
        fVar5 = _DAT_14295a0d8;
      }
      fVar6 = param_3 / fVar5;
      if (_DAT_142925904 <= param_3 / fVar5) {
        fVar6 = _DAT_142925904;
      }
      fVar5 = (float)func_0x000142923fa0(_DAT_142925904 / fVar6);
      uVar3 = (longlong)(fVar5 - _DAT_14295a0dc) & (longlong)fVar5 >> 0x3f | (longlong)fVar5;
      if (fVar5 < 0.0) {
        uVar3 = 0;
      }
      uVar4 = 0xffffffffffffffff;
      if (fVar5 <= _DAT_14295a0e0) {
        uVar4 = uVar3;
      }
      FUN_1411fac00(&lStack_50,param_2,uVar4 + 1 + (ulonglong)(uVar4 == 0),0);
      if (!SBORROW8(0,lStack_50)) {
        param_1[6] = lStack_20;
        param_1[4] = lStack_30;
        param_1[5] = lStack_28;
        param_1[2] = CONCAT44(uStack_3c,uStack_40);
        param_1[3] = lStack_38;
        *param_1 = lStack_50;
        param_1[1] = CONCAT44(uStack_44,uStack_48);
        return param_1;
      }
      goto LAB_1411f98e2;
    }
  }
  FUN_1428d9430(&UNK_142c18f10,0x24,&UNK_142c18f38);
LAB_1411f98e2:
  uStack_68 = uStack_48;
  uStack_64 = uStack_44;
  uStack_60 = uStack_40;
  uStack_5c = uStack_3c;
  FUN_1428d9760(&UNK_142c18bc8,0x2b,&uStack_68,&UNK_142c18ba8,&UNK_142c18f68);
  pcVar1 = (code *)swi(3);
  plVar2 = (longlong *)(*pcVar1)();
  return plVar2;
}

