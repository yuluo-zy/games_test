
undefined8 * FUN_1403ac400(undefined8 *param_1,undefined8 *param_2)

{
  code *pcVar1;
  undefined *puVar2;
  int iVar3;
  undefined8 uVar4;
  undefined *puVar5;
  undefined1 *puVar6;
  longlong lVar7;
  undefined *puVar8;
  undefined4 *puVar9;
  undefined *puVar10;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined *puStack_138;
  undefined *puStack_130;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_117;
  undefined3 uStack_113;
  int iStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined *puStack_100;
  undefined *puStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined8 uStack_a4;
  undefined8 uStack_9c;
  undefined8 uStack_94;
  undefined8 uStack_8c;
  undefined8 uStack_84;
  undefined *puStack_78;
  uint uStack_6c;
  undefined *puStack_68;
  undefined *puStack_60;
  undefined *puStack_58;
  undefined1 *puStack_50;
  undefined *puStack_48;
  undefined1 uStack_3b;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  FUN_140697b10(&uStack_e8,*param_2);
  if ((int)uStack_e8 == 1) {
    uStack_1a8 = uStack_e0;
    FUN_1428d9760(&UNK_1429cb6e8,0x2b,&uStack_1a8,&UNK_1429cb718,&UNK_1429cb850);
    goto LAB_1403ac9a9;
  }
  uStack_148 = uStack_84;
  uStack_158 = uStack_94;
  uStack_150 = uStack_8c;
  uStack_168 = uStack_a4;
  uStack_160 = uStack_9c;
  uStack_178 = uStack_b8._4_4_;
  uStack_174 = uStack_b0;
  uStack_170 = uStack_ac;
  uStack_16c = uStack_a8;
  uStack_188 = uStack_c8._4_4_;
  uStack_184 = (undefined4)uStack_c0;
  uStack_180 = uStack_c0._4_4_;
  uStack_17c = (undefined4)uStack_b8;
  uStack_198 = uStack_d8._4_4_;
  uStack_194 = (undefined4)uStack_d0;
  uStack_190 = uStack_d0._4_4_;
  uStack_18c = (undefined4)uStack_c8;
  uStack_1a8 = (undefined *)CONCAT44((undefined4)uStack_e0,uStack_e8._4_4_);
  uStack_1a0 = CONCAT44((undefined4)uStack_d8,uStack_e0._4_4_);
  puVar9 = &uStack_180;
  FUN_140695be0(&uStack_e8,puVar9,0x20);
  uVar4 = FUN_140d76ed0(&uStack_e8);
  FUN_140d77340(&iStack_110,uVar4,puVar9);
  puVar2 = puStack_100;
  puVar8 = (undefined *)CONCAT44(uStack_104,uStack_108);
  if ((longlong)puStack_100 < 0) {
    uVar4 = 0;
LAB_1403ac978:
    puStack_48 = puVar8;
    FUN_1428d8fe3(uVar4,puVar2,&UNK_1429cb788);
LAB_1403ac9a9:
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
  if (puStack_100 == (undefined *)0x0) {
    puVar5 = (undefined *)0x1;
  }
  else {
    uVar4 = 1;
    puVar5 = (undefined *)func_0x000140613c10(puStack_100,1);
    if (puVar5 == (undefined *)0x0) goto LAB_1403ac978;
  }
  func_0x000142923c00(puVar5,puVar8,puVar2);
  puVar10 = uStack_e8;
  if ((CONCAT44(uStack_10c,iStack_110) & 0x7fffffffffffffff) != 0) {
    func_0x000140613c20(puVar8,CONCAT44(uStack_10c,iStack_110),1);
    puVar10 = uStack_e8;
  }
  if (puVar10 != (undefined *)0x0) {
    func_0x000140613c20(uStack_e0,puVar10,1);
  }
  puStack_68 = puVar2;
  puStack_78 = puVar5;
  if (puVar2 < (undefined *)0x10) {
    if (puVar2 != (undefined *)0x0) {
      puVar10 = (undefined *)0x0;
      do {
        if (puVar5[(longlong)puVar10] == '\0') goto LAB_1403ac904;
        puVar10 = puVar10 + 1;
      } while (puVar2 != puVar10);
    }
  }
  else {
    uStack_39 = 0;
    lVar7 = FUN_140d88a40(0,puVar5,puVar2);
    puVar10 = puVar5;
    if (lVar7 != 0) {
LAB_1403ac904:
      uStack_e8 = puStack_68;
      uStack_e0 = puStack_78;
      uStack_d8 = puStack_68;
      uStack_d0 = puVar10;
      FUN_1428d9760(&UNK_1429cb6e8,0x2b,&uStack_e8,&UNK_1429cb6c8,&UNK_1429cb900);
      goto LAB_1403ac9a9;
    }
  }
  uStack_39 = 1;
  uStack_e8 = puStack_68;
  uStack_e0 = puStack_78;
  uStack_d8 = puStack_68;
  puVar6 = (undefined1 *)FUN_140d79950(&uStack_e8);
  uVar4 = func_0x000142923470(puVar6,0,0,0);
  lVar7 = func_0x000140613c40(0x800,1);
  puStack_50 = puVar6;
  puStack_48 = puVar10;
  if (lVar7 == 0) {
    FUN_1428d8fe3(1,0x800,&UNK_1429cb918);
    goto LAB_1403ac9a9;
  }
  uStack_6c = 0x800;
  iVar3 = func_0x000142923490(uVar4,&uStack_6c,lVar7);
  puStack_78 = (undefined *)CONCAT44(puStack_78._4_4_,iVar3);
  puStack_68 = (undefined *)lVar7;
  if (iVar3 == 0) {
    uStack_e8 = &UNK_1429cb958;
    uStack_e0 = (undefined *)0x1;
    uStack_d8 = (undefined *)0x8;
    uStack_d0 = (undefined *)0x0;
    uStack_c8 = (undefined *)0x0;
    puVar8 = (undefined *)FUN_1428a2020(&uStack_e8);
    func_0x000142923480(uVar4);
    func_0x000140613c20(puStack_68,0x800,1);
    puStack_138 = (undefined *)0x8000000000000000;
LAB_1403ac6fc:
    *puStack_50 = 0;
    if (puStack_48 != (undefined *)0x0) {
      func_0x000140613c20(puStack_50,puStack_48,1);
    }
    *param_1 = puStack_138;
    param_1[1] = puVar8;
    param_1[2] = lVar7;
    *(undefined1 *)(param_1 + 3) = 1;
    *(undefined4 *)((longlong)param_1 + 0x19) = uStack_117;
    *(uint *)((longlong)param_1 + 0x1c) = CONCAT31(uStack_113,uStack_117._3_1_);
  }
  else {
    uStack_d8 = (undefined *)0x800;
    if (uStack_6c < 0x800) {
      uStack_d8 = (undefined *)(ulonglong)uStack_6c;
    }
    uStack_e8 = (undefined *)0x800;
    uStack_e0 = (undefined *)lVar7;
    FUN_140d79ce0(&iStack_110,&uStack_e8);
    if (iStack_110 == 2) {
      FUN_140d79b10(&puStack_138,CONCAT44(uStack_104,uStack_108),puStack_100);
      if ((char)uStack_120 == '\x02') {
        if (!SBORROW8(0,(longlong)puStack_138)) {
          lVar7 = CONCAT44(uStack_124,uStack_128);
          func_0x000142923480(uVar4);
          puVar8 = puStack_130;
          goto LAB_1403ac6fc;
        }
      }
      else {
        puStack_58 = puStack_138;
        puStack_60 = puStack_130;
        uStack_3b = 1;
        FUN_140d56700(&uStack_e8);
        uStack_188 = (undefined4)uStack_c8;
        uStack_184 = uStack_c8._4_4_;
        uStack_180 = (undefined4)uStack_c0;
        uStack_17c = uStack_c0._4_4_;
        uStack_198 = (undefined4)uStack_d8;
        uStack_194 = uStack_d8._4_4_;
        uStack_190 = (undefined4)uStack_d0;
        uStack_18c = uStack_d0._4_4_;
        uStack_1a8 = uStack_e8;
        uStack_1a0 = (longlong)uStack_e0;
        uStack_d8 = puStack_58;
        uStack_d0 = puStack_60;
        uStack_c8 = (undefined *)CONCAT44(uStack_124,uStack_128);
        uStack_c0 = (undefined *)CONCAT44(uStack_11c,uStack_120);
        uStack_e8 = &UNK_1429cb968;
        uStack_e0 = (undefined *)0x18;
        uStack_3b = 0;
        puStack_130 = (undefined *)FUN_1428a3960(&uStack_e8,&uStack_1a8);
      }
      param_1[1] = puStack_130;
      *param_1 = 0x8000000000000000;
      *puStack_50 = 0;
    }
    else {
      puStack_58 = puStack_100;
      puStack_60 = puStack_f8;
      uStack_3a = 0;
      FUN_140d56700(&uStack_e8);
      uStack_188 = (undefined4)uStack_c8;
      uStack_184 = uStack_c8._4_4_;
      uStack_180 = (undefined4)uStack_c0;
      uStack_17c = uStack_c0._4_4_;
      uStack_198 = (undefined4)uStack_d8;
      uStack_194 = uStack_d8._4_4_;
      uStack_190 = (undefined4)uStack_d0;
      uStack_18c = uStack_d0._4_4_;
      uStack_1a8 = uStack_e8;
      uStack_1a0 = (longlong)uStack_e0;
      uStack_d8 = (undefined *)CONCAT44(uStack_10c,iStack_110);
      uStack_d0 = (undefined *)CONCAT44(uStack_104,uStack_108);
      uStack_c8 = puStack_58;
      uStack_c0 = puStack_60;
      uStack_b8 = uStack_f0;
      uStack_e8 = &UNK_1429cb968;
      uStack_e0 = (undefined *)0x18;
      uStack_3a = 1;
      uVar4 = FUN_1428a2bc0(&uStack_e8,&uStack_1a8);
      param_1[1] = uVar4;
      *param_1 = 0x8000000000000000;
      *puStack_50 = 0;
    }
    if (puStack_48 != (undefined *)0x0) {
      func_0x000140613c20(puStack_50,puStack_48,1);
    }
  }
  return param_1;
}

