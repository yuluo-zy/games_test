
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c08830(longlong *param_1,int *param_2)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 *puVar5;
  undefined8 uVar6;
  code *pcVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  bool bVar14;
  undefined4 uVar15;
  undefined1 uStack_c2;
  undefined1 uStack_c1;
  undefined1 *puStack_c0;
  undefined8 uStack_b8;
  int **ppiStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
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
      uStack_a0 = 0;
      ppiStack_b0 = &piStack_78;
      uStack_a8 = 1;
      piStack_78 = param_2;
      FUN_1428d9390(&puStack_c0,&UNK_142b14bf0);
      goto code_r0x000140c08ab1;
    }
    uVar2 = *(ulonglong *)(param_2 + 2);
    uVar10 = *puVar1;
    do {
      uVar13 = uVar2;
      if (uVar2 < uVar10) {
        uVar13 = uVar10;
      }
      LOCK();
      uVar11 = *puVar1;
      bVar14 = uVar10 == uVar11;
      if (bVar14) {
        *puVar1 = uVar13;
        uVar11 = uVar10;
      }
      UNLOCK();
      uVar10 = uVar11;
    } while (!bVar14);
  }
  lVar3 = param_1[1];
  lVar12 = FUN_1418aed30(*(undefined8 *)(lVar3 + 0x648),param_1[2] + 8,param_2);
  uVar9 = _UNK_142b0ff48;
  uVar8 = _UNK_142b0ff40;
  if (lVar12 != 0) {
    lVar4 = param_1[3];
    uStack_a0 = *(undefined8 *)(lVar4 + 0x30);
    uStack_c2 = *(undefined1 *)param_1[4];
    puVar5 = (undefined4 *)param_1[5];
    uStack_90 = *puVar5;
    uVar6 = *(undefined8 *)(param_2 + 0x12);
    piStack_78 = (int *)0x0;
    uStack_c1 = 0x34;
    puStack_c0 = &uStack_c2;
    uStack_b8 = 1;
    ppiStack_b0 = (int **)&uStack_c1;
    uStack_a8 = 1;
    uStack_98 = 0xffffffffffffffff;
    uStack_8c = _UNK_142b0ff40;
    uStack_84 = _UNK_142b0ff48;
    uStack_7c = 0;
    uStack_7a = 1;
    uVar15 = 0;
    FUN_1418afd20(lVar3 + 0x78,uVar6,&piStack_78,8,0,&puStack_c0,1);
    (**(code **)(lVar3 + 0x388))
              (uVar6,*(undefined8 *)(lVar12 + 0x20),*(undefined8 *)(lVar4 + 0x30),7,
               CONCAT44(uVar15,*(undefined4 *)(param_1[6] + 0x10)),*(undefined8 *)(param_1[6] + 8));
    uStack_a0 = *(undefined8 *)(lVar4 + 0x30);
    uStack_c1 = *(undefined1 *)param_1[7];
    uStack_90 = *puVar5;
    piStack_78 = (int *)0x0;
    uStack_c2 = 0x34;
    puStack_c0 = &uStack_c2;
    uStack_b8 = 1;
    ppiStack_b0 = (int **)&uStack_c1;
    uStack_a8 = 1;
    uStack_98 = 0xffffffffffffffff;
    uStack_8c = uVar8;
    uStack_84 = uVar9;
    uStack_7c = 0;
    uStack_7a = 0;
    FUN_1418afd20(lVar3 + 0x78,uVar6,&piStack_78,8,0,&puStack_c0,1);
    return;
  }
code_r0x000140c08ab1:
  FUN_1428d9310(&UNK_142b18ba0);
  pcVar7 = (code *)swi(3);
  (*pcVar7)();
  return;
}

