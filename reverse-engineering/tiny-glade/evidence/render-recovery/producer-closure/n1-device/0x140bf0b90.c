
void FUN_140bf0b90(ulonglong *param_1,longlong param_2)

{
  ulonglong uVar1;
  code *pcVar2;
  ulonglong uVar3;
  undefined4 uVar4;
  int iVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  int iStack_20;
  undefined4 uStack_1c;
  undefined1 uStack_18;
  undefined1 uStack_17;
  
  iVar5 = (**(code **)(param_2 + 0x478))(*(undefined8 *)(param_2 + 0x5c8),param_1[0x11],&iStack_20);
  uVar4 = uStack_1c;
  if (iVar5 != 0) {
    iStack_20 = iVar5;
    FUN_1428d9760(&UNK_142b15188,0x2b,&iStack_20,&UNK_142b15128,&UNK_142b15f40);
    pcVar2 = (code *)swi(3);
    (*pcVar2)();
    return;
  }
  uVar3 = CONCAT44(uStack_1c,iStack_20);
  if ((param_1[0xd] & 1) == 0) {
    uStack_18 = 1;
    uVar7 = 9;
  }
  else {
    iStack_20._1_3_ = (undefined3)iStack_20;
    iStack_20 = CONCAT31(iStack_20._1_3_,*(undefined1 *)((longlong)param_1 + 0x6c));
    uStack_1c = (undefined4)(uVar3 >> 0x18);
    uStack_18 = (undefined1)((uint)uVar4 >> 0x18);
    uStack_17 = 2;
    uVar7 = 10;
  }
  FUN_1418c1380(0x143634e40,&iStack_20,uVar7);
  param_1[0x10] = uVar3;
  uVar6 = param_1[3];
  if (uVar6 != 0) {
    uVar1 = *param_1;
    uVar8 = param_1[2];
    do {
      uVar6 = uVar6 - 1;
      if (uVar8 < uVar1) {
        uVar9 = *(ulonglong *)(param_1[1] + uVar8 * 8);
      }
      else {
        uVar9 = *(ulonglong *)(param_1[1] + (uVar8 - uVar1) * 8);
      }
      if (uVar3 < uVar9) {
        return;
      }
      uVar9 = uVar1;
      if (uVar8 + 1 < uVar1) {
        uVar9 = 0;
      }
      uVar8 = (uVar8 + 1) - uVar9;
      param_1[2] = uVar8;
      param_1[3] = uVar6;
    } while (uVar6 != 0);
  }
  return;
}

