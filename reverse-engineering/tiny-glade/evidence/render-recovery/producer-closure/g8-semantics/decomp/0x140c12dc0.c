
/* WARNING: Removing unreachable block (ram,0x000140c13a94) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140c12dc0(longlong *param_1,longlong param_2,longlong *param_3)

{
  char *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  longlong lVar8;
  uint uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  longlong ***ppplVar13;
  longlong ****pppplVar14;
  int iVar15;
  uint *puVar16;
  longlong ****pppplVar17;
  undefined4 *puVar18;
  code *pcVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong uVar23;
  char cVar24;
  undefined8 *puVar25;
  undefined8 uVar26;
  ulonglong *puVar27;
  undefined8 *puVar28;
  longlong lVar29;
  ulonglong uVar30;
  longlong lVar31;
  uint uVar32;
  longlong lVar33;
  uint *puVar34;
  code *pcVar35;
  longlong ****pppplVar36;
  longlong ****pppplVar37;
  longlong ****pppplVar38;
  longlong ****pppplVar39;
  bool bVar40;
  uint *in_stack_fffffffffffffdb8;
  longlong **pplStack_220;
  longlong ***ppplStack_218;
  longlong lStack_210;
  ulonglong uStack_208;
  longlong lStack_200;
  longlong lStack_1f8;
  longlong ***ppplStack_1f0;
  longlong ***ppplStack_1e8;
  ulonglong *puStack_1e0;
  longlong ***ppplStack_1d8;
  undefined8 uStack_1d0;
  longlong lStack_1c8;
  longlong lStack_1c0;
  code *pcStack_1b8;
  undefined8 uStack_1b0;
  uint auStack_1a8 [2];
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  uint uStack_188;
  uint uStack_184;
  uint uStack_180;
  uint uStack_17c;
  uint uStack_178;
  uint uStack_174;
  uint uStack_170;
  uint uStack_16c;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined *puStack_158;
  undefined8 uStack_150;
  undefined8 *puStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  longlong ***ppplStack_120;
  undefined2 uStack_113;
  undefined1 uStack_111;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  longlong ***ppplStack_e0;
  longlong *plStack_d8;
  undefined4 uStack_d0;
  undefined3 uStack_cc;
  longlong **pplStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  undefined4 uStack_ac;
  uint *puStack_a8;
  longlong ***ppplStack_a0;
  uint *puStack_98;
  undefined8 *puStack_90;
  ulonglong uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  bool bStack_4d;
  char cStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  ppplStack_218 = (longlong ***)param_3[4];
  pplStack_220 = (longlong **)param_3[5];
  plStack_d8 = param_1;
  pplStack_c8 = pplStack_220;
  lStack_b8 = param_2;
  ppplStack_a0 = ppplStack_218;
  if (*(char *)(param_2 + 0x61a) == '\0') {
    uStack_80 = (longlong ****)0x1;
  }
  else {
    FUN_140d56a30(&uStack_80);
  }
  auStack_1a8[0] = 1;
  auStack_1a8[1] = 0;
  uStack_1a0 = (undefined *)0x1;
  uStack_198 = uStack_80;
  uStack_190 = uStack_78;
  uStack_188 = (uint)uStack_70;
  uStack_184 = uStack_70._4_4_;
  uStack_180 = (uint)uStack_68;
  uStack_17c = uStack_68._4_4_;
  uStack_178 = (uint)uStack_60;
  uStack_174 = uStack_60._4_4_;
  uStack_170 = (uint)uStack_58;
  uStack_16c = uStack_58._4_4_;
  puVar16 = (uint *)func_0x000140613c10(0x40,8);
  if (puVar16 == (uint *)0x0) {
    FUN_1428d9000(8,0x40);
code_r0x000140c14229:
                    /* WARNING: Does not return */
    pcVar35 = (code *)invalidInstructionException();
    (*pcVar35)();
  }
  puVar16[0xc] = uStack_178;
  puVar16[0xd] = uStack_174;
  puVar16[0xe] = uStack_170;
  puVar16[0xf] = uStack_16c;
  puVar16[8] = uStack_188;
  puVar16[9] = uStack_184;
  puVar16[10] = uStack_180;
  puVar16[0xb] = uStack_17c;
  puVar16[4] = (uint)uStack_198;
  puVar16[5] = uStack_198._4_4_;
  puVar16[6] = (uint)uStack_190;
  puVar16[7] = uStack_190._4_4_;
  *puVar16 = auStack_1a8[0];
  puVar16[1] = auStack_1a8[1];
  puVar16[2] = (uint)uStack_1a0;
  puVar16[3] = uStack_1a0._4_4_;
  puVar27 = puRam000000014362c3a8;
  puStack_a8 = puVar16;
  if (*(char *)(lStack_b8 + 0x61b) != '\0') {
    if (3 < *puRam000000014362c3a8) {
      uStack_110 = (longlong ****)(param_3 + 2);
      uStack_80 = (longlong ****)&uStack_110;
      uStack_78 = (code *)&UNK_140533930;
      uStack_70 = &ppplStack_218;
      uStack_68 = FUN_140d8d6c0;
      uStack_60 = (longlong ****)&pplStack_220;
      uStack_58 = FUN_140d8d6c0;
      puVar16 = (uint *)func_0x00014067dd80(&UNK_142b1b448);
      uStack_188 = *puVar16;
      uStack_184 = puVar16[1];
      uStack_180 = puVar16[2];
      uStack_17c = puVar16[3];
      uStack_178 = 4;
      uStack_174 = 0;
      uStack_170 = 0x42b1b078;
      uStack_16c = 1;
      uStack_168 = 0x15;
      puStack_158 = &UNK_142b1b408;
      uStack_150 = 4;
      puStack_148 = &uStack_80;
      uStack_140 = 3;
      uStack_138 = 0;
      auStack_1a8[0] = 0;
      auStack_1a8[1] = 0;
      uStack_1a0 = &UNK_142b1b078;
      uStack_198 = (longlong ****)0x15;
      uStack_190 = (code *)0x0;
      uStack_160 = CONCAT44(puVar16[4],1);
      func_0x000140d4a330(&puStack_98,auStack_1a8);
    }
    if (*(char *)(lStack_b8 + 0x61d) != '\0') {
      FUN_140d56a30(&uStack_80);
      if (3 < *puVar27) {
        uStack_108 = FUN_140d56e50;
        uStack_110 = (longlong ****)&uStack_80;
        puVar16 = (uint *)func_0x00014067dd80(&UNK_142b1b488);
        uStack_188 = *puVar16;
        uStack_184 = puVar16[1];
        uStack_180 = puVar16[2];
        uStack_17c = puVar16[3];
        uStack_178 = 4;
        uStack_174 = 0;
        uStack_170 = 0x42b1b078;
        uStack_16c = 1;
        uStack_168 = 0x15;
        puStack_158 = &UNK_142b1b478;
        uStack_150 = 1;
        puStack_148 = &uStack_110;
        uStack_140 = 1;
        uStack_138 = 0;
        auStack_1a8[0] = 0;
        auStack_1a8[1] = 0;
        uStack_1a0 = &UNK_142b1b078;
        uStack_198 = (longlong ****)0x15;
        uStack_190 = (code *)0x0;
        uStack_160 = CONCAT44(puVar16[4],1);
        func_0x000140d4a330(&puStack_98,auStack_1a8);
      }
      FUN_1418beca0(&uStack_80);
    }
  }
  if (((longlong ****)ppplStack_218 == (longlong ****)0x0) ||
     (((ulonglong)pplStack_220 ^ (longlong)pplStack_220 - 1U) <= (longlong)pplStack_220 - 1U)) {
    *plStack_d8 = 3;
    goto code_r0x000140c13e4f;
  }
  lVar31 = (ulonglong)*(byte *)((longlong)param_3 + 0x39) * 4;
  uVar4 = *(uint *)(&UNK_142b1b6e4 + lVar31);
  uVar32 = *(uint *)(&UNK_142b1b6f8 + lVar31);
  uVar5 = *(uint *)(&UNK_142b1b70c + lVar31);
  uStack_138 = 0;
  uStack_130 = 0;
  puStack_148 = (undefined8 *)0x0;
  uStack_140 = 0;
  puStack_158 = (undefined *)0x0;
  uStack_150 = 0;
  uStack_168 = 0;
  uStack_160 = 0;
  uStack_178 = 0;
  uStack_174 = 0;
  uStack_170 = 0;
  uStack_16c = 0;
  uStack_188 = 0;
  uStack_184 = 0;
  uStack_180 = 0;
  uStack_17c = 0;
  uStack_198 = (longlong ****)0x0;
  uStack_190 = (code *)0x0;
  auStack_1a8[0] = 0;
  auStack_1a8[1] = 0;
  uStack_1a0 = (undefined *)0x0;
  if (*(longlong *)(lStack_b8 + 0x10) == 0) {
code_r0x000140c13299:
    *plStack_d8 = 2;
    goto code_r0x000140c13e4f;
  }
  lVar33 = *(longlong *)(lStack_b8 + 0x10) * 0x38 + *(longlong *)(lStack_b8 + 8);
  uVar6 = *(uint *)(param_3 + 6);
  lVar31 = *(longlong *)(lStack_b8 + 0x20);
  uVar23 = *(ulonglong *)(lStack_b8 + 0x28);
  uVar20 = 0;
  lVar22 = *(longlong *)(lStack_b8 + 8);
  do {
    while( true ) {
      lVar29 = lVar22 + 0x38;
      if ((uVar6 >> (*(uint *)(lVar22 + 0x18) & 0x1f) & 1) != 0) break;
code_r0x000140c131c0:
      lVar22 = lVar29;
      if (lVar29 == lVar33) goto code_r0x000140c13241;
    }
    uVar21 = *(ulonglong *)(lVar22 + 0x20);
    if (uVar23 <= uVar21) {
      FUN_1428d9518(uVar21,uVar23,&UNK_142d0bcc0);
      goto code_r0x000140c14229;
    }
    if ((*(longlong *****)(lVar31 + uVar21 * 0x10) < ppplStack_a0) ||
       (uVar7 = *(uint *)(lVar22 + 0x30), (uVar7 | uVar32) != 0xffffffff))
    goto code_r0x000140c131c0;
    if (uVar20 == 0x20) {
      FUN_1428d9518(0x20,0x20,&UNK_142b1b5a8);
      goto code_r0x000140c14229;
    }
    iVar15 = 0;
    if ((uVar7 & uVar5) == 0) {
      iVar15 = 0xfffe;
    }
    auStack_1a8[uVar20] =
         (iVar15 + -1 + (uint)((uVar7 & uVar4) == 0)) * 0x10000 + 0x640000 |
         *(uint *)(lVar22 + 0x18);
    uVar20 = uVar20 + 1;
    lVar22 = lVar29;
  } while (lVar29 != lVar33);
