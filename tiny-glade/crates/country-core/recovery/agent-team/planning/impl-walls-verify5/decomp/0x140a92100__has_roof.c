// has_roof  size=417
// void __cdecl has_roof(void)

void __cdecl country_core::resources::walls::public_walls::PublicWalls::has_roof(void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong in_RCX;
  longlong in_RDX;
  ulonglong uVar4;
  longlong lVar5;
  uint uVar6;
  ushort uVar7;
  byte bVar8;
  char cVar9;
  char cVar11;
  char cVar12;
  char cVar13;
  undefined1 auVar10 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  
  if (*(longlong *)(in_RCX + 0x78) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar8 = (byte)(extraout_RAX >> 0x39);
    auVar10 = ZEXT216(CONCAT11(bVar8,bVar8));
    auVar10 = pshuflw(auVar10,auVar10,0);
    lVar5 = 0;
    uVar4 = extraout_RAX;
    while( true ) {
      uVar4 = *(ulonglong *)(in_RCX + 0x68) & uVar4;
      pcVar1 = (char *)(*(longlong *)(in_RCX + 0x60) + uVar4);
      cVar9 = auVar10[0];
      auVar16[0] = -(*pcVar1 == cVar9);
      cVar11 = auVar10[1];
      auVar16[1] = -(pcVar1[1] == cVar11);
      cVar12 = auVar10[2];
      auVar16[2] = -(pcVar1[2] == cVar12);
      cVar13 = auVar10[3];
      auVar16[3] = -(pcVar1[3] == cVar13);
      auVar16[4] = -(pcVar1[4] == cVar9);
      auVar16[5] = -(pcVar1[5] == cVar11);
      auVar16[6] = -(pcVar1[6] == cVar12);
      auVar16[7] = -(pcVar1[7] == cVar13);
      auVar16[8] = -(pcVar1[8] == cVar9);
      auVar16[9] = -(pcVar1[9] == cVar11);
      auVar16[10] = -(pcVar1[10] == cVar12);
      auVar16[0xb] = -(pcVar1[0xb] == cVar13);
      auVar16[0xc] = -(pcVar1[0xc] == cVar9);
      auVar16[0xd] = -(pcVar1[0xd] == cVar11);
      auVar16[0xe] = -(pcVar1[0xe] == cVar12);
      auVar16[0xf] = -(pcVar1[0xf] == cVar13);
      uVar7 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe | (ushort)(auVar16[0xf] >> 7) << 0xf;
      uVar6 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar6; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        if (*(longlong *)
             (*(longlong *)(in_RCX + 0x60) + -0x10 +
             (uVar2 + uVar4 & *(ulonglong *)(in_RCX + 0x68)) * -0x10) == in_RDX) goto LAB_140a921d0;
        uVar7 = (ushort)(uVar6 - 1) & (ushort)uVar6;
        uVar6 = CONCAT22((short)(uVar6 - 1 >> 0x10),uVar7);
      }
      auVar14[0] = -(*pcVar1 == -1);
      auVar14[1] = -(pcVar1[1] == -1);
      auVar14[2] = -(pcVar1[2] == -1);
      auVar14[3] = -(pcVar1[3] == -1);
      auVar14[4] = -(pcVar1[4] == -1);
      auVar14[5] = -(pcVar1[5] == -1);
      auVar14[6] = -(pcVar1[6] == -1);
      auVar14[7] = -(pcVar1[7] == -1);
      auVar14[8] = -(pcVar1[8] == -1);
      auVar14[9] = -(pcVar1[9] == -1);
      auVar14[10] = -(pcVar1[10] == -1);
      auVar14[0xb] = -(pcVar1[0xb] == -1);
      auVar14[0xc] = -(pcVar1[0xc] == -1);
      auVar14[0xd] = -(pcVar1[0xd] == -1);
      auVar14[0xe] = -(pcVar1[0xe] == -1);
      auVar14[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar14 >> 7,0) & 1) != 0 || (SUB161(auVar14 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar14 >> 0x17,0) & 1) != 0) || (SUB161(auVar14 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar14 >> 0x27,0) & 1) != 0) || (SUB161(auVar14 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar14 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar14 >> 0x3f,0) & 1) != 0) || (SUB161(auVar14 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar14 >> 0x4f,0) & 1) != 0) || (SUB161(auVar14 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar14 >> 0x5f,0) & 1) != 0) || (SUB161(auVar14 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar14 >> 0x6f,0) & 1) != 0) || (SUB161(auVar14 >> 0x77,0) & 1) != 0) ||
          auVar14[0xf] < '\0') break;
      uVar4 = uVar4 + lVar5 + 0x10;
      lVar5 = lVar5 + 0x10;
    }
  }
