
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a78aa0(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,longlong *param_4,
                  undefined8 *param_5,undefined8 *param_6,longlong param_7,undefined8 *param_8,
                  undefined8 *param_9,undefined8 *param_10,longlong *param_11)

{
  longlong **pplVar1;
  longlong **pplVar2;
  longlong lVar3;
  longlong lVar4;
  code *pcVar5;
  undefined8 uVar6;
  longlong lVar7;
  char cVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
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
  int iVar27;
  undefined4 uVar28;
  undefined4 *puVar29;
  char cVar30;
  undefined8 *puVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  longlong lVar34;
  ulonglong uVar35;
  longlong **pplVar36;
  longlong ***ppplVar37;
  undefined *puVar38;
  longlong lVar39;
  int *piVar40;
  char cVar41;
  longlong *plVar42;
  longlong ***ppplVar43;
  longlong ***ppplVar44;
  ulonglong uVar45;
  undefined *puVar46;
  longlong ***ppplVar47;
  longlong ***ppplVar48;
  longlong ***ppplVar49;
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auStack_2d8 [24];
  undefined1 auStack_2c0 [24];
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  longlong lStack_290;
  longlong ***ppplStack_288;
  longlong **pplStack_280;
  undefined4 *puStack_278;
  undefined8 *puStack_270;
  longlong *plStack_268;
  longlong *plStack_260;
  longlong *plStack_258;
  longlong ****pppplStack_248;
  longlong lStack_240;
  longlong **pplStack_238;
  longlong lStack_230;
  undefined4 uStack_228;
  undefined4 uStack_224;
  longlong *plStack_220;
  longlong *plStack_218;
  longlong *plStack_210;
  longlong *plStack_208;
  longlong lStack_200;
  longlong lStack_1f8;
  longlong *plStack_1f0;
  int iStack_1e8;
  undefined4 uStack_1e4;
  int iStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 *puStack_1d0;
  undefined4 *puStack_1c8;
  undefined4 *puStack_1c0;
  undefined4 *puStack_1b8;
  undefined4 *puStack_1b0;
  undefined4 *puStack_1a8;
  undefined4 *puStack_1a0;
  undefined *puStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  ulonglong uStack_170;
  longlong lStack_168;
  ulonglong uStack_160;
  longlong lStack_158;
  undefined8 *puStack_150;
  longlong **pplStack_148;
  longlong ***ppplStack_140;
  longlong lStack_138;
  ulonglong uStack_128;
  longlong lStack_120;
  longlong ***ppplStack_118;
  longlong ***ppplStack_110;
  longlong ***ppplStack_108;
  undefined4 *puStack_100;
  undefined4 *puStack_f8;
  undefined4 *puStack_f0;
  undefined4 *puStack_e8;
  undefined4 *puStack_e0;
  undefined4 *puStack_d8;
  undefined8 uStack_d0;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined1 auStack_b0 [16];
  longlong lStack_98;
  longlong ***ppplStack_90;
  longlong **pplStack_88;
  longlong **pplStack_80;
  undefined1 uStack_74;
  char cStack_73;
  undefined1 uStack_72;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  puStack_150 = param_3;
  if (3 < *puRam000000014362c3a8) {
    puVar31 = (undefined8 *)func_0x00014067dd80(&UNK_142ad6d40);
    puStack_f8 = (undefined4 *)*puVar31;
    puStack_f0 = (undefined4 *)puVar31[1];
    puStack_e8 = (undefined4 *)0x4;
    puStack_e0 = (undefined4 *)&UNK_142ad6d58;
    puStack_d8 = (undefined4 *)0x1f;
    puStack_c8 = &UNK_142ad6d00;
    uStack_c0 = 1;
    uStack_b8 = 8;
    ppplStack_118 = (longlong ***)0x0;
    auStack_b0 = (undefined1  [16])0x0;
    ppplStack_110 = (longlong ***)&UNK_142ad6d58;
    ppplStack_108 = (longlong ***)0x1f;
    puStack_100 = (undefined4 *)0x0;
    uStack_d0 = (undefined4 *)CONCAT44(*(undefined4 *)(puVar31 + 2),1);
    func_0x000140d4a330(&iStack_1e8,&ppplStack_118);
  }
  lStack_98 = *(longlong *)*param_1 + 0x10;
  cVar30 = *(char *)(*param_4 + 0x58);
  plStack_1f0 = param_4;
  if (cVar30 == '\x04') {
    FUN_140b0eeb0(&ppplStack_118,&UNK_142ad6d77,0x1d);
    uVar28 = uStack_1dc;
    iVar27 = iStack_1e0;
    iStack_1e0 = (int)ppplStack_110;
    uStack_1dc = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
    if (SBORROW8(0,(longlong)ppplStack_118)) {
      iStack_1e8 = iStack_1e0;
      uStack_1e4 = uStack_1dc;
      iStack_1e0 = iVar27;
      uStack_1dc = uVar28;
      FUN_1428d9760(&UNK_142ad69a0,0x2b,&iStack_1e8,&UNK_142ad6980,&UNK_142ad6d98);
      goto code_r0x000140a79cf7;
    }
    uStack_178 = auStack_b0._8_8_;
    uStack_188 = uStack_b8;
    uStack_180 = auStack_b0._0_8_;
    puStack_198 = puStack_c8;
    uStack_190 = uStack_c0;
    puStack_1a8 = puStack_d8;
    puStack_1a0 = uStack_d0;
    puStack_1b8 = puStack_e8;
    puStack_1b0 = puStack_e0;
    puStack_1c8 = puStack_f8;
    puStack_1c0 = puStack_f0;
    uStack_1d8 = SUB84(ppplStack_108,0);
    uStack_1d4 = (undefined4)((ulonglong)ppplStack_108 >> 0x20);
    puStack_1d0 = puStack_100;
    iStack_1e8 = (int)ppplStack_118;
    uStack_1e4 = (undefined4)((ulonglong)ppplStack_118 >> 0x20);
    uVar32 = FUN_140b1aca0(&iStack_1e8,&UNK_142ad6db0,10,*puStack_150,*param_2);
    uVar33 = FUN_140b3fe40(lStack_98,&UNK_142ad6dba,0x1d,&UNK_142ad6db0,10,*param_6,*param_5);
    FUN_140d146a0(&ppplStack_118,param_7);
    puVar29 = puStack_f8;
    ppplVar37 = ppplStack_110;
    if (ppplStack_110 == (longlong ***)0x0) {
      ppplVar37 = ppplStack_108;
    }
    pplVar1 = ppplVar37[2];
    if ((ulonglong)((longlong)*ppplVar37 - (longlong)pplVar1) < 0x78) {
      FUN_1428a9a60(ppplVar37,pplVar1,0x78,1,1);
    }
    pplVar36 = ppplVar37[1];
    *(code **)((longlong)pplVar36 + (longlong)pplVar1) = FUN_1412dd460;
    uVar6 = _UNK_14293e0c8;
    *(undefined8 *)((longlong)(pplVar36 + 1) + (longlong)pplVar1) = _UNK_14293e0c0;
    ((undefined8 *)((longlong)(pplVar36 + 1) + (longlong)pplVar1))[1] = uVar6;
    *(undefined8 *)((longlong)(pplVar36 + 3) + (longlong)pplVar1) = 0;
    *(undefined4 *)((longlong)(pplVar36 + 4) + (longlong)pplVar1) = 0;
    *(undefined8 *)((longlong)pplVar36 + 0x24 + (longlong)pplVar1) = 0x3f8000003f800000;
    *(undefined4 *)((longlong)pplVar36 + 0x2c + (longlong)pplVar1) = 0x3f800000;
    *(undefined8 *)((longlong)(pplVar36 + 7) + (longlong)pplVar1) = uVar32;
    *(undefined8 *)((longlong)(pplVar36 + 8) + (longlong)pplVar1) = uVar33;
    *(undefined1 *)((longlong)(pplVar36 + 9) + (longlong)pplVar1) = 0;
    *(undefined4 **)((longlong)(pplVar36 + 0xb) + (longlong)pplVar1) = puVar29;
    *(code **)((longlong)(pplVar36 + 0xd) + (longlong)pplVar1) = FUN_141564d30;
    ppplVar37[2] = pplVar1 + 0xf;
    cVar30 = *(char *)(*plStack_1f0 + 0x58);
  }
  plVar42 = plStack_1f0;
  if (cVar30 == '\x01') {
    *(undefined4 *)param_8[2] = *(undefined4 *)((longlong)param_8 + 0x1c);
    FUN_1415539d0(auStack_2d8,*param_8,lStack_98,*plStack_1f0,param_7,*puStack_150,*param_2,*param_5
                 );
    FUN_1412dfbb0(param_7,auStack_2d8,&UNK_142ad6dd8);
    if (*(char *)(*plVar42 + 0x58) != '\x01') goto code_r0x000140a78e84;
code_r0x000140a78e04:
    uVar32 = 5;
    puVar46 = &UNK_142aef4d6;
  }
  else {
    if (cVar30 == '\x02') {
      FUN_141553120(puStack_150,param_2,param_7,param_5,param_8,lStack_98,plStack_1f0);
    }
    if (*(char *)(*plVar42 + 0x58) == '\x01') goto code_r0x000140a78e04;
code_r0x000140a78e84:
    plVar42 = *(longlong **)(param_7 + 8);
    if (plVar42 == (longlong *)0x0) {
      plVar42 = *(longlong **)(param_7 + 0x10);
    }
    lVar34 = plVar42[2];
    if ((ulonglong)(*plVar42 - lVar34) < 8) {
      FUN_1428a9a60(plVar42,lVar34,8,1,1);
    }
    *(code **)(plVar42[1] + lVar34) = FUN_1412dc6f0;
    plVar42[2] = lVar34 + 8;
    lVar34 = (ulonglong)*(byte *)(*plStack_1f0 + 0x58) * 8;
    puVar46 = *(undefined **)(&UNK_142ad7730 + lVar34);
    uVar32 = *(undefined8 *)(&UNK_142ad7758 + lVar34);
  }
  puVar38 = &UNK_142aef51a;
  FUN_140d62810(&ppplStack_118,&UNK_142aef51a,0xc,puVar46,uVar32);
  uVar32 = FUN_140d76ed0(&ppplStack_118);
  FUN_140d62810(&iStack_1e8,uVar32,puVar38,&UNK_142ad6df0,10);
  uVar33 = FUN_140d76ed0(&iStack_1e8);
  FUN_140d59140(&pplStack_148,uVar33,uVar32);
  if (CONCAT44(uStack_1e4,iStack_1e8) != 0) {
    func_0x000140613c20(CONCAT44(uStack_1dc,iStack_1e0),CONCAT44(uStack_1e4,iStack_1e8),1);
  }
  pplStack_80 = pplStack_148;
  if (SBORROW8(0,(longlong)pplStack_148)) {
    iStack_1e8 = (int)ppplStack_140;
    uStack_1e4 = (undefined4)((ulonglong)ppplStack_140 >> 0x20);
    FUN_1428d9760(&UNK_142ad6df0,10,&iStack_1e8,&UNK_142ad6940,&UNK_142ad6e00);
    goto code_r0x000140a79cf7;
  }
  ppplStack_90 = ppplStack_140;
  FUN_1414062b0(&plStack_220,ppplStack_140,lStack_138);
  if (SBORROW8(0,(longlong)plStack_220)) {
    iStack_1e8 = (int)plStack_218;
    uStack_1e4 = (undefined4)((ulonglong)plStack_218 >> 0x20);
    FUN_1428d9760(&UNK_142ad6e18,0x11,&iStack_1e8,&UNK_142ad6960,&UNK_142ad6e30);
    goto code_r0x000140a79cf7;
  }
  plStack_258 = plStack_210;
  plStack_268 = plStack_220;
  plStack_260 = plStack_218;
  if (pplStack_80 != (longlong **)0x0) {
    func_0x000140613c20(ppplStack_90,pplStack_80,1);
  }
  if (ppplStack_118 != (longlong ***)0x0) {
    func_0x000140613c20(ppplStack_110,ppplStack_118,1);
  }
  uStack_170 = 0;
  lStack_168 = 4;
  uStack_160 = 0;
  uStack_224 = *(undefined4 *)((longlong)param_11 + 0x1c);
  puStack_278 = (undefined4 *)param_11[2];
  *puStack_278 = uStack_224;
  lStack_240 = *param_11;
  ppplVar37 = *(longlong ****)(lStack_240 + 8);
  pplVar1 = *(longlong ***)(lStack_240 + 0x10);
  *(undefined8 *)(lStack_240 + 0x10) = 0;
  ppplStack_90 = ppplVar37;
  pplStack_88 = pplVar1;
  if (pplVar1 != (longlong **)0x0) {
    pplVar36 = (longlong **)0x1;
    do {
      pplVar2 = *ppplVar37;
      LOCK();
      *pplVar2 = (longlong *)((longlong)*pplVar2 + -1);
      UNLOCK();
      pplStack_80 = pplVar36;
      if (*pplVar2 == (longlong *)0x0) {
        func_0x00014064eb30(ppplVar37);
      }
      pplVar36 = (longlong **)((longlong)pplStack_80 + 1);
      ppplVar37 = ppplVar37 + 10;
    } while (pplStack_80 != pplVar1);
  }
  *(undefined1 *)(lStack_240 + 0x18) = 0;
  cVar26 = UNK_142a95d93;
  cVar25 = UNK_142a95d92;
  cVar24 = UNK_142a95d91;
  cVar23 = UNK_142a95d90;
  cVar22 = UNK_142a95d8f;
  cVar21 = UNK_142a95d8e;
  cVar20 = UNK_142a95d8d;
  cVar19 = UNK_142a95d8c;
  cVar18 = UNK_142a95d8b;
  cVar17 = UNK_142a95d8a;
  cVar16 = UNK_142a95d89;
  cVar15 = UNK_142a95d88;
  cVar14 = UNK_142a95d87;
  cVar13 = UNK_142a95d86;
  cVar12 = UNK_142a95d85;
  cVar11 = UNK_142a95d84;
  cVar10 = UNK_142a95d83;
  cVar9 = UNK_142a95d82;
  cVar8 = UNK_142a95d81;
  cVar30 = UNK_142a95d80;
  plStack_208 = plStack_260 + (longlong)plStack_258 * 10;
  plStack_220 = plStack_260;
  plStack_218 = plStack_260;
  plStack_210 = plStack_268;
  puStack_270 = param_2;
  while (plVar42 = plStack_218, plStack_218 != plStack_208) {
    plVar42 = plStack_218 + 10;
    lVar34 = *plStack_218;
    if (SBORROW8(0,lVar34)) break;
    pplStack_88 = (longlong **)plStack_218[1];
    lVar39 = plStack_218[2];
    lStack_200 = plStack_218[3];
    lStack_158 = plStack_218[4];
    lVar3 = plStack_218[5];
    uVar45 = plStack_218[6];
    lStack_120 = plStack_218[7];
    lVar4 = plStack_218[8];
    lVar7 = plStack_218[9];
    plStack_218 = plVar42;
    lStack_1f8 = lVar34;
    if ((lVar39 == 0x12) &&
       (auVar50[0] = -(*(char *)pplStack_88 == cVar30),
       auVar50[1] = -(*(char *)((longlong)pplStack_88 + 1) == cVar8),
       auVar50[2] = -(*(char *)((longlong)pplStack_88 + 2) == cVar9),
       auVar50[3] = -(*(char *)((longlong)pplStack_88 + 3) == cVar10),
       auVar50[4] = -(*(char *)((longlong)pplStack_88 + 4) == cVar11),
       auVar50[5] = -(*(char *)((longlong)pplStack_88 + 5) == cVar12),
       auVar50[6] = -(*(char *)((longlong)pplStack_88 + 6) == cVar13),
       auVar50[7] = -(*(char *)((longlong)pplStack_88 + 7) == cVar14),
       auVar50[8] = -(*(char *)(pplStack_88 + 1) == cVar15),
       auVar50[9] = -(*(char *)((longlong)pplStack_88 + 9) == cVar16),
       auVar50[10] = -(*(char *)((longlong)pplStack_88 + 10) == cVar17),
       auVar50[0xb] = -(*(char *)((longlong)pplStack_88 + 0xb) == cVar18),
       auVar50[0xc] = -(*(char *)((longlong)pplStack_88 + 0xc) == cVar19),
       auVar50[0xd] = -(*(char *)((longlong)pplStack_88 + 0xd) == cVar20),
       auVar50[0xe] = -(*(char *)((longlong)pplStack_88 + 0xe) == cVar21),
       auVar50[0xf] = -(*(char *)((longlong)pplStack_88 + 0xf) == cVar22),
       auVar51[0] = -((char)*(undefined2 *)(pplStack_88 + 2) == cVar23),
       auVar51[1] = -((char)((ushort)*(undefined2 *)(pplStack_88 + 2) >> 8) == cVar24),
       auVar51[2] = -(cVar25 == '\0'), auVar51[3] = -(cVar26 == '\0'), auVar51[4] = 0xff,
       auVar51[5] = 0xff, auVar51[6] = 0xff, auVar51[7] = 0xff, auVar51[8] = 0xff, auVar51[9] = 0xff
       , auVar51[10] = 0xff, auVar51[0xb] = 0xff, auVar51[0xc] = 0xff, auVar51[0xd] = 0xff,
       auVar51[0xe] = 0xff, auVar51[0xf] = 0xff, auVar51 = auVar51 & auVar50,
       (ushort)((ushort)(SUB161(auVar51 >> 7,0) & 1) | (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar51 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar51 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar51 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar51 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar51 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar51 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar51 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar51 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar51 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar51 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar51 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar51[0xf] >> 7) << 0xf) == 0xffff)) {
      if (lVar34 != 0) {
        func_0x000140613c20(pplStack_88,lVar34,1);
      }
      if (lStack_200 != 0) {
        func_0x000140613c20(lStack_158,lStack_200,1);
      }
      goto code_r0x000140a79630;
    }
    *(undefined4 *)param_8[2] = *(undefined4 *)((longlong)param_8 + 0x1c);
    uVar32 = *param_8;
    lVar34 = (ulonglong)*(byte *)(*plStack_1f0 + 0x58) * 8;
    uStack_72 = 1;
    uStack_128 = uVar45;
    FUN_140d62810(&ppplStack_118,&UNK_142aef51a,0xc,*(undefined8 *)(&UNK_142ad7730 + lVar34),
                  *(undefined8 *)(&UNK_142ad7758 + lVar34));
    uStack_72 = 1;
    FUN_1412e68e0(&iStack_1e8,lStack_158,lVar3,&ppplStack_118);
    uStack_72 = 1;
    FUN_1409365d0(&pplStack_148,uVar32,lStack_98,&iStack_1e8);
    cStack_73 = '\x01';
    pplStack_238 = (longlong **)FUN_141556080(pplStack_88);
    *puStack_278 = uStack_224;
    ppplStack_110 = ppplStack_140 + lStack_138 * 6;
    ppplStack_118 = ppplStack_140;
    ppplStack_108 = &pplStack_238;
    puStack_100 = &uStack_228;
    uStack_74 = 1;
    lStack_230 = lVar39;
    uStack_228 = (int)lVar7;
    FUN_1412fa010(lStack_240,&ppplStack_118,&UNK_142ad6eb0);
    cVar41 = '\x01';
    if (SBORROW8(0,uStack_128)) {
code_r0x000140a79557:
      LOCK();
      *pplStack_238 = (longlong *)((longlong)*pplStack_238 + -1);
      plVar42 = *pplStack_238;
      UNLOCK();
      goto joined_r0x000140a7955f;
    }
    FUN_1412e6ad0(&iStack_1e8,lStack_120,lVar4,&UNK_142ad6ec8,0xd);
    piVar40 = &iStack_1e8;
    FUN_140b0fe60(&ppplStack_118,&iStack_1e8);
    uVar28 = uStack_1dc;
    iVar27 = iStack_1e0;
    iStack_1e0 = (int)ppplStack_110;
    uStack_1dc = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
    if (SBORROW8(0,(longlong)ppplStack_118)) {
      iStack_1e8 = iStack_1e0;
      uStack_1e4 = uStack_1dc;
      iStack_1e0 = iVar27;
      uStack_1dc = uVar28;
      FUN_1428d9760(&UNK_142ad69a0,0x2b,&iStack_1e8,&UNK_142ad6980,&UNK_142ad6ed8);
      goto code_r0x000140a79cf7;
    }
    uStack_178 = auStack_b0._8_8_;
    uStack_188 = uStack_b8;
    uStack_180 = auStack_b0._0_8_;
    puStack_198 = puStack_c8;
    uStack_190 = uStack_c0;
    puStack_1a8 = puStack_d8;
    puStack_1a0 = uStack_d0;
    puStack_1b8 = puStack_e8;
    puStack_1b0 = puStack_e0;
    puStack_1c8 = puStack_f8;
    puStack_1c0 = puStack_f0;
    uStack_1d8 = SUB84(ppplStack_108,0);
    uStack_1d4 = (undefined4)((ulonglong)ppplStack_108 >> 0x20);
    puStack_1d0 = puStack_100;
    iStack_1e8 = (int)ppplStack_118;
    uStack_1e4 = (undefined4)((ulonglong)ppplStack_118 >> 0x20);
    uVar32 = FUN_140a07ca0(&iStack_1e8);
    FUN_1412612d0(&lStack_290,uVar32,piVar40,&UNK_142ad6928);
    FUN_140888b30(&iStack_1e8);
    ppplVar48 = ppplStack_140 + lStack_138 * 6;
    pplStack_80 = pplStack_280;
    ppplStack_90 = ppplStack_288;
    ppplVar37 = (longlong ***)((longlong)ppplStack_288 + (longlong)pplStack_280 * 0xc);
    for (ppplVar49 = ppplStack_140; ppplVar49 != ppplVar48; ppplVar49 = ppplVar49 + 6) {
      pppplStack_248 = &ppplStack_108;
      ppplVar43 = ppplStack_90;
      ppplVar47 = ppplVar37;
      ppplStack_110 = ppplVar37;
      ppplStack_108 = ppplVar49;
      if (pplStack_80 != (longlong **)0x0) {
        do {
          while( true ) {
            ppplVar44 = (longlong ***)((longlong)ppplVar43 + 0xc);
            ppplStack_118 = ppplVar44;
            FUN_140a123a0(&iStack_1e8,&pppplStack_248,ppplVar43);
            uVar45 = uStack_160;
            if (iStack_1e8 == 1) break;
            ppplVar43 = ppplVar44;
            if (ppplVar44 == ppplVar47) goto code_r0x000140a79410;
          }
          uStack_2a8 = CONCAT44(iStack_1e0,uStack_1e4);
          uStack_2a0 = CONCAT44(uStack_1d8,uStack_1dc);
          if (uStack_160 == uStack_170) {
            FUN_1428a9a60(&uStack_170,uStack_160,1,4,0x10);
          }
          puVar31 = (undefined8 *)(lStack_168 + uVar45 * 0x10);
          *puVar31 = uStack_2a8;
          puVar31[1] = uStack_2a0;
          uStack_160 = uVar45 + 1;
          pppplStack_248 = &ppplStack_108;
          ppplVar43 = ppplStack_118;
          ppplVar47 = ppplStack_110;
        } while (ppplStack_118 != ppplStack_110);
      }
code_r0x000140a79410:
    }
    if (lStack_290 != 0) {
      func_0x000140613c20(ppplStack_90,lStack_290 * 0xc,4);
    }
    if (uStack_128 != 0) {
      func_0x000140613c20(lStack_120,uStack_128,1);
      cVar41 = '\0';
      goto code_r0x000140a79557;
    }
    cVar41 = '\0';
    LOCK();
    *pplStack_238 = (longlong *)((longlong)*pplStack_238 + -1);
    plVar42 = *pplStack_238;
    UNLOCK();
joined_r0x000140a7955f:
    if (plVar42 == (longlong *)0x0) {
      cStack_73 = cVar41;
      func_0x00014064eb30(&pplStack_238);
    }
    if (pplStack_148 != (longlong **)0x0) {
      func_0x000140613c20(ppplStack_140,(longlong)pplStack_148 * 0x30,4);
    }
    if (lStack_1f8 != 0) {
      func_0x000140613c20(pplStack_88,lStack_1f8,1);
    }
    if (lStack_200 != 0) {
      func_0x000140613c20(lStack_158,lStack_200,1);
    }
    uVar45 = uStack_128;
    if (cVar41 != '\0') {
code_r0x000140a79630:
      if ((uVar45 & 0x7fffffffffffffff) != 0) {
        func_0x000140613c20(lStack_120,uVar45,1);
      }
    }
  }
  plStack_218 = plVar42;
  uStack_71 = 0;
  FUN_140879230(&plStack_220);
  *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
  uStack_71 = 0;
  lVar34 = FUN_1412c7380(*param_9);
  uVar45 = uStack_160;
  if ((lVar34 == 0) || (uStack_160 == 0)) {
code_r0x000140a79739:
    uVar35 = 0x44c;
    if (uVar45 < 0x44c) {
      uVar35 = uVar45;
    }
    *(undefined4 *)param_10[2] = *(undefined4 *)((longlong)param_10 + 0x1c);
    *(ulonglong *)*param_10 = uVar35;
    uVar32 = *param_6;
    uVar33 = *param_5;
    plStack_220 = (longlong *)&UNK_142ad6e60;
    plStack_218 = (longlong *)0x11;
    ppplStack_140 = (longlong ***)&UNK_140109e90;
    ppplStack_118 = (longlong ***)&UNK_142ad6ca0;
    ppplStack_110 = (longlong ***)0x2;
    ppplStack_108 = &pplStack_148;
    puStack_100 = (undefined4 *)0x1;
    puStack_f8 = (undefined4 *)0x0;
    uStack_71 = 0;
    pplStack_148 = &plStack_220;
    FUN_140d79f90(&iStack_1e8,&ppplStack_118);
    pplStack_80 = (longlong **)CONCAT44(uStack_1dc,iStack_1e0);
    uVar6 = CONCAT44(uStack_1d4,uStack_1d8);
    ppplStack_140 = (longlong ***)&UNK_140109e90;
    ppplStack_118 = (longlong ***)&UNK_142ad6cc8;
    ppplStack_110 = (longlong ***)0x2;
    ppplStack_108 = &pplStack_148;
    puStack_100 = (undefined4 *)0x1;
    puStack_f8 = (undefined4 *)0x0;
    pplStack_148 = &plStack_220;
    FUN_140d79f90(auStack_2c0,&ppplStack_118);
    uVar32 = FUN_140b40210(lStack_98,pplStack_80,uVar6,auStack_2c0,uVar32,uVar33);
    if (CONCAT44(uStack_1e4,iStack_1e8) != 0) {
      func_0x000140613c20(pplStack_80,CONCAT44(uStack_1e4,iStack_1e8),1);
    }
    uStack_71 = 0;
    FUN_140b0eeb0(&ppplStack_118,&UNK_142ad6e71,0x24);
    uVar28 = uStack_1dc;
    iVar27 = iStack_1e0;
    iStack_1e0 = (int)ppplStack_110;
    uStack_1dc = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
    if (!SBORROW8(0,(longlong)ppplStack_118)) {
      uStack_178 = auStack_b0._8_8_;
      uStack_188 = uStack_b8;
      uStack_180 = auStack_b0._0_8_;
      puStack_198 = puStack_c8;
      uStack_190 = uStack_c0;
      puStack_1a8 = puStack_d8;
      puStack_1a0 = uStack_d0;
      puStack_1b8 = puStack_e8;
      puStack_1b0 = puStack_e0;
      puStack_1c8 = puStack_f8;
      puStack_1c0 = puStack_f0;
      uStack_1d8 = SUB84(ppplStack_108,0);
      uStack_1d4 = (undefined4)((ulonglong)ppplStack_108 >> 0x20);
      puStack_1d0 = puStack_100;
      iStack_1e8 = (int)ppplStack_118;
      uStack_1e4 = (undefined4)((ulonglong)ppplStack_118 >> 0x20);
      uStack_71 = 0;
      uVar33 = FUN_140b1aca0(&iStack_1e8,&UNK_142ad6e60,0x11,*puStack_150,*puStack_270);
      uStack_71 = 0;
      FUN_140d146a0(&ppplStack_118,param_7);
      puVar29 = puStack_f8;
      ppplVar37 = ppplStack_110;
      if (ppplStack_110 == (longlong ***)0x0) {
        ppplVar37 = ppplStack_108;
      }
      pplVar1 = ppplVar37[2];
      if ((ulonglong)((longlong)*ppplVar37 - (longlong)pplVar1) < 0x78) {
        uStack_71 = 0;
        FUN_1428a9a60(ppplVar37,pplVar1,0x78,1,1);
      }
      pplVar36 = ppplVar37[1];
      *(code **)((longlong)pplVar36 + (longlong)pplVar1) = FUN_1412dd460;
      uVar6 = _UNK_14293e0c8;
      *(undefined8 *)((longlong)(pplVar36 + 1) + (longlong)pplVar1) = _UNK_14293e0c0;
      ((undefined8 *)((longlong)(pplVar36 + 1) + (longlong)pplVar1))[1] = uVar6;
      *(undefined8 *)((longlong)(pplVar36 + 3) + (longlong)pplVar1) = 0;
      *(undefined4 *)((longlong)(pplVar36 + 4) + (longlong)pplVar1) = 0;
      *(undefined8 *)((longlong)pplVar36 + 0x24 + (longlong)pplVar1) = 0x3f8000003f800000;
      *(undefined4 *)((longlong)pplVar36 + 0x2c + (longlong)pplVar1) = 0x3f800000;
      *(undefined8 *)((longlong)(pplVar36 + 7) + (longlong)pplVar1) = uVar33;
      *(undefined8 *)((longlong)(pplVar36 + 8) + (longlong)pplVar1) = uVar32;
      *(undefined1 *)((longlong)(pplVar36 + 9) + (longlong)pplVar1) = 0;
      *(undefined4 **)((longlong)(pplVar36 + 0xb) + (longlong)pplVar1) = puVar29;
      *(code **)((longlong)(pplVar36 + 0xd) + (longlong)pplVar1) = FUN_141564d30;
      ppplVar37[2] = pplVar1 + 0xf;
      if (uStack_170 != 0) {
        func_0x000140613c20(lStack_168,uStack_170 << 4,4);
      }
      return;
    }
    iStack_1e8 = iStack_1e0;
    uStack_1e4 = uStack_1dc;
    iStack_1e0 = iVar27;
    uStack_1dc = uVar28;
    FUN_1428d9760(&UNK_142ad69a0,0x2b,&iStack_1e8,&UNK_142ad6980,&UNK_142ad6e98);
  }
  else {
    ppplVar37 = (longlong ***)(uStack_160 << 4);
    if (*(longlong ****)(lVar34 + 0x18) < ppplVar37) {
      ppplStack_118 = (longlong ***)0xb;
    }
    else {
      pplStack_148 = (longlong **)0x0;
      uStack_71 = 0;
      ppplStack_140 = ppplVar37;
      FUN_140beefa0(&ppplStack_118,lStack_98,lVar34,lStack_168,ppplVar37,&pplStack_148,1,0);
      if (ppplStack_118 == (longlong ***)0xb) goto code_r0x000140a79739;
      puStack_1a8 = uStack_d0;
      puStack_1b8 = puStack_e0;
      puStack_1b0 = puStack_d8;
      puStack_1c8 = puStack_f0;
      puStack_1c0 = puStack_e8;
      uStack_1d8 = SUB84(puStack_100,0);
      uStack_1d4 = (undefined4)((ulonglong)puStack_100 >> 0x20);
      puStack_1d0 = puStack_f8;
      iStack_1e8 = (int)ppplStack_110;
      uStack_1e4 = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
      iStack_1e0 = (int)ppplStack_108;
      uStack_1dc = (undefined4)((ulonglong)ppplStack_108 >> 0x20);
    }
    ppplStack_110 = (longlong ***)CONCAT44(uStack_1e4,iStack_1e8);
    ppplStack_108 = (longlong ***)CONCAT44(uStack_1dc,iStack_1e0);
    puStack_100 = (undefined4 *)CONCAT44(uStack_1d4,uStack_1d8);
    puStack_f8 = puStack_1d0;
    puStack_f0 = puStack_1c8;
    puStack_e8 = puStack_1c0;
    puStack_e0 = puStack_1b8;
    puStack_d8 = puStack_1b0;
    uStack_d0 = puStack_1a8;
    FUN_1428d9760(&UNK_142ad69a0,0x2b,&ppplStack_118,&UNK_142ad69d0,&UNK_142ad6e48);
  }
code_r0x000140a79cf7:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

