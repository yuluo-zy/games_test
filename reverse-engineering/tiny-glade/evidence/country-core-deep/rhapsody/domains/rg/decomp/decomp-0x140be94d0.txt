
ulonglong FUN_140be94d0(ulonglong *param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined8 uStack_30;
  
  uVar2 = param_1[2];
  uVar5 = *param_2;
  uVar6 = param_2[1];
  uVar7 = param_2[2];
  uVar8 = param_2[3];
  uVar9 = param_2[4];
  uVar10 = param_2[5];
  uVar11 = param_2[6];
  uVar12 = param_2[7];
  uVar13 = param_2[8];
  uVar14 = param_2[9];
  uVar15 = param_2[10];
  uVar16 = param_2[0xb];
  if (uVar2 == *param_1) {
    FUN_140b5f9f0(param_1,&UNK_142b14220);
  }
  uVar3 = param_1[1];
  *(undefined8 *)(uVar3 + 0x40 + uVar2 * 0x48) = uStack_30;
  puVar1 = (undefined4 *)(uVar3 + 0x30 + uVar2 * 0x48);
  *puVar1 = uVar15;
  puVar1[1] = uVar16;
  puVar1[2] = uStack_38;
  puVar1[3] = uStack_34;
  puVar1 = (undefined4 *)(uVar3 + 0x20 + uVar2 * 0x48);
  *puVar1 = uVar11;
  puVar1[1] = uVar12;
  puVar1[2] = uVar13;
  puVar1[3] = uVar14;
  puVar1 = (undefined4 *)(uVar3 + 0x10 + uVar2 * 0x48);
  *puVar1 = uVar7;
  puVar1[1] = uVar8;
  puVar1[2] = uVar9;
  puVar1[3] = uVar10;
  puVar1 = (undefined4 *)(uVar3 + uVar2 * 0x48);
  *puVar1 = 0;
  puVar1[1] = 0;
  puVar1[2] = uVar5;
  puVar1[3] = uVar6;
  param_1[2] = uVar2 + 1;
  uVar3 = param_1[8];
  if (uVar3 == param_1[6]) {
    FUN_140562630(param_1 + 6,&UNK_142b14238);
  }
  uVar4 = param_1[7];
  *(undefined4 *)(uVar4 + uVar3 * 0x10) = 0xffffffff;
  *(undefined8 *)(uVar4 + 8 + uVar3 * 0x10) = 0;
  param_1[8] = uVar3 + 1;
  return uVar2 & 0xffffffff;
}

