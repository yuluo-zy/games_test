
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c08560(longlong *param_1,int *param_2)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  longlong lVar3;
  uint *puVar4;
  longlong lVar5;
  undefined8 uVar6;
  code *pcVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  bool bVar13;
  undefined4 uVar14;
  undefined1 uStack_c2;
  undefined1 uStack_c1;
  undefined1 *puStack_c0;
  uint uStack_b8;
  undefined4 uStack_b4;
  int **ppiStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  uint uStack_90;
  undefined8 uStack_8c;
  undefined8 uStack_84;
  undefined2 uStack_7c;
  undefined1 uStack_7a;
  int *piStack_78;
  code *pcStack_70;
  
  puVar1 = *(ulonglong **)(*param_1 + 8);
  if (*(int *)((longlong)puVar1 + 0xc) == *(int *)(*param_1 + 0x10)) {
    if (*param_2 != 1) {
      pcStack_70 = FUN_140bed620;
      puStack_c0 = &UNK_142b14bb0;
      uStack_b8 = 1;
      uStack_b4 = 0;
      uStack_a0 = 0;
      ppiStack_b0 = &piStack_78;
      uStack_a8 = 1;
      piStack_78 = param_2;
      FUN_1428d9390(&puStack_c0,&UNK_142b14bf0);
      pcVar7 = (code *)swi(3);
      (*pcVar7)();
      return;
    }
    uVar2 = *(ulonglong *)(param_2 + 2);
    uVar10 = *puVar1;
    do {
      uVar12 = uVar2;
      if (uVar2 < uVar10) {
        uVar12 = uVar10;
      }
      LOCK();
      uVar11 = *puVar1;
      bVar13 = uVar10 == uVar11;
      if (bVar13) {
        *puVar1 = uVar12;
        uVar11 = uVar10;
      }
      UNLOCK();
      uVar10 = uVar11;
    } while (!bVar13);
  }
  uVar9 = _UNK_142b0ff48;
  uVar8 = _UNK_142b0ff40;
  lVar3 = param_1[1];
  puVar4 = (uint *)param_1[2];
  uStack_a0 = *(undefined8 *)(lVar3 + 0x30);
  uStack_90 = *puVar4;
  lVar5 = param_1[3];
  uVar6 = *(undefined8 *)(param_2 + 0x12);
  piStack_78 = (int *)0x0;
  uStack_c2 = 0;
  uStack_c1 = 0x34;
  puStack_c0 = &uStack_c2;
  uStack_b8 = 1;
  uStack_b4 = 0;
  uStack_a8 = 1;
  uStack_98 = 0xffffffffffffffff;
  uStack_8c = _UNK_142b0ff40;
  uStack_84 = _UNK_142b0ff48;
  uStack_7c = 0;
  uStack_7a = 1;
  uVar14 = 0;
  ppiStack_b0 = (int **)&uStack_c1;
  FUN_1418afd20(lVar5 + 0x78,uVar6,&piStack_78,8,0,&puStack_c0,1);
  puStack_c0 = (undefined1 *)(ulonglong)*puVar4;
  if ((*puVar4 & 6) == 0) {
    piStack_78 = (int *)0x0;
    pcStack_70 = (code *)0x0;
    ppiStack_b0._0_4_ = *(undefined4 *)param_1[5];
    uStack_b4 = 0;
    uStack_b8 = (uint)*(byte *)param_1[4];
    (**(code **)(lVar5 + 0x3a8))
              (uVar6,*(undefined8 *)(lVar3 + 0x30),7,&piStack_78,CONCAT44(uVar14,1),&puStack_c0);
  }
  else {
    piStack_78 = (int *)0x0;
    ppiStack_b0._0_4_ = *(undefined4 *)param_1[5];
    uStack_b4 = 0;
    uStack_b8 = (uint)*(byte *)param_1[4];
    (**(code **)(lVar5 + 0x3b0))
              (uVar6,*(undefined8 *)(lVar3 + 0x30),7,&piStack_78,CONCAT44(uVar14,1),&puStack_c0);
  }
  uStack_a0 = *(undefined8 *)(lVar3 + 0x30);
  uStack_90 = *puVar4;
  piStack_78 = (int *)0x0;
  uStack_c2 = 0x34;
  uStack_c1 = 0x22;
  puStack_c0 = &uStack_c2;
  uStack_b8 = 1;
  uStack_b4 = 0;
  uStack_a8 = 1;
  uStack_98 = 0xffffffffffffffff;
  uStack_8c = uVar8;
  uStack_84 = uVar9;
  uStack_7c = 0;
  uStack_7a = 0;
  ppiStack_b0 = (int **)&uStack_c1;
  FUN_1418afd20(lVar5 + 0x78,uVar6,&piStack_78,8,0,&puStack_c0,1);
  return;
}

