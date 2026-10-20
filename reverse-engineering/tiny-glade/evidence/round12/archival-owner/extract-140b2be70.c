country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_storage_decorators<ref$<str$> > :: resources/walls/decorator_archivist.rs
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
longlong *
FUN_140b2be70(longlong *param_1,longlong *param_2,undefined1 (*param_3) [16],
             undefined1 (*param_4) [16],longlong *param_5,char param_6,byte param_7)
{
  char *pcVar1;
  code *pcVar2;
  uint uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  byte *pbVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined1 (*pauVar10) [16];
  undefined1 (**ppauVar11) [16];
  undefined1 (**ppauVar12) [16];
  undefined1 (*pauVar13) [16];
  uint uVar14;
  uint uVar15;
  longlong lVar16;
  ushort uVar17;
  longlong lVar18;
  byte bVar19;
  char cVar20;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar21 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 (**ppauStack_140) [16];
  code *pcStack_138;
  undefined1 (***pppauStack_130) [16];
  code *pcStack_128;
  longlong lStack_120;
  longlong lStack_118;
  undefined1 (*pauStack_110) [16];
  undefined1 (*pauStack_108) [16];
  undefined1 (**ppauStack_100) [16];
  code *pcStack_f8;
  undefined1 (***pppauStack_f0) [16];
  code *pcStack_e8;
  undefined1 (*pauStack_e0) [16];
  undefined1 (*pauStack_d8) [16];
  undefined1 (*pauStack_d0) [16];
  undefined1 (***pppauStack_c8) [16];
  undefined8 uStack_c0;
  longlong lStack_b8;
  undefined1 (**ppauStack_78) [16];
  code *pcStack_70;
  undefined1 (***pppauStack_68) [16];
  code *pcStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  pauStack_110 = param_3;
  pauStack_108 = param_4;
  if (param_3 == (undefined1 (*) [16])0x0) {
    pauStack_d8 = param_4;
    if (param_2[3] == 0) {
      lVar6 = 0;
    }
    else {
      uVar8 = FUN_1405b0d70(param_2 + 4,&pauStack_d8);
      lVar6 = *param_2;
      bVar19 = (byte)(uVar8 >> 0x39);
      auVar21 = ZEXT216(CONCAT11(bVar19,bVar19));
      auVar21 = pshuflw(auVar21,auVar21,0);
      lVar9 = 0;
      lVar16 = 0;
      while( true ) {
        uVar8 = param_2[1] & uVar8;
        pcVar1 = (char *)(lVar6 + uVar8);
        cVar20 = auVar21[0];
        auVar26[0] = -(*pcVar1 == cVar20);
        cVar22 = auVar21[1];
        auVar26[1] = -(pcVar1[1] == cVar22);
        cVar23 = auVar21[2];
        auVar26[2] = -(pcVar1[2] == cVar23);
        cVar24 = auVar21[3];
        auVar26[3] = -(pcVar1[3] == cVar24);
        auVar26[4] = -(pcVar1[4] == cVar20);
        auVar26[5] = -(pcVar1[5] == cVar22);
        auVar26[6] = -(pcVar1[6] == cVar23);
        auVar26[7] = -(pcVar1[7] == cVar24);
        auVar26[8] = -(pcVar1[8] == cVar20);
        auVar26[9] = -(pcVar1[9] == cVar22);
        auVar26[10] = -(pcVar1[10] == cVar23);
        auVar26[0xb] = -(pcVar1[0xb] == cVar24);
        auVar26[0xc] = -(pcVar1[0xc] == cVar20);
        auVar26[0xd] = -(pcVar1[0xd] == cVar22);
        auVar26[0xe] = -(pcVar1[0xe] == cVar23);
        auVar26[0xf] = -(pcVar1[0xf] == cVar24);
        uVar17 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf
        ;
        uVar14 = (uint)uVar17;
        while (uVar17 != 0) {
          uVar15 = 0;
          for (uVar3 = uVar14; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
            uVar15 = uVar15 + 1;
          }
          lVar18 = (uVar15 + uVar8 & param_2[1]) * -0x98;
          if (*(undefined1 (**) [16])(lVar6 + -0x98 + lVar18) == param_4) {
            lVar9 = lVar6 + lVar18;
            goto code_r0x000140b2bf9c;
          }
          uVar17 = (ushort)(uVar14 - 1) & (ushort)uVar14;
          uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar17);
        }
        auVar25[0] = -(*pcVar1 == -1);
        auVar25[1] = -(pcVar1[1] == -1);
        auVar25[2] = -(pcVar1[2] == -1);
        auVar25[3] = -(pcVar1[3] == -1);
        auVar25[4] = -(pcVar1[4] == -1);
        auVar25[5] = -(pcVar1[5] == -1);
        auVar25[6] = -(pcVar1[6] == -1);
        auVar25[7] = -(pcVar1[7] == -1);
        auVar25[8] = -(pcVar1[8] == -1);
        auVar25[9] = -(pcVar1[9] == -1);
        auVar25[10] = -(pcVar1[10] == -1);
        auVar25[0xb] = -(pcVar1[0xb] == -1);
        auVar25[0xc] = -(pcVar1[0xc] == -1);
        auVar25[0xd] = -(pcVar1[0xd] == -1);
        auVar25[0xe] = -(pcVar1[0xe] == -1);
        auVar25[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar25 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar25 >> 0x1f,0) & 1) != 0) || (SUB161(auVar25 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar25 >> 0x37,0) & 1) != 0) || (SUB161(auVar25 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar25 >> 0x47,0) & 1) != 0) || (SUB161(auVar25 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar25 >> 0x57,0) & 1) != 0) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') break;
        uVar8 = uVar8 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
