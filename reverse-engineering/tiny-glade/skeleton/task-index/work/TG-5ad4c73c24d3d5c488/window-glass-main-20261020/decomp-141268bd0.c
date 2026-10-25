
void __cdecl
hashbrown::map::HashMap::
retain<usize,country_core::resources::window_lights::LightInfo,std::hash::random::RandomState,alloc::alloc::Global,system_decorator::window_lights::window_lights::closure_env_1>
          (void)

{
  char *pcVar1;
  longlong lVar2;
  undefined1 (*pauVar3) [16];
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  ushort uVar7;
  short sVar8;
  uint uVar9;
  longlong lVar10;
  undefined8 *in_RCX;
  uint uVar11;
  longlong in_RDX;
  longlong lVar12;
  longlong lVar13;
  uint uVar14;
  ulonglong uVar15;
  undefined1 uVar16;
  uint uVar17;
  undefined1 (*pauVar18) [16];
  undefined1 (*pauVar19) [16];
  ulonglong uVar20;
  ushort uVar21;
  longlong lVar22;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  
  lVar10 = in_RCX[3];
  if (lVar10 != 0) {
    pauVar3 = (undefined1 (*) [16])*in_RCX;
    uVar4 = in_RCX[1];
    pauVar18 = pauVar3 + 1;
    auVar23 = *pauVar3;
    uVar14 = ~(uint)(ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar23[0xf] >> 7) << 0xf);
    lVar5 = *(longlong *)(in_RDX + 8);
    uVar6 = *(ulonglong *)(in_RDX + 0x10);
    uVar15 = uVar6 & 0xfffffffffffffff8;
    uVar11 = (uint)uVar6 & 7;
    lVar12 = in_RCX[2];
    pauVar19 = pauVar3;
    lVar22 = lVar10;
    do {
      if ((short)uVar14 == 0) {
        do {
          auVar23 = *pauVar18;
          pauVar19 = pauVar19 + -0x18;
          pauVar18 = pauVar18 + 1;
          uVar14 = (uint)(ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar23[0xf] >> 7) << 0xf);
        } while (uVar14 == 0xffff);
        uVar14 = ~uVar14;
      }
      uVar17 = 0;
      for (uVar9 = uVar14; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar17 = uVar17 + 1;
      }
      uVar14 = uVar14 - 1 & uVar14;
      lVar22 = lVar22 + -1;
      lVar13 = *(longlong *)((longlong)pauVar19 + (ulonglong)uVar17 * -0x18 + -0x18);
      uVar20 = 0;
      while (uVar15 != uVar20) {
        if (((((*(longlong *)(lVar5 + uVar20 * 8) == lVar13) ||
              (*(longlong *)(lVar5 + 8 + uVar20 * 8) == lVar13)) ||
             (*(longlong *)(lVar5 + 0x10 + uVar20 * 8) == lVar13)) ||
            ((*(longlong *)(lVar5 + 0x18 + uVar20 * 8) == lVar13 ||
             (*(longlong *)(lVar5 + 0x20 + uVar20 * 8) == lVar13)))) ||
           ((*(longlong *)(lVar5 + 0x28 + uVar20 * 8) == lVar13 ||
            ((*(longlong *)(lVar5 + 0x30 + uVar20 * 8) == lVar13 ||
             (lVar2 = uVar20 * 8, uVar20 = uVar20 + 8, *(longlong *)(lVar5 + 0x38 + lVar2) == lVar13
             )))))) goto LAB_141268c40;
      }
      if (((uVar6 & 7) == 0) ||
         ((*(longlong *)(lVar5 + uVar15 * 8) != lVar13 &&
          ((uVar11 == 1 ||
           ((*(longlong *)(lVar5 + 8 + uVar15 * 8) != lVar13 &&
            ((uVar11 == 2 ||
             ((*(longlong *)(lVar5 + 0x10 + uVar15 * 8) != lVar13 &&
              ((uVar11 == 3 ||
               ((*(longlong *)(lVar5 + 0x18 + uVar15 * 8) != lVar13 &&
                ((uVar11 == 4 ||
                 ((*(longlong *)(lVar5 + 0x20 + uVar15 * 8) != lVar13 &&
                  ((uVar11 == 5 ||
                   ((*(longlong *)(lVar5 + 0x28 + uVar15 * 8) != lVar13 &&
                    ((uVar11 == 6 ||
                     (*(longlong *)(lVar5 + 0x30 + uVar15 * 8) != lVar13 && uVar11 == 7)))))))))))))
               ))))))))))))) {
        lVar13 = ((longlong)((longlong)pauVar3 - ((longlong)pauVar19 + (ulonglong)uVar17 * -0x18))
                 >> 3) * -0x5555555555555555;
        uVar20 = lVar13 - 0x10U & uVar4;
        pcVar1 = *pauVar3 + uVar20;
        auVar23[0] = -(*pcVar1 == -1);
        auVar23[1] = -(pcVar1[1] == -1);
        auVar23[2] = -(pcVar1[2] == -1);
        auVar23[3] = -(pcVar1[3] == -1);
        auVar23[4] = -(pcVar1[4] == -1);
        auVar23[5] = -(pcVar1[5] == -1);
        auVar23[6] = -(pcVar1[6] == -1);
        auVar23[7] = -(pcVar1[7] == -1);
        auVar23[8] = -(pcVar1[8] == -1);
        auVar23[9] = -(pcVar1[9] == -1);
        auVar23[10] = -(pcVar1[10] == -1);
        auVar23[0xb] = -(pcVar1[0xb] == -1);
        auVar23[0xc] = -(pcVar1[0xc] == -1);
        auVar23[0xd] = -(pcVar1[0xd] == -1);
        auVar23[0xe] = -(pcVar1[0xe] == -1);
        auVar23[0xf] = -(pcVar1[0xf] == -1);
        uVar7 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
        if (uVar7 == 0) {
          uVar21 = 0x10;
        }
        else {
          uVar21 = 0xf;
          if (uVar7 != 0) {
            for (; uVar7 >> uVar21 == 0; uVar21 = uVar21 - 1) {
            }
          }
          uVar21 = uVar21 ^ 0xf;
        }
        pcVar1 = (char *)((longlong)pauVar3 + lVar13);
        auVar24[0] = -(*pcVar1 == -1);
        auVar24[1] = -(pcVar1[1] == -1);
        auVar24[2] = -(pcVar1[2] == -1);
        auVar24[3] = -(pcVar1[3] == -1);
        auVar24[4] = -(pcVar1[4] == -1);
        auVar24[5] = -(pcVar1[5] == -1);
        auVar24[6] = -(pcVar1[6] == -1);
        auVar24[7] = -(pcVar1[7] == -1);
        auVar24[8] = -(pcVar1[8] == -1);
        auVar24[9] = -(pcVar1[9] == -1);
        auVar24[10] = -(pcVar1[10] == -1);
        auVar24[0xb] = -(pcVar1[0xb] == -1);
        auVar24[0xc] = -(pcVar1[0xc] == -1);
        auVar24[0xd] = -(pcVar1[0xd] == -1);
        auVar24[0xe] = -(pcVar1[0xe] == -1);
        auVar24[0xf] = -(pcVar1[0xf] == -1);
        sVar8 = 0;
        for (uVar17 = (ushort)((ushort)(SUB161(auVar24 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe |
                              (ushort)(auVar24[0xf] >> 7) << 0xf) | 0x10000; (uVar17 & 1) == 0;
            uVar17 = uVar17 >> 1 | 0x80000000) {
          sVar8 = sVar8 + 1;
        }
        uVar16 = 0x80;
        if ((ushort)(uVar21 + sVar8) < 0x10) {
          lVar12 = lVar12 + 1;
          in_RCX[2] = lVar12;
          uVar16 = 0xff;
        }
        *(undefined1 *)((longlong)pauVar3 + lVar13) = uVar16;
        pauVar3[1][uVar20] = uVar16;
        lVar10 = lVar10 + -1;
        in_RCX[3] = lVar10;
      }
LAB_141268c40:
    } while (lVar22 != 0);
  }
  return;
}

