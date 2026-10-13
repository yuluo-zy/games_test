
void FUN_140df3d10(longlong param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  char acStack_39 [9];
  
  acStack_39[1] = -2;
  acStack_39[2] = -1;
  acStack_39[3] = -1;
  acStack_39[4] = -1;
  acStack_39[5] = -1;
  acStack_39[6] = -1;
  acStack_39[7] = -1;
  acStack_39[8] = -1;
  acStack_39[0] = '\0';
  FUN_141a4ff40(param_1 + 0x18,param_2,acStack_39);
  if (acStack_39[0] == '\0') {
    uVar1 = *param_2;
    uVar2 = param_2[1];
    uVar3 = param_2[2];
    uVar4 = param_2[3];
    lVar5 = *(longlong *)(param_1 + 0x28);
    if (lVar5 == *(longlong *)(param_1 + 0x18)) {
      FUN_140562c30(param_1 + 0x18,&UNK_142b78e58);
    }
    lVar6 = *(longlong *)(param_1 + 0x20);
    lVar7 = lVar5 * 0x20;
    *(undefined8 *)(lVar6 + lVar7) = uVar1;
    *(undefined8 *)(lVar6 + 8 + lVar7) = uVar2;
    *(undefined8 *)(lVar6 + 0x10 + lVar7) = uVar3;
    *(undefined8 *)(lVar6 + 0x18 + lVar7) = uVar4;
    *(longlong *)(param_1 + 0x28) = lVar5 + 1;
  }
  else if (param_2[1] != 0) {
    func_0x000140613c20(param_2[2],param_2[1],1);
  }
  return;
}