code_r0x000140c13241:
  if (0x20 < uVar20) {
    FUN_1428d9840(0,0x20,&UNK_142b1b590);
    goto code_r0x000140c14229;
  }
  if (uVar20 < 2) {
    bVar40 = uVar20 == 0;
    uVar20 = 1;
    if (bVar40) goto code_r0x000140c13299;
  }
  else if (uVar20 < 0x15) {
    lVar31 = 4;
    puVar16 = auStack_1a8 + 1;
    do {
      uVar32 = puVar16[-1];
      uVar4 = *puVar16;
      lVar22 = lVar31;
      if (uVar4 < uVar32) {
        do {
          lVar33 = lVar22;
          *(uint *)((longlong)auStack_1a8 + lVar33) = uVar32;
          puVar34 = auStack_1a8;
          if (lVar33 == 4) goto code_r0x000140c13fe7;
          uVar32 = *(uint *)((longlong)&uStack_1b0 + lVar33);
          lVar22 = lVar33 + -4;
        } while (uVar4 < uVar32);
        puVar34 = (uint *)((longlong)&uStack_1b0 + lVar33 + 4);
code_r0x000140c13fe7:
        *puVar34 = uVar4;
      }
      puVar16 = puVar16 + 1;
      lVar31 = lVar31 + 4;
    } while (puVar16 != auStack_1a8 + uVar20);
  }
  else {
    FUN_1418d3ec0(auStack_1a8,uVar20);
  }
  uVar23 = uVar20 - 1 & 0x3fffffffffffffff;
  if (uVar23 < 7) {
    puVar16 = auStack_1a8;
code_r0x000140c13300:
    do {
      *puVar16 = (uint)(ushort)*puVar16;
      puVar16 = puVar16 + 1;
    } while (puVar16 != auStack_1a8 + uVar20);
  }
  else {
    uVar23 = uVar23 + 1;
    uVar21 = uVar23 & 0xfffffffffffffff8;
    puVar16 = auStack_1a8 + uVar21;
    uVar30 = 0;
    do {
      uVar4 = *(uint *)((longlong)&uStack_1a0 + uVar30 * 4);
      uVar32 = *(uint *)((longlong)&uStack_1a0 + uVar30 * 4 + 4);
      lVar31 = uVar30 * 4;
      uVar5 = *(uint *)((longlong)&uStack_198 + lVar31);
      uVar6 = *(uint *)((longlong)&uStack_198 + lVar31 + 4);
      uVar7 = *(uint *)((longlong)&uStack_190 + lVar31);
      uVar9 = *(uint *)((longlong)&uStack_190 + lVar31 + 4);
      auStack_1a8[uVar30] = auStack_1a8[uVar30] & _UNK_142b1ab70;
      auStack_1a8[uVar30 + 1] = auStack_1a8[uVar30 + 1] & _UNK_142b1ab74;
      *(uint *)((longlong)&uStack_1a0 + uVar30 * 4) = uVar4 & _UNK_142b1ab78;
      *(uint *)((longlong)&uStack_1a0 + uVar30 * 4 + 4) = uVar32 & _UNK_142b1ab7c;
      lVar31 = uVar30 * 4;
      *(uint *)((longlong)&uStack_198 + lVar31) = uVar5 & _UNK_142b1ab70;
      *(uint *)((longlong)&uStack_198 + lVar31 + 4) = uVar6 & _UNK_142b1ab74;
      *(uint *)((longlong)&uStack_190 + lVar31) = uVar7 & _UNK_142b1ab78;
      *(uint *)((longlong)&uStack_190 + lVar31 + 4) = uVar9 & _UNK_142b1ab7c;
      uVar30 = uVar30 + 8;
    } while (uVar21 != uVar30);
    if (uVar23 != uVar21) goto code_r0x000140c13300;
  }
  lStack_200 = lStack_b8 + 0x30;
  cStack_4c = '\x02' - (*(byte *)(param_3 + 7) & 1);
  lStack_210 = *param_3;
  lStack_1f8 = param_3[1];
  bStack_4d = lStack_210 != 2;
  lStack_c0 = param_3[2];
  ppplStack_e0 = (longlong ***)param_3[3];
  lVar31 = *(longlong *)(lStack_b8 + 8);
  uStack_208 = *(ulonglong *)(lStack_b8 + 0x10);
  uStack_1d0 = *(undefined8 *)(lStack_b8 + 0x600);
  for (puVar16 = auStack_1a8; puVar16 != auStack_1a8 + uVar20; puVar16 = puVar16 + 1) {
    uVar23 = (ulonglong)*puVar16;
    if (uStack_208 <= uVar23) {
      FUN_1428d9518(uVar23,uStack_208,&UNK_142b1b4a0);
      goto code_r0x000140c14229;
    }
    LOCK();
    lVar22 = *(longlong *)puStack_a8;
    *(longlong *)puStack_a8 = *(longlong *)puStack_a8 + 1;
    UNLOCK();
    if (*(longlong *)puStack_a8 == 0 || SCARRY8(lVar22,1) != *(longlong *)puStack_a8 < 0)
    goto code_r0x000140c14229;
    puVar25 = (undefined8 *)(uVar23 * 0x38 + lVar31);
    puStack_98 = puStack_a8;
    pppplVar17 = *(longlong *****)(lStack_b8 + 0x608 + (ulonglong)(*(uint *)(puVar25 + 6) & 2) * 4);
    if ((lStack_210 == 2) && (ppplStack_a0 <= pppplVar17)) {
      lVar33 = puVar25[1];
      uStack_88 = CONCAT44(uStack_88._4_4_,(int)CONCAT71((int7)((ulonglong)puVar25[2] >> 8),1));
      puStack_90 = (undefined8 *)((ulonglong)puStack_90 & 0xffffffff00000000);
      lVar22 = puVar25[2] * 0x30 + lVar33;
code_r0x000140c13420:
      do {
        lVar29 = lVar22;
        if (lVar33 == lVar29) {
          uStack_49 = 1;
          in_stack_fffffffffffffdb8 =
               (uint *)((ulonglong)in_stack_fffffffffffffdb8 & 0xffffffffffffff00);
          FUN_140c113e0(&uStack_80,lStack_200,pppplVar17,puVar25[3],
                        *(undefined1 *)((longlong)puVar25 + 0x34),
                        *(undefined1 *)((longlong)puVar25 + 0x35),2,lStack_1f8,
                        in_stack_fffffffffffffdb8);
          ppplVar13 = ppplStack_e0;
          if ((char)uStack_58 == '\x02') goto code_r0x000140c137b5;
          uStack_cc = (undefined3)((ulonglong)uStack_58 >> 0x28);
          uStack_d0 = uStack_58._1_4_;
          if (((ulonglong)puStack_90 & 1) == 0) {
            pppplVar36 = (longlong ****)puVar25[2];
            if (pppplVar36 == (longlong ****)*puVar25) {
              FUN_140b9eff0(puVar25,&UNK_142b1b118);
            }
            lVar22 = puVar25[1];
            lVar33 = (longlong)pppplVar36 * 0x30;
            puVar18 = (undefined4 *)(lVar22 + 0x20 + lVar33);
            *puVar18 = (undefined4)uStack_60;
            puVar18[1] = uStack_60._4_4_;
            puVar18[2] = (undefined4)uStack_58;
            puVar18[3] = uStack_58._4_4_;
            puVar18 = (undefined4 *)(lVar22 + 0x10 + lVar33);
            *puVar18 = (undefined4)uStack_70;
            puVar18[1] = uStack_70._4_4_;
            puVar18[2] = (undefined4)uStack_68;
            puVar18[3] = uStack_68._4_4_;
            puVar18 = (undefined4 *)(lVar22 + lVar33);
            *puVar18 = (undefined4)uStack_80;
            puVar18[1] = uStack_80._4_4_;
            puVar18[2] = (undefined4)uStack_78;
            puVar18[3] = uStack_78._4_4_;
            pppplVar17 = (longlong ****)((longlong)pppplVar36 + 1);
            puVar25[2] = pppplVar17;
          }
          else {
            pppplVar17 = (longlong ****)puVar25[2];
            if (pppplVar17 <= ppplStack_120) {
              ppplStack_1d8 = (longlong ***)pppplVar17;
              FUN_1428d9518(ppplStack_120,pppplVar17,&UNK_142b1b100);
              goto code_r0x000140c14229;
            }
            puVar27 = (ulonglong *)(puVar25[1] + (longlong)ppplStack_120 * 0x30);
            pppplVar36 = (longlong ****)ppplStack_120;
            if (*(char *)(puVar25[1] + 0x28 + (longlong)ppplStack_120 * 0x30) != '\x02') {
              uStack_88 = *puVar27;
              puStack_90 = (undefined8 *)puVar27[1];
              puStack_1e0 = puVar27;
              ppplStack_1d8 = (longlong ***)pppplVar17;
              if ((code *)*puStack_90 != (code *)0x0) {
                (*(code *)*puStack_90)(uStack_88);
              }
              pppplVar36 = (longlong ****)ppplStack_120;
              puVar27 = puStack_1e0;
              if (puStack_90[1] != 0) {
                func_0x000140613c20(uStack_88,puStack_90[1],puStack_90[2]);
              }
            }
            *(undefined4 *)(puVar27 + 4) = (undefined4)uStack_60;
            *(uint *)((longlong)puVar27 + 0x24) = uStack_60._4_4_;
            *(undefined4 *)(puVar27 + 5) = (undefined4)uStack_58;
            *(uint *)((longlong)puVar27 + 0x2c) = uStack_58._4_4_;
            *(undefined4 *)(puVar27 + 2) = (undefined4)uStack_70;
            *(uint *)((longlong)puVar27 + 0x14) = uStack_70._4_4_;
            *(undefined4 *)(puVar27 + 3) = (undefined4)uStack_68;
            *(uint *)((longlong)puVar27 + 0x1c) = uStack_68._4_4_;
            *(undefined4 *)puVar27 = (undefined4)uStack_80;
            *(undefined4 *)((longlong)puVar27 + 4) = uStack_80._4_4_;
            *(undefined4 *)(puVar27 + 1) = (undefined4)uStack_78;
            *(undefined4 *)((longlong)puVar27 + 0xc) = uStack_78._4_4_;
            pppplVar17 = (longlong ****)puVar25[2];
          }
          puVar25[5] = puVar25[5] + 1;
          if (pppplVar17 <= pppplVar36) {
            uStack_49 = 1;
            FUN_1428d9518(pppplVar36,pppplVar17,&UNK_142b1b130);
            goto code_r0x000140c14229;
          }
          if (*(char *)(puVar25[1] + 0x28 + (longlong)pppplVar36 * 0x30) == '\x02') {
            pppplVar17 = (longlong ****)func_0x000140613c10(0x19,1);
            if (pppplVar17 != (longlong ****)0x0) goto code_r0x000140c13bf4;
            uStack_49 = 1;
            FUN_1428d8fe3(1,0x19,&UNK_142b1adc8);
            goto code_r0x000140c14229;
          }
          puVar28 = (undefined8 *)(puVar25[1] + (longlong)pppplVar36 * 0x30);
          uStack_49 = 0;
          in_stack_fffffffffffffdb8 = puStack_98;
          (**(code **)(puVar28[1] + 0x40))
                    (&uStack_80,*puVar28,ppplStack_a0,pplStack_c8,cStack_4c,uStack_1d0,lStack_c0,
                     ppplVar13,puStack_98);
          pppplVar38 = uStack_70;
          pcVar35 = uStack_78;
          ppplVar13 = ppplStack_e0;
          if (uStack_80 == (longlong ****)0x0) {
            pppplVar17 = (longlong ****)func_0x000140613c10(0x44,1);
            ppplVar13 = _UNK_142b1b180;
            if (pppplVar17 != (longlong ****)0x0) {
              pppplVar17[6] = _UNK_142b1b178;
              pppplVar17[7] = ppplVar13;
              ppplVar13 = _UNK_142b1b170;
              pppplVar17[4] = _UNK_142b1b168;
              pppplVar17[5] = ppplVar13;
              ppplVar13 = _UNK_142b1b160;
              pppplVar17[2] = _UNK_142b1b158;
              pppplVar17[3] = ppplVar13;
              uVar12 = _UNK_142b1b154;
              uVar11 = _UNK_142b1b150;
              uVar10 = _UNK_142b1b14c;
              *(undefined4 *)pppplVar17 = _UNK_142b1b148;
              *(undefined4 *)((longlong)pppplVar17 + 4) = uVar10;
              *(undefined4 *)(pppplVar17 + 1) = uVar11;
              *(undefined4 *)((longlong)pppplVar17 + 0xc) = uVar12;
              *(undefined4 *)(pppplVar17 + 8) = 0x2e726f74;
              pcVar35 = (code *)0x44;
              pppplVar39 = (longlong ****)0x5;
              goto code_r0x000140c13c42;
            }
            uStack_49 = 0;
            FUN_1428d8fe3(1,0x44,&UNK_142b1adc8);
            goto code_r0x000140c14229;
          }
          pppplVar17 = uStack_70;
          pppplVar39 = uStack_80;
          if ((int)uStack_80 != 9) goto code_r0x000140c13c42;
          pcVar19 = uStack_78 + puVar28[4];
          if (puVar28[4] == 0) {
            pcVar19 = (code *)0x0;
          }
          if ((longlong)ppplStack_e0 < 0) {
            pplStack_c8 = (longlong **)0x0;
code_r0x000140c1420c:
            uStack_49 = 0;
            FUN_1428d8fe3(pplStack_c8,ppplVar13,&UNK_142b1adc8);
            goto code_r0x000140c14229;
          }
          lStack_1c0 = puVar28[2];
          lStack_1c8 = puVar25[3];
          uStack_ac = *(undefined4 *)(puVar25 + 6);
          pcStack_1b8 = pcVar19;
          if ((longlong ****)ppplStack_e0 == (longlong ****)0x0) {
            pcVar19 = (code *)0x1;
          }
          else {
            pplStack_c8 = (longlong **)0x1;
            pcVar19 = (code *)func_0x000140613c10(ppplStack_e0,1);
            if (pcVar19 == (code *)0x0) goto code_r0x000140c1420c;
          }
          func_0x000142923c00(pcVar19,lStack_c0,ppplVar13);
          uStack_110 = (longlong ****)ppplVar13;
          uStack_100 = (longlong ****)ppplVar13;
          uStack_49 = 0;
          uStack_108 = pcVar19;
          uStack_1b0 = (undefined4 *)FUN_1418d31f0(&uStack_110);
          pppplVar17 = (longlong ****)ppplStack_a0;
          cVar24 = '\0';
          goto code_r0x000140c13f4d;
        }
        lVar22 = lVar29 + -0x30;
        pppplVar36 = (longlong ****)(((ulonglong)(lVar22 - lVar33) >> 4) * -0x5555555555555555);
        if (*(char *)(lVar29 + -8) == '\x02') {
          if ((uStack_88 & 1) != 0) {
            ppplStack_120 = (longlong ***)pppplVar36;
          }
          puStack_90 = (undefined8 *)
                       CONCAT44(puStack_90._4_4_,
                                (int)CONCAT71((int7)((ulonglong)ppplStack_120 >> 8),1));
          uStack_88 = uStack_88 & 0xffffffff00000000;
          goto code_r0x000140c13420;
        }
        uVar26 = *(undefined8 *)(lVar29 + -0x30);
        lVar8 = *(longlong *)(lVar29 + -0x28);
        LOCK();
        lVar3 = *(longlong *)puStack_98;
        *(longlong *)puStack_98 = *(longlong *)puStack_98 + 1;
        UNLOCK();
        if (*(longlong *)puStack_98 == 0 || SCARRY8(lVar3,1) != *(longlong *)puStack_98 < 0)
        goto code_r0x000140c14229;
        uStack_49 = 1;
        in_stack_fffffffffffffdb8 = puStack_98;
        (**(code **)(lVar8 + 0x40))
                  (&uStack_110,uVar26,ppplStack_a0,pplStack_c8,cStack_4c,uStack_1d0,lStack_c0,
                   ppplStack_e0,puStack_98);
        ppplVar13 = ppplStack_e0;
        pppplVar38 = uStack_100;
        pcVar35 = uStack_108;
      } while (uStack_110 == (longlong ****)0x0);
      if ((int)uStack_110 == 9) {
        pcVar19 = uStack_108 + *(longlong *)(lVar29 + -0x10);
        if (*(longlong *)(lVar29 + -0x10) == 0) {
          pcVar19 = (code *)0x0;
        }
        if ((longlong)ppplStack_e0 < 0) {
          uVar26 = 0;
code_r0x000140c140f6:
          uStack_49 = 1;
          FUN_1428d8fe3(uVar26,ppplStack_1f0,&UNK_142b1adc8);
          goto code_r0x000140c14229;
        }
        lStack_1c0 = *(longlong *)(lVar29 + -0x20);
        lStack_1c8 = puVar25[3];
        uStack_ac = *(undefined4 *)(puVar25 + 6);
        if ((longlong ****)ppplStack_e0 == (longlong ****)0x0) {
          pppplVar17 = (longlong ****)0x1;
        }
        else {
          uVar26 = 1;
          pppplVar17 = (longlong ****)func_0x000140613c10(ppplStack_e0,1);
          if (pppplVar17 == (longlong ****)0x0) {
            ppplStack_1f0 = ppplVar13;
            goto code_r0x000140c140f6;
          }
          func_0x000142923c00(pppplVar17,lStack_c0,ppplVar13);
        }
        uStack_60 = (longlong ****)ppplVar13;
        uStack_80 = (longlong ****)0x1;
        uStack_78 = (code *)0x1;
        uStack_70 = (longlong ****)ppplVar13;
        ppplStack_1f0 = (longlong ***)pppplVar17;
        pcStack_1b8 = pcVar19;
        uStack_68 = (code *)pppplVar17;
        puVar18 = (undefined4 *)func_0x000140613c10(0x28,8);
        if (puVar18 == (undefined4 *)0x0) {
          FUN_1428d9000(8,0x28);
          goto code_r0x000140c14229;
        }
        *(longlong *****)(puVar18 + 8) = uStack_60;
        *(longlong *****)(puVar18 + 4) = uStack_70;
        *(code **)(puVar18 + 6) = uStack_68;
        *puVar18 = (undefined4)uStack_80;
        puVar18[1] = uStack_80._4_4_;
        puVar18[2] = (undefined4)uStack_78;
        puVar18[3] = uStack_78._4_4_;
        uStack_1b0 = puVar18 + 4;
        cVar24 = '\0';
        pppplVar17 = (longlong ****)ppplStack_a0;
      }
      else {
        cVar24 = '\x02';
        pppplVar17 = uStack_100;
        pppplVar36 = uStack_f8;
        pppplVar38 = uStack_110;
      }
code_r0x000140c13c20:
      LOCK();
      *(longlong *)puStack_98 = *(longlong *)puStack_98 + -1;
      UNLOCK();
      if (*(longlong *)puStack_98 == 0) {
        FUN_140c15eb0(&puStack_98);
      }
      pppplVar39 = pppplVar38;
      if (cVar24 != '\x02') {
code_r0x000140c13f4d:
        *plStack_d8 = (longlong)pppplVar38;
        plStack_d8[1] = (longlong)pcVar35;
        plStack_d8[2] = (longlong)pppplVar17;
        plStack_d8[3] = (longlong)pppplVar36;
        plStack_d8[4] = lStack_1c8;
        plStack_d8[5] = lStack_1c0;
        plStack_d8[6] = (longlong)pcStack_1b8;
        plStack_d8[7] = (longlong)uStack_1b0;
        *(undefined4 *)(plStack_d8 + 8) = uStack_ac;
        *(char *)((longlong)plStack_d8 + 0x44) = cVar24;
        *(undefined2 *)((longlong)plStack_d8 + 0x45) = uStack_113;
        *(undefined1 *)((longlong)plStack_d8 + 0x47) = uStack_111;
        goto code_r0x000140c13e53;
      }
    }
    else {
      uStack_49 = 1;
      in_stack_fffffffffffffdb8 =
           (uint *)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffdb8 >> 8),
                            pppplVar17 < ppplStack_a0);
      FUN_140c113e0(&uStack_80,lStack_200,ppplStack_a0,puVar25[3],
                    *(undefined1 *)((longlong)puVar25 + 0x34),
                    *(undefined1 *)((longlong)puVar25 + 0x35),lStack_210,lStack_1f8,
                    in_stack_fffffffffffffdb8);
      ppplVar13 = ppplStack_e0;
      if ((char)uStack_58 == '\x02') {
code_r0x000140c137b5:
        cVar24 = '\x02';
        pcVar35 = uStack_78;
        pppplVar17 = uStack_70;
        pppplVar36 = (longlong ****)uStack_68;
        pppplVar38 = uStack_80;
        goto code_r0x000140c13c20;
      }
      uStack_110 = uStack_80;
      pppplVar17 = uStack_110;
      uStack_108 = uStack_78;
      pcVar35 = uStack_108;
      uStack_100 = uStack_70;
      pppplVar38 = uStack_100;
      uStack_f8 = (longlong ****)uStack_68;
      pppplVar39 = uStack_f8;
      uStack_f0 = uStack_60;
      pppplVar14 = uStack_f0;
      uStack_e8 = SUB84(uStack_58,0);
      lVar33 = puVar25[1];
      pppplVar36 = (longlong ****)puVar25[2];
      pppplVar37 = (longlong ****)0xffffffffffffffff;
      lVar29 = 0;
      lVar22 = lVar33;
      do {
        uStack_e4 = uStack_58._4_4_;
        if ((longlong)pppplVar36 * 0x30 == lVar29) {
          if (pppplVar36 == (longlong ****)*puVar25) {
            uStack_88 = CONCAT71(uStack_88._1_7_,(char)uStack_58);
            FUN_140b9eff0(puVar25,&UNK_142b1b190);
            lVar33 = puVar25[1];
          }
          lVar22 = (longlong)pppplVar36 * 0x30;
          plVar2 = (longlong *)(lVar33 + 0x20 + lVar22);
          *plVar2 = (longlong)uStack_f0;
          plVar2[1] = CONCAT44(uStack_e4,uStack_e8);
          plVar2 = (longlong *)(lVar33 + 0x10 + lVar22);
          *plVar2 = (longlong)uStack_100;
          plVar2[1] = (longlong)uStack_f8;
          puVar18 = (undefined4 *)(lVar33 + lVar22);
          *puVar18 = (undefined4)uStack_110;
          puVar18[1] = uStack_110._4_4_;
          puVar18[2] = (undefined4)uStack_108;
          puVar18[3] = uStack_108._4_4_;
          pppplVar17 = (longlong ****)((longlong)pppplVar36 + 1);
          puVar25[2] = pppplVar17;
          goto code_r0x000140c13833;
        }
        pppplVar37 = (longlong ****)((longlong)pppplVar37 + 1);
        lVar29 = lVar29 + 0x30;
        pcVar1 = (char *)(lVar22 + 0x28);
        lVar22 = lVar22 + 0x30;
      } while (*pcVar1 != '\x02');
      if (pppplVar36 <= pppplVar37) {
        uStack_4a = 1;
        uStack_4b = 1;
        FUN_1428d9518(pppplVar37,pppplVar36,&UNK_142b1b1a8);
        goto code_r0x000140c14229;
      }
      uVar23 = *(ulonglong *)(lVar33 + -0x30 + lVar29);
      puVar28 = *(undefined8 **)(lVar33 + -0x28 + lVar29);
      cVar24 = *(char *)(lVar33 + -8 + lVar29);
      uStack_110._0_4_ = SUB84(uStack_80,0);
      uStack_110._4_4_ = (undefined4)((ulonglong)uStack_80 >> 0x20);
      uStack_108._0_4_ = SUB84(uStack_78,0);
      uStack_108._4_4_ = (undefined4)((ulonglong)uStack_78 >> 0x20);
      uStack_100._0_4_ = SUB84(uStack_70,0);
      uStack_100._4_4_ = (undefined4)((ulonglong)uStack_70 >> 0x20);
      uStack_f8._0_4_ = SUB84(uStack_68,0);
      uStack_f8._4_4_ = (undefined4)((ulonglong)uStack_68 >> 0x20);
      uStack_f0._0_4_ = SUB84(uStack_60,0);
      uStack_f0._4_4_ = (undefined4)((ulonglong)uStack_60 >> 0x20);
      puVar18 = (undefined4 *)(lVar33 + -0x20 + lVar29);
      *puVar18 = (undefined4)uStack_100;
      puVar18[1] = uStack_100._4_4_;
      puVar18[2] = (undefined4)uStack_f8;
      puVar18[3] = uStack_f8._4_4_;
      puVar18 = (undefined4 *)(lVar33 + -0x30 + lVar29);
      *puVar18 = (undefined4)uStack_110;
      puVar18[1] = uStack_110._4_4_;
      puVar18[2] = (undefined4)uStack_108;
      puVar18[3] = uStack_108._4_4_;
      puVar18 = (undefined4 *)(lVar33 + -0x10 + lVar29);
      *puVar18 = (undefined4)uStack_f0;
      puVar18[1] = uStack_f0._4_4_;
      puVar18[2] = uStack_e8;
      puVar18[3] = uStack_58._4_4_;
      uStack_110 = pppplVar17;
      uStack_108 = pcVar35;
      uStack_100 = pppplVar38;
      uStack_f8 = pppplVar39;
      uStack_f0 = pppplVar14;
      if (cVar24 != '\x02') {
        puStack_90 = puVar28;
        uStack_88 = uVar23;
        if ((code *)*puVar28 != (code *)0x0) {
          (*(code *)*puVar28)(uVar23);
        }
        if (puStack_90[1] != 0) {
          func_0x000140613c20(uStack_88,puStack_90[1],puStack_90[2]);
        }
      }
      pppplVar17 = (longlong ****)puVar25[2];
      pppplVar36 = pppplVar37;
