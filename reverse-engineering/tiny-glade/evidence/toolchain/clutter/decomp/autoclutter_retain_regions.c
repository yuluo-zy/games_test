
void __cdecl
hashbrown::map::HashMap::
retain<system_clutter::autoclutter::ClutterRegionId,system_clutter::autoclutter::AutoClutterRegion,std::hash::random::RandomState,alloc::alloc::Global,system_clutter::autoclutter::impl_1::remove_stale_regions::closure_env_0>
          (void)

{
  char *pcVar1;
  ushort uVar2;
  char *pcVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong *plVar6;
  short sVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  uint uVar13;
  longlong *in_RCX;
  longlong lVar14;
  longlong in_RDX;
  ulonglong uVar15;
  undefined1 (*pauVar16) [16];
  ushort uVar17;
  undefined1 uVar18;
  uint uVar19;
  longlong lVar20;
  undefined1 (*pauVar21) [16];
  longlong lVar22;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar25;
  undefined1 unaff_XMM6_Bc;
  char cVar26;
  undefined1 unaff_XMM6_Bd;
  char cVar27;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar28;
  undefined1 unaff_XMM6_Bg;
  char cVar29;
  undefined1 unaff_XMM6_Bh;
  char cVar30;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar31;
  undefined1 unaff_XMM6_Bk;
  char cVar32;
  undefined1 unaff_XMM6_Bl;
  char cVar33;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar34;
  undefined1 unaff_XMM6_Bo;
  char cVar35;
  undefined1 unaff_XMM6_Bp;
  char cVar36;
  
  lVar20 = in_RCX[3];
  if (lVar20 != 0) {
    pauVar21 = (undefined1 (*) [16])*in_RCX;
    pauVar16 = pauVar21 + 1;
    auVar23 = *pauVar21;
    uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) |
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
    pcVar3 = *(char **)(in_RDX + 0x10);
    plVar4 = *(longlong **)(in_RDX + 8);
    iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    cVar25 = (char)((uint)iVar9 >> 8);
    cVar26 = (char)((uint)iVar9 >> 0x10);
    cVar27 = (char)((uint)iVar9 >> 0x18);
    iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                    CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    cVar28 = (char)((uint)iVar10 >> 8);
    cVar29 = (char)((uint)iVar10 >> 0x10);
    cVar30 = (char)((uint)iVar10 >> 0x18);
    iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                    CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    cVar31 = (char)((uint)iVar11 >> 8);
    cVar32 = (char)((uint)iVar11 >> 0x10);
    cVar33 = (char)((uint)iVar11 >> 0x18);
    iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    cVar34 = (char)((uint)iVar12 >> 8);
    cVar35 = (char)((uint)iVar12 >> 0x10);
    cVar36 = (char)((uint)iVar12 >> 0x18);
    do {
      if ((short)uVar13 == 0) {
        do {
          auVar23 = *pauVar16;
          pauVar21 = pauVar21 + -0x48;
          pauVar16 = pauVar16 + 1;
          uVar13 = (uint)(ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) |
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
        } while (uVar13 == 0xffff);
        uVar13 = ~uVar13;
      }
      uVar19 = 0;
      for (uVar8 = uVar13; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
        uVar19 = uVar19 + 1;
      }
      if (*(char *)((longlong)pauVar21 + (ulonglong)uVar19 * -0x48 + -4) != *pcVar3) {
        lVar22 = (ulonglong)uVar19 * -0x48 + (longlong)pauVar21;
        system_clutter::autoclutter::AutoClutterState::remove_region_from_lazy_grid();
        uVar2 = *(ushort *)(lVar22 + -6);
        if (uVar2 < 0xfffe) {
          lVar5 = plVar4[2];
          if (lVar5 == *plVar4) {
            alloc::raw_vec::RawVec::grow_one<u16,alloc::alloc::Global>();
          }
          *(ushort *)(plVar4[1] + lVar5 * 2) = uVar2;
          plVar4[2] = lVar5 + 1;
        }
        lVar5 = *in_RCX;
        lVar14 = (lVar5 - lVar22 >> 3) * -0x71c71c71c71c71c7;
        uVar15 = lVar14 - 0x10U & in_RCX[1];
        pcVar1 = (char *)(lVar5 + uVar15);
        auVar23[0] = -(*pcVar1 == (char)iVar9);
        auVar23[1] = -(pcVar1[1] == cVar25);
        auVar23[2] = -(pcVar1[2] == cVar26);
        auVar23[3] = -(pcVar1[3] == cVar27);
        auVar23[4] = -(pcVar1[4] == (char)iVar10);
        auVar23[5] = -(pcVar1[5] == cVar28);
        auVar23[6] = -(pcVar1[6] == cVar29);
        auVar23[7] = -(pcVar1[7] == cVar30);
        auVar23[8] = -(pcVar1[8] == (char)iVar11);
        auVar23[9] = -(pcVar1[9] == cVar31);
        auVar23[10] = -(pcVar1[10] == cVar32);
        auVar23[0xb] = -(pcVar1[0xb] == cVar33);
        auVar23[0xc] = -(pcVar1[0xc] == (char)iVar12);
        auVar23[0xd] = -(pcVar1[0xd] == cVar34);
        auVar23[0xe] = -(pcVar1[0xe] == cVar35);
        auVar23[0xf] = -(pcVar1[0xf] == cVar36);
        uVar2 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
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
        pcVar1 = (char *)(lVar5 + lVar14);
        if (uVar2 == 0) {
          uVar17 = 0x10;
        }
        else {
          uVar17 = 0xf;
          if (uVar2 != 0) {
            for (; uVar2 >> uVar17 == 0; uVar17 = uVar17 - 1) {
            }
          }
          uVar17 = uVar17 ^ 0xf;
        }
        auVar24[0] = -(*pcVar1 == (char)iVar9);
        auVar24[1] = -(pcVar1[1] == cVar25);
        auVar24[2] = -(pcVar1[2] == cVar26);
        auVar24[3] = -(pcVar1[3] == cVar27);
        auVar24[4] = -(pcVar1[4] == (char)iVar10);
        auVar24[5] = -(pcVar1[5] == cVar28);
        auVar24[6] = -(pcVar1[6] == cVar29);
        auVar24[7] = -(pcVar1[7] == cVar30);
        auVar24[8] = -(pcVar1[8] == (char)iVar11);
        auVar24[9] = -(pcVar1[9] == cVar31);
        auVar24[10] = -(pcVar1[10] == cVar32);
        auVar24[0xb] = -(pcVar1[0xb] == cVar33);
        auVar24[0xc] = -(pcVar1[0xc] == (char)iVar12);
        auVar24[0xd] = -(pcVar1[0xd] == cVar34);
        auVar24[0xe] = -(pcVar1[0xe] == cVar35);
        auVar24[0xf] = -(pcVar1[0xf] == cVar36);
        sVar7 = 0;
        for (uVar19 = (ushort)((ushort)(SUB161(auVar24 >> 7,0) & 1) |
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
                              (ushort)(auVar24[0xf] >> 7) << 0xf) | 0x10000; (uVar19 & 1) == 0;
            uVar19 = uVar19 >> 1 | 0x80000000) {
          sVar7 = sVar7 + 1;
        }
        uVar18 = 0x80;
        if ((ushort)(uVar17 + sVar7) < 0x10) {
          in_RCX[2] = in_RCX[2] + 1;
          uVar18 = 0xff;
        }
        *(undefined1 *)(lVar5 + lVar14) = uVar18;
        *(undefined1 *)(lVar5 + 0x10 + uVar15) = uVar18;
        in_RCX[3] = in_RCX[3] + -1;
        if (*(int *)(lVar22 + -0x40) == 4) {
          plVar6 = *(longlong **)(lVar22 + -0x38);
          LOCK();
          *plVar6 = *plVar6 + -1;
          UNLOCK();
          if (*plVar6 == 0) {
            alloc::sync::Arc::drop_slow<slice2_<glam::f32::vec3::Vec3>,alloc::alloc::Global>();
          }
        }
      }
      uVar13 = uVar13 - 1 & uVar13;
      lVar20 = lVar20 + -1;
    } while (lVar20 != 0);
  }
  return;
}

