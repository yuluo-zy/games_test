
void asset_pipe__texture__process_regular_image__closure$3(undefined4 *param_1,uint *param_2,byte *param_3,undefined8 param_4,byte param_5)
{
  byte bVar1;
  byte bVar2;
  code *pcVar3;
  undefined4 uVar4;
  ulonglong uVar5;
  uint uVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_100;
  undefined1 auStack_e0 [32];
  uint uStack_c0;
  uint uStack_bc;
  undefined1 auStack_b8 [32];
  uint uStack_98;
  uint uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  uint uStack_70;
  uint uStack_6c;
  undefined4 *puStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  uint uStack_4c;
  undefined8 uStack_48;
  uStack_48 = 0xfffffffffffffffe;
  uVar10 = *param_2 >> (param_5 & 0x1f);
  uVar10 = uVar10 + (uVar10 == 0);
  uVar9 = param_2[1] >> (param_5 & 0x1f);
  uVar9 = uVar9 + (uVar9 == 0);
  puStack_68 = param_1;
  FUN_140badad0(auStack_e0,param_4,uVar10,uVar9,2);
  bVar1 = *param_3;
  if (bVar1 == 0) {
    FUN_1428d9a20(&UNK_142cdd9c0);
  }
  else {
    bVar2 = param_3[1];
    if (bVar2 == 0) {
      FUN_1428d9a20(&UNK_142cdd9c0);
    }
    else {
      iVar7 = (uint)bVar1 - uVar10 % (uint)bVar1;
      if (uVar10 % (uint)bVar1 == 0) {
        iVar7 = 0;
      }
      uVar10 = iVar7 + uVar10;
      iVar7 = (uint)bVar2 - uVar9 % (uint)bVar2;
      if (uVar9 % (uint)bVar2 == 0) {
        iVar7 = 0;
      }
      uStack_58 = (ulonglong)uStack_c0;
      if (uStack_c0 <= uVar10) {
        uVar9 = iVar7 + uVar9;
        uVar5 = (ulonglong)uStack_bc;
        if (uStack_bc <= uVar9) {
          FUN_140badad0(auStack_b8,auStack_e0,uStack_58,uStack_bc,2);
          FUN_140bad820(&uStack_90,uVar10,uVar9);
          uStack_60 = uVar5;
          if ((uStack_98 <= uStack_70) && (uStack_94 <= uStack_6c)) {
            uStack_4c = uStack_94;
            if (uStack_98 != 0 && uStack_94 != 0) {
              uVar6 = 0;
              do {
                uVar8 = 0;
                do {
                  uVar4 = FUN_140badb80(auStack_b8,uVar8,uVar6);
                  FUN_140bae110(&uStack_90,uVar8,uVar6,uVar4);
                  uVar8 = uVar8 + 1;
                } while (uVar8 != uStack_98);
                uVar6 = uVar6 + 1;
              } while (uVar6 != uStack_4c);
            }
            uStack_4c = (int)uStack_58 + -1;
            uVar6 = 0;
            while (uVar8 = uVar6, uVar8 < (uint)uStack_60) {
              uVar4 = FUN_140badb80(auStack_b8,uStack_4c,uVar8);
              for (uVar5 = uStack_58; uVar6 = uVar8 + 1, (uint)uVar5 < uVar10;
                  uVar5 = (ulonglong)((uint)uVar5 + 1)) {
                FUN_140bae110(&uStack_90,uVar5,uVar8,uVar4);
              }
            }
            uStack_58 = CONCAT44(uStack_58._4_4_,(uint)uStack_60 - 1);
            uVar6 = 0;
            while (uVar8 = uVar6, uVar8 < uVar10) {
              uVar4 = FUN_140badb80(&uStack_90,uVar8,uStack_58 & 0xffffffff);
              for (uVar5 = uStack_60; uVar6 = uVar8 + 1, (uint)uVar5 < uVar9;
                  uVar5 = (ulonglong)((uint)uVar5 + 1)) {
                FUN_140bae110(&uStack_90,uVar8,uVar5,uVar4);
              }
            }
            *(ulonglong *)(puStack_68 + 8) = CONCAT44(uStack_6c,uStack_70);
            puStack_68[4] = uStack_80;
            puStack_68[5] = uStack_7c;
            puStack_68[6] = uStack_78;
            puStack_68[7] = uStack_74;
            *puStack_68 = uStack_90;
            puStack_68[1] = uStack_8c;
            puStack_68[2] = uStack_88;
            puStack_68[3] = uStack_84;
            func_0x0001417b5550(auStack_b8);
            func_0x0001417b5550(auStack_e0);
            return;
          }
          uStack_120 = 0x8000000000000006;
          uStack_118 = 0x8000000000000000;
          uStack_100 = 0;
          FUN_1428d9760(&UNK_142cdda68,0x2b,&uStack_120,&UNK_142cdda98,&UNK_142cdfaa8);
          goto LAB_1417bcc10;
        }
      }
      FUN_1428d9430(&UNK_142cdfa48,0x42,&UNK_142cdfa90);
    }
  }
LAB_1417bcc10:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}
