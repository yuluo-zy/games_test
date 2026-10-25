
void FUN_140bd5120(undefined4 *param_1,longlong param_2,undefined1 param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  longlong lVar8;
  undefined4 uStack_34;
  
  uVar2 = *(undefined8 *)(param_1 + 4);
  uVar5 = *param_1;
  uVar6 = *(undefined8 *)(param_1 + 1);
  uVar7 = *(undefined8 *)(param_1 + 2);
  lVar3 = *(longlong *)(param_2 + 0x28);
  if (lVar3 == *(longlong *)(param_2 + 0x18)) {
    FUN_1405629f0(param_2 + 0x18,&UNK_142b0f928);
  }
  lVar4 = *(longlong *)(param_2 + 0x20);
  lVar8 = lVar3 * 0x30;
  *(undefined4 *)(lVar4 + lVar8) = 1;
  puVar1 = (undefined8 *)(lVar4 + 4 + lVar8);
  *puVar1 = CONCAT44(uVar5,uStack_34);
  puVar1[1] = uVar6;
  puVar1 = (undefined8 *)(lVar4 + 0x10 + lVar8);
  *puVar1 = uVar7;
  puVar1[1] = uVar2;
  *(undefined1 *)(lVar4 + 0x28 + lVar8) = param_3;
  *(longlong *)(param_2 + 0x28) = lVar3 + 1;
  return;
}

