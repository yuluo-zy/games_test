/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_140aa4a60(longlong *param_1,longlong *param_2,undefined8 *param_3)
{
  longlong lVar1;
  undefined8 *puVar2;
  float fVar3;
  float fVar4;
  undefined1 auVar5 [16];
  char cVar6;
  char *pcVar7;
  undefined4 *puVar8;
  code *pcVar9;
  int iVar10;
  uint uVar11;
  float fVar12;
  float fVar13;
  longlong lVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  undefined1 (*pauVar17) [16];
  uint uVar18;
  ulonglong *puVar19;
  longlong lVar20;
  ulonglong uVar21;
  char *pcVar22;
  longlong lVar23;
  ulonglong *puVar24;
  undefined1 (*pauVar25) [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  longlong *plStack_f0;
  longlong *plStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_bc;
  char *pcStack_b8;
  char *pcStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  ulonglong *puStack_98;
  ulonglong uStack_90;
  ulonglong *puStack_88;
  undefined8 uStack_80;
  uStack_80 = 0xfffffffffffffffe;
  puVar19 = (ulonglong *)*param_1;
  fVar3 = *(float *)(puVar19 + 5);
  *(undefined4 *)param_1[2] = *(undefined4 *)((longlong)param_1 + 0x1c);
  fVar13 = _UNK_142930d70;
  if (puVar19[3] == 0) {
    return;
  }
  uVar15 = puVar19[1];
  uVar16 = *puVar19;
  if (puVar19[2] < *puVar19) {
    uVar16 = 0;
  }
  lVar14 = (puVar19[2] - uVar16) * 0x40;
  lVar1 = uVar15 + lVar14;
  uStack_a0 = 0;
  puStack_98 = (ulonglong *)0x8;
  uStack_90 = 0;
  pcVar7 = *(char **)(uVar15 + 8 + lVar14);
  pcStack_b8 = pcVar7 + *(longlong *)(uVar15 + 0x10 + lVar14) * 0x30;
  uStack_bc = *(undefined4 *)((longlong)param_3 + 0x1c);
  puStack_88 = (ulonglong *)*param_3;
  puVar8 = (undefined4 *)param_3[2];
  lVar14 = *param_2;
  uStack_a8 = 0;
  pcVar22 = pcStack_b8;
  while (pcStack_b0 = pcVar7, uVar15 = uStack_a8, pcStack_b0 != pcVar22) {
    pcVar7 = pcStack_b0 + 0x30;
    uStack_a8 = uVar15 + 1;
    if (*pcStack_b0 == '\t') {
      if ((*(float *)(pcStack_b0 + 0x18) == 0.0) && (!NAN(*(float *)(pcStack_b0 + 0x18)))) {
        lVar20 = *(longlong *)(pcStack_b0 + 0x10);
        cVar6 = pcStack_b0[0x1c];
        *puVar8 = uStack_bc;
        if (cVar6 == '\0') {
          FUN_1402ca2f0(puStack_88);
        }
        else {
          lVar23 = puStack_88[3];
          if (lVar23 != 0) {
            pauVar25 = (undefined1 (*) [16])*puStack_88;
            pauVar17 = pauVar25 + 1;
            auVar27 = *pauVar25;
            uVar18 = ~(uint)(ushort)((ushort)(SUB161(auVar27 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar27[0xf] >> 7) << 0xf);
            do {
              if ((short)uVar18 == 0) {
                do {
                  auVar27 = *pauVar17;
                  pauVar25 = pauVar25 + -8;
                  pauVar17 = pauVar17 + 1;
                  uVar18 = (uint)(ushort)((ushort)(SUB161(auVar27 >> 7,0) & 1) |
                                          (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                                          (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                                          (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                                          (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                                          (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                                          (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                                          (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                                          (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                                          (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                                          (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                                          (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                                          (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                                          (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                                          (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                                         (ushort)(byte)(auVar27[0xf] >> 7) << 0xf);
                } while (uVar18 == 0xffff);
                uVar18 = ~uVar18;
              }
              iVar10 = 0;
              for (uVar11 = uVar18; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
                iVar10 = iVar10 + 1;
              }
              if (*(longlong *)(lVar20 + 0x10) ==
                  *(longlong *)((longlong)pauVar25 + (-8 - (ulonglong)(uint)(iVar10 << 3)))) {
                func_0x000140b7a160(puStack_88);
              }
              uVar18 = uVar18 - 1 & uVar18;
              lVar23 = lVar23 + -1;
            } while (lVar23 != 0);
          }
        }
      }
      uVar16 = uStack_90;
      if ((pcStack_b0[8] & 1U) == 0) {
        if (uStack_90 == uStack_a0) {
          FUN_140b81df0(&uStack_a0);
        }
        puStack_98[uVar16] = uVar15;
        uStack_90 = uVar16 + 1;
        pcVar22 = pcStack_b8;
      }
      else {
        fVar4 = *(float *)(pcStack_b0 + 0x18);
        fVar12 = *(float *)(pcStack_b0 + 0xc) / fVar3;
        if (*(float *)(pcStack_b0 + 0xc) / fVar3 <= fVar13) {
          fVar12 = fVar13;
        }
        if (fVar12 < fVar4) {
          if (uStack_90 == uStack_a0) {
            FUN_140b81df0(&uStack_a0);
          }
          puStack_98[uVar16] = uVar15;
          uStack_90 = uVar16 + 1;
          fVar4 = *(float *)(pcStack_b0 + 0x18);
        }
        *(float *)(pcStack_b0 + 0x18) = fVar4 + *(float *)(lVar14 + 0x28);
        pcVar22 = pcStack_b8;
      }
    }
  }
  if (1 < uStack_90) {
    uVar15 = uStack_90 >> 1;
    if (uStack_90 < 8) {
      uVar16 = 0;
    }
    else {
      uVar16 = uVar15 & 0xfffffffffffffffc;
      pauVar17 = (undefined1 (*) [16])(puStack_98 + (uStack_90 - 2));
      uVar21 = 0;
      do {
        auVar27 = *(undefined1 (*) [16])(puStack_98 + uVar21);
        auVar5 = *(undefined1 (*) [16])(puStack_98 + uVar21 + 2);
        auVar26 = pauVar17[-1];
        auVar28 = *pauVar17;
        auVar29._0_8_ = auVar28._8_8_;
        auVar29._8_4_ = auVar28._0_4_;
        auVar29._12_4_ = auVar28._4_4_;
        auVar28._0_8_ = auVar26._8_8_;
        auVar28._8_4_ = auVar26._0_4_;
        auVar28._12_4_ = auVar26._4_4_;
        *(undefined1 (*) [16])(puStack_98 + uVar21) = auVar29;
        *(undefined1 (*) [16])(puStack_98 + uVar21 + 2) = auVar28;
        auVar26._0_8_ = auVar27._8_8_;
        auVar26._8_4_ = auVar27._0_4_;
        auVar26._12_4_ = auVar27._4_4_;
        *pauVar17 = auVar26;
        auVar27._0_8_ = auVar5._8_8_;
        auVar27._8_4_ = auVar5._0_4_;
        auVar27._12_4_ = auVar5._4_4_;
        pauVar17[-1] = auVar27;
        uVar21 = uVar21 + 4;
        pauVar17 = pauVar17 + -2;
      } while (uVar16 != uVar21);
      if (uVar15 == uVar16) goto joined_r0x000140aa4dc1;
    }
    puVar19 = puStack_98 + uVar16;
    lVar14 = -uVar16;
    do {
      uVar16 = *puVar19;
      *puVar19 = puStack_98[uStack_90 + lVar14 + -1];
      puStack_98[uStack_90 + lVar14 + -1] = uVar16;
      lVar14 = lVar14 + -1;
      puVar19 = puVar19 + 1;
    } while (-lVar14 != uVar15);
  }
joined_r0x000140aa4dc1:
  uStack_a8 = uStack_a0;
  puStack_88 = puStack_98;
  if (uStack_90 != 0) {
    puVar24 = puStack_98 + uStack_90;
    puVar19 = puStack_98;
    do {
      uVar15 = *puVar19;
      uVar16 = *(ulonglong *)(lVar1 + 0x10);
      if (uVar16 <= uVar15) {
        FUN_1428d90e9(uVar15,uVar16,&UNK_142adcd90);
                    /* WARNING: Does not return */
        pcVar9 = (code *)invalidInstructionException();
        (*pcVar9)();
      }
      lVar14 = *(longlong *)(lVar1 + 8);
      lVar20 = uVar15 * 0x30;
      uVar21 = *(ulonglong *)(lVar14 + lVar20);
      plStack_f0 = (longlong *)((ulonglong *)(lVar14 + lVar20))[1];
      puVar2 = (undefined8 *)(lVar14 + 0x10 + lVar20);
      plStack_e8 = (longlong *)*puVar2;
      uStack_e0 = puVar2[1];
      puVar2 = (undefined8 *)(lVar14 + 0x20 + lVar20);
      uStack_d8 = *puVar2;
      uStack_d0 = puVar2[1];
      func_0x000142923c10(lVar14 + lVar20,lVar14 + lVar20 + 0x30,(~uVar15 + uVar16) * 0x30);
      *(ulonglong *)(lVar1 + 0x10) = uVar16 - 1;
      switch(uVar21 & 0xff) {
      case 0:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          FUN_140b5cfc0(&plStack_e8);
        }
        break;
      case 1:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          FUN_140b5cfc0(&plStack_e8);
        }
        break;
      case 2:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          FUN_140b5cfc0(&plStack_e8);
        }
        break;
      case 3:
        LOCK();
        *plStack_f0 = *plStack_f0 + -1;
        UNLOCK();
        if (*plStack_f0 == 0) {
          FUN_140b5c2a0(&plStack_f0);
        }
        break;
      case 4:
        FUN_140891eb0(&plStack_f0);
        break;
      case 5:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          func_0x000140b5cd40(&plStack_e8);
        }
        break;
      case 6:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          func_0x000140b5d010(&plStack_e8);
        }
        break;
      case 7:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          func_0x000140b5d010(&plStack_e8);
        }
        break;
      case 8:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          FUN_140b5c040(&plStack_e8);
        }
        break;
      case 9:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          func_0x0001402e5d80(&plStack_e8);
        }
        break;
      default:
        LOCK();
        *plStack_e8 = *plStack_e8 + -1;
        UNLOCK();
        if (*plStack_e8 == 0) {
          func_0x0001406978e0(&plStack_e8);
        }
      }
      puVar19 = puVar19 + 1;
    } while (puVar19 != puVar24);
  }
  if (uStack_a8 != 0) {
    func_0x000140613c20(puStack_88,uStack_a8 << 3,8);
  }
  return;
}
