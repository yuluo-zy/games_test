
void FUN_140bebdf0(undefined8 *param_1,undefined8 param_2)

{
  longlong lVar1;
  code *pcVar2;
  undefined8 uVar3;
  ulonglong *puVar4;
  undefined *puVar5;
  int iVar6;
  undefined8 *puVar7;
  longlong lVar8;
  uint uVar9;
  undefined8 uVar10;
  int *piVar11;
  longlong lVar12;
  longlong alStack_218 [18];
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 *puStack_158;
  longlong lStack_150;
  undefined8 uStack_148;
  longlong lStack_140;
  longlong lStack_138;
  undefined *puStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_98;
  longlong lStack_90;
  longlong lStack_88;
  longlong lStack_80;
  undefined *puStack_78;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined1 uStack_53;
  undefined1 uStack_52;
  undefined1 uStack_51;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  puStack_78 = (undefined *)param_1[10];
  lStack_88 = param_1[8];
  lStack_80 = param_1[9];
  param_1[8] = 0;
  param_1[9] = 8;
  param_1[10] = 0;
  uStack_178 = param_1[2];
  uStack_188 = *(undefined4 *)param_1;
  uStack_184 = *(undefined4 *)((longlong)param_1 + 4);
  uStack_180 = *(undefined4 *)(param_1 + 1);
  uStack_17c = *(undefined4 *)((longlong)param_1 + 0xc);
  *param_1 = 0;
  lVar8 = param_1[3];
  uVar3 = param_1[4];
  param_1[3] = 0;
  func_0x000142923c00(alStack_218,param_1 + 0xb,0x90);
  puVar5 = puStack_78;
  lVar1 = lStack_80;
  param_1[0xb] = 0x8000000000000000;
  lVar12 = (longlong)puStack_78 * 0x58 + lStack_80;
  piVar11 = (int *)(lVar12 + -0x58);
  if (piVar11 == (int *)0x0 || puStack_78 == (undefined *)0x0) {
    if (lVar8 == 0) {
      if (!SBORROW8(0,alStack_218[0])) {
        uStack_52 = 1;
        FUN_140bdf860(alStack_218);
      }
      lStack_90 = lVar1;
      FUN_140be14b0(lVar1,puVar5);
      if (lStack_88 != 0) {
        func_0x000140613c20(lStack_90,lStack_88 * 0x58,8);
      }
      return;
    }
    uStack_51 = 1;
    FUN_1428d9430(&UNK_142b14890,0x2c,&UNK_142b148c0);
  }
  else {
    lVar1 = param_1[0x2f];
    uStack_168 = (int *)CONCAT71(uStack_168._1_7_,0x18);
    uStack_51 = 1;
    FUN_1418c1380(0x143634e40,&uStack_168,1);
    if (*piVar11 == 1) {
      iVar6 = (**(code **)(lVar1 + 0x2c8))(*(undefined8 *)(lVar12 + -0x10));
      if (iVar6 == 0) {
        piVar11[0] = 2;
        piVar11[1] = 0;
        uStack_98 = param_2;
        func_0x000142923c00(&uStack_128,alStack_218,0x90);
        lStack_140 = lStack_88;
        lStack_138 = lStack_80;
        puStack_130 = puStack_78;
        uStack_168 = (int *)CONCAT44(uStack_184,uStack_188);
        uStack_160 = (code *)CONCAT44(uStack_17c,uStack_180);
        puStack_158 = (undefined8 *)uStack_178;
        uStack_51 = 0;
        uVar10 = param_2;
        lStack_150 = lVar8;
        uStack_148 = uVar3;
        FUN_140c04e20(&uStack_68,param_2,0);
        uVar9 = (uint)uVar10;
        uStack_168 = uStack_68;
        uStack_160 = uStack_60;
        *(undefined4 *)(param_1 + 0x21) = (undefined4)uStack_68;
        *(undefined4 *)((longlong)param_1 + 0x10c) = uStack_68._4_4_;
        *(undefined4 *)(param_1 + 0x22) = (undefined4)uStack_60;
        *(undefined4 *)((longlong)param_1 + 0x114) = uStack_60._4_4_;
        puVar4 = puRam000000014362c3a8;
        if (*(char *)(param_1 + 0x31) == '\0') {
          return;
        }
        if (2 < *puRam000000014362c3a8) {
          uStack_51 = 0;
          puVar7 = (undefined8 *)func_0x00014067dd80(&UNK_142b147f8);
          uStack_148 = *puVar7;
          lStack_140 = puVar7[1];
          uStack_11c = *(undefined4 *)(puVar7 + 2);
          lStack_138 = 3;
          puStack_130 = &UNK_142b14810;
          uStack_128 = 0x16;
          puStack_118 = &UNK_142b147e8;
          uStack_110 = 1;
          uStack_108 = 8;
          uStack_168 = (int *)0x0;
          uStack_100 = 0;
          uStack_f8 = 0;
          uStack_160 = (code *)&UNK_142b14810;
          puStack_158 = (undefined8 *)0x16;
          lStack_150 = 0;
          uStack_120 = 1;
          uStack_51 = 0;
          uVar9 = (uint)&uStack_168;
          func_0x000140d4a330(&uStack_53);
        }
        uStack_51 = 0;
        uStack_68 = (int *)FUN_140d635e0();
        uStack_60 = (code *)CONCAT44(uStack_60._4_4_,uVar9);
        uStack_51 = 0;
        FUN_140bf8980(param_2);
        uStack_51 = 0;
        lVar8 = FUN_140d63910(&uStack_68);
        if (20000000 < uVar9 || lVar8 != 0) {
          if (2 < *puVar4) {
            uStack_51 = 0;
            puVar7 = (undefined8 *)func_0x00014067dd80(&UNK_142b14878);
            uStack_148 = *puVar7;
            lStack_140 = puVar7[1];
            uStack_11c = *(undefined4 *)(puVar7 + 2);
            lStack_138 = 3;
            puStack_130 = &UNK_142b14810;
            uStack_128 = 0x16;
            puStack_118 = &UNK_142b14868;
            uStack_110 = 1;
            uStack_108 = 8;
            uStack_168 = (int *)0x0;
            uStack_100 = 0;
            uStack_f8 = 0;
            uStack_160 = (code *)&UNK_142b14810;
            puStack_158 = (undefined8 *)0x16;
            lStack_150 = 0;
            uStack_120 = 1;
            uStack_51 = 0;
            func_0x000140d4a330(&uStack_53,&uStack_168);
          }
          uStack_51 = 0;
          FUN_140d55980(2,0);
        }
        uStack_51 = 0;
        FUN_140bf8980(param_2);
        return;
      }
      uStack_168 = (int *)CONCAT44(uStack_168._4_4_,iVar6);
      uStack_51 = 1;
      FUN_1428d9760(&UNK_142b14d20,0x12,&uStack_168,&UNK_142b14b30,&UNK_142b14d38);
    }
    else {
      uStack_60 = FUN_140bed620;
      uStack_168 = (int *)&UNK_142b14d78;
      uStack_160 = (code *)0x1;
      uStack_148 = 0;
      puStack_158 = &uStack_68;
      lStack_150 = 1;
      uStack_51 = 1;
      uStack_68 = piVar11;
      FUN_1428d9390(&uStack_168,&UNK_142b14d88);
    }
  }
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

