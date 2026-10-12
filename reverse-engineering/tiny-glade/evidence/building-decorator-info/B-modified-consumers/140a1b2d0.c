
void __cdecl country_core::resources::walls::decorator_storage::impl$5::next(void)

{
  char *pcVar1;
  longlong *plVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  longlong lVar6;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  uint uVar7;
  ulonglong uVar8;
  undefined1 (*pauVar9) [16];
  longlong lVar10;
  ushort uVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  
  lVar10 = in_RCX[4];
  if (lVar10 == 0) {
    return;
  }
  uVar11 = *(ushort *)(in_RCX + 3);
  uVar7 = (uint)uVar11;
  lVar6 = *in_RCX;
  if (uVar11 == 0) {
    pauVar9 = (undefined1 (*) [16])in_RCX[1];
    do {
      auVar14 = *pauVar9;
      uVar11 = (ushort)(SUB161(auVar14 >> 7,0) & 1) | (ushort)(SUB161(auVar14 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar14 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar14 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar14 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar14 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar14 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar14 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar14 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar14 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar14 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar14 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar14 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar14 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar14 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar14[0xf] >> 7) << 0xf;
      lVar6 = lVar6 + -0x40;
      pauVar9 = pauVar9 + 1;
    } while (uVar11 == 0xffff);
    in_RCX[1] = (longlong)pauVar9;
    *in_RCX = lVar6;
    uVar7 = ~(uint)uVar11;
    *(ushort *)(in_RCX + 3) = (ushort)uVar7 & -uVar11 - 2;
    in_RCX[4] = lVar10 + -1;
  }
  else {
    *(ushort *)(in_RCX + 3) = uVar11 - 1 & uVar11;
    in_RCX[4] = lVar10 + -1;
    if (lVar6 == 0) {
      return;
    }
  }
  iVar3 = 0;
  for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
    iVar3 = iVar3 + 1;
  }
  iVar3 = *(int *)((lVar6 - (ulonglong)(uint)(iVar3 << 2)) + -4);
  plVar2 = (longlong *)in_RCX[5];
  if (plVar2[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    bVar12 = (byte)(extraout_RAX >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar10 = 0;
    uVar8 = extraout_RAX;
    while( true ) {
      uVar8 = plVar2[1] & uVar8;
      pcVar1 = (char *)(*plVar2 + uVar8);
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
      uVar11 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
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
      uVar7 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar7; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        if (iVar3 == *(int *)(*plVar2 + -0x50 + (uVar4 + uVar8 & plVar2[1]) * -0x50)) {
          return;
        }
        uVar11 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar11);
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
      uVar8 = uVar8 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

