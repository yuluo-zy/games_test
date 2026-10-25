
undefined8 *
FUN_1400fb5b0(undefined8 *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             undefined4 *param_5,undefined8 param_6)

{
  undefined1 auStack_6d0 [536];
  longlong alStack_4b8 [67];
  undefined1 auStack_2a0 [144];
  undefined1 auStack_210 [216];
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
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
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  uStack_39 = 1;
  FUN_140166510(alStack_4b8,param_4,*(undefined8 *)(param_2 + 0x178),
                *(undefined8 *)(param_2 + 0x180),param_6);
  if (SBORROW8(0,alStack_4b8[0])) {
    *param_1 = 0;
    FUN_140be2610(param_5);
    FUN_140be24b0(param_5);
  }
  else {
    func_0x000142923c00(auStack_6d0,alStack_4b8,0x218);
    uStack_138 = *param_5;
    uStack_134 = param_5[1];
    uStack_130 = param_5[2];
    uStack_12c = param_5[3];
    uStack_128 = param_5[4];
    uStack_124 = param_5[5];
    uStack_120 = param_5[6];
    uStack_11c = param_5[7];
    uStack_118 = param_5[8];
    uStack_114 = param_5[9];
    uStack_110 = param_5[10];
    uStack_10c = param_5[0xb];
    uStack_39 = 0;
    FUN_1400fa190(auStack_210,param_2,auStack_6d0,&uStack_138,param_3);
    uStack_39 = 0;
    FUN_140be9d00(&uStack_138,auStack_210,*(undefined8 *)(param_2 + 0x180));
    func_0x000142923c00(auStack_2a0,&uStack_138,0x90);
    uStack_58 = uStack_88;
    uStack_54 = uStack_84;
    uStack_50 = uStack_80;
    uStack_4c = uStack_7c;
    uStack_68 = uStack_98;
    uStack_64 = uStack_94;
    uStack_60 = uStack_90;
    uStack_5c = uStack_8c;
    uStack_78 = uStack_a8;
    uStack_74 = uStack_a4;
    uStack_70 = uStack_a0;
    uStack_6c = uStack_9c;
    func_0x000142923c00(&uStack_138,param_4,0x90);
    func_0x000142923c00(param_4,auStack_2a0,0x90);
    if (!SBORROW8(0,CONCAT44(uStack_134,uStack_138))) {
      FUN_1405ef210(&uStack_138);
    }
    param_1[4] = CONCAT44(uStack_54,uStack_58);
    param_1[5] = CONCAT44(uStack_4c,uStack_50);
    param_1[2] = CONCAT44(uStack_64,uStack_68);
    param_1[3] = CONCAT44(uStack_5c,uStack_60);
    *param_1 = CONCAT44(uStack_74,uStack_78);
    param_1[1] = CONCAT44(uStack_6c,uStack_70);
  }
  return param_1;
}

