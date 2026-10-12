
undefined8 *
FUN_140c07950(undefined8 *param_1,longlong param_2,char *param_3,char *param_4,uint *param_5,
             longlong *param_6)

{
  char cVar1;
  undefined8 *puVar2;
  code *pcVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  char *pcVar7;
  char *pcVar8;
  char *pcVar9;
  int iVar10;
  uint uVar11;
  char *pcVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  char **ppcVar17;
  longlong unaff_RDI;
  char *unaff_R12;
  longlong unaff_R13;
  char *unaff_R14;
  char *unaff_R15;
  undefined8 *puVar18;
  bool bVar19;
  undefined1 auStack_430 [192];
  undefined1 auStack_370 [28];
  int iStack_354;
  int iStack_350;
  int iStack_34c;
  char *pcStack_318;
  char *pcStack_310;
  char *pcStack_308;
  char *pcStack_300;
  char *pcStack_2e8;
  undefined2 uStack_2e0;
  undefined6 uStack_2de;
  undefined8 uStack_258;
  undefined8 uStack_250;
  char *pcStack_248;
  char *pcStack_240;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  char *pcStack_218;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  longlong lStack_1f8;
  char *pcStack_1e8;
  char *pcStack_1e0;
  char *pcStack_1d8;
  uint uStack_1d0;
  uint uStack_1cc;
  char *pcStack_1c8;
  char *pcStack_1c0;
  char *pcStack_1b8;
  char *pcStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  char *pcStack_188;
  longlong lStack_180;
  char *pcStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined8 uStack_130;
  char cStack_bc;
  undefined2 uStack_bb;
  undefined1 uStack_b9;
  char cStack_ac;
  undefined2 uStack_ab;
  undefined1 uStack_a9;
  undefined8 *puStack_a8;
  undefined8 uStack_a0;
  undefined8 *puStack_98;
  undefined4 uStack_90;
  undefined2 uStack_8c;
  undefined1 uStack_8a;
  undefined1 uStack_89;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  char *pcStack_78;
  char *pcStack_70;
  char *pcStack_68;
  char *pcStack_60;
  char *pcStack_58;
  undefined2 uStack_4c;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_1f8 = param_6[2];
  uStack_208 = (undefined4)*param_6;
  uStack_204 = *(undefined4 *)((longlong)param_6 + 4);
  lVar6 = *param_6;
  uStack_200 = (undefined4)param_6[1];
  uStack_1fc = *(undefined4 *)((longlong)param_6 + 0xc);
  pcStack_60 = param_3;
  if (*param_5 == 0) {
    FUN_1428d9430(&UNK_142b189c7,0x3c,&UNK_142b18a08);
code_r0x000140c081a2:
    FUN_1428d9430(&UNK_142b18a20,0x2e,&UNK_142b18a50);
code_r0x000140c081ba:
    FUN_1428d9430(&UNK_142b18a68,0x2f,&UNK_142b18a98);
code_r0x000140c081d2:
    FUN_1428d9430(&UNK_142b18ab0,0x2e,&UNK_142b18ae0);
code_r0x000140c081ea:
    FUN_1428cd210(unaff_R14);
code_r0x000140c07aff:
    pcStack_70 = unaff_R14;
    FUN_140c81c70(unaff_R14);
    pcStack_308 = pcStack_60;
    pcStack_2e8 = pcStack_78;
    _uStack_2e0 = CONCAT62(uStack_2de,0x100);
    pcStack_318 = (char *)0x2;
    pcStack_300 = unaff_R15;
    FUN_140c0da40(&pcStack_178,unaff_R13 + 0x18,&pcStack_318);
    pcVar8 = uStack_160;
    pcVar7 = uStack_168;
    pcStack_58 = pcStack_178;
    lStack_180 = unaff_RDI;
    if ((char)uStack_134 == '\x02') {
      pcStack_68 = uStack_170;
      pcVar12 = (char *)func_0x000140613c10(0xf,1);
      pcVar9 = pcStack_70;
      if (pcVar12 == (char *)0x0) {
        pcStack_60 = pcVar7;
        FUN_1428d8fe3(1,0xf,&UNK_142b18390);
        goto code_r0x000140c08366;
      }
      builtin_strncpy(pcVar12,"GpuOnly texture",0xf);
      pcStack_248 = pcStack_78;
      uStack_258 = (char *)CONCAT44(uStack_84,uStack_88);
      uStack_250 = (char *)CONCAT44(uStack_7c,uStack_80);
      if (pcStack_58 != (char *)0xb) {
        pcStack_1d8 = pcStack_78;
        param_1[1] = pcStack_68;
        param_1[2] = pcVar7;
        param_1[3] = pcVar8;
        *(undefined4 *)(param_1 + 7) = uStack_88;
        *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_84;
        *(undefined4 *)(param_1 + 8) = uStack_80;
        *(undefined4 *)((longlong)param_1 + 0x44) = uStack_7c;
        param_1[9] = pcStack_78;
        *param_1 = pcStack_58;
        param_1[4] = 0xf;
        param_1[5] = pcVar12;
        param_1[6] = 0xf;
        FUN_140c81e10();
        LOCK();
        cVar1 = *pcVar9;
        if (cVar1 == '\x01') {
          *pcVar9 = '\0';
        }
        UNLOCK();
        if (cVar1 == '\x01') {
          return param_1;
        }
        FUN_1428cdb90(pcVar9,0);
        return param_1;
      }
      pcStack_58 = pcStack_68;
      uStack_1a8 = pcVar12;
      pcStack_1b8 = pcVar8;
      uStack_1a0 = 0xf;
      pcStack_1b0 = (char *)0xf;
      pcStack_1c0 = pcVar7;
    }
    else {
      uStack_250 = (char *)CONCAT44(uStack_13c,uStack_140);
      uStack_258 = uStack_148;
      pcStack_248 = (char *)CONCAT44(uStack_134,uStack_138);
      uStack_1a8 = uStack_158;
      pcStack_1b8 = uStack_168;
      uStack_1a0 = uStack_150;
      pcStack_1b0 = uStack_160;
      pcStack_1c0 = uStack_170;
    }
    pcVar7 = pcStack_70;
    pcStack_1d8 = pcStack_248;
    pcStack_1e8 = uStack_258;
    pcStack_1e0 = uStack_250;
    pcStack_1c8 = pcStack_58;
    uStack_198 = (undefined4)uStack_258;
    uStack_194 = uStack_258._4_4_;
    uStack_190 = (undefined4)uStack_250;
    uStack_18c = uStack_250._4_4_;
    pcStack_188 = pcStack_248;
    uStack_49 = 1;
    FUN_140c81e10();
    LOCK();
    cVar1 = *pcVar7;
    if (cVar1 == '\x01') {
      *pcVar7 = '\0';
    }
    UNLOCK();
    if (cVar1 != '\x01') {
      uStack_49 = 1;
      FUN_1428cdb90(pcVar7,0);
    }
    uStack_49 = 1;
    uVar13 = func_0x0001406d0cb0(&pcStack_1c8);
    uStack_49 = 1;
    uVar14 = func_0x0001406868b0(&pcStack_1c8);
    iVar10 = (**(code **)(param_2 + 0xe0))(*(undefined8 *)(param_2 + 0x640),unaff_R12,uVar13,uVar14)
    ;
    if (iVar10 == 0) {
      puVar2 = *(undefined8 **)(param_2 + 0x648);
      pcStack_218 = pcStack_188;
      uStack_228 = uStack_198;
      uStack_224 = uStack_194;
      uStack_220 = uStack_190;
      uStack_21c = uStack_18c;
      uStack_238 = (undefined4)uStack_1a8;
      uStack_234 = uStack_1a8._4_4_;
      uStack_230 = (undefined4)uStack_1a0;
      uStack_22c = uStack_1a0._4_4_;
      pcStack_248 = pcStack_1b8;
      pcStack_240 = pcStack_1b0;
      uStack_258 = pcStack_1c8;
      uStack_250 = pcStack_1c0;
      pcStack_1e8 = *(char **)param_5;
      pcStack_1e0 = *(char **)(param_5 + 2);
      pcStack_1d8 = *(char **)(param_5 + 4);
      uStack_1d0 = param_5[6];
      uStack_1cc = param_5[7];
      uStack_49 = 0;
      FUN_140bdc2d0(&pcStack_178,pcStack_60,unaff_R15,unaff_R12,param_2 + 0x78,&uStack_258,
                    &pcStack_1e8);
      if (cStack_bc == '\x03') {
        param_1[8] = CONCAT44(uStack_134,uStack_138);
        param_1[9] = uStack_130;
        param_1[6] = uStack_148;
        param_1[7] = CONCAT44(uStack_13c,uStack_140);
        param_1[4] = uStack_158;
        param_1[5] = uStack_150;
        param_1[2] = uStack_168;
        param_1[3] = uStack_160;
        *param_1 = pcStack_178;
        param_1[1] = uStack_170;
      }
      else {
        func_0x000142923c00(&pcStack_318,&pcStack_178,0xbc);
        uStack_4c = uStack_bb;
        uStack_4a = uStack_b9;
        func_0x000142923c00(auStack_430,&pcStack_318,0xbc);
        puVar15 = (undefined8 *)puVar2[0x2f];
        do {
          puVar18 = puVar15;
          if (puVar18 == (undefined8 *)0x0) {
            func_0x000142923c00(&uStack_168,auStack_430,0xbc);
            cStack_ac = cStack_bc;
            uStack_ab = uStack_4c;
            uStack_a9 = uStack_4a;
            pcStack_178 = (char *)0x0;
            uStack_170 = (char *)0x1;
            uStack_49 = 0;
            ppcVar17 = &pcStack_178;
            uVar16 = FUN_1418bbd30(puVar2 + 1);
            if ((ulonglong)puVar2[0x2d] <= uVar16) {
              uStack_49 = 0;
              FUN_1428d9430(&UNK_142d0528e,0x22,&UNK_142d02268);
              goto code_r0x000140c08366;
            }
            uStack_49 = 0;
            uVar11 = func_0x000140c111b0(uVar16);
            if (uVar11 < 0x2c) {
              puVar18 = (undefined8 *)((longlong)ppcVar17 * 0xd0 + (puVar2 + 1)[uVar11]);
              goto code_r0x000140c0802a;
            }
            uStack_49 = 0;
            FUN_1428d9518((ulonglong)uVar11,0x2c,&UNK_142d02268);
            goto code_r0x000140c08366;
          }
          LOCK();
          puVar15 = (undefined8 *)puVar2[0x2f];
          bVar19 = puVar18 == puVar15;
          if (bVar19) {
            puVar2[0x2f] = puVar18[2];
            puVar15 = puVar18;
          }
          UNLOCK();
        } while (!bVar19);
        func_0x000142923c00(puVar18 + 2,auStack_430,0xbc);
        *(char *)((longlong)puVar18 + 0xcc) = cStack_bc;
        *(undefined2 *)((longlong)puVar18 + 0xcd) = uStack_4c;
        *(undefined1 *)((longlong)puVar18 + 0xcf) = uStack_4a;
        *(undefined4 *)(puVar18 + 1) = 1;
        *puVar18 = 0;
code_r0x000140c0802a:
        uStack_90 = *(undefined4 *)((longlong)puVar18 + 0xc);
        uStack_8c = *(undefined2 *)(puVar2 + 0x14a);
        uStack_a0 = *puVar2;
        puStack_a8 = puVar18 + 2;
        uStack_8a = 0;
        puStack_98 = puVar18;
        if (lStack_180 != 0) {
          pcStack_318 = *(char **)param_5;
          pcStack_310 = *(char **)(param_5 + 2);
          pcStack_308 = *(char **)(param_5 + 4);
          pcStack_300 = *(char **)(param_5 + 6);
          FUN_140bfe270(&pcStack_178,param_2,&pcStack_318,&uStack_208,&puStack_a8);
          if (pcStack_178 != (char *)0xb) {
            param_1[9] = uStack_130;
            *(undefined4 *)(param_1 + 7) = uStack_140;
            *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_13c;
            *(undefined4 *)(param_1 + 8) = uStack_138;
            *(undefined4 *)((longlong)param_1 + 0x44) = uStack_134;
            *(undefined4 *)(param_1 + 5) = (undefined4)uStack_150;
            *(undefined4 *)((longlong)param_1 + 0x2c) = uStack_150._4_4_;
            *(undefined4 *)(param_1 + 6) = (undefined4)uStack_148;
            *(undefined4 *)((longlong)param_1 + 0x34) = uStack_148._4_4_;
            *(undefined4 *)(param_1 + 3) = (undefined4)uStack_160;
            *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_160._4_4_;
            *(undefined4 *)(param_1 + 4) = (undefined4)uStack_158;
            *(undefined4 *)((longlong)param_1 + 0x24) = uStack_158._4_4_;
            *(undefined4 *)(param_1 + 1) = (undefined4)uStack_170;
            *(undefined4 *)((longlong)param_1 + 0xc) = uStack_170._4_4_;
            *(undefined4 *)(param_1 + 2) = (undefined4)uStack_168;
            *(undefined4 *)((longlong)param_1 + 0x14) = uStack_168._4_4_;
            *param_1 = pcStack_178;
            uStack_49 = 0;
            FUN_140be24b0(&uStack_a0);
            return param_1;
          }
        }
        uVar13 = *(undefined8 *)param_5;
        uVar14 = *(undefined8 *)(param_5 + 2);
        uVar4 = *(undefined8 *)(param_5 + 4);
        uVar5 = *(undefined8 *)(param_5 + 6);
        param_1[1] = uStack_a0;
        param_1[2] = puStack_98;
        param_1[7] = uVar5;
        param_1[5] = uVar14;
        param_1[6] = uVar4;
        param_1[3] = CONCAT17(uStack_89,CONCAT16(uStack_8a,CONCAT24(uStack_8c,uStack_90)));
        param_1[4] = uVar13;
        *param_1 = 0xb;
      }
      return param_1;
    }
  }
  else {
    if (lVar6 != 0) {
      *param_5 = *param_5 | 2;
    }
    FUN_140bd5ed0(auStack_370,param_5,lVar6 != 0);
    unaff_RDI = lVar6;
    unaff_R14 = param_4;
    if (iStack_354 == 0) goto code_r0x000140c081a2;
    if (iStack_350 == 0) goto code_r0x000140c081ba;
    if (iStack_34c == 0) goto code_r0x000140c081d2;
    iVar10 = (**(code **)(param_2 + 0x1a0))
                       (*(undefined8 *)(param_2 + 0x640),auStack_370,0,&pcStack_178);
    unaff_R12 = pcStack_178;
    if (iVar10 == 0) {
      if (*(longlong *)(param_2 + 0x870) != 0) {
        FUN_140d797d0(&pcStack_318,pcStack_60,param_4);
        pcVar7 = pcStack_308;
        if (!SBORROW8(0,(longlong)pcStack_318)) {
          uStack_168 = pcStack_308;
          uStack_160 = pcStack_300;
          pcStack_178 = pcStack_318;
          uStack_170 = pcStack_310;
          FUN_1428d9760(&UNK_142b18318,0x2b,&pcStack_178,&UNK_142b182f8,&UNK_142b18b20);
          goto code_r0x000140c08366;
        }
        pcStack_178 = (char *)CONCAT44(pcStack_178._4_4_,0x3b9cbe00);
        uStack_170 = (char *)0x0;
        uStack_168 = (char *)CONCAT44(uStack_168._4_4_,10);
        uStack_160 = unaff_R12;
        uStack_158 = pcStack_310;
        iVar10 = (**(code **)(param_2 + 0x870))(*(undefined8 *)(param_2 + 0x8b0),&pcStack_178);
        if (iVar10 != 0) {
          pcStack_58 = pcStack_310;
          pcStack_60 = pcVar7;
          pcStack_318 = (char *)CONCAT44(pcStack_318._4_4_,iVar10);
          FUN_1428d9760(&UNK_142b18b38,0x1b,&pcStack_318,&UNK_142b182b8,&UNK_142b18b58);
          goto code_r0x000140c08366;
        }
        *pcStack_310 = '\0';
        if (pcVar7 != (char *)0x0) {
          func_0x000140613c20(pcStack_310,pcVar7,1);
        }
      }
      (**(code **)(param_2 + 0xf0))(*(undefined8 *)(param_2 + 0x640),unaff_R12,&uStack_88);
      unaff_R13 = *(longlong *)(param_2 + 0x678);
      unaff_R14 = (char *)(unaff_R13 + 0x10);
      LOCK();
      bVar19 = *(char *)(unaff_R13 + 0x10) == '\0';
      if (bVar19) {
        *(char *)(unaff_R13 + 0x10) = '\x01';
      }
      UNLOCK();
      unaff_R15 = param_4;
      if (!bVar19) goto code_r0x000140c081ea;
      goto code_r0x000140c07aff;
    }
    pcStack_178 = (char *)CONCAT44(pcStack_178._4_4_,iVar10);
    iVar10 = FUN_1428d9760(&UNK_142b18af8,0xc,&pcStack_178,&UNK_142b182b8,&UNK_142b18b08);
  }
  pcStack_178 = (char *)CONCAT44(pcStack_178._4_4_,iVar10);
  uStack_49 = 1;
  FUN_1428d9760(&UNK_142b18b70,0x11,&pcStack_178,&UNK_142b182b8,&UNK_142b18b88);
code_r0x000140c08366:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

