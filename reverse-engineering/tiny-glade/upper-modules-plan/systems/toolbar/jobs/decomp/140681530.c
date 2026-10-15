// 0x140681530 size
// signature: void __cdecl size(void)

void __cdecl toolbar::mei::widgets::PillButtonView::size(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  ushort uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  uint uVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined4 in_XMM2_Da;
  undefined1 auVar19 [16];
  undefined4 in_XMM3_Da;
  undefined1 auVar20 [16];
  
  lVar6 = *in_RDX;
  lVar7 = in_RDX[1];
  lVar2 = in_RDX[2];
  if (*(longlong *)(lVar6 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar3 = *(longlong *)(lVar6 + 0x1a0);
    bVar13 = (byte)(extraout_RAX >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    lVar11 = 0;
    uVar10 = extraout_RAX;
    while( true ) {
      uVar10 = *(ulonglong *)(lVar6 + 0x1a8) & uVar10;
      pcVar1 = (char *)(lVar3 + uVar10);
      cVar14 = auVar15[0];
      auVar20[0] = -(*pcVar1 == cVar14);
      cVar16 = auVar15[1];
      auVar20[1] = -(pcVar1[1] == cVar16);
      cVar17 = auVar15[2];
      auVar20[2] = -(pcVar1[2] == cVar17);
      cVar18 = auVar15[3];
      auVar20[3] = -(pcVar1[3] == cVar18);
      auVar20[4] = -(pcVar1[4] == cVar14);
      auVar20[5] = -(pcVar1[5] == cVar16);
      auVar20[6] = -(pcVar1[6] == cVar17);
      auVar20[7] = -(pcVar1[7] == cVar18);
      auVar20[8] = -(pcVar1[8] == cVar14);
      auVar20[9] = -(pcVar1[9] == cVar16);
      auVar20[10] = -(pcVar1[10] == cVar17);
      auVar20[0xb] = -(pcVar1[0xb] == cVar18);
      auVar20[0xc] = -(pcVar1[0xc] == cVar14);
      auVar20[0xd] = -(pcVar1[0xd] == cVar16);
      auVar20[0xe] = -(pcVar1[0xe] == cVar17);
      auVar20[0xf] = -(pcVar1[0xf] == cVar18);
      uVar8 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
              (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar12; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar9 = (uVar4 + uVar10 & *(ulonglong *)(lVar6 + 0x1a8)) * -0xc0;
        if (lVar7 == *(longlong *)(lVar3 + -0xc0 + lVar9)) {
          *(undefined1 (*) [16])(lVar3 + -0x84 + lVar9) = (undefined1  [16])0x0;
          *(undefined1 (*) [16])(lVar3 + -0x90 + lVar9) = (undefined1  [16])0x0;
          *(undefined1 (*) [16])(lVar3 + -0xa0 + lVar9) = (undefined1  [16])0x0;
          *(undefined4 *)(lVar3 + -0x74 + lVar9) = in_XMM2_Da;
          *(undefined4 *)(lVar3 + -0x70 + lVar9) = 0;
          *(undefined4 *)(lVar3 + -0x6c + lVar9) = in_XMM3_Da;
          *(undefined8 *)(lVar3 + -0x68 + lVar9) = 0;
          *(undefined8 *)(lVar3 + -99 + lVar9) = 0;
          *(undefined1 *)(lVar3 + -0x5b + lVar9) = 2;
          *(undefined2 *)(lVar3 + -0x5a + lVar9) = 1;
          in_RCX[2] = lVar2;
          *in_RCX = lVar6;
          in_RCX[1] = lVar7;
          return;
        }
        uVar8 = (ushort)(uVar12 - 1) & (ushort)uVar12;
        uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar8);
      }
      auVar19[0] = -(*pcVar1 == -1);
      auVar19[1] = -(pcVar1[1] == -1);
      auVar19[2] = -(pcVar1[2] == -1);
      auVar19[3] = -(pcVar1[3] == -1);
      auVar19[4] = -(pcVar1[4] == -1);
      auVar19[5] = -(pcVar1[5] == -1);
      auVar19[6] = -(pcVar1[6] == -1);
      auVar19[7] = -(pcVar1[7] == -1);
      auVar19[8] = -(pcVar1[8] == -1);
      auVar19[9] = -(pcVar1[9] == -1);
      auVar19[10] = -(pcVar1[10] == -1);
      auVar19[0xb] = -(pcVar1[0xb] == -1);
      auVar19[0xc] = -(pcVar1[0xc] == -1);
      auVar19[0xd] = -(pcVar1[0xd] == -1);
      auVar19[0xe] = -(pcVar1[0xe] == -1);
      auVar19[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      uVar10 = uVar10 + lVar11 + 0x10;
      lVar11 = lVar11 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

