
void FUN_140fac810(longlong param_1,undefined8 param_2,longlong *param_3)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 *puVar5;
  code *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_a8;
  ulonglong uStack_a0;
  longlong lStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar8 = *(undefined8 *)(param_1 + 0x418);
  lVar1 = *(longlong *)(param_1 + 0x420);
  *(longlong *)(param_1 + 0x440) = lVar1;
  uVar2 = *(undefined8 *)(param_1 + 0x430);
  lVar3 = *(longlong *)(param_1 + 0x438);
  lVar4 = *param_3;
  puVar5 = (undefined8 *)param_3[1];
  uVar7 = func_0x000140987ca0(*puVar5);
  if (lVar1 == 0) {
LAB_140fac8da:
    lVar1 = param_3[2];
    uVar8 = func_0x000140987ca0(*puVar5);
    if (lVar3 == 0) {
LAB_140fac946:
      *(undefined8 *)(param_1 + 0x420) = 0;
      *(undefined8 *)(param_1 + 0x438) = 0;
      FUN_140b8f680(param_1,param_2);
      *(int *)(param_1 + 0x4c8) = *(int *)(param_1 + 0x4c8) + 1;
      return;
    }
    uVar9 = lVar3 << 2;
    if (*(ulonglong *)(lVar1 + 0x18) < uVar9) {
      lStack_98 = 0xb;
    }
    else {
      uStack_a8 = 0;
      uStack_a0 = uVar9;
      FUN_140beefa0(&lStack_98,uVar8,lVar1,uVar2,uVar9,&uStack_a8,1,0);
      if (lStack_98 == 0xb) goto LAB_140fac946;
      uStack_b8 = uStack_50;
      uStack_f8 = CONCAT44(uStack_8c,uStack_90);
      uStack_f0 = CONCAT44(uStack_84,uStack_88);
      uStack_e8 = CONCAT44(uStack_7c,uStack_80);
      uStack_e0 = CONCAT44(uStack_74,uStack_78);
      uStack_d8 = CONCAT44(uStack_6c,uStack_70);
      uStack_d0 = CONCAT44(uStack_64,uStack_68);
      uStack_c8 = CONCAT44(uStack_5c,uStack_60);
      uStack_c0 = CONCAT44(uStack_54,uStack_58);
    }
    uStack_90 = (undefined4)uStack_f8;
    uStack_8c = uStack_f8._4_4_;
    uStack_88 = (undefined4)uStack_f0;
    uStack_84 = uStack_f0._4_4_;
    uStack_80 = (undefined4)uStack_e8;
    uStack_7c = uStack_e8._4_4_;
    uStack_78 = (undefined4)uStack_e0;
    uStack_74 = uStack_e0._4_4_;
    uStack_70 = (undefined4)uStack_d8;
    uStack_6c = uStack_d8._4_4_;
    uStack_68 = (undefined4)uStack_d0;
    uStack_64 = uStack_d0._4_4_;
    uStack_60 = (undefined4)uStack_c8;
    uStack_5c = uStack_c8._4_4_;
    uStack_58 = (undefined4)uStack_c0;
    uStack_54 = uStack_c0._4_4_;
    uStack_50 = uStack_b8;
    FUN_1428d9760(&UNK_142bb0db8,0x2b,&lStack_98,&UNK_142bb0d98,&UNK_142bb11c8);
  }
  else {
    uVar9 = lVar1 * 0x30;
    if (*(ulonglong *)(lVar4 + 0x18) < uVar9) {
      lStack_98 = 0xb;
    }
    else {
      uStack_a8 = 0;
      uStack_a0 = uVar9;
      FUN_140beefa0(&lStack_98,uVar7,lVar4,uVar8,uVar9,&uStack_a8,1,0);
      if (lStack_98 == 0xb) goto LAB_140fac8da;
      uStack_b8 = uStack_50;
      uStack_f8 = CONCAT44(uStack_8c,uStack_90);
      uStack_f0 = CONCAT44(uStack_84,uStack_88);
      uStack_e8 = CONCAT44(uStack_7c,uStack_80);
      uStack_e0 = CONCAT44(uStack_74,uStack_78);
      uStack_d8 = CONCAT44(uStack_6c,uStack_70);
      uStack_d0 = CONCAT44(uStack_64,uStack_68);
      uStack_c8 = CONCAT44(uStack_5c,uStack_60);
      uStack_c0 = CONCAT44(uStack_54,uStack_58);
    }
    uStack_90 = (undefined4)uStack_f8;
    uStack_8c = uStack_f8._4_4_;
    uStack_88 = (undefined4)uStack_f0;
    uStack_84 = uStack_f0._4_4_;
    uStack_80 = (undefined4)uStack_e8;
    uStack_7c = uStack_e8._4_4_;
    uStack_78 = (undefined4)uStack_e0;
    uStack_74 = uStack_e0._4_4_;
    uStack_70 = (undefined4)uStack_d8;
    uStack_6c = uStack_d8._4_4_;
    uStack_68 = (undefined4)uStack_d0;
    uStack_64 = uStack_d0._4_4_;
    uStack_60 = (undefined4)uStack_c8;
    uStack_5c = uStack_c8._4_4_;
    uStack_58 = (undefined4)uStack_c0;
    uStack_54 = uStack_c0._4_4_;
    uStack_50 = uStack_b8;
    FUN_1428d9760(&UNK_142bb0db8,0x2b,&lStack_98,&UNK_142bb0d98,&UNK_142bb11b0);
  }
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

