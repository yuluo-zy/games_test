
void FUN_140c029f0(undefined8 *param_1,longlong param_2)

{
  ulonglong uVar1;
  code *pcVar2;
  undefined *puVar3;
  undefined4 *puVar4;
  int aiStack_f8 [2];
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 uStack_78;
  undefined *puStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  uStack_a8 = CONCAT71(uStack_a8._1_7_,0x29);
  FUN_1418c1380(0x143634e40,&uStack_a8,1);
  if (4 < *puRam000000014362c3a8) {
    puVar4 = (undefined4 *)func_0x00014067dd80(&UNK_142b18020);
    uStack_88 = *puVar4;
    uStack_84 = puVar4[1];
    uStack_80 = puVar4[2];
    uStack_7c = puVar4[3];
    uStack_78 = 5;
    puStack_70 = &UNK_142b18038;
    uStack_68 = 0x20;
    puStack_58 = &UNK_142b18010;
    uStack_50 = 1;
    uStack_48 = 8;
    uStack_a8 = 0;
    uStack_40 = 0;
    uStack_38 = 0;
    uStack_a0 = &UNK_142b18038;
    uStack_98 = 0x20;
    uStack_90 = 0;
    uStack_60 = CONCAT44(puVar4[4],1);
    func_0x000140d4a330(aiStack_f8,&uStack_a8);
  }
  puVar3 = uStack_a0;
  uVar1 = *(ulonglong *)(*(longlong *)(param_2 + 0x658) + 0x340);
  if (uVar1 < 0x10000) {
    uStack_a8 = 0x40000;
    uStack_a0._7_1_ = SUB81(puVar3,7);
    uStack_a0._0_7_ = CONCAT16(2,CONCAT24((short)uVar1,0x20033));
    FUN_140c038c0(aiStack_f8,param_2,&UNK_142b18058,0x18,&uStack_a8);
    if (aiStack_f8[0] == 0xb) {
      param_1[4] = CONCAT44(uStack_cc,uStack_d0);
      param_1[2] = CONCAT44(uStack_dc,uStack_e0);
      param_1[3] = CONCAT44(uStack_d4,uStack_d8);
      *param_1 = CONCAT44(uStack_ec,uStack_f0);
      param_1[1] = CONCAT44(uStack_e4,uStack_e8);
      return;
    }
  }
  else {
    FUN_1428d9760(&UNK_142b0f8c8,5,aiStack_f8,&UNK_142b0f768,&UNK_142b0f8d0);
  }
  uStack_68 = uStack_b8;
  uStack_60 = uStack_b0;
  uStack_88 = uStack_d8;
  uStack_84 = uStack_d4;
  uStack_80 = uStack_d0;
  uStack_7c = uStack_cc;
  FUN_1428d9760(&UNK_142b17db0,0x2b,&uStack_a8,&UNK_142b17d90,&UNK_142b18070);
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

