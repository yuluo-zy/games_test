
undefined8 *
FUN_140bd4fc0(undefined8 *param_1,undefined8 *param_2,longlong *param_3,undefined1 param_4)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined3 uVar9;
  undefined1 uStack_91;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined8 uStack_40;
  
  lVar3 = param_2[1];
  LOCK();
  *(int *)(lVar3 + 8) = *(int *)(lVar3 + 8) + 1;
  UNLOCK();
  uVar2 = *(undefined4 *)(param_2 + 2);
  uVar9 = *(undefined3 *)((longlong)param_2 + 0x14);
  uVar4 = *param_2;
  uVar7 = param_2[3];
  uVar8 = param_2[4];
  lVar5 = param_3[2];
  uStack_78._0_2_ = CONCAT11(param_4,1);
  if (lVar5 == *param_3) {
    FUN_140b5f9f0(param_3,&UNK_142b14220);
  }
  lVar6 = param_3[1];
  uStack_70 = (undefined4)uVar4;
  uStack_6c = (undefined4)((ulonglong)uVar4 >> 0x20);
  uStack_68 = (undefined4)lVar3;
  uStack_64 = (undefined4)((ulonglong)lVar3 >> 0x20);
  *(undefined8 *)(lVar6 + 0x40 + lVar5 * 0x48) = uStack_40;
  puVar1 = (undefined4 *)(lVar6 + 0x30 + lVar5 * 0x48);
  *puVar1 = (int)uVar8;
  puVar1[1] = (int)((ulonglong)uVar8 >> 0x20);
  puVar1[2] = uStack_48;
  puVar1[3] = uStack_44;
  puVar1 = (undefined4 *)(lVar6 + 0x20 + lVar5 * 0x48);
  *puVar1 = uVar2;
  puVar1[1] = CONCAT13(uStack_91,uVar9);
  puVar1[2] = (int)uVar7;
  puVar1[3] = (int)((ulonglong)uVar7 >> 0x20);
  puVar1 = (undefined4 *)(lVar6 + 0x10 + lVar5 * 0x48);
  *puVar1 = uStack_70;
  puVar1[1] = uStack_6c;
  puVar1[2] = uStack_68;
  puVar1[3] = uStack_64;
  puVar1 = (undefined4 *)(lVar6 + lVar5 * 0x48);
  *puVar1 = 1;
  puVar1[1] = 0;
  puVar1[2] = uStack_78;
  puVar1[3] = uStack_74;
  param_3[2] = lVar5 + 1;
  lVar3 = param_3[8];
  if (lVar3 == param_3[6]) {
    FUN_140562630(param_3 + 6,&UNK_142b14238);
  }
  lVar6 = param_3[7];
  *(undefined4 *)(lVar6 + lVar3 * 0x10) = 0xffffffff;
  *(undefined8 *)(lVar6 + 8 + lVar3 * 0x10) = 0;
  param_3[8] = lVar3 + 1;
  uVar4 = param_2[4];
  *param_1 = param_2[3];
  param_1[1] = uVar4;
  *(int *)(param_1 + 2) = (int)lVar5;
  return param_1;
}

