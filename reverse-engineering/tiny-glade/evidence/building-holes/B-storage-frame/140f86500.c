
void __cdecl country_core::resources::walls::wall_holes::WallHoles::add(void)

{
  char *pcVar1;
  ulonglong *puVar2;
  undefined8 *puVar3;
  int iVar4;
  longlong lVar5;
  uint uVar6;
  undefined8 uVar7;
  uint uVar8;
  uint uVar9;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 *in_RCX;
  undefined1 (*pauVar10) [16];
  longlong in_RDX;
  ulonglong uVar11;
  ushort uVar12;
  longlong lVar13;
  undefined8 *in_R8;
  ulonglong uVar14;
  undefined1 in_R9B;
  longlong lVar15;
  ulonglong uVar16;
  byte bVar17;
  byte bVar18;
  char cVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  uint3 uStack_63;
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  pauVar10 = (undefined1 (*) [16])*in_RCX;
  uVar11 = in_RCX[1];
  uVar14 = uVar11 & extraout_RAX;
  bVar17 = (byte)(extraout_RAX >> 0x38);
  bVar18 = bVar17 >> 1;
  auVar20 = ZEXT216(CONCAT11(bVar18,bVar18));
  auVar20 = pshuflw(auVar20,auVar20,0);
  lVar15 = 0;
  uVar16 = uVar14;
  do {
    pcVar1 = *pauVar10 + uVar16;
    cVar19 = auVar20[0];
    auVar25[0] = -(*pcVar1 == cVar19);
    cVar21 = auVar20[1];
    auVar25[1] = -(pcVar1[1] == cVar21);
    cVar22 = auVar20[2];
    auVar25[2] = -(pcVar1[2] == cVar22);
    cVar23 = auVar20[3];
    auVar25[3] = -(pcVar1[3] == cVar23);
    auVar25[4] = -(pcVar1[4] == cVar19);
    auVar25[5] = -(pcVar1[5] == cVar21);
    auVar25[6] = -(pcVar1[6] == cVar22);
    auVar25[7] = -(pcVar1[7] == cVar23);
    auVar25[8] = -(pcVar1[8] == cVar19);
    auVar25[9] = -(pcVar1[9] == cVar21);
    auVar25[10] = -(pcVar1[10] == cVar22);
    auVar25[0xb] = -(pcVar1[0xb] == cVar23);
    auVar25[0xc] = -(pcVar1[0xc] == cVar19);
    auVar25[0xd] = -(pcVar1[0xd] == cVar21);
    auVar25[0xe] = -(pcVar1[0xe] == cVar22);
    auVar25[0xf] = -(pcVar1[0xf] == cVar23);
    uVar12 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
    uVar8 = (uint)uVar12;
    while (uVar12 != 0) {
      uVar9 = 0;
      for (uVar6 = uVar8; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      lVar13 = (uVar9 + uVar16 & uVar11) * -0x68;
      if (*(longlong *)((longlong)pauVar10 + lVar13 + -0x68) == in_RDX) {
        lVar13 = (longlong)pauVar10 + lVar13;
        iVar4 = *(int *)(lVar13 + -0x60);
        goto joined_r0x000140f865db;
      }
      uVar12 = (ushort)(uVar8 - 1) & (ushort)uVar8;
      uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar12);
    }
    auVar24[0] = -(*pcVar1 == -1);
    auVar24[1] = -(pcVar1[1] == -1);
    auVar24[2] = -(pcVar1[2] == -1);
    auVar24[3] = -(pcVar1[3] == -1);
    auVar24[4] = -(pcVar1[4] == -1);
    auVar24[5] = -(pcVar1[5] == -1);
    auVar24[6] = -(pcVar1[6] == -1);
    auVar24[7] = -(pcVar1[7] == -1);
    auVar24[8] = -(pcVar1[8] == -1);
    auVar24[9] = -(pcVar1[9] == -1);
    auVar24[10] = -(pcVar1[10] == -1);
    auVar24[0xb] = -(pcVar1[0xb] == -1);
    auVar24[0xc] = -(pcVar1[0xc] == -1);
    auVar24[0xd] = -(pcVar1[0xd] == -1);
    auVar24[0xe] = -(pcVar1[0xe] == -1);
    auVar24[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x17,0) & 1) != 0) || (SUB161(auVar24 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x27,0) & 1) != 0) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') {
      if (in_RCX[2] == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::resources::walls::WallId,enum2_<country_core::resources::walls::wall_holes::WallHoleStorage>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,enum2_<country_core::resources::walls::wall_holes::WallHoleStorage>,std::hash::random::RandomState>_>
                  ();
        pauVar10 = (undefined1 (*) [16])*in_RCX;
        uVar11 = in_RCX[1];
        uVar14 = extraout_RAX & uVar11;
      }
      auVar20 = *(undefined1 (*) [16])(*pauVar10 + uVar14);
      uVar12 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
               (ushort)(byte)(auVar20[0xf] >> 7) << 0xf;
      uVar8 = (uint)uVar12;
      if (uVar12 == 0) {
        lVar15 = 0x10;
        do {
          uVar14 = uVar14 + lVar15 & uVar11;
          auVar20 = *(undefined1 (*) [16])(*pauVar10 + uVar14);
          lVar15 = lVar15 + 0x10;
          uVar8 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
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
                                (ushort)(byte)(auVar20[0xf] >> 7) << 0xf);
        } while (uVar8 == 0);
      }
      uVar9 = 0;
      for (; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      uVar16 = uVar9 + uVar14 & uVar11;
      bVar18 = (*pauVar10)[uVar16];
      if (-1 < (char)bVar18) {
        auVar20 = *pauVar10;
        uVar8 = 0;
        for (uVar9 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
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
                                   (ushort)(byte)(auVar20[0xf] >> 7) << 0xf); (uVar9 & 1) == 0;
            uVar9 = uVar9 >> 1 | 0x80000000) {
          uVar8 = uVar8 + 1;
        }
        uVar16 = (ulonglong)uVar8;
        bVar18 = (*pauVar10)[uVar16];
      }
      bVar17 = bVar17 >> 1;
      (*pauVar10)[uVar16] = bVar17;
      pauVar10[1][uVar16 - 0x10 & uVar11] = bVar17;
      in_RCX[2] = in_RCX[2] - (ulonglong)(bVar18 & 1);
      lVar15 = uVar16 * -0x68;
      lVar13 = (longlong)pauVar10 + lVar15;
      *(longlong *)((longlong)pauVar10 + lVar15 + -0x68) = in_RDX;
      *(undefined1 (*) [16])((longlong)pauVar10 + lVar15 + -0x60) = (undefined1  [16])0x0;
      *(undefined8 *)((longlong)pauVar10 + lVar15 + -0x50) = 4;
      *(undefined1 (*) [16])((longlong)pauVar10 + lVar15 + -0x48) = (undefined1  [16])0x0;
      *(undefined8 *)((longlong)pauVar10 + lVar15 + -0x38) = 4;
      *(undefined8 *)((longlong)pauVar10 + lVar15 + -0x30) = 0;
      *(undefined4 *)((longlong)pauVar10 + lVar15 + -0x28) = 0;
      *(undefined1 *)((longlong)pauVar10 + lVar15 + -0x24) = 0;
      puVar2 = (ulonglong *)((longlong)pauVar10 + lVar15 + -0x23);
      *puVar2 = (ulonglong)uStack_63;
      puVar2[1] = 0;
      puVar3 = (undefined8 *)((longlong)pauVar10 + lVar15 + -0x13);
      *puVar3 = 0;
      puVar3[1] = 0;
      *(undefined4 *)((longlong)pauVar10 + lVar15 + -4) = 0;
      in_RCX[3] = in_RCX[3] + 1;
      iVar4 = *(int *)(lVar13 + -0x60);
