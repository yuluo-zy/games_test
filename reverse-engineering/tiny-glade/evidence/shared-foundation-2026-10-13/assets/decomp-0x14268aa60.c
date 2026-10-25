
char * FUN_14268aa60(char *param_1,undefined8 *param_2)

{
  undefined4 uVar1;
  char cVar2;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined1 auStack_108 [8];
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined1 auStack_e8 [4];
  undefined4 uStack_e4;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  uint uStack_78;
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
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined3 uStack_34;
  undefined4 uStack_30;
  undefined3 uStack_2c;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  FUN_14267f570(&uStack_80,*param_2);
  cVar2 = (char)uStack_80;
  if ((char)uStack_80 != '2') {
    uStack_2c = (undefined3)((uint)uStack_7c >> 8);
    uStack_e0 = CONCAT44(uStack_74,uStack_78);
    uStack_98 = CONCAT44(uStack_6c,uStack_70);
    uStack_90 = CONCAT44(uStack_64,uStack_68);
    uStack_88 = CONCAT44(uStack_5c,uStack_60);
    uStack_168 = CONCAT44(uStack_54,uStack_58);
    uStack_160 = CONCAT44(uStack_4c,uStack_50);
    uStack_158 = CONCAT44(uStack_44,uStack_48);
    uStack_150 = CONCAT44(uStack_3c,uStack_40);
    uVar1 = CONCAT13((undefined1)uStack_7c,uStack_80._1_3_);
    goto joined_r0x00014268ab78;
  }
  uStack_100 = uStack_70;
  uStack_fc = uStack_6c;
  uStack_f8 = uStack_68;
  uStack_f4 = uStack_64;
  uStack_d8 = 0x8000000000000000;
  if ((char)uStack_78 == '\x10') {
LAB_14268ad08:
    uStack_90 = uStack_148;
    uStack_88 = uStack_140;
    uStack_e0 = 0;
    uStack_98 = uStack_d8;
  }
  else {
    if ((uStack_78 & 0xff) == 0x11) {
      FUN_142691cd0(&uStack_80,CONCAT44(uStack_6c,uStack_70),&UNK_143150768,0x13,&UNK_143150780,1);
    }
    else {
      if ((uStack_78 & 0xff) == 0x12) goto LAB_14268ad08;
      FUN_142691cd0(&uStack_80,auStack_108,&UNK_143150768,0x13,&UNK_143150780,1);
    }
    if ((char)uStack_80 == '2') {
      uStack_d8 = CONCAT44(uStack_74,uStack_78);
      uStack_148 = CONCAT44(uStack_6c,uStack_70);
      uStack_140 = CONCAT44(uStack_64,uStack_68);
      goto LAB_14268ad08;
    }
    uStack_38 = CONCAT13((undefined1)uStack_7c,uStack_80._1_3_);
    uStack_34 = (undefined3)((uint)uStack_7c >> 8);
    uStack_d8 = CONCAT44(uStack_74,uStack_78);
    uStack_148 = CONCAT44(uStack_6c,uStack_70);
    uStack_140 = CONCAT44(uStack_64,uStack_68);
    if ((char)uStack_80 == '2') goto LAB_14268ad08;
    _auStack_e8 = CONCAT41(uStack_38,(char)uStack_80);
    _auStack_e8 = CONCAT44(uStack_7c,auStack_e8);
    uStack_c0 = uStack_58;
    uStack_bc = uStack_54;
    uStack_b8 = uStack_50;
    uStack_b4 = uStack_4c;
    uStack_b0 = uStack_48;
    uStack_ac = uStack_44;
    FUN_1426898d0(auStack_e8);
    FUN_142691cd0(&uStack_80,auStack_108,&UNK_143150768,0x13,&UNK_143150780,1);
    if ((char)uStack_80 == '2') {
      uStack_c8 = CONCAT44(uStack_64,uStack_68);
      uStack_d8 = CONCAT44(uStack_74,uStack_78);
      uStack_d0 = CONCAT44(uStack_6c,uStack_70);
      uStack_e0 = 1;
LAB_14268ae02:
      uStack_88 = uStack_c8;
      uStack_98 = uStack_d8;
      uStack_90 = uStack_d0;
    }
    else {
      _auStack_e8 = CONCAT44(uStack_7c,uStack_80);
      uStack_e0 = CONCAT44(uStack_74,uStack_78);
      uStack_b8 = uStack_50;
      uStack_b4 = uStack_4c;
      uStack_b0 = uStack_48;
      uStack_ac = uStack_44;
      uStack_c8 = CONCAT44(uStack_5c,uStack_60);
      uStack_c0 = uStack_58;
      uStack_bc = uStack_54;
      uStack_d8 = CONCAT44(uStack_6c,uStack_70);
      uStack_d0 = CONCAT44(uStack_64,uStack_68);
      if ((char)uStack_80 == '2') goto LAB_14268ae02;
      FUN_1426898d0(auStack_e8);
      FUN_142922ad0(&uStack_80,&UNK_143150438,0x3b);
      uStack_30._0_3_ = (undefined3)((uint)uStack_80 >> 8);
      uStack_30._3_1_ = (undefined1)uStack_7c;
      uStack_2c = (undefined3)((uint)uStack_7c >> 8);
      uStack_e0 = CONCAT44(uStack_74,uStack_78);
      uStack_98 = CONCAT44(uStack_6c,uStack_70);
      uStack_90 = CONCAT44(uStack_64,uStack_68);
      uStack_88 = CONCAT44(uStack_5c,uStack_60);
      uStack_168 = CONCAT44(uStack_54,uStack_58);
      uStack_160 = CONCAT44(uStack_4c,uStack_50);
      uStack_158 = CONCAT44(uStack_44,uStack_48);
      uStack_150 = CONCAT44(uStack_3c,uStack_40);
      cVar2 = (char)uStack_80;
    }
  }
  FUN_1426899a0(auStack_108);
  uVar1 = uStack_30;
joined_r0x00014268ab78:
  if (cVar2 == '2') {
    *(undefined8 *)(param_1 + 0x18) = uStack_88;
    *(undefined8 *)(param_1 + 8) = uStack_98;
    *(undefined8 *)(param_1 + 0x10) = uStack_90;
    *param_1 = '2';
  }
  else {
    uStack_30._3_1_ = (undefined1)((uint)uVar1 >> 0x18);
    *(uint *)(param_1 + 4) = CONCAT31(uStack_2c,uStack_30._3_1_);
    *(undefined4 *)(param_1 + 1) = uVar1;
    *(undefined8 *)(param_1 + 0x38) = uStack_158;
    *(undefined8 *)(param_1 + 0x40) = uStack_150;
    *(undefined8 *)(param_1 + 0x28) = uStack_168;
    *(undefined8 *)(param_1 + 0x30) = uStack_160;
    *(undefined8 *)(param_1 + 0x20) = uStack_88;
    *(undefined8 *)(param_1 + 0x10) = uStack_98;
    *(undefined8 *)(param_1 + 0x18) = uStack_90;
    *param_1 = cVar2;
    *(undefined8 *)(param_1 + 8) = uStack_e0;
  }
  return param_1;
}

