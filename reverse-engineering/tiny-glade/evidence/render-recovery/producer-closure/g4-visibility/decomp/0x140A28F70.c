
undefined8
FUN_140a28f70(undefined8 param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4,
             undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,int param_9
             ,undefined8 *param_10)

{
  undefined4 uVar1;
  undefined1 auStack_690 [400];
  undefined1 auStack_500 [400];
  undefined1 auStack_370 [400];
  undefined4 auStack_1e0 [68];
  undefined1 auStack_d0 [38];
  char cStack_aa;
  longlong lStack_a8;
  undefined8 *puStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_1415fcf10(auStack_370,&UNK_142aca980,0x12,param_2,param_3,param_4);
  uVar1 = (undefined4)((ulonglong)param_3 >> 0x20);
  FUN_141608a20(auStack_1e0,auStack_370,0x686c38b1c905f34d,1);
  FUN_141608260(auStack_370,auStack_1e0,0xd005d9dfc249ec9b,1);
  FUN_14096e000(auStack_1e0,auStack_370,0x77fada2ea41d423b,param_5,CONCAT44(uVar1,1));
  FUN_14096e000(auStack_370,auStack_1e0,0x7ab593be137ba575,param_6,1);
  FUN_14096e2a0(auStack_1e0,auStack_370,0x2a75aeb2d5bdccbd,param_7);
  FUN_14096e2a0(auStack_690,auStack_1e0,0x668a7272bfe497f1,param_8);
  if (param_9 == 0) {
    FUN_14160df90(auStack_370,auStack_690,0x35ab36bc43c7b8ed);
    FUN_141608e00(auStack_500,auStack_370,0x3783a9903b7f67c5,1);
  }
  else {
    FUN_14160cd70();
    FUN_1416091e0(auStack_500,auStack_370,0x3783a9903b7f67c5,1);
  }
  if (param_10 != (undefined8 *)0x0) {
    func_0x000142923c00(auStack_370,auStack_500,400);
    puStack_50 = param_10 + 1;
    auStack_1e0[0] = 2;
    FUN_1408ee240(&puStack_50,auStack_370,0x62b6812f89a8a569,0x19,auStack_1e0);
    func_0x000142923c00(auStack_1e0,auStack_370,400);
    if (cStack_aa != '\b') {
      FUN_14096b760(auStack_d0,lStack_a8 + 0xe8,0xe8d23613960341d3,*param_10);
    }
    func_0x000142923c00(auStack_500,auStack_1e0,400);
  }
  func_0x000142923c00(param_1,auStack_500,400);
  return param_1;
}

