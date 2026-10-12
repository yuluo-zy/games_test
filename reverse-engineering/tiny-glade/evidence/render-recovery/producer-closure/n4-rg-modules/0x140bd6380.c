
void FUN_140bd6380(undefined8 *param_1,longlong param_2,undefined1 param_3)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  
  lVar2 = *(longlong *)(param_2 + 0x28);
  if (lVar2 == *(longlong *)(param_2 + 0x18)) {
    FUN_1405629f0(param_2 + 0x18,&UNK_142b0fed8);
  }
  lVar3 = *(longlong *)(param_2 + 0x20);
  lVar7 = lVar2 * 0x30;
  *(undefined4 *)(lVar3 + lVar7) = 0;
  uVar4 = param_1[1];
  uVar5 = param_1[2];
  uVar6 = param_1[3];
  puVar1 = (undefined8 *)(lVar3 + 4 + lVar7);
  *puVar1 = *param_1;
  puVar1[1] = uVar4;
  puVar1 = (undefined8 *)(lVar3 + 0x14 + lVar7);
  *puVar1 = uVar5;
  puVar1[1] = uVar6;
  *(undefined4 *)(lVar3 + 0x24 + lVar7) = *(undefined4 *)(param_1 + 4);
  *(undefined1 *)(lVar3 + 0x28 + lVar7) = param_3;
  *(longlong *)(param_2 + 0x28) = lVar2 + 1;
  return;
}

