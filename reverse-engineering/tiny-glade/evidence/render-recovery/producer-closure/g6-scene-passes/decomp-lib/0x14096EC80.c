
ulonglong FUN_14096ec80(undefined8 *param_1,undefined8 param_2)

{
  int *piVar1;
  byte bVar2;
  undefined1 uVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  byte *pbVar9;
  ulonglong uVar10;
  longlong lVar11;
  undefined8 *unaff_RSI;
  undefined8 *puVar12;
  byte *unaff_R13;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  byte *pbStack_50;
  undefined8 uStack_48;
  
  uStack_f8 = param_1[2];
  uStack_108 = *param_1;
  uStack_100 = param_1[1];
  uStack_c8 = param_1[5];
  uStack_d8 = param_1[3];
  uStack_d0 = param_1[4];
  uStack_e0 = param_1[8];
  uStack_f0 = param_1[6];
  uStack_e8 = param_1[7];
  uStack_90 = param_1[0x16];
  uStack_88 = param_1[0x17];
  puStack_80 = (undefined *)param_1[0x18];
  uStack_78 = param_1[0x19];
  uStack_70 = param_1[0x1a];
  uStack_68 = param_1[0x1b];
  uStack_60 = param_1[0x1c];
  uStack_58 = param_1[0x1d];
  pbStack_50 = (byte *)param_1[0x1e];
  uStack_48 = param_1[0x1f];
  uStack_b0 = param_1[0xb];
  uStack_c0 = param_1[9];
  uStack_b8 = param_1[10];
  uStack_a8 = 1;
  uStack_a4 = (undefined4)param_2;
  uStack_118 = param_1[0x14];
  uStack_110 = param_1[0x15];
  uStack_158 = *(undefined4 *)(param_1 + 0xc);
  uStack_154 = *(undefined4 *)((longlong)param_1 + 100);
  uStack_150 = *(undefined4 *)(param_1 + 0xd);
  uStack_14c = *(undefined4 *)((longlong)param_1 + 0x6c);
  uStack_148 = *(undefined4 *)(param_1 + 0xe);
  uStack_144 = *(undefined4 *)((longlong)param_1 + 0x74);
  uStack_140 = *(undefined4 *)(param_1 + 0xf);
  uStack_13c = *(undefined4 *)((longlong)param_1 + 0x7c);
  uStack_138 = *(undefined4 *)(param_1 + 0x10);
  uStack_134 = *(undefined4 *)((longlong)param_1 + 0x84);
  uStack_130 = *(undefined4 *)(param_1 + 0x11);
  uStack_12c = *(undefined4 *)((longlong)param_1 + 0x8c);
  uStack_128 = *(undefined4 *)(param_1 + 0x12);
  uStack_124 = *(undefined4 *)((longlong)param_1 + 0x94);
  uStack_120 = *(undefined4 *)(param_1 + 0x13);
  uStack_11c = *(undefined4 *)((longlong)param_1 + 0x9c);
  uStack_a0 = (undefined4)((ulonglong)param_2 >> 0x20);
  uStack_9c = 1;
  uVar10 = FUN_1409dd4d0(&uStack_158,&uStack_108);
  if (*(char *)((longlong)param_1 + 0x136) == '\b') {
    return uVar10;
  }
  puVar12 = param_1 + 0x24;
  uStack_48 = 0xfffffffffffffffe;
  uVar10 = param_1[0x25];
  if (uVar10 == 0) {
    puStack_80 = &UNK_140be25a3;
    FUN_1428d9430(&UNK_142b12c50,0x2a,&UNK_142b12c80);
code_r0x000140be25a3:
    puStack_80 = &UNK_140be25b1;
    FUN_1428cd210(unaff_R13);
    puVar12 = unaff_RSI;
  }
  else {
    LOCK();
    piVar1 = (int *)(uVar10 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 != 0) goto code_r0x000140be2572;
    unaff_R13 = (byte *)*puVar12;
    LOCK();
    bVar2 = *unaff_R13;
    if (bVar2 == 0) {
      *unaff_R13 = 1;
    }
    UNLOCK();
    unaff_RSI = puVar12;
    if (bVar2 != 0) goto code_r0x000140be25a3;
  }
  puStack_80 = &UNK_140be2502;
  FUN_140c81c70(unaff_R13);
  uVar6 = puVar12[1];
  uVar5 = *(undefined4 *)(puVar12 + 2);
  uVar3 = *(undefined1 *)((longlong)puVar12 + 0x16);
  uVar4 = *(undefined2 *)((longlong)puVar12 + 0x14);
  lVar7 = *(longlong *)(unaff_R13 + 0x18);
  pbStack_50 = unaff_R13;
  if (lVar7 == *(longlong *)(unaff_R13 + 8)) {
    puStack_80 = &UNK_140be2535;
    FUN_140562630(unaff_R13 + 8,&UNK_142b12c38);
  }
  pbVar9 = pbStack_50;
  lVar8 = *(longlong *)(pbStack_50 + 0x10);
  lVar11 = lVar7 * 0x10;
  *(undefined8 *)(lVar8 + lVar11) = uVar6;
  *(undefined4 *)(lVar8 + 8 + lVar11) = uVar5;
  *(undefined2 *)(lVar8 + 0xc + lVar11) = uVar4;
  *(undefined1 *)(lVar8 + 0xe + lVar11) = uVar3;
  *(longlong *)(pbStack_50 + 0x18) = lVar7 + 1;
  puStack_80 = &UNK_140be2566;
  FUN_140c81e10();
  uVar10 = 1;
  LOCK();
  bVar2 = *pbVar9;
  if (bVar2 == 1) {
    *pbVar9 = 0;
  }
  else {
    uVar10 = (ulonglong)bVar2;
  }
  UNLOCK();
  if (bVar2 != 1) {
    puStack_80 = &UNK_140be25c0;
    uVar10 = FUN_1428cdb90(pbVar9,0);
  }
code_r0x000140be2572:
  puVar12[1] = 0;
  return uVar10;
}

