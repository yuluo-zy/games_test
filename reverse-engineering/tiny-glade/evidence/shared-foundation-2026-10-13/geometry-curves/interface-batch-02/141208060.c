
float FUN_141208060(undefined4 *param_1,float param_2)

{
  uint uStack_a4;
  float fStack_a0;
  float fStack_9c;
  float fStack_98;
  float fStack_94;
  float fStack_90;
  float fStack_8c;
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
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  
  uStack_38 = *(undefined8 *)(param_1 + 0x14);
  uStack_30 = *(undefined8 *)(param_1 + 0x16);
  uStack_48 = *(undefined8 *)(param_1 + 0x10);
  uStack_40 = *(undefined8 *)(param_1 + 0x12);
  uStack_88 = *param_1;
  uStack_84 = param_1[1];
  uStack_80 = param_1[2];
  uStack_7c = param_1[3];
  uStack_78 = param_1[4];
  uStack_74 = param_1[5];
  uStack_70 = param_1[6];
  uStack_6c = param_1[7];
  uStack_68 = param_1[8];
  uStack_64 = param_1[9];
  uStack_60 = param_1[10];
  uStack_5c = param_1[0xb];
  uStack_58 = param_1[0xc];
  uStack_54 = param_1[0xd];
  uStack_50 = param_1[0xe];
  uStack_4c = param_1[0xf];
  FUN_141237a70(&uStack_a4,&uStack_88);
  if (uStack_a4 == 1) {
    do {
      param_2 = param_2 + SQRT((fStack_8c - fStack_98) * (fStack_8c - fStack_98) +
                               (fStack_90 - fStack_9c) * (fStack_90 - fStack_9c) +
                               (fStack_94 - fStack_a0) * (fStack_94 - fStack_a0));
      FUN_141237a70(&uStack_a4,&uStack_88);
    } while ((uStack_a4 & 1) != 0);
  }
  return param_2;
}

