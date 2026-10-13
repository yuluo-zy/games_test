
void asset__prefab__scan__load_prefab_config(longlong *param_1,undefined8 param_2,undefined8 param_3)
{
  undefined8 uVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  longlong lStack_e8;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  longlong lStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined8 *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined *puStack_38;
  undefined1 uStack_29;
  undefined8 uStack_28;
  uStack_28 = 0xfffffffffffffffe;
  uStack_108 = param_2;
  uStack_100 = param_3;
  FUN_140d59140(&puStack_98);
  puVar2 = puStack_88;
  uVar1 = uStack_90;
  if (SBORROW8(0,(longlong)puStack_98)) {
    lVar3 = anyhow__error__impl$1__from(uStack_90);
  }
  else {
    puStack_38 = puStack_98;
    puStack_88 = (undefined8 *)0x0;
    puStack_98 = (undefined *)0x1;
    uStack_90 = 0x80;
    uStack_40 = uVar1;
    ron__options__Options__from_str_seed(&uStack_f8,&puStack_98,uVar1,puVar2);
    if ((char)uStack_f8 == '2') {
      if (puStack_38 != (undefined *)0x0) {
        func_0x000140613c20(uStack_40,puStack_38,1);
      }
      lVar3 = lStack_e8;
      if (uStack_f0 != (undefined *)0x0) {
        *param_1 = (longlong)uStack_f0;
        param_1[1] = lStack_e8;
        param_1[2] = uStack_e0;
        param_1[3] = CONCAT44(uStack_d4,uStack_d8);
        param_1[4] = CONCAT44(uStack_cc,uStack_d0);
        param_1[5] = CONCAT44(uStack_c4,uStack_c8);
        return;
      }
    }
    else {
      uStack_128 = (undefined4)uStack_e0;
      uStack_124 = uStack_e0._4_4_;
      uStack_120 = uStack_d8;
      uStack_11c = uStack_d4;
      uStack_118 = uStack_d0;
      uStack_114 = uStack_cc;
      uStack_110 = uStack_c8;
      uStack_10c = uStack_c4;
      uStack_60 = uStack_c0;
      uStack_58 = uStack_b8;
      uStack_50 = uStack_b0;
      uStack_48 = uStack_a8;
      uStack_70 = uStack_d0;
      uStack_6c = uStack_cc;
      uStack_68 = uStack_c8;
      uStack_64 = uStack_c4;
      lVar3 = anyhow__error__impl$1__from(&puStack_98);
      if (puStack_38 != (undefined *)0x0) {
        func_0x000140613c20(uStack_40,puStack_38,1);
      }
    }
  }
  uStack_f8 = &uStack_108;
  uStack_f0 = &UNK_14002e8d0;
  puStack_98 = &UNK_1430718c0;
  uStack_90 = 1;
  puStack_88 = &uStack_f8;
  uStack_80 = 1;
  uStack_78 = 0;
  uStack_29 = 1;
  lStack_a0 = lVar3;
  FUN_140d79f90(&uStack_128,&puStack_98);
  lStack_e8 = CONCAT44(uStack_114,uStack_118);
  uStack_f8 = (undefined8 *)CONCAT44(uStack_124,uStack_128);
  uStack_f0 = (undefined *)CONCAT44(uStack_11c,uStack_120);
  puStack_98 = (undefined *)0x3;
  uStack_29 = 0;
  uStack_e0 = lVar3;
  lVar3 = anyhow__Error__construct(&uStack_f8,&puStack_98);
  param_1[1] = lVar3;
  *param_1 = 0;
  return;
}
