
undefined8 * FUN_1418ae8a0(longlong param_1,undefined8 *param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  uint uVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  undefined8 *puVar12;
  bool bVar13;
  undefined4 extraout_XMM0_Da;
  undefined4 uVar14;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  puVar12 = *(undefined8 **)(param_1 + 0x170);
  do {
    puVar10 = puVar12;
    if (puVar10 == (undefined8 *)0x0) {
      uStack_18 = param_2[0xc];
      uStack_10 = param_2[0xd];
      uStack_28 = param_2[10];
      uStack_20 = param_2[0xb];
      uStack_38 = param_2[8];
      uStack_30 = param_2[9];
      uStack_78 = *(undefined4 *)param_2;
      uStack_74 = *(undefined4 *)((longlong)param_2 + 4);
      uStack_70 = *(undefined4 *)(param_2 + 1);
      uStack_6c = *(undefined4 *)((longlong)param_2 + 0xc);
      uStack_68 = *(undefined4 *)(param_2 + 2);
      uStack_64 = *(undefined4 *)((longlong)param_2 + 0x14);
      uStack_60 = *(undefined4 *)(param_2 + 3);
      uStack_5c = *(undefined4 *)((longlong)param_2 + 0x1c);
      uStack_58 = *(undefined4 *)(param_2 + 4);
      uStack_54 = *(undefined4 *)((longlong)param_2 + 0x24);
      uStack_50 = *(undefined4 *)(param_2 + 5);
      uStack_4c = *(undefined4 *)((longlong)param_2 + 0x2c);
      uStack_48 = *(undefined4 *)(param_2 + 6);
      uStack_44 = *(undefined4 *)((longlong)param_2 + 0x34);
      uStack_40 = *(undefined4 *)(param_2 + 7);
      uStack_3c = *(undefined4 *)((longlong)param_2 + 0x3c);
      uStack_88 = 0;
      uStack_80 = 1;
      puVar12 = &uStack_88;
      uVar11 = FUN_1418bbb80(param_1,puVar12);
      if (uVar11 < *(ulonglong *)(param_1 + 0x160)) {
        uVar9 = func_0x000140c111b0(uVar11);
        uVar14 = extraout_XMM0_Da;
        if (uVar9 < 0x2c) {
          return (undefined8 *)
                 ((longlong)puVar12 * 0x80 + *(longlong *)(param_1 + (ulonglong)uVar9 * 8));
        }
      }
      else {
        uVar14 = FUN_1428d9430(&UNK_142d0528e,0x22,&UNK_142d02268);
      }
      FUN_1428d9518(uVar14,0x2c,&UNK_142d02268);
      pcVar1 = (code *)swi(3);
      puVar12 = (undefined8 *)(*pcVar1)();
      return puVar12;
    }
    LOCK();
    puVar12 = *(undefined8 **)(param_1 + 0x170);
    bVar13 = puVar10 == puVar12;
    if (bVar13) {
      *(undefined8 *)(param_1 + 0x170) = puVar10[2];
      puVar12 = puVar10;
    }
    UNLOCK();
  } while (!bVar13);
  uVar2 = param_2[0xd];
  puVar10[0xe] = param_2[0xc];
  puVar10[0xf] = uVar2;
  uVar2 = param_2[0xb];
  puVar10[0xc] = param_2[10];
  puVar10[0xd] = uVar2;
  uVar2 = param_2[9];
  puVar10[10] = param_2[8];
  puVar10[0xb] = uVar2;
  uVar2 = *param_2;
  uVar3 = param_2[1];
  uVar4 = param_2[2];
  uVar5 = param_2[3];
  uVar6 = param_2[4];
  uVar7 = param_2[5];
  uVar8 = param_2[7];
  puVar10[8] = param_2[6];
  puVar10[9] = uVar8;
  puVar10[6] = uVar6;
  puVar10[7] = uVar7;
  puVar10[4] = uVar4;
  puVar10[5] = uVar5;
  puVar10[2] = uVar2;
  puVar10[3] = uVar3;
  *(undefined4 *)(puVar10 + 1) = 1;
  *puVar10 = 0;
  return puVar10;
}

