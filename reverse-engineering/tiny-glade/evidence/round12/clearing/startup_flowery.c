
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void startup_flowery(void *p0,void *p1,void *p2,void *p3,void *p4,void *p5,void *p6)

{
  undefined4 *puVar1;
  undefined8 *puVar2;
  float *pfVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  longlong lVar7;
  code *pcVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  float fVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  undefined *puVar18;
  longlong lVar19;
  longlong *plVar20;
  float fVar21;
  float fVar22;
  longlong lStack_1e8;
  longlong lStack_1e0;
  longlong lStack_1d8;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined4 uStack_138;
  undefined4 uStack_134;
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
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  void *pvStack_c0;
  longlong lStack_b8;
  undefined8 uStack_b0;
  
  uStack_b0 = 0xfffffffffffffffe;
  pvStack_c0 = p2;
  FUN_140b0eeb0(&uStack_138,&UNK_142c6c7d8,0x25);
  if (SBORROW8(0,CONCAT44(uStack_134,uStack_138))) {
    FUN_1428d9760(&UNK_142c6c1c0,0x2b,&uStack_1c8,&UNK_142c6c1a0,&UNK_142c6c838);
  }
  else {
    uStack_158 = uStack_c8;
    uStack_168 = uStack_d8;
    uStack_160 = uStack_d0;
    uStack_178 = uStack_e8;
    uStack_170 = uStack_e0;
    uStack_188 = uStack_f8;
    uStack_180 = uStack_f0;
    uStack_198 = uStack_108;
    uStack_194 = uStack_104;
    uStack_190 = uStack_100;
    uStack_18c = uStack_fc;
    uStack_1a8 = uStack_118;
    uStack_1a4 = uStack_114;
    uStack_1a0 = uStack_110;
    uStack_19c = uStack_10c;
    uStack_1b8 = uStack_128;
    uStack_1b4 = uStack_124;
    uStack_1b0 = uStack_120;
    uStack_1ac = uStack_11c;
    uStack_1c8 = CONCAT44(uStack_134,uStack_138);
    uStack_1c0 = uStack_130;
    uStack_1bc = uStack_12c;
    uVar17 = *(undefined8 *)p0;
    uVar4 = *(undefined8 *)p1;
    uVar15 = FUN_140b1aca0(&uStack_1c8,&UNK_142c6c850,0x12,uVar17,uVar4);
    uVar5 = *(undefined8 *)p3;
    puVar18 = &UNK_142c6c862;
    uVar16 = FUN_140937aa0(uVar5,&UNK_142c6c862,0x16);
    if ((uVar16 & 1) == 0) {
      FUN_1428d9310(&UNK_142c6c878);
    }
    else {
      uStack_150 = uVar17;
      uStack_148 = uVar5;
      uStack_140 = uVar4;
      FUN_140d146a0(&uStack_138,pvStack_c0);
      plVar20 = (longlong *)CONCAT44(uStack_12c,uStack_130);
      if (plVar20 == (longlong *)0x0) {
        plVar20 = (longlong *)CONCAT44(uStack_124,uStack_128);
      }
      uVar17 = CONCAT44(uStack_114,uStack_118);
      lVar19 = plVar20[2];
      if ((ulonglong)(*plVar20 - lVar19) < 0x78) {
        FUN_1428a9a60(plVar20,lVar19,0x78,1,1);
      }
      lVar6 = plVar20[1];
      *(code **)(lVar6 + lVar19) = FUN_1412d9de0;
      uVar5 = _UNK_14293e0c8;
      uVar4 = _DAT_14293e0c0;
      puVar2 = (undefined8 *)(lVar6 + 8 + lVar19);
      *puVar2 = _DAT_14293e0c0;
      puVar2[1] = uVar5;
      *(undefined8 *)(lVar6 + 0x18 + lVar19) = 0;
      *(undefined4 *)(lVar6 + 0x20 + lVar19) = 0;
      *(undefined8 *)(lVar6 + 0x24 + lVar19) = 0x3f8000003f800000;
      *(undefined4 *)(lVar6 + 0x2c + lVar19) = 0x3f800000;
      *(undefined8 *)(lVar6 + 0x38 + lVar19) = uVar15;
      *(undefined **)(lVar6 + 0x40 + lVar19) = puVar18;
      *(undefined1 *)(lVar6 + 0x48 + lVar19) = 0;
      *(undefined8 *)(lVar6 + 0x58 + lVar19) = uVar17;
      *(code **)(lVar6 + 0x68 + lVar19) = FUN_141564d30;
      plVar20[2] = lVar19 + 0x78;
      **(undefined4 **)((longlong)p4 + 0x10) = *(undefined4 *)((longlong)p4 + 0x1c);
      uVar17 = *(undefined8 *)p4;
      lVar19 = (ulonglong)*(byte *)(*(longlong *)p6 + 0x58) * 8;
      puVar18 = &UNK_142aef51a;
      FUN_140d62810(&uStack_1c8,&UNK_142aef51a,0xc,*(undefined8 *)(&UNK_142c6cbc8 + lVar19),
                    *(undefined8 *)(&UNK_142c6cbf0 + lVar19));
      uVar15 = FUN_140d76ed0(&uStack_1c8);
      FUN_140d62810(&uStack_138,uVar15,puVar18,&UNK_142c6c890,0x14);
      FUN_1409365d0(&lStack_1e8,uVar17,p5,&uStack_138);
      fVar13 = _DAT_142925870;
      lStack_b8 = lStack_1e0;
      if (lStack_1d8 != 0) {
        lVar19 = 0;
        do {
          lVar6 = lStack_b8;
          fVar22 = *(float *)(lStack_b8 + 0x10 + lVar19) * fVar13;
          fVar21 = (float)func_0x000142923fd0(fVar22);
          fVar22 = (float)func_0x000142923e80(fVar22);
          puVar1 = (undefined4 *)(lVar6 + lVar19);
          uVar9 = *puVar1;
          uVar10 = puVar1[1];
          uVar11 = puVar1[2];
          uVar12 = puVar1[3];
          FUN_140d146a0(&uStack_138,pvStack_c0);
          plVar20 = (longlong *)CONCAT44(uStack_12c,uStack_130);
          if (plVar20 == (longlong *)0x0) {
            plVar20 = (longlong *)CONCAT44(uStack_124,uStack_128);
          }
          uVar17 = CONCAT44(uStack_114,uStack_118);
          lVar6 = plVar20[2];
          if ((ulonglong)(*plVar20 - lVar6) < 0x88) {
            FUN_1428a9a60(plVar20,lVar6,0x88,1,1);
          }
          lVar7 = plVar20[1];
          *(code **)(lVar7 + lVar6) = FUN_1412dabb0;
          pfVar3 = (float *)(lVar7 + 8 + lVar6);
          *pfVar3 = fVar21 * 0.0;
          pfVar3[1] = fVar21;
          pfVar3[2] = fVar21 * 0.0;
          pfVar3[3] = fVar22;
          puVar1 = (undefined4 *)(lVar7 + 0x18 + lVar6);
          *puVar1 = uVar9;
          puVar1[1] = uVar10;
          puVar1[2] = uVar11;
          puVar1[3] = uVar12;
          *(ulonglong *)(lVar7 + 0x28 + lVar6) = CONCAT44(uVar12,uVar12);
          *(undefined4 *)(lVar7 + 0x38 + lVar6) = 0;
          *(undefined8 *)(lVar7 + 0x40 + lVar6) = 0x13fa53c58fdddee8;
          *(undefined1 *)(lVar7 + 0x48 + lVar6) = 0xe;
          *(undefined1 *)(lVar7 + 0x58 + lVar6) = 0;
          *(undefined8 *)(lVar7 + 0x68 + lVar6) = uVar17;
          *(code **)(lVar7 + 0x78 + lVar6) = FUN_141564d30;
          plVar20[2] = lVar6 + 0x88;
          lVar19 = lVar19 + 0x30;
        } while (lStack_1d8 * 0x30 != lVar19);
      }
      if (lStack_1e8 != 0) {
        func_0x000140613c20(lStack_b8,lStack_1e8 * 0x30,4);
      }
      uVar14 = uStack_140;
      uVar15 = uStack_148;
      uVar17 = uStack_150;
      if (uStack_1c8 != 0) {
        func_0x000140613c20(CONCAT44(uStack_1bc,uStack_1c0),uStack_1c8,1);
      }
      FUN_140b0eeb0(&uStack_138,&UNK_142c6c8a4,0x29);
      if (!SBORROW8(0,CONCAT44(uStack_134,uStack_138))) {
        uStack_158 = uStack_c8;
        uStack_168 = uStack_d8;
        uStack_160 = uStack_d0;
        uStack_178 = uStack_e8;
        uStack_170 = uStack_e0;
        uStack_188 = uStack_f8;
        uStack_180 = uStack_f0;
        uStack_198 = uStack_108;
        uStack_194 = uStack_104;
        uStack_190 = uStack_100;
        uStack_18c = uStack_fc;
        uStack_1a8 = uStack_118;
        uStack_1a4 = uStack_114;
        uStack_1a0 = uStack_110;
        uStack_19c = uStack_10c;
        uStack_1b8 = uStack_128;
        uStack_1b4 = uStack_124;
        uStack_1b0 = uStack_120;
        uStack_1ac = uStack_11c;
        uStack_1c0 = uStack_130;
        uStack_1bc = uStack_12c;
        uVar17 = FUN_140b1aca0(&uStack_1c8,&UNK_142c6c8e8,0xb,uVar17,uVar14);
        puVar18 = &UNK_142c6c8f3;
        uVar16 = FUN_140937aa0(uVar15,&UNK_142c6c8f3,0xd);
        if ((uVar16 & 1) == 0) {
          FUN_1428d9310(&UNK_142c6c900);
          pcVar8 = (code *)swi(3);
          (*pcVar8)();
          return;
        }
        uVar15 = FUN_140c9cd80(&UNK_142c6c918,0xe);
        FUN_140d146a0(&uStack_138,pvStack_c0);
        plVar20 = (longlong *)CONCAT44(uStack_12c,uStack_130);
        if (plVar20 == (longlong *)0x0) {
          plVar20 = (longlong *)CONCAT44(uStack_124,uStack_128);
        }
        lVar19 = plVar20[2];
        if ((ulonglong)(*plVar20 - lVar19) < 0x88) {
          FUN_1428a9a60(plVar20,lVar19,0x88,1,1);
        }
        lVar6 = plVar20[1];
        *(code **)(lVar6 + lVar19) = FUN_1412d9270;
        puVar2 = (undefined8 *)(lVar6 + 8 + lVar19);
        *puVar2 = uVar4;
        puVar2[1] = uVar5;
        *(undefined8 *)(lVar6 + 0x18 + lVar19) = 0;
        *(undefined4 *)(lVar6 + 0x20 + lVar19) = 0;
        *(undefined8 *)(lVar6 + 0x24 + lVar19) = 0x3f8000003f800000;
        *(undefined4 *)(lVar6 + 0x2c + lVar19) = 0x3f800000;
        *(undefined8 *)(lVar6 + 0x38 + lVar19) = uVar17;
        *(undefined **)(lVar6 + 0x40 + lVar19) = puVar18;
        *(undefined8 *)(lVar6 + 0x48 + lVar19) = uVar15;
        *(undefined1 *)(lVar6 + 0x58 + lVar19) = 0;
        *(ulonglong *)(lVar6 + 0x68 + lVar19) = CONCAT44(uStack_114,uStack_118);
        *(code **)(lVar6 + 0x78 + lVar19) = FUN_141564d30;
        plVar20[2] = lVar19 + 0x88;
        return;
      }
    }
    FUN_1428d9760(&UNK_142c6c1c0,0x2b,&uStack_1c8,&UNK_142c6c1a0,&UNK_142c6c8d0);
  }
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

