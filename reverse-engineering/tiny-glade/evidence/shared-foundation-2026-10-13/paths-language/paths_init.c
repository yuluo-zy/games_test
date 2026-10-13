
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

code * FUN_140b9df10(undefined8 param_1,undefined8 *param_2)

{
  longlong lVar1;
  code *pcVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  code *pcVar7;
  undefined8 uVar8;
  code *pcVar9;
  undefined8 *puVar10;
  code *pcVar11;
  undefined8 uVar12;
  bool bVar13;
  code *pcStack_190;
  code *pcStack_188;
  undefined8 uStack_180;
  undefined8 *puStack_178;
  undefined8 uStack_170;
  code *pcStack_168;
  undefined8 *puStack_160;
  undefined8 *puStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 *puStack_140;
  undefined8 *puStack_138;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined8 *puStack_100;
  code *pcStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 *puStack_e0;
  code *pcStack_d8;
  undefined8 *puStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  code *pcStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 *puStack_58;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_170 = param_1;
  if (param_2 == (undefined8 *)0x0) {
    FUN_140b9dc80(&uStack_88);
    if (SBORROW8(0,(longlong)uStack_88)) {
      return uStack_80;
    }
    uStack_f0 = (undefined4)uStack_78;
    uStack_ec = uStack_78._4_4_;
    uStack_e8 = (undefined4)uStack_70;
    uStack_e4 = uStack_70._4_4_;
    puStack_100 = uStack_88;
    pcStack_f8 = uStack_80;
  }
  else {
    FUN_140d61b00(&uStack_88);
    uVar8 = FUN_140d76ed0(&uStack_88);
    FUN_140d62810(&puStack_100,uVar8,param_2,&UNK_142afd9d6,10);
    param_2 = uStack_88;
    if (uStack_88 != (undefined8 *)0x0) {
      func_0x000140613c20(uStack_80,uStack_88,1);
      param_2 = uStack_88;
    }
  }
  uStack_4b = 1;
  uVar8 = FUN_140d76ed0(&puStack_100);
  uStack_c8 = &uStack_170;
  uStack_c0 = FUN_140d8d6c0;
  uStack_88._0_4_ = 0x42afd9e8;
  uStack_88._4_4_ = 1;
  uStack_80._0_4_ = 1;
  uStack_80._4_4_ = 0;
  uStack_78 = (undefined8 **)&uStack_c8;
  uStack_70._0_4_ = 1;
  uStack_70._4_4_ = 0;
  uStack_68 = 0;
  uStack_4b = 1;
  FUN_140d79f90(&pcStack_190,&uStack_88);
  pcStack_90 = pcStack_190;
  pcStack_d8 = pcStack_188;
  FUN_140d62810(&uStack_120,uVar8,param_2,pcStack_188,uStack_180);
  pcVar11 = pcStack_90;
  if (pcStack_90 != (code *)0x0) {
    func_0x000140613c20(pcStack_d8,pcStack_90,1);
  }
  uStack_49 = 1;
  uStack_4a = 1;
  uVar8 = FUN_140d76ed0(&uStack_120);
  uStack_49 = 1;
  uStack_4a = 1;
  FUN_140d62810(&uStack_88,uVar8,pcVar11,&UNK_142afd9f8,5);
  uStack_49 = 1;
  uStack_4a = 1;
  puVar10 = &uStack_88;
  FUN_14184a0a0(&uStack_c8,puVar10);
  pcVar11 = uStack_c0;
  puVar3 = uStack_c8;
  if (uStack_c8 == (undefined8 *)0x0) {
LAB_140b9e436:
    lVar1 = CONCAT44(uStack_11c,uStack_120);
    pcVar9 = uStack_c0;
  }
  else {
    uStack_49 = 1;
    uStack_4a = 1;
    uVar8 = FUN_140d76ed0(&uStack_120);
    uStack_49 = 1;
    uStack_4a = 1;
    FUN_140d62810(&uStack_88,uVar8,puVar10,&UNK_142afd9fd,10);
    uStack_49 = 1;
    uStack_4a = 1;
    puVar10 = &uStack_88;
    FUN_14184a0a0(&uStack_c8,puVar10);
    pcVar7 = uStack_c0;
    puVar4 = uStack_c8;
    if (uStack_c8 != (undefined8 *)0x0) {
      uStack_49 = 1;
      uStack_4a = 1;
      uVar8 = FUN_140d76ed0(&uStack_120);
      uStack_49 = 1;
      uStack_4a = 1;
      FUN_140d62810(&uStack_88,uVar8,puVar10,&UNK_142afda07,3);
      uStack_49 = 1;
      uStack_4a = 1;
      puVar10 = &uStack_88;
      FUN_14184a0a0(&uStack_c8,puVar10);
      puVar5 = uStack_c8;
      if (uStack_c8 != (undefined8 *)0x0) {
        pcStack_90 = uStack_c0;
        uStack_49 = 1;
        uStack_4a = 1;
        uVar8 = FUN_140d76ed0(&uStack_120);
        uStack_49 = 1;
        uStack_4a = 1;
        FUN_140d62810(&uStack_88,uVar8,puVar10,&UNK_142afda0a,0xd);
        uStack_49 = 1;
        uStack_4a = 1;
        puVar10 = &uStack_88;
        FUN_14184a0a0(&uStack_c8,puVar10);
        puVar6 = uStack_c8;
        if (uStack_c8 != (undefined8 *)0x0) {
          pcStack_d8 = uStack_c0;
          uStack_49 = 1;
          uStack_4a = 1;
          uVar8 = FUN_140d76ed0(&uStack_120);
          uStack_49 = 1;
          uStack_4a = 1;
          FUN_140d62810(&uStack_88,uVar8,puVar10,&UNK_142afda17,0xc);
          uStack_49 = 1;
          uStack_4a = 1;
          FUN_14184a0a0(&uStack_c8,&uStack_88);
          if (uStack_c8 != (undefined8 *)0x0) {
            puStack_160 = uStack_c8;
            pcStack_168 = uStack_c0;
            uStack_78 = (undefined8 **)CONCAT44(uStack_10c,uStack_110);
            uStack_70._0_4_ = uStack_108;
            uStack_70._4_4_ = uStack_104;
            uStack_88._0_4_ = uStack_120;
            uStack_88._4_4_ = uStack_11c;
            uStack_80._0_4_ = uStack_118;
            uStack_80._4_4_ = uStack_114;
            uStack_49 = 0;
            uStack_4a = 1;
            FUN_14184a0a0(&uStack_c8,&uStack_88);
            pcVar9 = uStack_c0;
            if (uStack_c8 != (undefined8 *)0x0) {
              puStack_e0 = uStack_c8;
              uStack_49 = 0;
              uStack_4a = 1;
              FUN_140d57650(&puStack_140);
              puStack_158 = puVar6;
              puStack_58 = puStack_140;
              if (!SBORROW8(0,(longlong)puStack_140)) {
                puStack_d0 = puStack_138;
                uVar8 = 8;
                puVar10 = (undefined8 *)func_0x000140613c10(0x20);
                if (puVar10 == (undefined8 *)0x0) {
                  FUN_1428d9000(8,0x20);
                }
                else {
                  *puVar10 = puStack_58;
                  puVar10[1] = puStack_d0;
                  *(undefined4 *)(puVar10 + 2) = uStack_130;
                  *(undefined4 *)((longlong)puVar10 + 0x14) = uStack_12c;
                  *(undefined4 *)(puVar10 + 3) = uStack_128;
                  *(undefined4 *)((longlong)puVar10 + 0x1c) = uStack_124;
                  uStack_49 = 0;
                  uStack_4a = 1;
                  puStack_d0 = (undefined8 *)FUN_140d76ed0(puVar10);
                  pcVar2 = pcStack_f8;
                  puStack_58 = puStack_100;
                  uStack_88._0_4_ = uStack_f0;
                  uStack_88._4_4_ = uStack_ec;
                  uStack_80._0_4_ = uStack_e8;
                  uStack_80._4_4_ = uStack_e4;
                  uVar12 = 8;
                  uStack_150 = uVar8;
                  puVar10 = (undefined8 *)func_0x000140613c10(0x20);
                  if (puVar10 == (undefined8 *)0x0) {
                    pcStack_90 = pcVar2;
                    FUN_1428d9000(8,0x20);
                  }
                  else {
                    *puVar10 = puStack_58;
                    puVar10[1] = pcVar2;
                    *(undefined4 *)(puVar10 + 2) = (undefined4)uStack_88;
                    *(undefined4 *)((longlong)puVar10 + 0x14) = uStack_88._4_4_;
                    *(undefined4 *)(puVar10 + 3) = (undefined4)uStack_80;
                    *(undefined4 *)((longlong)puVar10 + 0x1c) = uStack_80._4_4_;
                    uStack_49 = 0;
                    uStack_4a = 0;
                    puStack_58 = (undefined8 *)FUN_140d76ed0(puVar10);
                    uStack_49 = 0;
                    uStack_4a = 0;
                    uStack_148 = uVar12;
                    FUN_140d77340(&uStack_c8,puStack_e0,pcVar9);
                    puStack_138 = (undefined8 *)&UNK_14003bad0;
                    uStack_88 = (undefined8 *)&UNK_142afda38;
                    uStack_80 = (code *)0x2;
                    uStack_68 = 0;
                    uStack_78 = &puStack_140;
                    uStack_70 = 1;
                    puStack_140 = &uStack_c8;
                    FUN_140d5c990(&uStack_88);
                    if ((!SBORROW8(0,(longlong)uStack_c8)) && (uStack_c8 != (undefined8 *)0x0)) {
                      func_0x000140613c20(uStack_c0,uStack_c8,1);
                    }
                    uStack_49 = 0;
                    uStack_4a = 0;
                    FUN_140d77340(&uStack_c8,puVar3,pcVar11);
                    puStack_138 = (undefined8 *)&UNK_14003bad0;
                    uStack_88 = (undefined8 *)&UNK_142afda60;
                    uStack_80 = (code *)0x2;
                    uStack_68 = 0;
                    uStack_78 = &puStack_140;
                    uStack_70 = 1;
                    puStack_140 = &uStack_c8;
                    FUN_140d5c990(&uStack_88);
                    if ((!SBORROW8(0,(longlong)uStack_c8)) && (uStack_c8 != (undefined8 *)0x0)) {
                      func_0x000140613c20(uStack_c0,uStack_c8,1);
                    }
                    bVar13 = _DAT_143634db8 == (undefined8 *)0x0;
                    _DAT_143634db8 = puVar3;
                    _DAT_143634dc0 = pcVar11;
                    _DAT_143634dc8 = puVar4;
                    _DAT_143634dd0 = pcVar7;
                    _DAT_143634dd8 = puStack_e0;
                    _DAT_143634de0 = pcVar9;
                    _DAT_143634de8 = puVar5;
                    _DAT_143634df0 = pcStack_90;
                    _DAT_143634df8 = puStack_d0;
                    _DAT_143634e00 = uStack_150;
                    _DAT_143634e08 = puStack_158;
                    _DAT_143634e10 = pcStack_d8;
                    _DAT_143634e18 = puStack_160;
                    _DAT_143634e20 = pcStack_168;
                    _DAT_143634e28 = puStack_58;
                    _DAT_143634e30 = uStack_148;
                    if (bVar13) {
                      return (code *)0x0;
                    }
                    uStack_88._0_4_ = 0x42afdaa8;
                    uStack_88._4_4_ = 1;
                    uStack_80._0_4_ = 1;
                    uStack_80._4_4_ = 0;
                    uStack_78 = (undefined8 **)0x8;
                    uStack_70._0_4_ = 0;
                    uStack_70._4_4_ = 0;
                    uStack_68 = 0;
                    uStack_49 = 0;
                    uStack_4a = 0;
                    FUN_1428d9390(&uStack_88,&UNK_142afdab8);
                  }
                }
                    /* WARNING: Does not return */
                pcVar11 = (code *)invalidInstructionException();
                (*pcVar11)();
              }
              puStack_178 = puStack_138;
              uStack_4c = 1;
              FUN_140d56700(&uStack_88);
              uStack_a8 = (undefined4)uStack_68;
              uStack_a4 = uStack_68._4_4_;
              uStack_a0 = uStack_60;
              uStack_9c = uStack_5c;
              uStack_b8 = (undefined4)uStack_78;
              uStack_b4 = uStack_78._4_4_;
              uStack_b0 = (undefined4)uStack_70;
              uStack_ac = uStack_70._4_4_;
              uStack_c8 = (undefined8 *)CONCAT44(uStack_88._4_4_,(undefined4)uStack_88);
              uStack_c0 = (code *)CONCAT44(uStack_80._4_4_,(undefined4)uStack_80);
              uStack_88._0_4_ = 0x42afda23;
              uStack_88._4_4_ = 1;
              uStack_80._0_4_ = 0xd;
              uStack_80._4_4_ = 0;
              uStack_78 = (undefined8 **)puStack_138;
              uStack_4c = 0;
              pcVar9 = (code *)FUN_142903de0(&uStack_88,&uStack_c8);
            }
            goto LAB_140b9e44e;
          }
        }
      }
      goto LAB_140b9e436;
    }
    lVar1 = CONCAT44(uStack_11c,uStack_120);
    pcVar9 = uStack_c0;
  }
  uStack_c0 = pcVar9;
  if (lVar1 != 0) {
    func_0x000140613c20(CONCAT44(uStack_114,uStack_118),lVar1,1);
  }
LAB_140b9e44e:
  if (puStack_100 != (undefined8 *)0x0) {
    func_0x000140613c20(pcStack_f8,puStack_100,1);
  }
  return pcVar9;
}

