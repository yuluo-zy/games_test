
undefined4 *
FUN_140bd6220(undefined4 *param_1,undefined8 *param_2,longlong *param_3,undefined1 param_4)

{
  undefined4 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined4 uStack_b6;
  undefined2 uStack_b2;
  undefined2 uStack_6e;
  undefined2 uStack_6c;
  undefined4 uStack_5c;
  
  uVar2 = param_2[3];
  uVar14 = param_2[4];
  uVar15 = param_2[6];
  *(undefined8 *)(param_1 + 5) = param_2[5];
  *(undefined8 *)(param_1 + 7) = uVar15;
  *(undefined8 *)(param_1 + 1) = uVar2;
  *(undefined8 *)(param_1 + 3) = uVar14;
  uVar2 = param_2[6];
  uVar14 = *param_2;
  uVar15 = param_2[1];
  uVar6 = *(undefined4 *)(param_2 + 2);
  uVar7 = *(undefined4 *)((longlong)param_2 + 0x14);
  uVar8 = *(undefined4 *)(param_2 + 3);
  uVar9 = *(undefined4 *)((longlong)param_2 + 0x1c);
  uVar10 = *(undefined4 *)(param_2 + 4);
  uVar11 = *(undefined4 *)((longlong)param_2 + 0x24);
  uVar12 = *(undefined4 *)(param_2 + 5);
  uVar13 = *(undefined4 *)((longlong)param_2 + 0x2c);
  lVar3 = param_3[2];
  uStack_5c = (undefined4)((ulonglong)uVar15 >> 0x20);
  uStack_6e = (undefined2)uStack_b6;
  uStack_6c = (undefined2)((uint)uStack_b6 >> 0x10);
  if (lVar3 == *param_3) {
    FUN_140b5f9f0(param_3,&UNK_142b14220);
  }
  lVar4 = param_3[1];
  *(undefined8 *)(lVar4 + 0x40 + lVar3 * 0x48) = uVar2;
  puVar1 = (undefined4 *)(lVar4 + 0x30 + lVar3 * 0x48);
  *puVar1 = uVar10;
  puVar1[1] = uVar11;
  puVar1[2] = uVar12;
  puVar1[3] = uVar13;
  puVar1 = (undefined4 *)(lVar4 + 0x20 + lVar3 * 0x48);
  *puVar1 = uVar6;
  puVar1[1] = uVar7;
  puVar1[2] = uVar8;
  puVar1[3] = uVar9;
  puVar1 = (undefined4 *)(lVar4 + 0x10 + lVar3 * 0x48);
  *puVar1 = (int)uVar14;
  puVar1[1] = (int)((ulonglong)uVar14 >> 0x20);
  puVar1[2] = (int)uVar15;
  puVar1[3] = uStack_5c;
  puVar1 = (undefined4 *)(lVar4 + lVar3 * 0x48);
  *puVar1 = 2;
  puVar1[1] = 0;
  puVar1[2] = (uint)CONCAT21(uStack_6e,param_4) << 8;
  puVar1[3] = CONCAT22(uStack_b2,uStack_6c);
  param_3[2] = lVar3 + 1;
  lVar4 = param_3[8];
  if (lVar4 == param_3[6]) {
    FUN_140562630(param_3 + 6,&UNK_142b14238);
  }
  lVar5 = param_3[7];
  *(undefined4 *)(lVar5 + lVar4 * 0x10) = 0xffffffff;
  *(undefined8 *)(lVar5 + 8 + lVar4 * 0x10) = 0;
  param_3[8] = lVar4 + 1;
  *param_1 = (int)lVar3;
  return param_1;
}

