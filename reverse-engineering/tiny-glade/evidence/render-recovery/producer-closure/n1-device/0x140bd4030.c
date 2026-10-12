
void FUN_140bd4030(longlong param_1,longlong param_2)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  longlong lVar3;
  code *pcVar4;
  undefined8 *puVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined8 *puVar11;
  undefined8 auStack_d8 [2];
  undefined8 uStack_c8;
  undefined2 *puStack_c0;
  undefined8 uStack_b8;
  longlong lStack_b0;
  ulonglong uStack_a8;
  longlong lStack_a0;
  undefined8 uStack_98;
  longlong lStack_90;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  longlong lStack_60;
  ulonglong uStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_68 = 0;
  lStack_60 = 1;
  uStack_58 = 0;
  uStack_49 = 1;
  lVar6 = FUN_140bd3a40(&uStack_68,param_2,&UNK_142b0f580,5);
  puVar2 = *(undefined8 **)(param_1 + 8);
  lVar3 = *(longlong *)(param_1 + 0x10);
  lStack_80 = 1;
  lVar10 = 0;
  puVar5 = puVar2;
  uStack_70 = lVar6;
  do {
    do {
      puVar11 = puVar5;
      uVar1 = uStack_58;
      if (puVar11 == puVar2 + lVar3 * 3) {
        uVar9 = lVar6 + 8;
        if (uStack_58 < uVar9) {
          uStack_49 = 1;
          FUN_1428d9840(uVar9,uStack_58,&UNK_142b0f4c8);
        }
        else {
          *(ulonglong *)(lStack_60 + uStack_70) = uStack_58 - uVar9;
          if (uStack_58 == uStack_68) {
            uStack_49 = 1;
            FUN_140d79570(&uStack_68,&UNK_142b0f4e0);
          }
          *(undefined1 *)(lStack_60 + uVar1) = 0x29;
          uStack_58 = uVar1 + 1;
          if (uStack_68 - uStack_58 < 8) {
            uStack_49 = 1;
            FUN_1428ca940(&uStack_68,uStack_58,8,1,1);
          }
          *(longlong *)(lStack_60 + uStack_58) = lVar10 + param_2;
          uStack_58 = uStack_58 + 8;
          puStack_c0 = (undefined2 *)func_0x000140613c10(3,1);
          if (puStack_c0 != (undefined2 *)0x0) {
            *(undefined1 *)(puStack_c0 + 1) = 0x75;
            *puStack_c0 = 0x7067;
            auStack_d8[0] = 0;
            uStack_c8 = 3;
            uStack_b8 = 3;
            uStack_70 = uStack_68;
            lStack_78 = lStack_60;
            lStack_b0 = lStack_60;
            uStack_a8 = uStack_58;
            lStack_a0 = lStack_80;
            uStack_98 = 1;
            lStack_90 = param_2;
            lStack_88 = lVar10 + param_2;
            FUN_140c16a10(auStack_d8,&lStack_b0);
            if (uStack_70 != 0) {
              func_0x000140613c20(lStack_78,uStack_70,1);
            }
            return;
          }
          uStack_49 = 1;
          FUN_1428d8fe3(1,3,&UNK_142b0f440);
        }
code_r0x000140bd4328:
                    /* WARNING: Does not return */
        pcVar4 = (code *)invalidInstructionException();
        (*pcVar4)();
      }
      uStack_49 = 1;
      lVar7 = func_0x00014067dd80(puVar11[2]);
      puVar5 = puVar11 + 3;
    } while (lVar7 < 1);
    uStack_49 = 1;
    lVar8 = FUN_140bd3a40(&uStack_68,lVar10 + param_2,*puVar11,puVar11[1]);
    uVar9 = uStack_58;
    uVar1 = lVar8 + 8;
    if (uStack_58 < uVar1) {
      uStack_49 = 1;
      FUN_1428d9840(uVar1,uStack_58,&UNK_142b0f4c8);
      goto code_r0x000140bd4328;
    }
    *(ulonglong *)(lStack_60 + lVar8) = uStack_58 - uVar1;
    if (uStack_58 == uStack_68) {
      uStack_49 = 1;
      FUN_140d79570(&uStack_68,&UNK_142b0f4e0);
    }
    *(undefined1 *)(lStack_60 + uVar9) = 0x29;
    uStack_58 = uVar9 + 1;
    if (uStack_68 - uStack_58 < 8) {
      uStack_49 = 1;
      FUN_1428ca940(&uStack_68,uStack_58,8,1,1);
    }
    lVar10 = lVar10 + lVar7;
    *(longlong *)(lStack_60 + uStack_58) = lVar10 + param_2;
    uStack_58 = uStack_58 + 8;
    lStack_80 = lStack_80 + 1;
  } while( true );
}

