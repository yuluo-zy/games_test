
void __cdecl
toolbar::mei::core::
iter_subtree<country_core::systems::ui::GlobalId,toolbar::mei::core::update_nav::closure_env_0>
          (void)

{
  char *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RCX;
  longlong in_RDX;
  ushort uVar6;
  longlong lVar7;
  longlong *in_R8;
  ulonglong uVar8;
  uint uVar9;
  longlong lVar10;
  byte bVar11;
  char cVar12;
  char cVar14;
  char cVar15;
  char cVar16;
  undefined1 auVar13 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  
  plVar2 = (longlong *)*in_R8;
  if (plVar2[3] != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar3 = *plVar2;
    bVar11 = (byte)(extraout_RAX >> 0x39);
    auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
    auVar13 = pshuflw(auVar13,auVar13,0);
    lVar10 = 0;
    uVar8 = extraout_RAX;
    while( true ) {
      uVar8 = plVar2[1] & uVar8;
      pcVar1 = (char *)(lVar3 + uVar8);
      cVar12 = auVar13[0];
      auVar19[0] = -(*pcVar1 == cVar12);
      cVar14 = auVar13[1];
      auVar19[1] = -(pcVar1[1] == cVar14);
      cVar15 = auVar13[2];
      auVar19[2] = -(pcVar1[2] == cVar15);
      cVar16 = auVar13[3];
      auVar19[3] = -(pcVar1[3] == cVar16);
      auVar19[4] = -(pcVar1[4] == cVar12);
      auVar19[5] = -(pcVar1[5] == cVar14);
      auVar19[6] = -(pcVar1[6] == cVar15);
      auVar19[7] = -(pcVar1[7] == cVar16);
      auVar19[8] = -(pcVar1[8] == cVar12);
      auVar19[9] = -(pcVar1[9] == cVar14);
      auVar19[10] = -(pcVar1[10] == cVar15);
      auVar19[0xb] = -(pcVar1[0xb] == cVar16);
      auVar19[0xc] = -(pcVar1[0xc] == cVar12);
      auVar19[0xd] = -(pcVar1[0xd] == cVar14);
      auVar19[0xe] = -(pcVar1[0xe] == cVar15);
      auVar19[0xf] = -(pcVar1[0xf] == cVar16);
      uVar6 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar9 = (uint)uVar6;
      while (uVar6 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar7 = (uVar4 + uVar8 & plVar2[1]) * -0xc0;
        if (*(longlong *)(lVar3 + -0xc0 + lVar7) == in_RDX) {
          if ((*(byte *)(lVar3 + -1 + lVar7) & 0x20) != 0) {
            return;
          }
          if (*(longlong *)(in_RCX + 0x1b8) != 0) {
            ::core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar3 = *(longlong *)(in_RCX + 0x1a0);
            bVar11 = (byte)(extraout_RAX_00 >> 0x39);
            auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
            auVar13 = pshuflw(auVar13,auVar13,0);
            lVar10 = 0;
            uVar8 = extraout_RAX_00;
            while( true ) {
              uVar8 = *(ulonglong *)(in_RCX + 0x1a8) & uVar8;
              pcVar1 = (char *)(lVar3 + uVar8);
              cVar12 = auVar13[0];
              auVar20[0] = -(*pcVar1 == cVar12);
              cVar14 = auVar13[1];
              auVar20[1] = -(pcVar1[1] == cVar14);
              cVar15 = auVar13[2];
              auVar20[2] = -(pcVar1[2] == cVar15);
              cVar16 = auVar13[3];
              auVar20[3] = -(pcVar1[3] == cVar16);
              auVar20[4] = -(pcVar1[4] == cVar12);
              auVar20[5] = -(pcVar1[5] == cVar14);
              auVar20[6] = -(pcVar1[6] == cVar15);
              auVar20[7] = -(pcVar1[7] == cVar16);
              auVar20[8] = -(pcVar1[8] == cVar12);
              auVar20[9] = -(pcVar1[9] == cVar14);
              auVar20[10] = -(pcVar1[10] == cVar15);
              auVar20[0xb] = -(pcVar1[0xb] == cVar16);
              auVar20[0xc] = -(pcVar1[0xc] == cVar12);
              auVar20[0xd] = -(pcVar1[0xd] == cVar14);
              auVar20[0xe] = -(pcVar1[0xe] == cVar15);
              auVar20[0xf] = -(pcVar1[0xf] == cVar16);
              uVar6 = (ushort)(SUB161(auVar20 >> 7,0) & 1) |
                      (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
                      (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
                      (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
                      (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
                      (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
                      (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
                      (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
                      (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
                      (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
                      (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
                      (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
                      (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
                      (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
                      (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe |
                      (ushort)(auVar20[0xf] >> 7) << 0xf;
              uVar9 = (uint)uVar6;
              while (uVar6 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                lVar7 = (uVar4 + uVar8 & *(ulonglong *)(in_RCX + 0x1a8)) * -0xc0;
                if (*(longlong *)(lVar3 + -0xc0 + lVar7) == in_RDX) {
                  lVar3 = *(longlong *)(lVar3 + -0xa8 + lVar7);
                  lVar10 = 0;
                  do {
                    if (lVar3 << 3 == lVar10) {
                      return;
                    }
                    iter_subtree<country_core::systems::ui::GlobalId,toolbar::mei::core::update_nav::closure_env_0>
                              ();
                    lVar10 = lVar10 + 8;
                  } while (extraout_RAX_01 == 0);
                  return;
                }
                uVar6 = (ushort)(uVar9 - 1) & (ushort)uVar9;
                uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar6);
              }
              auVar18[0] = -(*pcVar1 == -1);
              auVar18[1] = -(pcVar1[1] == -1);
              auVar18[2] = -(pcVar1[2] == -1);
              auVar18[3] = -(pcVar1[3] == -1);
              auVar18[4] = -(pcVar1[4] == -1);
              auVar18[5] = -(pcVar1[5] == -1);
              auVar18[6] = -(pcVar1[6] == -1);
              auVar18[7] = -(pcVar1[7] == -1);
              auVar18[8] = -(pcVar1[8] == -1);
              auVar18[9] = -(pcVar1[9] == -1);
              auVar18[10] = -(pcVar1[10] == -1);
              auVar18[0xb] = -(pcVar1[0xb] == -1);
              auVar18[0xc] = -(pcVar1[0xc] == -1);
              auVar18[0xd] = -(pcVar1[0xd] == -1);
              auVar18[0xe] = -(pcVar1[0xe] == -1);
              auVar18[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar18 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar18 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar18 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar18 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar18 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar18 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar18 >> 0x67,0) & 1) != 0) || (SUB161(auVar18 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar18 >> 0x77,0) & 1) != 0) || auVar18[0xf] < '\0') break;
              uVar8 = uVar8 + lVar10 + 0x10;
              lVar10 = lVar10 + 0x10;
            }
          }
                    // WARNING: Subroutine does not return
          ::core::option::expect_failed();
        }
        uVar6 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar6);
      }
      auVar17[0] = -(*pcVar1 == -1);
      auVar17[1] = -(pcVar1[1] == -1);
      auVar17[2] = -(pcVar1[2] == -1);
      auVar17[3] = -(pcVar1[3] == -1);
      auVar17[4] = -(pcVar1[4] == -1);
      auVar17[5] = -(pcVar1[5] == -1);
      auVar17[6] = -(pcVar1[6] == -1);
      auVar17[7] = -(pcVar1[7] == -1);
      auVar17[8] = -(pcVar1[8] == -1);
      auVar17[9] = -(pcVar1[9] == -1);
      auVar17[10] = -(pcVar1[10] == -1);
      auVar17[0xb] = -(pcVar1[0xb] == -1);
      auVar17[0xc] = -(pcVar1[0xc] == -1);
      auVar17[0xd] = -(pcVar1[0xd] == -1);
      auVar17[0xe] = -(pcVar1[0xe] == -1);
      auVar17[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      uVar8 = uVar8 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::expect_failed();
}

