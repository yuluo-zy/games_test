
void asset__inheritable__impl$1__inherit_from(ulonglong *param_1,undefined ***param_2)
{
  undefined **ppuVar1;
  code *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined ***unaff_RBX;
  undefined ***unaff_RSI;
  ulonglong unaff_RDI;
  ulonglong uVar6;
  undefined *unaff_R12;
  ulonglong unaff_R13;
  undefined *puVar7;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined *puStack_c8;
  ulonglong uStack_c0;
  undefined ***pppuStack_b8;
  undefined **ppuStack_b0;
  undefined *puStack_a8;
  ulonglong uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined *puStack_80;
  ulonglong uStack_78;
  undefined ***pppuStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined *puStack_50;
  ulonglong uStack_48;
  uStack_48 = 0xfffffffffffffffe;
  uVar6 = *param_1;
  uVar4 = uVar6 + 0x7ffffffffffffffe;
  if (1 < uVar4) {
    uVar4 = 2;
  }
  if (uVar4 == 0) {
    uVar3 = (longlong)*param_2 + 0x7ffffffffffffffe;
    uVar4 = 2;
    if (uVar3 < 2) {
      uVar4 = uVar3;
    }
    if (uVar4 == 0) {
      puVar7 = (undefined *)0x8000000000000002;
    }
    else if (uVar4 == 2) {
      uVar5 = (ulonglong)*param_2 ^ 0x8000000000000000;
      uVar3 = 2;
      if (uVar5 < 2) {
        uVar3 = uVar5;
      }
      if (uVar3 == 2) {
        FUN_140d7bd90(&puStack_98);
        unaff_RBX = param_2;
      }
      else if (uVar3 == 1) {
        puStack_98 = (undefined *)0x8000000000000001;
      }
      else {
        puStack_98 = (undefined *)0x8000000000000000;
      }
      uVar3 = (longlong)param_2[6] + 0x7fffffffffffffff;
      if (1 < uVar3) {
        uVar3 = 2;
      }
      puStack_50 = (undefined *)0x8000000000000001;
      if (uVar3 != 0) {
        if (uVar3 == 2) {
          puStack_50 = (undefined *)0x8000000000000000;
          if (!SBORROW8(0,(longlong)param_2[6])) {
            FUN_140d7bd90(&puStack_80,param_2 + 6);
            unaff_RBX = pppuStack_70;
            uVar4 = uStack_78;
            puStack_50 = puStack_80;
          }
        }
        else {
          puStack_50 = (undefined *)0x8000000000000002;
        }
      }
      uVar3 = 2;
      if (((ulonglong)param_2[3] ^ 0x8000000000000000) < 2) {
        uVar3 = (ulonglong)param_2[3] ^ 0x8000000000000000;
      }
      uStack_a0 = uVar4;
      if (uVar3 == 2) {
        FUN_140d7bd90(&puStack_80,param_2 + 3);
      }
      else {
        puStack_80 = (undefined *)0x8000000000000000;
        if (uVar3 == 1) {
          puStack_80 = (undefined *)0x8000000000000001;
        }
      }
      uStack_d8 = (undefined4)uStack_90;
      uStack_d4 = uStack_90._4_4_;
      uStack_d0 = uStack_88;
      uStack_cc = uStack_84;
      puStack_c8 = puStack_80;
      uStack_c0 = uStack_78;
      pppuStack_b8 = pppuStack_70;
      unaff_R12 = puStack_50;
      unaff_R13 = uStack_a0;
      puVar7 = puStack_98;
    }
    else {
      puVar7 = (undefined *)0x8000000000000003;
    }
    if (uVar6 >> 1 != 0x4000000000000001) {
      if ((-0x7fffffffffffffff < (longlong)uVar6) && (uVar6 != 0)) {
        func_0x000140613c20(param_1[1],uVar6,1);
      }
      uVar4 = param_1[6];
      if ((-0x7ffffffffffffffe < (longlong)uVar4) && (uVar4 != 0)) {
        func_0x000140613c20(param_1[7],uVar4,1);
      }
      uVar4 = param_1[3];
      if ((-0x7fffffffffffffff < (longlong)uVar4) && (uVar4 != 0)) {
        func_0x000140613c20(param_1[4],uVar4,1);
      }
    }
    *param_1 = (ulonglong)puVar7;
    param_1[1] = CONCAT44(uStack_d4,uStack_d8);
    param_1[2] = CONCAT44(uStack_cc,uStack_d0);
    param_1[3] = (ulonglong)puStack_c8;
    param_1[4] = uStack_c0;
    param_1[5] = (ulonglong)pppuStack_b8;
    param_1[6] = (ulonglong)unaff_R12;
    param_1[7] = unaff_R13;
    param_1[8] = (ulonglong)unaff_RBX;
    return;
  }
  if (uVar4 == 2) {
    uVar4 = 2;
    if ((longlong)*param_2 + 0x7ffffffffffffffeU < 2) {
      uVar4 = (longlong)*param_2 + 0x7ffffffffffffffeU;
    }
    if (uVar4 == 2) {
      if (SBORROW8(0,*param_1)) {
        uVar4 = 2;
        if (((ulonglong)*param_2 ^ 0x8000000000000000) < 2) {
          uVar4 = (ulonglong)*param_2 ^ 0x8000000000000000;
        }
        puVar7 = (undefined *)0x8000000000000000;
        if (uVar4 != 0) {
          if (uVar4 == 1) {
            puVar7 = (undefined *)0x8000000000000001;
          }
          else {
            puStack_80 = (undefined *)0x142879d15;
            FUN_140d7bd90(&puStack_50);
            puVar7 = puStack_50;
          }
        }
        puStack_50 = puVar7;
        param_1[2] = (ulonglong)unaff_RBX;
        *param_1 = (ulonglong)puStack_50;
        param_1[1] = uStack_48;
        uVar4 = param_1[6];
      }
      else {
        uVar4 = param_1[6];
      }
      if (uVar4 == 0x8000000000000001) {
        ppuVar1 = param_2[6];
        uVar6 = (longlong)ppuVar1 + 0x7fffffffffffffff;
        uVar4 = 2;
        if (uVar6 < 2) {
          uVar4 = uVar6;
        }
        puVar7 = (undefined *)0x8000000000000001;
        if (uVar4 != 0) {
          if (uVar4 == 2) {
            puVar7 = (undefined *)0x8000000000000000;
            unaff_RDI = uStack_48;
            unaff_RSI = unaff_RBX;
            if (!SBORROW8(0,(longlong)ppuVar1)) {
              puStack_80 = (undefined *)0x142879ceb;
              FUN_140d7bd90(&puStack_50,param_2 + 6);
              puVar7 = puStack_50;
              unaff_RDI = uStack_48;
            }
          }
          else {
            puVar7 = (undefined *)0x8000000000000002;
          }
        }
        param_1[6] = (ulonglong)puVar7;
        *(int *)(param_1 + 7) = (int)unaff_RDI;
        *(int *)((longlong)param_1 + 0x3c) = (int)(unaff_RDI >> 0x20);
        *(int *)(param_1 + 8) = (int)unaff_RSI;
        *(int *)((longlong)param_1 + 0x44) = (int)((ulonglong)unaff_RSI >> 0x20);
        uVar4 = param_1[3];
      }
      else {
        uVar4 = param_1[3];
      }
      if (SBORROW8(0,uVar4)) {
        uVar4 = 2;
        if (((ulonglong)param_2[3] ^ 0x8000000000000000) < 2) {
          uVar4 = (ulonglong)param_2[3] ^ 0x8000000000000000;
        }
        if (uVar4 == 2) {
          puStack_80 = (undefined *)0x142879da8;
          FUN_140d7bd90(&puStack_50);
        }
        else if (uVar4 == 1) {
          puStack_50 = (undefined *)0x8000000000000001;
        }
        else {
          puStack_50 = (undefined *)0x8000000000000000;
        }
        param_1[5] = (ulonglong)unaff_RBX;
        param_1[3] = (ulonglong)puStack_50;
        param_1[4] = uStack_48;
      }
      return;
    }
    if (uVar4 != 1) goto LAB_14268a1eb;
    puStack_98 = &UNK_143151138;
    uStack_90 = 0x29;
    ppuStack_b0 = &puStack_98;
    puStack_a8 = &UNK_140109e90;
    puStack_80 = &UNK_143150340;
    uStack_78 = 2;
    uStack_60 = 0;
    pppuStack_70 = &ppuStack_b0;
    uStack_68 = 1;
    FUN_1428d9390(&puStack_80,&UNK_143150420);
  }
  FUN_1428d9430(&UNK_1431503c0,0x28,&UNK_143150408);
LAB_14268a1eb:
  puStack_80 = &UNK_1431502f8;
  uStack_78 = 1;
  pppuStack_70 = (undefined ***)0x8;
  uStack_68 = 0;
  uStack_60 = 0;
  FUN_1428d9390(&puStack_80,&UNK_143150420);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}
