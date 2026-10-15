// 0x140686690 ignore_input
// signature: void __cdecl ignore_input(void)

void __cdecl toolbar::mei::core::NodeContext::ignore_input(void)

{
  char *pcVar1;
  byte *pbVar2;
  longlong lVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  ushort uVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  uint uVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  
  lVar3 = *in_RDX;
  if (*(longlong *)(lVar3 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar4 = *(longlong *)(lVar3 + 0x1a0);
    bVar12 = (byte)(extraout_RAX >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar10 = 0;
    uVar9 = extraout_RAX;
    while( true ) {
      uVar9 = *(ulonglong *)(lVar3 + 0x1a8) & uVar9;
      pcVar1 = (char *)(lVar4 + uVar9);
      cVar13 = auVar14[0];
      auVar19[0] = -(*pcVar1 == cVar13);
      cVar15 = auVar14[1];
      auVar19[1] = -(pcVar1[1] == cVar15);
      cVar16 = auVar14[2];
      auVar19[2] = -(pcVar1[2] == cVar16);
      cVar17 = auVar14[3];
      auVar19[3] = -(pcVar1[3] == cVar17);
      auVar19[4] = -(pcVar1[4] == cVar13);
      auVar19[5] = -(pcVar1[5] == cVar15);
      auVar19[6] = -(pcVar1[6] == cVar16);
      auVar19[7] = -(pcVar1[7] == cVar17);
      auVar19[8] = -(pcVar1[8] == cVar13);
      auVar19[9] = -(pcVar1[9] == cVar15);
      auVar19[10] = -(pcVar1[10] == cVar16);
      auVar19[0xb] = -(pcVar1[0xb] == cVar17);
      auVar19[0xc] = -(pcVar1[0xc] == cVar13);
      auVar19[0xd] = -(pcVar1[0xd] == cVar15);
      auVar19[0xe] = -(pcVar1[0xe] == cVar16);
      auVar19[0xf] = -(pcVar1[0xf] == cVar17);
      uVar7 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
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
      uVar11 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar8 = (uVar5 + uVar9 & *(ulonglong *)(lVar3 + 0x1a8)) * -0xc0;
        if (in_RDX[1] == *(longlong *)(lVar4 + -0xc0 + lVar8)) {
          pbVar2 = (byte *)(lVar4 + -1 + lVar8);
          *pbVar2 = *pbVar2 | 2;
          in_RCX[2] = in_RDX[2];
          lVar3 = in_RDX[1];
          *in_RCX = *in_RDX;
          in_RCX[1] = lVar3;
          return;
        }
        uVar7 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar7);
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
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      uVar9 = uVar9 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

