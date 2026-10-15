// 0x140686790 with_nav
// signature: void __cdecl with_nav(void)

void __cdecl toolbar::mei::core::NodeContext::with_nav(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  ushort uVar6;
  char in_R8B;
  ulonglong uVar7;
  longlong lVar8;
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
  
  lVar2 = *in_RDX;
  if (*(longlong *)(lVar2 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar3 = *(longlong *)(lVar2 + 0x1a0);
    bVar11 = (byte)(extraout_RAX >> 0x39);
    auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
    auVar13 = pshuflw(auVar13,auVar13,0);
    lVar8 = 0;
    uVar7 = extraout_RAX;
    while( true ) {
      uVar7 = *(ulonglong *)(lVar2 + 0x1a8) & uVar7;
      pcVar1 = (char *)(lVar3 + uVar7);
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
      uVar6 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
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
      uVar9 = (uint)uVar6;
      while (uVar6 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar10 = (uVar4 + uVar7 & *(ulonglong *)(lVar2 + 0x1a8)) * -0xc0;
        if (in_RDX[1] == *(longlong *)(lVar3 + -0xc0 + lVar10)) {
          *(byte *)(lVar3 + -1 + lVar10) = in_R8B << 5 | *(byte *)(lVar3 + -1 + lVar10) & 0xdf;
          in_RCX[2] = in_RDX[2];
          lVar2 = in_RDX[1];
          *in_RCX = *in_RDX;
          in_RCX[1] = lVar2;
          return;
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
      uVar7 = uVar7 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

