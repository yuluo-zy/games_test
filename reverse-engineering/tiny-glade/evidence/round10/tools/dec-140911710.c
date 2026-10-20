
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint * FUN_140911710(uint *param_1,longlong param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined1 uStack_e9;
  undefined8 uStack_e8;
  undefined *puStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  
  uVar2 = (uint)*(undefined8 *)(param_2 + 0x138);
  uVar3 = (uint)((ulonglong)*(undefined8 *)(param_2 + 0x138) >> 0x20);
  uVar4 = *(uint *)(param_2 + 0x140);
  if ((0x7f7fffff < (uVar3 & 0x7fffffff) || 0x7f7fffff < (uVar2 & 0x7fffffff)) ||
      0x7f7fffff < (uVar4 & 0x7fffffff)) {
    uVar2 = (uint)_UNK_142aab4c0;
    uVar3 = (uint)((ulonglong)_UNK_142aab4c0 >> 0x20);
    uVar4 = 0;
    if (*plRam000000014362c3a8 != 0) {
      puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142aab7a8);
      uStack_c8 = *puVar1;
      uStack_c4 = puVar1[1];
      uStack_c0 = puVar1[2];
      uStack_bc = puVar1[3];
      uStack_9c = puVar1[4];
      uStack_b8 = 1;
      puStack_b0 = &UNK_142aab7c0;
      uStack_a8 = 0x21;
      puStack_98 = &UNK_142aab798;
      uStack_90 = 1;
      uStack_88 = 8;
      uStack_e8 = 0;
      uStack_80 = 0;
      uStack_78 = 0;
      puStack_e0 = &UNK_142aab7c0;
      uStack_d8 = 0x21;
      uStack_d0 = 0;
      uStack_a0 = 1;
      func_0x000140d4a330(&uStack_e9,&uStack_e8);
    }
  }
  uVar5 = *(uint *)(param_2 + 0x154);
  if ((0x7f7fffff < (uVar5 & 0x7fffffff)) && (uVar5 = 0, *plRam000000014362c3a8 != 0)) {
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142aab828);
    uStack_c8 = *puVar1;
    uStack_c4 = puVar1[1];
    uStack_c0 = puVar1[2];
    uStack_bc = puVar1[3];
    uStack_9c = puVar1[4];
    uStack_b8 = 1;
    puStack_b0 = &UNK_142aab7c0;
    uStack_a8 = 0x21;
    puStack_98 = &UNK_142aab818;
    uStack_90 = 1;
    uStack_88 = 8;
    uStack_e8 = 0;
    uStack_80 = 0;
    uStack_78 = 0;
    puStack_e0 = &UNK_142aab7c0;
    uStack_d8 = 0x21;
    uStack_d0 = 0;
    uStack_a0 = 1;
    func_0x000140d4a330(&uStack_e9,&uStack_e8);
  }
  uVar6 = *(uint *)(param_2 + 0x150);
  if ((0x7f7fffff < (uVar6 & 0x7fffffff)) && (uVar6 = 0, *plRam000000014362c3a8 != 0)) {
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142aab888);
    uStack_c8 = *puVar1;
    uStack_c4 = puVar1[1];
    uStack_c0 = puVar1[2];
    uStack_bc = puVar1[3];
    uStack_9c = puVar1[4];
    uStack_b8 = 1;
    puStack_b0 = &UNK_142aab7c0;
    uStack_a8 = 0x21;
    puStack_98 = &UNK_142aab878;
    uStack_90 = 1;
    uStack_88 = 8;
    uStack_e8 = 0;
    uStack_80 = 0;
    uStack_78 = 0;
    puStack_e0 = &UNK_142aab7c0;
    uStack_d8 = 0x21;
    uStack_d0 = 0;
    uStack_a0 = 1;
    func_0x000140d4a330(&uStack_e9,&uStack_e8);
  }
  uVar7 = *(uint *)(param_2 + 400);
  if ((0x7f7fffff < (uVar7 & 0x7fffffff)) && (uVar7 = 0, *plRam000000014362c3a8 != 0)) {
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142aab8f8);
    uStack_c8 = *puVar1;
    uStack_c4 = puVar1[1];
    uStack_c0 = puVar1[2];
    uStack_bc = puVar1[3];
    uStack_9c = puVar1[4];
    uStack_b8 = 1;
    puStack_b0 = &UNK_142aab7c0;
    uStack_a8 = 0x21;
    puStack_98 = &UNK_142aab8e8;
    uStack_90 = 1;
    uStack_88 = 8;
    uStack_e8 = 0;
    uStack_80 = 0;
    uStack_78 = 0;
    puStack_e0 = &UNK_142aab7c0;
    uStack_d8 = 0x21;
    uStack_d0 = 0;
    uStack_a0 = 1;
    func_0x000140d4a330(&uStack_e9,&uStack_e8);
  }
  *param_1 = uVar5;
  param_1[1] = uVar6;
  *(ulonglong *)(param_1 + 2) = CONCAT44(uVar3,uVar2);
  param_1[4] = uVar4;
  param_1[5] = 0;
  param_1[6] = uVar7;
  param_1[7] = 0;
  param_1[8] = 0;
  return param_1;
}

