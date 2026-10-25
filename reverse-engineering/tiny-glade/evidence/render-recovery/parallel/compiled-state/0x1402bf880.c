
longlong * FUN_1402bf880(longlong *param_1,longlong *param_2,undefined8 param_3)

{
  int *piVar1;
  int iVar2;
  longlong lVar3;
  code *pcVar4;
  longlong lVar5;
  int *piVar6;
  longlong *plVar7;
  longlong lVar8;
  int iVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lStack_60;
  int *piStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar3 = param_2[1];
  lVar12 = param_2[2];
  iVar9 = (int)lVar12 + -1;
  lVar10 = *param_2;
  do {
    lVar5 = lVar10 + 0x18;
    if (lVar10 == lVar3) {
      *param_1 = 0;
      param_1[1] = 4;
      param_1[2] = 0;
      return param_1;
    }
    *param_2 = lVar5;
    lVar12 = lVar12 + 1;
    iVar9 = iVar9 + 1;
    iVar2 = *(int *)(lVar10 + 0x10);
    param_2[2] = lVar12;
    lVar10 = lVar5;
  } while (iVar2 == -1);
  piVar6 = (int *)func_0x000140613c10(0x10,4);
  if (piVar6 != (int *)0x0) {
    *piVar6 = iVar9;
    lStack_60 = 4;
    lVar10 = 1;
    piStack_58 = piVar6;
    do {
      lVar11 = 0;
      lVar8 = lVar5;
      do {
        if (lVar8 == lVar3) {
          param_1[2] = lVar10;
          *param_1 = lStack_60;
          param_1[1] = (longlong)piStack_58;
          return param_1;
        }
        lVar5 = lVar8 + 0x18;
        lVar11 = lVar11 + -1;
        piVar1 = (int *)(lVar8 + 0x10);
        lVar8 = lVar5;
      } while (*piVar1 == -1);
      if (lVar10 == lStack_60) {
        lStack_50 = lVar10;
        FUN_1428a29e0(&lStack_60,lVar10,1,4,4);
        piVar6 = piStack_58;
      }
      iVar9 = (int)lVar12;
      lVar12 = lVar12 - lVar11;
      piVar6[lVar10] = (iVar9 + -1) - (int)lVar11;
      lVar10 = lVar10 + 1;
    } while( true );
  }
  FUN_1428d8fe3(4,0x10,param_3);
  pcVar4 = (code *)swi(3);
  plVar7 = (longlong *)(*pcVar4)();
  return plVar7;
}

