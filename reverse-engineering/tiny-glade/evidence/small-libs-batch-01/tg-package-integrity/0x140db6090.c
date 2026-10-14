
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140db6090(undefined8 *param_1,undefined8 param_2,ulonglong param_3)

{
  byte bVar1;
  byte bVar2;
  undefined8 in_stack_fffffffffffffef8;
  undefined7 uVar3;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined7 uStack_88;
  undefined1 uStack_81;
  undefined1 uStack_80;
  char cStack_7f;
  byte bStack_7e;
  byte bStack_7d;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  undefined1 uStack_10;
  ushort uStack_f;
  
  if (param_3 < 0x401) {
    uStack_b8 = 0;
    uStack_b0 = 0;
    uStack_c8 = 0;
    uStack_c0 = 0;
    uStack_d8 = 0;
    uStack_d0 = 0;
    uStack_e8 = 0;
    uStack_e0 = 0;
    uStack_a8 = CONCAT44(_UNK_142b6fa74,_DAT_142b6fa70);
    uStack_a0 = CONCAT44(_UNK_142b6fa7c,_UNK_142b6fa78);
    uStack_98 = _DAT_142b6fa80;
    uStack_94 = _UNK_142b6fa84;
    uStack_90 = _UNK_142b6fa88;
    uStack_8c = _UNK_142b6fa8c;
    uStack_88 = 0;
    uStack_81 = 0;
    uStack_80 = 0;
    cStack_7f = '\0';
    bStack_7e = 0;
    bStack_7d = 1;
    FUN_140db5360(&uStack_e8);
    bVar1 = cStack_7f == '\0' | bStack_7e;
    uStack_38 = uStack_a8;
    uStack_30 = uStack_a0;
    uStack_28 = CONCAT44(uStack_94,uStack_98);
    uStack_20 = CONCAT44(uStack_8c,uStack_90);
    uStack_78 = uStack_e8;
    uStack_70 = uStack_e0;
    uStack_68 = uStack_d8;
    uStack_60 = uStack_d0;
    uStack_58 = (undefined4)uStack_c8;
    uStack_54 = uStack_c8._4_4_;
    uStack_50 = (undefined4)uStack_c0;
    uStack_4c = uStack_c0._4_4_;
    uStack_48 = (undefined4)uStack_b8;
    uStack_44 = uStack_b8._4_4_;
    uStack_40 = (undefined4)uStack_b0;
    uStack_3c = uStack_b0._4_4_;
    uStack_18 = CONCAT17(uStack_81,uStack_88);
    uStack_10 = uStack_80;
    uStack_f = CONCAT11(bStack_7d,bVar1) | 2;
    uStack_d8 = CONCAT44(uStack_94,uStack_98);
    uStack_d0 = CONCAT44(uStack_8c,uStack_90);
    uStack_e8 = uStack_a8;
    uStack_e0 = uStack_a0;
    bVar2 = bVar1 | 10;
    uVar3 = (undefined7)((ulonglong)in_stack_fffffffffffffef8 >> 8);
    if (bStack_7d - 2 < 2) {
      FUN_1419c78d0(&uStack_e8,&uStack_78,uStack_80,0,CONCAT71(uVar3,bVar1) | 10);
      goto LAB_140db6279;
    }
    if (bStack_7d == 0) {
      FUN_1419cb220(&uStack_e8,&uStack_78,uStack_80,0,CONCAT71(uVar3,bVar1) | 10);
      goto LAB_140db6279;
    }
  }
  else {
    FUN_140db5e80(&uStack_78,param_2,param_3,&DAT_142b6fa70,0,0,1);
    uStack_28 = CONCAT44(_UNK_142b6fa84,_DAT_142b6fa80);
    uStack_20 = CONCAT44(_UNK_142b6fa8c,_UNK_142b6fa88);
    uStack_38 = CONCAT44(_UNK_142b6fa74,_DAT_142b6fa70);
    uStack_30 = CONCAT44(_UNK_142b6fa7c,_UNK_142b6fa78);
    uStack_10 = 0x40;
    uStack_18 = 0;
    uStack_f = 0x104;
    uStack_d8 = CONCAT44(_UNK_142b6fa84,_DAT_142b6fa80);
    uStack_d0 = CONCAT44(_UNK_142b6fa8c,_UNK_142b6fa88);
    bVar2 = 0xc;
  }
  uStack_e8 = uStack_38;
  uStack_e0 = uStack_30;
  FUN_1419cbe60(&uStack_e8,&uStack_78,uStack_10,0,bVar2);
LAB_140db6279:
  *param_1 = uStack_e8;
  param_1[1] = uStack_e0;
  param_1[2] = uStack_d8;
  param_1[3] = uStack_d0;
  return param_1;
}