code_r0x000140c13833:
      if (pppplVar17 <= pppplVar36) {
        uStack_4a = 1;
        uStack_4b = 0;
        FUN_1428d9518(pppplVar36,pppplVar17,&UNK_142b1b1c0);
        goto code_r0x000140c14229;
      }
      if (*(char *)(puVar25[1] + 0x28 + (longlong)pppplVar36 * 0x30) == '\x02') {
        pppplVar17 = (longlong ****)func_0x000140613c10(0x19,1);
        if (pppplVar17 != (longlong ****)0x0) {
code_r0x000140c13bf4:
          uVar26 = _UNK_142b1b1e9;
          *(ulonglong *)((longlong)pppplVar17 + 9) =
               CONCAT17(UNK_142b1b1e8,CONCAT43(_UNK_142b1b1e4,_UNK_142b1b1e1));
          *(undefined8 *)((longlong)pppplVar17 + 0x11) = uVar26;
          uVar12 = _UNK_142b1b1e4;
          uVar11 = _UNK_142b1b1e0;
          uVar10 = _UNK_142b1b1dc;
          *(undefined4 *)pppplVar17 = _UNK_142b1b1d8;
          *(undefined4 *)((longlong)pppplVar17 + 4) = uVar10;
          *(undefined4 *)(pppplVar17 + 1) = uVar11;
          *(undefined4 *)((longlong)pppplVar17 + 0xc) = uVar12;
          pppplVar38 = (longlong ****)0x5;
          pppplVar36 = (longlong ****)0x19;
          cVar24 = '\x02';
          pcVar35 = (code *)0x19;
          goto code_r0x000140c13c20;
        }
        uStack_4a = 1;
        uStack_4b = 0;
        FUN_1428d8fe3(1,0x19,&UNK_142b1adc8);
        goto code_r0x000140c14229;
      }
      puVar28 = (undefined8 *)(puVar25[1] + (longlong)pppplVar36 * 0x30);
      uStack_4a = 0;
      uStack_4b = 0;
      in_stack_fffffffffffffdb8 = puStack_98;
      (**(code **)(puVar28[1] + 0x40))
                (&uStack_80,*puVar28,ppplStack_a0,pplStack_c8,cStack_4c,uStack_1d0,lStack_c0,
                 ppplVar13,puStack_98);
      pppplVar38 = uStack_70;
      pcVar35 = uStack_78;
      pppplVar17 = uStack_70;
      pppplVar39 = uStack_80;
      if (uStack_80 == (longlong ****)0x9) {
        if ((longlong)ppplVar13 < 0) {
          uVar26 = 0;
code_r0x000140c141a7:
          uStack_4a = 0;
          uStack_4b = 0;
          FUN_1428d8fe3(uVar26,ppplStack_1e8,&UNK_142b1adc8);
        }
        else {
          lStack_1c8 = puVar25[3];
          lVar22 = puVar28[2];
          pcVar19 = (code *)puVar28[4];
          uStack_ac = *(undefined4 *)(puVar25 + 6);
          if ((longlong ****)ppplVar13 == (longlong ****)0x0) {
            pppplVar17 = (longlong ****)0x1;
          }
          else {
            uVar26 = 1;
            pppplVar17 = (longlong ****)func_0x000140613c10(ppplVar13,1);
            if (pppplVar17 == (longlong ****)0x0) {
              ppplStack_1e8 = ppplVar13;
              goto code_r0x000140c141a7;
            }
            func_0x000142923c00(pppplVar17,lStack_c0,ppplVar13);
          }
          uStack_60 = (longlong ****)ppplVar13;
          uStack_80 = (longlong ****)0x1;
          uStack_78 = (code *)0x1;
          uStack_70 = (longlong ****)ppplVar13;
          ppplStack_1e8 = (longlong ***)pppplVar17;
          lStack_1c0 = lVar22;
          pcStack_1b8 = pcVar19;
          uStack_68 = (code *)pppplVar17;
          puVar18 = (undefined4 *)func_0x000140613c10(0x28,8);
          if (puVar18 != (undefined4 *)0x0) {
            *(longlong *****)(puVar18 + 8) = uStack_60;
            *(longlong *****)(puVar18 + 4) = uStack_70;
            *(code **)(puVar18 + 6) = uStack_68;
            *puVar18 = (undefined4)uStack_80;
            puVar18[1] = uStack_80._4_4_;
            puVar18[2] = (undefined4)uStack_78;
            puVar18[3] = uStack_78._4_4_;
            uStack_1b0 = puVar18 + 4;
            pppplVar17 = (longlong ****)ppplStack_a0;
            pppplVar39 = pppplVar38;
            cVar24 = bStack_4d;
            if (bStack_4d == true) goto code_r0x000140c13c42;
            goto code_r0x000140c13f4d;
          }
          FUN_1428d9000(8,0x28);
        }
        goto code_r0x000140c14229;
      }
    }
code_r0x000140c13c42:
    if ((pppplVar39 < (longlong ****)0x6) && ((0x32UL >> ((ulonglong)pppplVar39 & 0x3f) & 1) != 0))
    {
      puVar25 = &uStack_80;
      if (pcVar35 != (code *)0x0) {
        uStack_80 = (longlong ****)0x1;
        puVar25 = &uStack_110;
      }
      *puVar25 = pcVar35;
      if ((uStack_80 != (longlong ****)0x0) && (uStack_110 != (longlong ****)0x0)) {
        func_0x000140613c20(pppplVar17);
      }
    }
  }
  *plStack_d8 = 0;
code_r0x000140c13e4f:
  *(undefined1 *)((longlong)plStack_d8 + 0x44) = 2;
code_r0x000140c13e53:
  LOCK();
  *(longlong *)puStack_a8 = *(longlong *)puStack_a8 + -1;
  UNLOCK();
  if (*(longlong *)puStack_a8 == 0) {
    FUN_140c15eb0(&puStack_a8);
  }
  return plStack_d8;
}