LAB_140a921d0:
  if (*(longlong *)(in_RCX + 0x48) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar8 = (byte)(extraout_RAX_00 >> 0x39);
    auVar10 = ZEXT216(CONCAT11(bVar8,bVar8));
    auVar10 = pshuflw(auVar10,auVar10,0);
    lVar5 = 0;
    uVar4 = extraout_RAX_00;
    while( true ) {
      uVar4 = *(ulonglong *)(in_RCX + 0x38) & uVar4;
      pcVar1 = (char *)(*(longlong *)(in_RCX + 0x30) + uVar4);
      cVar9 = auVar10[0];
      auVar17[0] = -(*pcVar1 == cVar9);
      cVar11 = auVar10[1];
      auVar17[1] = -(pcVar1[1] == cVar11);
      cVar12 = auVar10[2];
      auVar17[2] = -(pcVar1[2] == cVar12);
      cVar13 = auVar10[3];
      auVar17[3] = -(pcVar1[3] == cVar13);
      auVar17[4] = -(pcVar1[4] == cVar9);
      auVar17[5] = -(pcVar1[5] == cVar11);
      auVar17[6] = -(pcVar1[6] == cVar12);
      auVar17[7] = -(pcVar1[7] == cVar13);
      auVar17[8] = -(pcVar1[8] == cVar9);
      auVar17[9] = -(pcVar1[9] == cVar11);
      auVar17[10] = -(pcVar1[10] == cVar12);
      auVar17[0xb] = -(pcVar1[0xb] == cVar13);
      auVar17[0xc] = -(pcVar1[0xc] == cVar9);
      auVar17[0xd] = -(pcVar1[0xd] == cVar11);
      auVar17[0xe] = -(pcVar1[0xe] == cVar12);
      auVar17[0xf] = -(pcVar1[0xf] == cVar13);
      uVar7 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
      uVar6 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar6; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        if (*(longlong *)
             (*(longlong *)(in_RCX + 0x30) + -0x10 +
             (uVar2 + uVar4 & *(ulonglong *)(in_RCX + 0x38)) * -0x10) == in_RDX) {
          return;
        }
        uVar7 = (ushort)(uVar6 - 1) & (ushort)uVar6;
        uVar6 = CONCAT22((short)(uVar6 - 1 >> 0x10),uVar7);
      }
      auVar15[0] = -(*pcVar1 == -1);
      auVar15[1] = -(pcVar1[1] == -1);
      auVar15[2] = -(pcVar1[2] == -1);
      auVar15[3] = -(pcVar1[3] == -1);
      auVar15[4] = -(pcVar1[4] == -1);
      auVar15[5] = -(pcVar1[5] == -1);
      auVar15[6] = -(pcVar1[6] == -1);
      auVar15[7] = -(pcVar1[7] == -1);
      auVar15[8] = -(pcVar1[8] == -1);
      auVar15[9] = -(pcVar1[9] == -1);
      auVar15[10] = -(pcVar1[10] == -1);
      auVar15[0xb] = -(pcVar1[0xb] == -1);
      auVar15[0xc] = -(pcVar1[0xc] == -1);
      auVar15[0xd] = -(pcVar1[0xd] == -1);
      auVar15[0xe] = -(pcVar1[0xe] == -1);
      auVar15[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar15 >> 7,0) & 1) != 0 || (SUB161(auVar15 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar15 >> 0x17,0) & 1) != 0) || (SUB161(auVar15 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar15 >> 0x27,0) & 1) != 0) || (SUB161(auVar15 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar15 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar15 >> 0x3f,0) & 1) != 0) || (SUB161(auVar15 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar15 >> 0x4f,0) & 1) != 0) || (SUB161(auVar15 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar15 >> 0x5f,0) & 1) != 0) || (SUB161(auVar15 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar15 >> 0x6f,0) & 1) != 0) || (SUB161(auVar15 >> 0x77,0) & 1) != 0) ||
          auVar15[0xf] < '\0') break;
      uVar4 = uVar4 + lVar5 + 0x10;
      lVar5 = lVar5 + 0x10;
    }
  }
  return;
}



// CALLEES: [{'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}]