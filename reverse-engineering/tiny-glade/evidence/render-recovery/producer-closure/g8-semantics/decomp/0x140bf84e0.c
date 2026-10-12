
undefined8 * FUN_140bf84e0(undefined8 *param_1,longlong param_2)

{
  char cVar1;
  longlong lVar2;
  code *pcVar3;
  undefined8 uVar4;
  int iVar5;
  char *pcVar6;
  bool bVar7;
  undefined8 in_stack_fffffffffffffe48;
  undefined4 uVar8;
  longlong lStack_180;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined *puStack_d0;
  undefined8 uStack_c8;
  undefined8 *puStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined4 *puStack_50;
  undefined8 uStack_48;
  char *pcStack_40;
  undefined1 uStack_31;
  undefined8 uStack_30;
  
  uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffe48 >> 0x20);
  uStack_30 = 0xfffffffffffffffe;
  pcVar6 = (char *)(param_2 + 0x9f0);
  LOCK();
  bVar7 = *(char *)(param_2 + 0x9f0) == '\0';
  if (bVar7) {
    *(char *)(param_2 + 0x9f0) = '\x01';
  }
  UNLOCK();
  if (!bVar7) {
    FUN_1428cd210(pcVar6,param_2,1000000000);
  }
  FUN_140c81c70(pcVar6);
  lVar2 = *(longlong *)(param_2 + 0x9f8);
  *(undefined8 *)(param_2 + 0x9f8) = 4;
  if (lVar2 != 4) {
    uVar4 = *(undefined8 *)(param_2 + 0xa40);
    uStack_130 = *(undefined8 *)(param_2 + 0xa48);
    uStack_178 = *(undefined4 *)(param_2 + 0xa00);
    uStack_174 = *(undefined4 *)(param_2 + 0xa04);
    uStack_170 = *(undefined4 *)(param_2 + 0xa08);
    uStack_16c = *(undefined4 *)(param_2 + 0xa0c);
    uStack_168 = *(undefined4 *)(param_2 + 0xa10);
    uStack_164 = *(undefined4 *)(param_2 + 0xa14);
    uStack_160 = *(undefined4 *)(param_2 + 0xa18);
    uStack_15c = *(undefined4 *)(param_2 + 0xa1c);
    uStack_158 = *(undefined4 *)(param_2 + 0xa20);
    uStack_154 = *(undefined4 *)(param_2 + 0xa24);
    uStack_150 = *(undefined4 *)(param_2 + 0xa28);
    uStack_14c = *(undefined4 *)(param_2 + 0xa2c);
    uStack_148 = *(undefined4 *)(param_2 + 0xa30);
    uStack_144 = *(undefined4 *)(param_2 + 0xa34);
    uStack_140 = *(undefined4 *)(param_2 + 0xa38);
    uStack_13c = *(undefined4 *)(param_2 + 0xa3c);
    puStack_d0 = (undefined *)CONCAT44(puStack_d0._4_4_,0x2e);
    uStack_c8 = 0;
    puStack_c0 = (undefined8 *)0x1800000018000;
    lStack_180 = lVar2;
    uStack_138 = uVar4;
    pcStack_40 = pcVar6;
    (**(code **)(param_2 + 0x3e0))(uVar4,0x10000,0x10000,0,CONCAT44(uVar8,1),&puStack_d0,0,8,0,8);
    puStack_d0 = (undefined *)CONCAT71(puStack_d0._1_7_,0x18);
    uStack_31 = 1;
    FUN_1418c1380(0x143634e40,&puStack_d0,1);
    if (lVar2 == 1) {
      iVar5 = (**(code **)(param_2 + 0x2c8))(uVar4);
      if (iVar5 == 0) {
        lStack_180 = 2;
        puStack_d0 = (undefined *)0x0;
        uStack_c8 = 8;
        puStack_c0 = (undefined8 *)0x0;
        uStack_b8 = 0;
        uStack_b0 = 4;
        uStack_a8 = 0;
        uStack_a0 = 0;
        uStack_98 = 8;
        uStack_90 = 0;
        uStack_88 = 0;
        uStack_80 = 8;
        uStack_78 = 0;
        uStack_70 = 0;
        uStack_68 = 8;
        uStack_60 = 0;
        uStack_58 = 0;
        puStack_50 = (undefined4 *)0x8;
        uStack_48 = 0;
        uStack_d8 = uStack_130;
        uStack_118 = CONCAT44(uStack_16c,uStack_170);
        uStack_110 = CONCAT44(uStack_164,uStack_168);
        uStack_e8 = uStack_140;
        uStack_e4 = uStack_13c;
        uStack_e0 = (undefined4)uStack_138;
        uStack_dc = uStack_138._4_4_;
        uStack_f8 = uStack_150;
        uStack_f4 = uStack_14c;
        uStack_f0 = uStack_148;
        uStack_ec = uStack_144;
        uStack_108 = uStack_160;
        uStack_104 = uStack_15c;
        uStack_100 = uStack_158;
        uStack_fc = uStack_154;
        uStack_128 = (longlong *)0x2;
        uStack_120 = (code *)CONCAT44(uStack_174,uStack_178);
        FUN_140562f30(&uStack_58,&UNK_142d06710);
        *(undefined8 *)(puStack_50 + 0x14) = uStack_d8;
        *(ulonglong *)(puStack_50 + 0x10) = CONCAT44(uStack_e4,uStack_e8);
        *(ulonglong *)(puStack_50 + 0x12) = CONCAT44(uStack_dc,uStack_e0);
        puStack_50[0xc] = uStack_f8;
        puStack_50[0xd] = uStack_f4;
        puStack_50[0xe] = uStack_f0;
        puStack_50[0xf] = uStack_ec;
        puStack_50[8] = uStack_108;
        puStack_50[9] = uStack_104;
        puStack_50[10] = uStack_100;
        puStack_50[0xb] = uStack_fc;
        puStack_50[4] = (undefined4)uStack_118;
        puStack_50[5] = uStack_118._4_4_;
        puStack_50[6] = (undefined4)uStack_110;
        puStack_50[7] = uStack_110._4_4_;
        *puStack_50 = (undefined4)uStack_128;
        puStack_50[1] = uStack_128._4_4_;
        puStack_50[2] = (undefined4)uStack_120;
        puStack_50[3] = uStack_120._4_4_;
        uStack_48 = 1;
        func_0x000142923c00(param_1,&puStack_d0,0x90);
        *(undefined4 *)(param_1 + 0x12) = 0;
        pcVar6 = pcStack_40;
        goto code_r0x000140bf8782;
      }
      puStack_d0 = (undefined *)CONCAT44(puStack_d0._4_4_,iVar5);
      uStack_31 = 1;
      FUN_1428d9760(&UNK_142b14d20,0x12,&puStack_d0,&UNK_142b14b30,&UNK_142b14d38);
    }
    else {
      uStack_128 = &lStack_180;
      uStack_120 = FUN_140bed620;
      puStack_d0 = &UNK_142b14d78;
      uStack_c8 = 1;
      uStack_b0 = 0;
      puStack_c0 = &uStack_128;
      uStack_b8 = 1;
      uStack_31 = 1;
      FUN_1428d9390(&puStack_d0,&UNK_142b14d88);
    }
                    /* WARNING: Does not return */
    pcVar3 = (code *)invalidInstructionException();
    (*pcVar3)();
  }
  *param_1 = 0x8000000000000000;
code_r0x000140bf8782:
  FUN_140c81e10();
  LOCK();
  cVar1 = *pcVar6;
  if (cVar1 == '\x01') {
    *pcVar6 = '\0';
  }
  UNLOCK();
  if (cVar1 != '\x01') {
    FUN_1428cdb90(pcVar6,0);
  }
  return param_1;
}

