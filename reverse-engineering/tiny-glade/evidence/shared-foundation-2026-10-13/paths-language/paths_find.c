
undefined8 * FUN_140b9dc80(undefined8 *param_1)

{
  undefined8 uVar1;
  longlong lVar2;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  longlong lStack_30;
  longlong lStack_28;
  undefined1 uStack_19;
  undefined8 uStack_18;
  
  uStack_18 = 0xfffffffffffffffe;
  lVar2 = 0x6c;
  FUN_141849e90(&uStack_c8,0x6c);
  if (SBORROW8(0,CONCAT44(uStack_c4,uStack_c8))) {
    FUN_140d57930(&lStack_a8,&UNK_142afd998,0xb);
    if (lStack_a8 == -0x7fffffffffffffff) {
      lStack_28 = CONCAT44(uStack_94,uStack_98);
      if (!SBORROW8(0,lStack_a0)) {
        lStack_30 = lStack_a0;
        FUN_140d62810(&uStack_88,lStack_28,CONCAT44(uStack_8c,uStack_90),&UNK_142afd9cb,0xb);
        lVar2 = lStack_30;
        if (lStack_30 != 0) {
          func_0x000140613c20(lStack_28,lStack_30,1);
        }
        goto LAB_140b9dd4e;
      }
    }
    else {
      lStack_28 = lStack_a8;
      lStack_30 = lStack_a0;
      uStack_19 = 0;
      FUN_140d56700(&uStack_60);
      uStack_d8 = uStack_40;
      uStack_d4 = uStack_3c;
      uStack_d0 = uStack_38;
      uStack_cc = uStack_34;
      uStack_e8 = (undefined4)uStack_50;
      uStack_e4 = uStack_50._4_4_;
      uStack_e0 = (undefined4)uStack_48;
      uStack_dc = uStack_48._4_4_;
      uStack_f8 = uStack_60;
      uStack_f0 = uStack_58;
      uStack_50 = lStack_28;
      uStack_48 = lStack_30;
      uStack_40 = uStack_98;
      uStack_3c = uStack_94;
      uStack_38 = uStack_90;
      uStack_34 = uStack_8c;
      uStack_60 = &UNK_142afd9a3;
      uStack_58 = 0x28;
      uStack_19 = 1;
      lStack_28 = FUN_142903ec0(&uStack_60,&uStack_f8);
    }
    param_1[1] = lStack_28;
    *param_1 = 0x8000000000000000;
  }
  else {
    uStack_78 = uStack_b8;
    uStack_70 = uStack_b0;
    uStack_88 = uStack_c8;
    uStack_84 = uStack_c4;
    uStack_80 = uStack_c0;
    uStack_7c = uStack_bc;
LAB_140b9dd4e:
    uVar1 = FUN_140d76ed0(&uStack_88);
    FUN_140d62810(&uStack_60,uVar1,lVar2,&UNK_142afd9d6,10);
    *(undefined4 *)(param_1 + 2) = (undefined4)uStack_50;
    *(undefined4 *)((longlong)param_1 + 0x14) = uStack_50._4_4_;
    *(undefined4 *)(param_1 + 3) = (undefined4)uStack_48;
    *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_48._4_4_;
    *(undefined4 *)param_1 = (undefined4)uStack_60;
    *(undefined4 *)((longlong)param_1 + 4) = uStack_60._4_4_;
    *(undefined4 *)(param_1 + 1) = (undefined4)uStack_58;
    *(undefined4 *)((longlong)param_1 + 0xc) = uStack_58._4_4_;
    if (CONCAT44(uStack_84,uStack_88) != 0) {
      func_0x000140613c20(CONCAT44(uStack_7c,uStack_80),CONCAT44(uStack_84,uStack_88),1);
    }
  }
  return param_1;
}

