
void FUN_140db8370(longlong *param_1,undefined8 *param_2)

{
  undefined4 *puVar1;
  longlong lVar2;
  code *pcVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined8 *puVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  longlong lStack_70;
  longlong lStack_68;
  longlong *plStack_60;
  longlong lStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_68 = *param_1;
  plStack_50 = (longlong *)*param_2;
  lStack_58 = param_2[1];
  if (lStack_68 != param_1[1]) {
    plStack_60 = (longlong *)param_1[2];
    lVar2 = param_2[2];
    lStack_70 = ((ulonglong)(param_1[1] - lStack_68) >> 3) * -0x3333333333333333;
    lVar11 = 0;
    do {
      lVar9 = *(longlong *)(lStack_68 + 0x18 + lVar11 * 0x28) - *plStack_60;
      if (lVar9 < 1) {
        lVar9 = 0;
      }
      uVar7 = 0x19db1de;
      uVar4 = FUN_140d63d00(0xd53e8000,0x19db1de,lVar9,0);
      lVar9 = lStack_68 + lVar11 * 0x28;
      lVar10 = *(longlong *)(lVar9 + 0x20) - *plStack_60;
      if (lVar10 < 1) {
        lVar10 = 0;
      }
      uVar8 = 0x19db1de;
      uVar5 = FUN_140d63d00(0xd53e8000,0x19db1de,lVar10,0);
      FUN_140d7bd90(&uStack_c0,lVar9);
      uStack_a8 = 1;
      uStack_a0 = 1;
      uStack_98 = 0x8000000000000000;
      uStack_90 = 0x8000000000000000;
      puVar6 = (undefined8 *)func_0x000140613c10(0x30,8);
      if (puVar6 == (undefined8 *)0x0) {
        FUN_1428d9000(8,0x30);
                    /* WARNING: Does not return */
        pcVar3 = (code *)invalidInstructionException();
        (*pcVar3)();
      }
      *(undefined4 *)(puVar6 + 4) = uStack_88;
      *(undefined4 *)((longlong)puVar6 + 0x24) = uStack_84;
      *(undefined4 *)(puVar6 + 5) = uStack_80;
      *(undefined4 *)((longlong)puVar6 + 0x2c) = uStack_7c;
      *(undefined4 *)(puVar6 + 2) = (undefined4)uStack_98;
      *(undefined4 *)((longlong)puVar6 + 0x14) = uStack_98._4_4_;
      *(undefined4 *)(puVar6 + 3) = (undefined4)uStack_90;
      *(undefined4 *)((longlong)puVar6 + 0x1c) = uStack_90._4_4_;
      *puVar6 = uStack_a8;
      puVar6[1] = uStack_a0;
      lVar9 = lStack_58 * 0x30;
      *(undefined8 *)(lVar2 + 0x10 + lVar9) = uStack_b0;
      puVar1 = (undefined4 *)(lVar2 + lVar9);
      *puVar1 = uStack_c0;
      puVar1[1] = uStack_bc;
      puVar1[2] = uStack_b8;
      puVar1[3] = uStack_b4;
      *(undefined4 *)(lVar2 + 0x18 + lVar9) = uVar4;
      *(undefined4 *)(lVar2 + 0x1c + lVar9) = uVar7;
      *(undefined4 *)(lVar2 + 0x20 + lVar9) = uVar5;
      *(undefined4 *)(lVar2 + 0x24 + lVar9) = uVar8;
      *(undefined8 **)(lVar2 + 0x28 + lVar9) = puVar6 + 2;
      lStack_58 = lStack_58 + 1;
      lVar11 = lVar11 + 1;
    } while (lVar11 != lStack_70);
  }
  *plStack_50 = lStack_58;
  return;
}

