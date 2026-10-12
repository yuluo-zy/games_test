
void FUN_140a29280(undefined8 *param_1,undefined8 param_2,undefined8 param_3,longlong param_4,
                  undefined1 param_5,undefined1 param_6,undefined8 param_7,undefined8 param_8,
                  undefined8 param_9,undefined8 param_10)

{
  uint uVar1;
  longlong lVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uStack_b68;
  undefined4 uStack_b64;
  undefined4 uStack_b60;
  undefined4 uStack_b5c;
  undefined4 uStack_b58;
  undefined4 uStack_b54;
  undefined8 uStack_b50;
  longlong lStack_b48;
  undefined8 *puStack_b40;
  longlong lStack_b38;
  undefined4 uStack_b30;
  undefined4 uStack_b2c;
  undefined *puStack_9a0;
  undefined8 uStack_998;
  undefined1 auStack_990 [384];
  undefined4 uStack_810;
  undefined4 uStack_80c;
  undefined4 uStack_808;
  undefined4 uStack_804;
  undefined1 auStack_680 [400];
  undefined1 auStack_4f0 [400];
  undefined1 auStack_360 [400];
  undefined1 auStack_1d0 [400];
  
  lStack_b48 = *(longlong *)(param_4 + 0x1450);
  uStack_b50 = param_3;
  puStack_b40 = param_1;
  FUN_140bd4e20(&uStack_810,(ulonglong)*(uint *)(lStack_b48 + 0x60) << 2,0x20);
  uStack_b30 = uStack_808;
  uStack_b2c = uStack_804;
  func_0x000140bd6d00(auStack_990,&lStack_b38);
  puStack_9a0 = &UNK_142aca992;
  uStack_998 = 0x15;
  uStack_b58 = FUN_140be94d0(param_2,&puStack_9a0);
  uStack_b68 = uStack_810;
  uStack_b64 = uStack_80c;
  uStack_b60 = uStack_808;
  uStack_b5c = uStack_804;
  uVar3 = func_0x000140bece50();
  uVar4 = 0;
  FUN_1416032a0(&puStack_9a0,&UNK_142aca9a7,0x20,&UNK_142aca9c7,0x29,param_3,param_2);
  FUN_14160d110(&lStack_b38,&puStack_9a0,0x86fe9dc51990c5e3);
  FUN_14096dac0(&uStack_810,&lStack_b38,0x5337bbde238aac84,param_8);
  FUN_14160c630(auStack_1d0,&uStack_810,0xd005d9dfc249ec9b);
  FUN_14096dac0(auStack_360,auStack_1d0,0x77fada2ea41d423b,param_10);
  FUN_14096e000(auStack_4f0,auStack_360,0xfee3c4c9d72cf101,param_9,CONCAT44(uVar4,1));
  FUN_14096e000(auStack_680,auStack_4f0,0x9b5e79943aa2230e,&uStack_b68,1);
  func_0x000142923c00(&puStack_9a0,auStack_680,400);
  FUN_141608450(auStack_680,&puStack_9a0,0xd024032eecbe8bdf,uVar3);
  func_0x000142923c00(&puStack_9a0,auStack_680,400);
  lVar2 = lStack_b48;
  uVar1 = uStack_b30 >> 0x10;
  uStack_b30._0_2_ = CONCAT11(param_6,param_5);
  uStack_b30 = CONCAT22((short)uVar1,(undefined2)uStack_b30) & 0xff00ffff;
  lStack_b38 = lStack_b48;
  FUN_14160bdf0(&puStack_9a0,param_7,&lStack_b38);
  uVar3 = 0;
  FUN_1416032a0(&puStack_9a0,&UNK_142aca9f0,0x1e,&UNK_142acaa0e,0x27,uStack_b50,param_2);
  FUN_14160d110(&lStack_b38,&puStack_9a0,0x86fe9dc51990c5e3);
  FUN_14096dac0(&uStack_810,&lStack_b38,0x5337bbde238aac84,param_8);
  FUN_14160c630(auStack_1d0,&uStack_810,0xd005d9dfc249ec9b);
  FUN_14096dac0(auStack_360,auStack_1d0,0x77fada2ea41d423b,param_10);
  FUN_14096e000(auStack_4f0,auStack_360,0xfee3c4c9d72cf101,param_9,CONCAT44(uVar3,1));
  FUN_14096e000(auStack_680,auStack_4f0,0x9b5e79943aa2230e,&uStack_b68,1);
  func_0x000142923c00(&puStack_9a0,auStack_680,400);
  uStack_b30 = CONCAT13(uStack_b30._3_1_,0x10000);
  uStack_b30._0_2_ = CONCAT11(param_6,param_5);
  lStack_b38 = lVar2;
  FUN_14160bdf0(&puStack_9a0,param_7,&lStack_b38);
  puStack_b40[2] = CONCAT44(uStack_b54,uStack_b58);
  *puStack_b40 = CONCAT44(uStack_b64,uStack_b68);
  puStack_b40[1] = CONCAT44(uStack_b5c,uStack_b60);
  return;
}

