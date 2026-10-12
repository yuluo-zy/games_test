
void FUN_140bebc90(longlong param_1)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
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
  undefined1 uStack_29;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  if (*(longlong *)(param_1 + 0x50) == 0) {
    lVar1 = *(longlong *)(param_1 + 0x178);
    FUN_140bef4a0(&uStack_e0,lVar1,&UNK_142b14799,0xe,*(undefined4 *)(lVar1 + 0xad4));
    uStack_29 = 1;
    FUN_140bed230(&uStack_e0,lVar1,0);
    uStack_38 = uStack_90;
    uStack_48 = uStack_a0;
    uStack_40 = uStack_98;
    uStack_58 = uStack_b0;
    uStack_54 = uStack_ac;
    uStack_50 = uStack_a8;
    uStack_4c = uStack_a4;
    uStack_68 = uStack_c0;
    uStack_64 = uStack_bc;
    uStack_60 = uStack_b8;
    uStack_5c = uStack_b4;
    uStack_78 = uStack_d0;
    uStack_74 = uStack_cc;
    uStack_70 = uStack_c8;
    uStack_6c = uStack_c4;
    uStack_88 = uStack_e0;
    uStack_84 = uStack_dc;
    uStack_80 = uStack_d8;
    uStack_7c = uStack_d4;
    if (*(longlong *)(param_1 + 0x40) == 0) {
      FUN_140562f30((longlong *)(param_1 + 0x40),&UNK_142b147a8);
    }
    puVar2 = *(undefined8 **)(param_1 + 0x48);
    puVar2[10] = uStack_38;
    puVar2[8] = uStack_48;
    puVar2[9] = uStack_40;
    puVar2[6] = CONCAT44(uStack_54,uStack_58);
    puVar2[7] = CONCAT44(uStack_4c,uStack_50);
    puVar2[4] = CONCAT44(uStack_64,uStack_68);
    puVar2[5] = CONCAT44(uStack_5c,uStack_60);
    puVar2[2] = CONCAT44(uStack_74,uStack_78);
    puVar2[3] = CONCAT44(uStack_6c,uStack_70);
    *puVar2 = CONCAT44(uStack_84,uStack_88);
    puVar2[1] = CONCAT44(uStack_7c,uStack_80);
    *(undefined8 *)(param_1 + 0x50) = 1;
  }
  return;
}

