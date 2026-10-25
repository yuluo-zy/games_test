
void FUN_140a297b0(undefined8 param_1,undefined8 param_2,longlong param_3,undefined1 param_4,
                  undefined1 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  undefined8 uVar1;
  undefined8 uStack_360;
  undefined1 uStack_358;
  undefined1 uStack_357;
  undefined1 auStack_1d0 [400];
  
  uVar1 = *(undefined8 *)(param_3 + 0x1450);
  FUN_1416032a0(&uStack_360,&UNK_142acaae7,0x1e,&UNK_142acab05,0x27,param_2,param_1);
  FUN_141606900(auStack_1d0,&uStack_360,0x8661a811321c4474);
  FUN_14096dd60(&uStack_360,auStack_1d0,0xaab861d318ad7248,param_7);
  FUN_14096dd60(auStack_1d0,&uStack_360,0xc3674440971f6990,param_8);
  uStack_357 = param_5;
  uStack_360 = uVar1;
  uStack_358 = param_4;
  FUN_14160b850(auStack_1d0,param_6,&uStack_360);
  FUN_1416032a0(&uStack_360,&UNK_142acab2c,0x1d,&UNK_142acab49,0x21,param_2,param_1);
  FUN_14096dd60(auStack_1d0,&uStack_360,0xaab861d318ad7248,param_8);
  uStack_357 = param_5;
  uStack_360 = uVar1;
  uStack_358 = param_4;
  FUN_14160b580(auStack_1d0,param_6,&uStack_360);
  return;
}

