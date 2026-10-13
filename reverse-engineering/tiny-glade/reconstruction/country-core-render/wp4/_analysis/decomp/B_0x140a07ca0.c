// ADDRESS: 0x140a07ca0
// SIZE: 341
// SIGNATURE: void __cdecl positions_without_indices(void)

void __cdecl country_core::render::mesh::Mesh::positions_without_indices(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong *plVar3;
  uint uVar4;
  uint uVar5;
  ulonglong extraout_RAX;
  longlong in_RCX;
  uint uVar6;
  ushort uVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  byte bVar11;
  char cVar12;
  char cVar14;
  char cVar15;
  char cVar16;
  undefined1 auVar13 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  
  if (*(longlong *)(in_RCX + 0x50) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar2 = *(longlong *)(in_RCX + 0x38);
    bVar11 = (byte)(extraout_RAX >> 0x39);
    auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
    auVar13 = pshuflw(auVar13,auVar13,0);
    lVar10 = 0;
    uVar9 = extraout_RAX;
    while( true ) {
      uVar9 = *(ulonglong *)(in_RCX + 0x40) & uVar9;
      pcVar1 = (char *)(lVar2 + uVar9);
      cVar12 = auVar13[0];
      auVar18[0] = -(*pcVar1 == cVar12);
      cVar14 = auVar13[1];
      auVar18[1] = -(pcVar1[1] == cVar14);
      cVar15 = auVar13[2];
      auVar18[2] = -(pcVar1[2] == cVar15);
      cVar16 = auVar13[3];
      auVar18[3] = -(pcVar1[3] == cVar16);
      auVar18[4] = -(pcVar1[4] == cVar12);
      auVar18[5] = -(pcVar1[5] == cVar14);
      auVar18[6] = -(pcVar1[6] == cVar15);
      auVar18[7] = -(pcVar1[7] == cVar16);
      auVar18[8] = -(pcVar1[8] == cVar12);
      auVar18[9] = -(pcVar1[9] == cVar14);
      auVar18[10] = -(pcVar1[10] == cVar15);
      auVar18[0xb] = -(pcVar1[0xb] == cVar16);
      auVar18[0xc] = -(pcVar1[0xc] == cVar12);
      auVar18[0xd] = -(pcVar1[0xd] == cVar14);
      auVar18[0xe] = -(pcVar1[0xe] == cVar15);
      auVar18[0xf] = -(pcVar1[0xf] == cVar16);
      uVar7 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
      uVar6 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar6; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar8 = (uVar4 + uVar9 & *(ulonglong *)(in_RCX + 0x40)) * -0x38;
        if ((*(longlong *)(lVar2 + -0x28 + lVar8) == 0xf) &&
           (plVar3 = *(longlong **)(lVar2 + lVar8 + -0x30),
           *(longlong *)((longlong)plVar3 + 7) == 0x6e6f697469736f50 &&
           *plVar3 == 0x505f786574726556)) {
          if (*(int *)(lVar2 + -0x20 + lVar8) == 3) {
            return;
          }
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        uVar7 = (ushort)(uVar6 - 1) & (ushort)uVar6;
        uVar6 = CONCAT22((short)(uVar6 - 1 >> 0x10),uVar7);
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
      uVar9 = uVar9 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

