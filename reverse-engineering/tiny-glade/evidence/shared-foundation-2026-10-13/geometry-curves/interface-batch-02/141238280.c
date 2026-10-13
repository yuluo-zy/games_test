
undefined8 * FUN_141238280(undefined8 *param_1,int *param_2)

{
  ulonglong uVar1;
  code *pcVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined8 *puVar7;
  longlong lVar8;
  int in_XMM1_Da;
  int iStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  longlong lStack_40;
  int iStack_38;
  int iStack_34;
  int *piStack_30;
  
  if ((char)param_2[0x20] == '\0') {
    lVar8 = *(longlong *)(param_2 + 0x16);
    *(longlong *)(param_2 + 0x16) = lVar8 + 1;
    if (*(longlong *)(param_2 + 0x14) == 0) {
      func_0x0001412002f0(&iStack_70);
      if (iStack_70 != 1) goto LAB_141238297;
      lVar6 = *(longlong *)(param_2 + 0xc);
      uVar1 = *(ulonglong *)(param_2 + 0xe);
      uVar3 = FUN_141200280(uStack_6c,uStack_68);
      if (uVar1 <= uVar3) {
        FUN_1428d9518(uVar3,uVar1,&UNK_142c1a720);
        pcVar2 = (code *)swi(3);
        puVar7 = (undefined8 *)(*pcVar2)();
        return puVar7;
      }
      lVar6 = lVar6 + uVar3 * 0xc;
    }
    else {
      param_2[0x14] = 0;
      param_2[0x15] = 0;
      lVar6 = FUN_141fd30d0(param_2 + 0xc);
    }
    if (lVar6 != 0) {
      iStack_38 = (*(code *)**(undefined8 **)(param_2 + 10))(lVar8);
      if (*param_2 == 1) {
        uVar4 = *(undefined8 *)(param_2 + 6);
        uVar5 = *(undefined8 *)(param_2 + 8);
        *(longlong *)(param_2 + 6) = lVar8;
        param_2[8] = iStack_38;
        param_2[9] = in_XMM1_Da;
        *(undefined8 *)(param_2 + 2) = uVar4;
        *(undefined8 *)(param_2 + 4) = uVar5;
      }
      else {
        piStack_30 = param_2 + 10;
        uStack_48 = 1;
        lStack_40 = lVar8;
        iStack_34 = in_XMM1_Da;
        FUN_141fcf3d0(&iStack_70,&uStack_48);
        *(undefined8 *)(param_2 + 8) = uStack_50;
        *(undefined8 *)(param_2 + 4) = uStack_60;
        *(undefined8 *)(param_2 + 6) = uStack_58;
        *(ulonglong *)param_2 = CONCAT44(uStack_6c,iStack_70);
        *(ulonglong *)(param_2 + 2) = CONCAT44(uStack_64,uStack_68);
        if (*param_2 != 1) goto LAB_141238297;
        uVar4 = *(undefined8 *)(param_2 + 2);
        uVar5 = *(undefined8 *)(param_2 + 4);
        lVar8 = *(longlong *)(param_2 + 6);
        iStack_38 = (int)*(undefined8 *)(param_2 + 8);
        in_XMM1_Da = (int)((ulonglong)*(undefined8 *)(param_2 + 8) >> 0x20);
      }
      param_1[1] = uVar4;
      param_1[2] = uVar5;
      param_1[3] = lVar8;
      param_1[4] = CONCAT44(in_XMM1_Da,iStack_38);
      uVar5 = 1;
      goto LAB_1412383b2;
    }
  }
LAB_141238297:
  uVar5 = 0;
LAB_1412383b2:
  *param_1 = uVar5;
  return param_1;
}

