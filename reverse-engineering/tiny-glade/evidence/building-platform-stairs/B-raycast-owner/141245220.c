
void __cdecl
country_core::systems::collision::world::RaycastWorld::
remove_collider<country_core::resources::walls::WallId>(void)

{
  char *pcVar1;
  longlong lVar2;
  short sVar3;
  uint uVar4;
  uint uVar5;
  ushort uVar6;
  ulonglong extraout_RAX;
  ulonglong uVar7;
  longlong in_RCX;
  undefined1 uVar8;
  uint uVar9;
  longlong *in_R8;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong in_R9;
  ushort uVar12;
  longlong lVar13;
  byte bVar14;
  char cVar15;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar2 = *in_R8;
  uVar11 = in_R8[1];
  bVar14 = (byte)(extraout_RAX >> 0x39);
  auVar16 = ZEXT216(CONCAT11(bVar14,bVar14));
  auVar16 = pshuflw(auVar16,auVar16,0);
  lVar13 = 0;
  uVar10 = extraout_RAX;
  do {
    uVar10 = uVar11 & uVar10;
    pcVar1 = (char *)(lVar2 + uVar10);
    cVar15 = auVar16[0];
    auVar22[0] = -(*pcVar1 == cVar15);
    cVar18 = auVar16[1];
    auVar22[1] = -(pcVar1[1] == cVar18);
    cVar19 = auVar16[2];
    auVar22[2] = -(pcVar1[2] == cVar19);
    cVar20 = auVar16[3];
    auVar22[3] = -(pcVar1[3] == cVar20);
    auVar22[4] = -(pcVar1[4] == cVar15);
    auVar22[5] = -(pcVar1[5] == cVar18);
    auVar22[6] = -(pcVar1[6] == cVar19);
    auVar22[7] = -(pcVar1[7] == cVar20);
    auVar22[8] = -(pcVar1[8] == cVar15);
    auVar22[9] = -(pcVar1[9] == cVar18);
    auVar22[10] = -(pcVar1[10] == cVar19);
    auVar22[0xb] = -(pcVar1[0xb] == cVar20);
    auVar22[0xc] = -(pcVar1[0xc] == cVar15);
    auVar22[0xd] = -(pcVar1[0xd] == cVar18);
    auVar22[0xe] = -(pcVar1[0xe] == cVar19);
    auVar22[0xf] = -(pcVar1[0xf] == cVar20);
    uVar6 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf;
    uVar9 = (uint)uVar6;
    while (uVar6 != 0) {
      uVar4 = 0;
      for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      uVar7 = uVar4 + uVar10 & uVar11;
      if (*(longlong *)(lVar2 + -0x20 + uVar7 * -0x20) == in_R9) {
        lVar13 = (longlong)(uVar7 * 0x20) >> 5;
        uVar11 = lVar13 - 0x10U & uVar11;
        pcVar1 = (char *)(lVar2 + uVar11);
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
        uVar6 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
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
        pcVar1 = (char *)(lVar2 + lVar13);
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
        if (uVar6 == 0) {
          uVar12 = 0x10;
        }
        else {
          uVar12 = 0xf;
          if (uVar6 != 0) {
            for (; uVar6 >> uVar12 == 0; uVar12 = uVar12 - 1) {
            }
          }
          uVar12 = uVar12 ^ 0xf;
        }
        sVar3 = 0;
        for (uVar9 = (ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
                              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
                             (ushort)(auVar17[0xf] >> 7) << 0xf) | 0x10000; (uVar9 & 1) == 0;
            uVar9 = uVar9 >> 1 | 0x80000000) {
          sVar3 = sVar3 + 1;
        }
        uVar8 = 0x80;
        if ((ushort)(uVar12 + sVar3) < 0x10) {
          in_R8[2] = in_R8[2] + 1;
          uVar8 = 0xff;
        }
        *(undefined1 *)(lVar2 + lVar13) = uVar8;
        *(undefined1 *)(lVar2 + 0x10 + uVar11) = uVar8;
        in_R8[3] = in_R8[3] + -1;
        dispose_collider_mesh_instance();
        return;
      }
      uVar6 = (ushort)(uVar9 - 1) & (ushort)uVar9;
      uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar6);
    }
    auVar21[0] = -(*pcVar1 == -1);
    auVar21[1] = -(pcVar1[1] == -1);
    auVar21[2] = -(pcVar1[2] == -1);
    auVar21[3] = -(pcVar1[3] == -1);
    auVar21[4] = -(pcVar1[4] == -1);
    auVar21[5] = -(pcVar1[5] == -1);
    auVar21[6] = -(pcVar1[6] == -1);
    auVar21[7] = -(pcVar1[7] == -1);
    auVar21[8] = -(pcVar1[8] == -1);
    auVar21[9] = -(pcVar1[9] == -1);
    auVar21[10] = -(pcVar1[10] == -1);
    auVar21[0xb] = -(pcVar1[0xb] == -1);
    auVar21[0xc] = -(pcVar1[0xc] == -1);
    auVar21[0xd] = -(pcVar1[0xd] == -1);
    auVar21[0xe] = -(pcVar1[0xe] == -1);
    auVar21[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x37,0) & 1) != 0) || (SUB161(auVar21 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar21 >> 0x47,0) & 1) != 0) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') {
      if (in_R8[2] == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
                  ();
      }
      *(undefined1 *)(in_RCX + 0x20) = 0x11;
      return;
    }
    uVar10 = uVar10 + lVar13 + 0x10;
    lVar13 = lVar13 + 0x10;
  } while( true );
}

