
undefined4 *
FUN_1416a4b20(undefined4 *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             longlong param_5)

{
  longlong lVar1;
  undefined8 uVar2;
  code *pcVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined1 *puVar6;
  undefined8 *puVar7;
  int iVar8;
  uint uVar9;
  undefined8 *puVar10;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined4 uStack_1c8;
  int iStack_1c4;
  undefined8 uStack_1c0;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  ulonglong uStack_1a8;
  undefined8 *puStack_1a0;
  undefined *puStack_198;
  undefined1 *puStack_190;
  undefined8 *puStack_188;
  undefined8 uStack_178;
  undefined8 *puStack_170;
  undefined8 uStack_168;
  ulonglong uStack_160;
  undefined8 uStack_154;
  int iStack_14c;
  longlong lStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 *puStack_130;
  code *pcStack_128;
  undefined4 uStack_120;
  undefined8 uStack_11c;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 *puStack_f8;
  undefined *puStack_f0;
  undefined8 uStack_e8;
  int iStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 *puStack_c8;
  undefined8 uStack_c0;
  undefined1 *puStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined *puStack_a8;
  undefined1 *puStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  int iStack_70;
  undefined4 uStack_6c;
  undefined8 *puStack_68;
  undefined1 *puStack_60;
  longlong lStack_58;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_4b = 1;
  uStack_1d8 = param_3;
  uStack_1d0 = param_4;
  FUN_140be2a70();
  lStack_148 = param_5;
  lStack_58 = param_5;
  if (*(longlong *)(param_5 + 0xd0) == 0) {
    FUN_140be60a0(&iStack_d8,param_2,param_5 + 0x40,0x20);
    lVar1 = lStack_58;
    puVar7 = puStack_c8;
    iStack_70 = iStack_d8;
    uStack_6c = uStack_d4;
    uStack_110 = uStack_d0;
    uStack_10c = uStack_cc;
    puStack_188 = (undefined8 *)CONCAT44(uStack_ac,uStack_b0);
    puStack_198 = uStack_c0;
    puStack_190 = puStack_b8;
    uStack_cc = 0;
    uStack_d4 = 0;
    uStack_d0 = 0;
    uStack_1c8 = 0x1e;
    iStack_1c4 = iStack_d8;
    uStack_1c0 = 0;
    uStack_1b8 = 0;
    uStack_1b4 = SUB84(puStack_c8,0);
    puStack_1a0 = (undefined8 *)0x0;
    iStack_14c = *(int *)(lStack_58 + 0x148);
    uStack_154 = 0x20;
    if (iStack_14c != 0) {
      puStack_1a0 = &uStack_154;
    }
    uStack_1a8 = (ulonglong)(iStack_14c != 0);
    iVar8 = (**(code **)(param_2 + 0x210))
                      (*(undefined8 *)(param_2 + 0x640),&uStack_1c8,0,&iStack_d8);
    if (iVar8 == 0) {
      if (*(longlong *)(lVar1 + 0x20) == 0) {
        uStack_4a = 1;
        FUN_1428d9518(0,0,&UNK_142c88c98);
      }
      else {
        uVar5 = CONCAT44(uStack_d4,iStack_d8);
        lVar1 = *(longlong *)(lVar1 + 0x18);
        uStack_4a = 1;
        FUN_140d797d0(&uStack_108,*(undefined8 *)(lVar1 + 8),*(undefined8 *)(lVar1 + 0x10));
        puVar6 = uStack_100;
        if (SBORROW8(0,(longlong)uStack_108)) {
          puStack_68 = puStack_f8;
          puStack_60 = uStack_100;
          if (*(longlong *)(lStack_58 + 0x20) == 0) {
            uStack_49 = 1;
            FUN_1428d9518(0,0,&UNK_142c88cc8);
          }
          else {
            puStack_b8 = *(undefined1 **)(*(longlong *)(lStack_58 + 0x18) + 0x20);
            uStack_c0 = (undefined *)(*(longlong *)(*(longlong *)(lStack_58 + 0x18) + 0x28) << 2);
            iStack_d8 = 0x10;
            uStack_d0 = 0;
            uStack_cc = 0;
            puStack_c8 = (undefined8 *)((ulonglong)puStack_c8 & 0xffffffff00000000);
            iVar8 = (**(code **)(param_2 + 0x1c8))
                              (*(undefined8 *)(param_2 + 0x640),&iStack_d8,0,&uStack_108);
            if (iVar8 == 0) {
              uStack_11c = 0;
              uStack_114 = 0;
              uStack_80 = 0;
              uStack_88 = 0;
              puStack_b8 = (undefined1 *)0x0;
              uStack_b0 = 0;
              uVar2 = *(undefined8 *)(param_2 + 0x808);
              iStack_d8 = 0x1d;
              uStack_d0 = 0;
              uStack_cc = 0;
              puStack_c8 = (undefined8 *)((ulonglong)puStack_c8 & 0xffffffff00000000);
              uStack_c0 = (undefined *)CONCAT44(uStack_120,0x12);
              uStack_ac = 0x20;
              puStack_a8 = uStack_108;
              puStack_a0 = puVar6;
              uStack_98 = 0;
              pcVar3 = *(code **)(param_2 + 0x200);
              uVar4 = *(undefined8 *)(param_2 + 0x640);
              uStack_90 = uVar5;
              puVar10 = (undefined8 *)func_0x000140613c10(8,8);
              if (puVar10 == (undefined8 *)0x0) {
                uStack_49 = 1;
                FUN_1428d8fe3(8,8,&UNK_142c885e8);
              }
              else {
                uVar9 = (*pcVar3)(uVar4,uVar2,1,&iStack_d8,0,puVar10);
                if (uVar9 == 0) {
                  uVar2 = *puVar10;
                  func_0x000140613c20(puVar10,8,8);
                  puStack_f8 = puStack_188;
                  uStack_108 = puStack_198;
                  uStack_100 = puStack_190;
                  iStack_d8 = iStack_70;
                  uStack_d4 = uStack_6c;
                  uStack_d0 = uStack_110;
                  uStack_cc = uStack_10c;
                  uStack_49 = 0;
                  puStack_c8 = puVar7;
                  FUN_140be66c0(&uStack_140,&uStack_108,&iStack_d8);
                  if ((*(byte *)(lStack_58 + 0x58) & 1) != 0) {
                    param_1[0x10] = *(undefined4 *)(lStack_58 + 100);
                    *(undefined8 *)(param_1 + 0xe) = *(undefined8 *)(lStack_58 + 0x5c);
                    *param_1 = (undefined4)uStack_140;
                    param_1[1] = uStack_140._4_4_;
                    param_1[2] = (undefined4)uStack_138;
                    param_1[3] = uStack_138._4_4_;
                    *(undefined8 **)(param_1 + 4) = puStack_130;
                    *(longlong *)(param_1 + 6) = lStack_58;
                    *(undefined8 *)(param_1 + 8) = uVar5;
                    *(undefined8 *)(param_1 + 10) = uVar2;
                    param_1[0xc] = 1;
                    *puStack_60 = 0;
                    if (puStack_68 != (undefined8 *)0x0) {
                      func_0x000140613c20(puStack_60,puStack_68,1);
                    }
                    return param_1;
                  }
                  FUN_1428d9330(&UNK_142c88cf8,0x12,&UNK_142c88d10);
                }
                else {
                  uStack_160 = (ulonglong)uVar9;
                  uStack_178 = 1;
                  uStack_168 = 1;
                  uStack_140 = &uStack_1d8;
                  uStack_138 = &UNK_1405338a0;
                  puStack_130 = &uStack_178;
                  pcStack_128 = FUN_1412f35e0;
                  uStack_108 = &UNK_142c88d50;
                  uStack_100 = (undefined1 *)0x2;
                  uStack_e8 = 0;
                  puStack_f8 = &uStack_140;
                  puStack_f0 = (undefined *)0x2;
                  puStack_170 = puVar10;
                  FUN_1428d9390(&uStack_108,&UNK_142c88d70);
                }
              }
            }
            else {
              uStack_108 = (undefined *)CONCAT44(uStack_108._4_4_,iVar8);
              uStack_49 = 1;
              FUN_1428d9760(&UNK_142c88708,0x2b,&uStack_108,&UNK_142c886c8,&UNK_142c88ce0);
            }
          }
        }
        else {
          puStack_c8 = puStack_f8;
          uStack_c0 = puStack_f0;
          iStack_d8 = (int)uStack_108;
          uStack_d4 = uStack_108._4_4_;
          uStack_d0 = (undefined4)uStack_100;
          uStack_cc = uStack_100._4_4_;
          FUN_1428d9760(&UNK_142c88708,0x2b,&iStack_d8,&UNK_142c886e8,&UNK_142c88cb0);
        }
      }
    }
    else {
      uStack_4a = 1;
      iStack_d8 = iVar8;
      FUN_1428d9760(&UNK_142c88708,0x2b,&iStack_d8,&UNK_142c886c8,&UNK_142c88c80);
    }
  }
  else {
    FUN_1428d9430(&UNK_142c88c30,0x37,&UNK_142c88c68);
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

