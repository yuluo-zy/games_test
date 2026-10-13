
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
longlong * asset_pipe__texture__compress(longlong *param_1,longlong param_2,undefined8 param_3,uint param_4)
{
  uint *puVar1;
  code *pcVar2;
  code *pcVar3;
  code *pcVar4;
  code *pcVar5;
  uint uVar6;
  uint uVar7;
  undefined1 auVar8 [16];
  short sVar9;
  short sVar10;
  short sVar11;
  short sVar12;
  short sVar13;
  short sVar14;
  short sVar15;
  short sVar16;
  short sVar17;
  short sVar18;
  short sVar19;
  short sVar20;
  short sVar21;
  short sVar22;
  char cVar23;
  char cVar31;
  undefined8 uVar39;
  ushort uVar40;
  ushort uVar41;
  ushort uVar42;
  ushort uVar43;
  ushort uVar44;
  ushort uVar45;
  ushort uVar46;
  ushort uVar47;
  ulonglong uVar48;
  undefined8 *puVar49;
  code *pcVar50;
  code *pcVar51;
  code *unaff_RBX;
  ulonglong uVar52;
  longlong lVar53;
  ulonglong uVar54;
  byte bVar55;
  bool bVar56;
  bool bVar57;
  ushort uVar58;
  short sVar59;
  ushort uVar66;
  ushort uVar67;
  ushort uVar68;
  ushort uVar69;
  ushort uVar70;
  ushort uVar71;
  ushort uVar72;
  ushort uVar73;
  ushort uVar74;
  ushort uVar75;
  ushort uVar76;
  ushort uVar77;
  ushort uVar78;
  ushort uVar79;
  ushort uVar80;
  short sVar81;
  code *pcStack_138;
  undefined *puStack_130;
  code *pcStack_128;
  ulonglong uStack_120;
  int iStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  int iStack_10c;
  undefined8 uStack_108;
  undefined *puStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  code **ppcStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  code *pcStack_b8;
  code *pcStack_b0;
  uint *puStack_a8;
  code *pcStack_a0;
  byte *pbStack_98;
  undefined *puStack_90;
  code *pcStack_88;
  uint uStack_7c;
  uint uStack_78;
  uint uStack_74;
  longlong lStack_70;
  undefined1 uStack_61;
  longlong lStack_60;
  code *pcStack_58;
  byte abStack_49 [9];
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  char cVar29;
  char cVar30;
  char cVar32;
  char cVar33;
  char cVar34;
  char cVar35;
  char cVar36;
  char cVar37;
  char cVar38;
  undefined4 uVar60;
  undefined6 uVar61;
  undefined8 uVar62;
  undefined1 auVar63 [12];
  undefined1 auVar64 [14];
  undefined1 auVar65 [16];
  undefined4 uVar82;
  undefined6 uVar83;
  undefined8 uVar84;
  undefined1 auVar85 [12];
  undefined1 auVar86 [14];
  undefined1 auVar87 [16];
  abStack_49[1] = 0xfe;
  abStack_49[2] = 0xff;
  abStack_49[3] = 0xff;
  abStack_49[4] = 0xff;
  abStack_49[5] = 0xff;
  abStack_49[6] = 0xff;
  abStack_49[7] = 0xff;
  abStack_49[8] = 0xff;
  abStack_49[0] = (byte)param_4;
  puVar1 = (uint *)(param_2 + 0x18);
  uVar6 = *puVar1;
  uVar7 = *(uint *)(param_2 + 0x1c);
  uVar39 = *(undefined8 *)puVar1;
  uVar84 = *(undefined8 *)puVar1;
  uVar62 = *(undefined8 *)puVar1;
  auVar8._8_8_ = 0;
  auVar8._0_8_ = (ulonglong)uVar6 * 4;
  uVar48 = SUB168(auVar8 * ZEXT416(uVar7),0);
  if (SUB168(auVar8 * ZEXT416(uVar7),8) == 0) {
    uVar52 = *(ulonglong *)(param_2 + 0x10);
    if (uVar52 < uVar48) {
      FUN_1428d9840(uVar48,uVar52,&UNK_142fded48);
      goto LAB_1417bd20c;
    }
    unaff_RBX = (code *)(ulonglong)param_4;
    uStack_74 = uVar7 * uVar6 + 0xf;
    pcVar50 = *(code **)(param_2 + 8);
    if (uVar48 == 0) {
      bVar56 = true;
    }
    else {
      lVar53 = 0;
      do {
        bVar56 = pcVar50[lVar53 + 3] == (code)0xff;
        if (!bVar56) break;
        bVar57 = uVar48 - 4 != lVar53;
        lVar53 = lVar53 + 4;
      } while (bVar57);
    }
    lStack_60 = (ulonglong)(uStack_74 >> 4) << (4U - (abStack_49[0] == 1) & 0x3f);
    lStack_70 = 1;
    if ((uStack_74 < 0x10) || (lStack_70 = func_0x000140613c40(lStack_60,1), lStack_70 != 0)) {
      if (2 < *_DAT_14362c3a8) {
        pcStack_b8 = (code *)&uStack_78;
        pcStack_b0 = FUN_140d8d560;
        puStack_a8 = &uStack_7c;
        pcStack_a0 = FUN_140d8d560;
        pbStack_98 = abStack_49;
        puStack_90 = &UNK_1417bd340;
        uStack_7c = uVar7;
        uStack_78 = uVar6;
        puVar49 = (undefined8 *)asset_recovery_14067dd80(&UNK_142ce0090);
        uStack_ec = *(undefined4 *)(puVar49 + 2);
        uStack_108 = 3;
        puStack_100 = &UNK_142cdf658;
        uStack_f8 = 0x13;
        puStack_e8 = &UNK_142ce0050;
        uStack_e0 = 4;
        ppcStack_d8 = &pcStack_b8;
        uStack_d0 = 3;
        uStack_c8 = 0;
        pcStack_138 = (code *)0x0;
        puStack_130 = &UNK_142cdf658;
        pcStack_128 = (code *)0x13;
        uStack_120 = 0;
        iStack_118 = (int)*puVar49;
        uStack_114 = (undefined4)((ulonglong)*puVar49 >> 0x20);
        uStack_110 = (undefined4)puVar49[1];
        iStack_10c = (int)((ulonglong)puVar49[1] >> 0x20);
        uStack_f0 = 1;
        func_0x000140d4a330(&uStack_61,&pcStack_138);
        param_4 = (uint)abStack_49[0];
      }
      if ((char)param_4 == '\0') {
        puStack_130 = (undefined *)(uVar52 & 0xfffffffffffffffc);
        uStack_120 = (ulonglong)((uint)uVar52 & 3);
        pcStack_128 = pcVar50 + (longlong)puStack_130;
        iStack_118 = 4;
        uStack_114 = 0;
        pcStack_138 = pcVar50;
        alloc__vec__spec_from_iter__impl$0__from_iter(&pcStack_b8,&pcStack_138,&UNK_142cdda30);
        puStack_130 = (undefined *)((longlong)puStack_a8 * 2);
        pcStack_58 = pcStack_b0;
        pcStack_138 = pcStack_b0;
        uStack_120 = CONCAT44(uStack_120._4_4_,uVar6 * 2);
        pcStack_128 = (code *)uVar62;
        FUN_1420e37b0(&pcStack_138,lStack_70,lStack_60);
        if (pcStack_b8 == (code *)0x0) goto LAB_1417bd1d0;
        pcVar50 = pcStack_58;
        pcVar51 = (code *)((longlong)pcStack_b8 * 2);
      }
      else {
        if ((param_4 & 0xff) != 1) {
          pcStack_a0 = (code *)CONCAT44(pcStack_a0._4_4_,uVar6 << 2);
          pcStack_138 = _DAT_142cdd960;
          puStack_130 = (undefined *)_UNK_142cdd968;
          pcStack_128 = (code *)_DAT_142b011a0;
          uStack_120 = _UNK_142b011a8;
          bVar55 = bVar56 ^ 1;
          iStack_118 = (uint)bVar55 + (uint)bVar55;
          iStack_10c = (uint)bVar55 << 2;
          puStack_100 = (undefined *)CONCAT35(puStack_100._5_3_,0x100000000);
          uStack_114 = 0xc;
          uStack_110 = 8;
          uStack_108 = 0x200000000;
          puStack_100 = (undefined *)CONCAT44(puStack_100._4_4_,4 - (uint)bVar56);
          pcStack_b8 = pcVar50;
          pcStack_b0 = (code *)uVar52;
          puStack_a8 = (uint *)uVar84;
          FUN_1420e3830(&pcStack_138,&pcStack_b8,lStack_70,lStack_60);
          goto LAB_1417bd1d0;
        }
        if (uVar52 < 4) {
          pcStack_88 = (code *)0x1;
          pcStack_58 = (code *)0x0;
          puStack_130 = (undefined *)0x0;
        }
        else {
          unaff_RBX = (code *)(uVar52 >> 2);
          pcStack_88 = (code *)func_0x000140613c10(unaff_RBX);
          uVar47 = _UNK_1429b2b8e;
          uVar46 = _UNK_1429b2b8c;
          uVar45 = _UNK_1429b2b8a;
          uVar44 = _UNK_1429b2b88;
          uVar43 = _UNK_1429b2b86;
          uVar42 = _UNK_1429b2b84;
          uVar41 = _UNK_1429b2b82;
          uVar40 = _DAT_1429b2b80;
          if (pcStack_88 == (code *)0x0) goto LAB_1417bd22d;
          uVar48 = uVar52 & 0xfffffffffffffffc;
          uVar52 = uVar52 - 4;
          if ((uVar52 < 0x40) ||
             (pcStack_88 < pcVar50 + (uVar52 & 0xfffffffffffffffc) + 1 &&
              pcVar50 < pcStack_88 + (uVar52 >> 2) + 1)) {
            puStack_130 = (undefined *)0x0;
            pcVar51 = pcVar50;
          }
          else {
            uVar54 = (uVar52 >> 2) + 1;
            uVar52 = 0x10;
            if ((uVar54 & 0xf) != 0) {
              uVar52 = (ulonglong)((uint)uVar54 & 0xf);
            }
            puStack_130 = (undefined *)(uVar54 - uVar52);
            uVar48 = uVar48 + (longlong)puStack_130 * -4;
            pcVar51 = pcVar50 + (longlong)puStack_130 * 4;
            lVar53 = 0;
            do {
              pcVar2 = pcVar50 + lVar53 * 4;
              pcVar3 = pcVar50 + lVar53 * 4 + 0x10;
              pcVar4 = pcVar50 + lVar53 * 4 + 0x20;
              pcVar5 = pcVar50 + lVar53 * 4 + 0x30;
              uVar73 = *(ushort *)pcVar5 & uVar40;
              uVar74 = *(ushort *)(pcVar5 + 2) & uVar41;
              uVar75 = *(ushort *)(pcVar5 + 4) & uVar42;
              uVar76 = *(ushort *)(pcVar5 + 6) & uVar43;
              uVar77 = *(ushort *)(pcVar5 + 8) & uVar44;
              uVar78 = *(ushort *)(pcVar5 + 10) & uVar45;
              uVar79 = *(ushort *)(pcVar5 + 0xc) & uVar46;
              uVar80 = *(ushort *)(pcVar5 + 0xe) & uVar47;
              uVar58 = *(ushort *)pcVar4 & uVar40;
              uVar66 = *(ushort *)(pcVar4 + 2) & uVar41;
              uVar67 = *(ushort *)(pcVar4 + 4) & uVar42;
              uVar68 = *(ushort *)(pcVar4 + 6) & uVar43;
              uVar69 = *(ushort *)(pcVar4 + 8) & uVar44;
              uVar70 = *(ushort *)(pcVar4 + 10) & uVar45;
              uVar71 = *(ushort *)(pcVar4 + 0xc) & uVar46;
              uVar72 = *(ushort *)(pcVar4 + 0xe) & uVar47;
              cVar23 = (0 < (short)uVar58) * ((short)uVar58 < 0x100) * (char)uVar58 -
                       (0xff < (short)uVar58);
              sVar81 = CONCAT11((0 < (short)uVar66) * ((short)uVar66 < 0x100) * (char)uVar66 -
                                (0xff < (short)uVar66),cVar23);
              cVar24 = (0 < (short)uVar67) * ((short)uVar67 < 0x100) * (char)uVar67 -
                       (0xff < (short)uVar67);
              uVar82 = CONCAT13((0 < (short)uVar68) * ((short)uVar68 < 0x100) * (char)uVar68 -
                                (0xff < (short)uVar68),CONCAT12(cVar24,sVar81));
              cVar25 = (0 < (short)uVar69) * ((short)uVar69 < 0x100) * (char)uVar69 -
                       (0xff < (short)uVar69);
              uVar83 = CONCAT15((0 < (short)uVar70) * ((short)uVar70 < 0x100) * (char)uVar70 -
                                (0xff < (short)uVar70),CONCAT14(cVar25,uVar82));
              cVar26 = (0 < (short)uVar71) * ((short)uVar71 < 0x100) * (char)uVar71 -
                       (0xff < (short)uVar71);
              uVar84 = CONCAT17((0 < (short)uVar72) * ((short)uVar72 < 0x100) * (char)uVar72 -
                                (0xff < (short)uVar72),CONCAT16(cVar26,uVar83));
              cVar27 = (0 < (short)uVar73) * ((short)uVar73 < 0x100) * (char)uVar73 -
                       (0xff < (short)uVar73);
              auVar85._0_10_ =
                   CONCAT19((0 < (short)uVar74) * ((short)uVar74 < 0x100) * (char)uVar74 -
                            (0xff < (short)uVar74),CONCAT18(cVar27,uVar84));
              cVar28 = (0 < (short)uVar75) * ((short)uVar75 < 0x100) * (char)uVar75 -
                       (0xff < (short)uVar75);
              auVar85[10] = cVar28;
              auVar85[0xb] = (0 < (short)uVar76) * ((short)uVar76 < 0x100) * (char)uVar76 -
                             (0xff < (short)uVar76);
              cVar29 = (0 < (short)uVar77) * ((short)uVar77 < 0x100) * (char)uVar77 -
                       (0xff < (short)uVar77);
              auVar86[0xc] = cVar29;
              auVar86._0_12_ = auVar85;
              auVar86[0xd] = (0 < (short)uVar78) * ((short)uVar78 < 0x100) * (char)uVar78 -
                             (0xff < (short)uVar78);
              cVar30 = (0 < (short)uVar79) * ((short)uVar79 < 0x100) * (char)uVar79 -
                       (0xff < (short)uVar79);
              auVar87[0xe] = cVar30;
              auVar87._0_14_ = auVar86;
              auVar87[0xf] = (0 < (short)uVar80) * ((short)uVar80 < 0x100) * (char)uVar80 -
                             (0xff < (short)uVar80);
              uVar73 = *(ushort *)pcVar3 & uVar40;
              uVar74 = *(ushort *)(pcVar3 + 2) & uVar41;
              uVar75 = *(ushort *)(pcVar3 + 4) & uVar42;
              uVar76 = *(ushort *)(pcVar3 + 6) & uVar43;
              uVar77 = *(ushort *)(pcVar3 + 8) & uVar44;
              uVar78 = *(ushort *)(pcVar3 + 10) & uVar45;
              uVar79 = *(ushort *)(pcVar3 + 0xc) & uVar46;
              uVar80 = *(ushort *)(pcVar3 + 0xe) & uVar47;
              uVar58 = *(ushort *)pcVar2 & uVar40;
              uVar66 = *(ushort *)(pcVar2 + 2) & uVar41;
              uVar67 = *(ushort *)(pcVar2 + 4) & uVar42;
              uVar68 = *(ushort *)(pcVar2 + 6) & uVar43;
              uVar69 = *(ushort *)(pcVar2 + 8) & uVar44;
              uVar70 = *(ushort *)(pcVar2 + 10) & uVar45;
              uVar71 = *(ushort *)(pcVar2 + 0xc) & uVar46;
              uVar72 = *(ushort *)(pcVar2 + 0xe) & uVar47;
              cVar31 = (0 < (short)uVar58) * ((short)uVar58 < 0x100) * (char)uVar58 -
                       (0xff < (short)uVar58);
              sVar59 = CONCAT11((0 < (short)uVar66) * ((short)uVar66 < 0x100) * (char)uVar66 -
                                (0xff < (short)uVar66),cVar31);
              cVar32 = (0 < (short)uVar67) * ((short)uVar67 < 0x100) * (char)uVar67 -
                       (0xff < (short)uVar67);
              uVar60 = CONCAT13((0 < (short)uVar68) * ((short)uVar68 < 0x100) * (char)uVar68 -
                                (0xff < (short)uVar68),CONCAT12(cVar32,sVar59));
              cVar33 = (0 < (short)uVar69) * ((short)uVar69 < 0x100) * (char)uVar69 -
                       (0xff < (short)uVar69);
              uVar61 = CONCAT15((0 < (short)uVar70) * ((short)uVar70 < 0x100) * (char)uVar70 -
                                (0xff < (short)uVar70),CONCAT14(cVar33,uVar60));
              cVar34 = (0 < (short)uVar71) * ((short)uVar71 < 0x100) * (char)uVar71 -
                       (0xff < (short)uVar71);
              uVar62 = CONCAT17((0 < (short)uVar72) * ((short)uVar72 < 0x100) * (char)uVar72 -
                                (0xff < (short)uVar72),CONCAT16(cVar34,uVar61));
              cVar35 = (0 < (short)uVar73) * ((short)uVar73 < 0x100) * (char)uVar73 -
                       (0xff < (short)uVar73);
              auVar63._0_10_ =
                   CONCAT19((0 < (short)uVar74) * ((short)uVar74 < 0x100) * (char)uVar74 -
                            (0xff < (short)uVar74),CONCAT18(cVar35,uVar62));
              cVar36 = (0 < (short)uVar75) * ((short)uVar75 < 0x100) * (char)uVar75 -
                       (0xff < (short)uVar75);
              auVar63[10] = cVar36;
              auVar63[0xb] = (0 < (short)uVar76) * ((short)uVar76 < 0x100) * (char)uVar76 -
                             (0xff < (short)uVar76);
              cVar37 = (0 < (short)uVar77) * ((short)uVar77 < 0x100) * (char)uVar77 -
                       (0xff < (short)uVar77);
              auVar64[0xc] = cVar37;
              auVar64._0_12_ = auVar63;
              auVar64[0xd] = (0 < (short)uVar78) * ((short)uVar78 < 0x100) * (char)uVar78 -
                             (0xff < (short)uVar78);
              cVar38 = (0 < (short)uVar79) * ((short)uVar79 < 0x100) * (char)uVar79 -
                       (0xff < (short)uVar79);
              auVar65[0xe] = cVar38;
              auVar65._0_14_ = auVar64;
              auVar65[0xf] = (0 < (short)uVar80) * ((short)uVar80 < 0x100) * (char)uVar80 -
                             (0xff < (short)uVar80);
              sVar9 = (short)((uint)uVar60 >> 0x10);
              sVar10 = (short)((uint6)uVar61 >> 0x20);
              sVar11 = (short)((ulonglong)uVar62 >> 0x30);
              sVar12 = (short)((unkuint10)auVar63._0_10_ >> 0x40);
              sVar13 = auVar63._10_2_;
              sVar14 = auVar64._12_2_;
              sVar15 = auVar65._14_2_;
              sVar16 = (short)((uint)uVar82 >> 0x10);
              sVar17 = (short)((uint6)uVar83 >> 0x20);
              sVar18 = (short)((ulonglong)uVar84 >> 0x30);
              sVar19 = (short)((unkuint10)auVar85._0_10_ >> 0x40);
              sVar20 = auVar85._10_2_;
              sVar21 = auVar86._12_2_;
              sVar22 = auVar87._14_2_;
              pcVar2 = pcStack_88 + lVar53;
              *pcVar2 = (code)((0 < sVar59) * (sVar59 < 0x100) * cVar31 - (0xff < sVar59));
              pcVar2[1] = (code)((0 < sVar9) * (sVar9 < 0x100) * cVar32 - (0xff < sVar9));
              pcVar2[2] = (code)((0 < sVar10) * (sVar10 < 0x100) * cVar33 - (0xff < sVar10));
              pcVar2[3] = (code)((0 < sVar11) * (sVar11 < 0x100) * cVar34 - (0xff < sVar11));
              pcVar2[4] = (code)((0 < sVar12) * (sVar12 < 0x100) * cVar35 - (0xff < sVar12));
              pcVar2[5] = (code)((0 < sVar13) * (sVar13 < 0x100) * cVar36 - (0xff < sVar13));
              pcVar2[6] = (code)((0 < sVar14) * (sVar14 < 0x100) * cVar37 - (0xff < sVar14));
              pcVar2[7] = (code)((0 < sVar15) * (sVar15 < 0x100) * cVar38 - (0xff < sVar15));
              pcVar2[8] = (code)((0 < sVar81) * (sVar81 < 0x100) * cVar23 - (0xff < sVar81));
              pcVar2[9] = (code)((0 < sVar16) * (sVar16 < 0x100) * cVar24 - (0xff < sVar16));
              pcVar2[10] = (code)((0 < sVar17) * (sVar17 < 0x100) * cVar25 - (0xff < sVar17));
              pcVar2[0xb] = (code)((0 < sVar18) * (sVar18 < 0x100) * cVar26 - (0xff < sVar18));
              pcVar2[0xc] = (code)((0 < sVar19) * (sVar19 < 0x100) * cVar27 - (0xff < sVar19));
              pcVar2[0xd] = (code)((0 < sVar20) * (sVar20 < 0x100) * cVar28 - (0xff < sVar20));
              pcVar2[0xe] = (code)((0 < sVar21) * (sVar21 < 0x100) * cVar29 - (0xff < sVar21));
              pcVar2[0xf] = (code)((0 < sVar22) * (sVar22 < 0x100) * cVar30 - (0xff < sVar22));
              lVar53 = lVar53 + 0x10;
            } while (puStack_130 != (undefined *)lVar53);
          }
          uVar52 = 0;
          do {
            pcStack_88[(longlong)puStack_130] = pcVar51[uVar52];
            puStack_130 = (undefined *)((longlong)puStack_130 + 1);
            uVar52 = uVar52 + 4;
            pcStack_58 = unaff_RBX;
          } while (uVar48 != uVar52);
        }
        uStack_120 = CONCAT44(uStack_120._4_4_,uVar6);
        pcStack_138 = pcStack_88;
        pcStack_128 = (code *)uVar39;
        FUN_1420e3720(&pcStack_138,lStack_70,lStack_60);
        pcVar50 = pcStack_88;
        pcVar51 = pcStack_58;
        if (pcStack_58 == (code *)0x0) goto LAB_1417bd1d0;
      }
      func_0x000140613c20(pcVar50,pcVar51,1);
LAB_1417bd1d0:
      *param_1 = lStack_60;
      param_1[1] = lStack_70;
      param_1[2] = lStack_60;
      return param_1;
    }
  }
  else {
LAB_1417bd20c:
    FUN_1428d9310(&UNK_142fded30);
  }
  FUN_1428d8fe3(1,lStack_60,&UNK_142ce0018);
LAB_1417bd22d:
  FUN_1428d8fe3(1,unaff_RBX,&UNK_142cdda30);
                    /* WARNING: Does not return */
  pcVar50 = (code *)invalidInstructionException();
  (*pcVar50)();
}
