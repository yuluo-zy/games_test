
undefined8 *
FUN_141208d10(undefined8 *param_1,longlong *param_2,int param_3,float param_4,undefined8 param_5)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint uVar5;
  longlong lVar6;
  code *pcVar7;
  uint uVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  int iVar11;
  ulonglong unaff_RSI;
  ulonglong in_R11;
  float fVar12;
  float fVar13;
  
  uVar5 = *(uint *)(param_2 + 3);
  if (uVar5 == 0) {
LAB_141208de7:
    FUN_1428d9a20(param_5);
  }
  else {
    param_3 = param_3 + (int)param_2[2];
    if (param_3 == -0x80000000 && uVar5 == 0xffffffff) {
LAB_141208ddf:
      FUN_1428d99a0(param_5);
      goto LAB_141208de7;
    }
    uVar8 = -uVar5;
    if ((int)-uVar5 < 0) {
      uVar8 = uVar5;
    }
    in_R11 = (ulonglong)(int)((param_3 % (int)uVar5 >> 0x1f & uVar8) + param_3 % (int)uVar5);
    unaff_RSI = param_2[1];
    if (in_R11 < unaff_RSI) {
      if (uVar5 != 0xffffffff || param_3 != 0x7fffffff) {
        iVar11 = (param_3 + 1) % (int)uVar5;
        uVar9 = (ulonglong)(int)((iVar11 >> 0x1f & uVar8) + iVar11);
        if (uVar9 < unaff_RSI) {
          lVar6 = *param_2;
          uVar3 = *(undefined8 *)(lVar6 + in_R11 * 0xc);
          fVar1 = *(float *)(lVar6 + 8 + in_R11 * 0xc);
          fVar2 = *(float *)(lVar6 + 8 + uVar9 * 0xc);
          uVar4 = *(undefined8 *)(lVar6 + uVar9 * 0xc);
          fVar12 = (float)uVar3;
          fVar13 = (float)((ulonglong)uVar3 >> 0x20);
          *param_1 = CONCAT44(param_4 * ((float)((ulonglong)uVar4 >> 0x20) - fVar13) + fVar13,
                              param_4 * ((float)uVar4 - fVar12) + fVar12);
          *(float *)(param_1 + 1) = (fVar2 - fVar1) * param_4 + fVar1;
          return param_1;
        }
        goto LAB_141208dfa;
      }
      goto LAB_141208ddf;
    }
  }
  uVar9 = FUN_1428d9518(in_R11,unaff_RSI);
LAB_141208dfa:
  FUN_1428d9518(uVar9,unaff_RSI);
  pcVar7 = (code *)swi(3);
  puVar10 = (undefined8 *)(*pcVar7)();
  return puVar10;
}

