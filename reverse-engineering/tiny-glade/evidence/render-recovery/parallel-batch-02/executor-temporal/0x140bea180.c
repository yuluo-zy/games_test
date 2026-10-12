
longlong FUN_140bea180(longlong param_1,undefined4 param_2,undefined8 *param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  
  func_0x000142923c00(param_1,param_2,0x90);
  uVar1 = param_3[1];
  uVar2 = param_3[2];
  uVar3 = param_3[3];
  uVar4 = param_3[4];
  uVar5 = param_3[5];
  uVar6 = param_3[6];
  uVar7 = param_3[7];
  *(undefined8 *)(param_1 + 0x90) = *param_3;
  *(undefined8 *)(param_1 + 0x98) = uVar1;
  *(undefined8 *)(param_1 + 0xa0) = uVar2;
  *(undefined8 *)(param_1 + 0xa8) = uVar3;
  *(undefined8 *)(param_1 + 0xb0) = uVar4;
  *(undefined8 *)(param_1 + 0xb8) = uVar5;
  *(undefined8 *)(param_1 + 0xc0) = uVar6;
  *(undefined8 *)(param_1 + 200) = uVar7;
  *(undefined8 *)(param_1 + 0xd0) = param_3[8];
  return param_1;
}

