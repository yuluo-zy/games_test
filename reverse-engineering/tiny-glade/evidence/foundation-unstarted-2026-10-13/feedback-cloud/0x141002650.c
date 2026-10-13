
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong *
FUN_141002650(ulonglong *param_1,longlong param_2,ulonglong param_3,undefined8 param_4,
             undefined2 param_5,undefined2 param_6)

{
  code *pcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  char cVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined4 *puVar7;
  ulonglong uVar8;
  longlong *plVar9;
  ulonglong uVar10;
  undefined4 **ppuVar11;
  ulonglong *puVar12;
  longlong lVar13;
  undefined4 *puStack_278;
  code *pcStack_270;
  undefined4 *puStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  ulonglong uStack_248;
  longlong lStack_240;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  uint uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_208;
  undefined *puStack_1f8;
  undefined8 uStack_1f0;
  undefined8 *puStack_1e8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  longlong lStack_1b8;
  longlong lStack_1b0;
  ulonglong uStack_1a8;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  longlong lStack_180;
  longlong lStack_178;
  ulonglong uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 *puStack_148;
  ulonglong uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined1 uStack_118;
  undefined1 uStack_117;
  undefined2 uStack_116;
  undefined2 uStack_114;
  undefined2 uStack_112;
  undefined1 uStack_110;
  undefined1 uStack_10f;
  undefined1 uStack_10e;
  undefined1 uStack_10d;
  uint uStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  undefined8 *puStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 *puStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  ulonglong *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uStack_130 = (undefined *)0x8000000000000000;
  uStack_120 = (undefined4)param_4;
  uStack_11c = (undefined4)((ulonglong)param_4 >> 0x20);
  uStack_138 = 0x8000000000000001;
  uStack_88 = (undefined4 *)0x0;
  uStack_80 = (code *)0x8;
  uStack_78 = (undefined4 *)0x0;
  lVar13 = param_2;
  uStack_128 = param_3;
  FUN_140bc05d0(&uStack_1a0);
  uStack_248 = uStack_138;
  lStack_240 = (longlong)uStack_130;
  uStack_238 = (undefined4)uStack_128;
  uStack_234 = uStack_128._4_4_;
  uStack_230 = uStack_120;
  uStack_22c = uStack_11c;
  uStack_21c = uStack_10c;
  uStack_218 = (undefined4)uStack_108;
  uStack_214 = uStack_108._4_4_;
  uStack_210 = (undefined4)uStack_100;
  uStack_20c = uStack_100._4_4_;
  uStack_208 = uStack_f8;
  puStack_1f8 = puStack_e8;
  uStack_1f0 = uStack_e0;
  puStack_1e8 = puStack_d8;
  puStack_268 = uStack_78;
  puStack_278 = uStack_88;
  pcStack_270 = uStack_80;
  uStack_260 = 0;
  uStack_1d0 = 0;
  uStack_1c8 = 0;
  uStack_1c0 = 0;
  uStack_258 = 1;
  uStack_250 = 0;
  lStack_180 = param_2;
  uVar5 = FUN_140d76ed0(param_2);
  lVar6 = FUN_140d61e00(uVar5);
  if (lVar6 == 0) {
    FUN_1428d9310(&UNK_142bc8c08);
  }
  else {
    FUN_140d58e70(&uStack_138,lVar6,lVar13);
    puStack_98 = param_1;
    if ((int)uStack_138 != 1) {
      lStack_1b0 = (longlong)uStack_130;
      uStack_1a8 = uStack_128;
      lStack_b0 = 0;
      uStack_a8 = 1;
      uStack_a0 = 0;
      lVar13 = *(longlong *)(lStack_180 + 0x28);
      lStack_1b8 = lVar13 + *(longlong *)(lStack_180 + 0x30) * 0x18;
      lVar6 = lStack_180;
      if (lVar13 != lStack_1b8) {
LAB_14100283e:
        do {
          FUN_141ec53e0(&uStack_1a0,*(undefined8 *)(lVar13 + 8),*(undefined8 *)(lVar13 + 0x10),lVar6
                       );
          uStack_62 = 1;
          FUN_141ec5280(&uStack_170,lStack_1b0,uStack_1a8,lVar13);
          uStack_88 = (undefined4 *)0x0;
          uStack_80 = (code *)0x1;
          uStack_78 = (undefined4 *)0x0;
          uStack_138 = 0;
          uStack_128 = 0;
          uStack_118 = 0x20;
          uStack_117 = 0;
          uStack_116 = 0;
          uStack_114 = 0;
          uStack_112 = 0;
          uStack_110 = 3;
          uStack_108 = &uStack_88;
          uStack_100 = &UNK_142bc8a88;
          uStack_90 = uStack_168;
          cVar4 = FUN_140d886f0(uStack_168,uStack_160,&uStack_138);
          if (cVar4 != '\0') {
            FUN_1428d9760(&UNK_142bc8ab8,0x37,&uStack_63,&UNK_142bc8b58,&UNK_142bc8b40);
            goto LAB_141002eaf;
          }
          uStack_158 = (undefined4)uStack_88;
          uStack_154 = uStack_88._4_4_;
          uStack_150 = (undefined4)uStack_80;
          uStack_14c = uStack_80._4_4_;
          puStack_148 = uStack_78;
          uVar10 = uStack_170;
          if (uStack_170 != 0) {
            func_0x000140613c20(uStack_90,uStack_170,1);
          }
          uStack_61 = 1;
          uVar5 = FUN_140d76ed0(&uStack_1a0);
          uStack_61 = 1;
          FUN_140d6a710(&uStack_138,uVar5,uVar10);
          uVar10 = uStack_138;
          if (uStack_138 != 2) {
            uStack_61 = 1;
            uVar5 = FUN_140d76ed0(&uStack_1a0);
            uStack_61 = 1;
            cVar4 = FUN_140d62cf0(uVar5);
            if (cVar4 == '\0') {
              uStack_61 = 1;
              FUN_1428d9430(&UNK_142bc8c38,0x13,&UNK_142bc8c50);
              goto LAB_141002eaf;
            }
            if (4 < *_DAT_14362c3a8) {
              uStack_80 = FUN_140d61ae0;
              uStack_78 = &uStack_158;
              uStack_70 = &UNK_1400dd3f0;
              uStack_61 = 1;
              uStack_88 = &uStack_1a0;
              puVar7 = (undefined4 *)func_0x00014067dd80(&UNK_142bc8cc0);
              uVar2 = *puVar7;
              uVar3 = puVar7[2];
              uStack_10c = puVar7[3];
              uStack_ec = puVar7[4];
              uStack_108 = (undefined8 *)0x5;
              uStack_100 = &UNK_142bc8cd8;
              uStack_f8 = 0x11;
              puStack_e8 = &UNK_142bc8c90;
              uStack_e0 = 3;
              puStack_d8 = &uStack_88;
              uStack_d0 = 2;
              uStack_c8 = 0;
              uStack_138 = 0;
              uStack_130 = &UNK_142bc8cd8;
              uStack_128 = 0x11;
              uStack_120 = 0;
              uStack_11c = 0;
              uStack_118 = (undefined1)uVar2;
              uStack_117 = (undefined1)((uint)uVar2 >> 8);
              uStack_116 = (undefined2)((uint)uVar2 >> 0x10);
              uStack_114 = (undefined2)puVar7[1];
              uStack_112 = (undefined2)((uint)puVar7[1] >> 0x10);
              uStack_110 = (undefined1)uVar3;
              uStack_10f = (undefined1)((uint)uVar3 >> 8);
              uStack_10e = (undefined1)((uint)uVar3 >> 0x10);
              uStack_10d = (undefined1)((uint)uVar3 >> 0x18);
              uStack_f0 = 1;
              uStack_61 = 1;
              func_0x000140d4a330(&uStack_63,&uStack_138);
            }
            uStack_138 = 0;
            uStack_130 = (undefined *)0x1ed00000001;
            uStack_128 = uStack_128 & 0xffffffff00000000;
            uStack_118 = (undefined1)param_5;
            uStack_117 = (undefined1)((ushort)param_5 >> 8);
            uStack_116 = param_6;
            uStack_114 = 0x7bc;
            uStack_112 = 0x101;
            uStack_110 = 0;
            uStack_10f = 0;
            uStack_10e = 0;
            uStack_10c = uStack_10c & 0xffffff00;
            uStack_61 = 1;
            ppuVar11 = &puStack_278;
            FUN_1410036f0(&uStack_88,ppuVar11,&uStack_158,&uStack_138);
            puVar7 = uStack_88;
            if (uStack_88 == (undefined4 *)0x4) {
              uStack_78 = (undefined4 *)CONCAT44(uStack_18c,uStack_190);
              uStack_70 = (undefined *)CONCAT44(uStack_184,uStack_188);
              uStack_88 = (undefined4 *)CONCAT44(uStack_19c,uStack_1a0);
              uStack_80 = (code *)CONCAT44(uStack_194,uStack_198);
              uStack_138 = uStack_138 & 0xffffffff00000000;
              uStack_130 = (undefined *)0x0;
              uStack_128 = 0;
              uStack_120 = 7;
              uStack_114 = 0;
              uStack_11c = 0;
              uStack_117 = 0;
              uStack_116 = 0;
              uStack_118 = 1;
              uVar8 = FUN_140d76ed0(&uStack_88);
              lVar6 = FUN_140d59dc0(&uStack_138,uVar8,ppuVar11);
              if (uStack_88 != (undefined4 *)0x0) {
                func_0x000140613c20(uStack_80,uStack_88,1);
              }
              if (lVar6 == 0) {
                plVar9 = &lStack_b0;
                uStack_170 = uVar8;
                lVar6 = FUN_140d59c30(&uStack_170);
                if ((lVar6 == 0) &&
                   (plVar9 = (longlong *)FUN_141003250(&puStack_278,uStack_a8,uStack_a0),
                   plVar9 == (longlong *)0x0)) {
                  uStack_a0 = 0;
                  CloseHandle(uStack_170);
                  lVar6 = lStack_180;
                  goto LAB_141002cba;
                }
                puVar12 = puStack_98;
                *puStack_98 = 0;
                puStack_98[1] = (ulonglong)plVar9;
                CloseHandle(uStack_170);
                lVar13 = CONCAT44(uStack_154,uStack_158);
              }
              else {
                *puStack_98 = 0;
                puStack_98[1] = uVar8;
                lVar13 = CONCAT44(uStack_154,uStack_158);
                puVar12 = puStack_98;
              }
            }
            else {
              *(undefined4 *)(puStack_98 + 1) = (undefined4)uStack_80;
              *(undefined4 *)((longlong)puStack_98 + 0xc) = uStack_80._4_4_;
              *(undefined4 *)(puStack_98 + 2) = (undefined4)uStack_78;
              *(undefined4 *)((longlong)puStack_98 + 0x14) = uStack_78._4_4_;
              *puStack_98 = (ulonglong)uStack_88;
              lVar13 = CONCAT44(uStack_154,uStack_158);
              puVar12 = puStack_98;
            }
            if (lVar13 != 0) {
              func_0x000140613c20(CONCAT44(uStack_14c,uStack_150),lVar13,1);
            }
            if (((int)puVar7 != 4) && (CONCAT44(uStack_19c,uStack_1a0) != 0)) {
              func_0x000140613c20(CONCAT44(uStack_194,uStack_198),CONCAT44(uStack_19c,uStack_1a0),1)
              ;
            }
            goto LAB_141002ddb;
          }
          if (((uint)uStack_130 & 3) == 1) {
            lStack_178 = (longlong)uStack_130 + -1;
            uStack_90 = *(undefined8 *)((longlong)uStack_130 + -1);
            puStack_b8 = *(undefined8 **)((longlong)uStack_130 + 7);
            if ((code *)*puStack_b8 != (code *)0x0) {
              (*(code *)*puStack_b8)(uStack_90);
            }
            if (puStack_b8[1] != 0) {
              func_0x000140613c20(uStack_90,puStack_b8[1],puStack_b8[2]);
            }
            func_0x000140613c20(lStack_178,0x18,8);
          }
LAB_141002cba:
          if (CONCAT44(uStack_154,uStack_158) != 0) {
            func_0x000140613c20(CONCAT44(uStack_14c,uStack_150),CONCAT44(uStack_154,uStack_158),1);
          }
          if (((int)uVar10 == 2) && (CONCAT44(uStack_19c,uStack_1a0) != 0)) {
            func_0x000140613c20(CONCAT44(uStack_194,uStack_198),CONCAT44(uStack_19c,uStack_1a0),1);
            lVar13 = lVar13 + 0x18;
            if (lVar13 == lStack_1b8) break;
            goto LAB_14100283e;
          }
          lVar13 = lVar13 + 0x18;
        } while (lVar13 != lStack_1b8);
      }
      FUN_141004df0(&uStack_138,&puStack_278);
      puVar12 = puStack_98;
      if (uStack_138 == 4) {
        *puStack_98 = 4;
      }
      else {
        *puStack_98 = uStack_138;
        *(undefined4 *)(puStack_98 + 1) = (undefined4)uStack_130;
        *(undefined4 *)((longlong)puStack_98 + 0xc) = uStack_130._4_4_;
        *(undefined4 *)(puStack_98 + 2) = (undefined4)uStack_128;
        *(undefined4 *)((longlong)puStack_98 + 0x14) = uStack_128._4_4_;
      }
LAB_141002ddb:
      if (lStack_b0 != 0) {
        func_0x000140613c20(uStack_a8,lStack_b0,1);
      }
      FUN_140fffc20(&puStack_278);
      return puVar12;
    }
    FUN_1428d9310(&UNK_142bc8c20);
  }
LAB_141002eaf:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

