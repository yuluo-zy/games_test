
void __cdecl toolbar::mei::core::MeiContext::reuse_or_create_node(void)

{
  char *pcVar1;
  undefined4 *puVar2;
  undefined1 (*pauVar3) [16];
  uint uVar4;
  ulonglong extraout_RAX;
  longlong in_RCX;
  uint uVar5;
  uint uVar6;
  longlong in_RDX;
  byte bVar7;
  ushort uVar8;
  byte bVar9;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong uVar12;
  undefined4 uVar13;
  byte bVar14;
  char cVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  longlong local_190;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined4 local_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined1 local_da;
  byte local_d9;
  
  ::core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  bVar14 = (byte)(extraout_RAX >> 0x38);
  bVar7 = bVar14 >> 1;
  auVar16 = ZEXT216(CONCAT11(bVar7,bVar7));
  auVar16 = pshuflw(auVar16,auVar16,0);
  lVar11 = 0;
  uVar10 = extraout_RAX;
  do {
    uVar10 = *(ulonglong *)(in_RCX + 0x1a8) & uVar10;
    pcVar1 = (char *)(*(longlong *)(in_RCX + 0x1a0) + uVar10);
    cVar15 = auVar16[0];
    auVar21[0] = -(*pcVar1 == cVar15);
    cVar17 = auVar16[1];
    auVar21[1] = -(pcVar1[1] == cVar17);
    cVar18 = auVar16[2];
    auVar21[2] = -(pcVar1[2] == cVar18);
    cVar19 = auVar16[3];
    auVar21[3] = -(pcVar1[3] == cVar19);
    auVar21[4] = -(pcVar1[4] == cVar15);
    auVar21[5] = -(pcVar1[5] == cVar17);
    auVar21[6] = -(pcVar1[6] == cVar18);
    auVar21[7] = -(pcVar1[7] == cVar19);
    auVar21[8] = -(pcVar1[8] == cVar15);
    auVar21[9] = -(pcVar1[9] == cVar17);
    auVar21[10] = -(pcVar1[10] == cVar18);
    auVar21[0xb] = -(pcVar1[0xb] == cVar19);
    auVar21[0xc] = -(pcVar1[0xc] == cVar15);
    auVar21[0xd] = -(pcVar1[0xd] == cVar17);
    auVar21[0xe] = -(pcVar1[0xe] == cVar18);
    auVar21[0xf] = -(pcVar1[0xf] == cVar19);
    uVar8 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
    uVar5 = (uint)uVar8;
    while (uVar8 != 0) {
      uVar6 = 0;
      for (uVar4 = uVar5; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar6 = uVar6 + 1;
      }
      if (*(longlong *)
           (*(longlong *)(in_RCX + 0x1a0) + -0xc0 +
           (uVar6 + uVar10 & *(ulonglong *)(in_RCX + 0x1a8)) * -0xc0) == in_RDX) {
        return;
      }
      uVar8 = (ushort)(uVar5 - 1) & (ushort)uVar5;
      uVar5 = CONCAT22((short)(uVar5 - 1 >> 0x10),uVar8);
    }
    auVar20[0] = -(*pcVar1 == -1);
    auVar20[1] = -(pcVar1[1] == -1);
    auVar20[2] = -(pcVar1[2] == -1);
    auVar20[3] = -(pcVar1[3] == -1);
    auVar20[4] = -(pcVar1[4] == -1);
    auVar20[5] = -(pcVar1[5] == -1);
    auVar20[6] = -(pcVar1[6] == -1);
    auVar20[7] = -(pcVar1[7] == -1);
    auVar20[8] = -(pcVar1[8] == -1);
    auVar20[9] = -(pcVar1[9] == -1);
    auVar20[10] = -(pcVar1[10] == -1);
    auVar20[0xb] = -(pcVar1[0xb] == -1);
    auVar20[0xc] = -(pcVar1[0xc] == -1);
    auVar20[0xd] = -(pcVar1[0xd] == -1);
    auVar20[0xe] = -(pcVar1[0xe] == -1);
    auVar20[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar20 >> 0x37,0) & 1) != 0) || (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar20 >> 0x47,0) & 1) != 0) || (SUB161(auVar20 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar20 >> 0x57,0) & 1) != 0) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') {
      if (*(longlong *)(in_RCX + 0x1b0) == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState>_>
                  ();
      }
      ::core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node>_>
                ();
      if (SBORROW8(0,local_190)) {
        local_110 = 0;
        uStack_10c = 0;
        uStack_108 = 0;
        uStack_104 = 0;
        bVar7 = 0;
        local_da = 0;
        uVar13 = 0;
      }
      else {
        bVar7 = local_d9 & 0xf3;
        local_1a8 = local_120;
        uStack_1a0 = uStack_118;
        uVar13 = 1;
        if (local_190 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      pauVar3 = *(undefined1 (**) [16])(in_RCX + 0x1a0);
      uVar10 = *(ulonglong *)(in_RCX + 0x1a8);
      uVar12 = extraout_RAX & uVar10;
      auVar16 = *(undefined1 (*) [16])(*pauVar3 + uVar12);
      uVar8 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
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
              (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe |
              (ushort)(byte)(auVar16[0xf] >> 7) << 0xf;
      uVar5 = (uint)uVar8;
      if (uVar8 == 0) {
        lVar11 = 0x10;
        do {
          uVar12 = uVar12 + lVar11 & uVar10;
          auVar16 = *(undefined1 (*) [16])(*pauVar3 + uVar12);
          lVar11 = lVar11 + 0x10;
          uVar5 = (uint)(ushort)((ushort)(SUB161(auVar16 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
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
                                 (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar16[0xf] >> 7) << 0xf);
        } while (uVar5 == 0);
      }
      uVar6 = 0;
      for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar6 = uVar6 + 1;
      }
      uVar12 = uVar6 + uVar12 & uVar10;
      bVar9 = (*pauVar3)[uVar12];
      if (-1 < (char)bVar9) {
        auVar16 = *pauVar3;
        uVar5 = 0;
        for (uVar6 = (uint)(ushort)((ushort)(SUB161(auVar16 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
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
                                    (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar16[0xf] >> 7) << 0xf); (uVar6 & 1) == 0;
            uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar12 = (ulonglong)uVar5;
        bVar9 = (*pauVar3)[uVar12];
      }
      bVar14 = bVar14 >> 1;
      (*pauVar3)[uVar12] = bVar14;
      pauVar3[1][uVar12 - 0x10 & uVar10] = bVar14;
      *(longlong *)(in_RCX + 0x1b0) = *(longlong *)(in_RCX + 0x1b0) - (ulonglong)(bVar9 & 1);
      *(longlong *)pauVar3[uVar12 * -0xc + -0xc] = in_RDX;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -0xc] + 8) = 0;
      *(undefined8 *)pauVar3[uVar12 * -0xc + -0xb] = 8;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -0xb] + 8) = 0;
      *(undefined8 *)pauVar3[uVar12 * -0xc + -10] = 0;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -10] + 8) = 0;
      *(undefined8 *)pauVar3[uVar12 * -0xc + -9] = 0;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -9] + 8) = 0;
      *(undefined8 *)pauVar3[uVar12 * -0xc + -8] = 0;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -8] + 8) = 0;
      *(undefined8 *)pauVar3[uVar12 * -0xc + -7] = 0;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -7] + 8) = 0;
      *(undefined4 *)pauVar3[uVar12 * -0xc + -6] = 0;
      *(undefined2 *)(pauVar3[uVar12 * -0xc + -6] + 4) = 0;
      pauVar3[uVar12 * -0xc + -6][6] = 4;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -6] + 8) = 0;
      *(undefined8 *)((longlong)(pauVar3[uVar12 * -0xc + -6] + 8) + 8) = 0;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -5] + 8) = 0;
      *(undefined8 *)((longlong)(pauVar3[uVar12 * -0xc + -5] + 8) + 8) = 0;
      puVar2 = (undefined4 *)(pauVar3[uVar12 * -0xc + -4] + 8);
      *puVar2 = local_110;
      puVar2[1] = uStack_10c;
      puVar2[2] = uStack_108;
      puVar2[3] = uStack_104;
      *(undefined4 *)(pauVar3[uVar12 * -0xc + -3] + 8) = uVar13;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -3] + 0xc) = local_1a8;
      *(undefined8 *)((longlong)(pauVar3[uVar12 * -0xc + -3] + 0xc) + 8) = uStack_1a0;
      *(undefined8 *)(pauVar3[uVar12 * -0xc + -2] + 0xc) = 0x3f800000;
      *(undefined8 *)((longlong)(pauVar3[uVar12 * -0xc + -2] + 0xc) + 8) = 0x3f80000000000000;
      *(undefined2 *)(pauVar3[uVar12 * -0xc + -1] + 0xc) = 0xd0d;
      pauVar3[uVar12 * -0xc + -1][0xe] = local_da;
      pauVar3[uVar12 * -0xc + -1][0xf] = bVar7;
      *(longlong *)(in_RCX + 0x1b8) = *(longlong *)(in_RCX + 0x1b8) + 1;
      return;
    }
    uVar10 = uVar10 + lVar11 + 0x10;
    lVar11 = lVar11 + 0x10;
  } while( true );
}

