
undefined8 * FUN_140bd3cb0(undefined8 *param_1,longlong *param_2)

{
  ulonglong uVar1;
  longlong lVar2;
  uint *puVar3;
  code *pcVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  undefined8 uVar9;
  uint *puVar10;
  undefined8 uStack_68;
  undefined8 uStack_60;
  ulonglong uStack_58;
  uint uStack_50;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar1 = param_2[2];
  if (uVar1 < 0x555555555555556) {
    if (uVar1 == 0) {
      uStack_68 = 0;
      uStack_60 = 8;
      uStack_58 = 0;
      *param_1 = 0;
      param_1[1] = 8;
      param_1[2] = 0;
      if (*param_2 != 0) {
        func_0x000140613c20(param_2[1],*param_2 << 4,8);
      }
      lVar2 = param_2[3];
    }
    else {
      uVar9 = 8;
      uStack_60 = func_0x000140613c10(uVar1 * 0x18,8);
      if (uStack_60 == 0) goto code_r0x000140bd3d1d;
      uStack_58 = 0;
      lVar2 = *param_2;
      puVar3 = (uint *)param_2[1];
      uStack_68 = uVar1;
      if (uVar1 != 1) {
        uVar8 = (ulonglong)*puVar3;
        uVar9 = *(undefined8 *)(puVar3 + 2);
        uStack_50 = puVar3[4];
        uVar7 = *(undefined8 *)(puVar3 + 6);
        uVar6 = FUN_140c10ee0();
        uVar9 = func_0x000140c10e50(uVar7,uVar9);
        uVar9 = func_0x000140c10e60(uVar9,(int)param_2[6]);
        uVar5 = uStack_58;
        if (uStack_58 == uStack_68) {
          FUN_140562db0(&uStack_68,&UNK_142b0f568);
        }
        *(undefined8 *)(uStack_60 + uVar5 * 0x18) = uVar6;
        *(ulonglong *)(uStack_60 + 8 + uVar5 * 0x18) = uVar8;
        *(undefined8 *)(uStack_60 + 0x10 + uVar5 * 0x18) = uVar9;
        puVar10 = puVar3 + 4;
        while( true ) {
          uStack_58 = uVar5 + 1;
          uVar8 = (ulonglong)uStack_50;
          if (puVar10 + 4 == puVar3 + uVar1 * 4) break;
          uStack_50 = puVar10[4];
          uVar9 = *(undefined8 *)(puVar10 + 6);
          uVar6 = FUN_140c10ee0(param_2 + 3);
          uVar7 = func_0x000140c10e50(uVar9,uVar7);
          uVar7 = func_0x000140c10e60(uVar7,(int)param_2[6]);
          uVar5 = uStack_58;
          if (uStack_58 == uStack_68) {
            FUN_140562db0(&uStack_68,&UNK_142b0f568);
          }
          *(undefined8 *)(uStack_60 + uVar5 * 0x18) = uVar6;
          *(ulonglong *)(uStack_60 + 8 + uVar5 * 0x18) = uVar8;
          *(undefined8 *)(uStack_60 + 0x10 + uVar5 * 0x18) = uVar7;
          uVar7 = uVar9;
          puVar10 = puVar10 + 4;
        }
      }
      if (lVar2 != 0) {
        func_0x000140613c20(puVar3,lVar2 << 4,8);
      }
      param_1[2] = uStack_58;
      *(undefined4 *)param_1 = (undefined4)uStack_68;
      *(undefined4 *)((longlong)param_1 + 4) = uStack_68._4_4_;
      *(undefined4 *)(param_1 + 1) = (undefined4)uStack_60;
      *(undefined4 *)((longlong)param_1 + 0xc) = uStack_60._4_4_;
      lVar2 = param_2[3];
    }
    if (lVar2 != 0) {
      func_0x000140613c20(param_2[4],lVar2 << 4,8);
    }
    return param_1;
  }
  uVar9 = 0;
code_r0x000140bd3d1d:
  uStack_49 = 1;
  FUN_1428d8fe3(uVar9,uVar1 * 0x18,&UNK_142b0f550);
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

