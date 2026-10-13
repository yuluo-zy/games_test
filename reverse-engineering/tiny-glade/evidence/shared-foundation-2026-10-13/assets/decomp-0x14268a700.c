
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14268a700(longlong *param_1,ulonglong *param_2)

{
  code *pcVar1;
  longlong lVar2;
  undefined1 unaff_BL;
  uint unaff_ESI;
  undefined *puVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined1 uStack_60;
  undefined *puStack_58;
  undefined4 uStack_50;
  undefined **ppuStack_48;
  undefined *puStack_40;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  
  lVar2 = *param_1;
  uVar4 = lVar2 + 0x7ffffffffffffffb;
  if (1 < uVar4) {
    uVar4 = 2;
  }
  if (uVar4 == 0) {
    uVar4 = *param_2;
    uVar5 = 2;
    if (uVar4 + 0x7ffffffffffffffb < 2) {
      uVar5 = uVar4 + 0x7ffffffffffffffb;
    }
    if (uVar5 == 0) {
      puVar3 = (undefined *)0x8000000000000005;
    }
    else if (uVar5 == 2) {
      puVar3 = (undefined *)0x8000000000000004;
      if (uVar4 != 0x8000000000000004) {
        uVar5 = 2;
        if (uVar4 + 0x7ffffffffffffffe < 2) {
          uVar5 = uVar4 + 0x7ffffffffffffffe;
        }
        if (uVar5 == 0) {
          puVar3 = (undefined *)0x8000000000000002;
        }
        else if (uVar5 == 2) {
          unaff_ESI = (uint)param_2[3];
          if (1 < unaff_ESI) {
            uStack_90 = CONCAT44(uStack_90._4_4_,*(undefined4 *)((longlong)param_2 + 0x24));
            puStack_98 = *(undefined **)((longlong)param_2 + 0x1c);
          }
          unaff_BL = (undefined1)param_2[5];
          uVar5 = 2;
          if ((uVar4 ^ 0x8000000000000000) < 2) {
            uVar5 = uVar4 ^ 0x8000000000000000;
          }
          puVar3 = (undefined *)0x8000000000000000;
          if (uVar5 != 0) {
            if (uVar5 == 2) {
              FUN_140d7bd90(&puStack_88,param_2);
              puVar3 = puStack_88;
            }
            else {
              puVar3 = (undefined *)0x8000000000000001;
            }
          }
          uStack_2c = uStack_78._4_4_;
          uStack_30 = (undefined4)uStack_78;
          uStack_34 = uStack_80._4_4_;
          uStack_38 = (undefined4)uStack_80;
          uStack_50 = (undefined4)uStack_90;
          puStack_58 = puStack_98;
        }
        else {
          puVar3 = (undefined *)0x8000000000000003;
        }
      }
    }
    else {
      puVar3 = (undefined *)0x8000000000000006;
    }
    if (((2 < lVar2 + 0x7ffffffffffffffcU) && (-0x7ffffffffffffffd < lVar2)) && (lVar2 != 0)) {
      func_0x000140613c20(param_1[1],lVar2,1);
    }
    *param_1 = (longlong)puVar3;
    param_1[1] = CONCAT44(uStack_34,uStack_38);
    param_1[2] = CONCAT44(uStack_2c,uStack_30);
    *(uint *)(param_1 + 3) = unaff_ESI;
    *(undefined **)((longlong)param_1 + 0x1c) = puStack_58;
    *(undefined4 *)((longlong)param_1 + 0x24) = uStack_50;
    *(undefined1 *)(param_1 + 5) = unaff_BL;
    return;
  }
  if (uVar4 == 2) {
    uVar5 = *param_2 + 0x7ffffffffffffffb;
    uVar4 = 2;
    if (uVar5 < 2) {
      uVar4 = uVar5;
    }
    if (uVar4 == 2) {
      if (lVar2 != -0x7ffffffffffffffc && *param_2 != 0x8000000000000004) {
        FUN_142689b90(param_1,param_2);
        lVar2 = *param_1;
      }
      if (lVar2 == -0x7ffffffffffffffc) {
        return;
      }
      uStack_60 = 1;
      uStack_70 = 2;
      uStack_6c = (undefined4)_DAT_14292e6b0;
      uStack_68 = (undefined4)((ulonglong)_DAT_14292e6b0 >> 0x20);
      uStack_64 = 0x3f800000;
      puStack_88 = (undefined *)0x8000000000000001;
      FUN_142689b90(param_1,&puStack_88);
      return;
    }
    if (uVar4 != 1) goto LAB_14268aa1b;
    puStack_98 = &UNK_143151161;
    uStack_90 = 0x3e;
    ppuStack_48 = &puStack_98;
    puStack_40 = &UNK_140109e90;
    puStack_88 = &UNK_143150340;
    uStack_80 = 2;
    uStack_68 = 0;
    uStack_64 = 0;
    uStack_78 = &ppuStack_48;
    uStack_70 = 1;
    uStack_6c = 0;
    FUN_1428d9390(&puStack_88,&UNK_143150420);
  }
  FUN_1428d9430(&UNK_1431503c0,0x28,&UNK_143150408);
LAB_14268aa1b:
  puStack_88 = &UNK_1431502f8;
  uStack_80 = 1;
  uStack_78 = (undefined ***)0x8;
  uStack_70 = 0;
  uStack_6c = 0;
  uStack_68 = 0;
  uStack_64 = 0;
  FUN_1428d9390(&puStack_88,&UNK_143150420);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

