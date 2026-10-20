
undefined1 * FUN_141560fc0(undefined1 *param_1,undefined8 param_2)

{
  longlong lVar1;
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
  undefined8 uStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 uStack_38;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 uStack_28;
  undefined1 uStack_19;
  undefined8 uStack_18;
  
  uStack_18 = 0xfffffffffffffffe;
  uStack_50 = -0x8000000000000000;
  uStack_38 = 0;
  uStack_28 = 0;
  uStack_19 = 1;
  lVar1 = FUN_1415997d0(&uStack_50,&UNK_142c6e7ad,5,param_2);
  if (lVar1 == 0) {
    uStack_68 = uStack_30;
    uStack_64 = uStack_2c;
    uStack_60 = (undefined4)uStack_28;
    uStack_5c = uStack_28._4_4_;
    uStack_78 = uStack_40;
    uStack_74 = uStack_3c;
    uStack_70 = (undefined4)uStack_38;
    uStack_6c = uStack_38._4_4_;
    uStack_88 = (undefined4)uStack_50;
    uStack_84 = uStack_50._4_4_;
    uStack_80 = uStack_48;
    uStack_7c = uStack_44;
    uStack_19 = 0;
    FUN_140b864c0(param_1,&uStack_88);
  }
  else {
    *(longlong *)(param_1 + 8) = lVar1;
    *param_1 = 6;
    FUN_140898f40(&uStack_38);
    if ((!SBORROW8(0,uStack_50)) && (uStack_50 != 0)) {
      func_0x000140613c20(CONCAT44(uStack_44,uStack_48),uStack_50,1);
    }
  }
  return param_1;
}