code_r0x000140b2bf9c:
      lVar6 = lVar9 + -0x98;
      if (lVar9 == 0) {
        lVar6 = 0;
      }
    }
    param_2 = (longlong *)(lVar6 + 8);
    if (lVar6 == 0) {
      param_2 = (longlong *)0x0;
    }
  }
  else {
    if (param_3 == (undefined1 (*) [16])0x1) {
      lVar6 = param_2[0x12];
      param_2 = param_2 + 0x12;
    }
    else {
      lVar6 = param_2[0x24];
      param_2 = param_2 + 0x24;
    }
    if (lVar6 == 0) {
      param_2 = (longlong *)0x0;
    }
  }
  if ((param_2 != (longlong *)0x0 & (param_7 ^ 1)) == 0) {
code_r0x000140b2c07c:
    if (param_2 != (longlong *)0x0) goto code_r0x000140b2c085;
    ppauVar11 = (undefined1 (**) [16])*param_5;
    ppauVar12 = (undefined1 (**) [16])0x0;
    if (ppauVar11 == (undefined1 (**) [16])0x0) goto code_r0x000140b2c21b;
  }
  else {
    lVar6 = param_2[9];
    if (lVar6 != 0) {
      pauVar10 = (undefined1 (*) [16])param_2[6];
      pauVar13 = pauVar10 + 1;
      auVar21 = *pauVar10;
      uVar14 = ~(uint)(ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar21[0xf] >> 7) << 0xf);
      do {
        if ((short)uVar14 == 0) {
          do {
            auVar21 = *pauVar13;
            uVar17 = (ushort)(SUB161(auVar21 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar21[0xf] >> 7) << 0xf;
            pauVar10 = pauVar10 + -0x50;
            pauVar13 = pauVar13 + 1;
          } while (uVar17 == 0xffff);
          uVar14 = ~(uint)uVar17;
          uVar15 = uVar14 & -(uint)uVar17 - 2;
        }
        else {
          uVar15 = uVar14 - 1 & uVar14;
        }
        uVar3 = 0;
        for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar8 = (ulonglong)uVar3;
        pauStack_e0 = pauVar10 + uVar8 * -5 + -5;
        ppauStack_100 = (undefined1 (**) [16])(pauVar10[uVar8 * -5 + -5] + 8);
        if (pauVar10[uVar8 * -5 + -1][0xf] != '\0') {
          ppauStack_78 = &pauStack_e0;
          pcStack_70 = FUN_140a44830;
          pppauStack_68 = &ppauStack_100;
          pcStack_60 = FUN_140a43f00;
          pauStack_d8 = (undefined1 (*) [16])&UNK_142aeb540;
          pauStack_d0 = (undefined1 (*) [16])0x2;
          lStack_b8 = 0;
          pppauStack_c8 = &ppauStack_78;
          uStack_c0 = 2;
          uStack_39 = 0;
          FUN_1428d9390(&pauStack_d8,&UNK_142aeb5a0);
          goto code_r0x000140b2c3d0;
        }
        lVar6 = lVar6 + -1;
        uVar14 = uVar15;
      } while (lVar6 != 0);
      goto code_r0x000140b2c07c;
    }
code_r0x000140b2c085:
    pauStack_d8 = (undefined1 (*) [16])param_2[6];
    pppauStack_c8 = (undefined1 (***) [16])(*pauStack_d8 + param_2[7] + 1);
    auVar21 = *pauStack_d8;
    uStack_c0 = CONCAT62(uStack_c0._2_6_,
                         ~((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar21[0xf] >> 7) << 0xf));
    pauStack_d0 = pauStack_d8 + 1;
    lStack_b8 = param_2[9];
    uStack_39 = 0;
    ppauVar12 = &pauStack_d8;
    FUN_141449670(&ppauStack_140);
    ppauVar11 = (undefined1 (**) [16])*param_5;
    if (ppauVar11 == (undefined1 (**) [16])0x0) {
      if (ppauStack_140 == (undefined1 (**) [16])0x0) {
code_r0x000140b2c21b:
        *param_1 = 0;
        return param_1;
      }
      lStack_58 = lStack_120;
      lStack_50 = lStack_118;
      pppauStack_68 = pppauStack_130;
      pcStack_60 = pcStack_128;
      ppauStack_78 = ppauStack_140;
      pcStack_70 = pcStack_138;
      if (param_6 != '\0') {
        ppauStack_100 = &pauStack_110;
        pcStack_f8 = FUN_140b30a90;
        pppauStack_f0 = &ppauStack_78;
        pcStack_e8 = FUN_140b30a10;
        pauStack_d8 = (undefined1 (*) [16])&UNK_142aeb620;
        pauStack_d0 = (undefined1 (*) [16])0x3;
        lStack_b8 = 0;
        pppauStack_c8 = &ppauStack_100;
        uStack_c0 = 2;
        FUN_1428d9390(&pauStack_d8,&UNK_142aeb650);
code_r0x000140b2c3d0:
                    /* WARNING: Does not return */
        pcVar2 = (code *)invalidInstructionException();
        (*pcVar2)();
      }
      *param_1 = 0;
      if (pcStack_138 == (code *)0x0) {
        return param_1;
      }
      uVar8 = (longlong)(pcStack_138 + 1) * 0x58 + 0xfU & 0xfffffffffffffff0;
      pcStack_138 = pcStack_138 + uVar8;
      goto joined_r0x000140b2c256;
    }
    if (ppauStack_140 != (undefined1 (**) [16])0x0) {
      lVar6 = *param_5;
      lVar9 = param_5[1];
      lVar16 = param_5[2];
      lVar18 = param_5[3];
      lVar4 = param_5[4];
      lVar5 = param_5[5];
      param_1[10] = lStack_120;
      param_1[0xb] = lStack_118;
      param_1[8] = (longlong)pppauStack_130;
      param_1[9] = (longlong)pcStack_128;
      param_1[6] = (longlong)ppauStack_140;
      param_1[7] = (longlong)pcStack_138;
      param_1[4] = lVar4;
      param_1[5] = lVar5;
      param_1[2] = lVar16;
      param_1[3] = lVar18;
      *param_1 = lVar6;
      param_1[1] = lVar9;
      return param_1;
    }
  }
  if (param_5[3] != 0) {
    ppauStack_78 = (undefined1 (**) [16])*param_5;
    pcStack_70 = (code *)param_5[1];
    pppauStack_68 = (undefined1 (***) [16])param_5[2];
    pcStack_60 = (code *)param_5[3];
    lStack_58 = param_5[4];
    lStack_50 = param_5[5];
    pbVar7 = (byte *)FUN_140d4d740();
    if ((*pbVar7 & 1) == 0) {
      lVar6 = FUN_140d64e00();
      pbVar7[0] = 1;
      pbVar7[1] = 0;
      pbVar7[2] = 0;
      pbVar7[3] = 0;
      pbVar7[4] = 0;
      pbVar7[5] = 0;
      pbVar7[6] = 0;
      pbVar7[7] = 0;
      *(longlong *)(pbVar7 + 8) = lVar6;
      *(undefined1 (***) [16])(pbVar7 + 0x10) = ppauVar12;
    }
    else {
      lVar6 = *(longlong *)(pbVar7 + 8);
      ppauVar12 = *(undefined1 (***) [16])(pbVar7 + 0x10);
    }
    *(longlong *)(pbVar7 + 8) = lVar6 + 1;
    lVar18 = _UNK_142aeb108;
    lVar16 = _UNK_142aeb0f8;
    lVar9 = _UNK_142aeb0f0;
    param_1[8] = _UNK_142aeb100;
    param_1[9] = lVar18;
    param_1[6] = lVar9;
    param_1[7] = lVar16;
    param_1[4] = lStack_58;
    param_1[5] = lStack_50;
    param_1[2] = (longlong)pppauStack_68;
    param_1[3] = (longlong)pcStack_60;
    *param_1 = (longlong)ppauStack_78;
    param_1[1] = (longlong)pcStack_70;
    param_1[10] = lVar6;
    param_1[0xb] = (longlong)ppauVar12;
    return param_1;
  }
  lVar6 = param_5[1];
  *param_1 = 0;
  if (lVar6 == 0) {
    return param_1;
  }
  uVar8 = (lVar6 + 1) * 0x58 + 0xfU & 0xfffffffffffffff0;
  pcStack_138 = (code *)(lVar6 + uVar8);
  ppauStack_140 = ppauVar11;
joined_r0x000140b2c256:
  if (pcStack_138 + 0x11 != (code *)0x0) {
    func_0x000140613c20((longlong)ppauStack_140 - uVar8,pcStack_138 + 0x11,0x10);
  }
  return param_1;
}
