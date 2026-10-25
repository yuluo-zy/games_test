// decorator_cull_allow_wall  size=292
// void __cdecl decorator_cull_allow_wall(void)

void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
          (void)

{
  char *pcVar1;
  longlong lVar2;
  uint uVar3;
  uint uVar4;
  ulonglong extraout_RAX;
  longlong in_RCX;
  longlong in_RDX;
  longlong *in_R8;
  ulonglong uVar5;
  longlong lVar6;
  uint uVar7;
  ushort uVar8;
  longlong lVar9;
  byte bVar10;
  char cVar11;
  char cVar13;
  char cVar14;
  char cVar15;
  undefined1 auVar12 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  longlong local_48;
  longlong *local_40;
  undefined1 (*local_38) [16];
  undefined1 (*local_30) [16];
  undefined1 *local_28;
  ushort local_20;
  
  if (in_R8[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar2 = *in_R8;
    bVar10 = (byte)(extraout_RAX >> 0x39);
    auVar12 = ZEXT216(CONCAT11(bVar10,bVar10));
    auVar12 = pshuflw(auVar12,auVar12,0);
    lVar6 = 0;
    uVar5 = extraout_RAX;
    while( true ) {
      uVar5 = in_R8[1] & uVar5;
      pcVar1 = (char *)(lVar2 + uVar5);
      cVar11 = auVar12[0];
      auVar17[0] = -(*pcVar1 == cVar11);
      cVar13 = auVar12[1];
      auVar17[1] = -(pcVar1[1] == cVar13);
      cVar14 = auVar12[2];
      auVar17[2] = -(pcVar1[2] == cVar14);
      cVar15 = auVar12[3];
      auVar17[3] = -(pcVar1[3] == cVar15);
      auVar17[4] = -(pcVar1[4] == cVar11);
      auVar17[5] = -(pcVar1[5] == cVar13);
      auVar17[6] = -(pcVar1[6] == cVar14);
      auVar17[7] = -(pcVar1[7] == cVar15);
      auVar17[8] = -(pcVar1[8] == cVar11);
      auVar17[9] = -(pcVar1[9] == cVar13);
      auVar17[10] = -(pcVar1[10] == cVar14);
      auVar17[0xb] = -(pcVar1[0xb] == cVar15);
      auVar17[0xc] = -(pcVar1[0xc] == cVar11);
      auVar17[0xd] = -(pcVar1[0xd] == cVar13);
      auVar17[0xe] = -(pcVar1[0xe] == cVar14);
      auVar17[0xf] = -(pcVar1[0xf] == cVar15);
      uVar8 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
      uVar7 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        lVar9 = (uVar3 + uVar5 & in_R8[1]) * -0x98;
        if (*(longlong *)(lVar2 + -0x98 + lVar9) == in_RDX) {
          local_48 = in_RCX + 0x30;
          local_38 = *(undefined1 (**) [16])(lVar2 + -0x60 + lVar9);
          local_28 = *local_38 + *(longlong *)(lVar2 + -0x58 + lVar9) + 1;
          auVar12 = *local_38;
          local_20 = ~((ushort)(SUB161(auVar12 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar12 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar12 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar12 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar12 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar12 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar12 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar12 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar12 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar12 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar12 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar12 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar12 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar12 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar12[0xf] >> 7) << 0xf);
          local_40 = &local_48;
          local_30 = local_38 + 1;
          _ZN9hashbrown3raw21RawIterRange_LT_T_GT_9fold_impl17hac763e63c0aff34fE();
          return;
        }
        uVar8 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar8);
      }
      auVar16[0] = -(*pcVar1 == -1);
      auVar16[1] = -(pcVar1[1] == -1);
      auVar16[2] = -(pcVar1[2] == -1);
      auVar16[3] = -(pcVar1[3] == -1);
      auVar16[4] = -(pcVar1[4] == -1);
      auVar16[5] = -(pcVar1[5] == -1);
      auVar16[6] = -(pcVar1[6] == -1);
      auVar16[7] = -(pcVar1[7] == -1);
      auVar16[8] = -(pcVar1[8] == -1);
      auVar16[9] = -(pcVar1[9] == -1);
      auVar16[10] = -(pcVar1[10] == -1);
      auVar16[0xb] = -(pcVar1[0xb] == -1);
      auVar16[0xc] = -(pcVar1[0xc] == -1);
      auVar16[0xd] = -(pcVar1[0xd] == -1);
      auVar16[0xe] = -(pcVar1[0xe] == -1);
      auVar16[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar16 >> 7,0) & 1) != 0 || (SUB161(auVar16 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar16 >> 0x17,0) & 1) != 0) || (SUB161(auVar16 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar16 >> 0x27,0) & 1) != 0) || (SUB161(auVar16 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar16 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar16 >> 0x3f,0) & 1) != 0) || (SUB161(auVar16 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar16 >> 0x4f,0) & 1) != 0) || (SUB161(auVar16 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar16 >> 0x5f,0) & 1) != 0) || (SUB161(auVar16 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar16 >> 0x6f,0) & 1) != 0) || (SUB161(auVar16 >> 0x77,0) & 1) != 0) ||
          auVar16[0xf] < '\0') break;
      uVar5 = uVar5 + lVar6 + 0x10;
      lVar6 = lVar6 + 0x10;
    }
  }
  return;
}



// CALLEES: [{'name': '_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9fold_impl17hac763e63c0aff34fE', 'address': '140b76260'}, {'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}]