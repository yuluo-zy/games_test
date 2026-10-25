
uint FUN_140befb00(longlong param_1,int *param_2,undefined8 param_3,undefined8 param_4)

{
  longlong lVar1;
  uint uVar2;
  undefined8 uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  code *pcVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  bool bVar10;
  undefined1 auStack_70 [24];
  uint auStack_58 [2];
  undefined8 uStack_50;
  undefined8 uStack_48;
  longlong lStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar1 = param_1 + 0x810;
  LOCK();
  bVar10 = *(char *)(param_1 + 0x810) == '\0';
  if (bVar10) {
    *(char *)(param_1 + 0x810) = '\x01';
  }
  UNLOCK();
  if (!bVar10) {
    FUN_1428cd210(lVar1,param_2,1000000000);
  }
  lStack_40 = lVar1;
  FUN_140c81c70(lVar1);
  uVar2 = *(uint *)(param_1 + 0x848);
  *(uint *)(param_1 + 0x848) = uVar2 + 1;
  auStack_58[0] = uVar2;
  uStack_50 = param_3;
  uStack_48 = param_4;
  FUN_1418ad4c0(auStack_70,param_1 + 0x818,uVar2 & 0xfff,auStack_58);
  if (*param_2 != 1) {
    FUN_1428d9430(&UNK_142b15b78,0x4c,&UNK_142b15bf0);
                    /* WARNING: Does not return */
    pcVar6 = (code *)invalidInstructionException();
    (*pcVar6)();
  }
  uVar3 = *(undefined8 *)(param_2 + 0x12);
  puVar4 = *(ulonglong **)(param_1 + 0x58);
  if (*(int *)((longlong)puVar4 + 0xc) == *(int *)(param_1 + 0x60)) {
    uVar5 = *(ulonglong *)(param_2 + 2);
    uVar7 = *puVar4;
    do {
      uVar9 = uVar5;
      if (uVar5 < uVar7) {
        uVar9 = uVar7;
      }
      LOCK();
      uVar8 = *puVar4;
      bVar10 = uVar7 == uVar8;
      if (bVar10) {
        *puVar4 = uVar9;
        uVar8 = uVar7;
      }
      UNLOCK();
      uVar7 = uVar8;
    } while (!bVar10);
  }
  (**(code **)(param_1 + 0x3a0))
            (uVar3,*(undefined8 *)(*(longlong *)(param_1 + 0x48) + 0x20),0,4,uVar2);
  lVar1 = lStack_40;
  FUN_140c81e10(lStack_40);
  LOCK();
  bVar10 = *(char *)(param_1 + 0x810) == '\x01';
  if (bVar10) {
    *(char *)(param_1 + 0x810) = '\0';
  }
  UNLOCK();
  if (!bVar10) {
    FUN_1428cdb90(lVar1,0);
  }
  return uVar2;
}

