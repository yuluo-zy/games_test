
ulonglong * FUN_1418cd4d0(ulonglong *param_1,longlong *param_2)

{
  longlong lVar1;
  longlong lVar2;
  code *pcVar3;
  uint uVar4;
  int iVar5;
  ulonglong uVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  uint uStack_2c;
  
  uStack_2c = 0;
  lVar2 = *param_2;
  iVar5 = (**(code **)(lVar2 + 0x48))(*(undefined8 *)(lVar2 + 0x108),&uStack_2c,0);
  uVar4 = uStack_2c;
  if ((iVar5 == 5) || (iVar5 == 0)) {
    uVar8 = (ulonglong)uStack_2c;
    lVar1 = uVar8 * 8;
    if (uVar8 == 0) {
      uVar6 = 8;
    }
    else {
      uVar6 = func_0x000140613c10(lVar1,8);
      if (uVar6 == 0) {
        FUN_1428d8fe3(8,lVar1,&UNK_142d08bc8);
        pcVar3 = (code *)swi(3);
        puVar7 = (ulonglong *)(*pcVar3)();
        return puVar7;
      }
    }
    iVar5 = (**(code **)(lVar2 + 0x48))(*(undefined8 *)(lVar2 + 0x108),&uStack_2c,uVar6);
    if ((iVar5 == 5) || (iVar5 == 0)) {
      *param_1 = uVar8;
      param_1[1] = uVar6;
      param_1[2] = (ulonglong)uStack_2c;
    }
    else {
      *(int *)(param_1 + 1) = iVar5;
      *param_1 = 0x8000000000000000;
      if (uVar4 != 0) {
        func_0x000140613c20(uVar6,lVar1,8);
      }
    }
  }
  else {
    *(int *)(param_1 + 1) = iVar5;
    *param_1 = 0x8000000000000000;
  }
  return param_1;
}

