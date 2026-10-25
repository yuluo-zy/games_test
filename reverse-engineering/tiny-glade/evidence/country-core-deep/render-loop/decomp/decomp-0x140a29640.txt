
undefined8 *
FUN_140a29640(undefined8 *param_1,undefined8 param_2,undefined8 param_3,longlong param_4,
             undefined1 param_5,undefined1 param_6)

{
  undefined *puVar1;
  undefined4 uStack_388;
  undefined4 uStack_384;
  undefined4 uStack_380;
  undefined4 uStack_37c;
  undefined4 uStack_378;
  undefined4 uStack_374;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined4 uStack_35c;
  undefined *puStack_358;
  undefined8 uStack_350;
  undefined1 auStack_348 [384];
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  
  puVar1 = *(undefined **)(param_4 + 0x1450);
  FUN_140bd4e20(&uStack_368,0x6c00,0x20);
  uStack_1c8 = uStack_368;
  uStack_1c4 = uStack_364;
  uStack_1c0 = uStack_360;
  uStack_1bc = uStack_35c;
  func_0x000140bd6d00(auStack_348,&uStack_1c8);
  puStack_358 = &UNK_142acaa35;
  uStack_350 = 0x1e;
  uStack_378 = FUN_140be94d0(param_2,&puStack_358);
  uStack_388 = uStack_368;
  uStack_384 = uStack_364;
  uStack_380 = uStack_360;
  uStack_37c = uStack_35c;
  FUN_1416032a0(&puStack_358,&UNK_142acaa53,0x22,&UNK_142acaa75,0x3b,param_3,param_2);
  FUN_14160d110(&uStack_1c8,&puStack_358,0x86fe9dc51990c5e3);
  FUN_141608090(&puStack_358,&uStack_1c8,0xd024032eecbe8bdf);
  FUN_14096dd60(&uStack_1c8,&puStack_358,0x5fc6561e3e33cde8,&uStack_388);
  uStack_350._0_2_ = CONCAT11(param_6,param_5);
  puStack_358 = puVar1;
  FUN_14160a0a0(&uStack_1c8,&puStack_358);
  param_1[2] = CONCAT44(uStack_374,uStack_378);
  *param_1 = CONCAT44(uStack_384,uStack_388);
  param_1[1] = CONCAT44(uStack_37c,uStack_380);
  return param_1;
}

