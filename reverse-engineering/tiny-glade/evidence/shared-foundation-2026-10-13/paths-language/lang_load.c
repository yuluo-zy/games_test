
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong ******* FUN_140b9c4a0(longlong ******param_1,longlong *******param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 *puVar4;
  longlong *******ppppppplVar5;
  undefined4 *puVar6;
  longlong ******pppppplVar7;
  undefined8 uVar8;
  longlong *******ppppppplVar9;
  longlong *****ppppplStack_218;
  undefined8 uStack_210;
  longlong ******pppppplStack_208;
  longlong *******ppppppplStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined *puStack_1e8;
  undefined *puStack_1e0;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  longlong *******ppppppplStack_1c0;
  longlong *******ppppppplStack_1b8;
  longlong *******ppppppplStack_1b0;
  undefined *puStack_1a8;
  undefined *puStack_1a0;
  undefined *puStack_198;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  longlong ******pppppplStack_180;
  undefined *puStack_178;
  longlong *******ppppppplStack_170;
  undefined8 uStack_168;
  longlong *******ppppppplStack_150;
  longlong *******ppppppplStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined *puStack_130;
  undefined *puStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  longlong ******pppppplStack_110;
  longlong *******ppppppplStack_108;
  undefined8 uStack_100;
  longlong *******ppppppplStack_f8;
  longlong *******ppppppplStack_f0;
  longlong *******ppppppplStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined *puStack_80;
  undefined8 uStack_78;
  longlong *******ppppppplStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *******ppppppplStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  ppppppplStack_f0 = (longlong *******)&UNK_140109e90;
  uStack_d0 = (longlong *******)&UNK_142afd650;
  uStack_c8 = (longlong *******)0x2;
  uStack_c0 = (longlong *******)&ppppppplStack_f8;
  uStack_b8 = (longlong *******)0x1;
  uStack_b0._0_4_ = 0;
  uStack_b0._4_4_ = 0;
  pppppplStack_208 = param_1;
  ppppppplStack_200 = param_2;
  pppppplStack_110 = param_1;
  ppppppplStack_108 = param_2;
  ppppppplStack_f8 = &pppppplStack_110;
  FUN_140d79f90(&ppppppplStack_150,&uStack_d0);
  ppppppplStack_50 = ppppppplStack_148;
  FUN_140d62810(&ppppppplStack_170,&UNK_142afd640,0xc,ppppppplStack_148,uStack_140);
  ppppppplVar5 = ppppppplStack_150;
  if (ppppppplStack_150 != (longlong *******)0x0) {
    func_0x000140613c20(ppppppplStack_50,ppppppplStack_150,1);
  }
  uVar2 = FUN_140d76ed0(&ppppppplStack_170);
  if ((longlong)param_2 < 0) {
    uVar8 = 0;
LAB_140b9cd8e:
    FUN_1428d8fe3(uVar8,param_2,&UNK_142afd628);
  }
  else {
    if (param_2 == (longlong *******)0x0) {
      lVar3 = 1;
    }
    else {
      uVar8 = 1;
      lVar3 = func_0x000140613c10(param_2,1);
      if (lVar3 == 0) goto LAB_140b9cd8e;
    }
    func_0x000142923c00(lVar3,param_1,param_2);
    uStack_d0 = param_2;
    uStack_c8 = (longlong *******)lVar3;
    uStack_c0 = param_2;
    FUN_141848770(&ppppppplStack_1c0,uVar2,ppppppplVar5,&uStack_d0);
    if (ppppppplStack_1c0 == (longlong *******)0x0) {
      ppppppplStack_d8 = ppppppplStack_1b8;
      if (1 < *_DAT_14362c3a8) {
        ppppppplStack_150 = &pppppplStack_208;
        ppppppplStack_148 = (longlong *******)&UNK_1405338a0;
        uStack_140 = (longlong *******)&ppppppplStack_d8;
        uStack_138 = &UNK_140d4cc10;
        puStack_130 = &UNK_142afd6a0;
        puStack_128 = &UNK_1405338a0;
        puVar4 = (undefined8 *)func_0x00014067dd80(&UNK_142afd718);
        uStack_b0 = *puVar4;
        uStack_a8 = *(undefined4 *)(puVar4 + 1);
        uStack_a4 = *(undefined4 *)((longlong)puVar4 + 0xc);
        uStack_84 = *(undefined4 *)(puVar4 + 2);
        uStack_a0 = (undefined *)0x2;
        uStack_98 = &UNK_142afd730;
        uStack_90 = 8;
        puStack_80 = &UNK_142afd6e8;
        uStack_78 = 3;
        uStack_68 = 3;
        uStack_60 = 0;
        uStack_d0 = (longlong *******)0x0;
        uStack_c8 = (longlong *******)&UNK_142afd730;
        uStack_c0 = (longlong *******)0x8;
        uStack_b8 = (longlong *******)0x0;
        uStack_88 = 1;
        ppppppplStack_70 = (longlong *******)&ppppppplStack_150;
        func_0x000140d4a330(&ppppppplStack_f8,&uStack_d0);
      }
      ppppplStack_218 = (longlong *****)&UNK_142afd69a;
      uStack_210 = 5;
      pppppplStack_180 = &ppppplStack_218;
      puStack_178 = &UNK_140109e90;
      uStack_d0 = (longlong *******)&UNK_142afd650;
      uStack_c8 = (longlong *******)0x2;
      uStack_c0 = &pppppplStack_180;
      uStack_b8 = (longlong *******)0x1;
      uStack_b0._0_4_ = 0;
      uStack_b0._4_4_ = 0;
      FUN_140d79f90(&pppppplStack_110,&uStack_d0);
      ppppppplStack_50 = ppppppplStack_108;
      FUN_140d62810(&ppppppplStack_f8,&UNK_142afd640,0xc,ppppppplStack_108,uStack_100);
      pppppplVar7 = pppppplStack_110;
      if (pppppplStack_110 != (longlong ******)0x0) {
        func_0x000140613c20(ppppppplStack_50,pppppplStack_110,1);
      }
      uVar2 = FUN_140d76ed0(&ppppppplStack_f8);
      ppppppplVar5 = (longlong *******)func_0x000140613c10(5,1);
      if (ppppppplVar5 == (longlong *******)0x0) {
        FUN_1428d8fe3(1,5,&UNK_142afd628);
        goto LAB_140b9ce76;
      }
      *(undefined1 *)((longlong)ppppppplVar5 + 4) = 0x53;
      *(undefined4 *)ppppppplVar5 = 0x552d6e65;
      pppppplStack_110 = (longlong ******)0x5;
      uStack_100 = 5;
      ppppppplStack_108 = ppppppplVar5;
      FUN_141848770(&uStack_d0,uVar2,pppppplVar7,&pppppplStack_110);
      FUN_141848b50(&ppppppplStack_150,&uStack_d0);
      ppppppplVar9 = ppppppplStack_148;
      ppppppplVar5 = ppppppplStack_150;
      if (ppppppplStack_150 == (longlong *******)0x0) {
        if (ppppppplStack_f8 != (longlong *******)0x0) {
          func_0x000140613c20(ppppppplStack_f0,ppppppplStack_f8,1);
        }
        func_0x000140d4cc30(&ppppppplStack_d8);
        if (ppppppplStack_170 == (longlong *******)0x0) {
          return ppppppplVar9;
        }
        func_0x000140613c20(uStack_168,ppppppplStack_170,1);
        return ppppppplVar9;
      }
      uStack_1d8 = uStack_120;
      uStack_1d4 = uStack_11c;
      uStack_1d0 = uStack_118;
      uStack_1cc = uStack_114;
      puStack_1e8 = puStack_130;
      puStack_1e0 = puStack_128;
      uStack_1f8 = uStack_140;
      uStack_1f0 = uStack_138;
      if (ppppppplStack_f8 != (longlong *******)0x0) {
        func_0x000140613c20(ppppppplStack_f0,ppppppplStack_f8,1);
      }
      func_0x000140d4cc30(&ppppppplStack_d8);
    }
    else {
      uStack_1f8 = ppppppplStack_1b0;
      uStack_1f0 = puStack_1a8;
      puStack_1e8 = puStack_1a0;
      puStack_1e0 = puStack_198;
      uStack_1d8 = uStack_190;
      uStack_1d4 = uStack_18c;
      uStack_1d0 = uStack_188;
      uStack_1cc = uStack_184;
      ppppppplVar5 = ppppppplStack_1c0;
      ppppppplVar9 = ppppppplStack_1b8;
    }
    if (ppppppplStack_170 != (longlong *******)0x0) {
      func_0x000140613c20(uStack_168,ppppppplStack_170,1);
    }
    uStack_b0._0_4_ = SUB84(uStack_1f8,0);
    uStack_b0._4_4_ = uStack_1f8._4_4_;
    uStack_a8 = (undefined4)uStack_1f0;
    uStack_a4 = uStack_1f0._4_4_;
    uStack_a0 = puStack_1e8;
    uStack_98 = puStack_1e0;
    uStack_90 = CONCAT44(uStack_1d4,uStack_1d8);
    uStack_88 = uStack_1d0;
    uStack_84 = uStack_1cc;
    uStack_d0 = (longlong *******)0x1;
    uStack_c8 = (longlong *******)0x1;
    uStack_c0 = ppppppplVar5;
    uStack_b8 = ppppppplVar9;
    puVar6 = (undefined4 *)func_0x000140613c10(0x50,8);
    if (puVar6 == (undefined4 *)0x0) {
      FUN_1428d9000(8,0x50);
      goto LAB_140b9ce76;
    }
    *(undefined8 *)(puVar6 + 0x10) = uStack_90;
    *(ulonglong *)(puVar6 + 0x12) = CONCAT44(uStack_84,uStack_88);
    puVar6[0xc] = (undefined4)uStack_a0;
    puVar6[0xd] = uStack_a0._4_4_;
    puVar6[0xe] = (undefined4)uStack_98;
    puVar6[0xf] = uStack_98._4_4_;
    puVar6[8] = (undefined4)uStack_b0;
    puVar6[9] = uStack_b0._4_4_;
    puVar6[10] = uStack_a8;
    puVar6[0xb] = uStack_a4;
    puVar6[4] = (undefined4)uStack_c0;
    puVar6[5] = uStack_c0._4_4_;
    puVar6[6] = (undefined4)uStack_b8;
    puVar6[7] = uStack_b8._4_4_;
    *puVar6 = (undefined4)uStack_d0;
    puVar6[1] = uStack_d0._4_4_;
    puVar6[2] = (undefined4)uStack_c8;
    puVar6[3] = uStack_c8._4_4_;
    ppppppplVar5 = _DAT_143634da8;
    LOCK();
    UNLOCK();
    pppppplStack_110 = (longlong ******)&DAT_143634db0;
    ppppppplStack_170 = (longlong *******)&DAT_143634da8;
    ppppppplStack_f8 = _DAT_143634da8;
    ppppppplStack_1c0 = (longlong *******)&DAT_143634da8;
    uStack_d0 = (longlong *******)&ppppppplStack_f8;
    _DAT_143634da8 = (longlong *******)(puVar6 + 4);
    ppppppplStack_150 = &pppppplStack_110;
    ppppppplStack_148 = (longlong *******)&ppppppplStack_170;
    uStack_c8 = (longlong *******)&ppppppplStack_1c0;
    uStack_c0 = (longlong *******)&ppppppplStack_150;
    FUN_1418490e0(&uStack_d0);
    uStack_d0 = ppppppplVar5 + -2;
    if (ppppppplVar5 != (longlong *******)0x0) {
      LOCK();
      *uStack_d0 = (longlong ******)((longlong)*uStack_d0 + -1);
      UNLOCK();
      ppppppplVar5 = uStack_d0;
      if (*uStack_d0 == (longlong ******)0x0) {
        FUN_1418491f0(&uStack_d0);
        ppppppplVar5 = uStack_d0;
      }
    }
    uStack_d0 = ppppppplVar5;
    FUN_140d57930(&uStack_d0,&UNK_142afd738,0x18);
    if (!SBORROW8(0,(longlong)uStack_d0)) {
      if (uStack_d0 == (longlong *******)0x8000000000000001) {
        ppppppplVar5 = uStack_c8;
        ppppppplVar9 = uStack_c0;
        pppppplVar7 = (longlong ******)0x8000000000000001;
        goto joined_r0x000140b9cd1a;
      }
      if (uStack_d0 != (longlong *******)0x0) {
        func_0x000140613c20(uStack_c8,uStack_d0,1);
      }
    }
    ppppppplStack_170 = (longlong *******)&UNK_142afd69a;
    uStack_168 = 5;
    ppppppplStack_1b8 = (longlong *******)&UNK_140109e90;
    uStack_d0 = (longlong *******)&UNK_142afd650;
    uStack_c8 = (longlong *******)0x2;
    uStack_b8 = (longlong *******)0x1;
    uStack_b0._0_4_ = 0;
    uStack_b0._4_4_ = 0;
    ppppppplStack_1c0 = (longlong *******)&ppppppplStack_170;
    uStack_c0 = (longlong *******)&ppppppplStack_1c0;
    FUN_140d79f90(&ppppppplStack_150,&uStack_d0);
    ppppppplStack_50 = ppppppplStack_148;
    FUN_140d62810(&ppppppplStack_f8,&UNK_142afd640,0xc,ppppppplStack_148,uStack_140);
    ppppppplVar5 = ppppppplStack_150;
    if (ppppppplStack_150 != (longlong *******)0x0) {
      func_0x000140613c20(ppppppplStack_50,ppppppplStack_150,1);
    }
    uVar2 = FUN_140d76ed0(&ppppppplStack_f8);
    puVar6 = (undefined4 *)func_0x000140613c10(5,1);
    if (puVar6 == (undefined4 *)0x0) {
      FUN_1428d8fe3(1,5,&UNK_142afd628);
    }
    else {
      *(undefined1 *)(puVar6 + 1) = 0x53;
      *puVar6 = 0x552d6e65;
      uStack_d0 = (longlong *******)0x5;
      uStack_c0 = (longlong *******)0x5;
      uStack_c8 = (longlong *******)puVar6;
      FUN_141848770(&ppppppplStack_150,uVar2,ppppppplVar5,&uStack_d0);
      if (ppppppplStack_150 == (longlong *******)0x0) {
        ppppppplStack_170 = ppppppplStack_148;
        ppppppplStack_1c0 = (longlong *******)&UNK_142afd6a0;
        ppppppplStack_1b8 = (longlong *******)&UNK_1405338a0;
        puStack_1a8 = &UNK_140d4cc20;
        uStack_d0 = (longlong *******)&UNK_142afd770;
        uStack_c8 = (longlong *******)0x2;
        uStack_b0._0_4_ = 0;
        uStack_b0._4_4_ = 0;
        uStack_b8 = (longlong *******)0x2;
        ppppppplStack_1b0 = (longlong *******)&ppppppplStack_170;
        uStack_c0 = (longlong *******)&ppppppplStack_1c0;
        FUN_1428d9390(&uStack_d0,&UNK_142afd790);
      }
      else {
        uStack_90 = CONCAT44(uStack_11c,uStack_120);
        uStack_88 = uStack_118;
        uStack_84 = uStack_114;
        uStack_a0 = puStack_130;
        uStack_98 = puStack_128;
        uStack_b0._0_4_ = SUB84(uStack_140,0);
        uStack_b0._4_4_ = uStack_140._4_4_;
        uStack_a8 = (undefined4)uStack_138;
        uStack_a4 = uStack_138._4_4_;
        uStack_c0 = ppppppplStack_150;
        uStack_b8 = ppppppplStack_148;
        uStack_d0 = (longlong *******)0x1;
        uStack_c8 = (longlong *******)0x1;
        puVar6 = (undefined4 *)func_0x000140613c10(0x50,8);
        if (puVar6 != (undefined4 *)0x0) {
          *(undefined8 *)(puVar6 + 0x10) = uStack_90;
          *(ulonglong *)(puVar6 + 0x12) = CONCAT44(uStack_84,uStack_88);
          puVar6[0xc] = (undefined4)uStack_a0;
          puVar6[0xd] = uStack_a0._4_4_;
          puVar6[0xe] = (undefined4)uStack_98;
          puVar6[0xf] = uStack_98._4_4_;
          puVar6[8] = (undefined4)uStack_b0;
          puVar6[9] = uStack_b0._4_4_;
          puVar6[10] = uStack_a8;
          puVar6[0xb] = uStack_a4;
          puVar6[4] = (undefined4)uStack_c0;
          puVar6[5] = uStack_c0._4_4_;
          puVar6[6] = (undefined4)uStack_b8;
          puVar6[7] = uStack_b8._4_4_;
          *puVar6 = (undefined4)uStack_d0;
          puVar6[1] = uStack_d0._4_4_;
          puVar6[2] = (undefined4)uStack_c8;
          puVar6[3] = uStack_c8._4_4_;
          pppppplVar7 = _DAT_143634db0;
          LOCK();
          UNLOCK();
          pppppplStack_180 = (longlong ******)&DAT_143634db8;
          pppppplStack_110 = (longlong ******)&DAT_143634db0;
          ppppppplStack_170 = (longlong *******)_DAT_143634db0;
          ppppppplStack_1c0 = (longlong *******)&DAT_143634db0;
          ppppppplStack_150 = &pppppplStack_180;
          _DAT_143634db0 = (longlong ******)(puVar6 + 4);
          ppppppplStack_148 = &pppppplStack_110;
          uStack_d0 = (longlong *******)&ppppppplStack_170;
          uStack_c8 = (longlong *******)&ppppppplStack_1c0;
          uStack_c0 = (longlong *******)&ppppppplStack_150;
          FUN_1418490e0(&uStack_d0);
          uStack_d0 = (longlong *******)(pppppplVar7 + -2);
          ppppppplVar5 = ppppppplStack_f8;
          ppppppplVar9 = ppppppplStack_f0;
          if (pppppplVar7 != (longlong ******)0x0) {
            LOCK();
            *uStack_d0 = (longlong ******)((longlong)*uStack_d0 + -1);
            UNLOCK();
            pppppplVar7 = (longlong ******)uStack_d0;
            if (*uStack_d0 == (longlong ******)0x0) {
              FUN_1418491f0(&uStack_d0);
              ppppppplVar5 = ppppppplStack_f8;
              ppppppplVar9 = ppppppplStack_f0;
              pppppplVar7 = (longlong ******)uStack_d0;
            }
          }
joined_r0x000140b9cd1a:
          uStack_d0 = (longlong *******)pppppplVar7;
          if (ppppppplVar5 != (longlong *******)0x0) {
            func_0x000140613c20(ppppppplVar9,ppppppplVar5,1);
          }
          return (longlong *******)0x0;
        }
        FUN_1428d9000(8,0x50);
      }
    }
  }
LAB_140b9ce76:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

