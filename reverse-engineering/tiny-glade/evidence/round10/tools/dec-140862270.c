
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140862270(longlong *param_1,undefined8 param_2,undefined8 *param_3,undefined8 *param_4,
                  undefined8 *param_5)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  float fVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  uint ******ppppppuVar8;
  uint ******ppppppuVar9;
  uint ******ppppppuVar10;
  longlong lVar11;
  code *pcVar12;
  char *pcVar13;
  uint uVar14;
  float fVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  char cVar29;
  char cVar30;
  char cVar31;
  char cVar32;
  char cVar33;
  char cVar34;
  char cVar35;
  uint ******ppppppuVar36;
  undefined8 *puVar37;
  longlong lVar38;
  undefined8 uVar39;
  undefined8 uVar40;
  longlong lVar41;
  ulonglong uVar42;
  longlong lVar43;
  undefined *puVar44;
  undefined8 uVar45;
  uint *******pppppppuVar46;
  uint *******pppppppuVar47;
  uint *******pppppppuVar48;
  char *pcVar49;
  ulonglong uVar50;
  uint *******pppppppuVar51;
  ulonglong uVar52;
  ulonglong uVar53;
  ulonglong uVar54;
  uint ******ppppppuVar55;
  char *pcVar56;
  uint *******pppppppuVar57;
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  float fVar60;
  uint ****ppppuStack_338;
  undefined8 uStack_330;
  uint *****pppppuStack_318;
  uint ******ppppppuStack_310;
  uint *****pppppuStack_308;
  undefined8 uStack_2f8;
  undefined8 uStack_2f0;
  uint uStack_2e8;
  uint ******ppppppuStack_2e0;
  uint *****pppppuStack_2d8;
  longlong lStack_2d0;
  undefined8 uStack_2c8;
  undefined4 *puStack_2c0;
  undefined4 *puStack_2b8;
  longlong lStack_2b0;
  char *pcStack_2a8;
  uint ******ppppppuStack_2a0;
  char *pcStack_298;
  uint ******ppppppuStack_290;
  longlong lStack_288;
  undefined8 uStack_280;
  undefined8 uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  char *pcStack_248;
  char *pcStack_240;
  uint ******ppppppuStack_238;
  char *pcStack_230;
  uint ******ppppppuStack_228;
  uint ******ppppppuStack_220;
  uint ******ppppppuStack_218;
  uint ******ppppppuStack_210;
  uint ******ppppppuStack_208;
  uint ******ppppppuStack_200;
  uint *****pppppuStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  uint uStack_1cc;
  undefined8 uStack_1c8;
  undefined *puStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined1 auStack_1a8 [16];
  uint ******ppppppuStack_190;
  char *pcStack_188;
  uint ******ppppppuStack_180;
  longlong lStack_178;
  undefined8 uStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  uint ******ppppppuStack_138;
  char *pcStack_130;
  uint ******ppppppuStack_128;
  uint ******ppppppuStack_120;
  uint ******ppppppuStack_118;
  uint *****pppppuStack_110;
  uint ******ppppppuStack_108;
  uint ******ppppppuStack_100;
  uint ******ppppppuStack_f8;
  uint ******ppppppuStack_f0;
  char *pcStack_e8;
  undefined8 uStack_e0;
  
  uStack_e0 = 0xfffffffffffffffe;
  puVar5 = (ulonglong *)*param_3;
  lVar38 = param_3[1];
  uVar6 = *puVar5;
  uVar53 = 0;
  uVar42 = uVar6 - *(ulonglong *)(lVar38 + 0x18);
  if (uVar6 < *(ulonglong *)(lVar38 + 0x18)) {
    uVar42 = uVar53;
  }
  uVar52 = uVar6 - *(ulonglong *)(lVar38 + 0x38);
  if (uVar6 < *(ulonglong *)(lVar38 + 0x38)) {
    uVar52 = uVar53;
  }
  uVar6 = *(ulonglong *)(lVar38 + 0x30);
  uVar54 = uVar6 - uVar52;
  if (uVar6 < uVar52) {
    uVar54 = uVar53;
  }
  uVar7 = *(ulonglong *)(lVar38 + 0x10);
  uVar50 = uVar7 - uVar42;
  if (uVar7 < uVar42) {
    uVar50 = uVar53;
  }
  uVar53 = *(ulonglong *)(lVar38 + 0x40);
  *puVar5 = uVar53 - (uVar50 + uVar54);
  if ((uVar7 < uVar42 || uVar7 - uVar42 == 0) && (uVar6 < uVar52 || uVar6 - uVar52 == 0)) {
    return;
  }
  *puVar5 = uVar53;
  if (2 < *puRam000000014362c3a8) {
    puVar37 = (undefined8 *)func_0x00014067dd80(&UNK_142a96140);
    uStack_1f0 = (uint *******)*puVar37;
    uStack_1e8 = (uint ******)puVar37[1];
    uStack_1e0 = 3;
    uStack_1dc = 0;
    uStack_1d8 = 0x42a96158;
    uStack_1d4 = 1;
    uStack_1d0 = 0x2c;
    uStack_1cc = 0;
    puStack_1c0 = &UNK_142a960f8;
    uStack_1b8 = 1;
    uStack_1b0 = 8;
    ppppppuStack_210 = (uint ******)0x0;
    auStack_1a8 = (undefined1  [16])0x0;
    ppppppuStack_208 = (uint ******)&UNK_142a96158;
    ppppppuStack_200 = (uint ******)0x2c;
    pppppuStack_1f8 = (uint *****)0x0;
    uStack_1c8 = CONCAT44(*(undefined4 *)(puVar37 + 2),1);
    func_0x000140d4a330(&ppppppuStack_190,&ppppppuStack_210);
  }
  lStack_2d0 = *param_1;
  lVar38 = (ulonglong)*(byte *)(lStack_2d0 + 0x58) * 8;
  puVar44 = &UNK_142aef51a;
  FUN_140d62810(&ppppppuStack_210,&UNK_142aef51a,0xc,*(undefined8 *)(&UNK_142a96690 + lVar38),
                *(undefined8 *)(&UNK_142a966b8 + lVar38));
  uVar39 = FUN_140d76ed0(&ppppppuStack_210);
  FUN_140d62810(&ppppppuStack_190,uVar39,puVar44,&UNK_142a96184,10);
  uVar40 = FUN_140d76ed0(&ppppppuStack_190);
  FUN_140d59140(&pcStack_298,uVar40,uVar39);
  if ((uint *******)ppppppuStack_190 != (uint *******)0x0) {
    func_0x000140613c20(pcStack_188,ppppppuStack_190,1);
  }
  pcStack_e8 = pcStack_298;
  if (SBORROW8(0,(longlong)pcStack_298)) {
    ppppppuStack_190 = ppppppuStack_290;
    FUN_1428d9760(&UNK_142a96184,10,&ppppppuStack_190,&UNK_142a95ea0,&UNK_142a96190);
  }
  else {
    ppppppuStack_100 = ppppppuStack_290;
    FUN_1414062b0(&ppppppuStack_190,ppppppuStack_290,lStack_288);
    ppppppuVar8 = ppppppuStack_180;
    pcVar49 = pcStack_188;
    ppppppuVar55 = ppppppuStack_190;
    if (!SBORROW8(0,(longlong)ppppppuStack_190)) {
      uStack_2c8 = param_2;
      if (pcStack_e8 != (char *)0x0) {
        func_0x000140613c20(ppppppuStack_100,pcStack_e8,1);
      }
      if ((uint *******)ppppppuStack_210 != (uint *******)0x0) {
        func_0x000140613c20(ppppppuStack_208,ppppppuStack_210,1);
      }
      uVar39 = _UNK_142a95da0;
      cVar35 = UNK_142a95d93;
      cVar34 = UNK_142a95d92;
      cVar33 = UNK_142a95d91;
      cVar32 = UNK_142a95d90;
      cVar31 = UNK_142a95d8f;
      cVar30 = UNK_142a95d8e;
      cVar29 = UNK_142a95d8d;
      cVar28 = UNK_142a95d8c;
      cVar27 = UNK_142a95d8b;
      cVar26 = UNK_142a95d8a;
      cVar25 = UNK_142a95d89;
      cVar24 = UNK_142a95d88;
      cVar23 = UNK_142a95d87;
      cVar22 = UNK_142a95d86;
      cVar21 = UNK_142a95d85;
      cVar20 = UNK_142a95d84;
      cVar19 = UNK_142a95d83;
      cVar18 = UNK_142a95d82;
      cVar17 = UNK_142a95d81;
      cVar16 = UNK_142a95d80;
      fVar15 = _UNK_142a83814;
      uVar14 = _UNK_14292e980;
      pcStack_230 = pcVar49 + (longlong)ppppppuVar8 * 0x50;
      pcStack_248 = pcVar49;
      ppppppuStack_238 = ppppppuVar55;
      uStack_13c = *(undefined4 *)((longlong)param_4 + 0x1c);
      ppppppuStack_100 = (uint ******)*param_4;
      puStack_2b8 = (undefined4 *)param_4[2];
      uStack_140 = *(undefined4 *)((longlong)param_5 + 0x1c);
      pcStack_e8 = (char *)*param_5;
      puStack_2c0 = (undefined4 *)param_5[2];
      pcStack_2a8 = pcStack_e8 + 0x20;
      lVar38 = 0;
      fVar60 = (float)_UNK_142a95db0;
      uVar6 = (ulonglong)_UNK_142a95db0 >> 0x20;
      pcVar13 = pcStack_230;
      if (pcVar49 != pcStack_230) {
        do {
          while( true ) {
            pcVar56 = pcVar49 + 0x50;
            pppppppuVar57 = *(uint ********)pcVar49;
            uStack_258 = *(undefined8 *)(pcVar49 + 0x48);
            uStack_268 = *(undefined8 *)(pcVar49 + 0x38);
            uStack_260 = *(undefined8 *)(pcVar49 + 0x40);
            uVar40 = *(undefined8 *)(pcVar49 + 0x28);
            uStack_270 = *(undefined8 *)(pcVar49 + 0x30);
            lStack_288 = *(longlong *)(pcVar49 + 0x18);
            uVar45 = *(undefined8 *)(pcVar49 + 0x20);
            pcStack_298 = *(char **)(pcVar49 + 8);
            pppppppuVar46 = *(uint ********)(pcVar49 + 0x10);
            pcVar49 = pcVar56;
            ppppppuStack_290 = (uint ******)pppppppuVar46;
            uStack_280 = uVar45;
            uStack_278 = uVar40;
            if (SBORROW8(0,(longlong)pppppppuVar57)) goto code_r0x000140862dd5;
            uStack_278._0_4_ = (undefined4)uVar40;
            uStack_278._4_4_ = (undefined4)((ulonglong)uVar40 >> 0x20);
            uStack_270._4_4_ = (undefined4)((ulonglong)uStack_270 >> 0x20);
            uStack_268._4_4_ = (undefined4)((ulonglong)uStack_268 >> 0x20);
            uStack_260._4_4_ = (undefined4)((ulonglong)uStack_260 >> 0x20);
            uStack_158 = (undefined4)uStack_268;
            uStack_154 = uStack_268._4_4_;
            uStack_150 = (undefined4)uStack_260;
            uStack_14c = uStack_260._4_4_;
            uStack_168 = (undefined4)uStack_278;
            uStack_164 = uStack_278._4_4_;
            uStack_160 = (undefined4)uStack_270;
            uStack_15c = uStack_270._4_4_;
            ppppppuStack_190 = (uint ******)pppppppuVar57;
            pcStack_188 = pcStack_298;
            ppppppuStack_180 = (uint ******)pppppppuVar46;
            lStack_178 = lStack_288;
            uStack_170 = uVar45;
            uStack_148 = uStack_258;
            if ((pppppppuVar46 != (uint *******)0x12) ||
               (auVar58[0] = -(*pcStack_298 == cVar16), auVar58[1] = -(pcStack_298[1] == cVar17),
               auVar58[2] = -(pcStack_298[2] == cVar18), auVar58[3] = -(pcStack_298[3] == cVar19),
               auVar58[4] = -(pcStack_298[4] == cVar20), auVar58[5] = -(pcStack_298[5] == cVar21),
               auVar58[6] = -(pcStack_298[6] == cVar22), auVar58[7] = -(pcStack_298[7] == cVar23),
               auVar58[8] = -(pcStack_298[8] == cVar24), auVar58[9] = -(pcStack_298[9] == cVar25),
               auVar58[10] = -(pcStack_298[10] == cVar26),
               auVar58[0xb] = -(pcStack_298[0xb] == cVar27),
               auVar58[0xc] = -(pcStack_298[0xc] == cVar28),
               auVar58[0xd] = -(pcStack_298[0xd] == cVar29),
               auVar58[0xe] = -(pcStack_298[0xe] == cVar30),
               auVar58[0xf] = -(pcStack_298[0xf] == cVar31),
               auVar59[0] = -((char)*(undefined2 *)(pcStack_298 + 0x10) == cVar32),
               auVar59[1] = -((char)((ushort)*(undefined2 *)(pcStack_298 + 0x10) >> 8) == cVar33),
               auVar59[2] = -(cVar34 == '\0'), auVar59[3] = -(cVar35 == '\0'), auVar59[4] = 0xff,
               auVar59[5] = 0xff, auVar59[6] = 0xff, auVar59[7] = 0xff, auVar59[8] = 0xff,
               auVar59[9] = 0xff, auVar59[10] = 0xff, auVar59[0xb] = 0xff, auVar59[0xc] = 0xff,
               auVar59[0xd] = 0xff, auVar59[0xe] = 0xff, auVar59[0xf] = 0xff,
               auVar59 = auVar59 & auVar58,
               (ushort)((ushort)(SUB161(auVar59 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar59[0xf] >> 7) << 0xf) != 0xffff)) break;
            if (pppppppuVar57 != (uint *******)0x0) {
              func_0x000140613c20(pcStack_298,pppppppuVar57,1);
            }
            if (lStack_178 != 0) {
              func_0x000140613c20(uStack_170,lStack_178,1);
            }
            lVar41 = CONCAT44(uStack_15c,uStack_160);
            if ((!SBORROW8(0,lVar41)) && (lVar41 != 0)) {
              func_0x000140613c20(CONCAT44(uStack_154,uStack_158),lVar41,1);
            }
            if (pcVar56 == pcVar13) goto code_r0x000140862dd5;
          }
          lVar41 = (ulonglong)*(byte *)(lStack_2d0 + 0x58) * 8;
          pcStack_240 = pcVar56;
          pcStack_130 = pcStack_298;
          FUN_140d62810(&ppppppuStack_210,&UNK_142aef51a,0xc,
                        *(undefined8 *)(&UNK_142a96690 + lVar41),
                        *(undefined8 *)(&UNK_142a966b8 + lVar41));
          FUN_1412e68e0(&ppppuStack_338,uVar45,uVar40,&ppppppuStack_210);
          pppppuStack_2d8 = &ppppuStack_338;
          uVar40 = FUN_140d76ed0(&ppppuStack_338);
          FUN_140d59140(&ppppppuStack_128,uVar40,uVar45);
          ppppppuStack_228 = ppppppuStack_128;
          if (SBORROW8(0,(longlong)ppppppuStack_128)) {
            ppppppuStack_2e0 = ppppppuStack_120;
            ppppppuStack_f8 = &pppppuStack_2d8;
            ppppppuStack_f0 = (uint ******)&UNK_14002e8f0;
            ppppppuStack_210 = (uint ******)&UNK_142a96250;
            ppppppuStack_208 = (uint ******)0x1;
            uStack_1f0 = (uint *******)0x0;
            ppppppuStack_200 = (uint ******)&ppppppuStack_f8;
            pppppuStack_1f8 = (uint *****)0x1;
            FUN_1428d9390(&ppppppuStack_210,&UNK_142a96260);
            goto code_r0x000140862f7a;
          }
          ppppppuStack_218 = ppppppuStack_120;
          FUN_141405cf0(&ppppppuStack_210,ppppppuStack_120,ppppppuStack_118);
          ppppppuVar55 = ppppppuStack_200;
          ppppppuStack_220 = ppppppuStack_210;
          if (SBORROW8(0,(longlong)ppppppuStack_210)) {
            ppppppuStack_128 = ppppppuStack_208;
            FUN_1428d9760(&UNK_142a95f00,0x2b,&ppppppuStack_128,&UNK_142a95ec0,&UNK_142a961d8);
            goto code_r0x000140862f7a;
          }
          ppppppuStack_138 = ppppppuStack_208;
          ppppppuStack_f8 = (uint ******)FUN_141556080(pcStack_130);
          pcStack_130 = (char *)CONCAT44(pcStack_130._4_4_,(undefined4)uStack_148);
          pppppppuVar51 = (uint *******)(ppppppuStack_138 + (longlong)ppppppuVar55 * 6);
          pppppppuVar57 = (uint *******)ppppppuStack_138;
          ppppppuStack_f0 = (uint ******)pppppppuVar46;
          while (pppppppuVar46 = pppppppuVar57, pppppppuVar46 != pppppppuVar51) {
            pppppppuVar57 = pppppppuVar46 + 6;
            if (((float)(*(uint *)pppppppuVar46 & uVar14) < fVar15) &&
               ((float)(*(uint *)(pppppppuVar46 + 1) & uVar14) < fVar15)) {
              FUN_140d146a0(&ppppppuStack_210,uStack_2c8);
              pppppppuVar48 = uStack_1f0;
              pppppppuVar47 = (uint *******)ppppppuStack_208;
              if ((uint *******)ppppppuStack_208 == (uint *******)0x0) {
                pppppppuVar47 = (uint *******)ppppppuStack_200;
              }
              ppppppuVar55 = pppppppuVar47[2];
              if ((ulonglong)((longlong)*pppppppuVar47 - (longlong)ppppppuVar55) < 0x30) {
                FUN_1428a9a60(pppppppuVar47,ppppppuVar55,0x30,1,1);
              }
              lStack_2b0 = lVar38 + 0x186a1;
              ppppppuVar8 = pppppppuVar47[1];
              *(code **)((longlong)ppppppuVar8 + (longlong)ppppppuVar55) = FUN_1412d74f0;
              *(code **)((longlong)(ppppppuVar8 + 1) + (longlong)ppppppuVar55) = FUN_141564d30;
              *(uint ********)((longlong)(ppppppuVar8 + 2) + (longlong)ppppppuVar55) = pppppppuVar48
              ;
              *(longlong *)((longlong)(ppppppuVar8 + 3) + (longlong)ppppppuVar55) = lStack_2b0;
              *(undefined1 *)((longlong)(ppppppuVar8 + 4) + (longlong)ppppppuVar55) = 1;
              *(undefined1 *)((longlong)(ppppppuVar8 + 5) + (longlong)ppppppuVar55) = 0;
              pppppppuVar47[2] = ppppppuVar55 + 6;
              ppppppuStack_108 = (uint ******)uStack_1f0;
              ppppppuStack_118 = ppppppuStack_200;
              pppppuStack_110 = pppppuStack_1f8;
              ppppppuStack_128 = ppppppuStack_210;
              ppppppuStack_120 = ppppppuStack_208;
              LOCK();
              ppppppuVar55 = (uint ******)*ppppppuStack_f8;
              *ppppppuStack_f8 = (uint *****)((longlong)*ppppppuStack_f8 + 1);
              UNLOCK();
              if ((uint ******)*ppppppuStack_f8 == (uint ******)0x0 ||
                  SCARRY8((longlong)ppppppuVar55,1) != (longlong)*ppppppuStack_f8 < 0)
              goto code_r0x000140862f7a;
              FUN_1412deaf0(&ppppppuStack_128,ppppppuStack_f8,ppppppuStack_f0,&UNK_142a961f0);
              ppppppuVar55 = ppppppuStack_108;
              pppppppuVar48 = (uint *******)ppppppuStack_120;
              if ((uint *******)ppppppuStack_120 == (uint *******)0x0) {
                pppppppuVar48 = (uint *******)ppppppuStack_118;
              }
              ppppppuVar8 = pppppppuVar48[2];
              if ((ulonglong)((longlong)*pppppppuVar48 - (longlong)ppppppuVar8) < 0x30) {
                FUN_1428a9a60(pppppppuVar48,ppppppuVar8,0x30,1,1);
              }
              ppppppuVar10 = ppppppuStack_108;
              ppppppuVar9 = pppppppuVar48[1];
              *(code **)((longlong)ppppppuVar9 + (longlong)ppppppuVar8) = FUN_1412d67d0;
              *(code **)((longlong)(ppppppuVar9 + 1) + (longlong)ppppppuVar8) = FUN_141564d30;
              *(uint *******)((longlong)(ppppppuVar9 + 2) + (longlong)ppppppuVar8) = ppppppuVar55;
              *(undefined8 *)((longlong)(ppppppuVar9 + 3) + (longlong)ppppppuVar8) = uVar39;
              *(undefined4 *)((longlong)(ppppppuVar9 + 4) + (longlong)ppppppuVar8) = 0x3f800000;
              *(undefined1 *)((longlong)ppppppuVar9 + 0x24 + (longlong)ppppppuVar8) = 0;
              *(undefined1 *)((longlong)(ppppppuVar9 + 5) + (longlong)ppppppuVar8) = 0;
              pppppppuVar48[2] = ppppppuVar8 + 6;
              pppppppuVar48 = (uint *******)ppppppuStack_120;
              if ((uint *******)ppppppuStack_120 == (uint *******)0x0) {
                pppppppuVar48 = (uint *******)ppppppuStack_118;
              }
              uVar2 = *(uint *)pppppppuVar46;
              uVar3 = *(uint *)(pppppppuVar46 + 1);
              ppppppuVar55 = pppppppuVar48[2];
              if ((ulonglong)((longlong)*pppppppuVar48 - (longlong)ppppppuVar55) < 0x28) {
                FUN_1428a9a60(pppppppuVar48,ppppppuVar55,0x28,1,1);
              }
              ppppppuVar9 = ppppppuStack_108;
              ppppppuVar8 = pppppppuVar48[1];
              *(code **)((longlong)ppppppuVar8 + (longlong)ppppppuVar55) = FUN_1412db1a0;
              *(code **)((longlong)(ppppppuVar8 + 1) + (longlong)ppppppuVar55) = FUN_141564d30;
              *(uint *******)((longlong)(ppppppuVar8 + 2) + (longlong)ppppppuVar55) = ppppppuVar10;
              *(uint *)((longlong)(ppppppuVar8 + 3) + (longlong)ppppppuVar55) = uVar2;
              *(uint *)((longlong)ppppppuVar8 + 0x1c + (longlong)ppppppuVar55) = uVar3;
              *(undefined1 *)((longlong)(ppppppuVar8 + 4) + (longlong)ppppppuVar55) = 0;
              pppppppuVar48[2] = ppppppuVar55 + 5;
              ppppppuVar55 = *pppppppuVar46;
              pppppppuVar48 = (uint *******)ppppppuStack_120;
              if ((uint *******)ppppppuStack_120 == (uint *******)0x0) {
                pppppppuVar48 = (uint *******)ppppppuStack_118;
              }
              fVar4 = *(float *)(pppppppuVar46 + 1);
              ppppppuVar8 = pppppppuVar48[2];
              if ((ulonglong)((longlong)*pppppppuVar48 - (longlong)ppppppuVar8) < 0x28) {
                FUN_1428a9a60(pppppppuVar48,ppppppuVar8,0x28,1,1);
              }
              ppppppuVar36 = ppppppuStack_108;
              ppppppuVar10 = pppppppuVar48[1];
              *(code **)((longlong)ppppppuVar10 + (longlong)ppppppuVar8) = FUN_1412d95b0;
              *(code **)((longlong)(ppppppuVar10 + 1) + (longlong)ppppppuVar8) = FUN_141564d30;
              *(uint *******)((longlong)(ppppppuVar10 + 2) + (longlong)ppppppuVar8) = ppppppuVar9;
              *(ulonglong *)((longlong)(ppppppuVar10 + 3) + (longlong)ppppppuVar8) =
                   CONCAT44((float)((ulonglong)ppppppuVar55 >> 0x20) + (float)uVar6,
                            SUB84(ppppppuVar55,0) + fVar60);
              *(float *)((longlong)(ppppppuVar10 + 4) + (longlong)ppppppuVar8) = fVar4 + 0.0;
              *(undefined1 *)((longlong)ppppppuVar10 + 0x24 + (longlong)ppppppuVar8) = 0;
              pppppppuVar48[2] = ppppppuVar8 + 5;
              pppppuStack_318 = (uint *****)*pppppppuVar46;
              ppppppuStack_310 = pppppppuVar46[1];
              pppppuStack_308 = (uint *****)pppppppuVar46[2];
              uStack_2f8 = *(undefined8 *)((longlong)pppppppuVar46 + 0x1c);
              uStack_2f0 = *(undefined8 *)((longlong)pppppppuVar46 + 0x24);
              uStack_2e8 = *(uint *)((longlong)pppppppuVar46 + 0x2c);
              LOCK();
              ppppppuVar55 = (uint ******)*ppppppuStack_f8;
              *ppppppuStack_f8 = (uint *****)((longlong)*ppppppuStack_f8 + 1);
              UNLOCK();
              if ((uint ******)*ppppppuStack_f8 == (uint ******)0x0 ||
                  SCARRY8((longlong)ppppppuVar55,1) != (longlong)*ppppppuStack_f8 < 0)
              goto code_r0x000140862f7a;
              *puStack_2c0 = uStack_140;
              uStack_1c8 = *(undefined8 *)(pcStack_e8 + 0x40);
              ppppppuStack_2a0 = ppppppuStack_f8;
              ppppppuStack_210 = ppppppuStack_f8;
              ppppppuStack_208 = ppppppuStack_f0;
              ppppppuStack_200 = ppppppuStack_108;
              uStack_1e0 = pcStack_130._0_4_;
              uStack_1dc = (undefined4)uStack_2f8;
              uStack_1d8 = (undefined4)((ulonglong)uStack_2f8 >> 0x20);
              uStack_1d4 = (undefined4)uStack_2f0;
              uStack_1d0 = (undefined4)((ulonglong)uStack_2f0 >> 0x20);
              lVar41 = *(longlong *)(pcStack_e8 + 0x30);
              pppppuStack_1f8 = pppppuStack_318;
              uStack_1f0 = (uint *******)ppppppuStack_310;
              uStack_1e8 = (uint ******)pppppuStack_308;
              uStack_1cc = uStack_2e8;
              if (lVar41 == *(longlong *)(pcStack_e8 + 0x20)) {
                FUN_140563540(pcStack_2a8,&UNK_142c3a148);
              }
              lVar11 = *(longlong *)(pcStack_e8 + 0x28);
              lVar43 = lVar41 * 0x50;
              puVar37 = (undefined8 *)(lVar11 + 0x40 + lVar43);
              *puVar37 = CONCAT44(uStack_1cc,uStack_1d0);
              puVar37[1] = uStack_1c8;
              puVar1 = (undefined4 *)(lVar11 + 0x30 + lVar43);
              *puVar1 = uStack_1e0;
              puVar1[1] = uStack_1dc;
              puVar1[2] = uStack_1d8;
              puVar1[3] = uStack_1d4;
              puVar1 = (undefined4 *)(lVar11 + 0x20 + lVar43);
              *puVar1 = (undefined4)uStack_1f0;
              puVar1[1] = uStack_1f0._4_4_;
              puVar1[2] = (undefined4)uStack_1e8;
              puVar1[3] = uStack_1e8._4_4_;
              puVar37 = (undefined8 *)(lVar11 + 0x10 + lVar43);
              *puVar37 = ppppppuStack_200;
              puVar37[1] = pppppuStack_1f8;
              *(undefined8 *)(lVar11 + lVar43) = ppppppuStack_210;
              ((undefined8 *)(lVar11 + lVar43))[1] = ppppppuStack_208;
              *(longlong *)(pcStack_e8 + 0x30) = lVar41 + 1;
              *(longlong *)(pcStack_e8 + 0x40) = *(longlong *)(pcStack_e8 + 0x40) + 1;
              *puStack_2b8 = uStack_13c;
              uVar42 = FUN_140eec970(ppppppuStack_100,lStack_2b0,ppppppuVar36);
              ppppppuVar55 = (uint ******)(lVar38 + 0x186a2);
              if ((uint ******)(lVar38 + 0x186a2) <= ppppppuStack_100[6]) {
                ppppppuVar55 = (uint ******)ppppppuStack_100[6];
              }
              ppppppuStack_100[6] = (uint *****)ppppppuVar55;
              lVar38 = lVar38 + 1;
              if (uVar42 >> 0x20 != 0) {
                FUN_1428d9430(&UNK_142a96208,0x20,&UNK_142a96228);
                goto code_r0x000140862f7a;
              }
            }
          }
          LOCK();
          *ppppppuStack_f8 = (uint *****)((longlong)*ppppppuStack_f8 + -1);
          UNLOCK();
          if ((uint ******)*ppppppuStack_f8 == (uint ******)0x0) {
            func_0x00014064eb30(&ppppppuStack_f8);
          }
          if ((uint *******)ppppppuStack_220 != (uint *******)0x0) {
            func_0x000140613c20(ppppppuStack_138,(longlong)ppppppuStack_220 * 0x30,4);
          }
          if ((uint *******)ppppppuStack_228 != (uint *******)0x0) {
            func_0x000140613c20(ppppppuStack_218,ppppppuStack_228,1);
          }
          if (ppppuStack_338 != (uint ****)0x0) {
            func_0x000140613c20(uStack_330,ppppuStack_338,1);
          }
          if ((uint *******)ppppppuStack_190 != (uint *******)0x0) {
            func_0x000140613c20(pcStack_188,ppppppuStack_190,1);
          }
          if (lStack_178 != 0) {
            func_0x000140613c20(uStack_170,lStack_178,1);
          }
          lVar41 = CONCAT44(uStack_15c,uStack_160);
          if ((!SBORROW8(0,lVar41)) && (lVar41 != 0)) {
            func_0x000140613c20(CONCAT44(uStack_154,uStack_158),lVar41,1);
          }
          pcVar49 = pcStack_240;
          pcVar13 = pcStack_230;
        } while (pcStack_240 != pcStack_230);
      }
code_r0x000140862dd5:
      pcStack_240 = pcVar49;
      FUN_140879230(&pcStack_248);
      return;
    }
    pcStack_298 = pcStack_188;
    FUN_1428d9760(&UNK_142a961a8,0x11,&pcStack_298,&UNK_142a95ec0,&UNK_142a961c0);
  }
code_r0x000140862f7a:
                    /* WARNING: Does not return */
  pcVar12 = (code *)invalidInstructionException();
  (*pcVar12)();
}