joined_r0x000140f865db:
      if (iVar4 == 1) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      ::utils::
      calculate_hash<tuple_<array_<u32,2>,array_<u32,2>,country_core::resources::walls::wall_holes::HoleType>_>
                ();
      *(longlong *)(lVar13 + -0x20) = *(longlong *)(lVar13 + -0x20) + extraout_RAX_00;
      *(ulonglong *)(lVar13 + -0x18) = *(ulonglong *)(lVar13 + -0x18) ^ extraout_RAX_00;
      lVar15 = *(longlong *)(lVar13 + -0x48);
      if (lVar15 == *(longlong *)(lVar13 + -0x58)) {
        alloc::raw_vec::RawVec::
        grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
      }
      lVar5 = *(longlong *)(lVar13 + -0x50);
      uVar7 = in_R8[1];
      puVar3 = (undefined8 *)(lVar5 + lVar15 * 0x14);
      *puVar3 = *in_R8;
      puVar3[1] = uVar7;
      *(undefined1 *)(lVar5 + 0x10 + lVar15 * 0x14) = in_R9B;
      *(longlong *)(lVar13 + -0x48) = lVar15 + 1;
      return;
    }
    lVar13 = uVar16 + lVar15;
    lVar15 = lVar15 + 0x10;
    uVar16 = lVar13 + 0x10U & uVar11;
  } while( true );
}

