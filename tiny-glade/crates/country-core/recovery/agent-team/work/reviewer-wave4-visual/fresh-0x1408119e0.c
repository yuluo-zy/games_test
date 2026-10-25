
void __cdecl system_decorator::decorator_visual_state::impl$4::add_decorator_visual_entity(void)

{
  char *pcVar1;
  ulonglong *puVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  undefined1 (*pauVar6) [16];
  uint in_EDX;
  ulonglong uVar7;
  ushort uVar8;
  undefined8 in_R8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  byte bVar14;
  byte bVar15;
  char cVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  pauVar6 = (undefined1 (*) [16])*in_RCX;
  uVar7 = in_RCX[1];
  uVar9 = uVar7 & extraout_RAX;
  bVar14 = (byte)(extraout_RAX >> 0x38);
  bVar15 = bVar14 >> 1;
  auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
  auVar17 = pshuflw(auVar17,auVar17,0);
  lVar10 = 0;
  uVar12 = uVar9;
  do {
    pcVar1 = *pauVar6 + uVar12;
    cVar16 = auVar17[0];
    auVar22[0] = -(*pcVar1 == cVar16);
    cVar18 = auVar17[1];
    auVar22[1] = -(pcVar1[1] == cVar18);
    cVar19 = auVar17[2];
    auVar22[2] = -(pcVar1[2] == cVar19);
    cVar20 = auVar17[3];
    auVar22[3] = -(pcVar1[3] == cVar20);
    auVar22[4] = -(pcVar1[4] == cVar16);
    auVar22[5] = -(pcVar1[5] == cVar18);
    auVar22[6] = -(pcVar1[6] == cVar19);
    auVar22[7] = -(pcVar1[7] == cVar20);
    auVar22[8] = -(pcVar1[8] == cVar16);
    auVar22[9] = -(pcVar1[9] == cVar18);
    auVar22[10] = -(pcVar1[10] == cVar19);
    auVar22[0xb] = -(pcVar1[0xb] == cVar20);
    auVar22[0xc] = -(pcVar1[0xc] == cVar16);
    auVar22[0xd] = -(pcVar1[0xd] == cVar18);
    auVar22[0xe] = -(pcVar1[0xe] == cVar19);
    auVar22[0xf] = -(pcVar1[0xf] == cVar20);
    uVar8 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
    uVar4 = (uint)uVar8;
    while (uVar8 != 0) {
      uVar5 = 0;
      for (uVar3 = uVar4; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
        uVar5 = uVar5 + 1;
      }
      uVar13 = uVar5 + uVar12 & uVar7;
      if (*(uint *)((longlong)pauVar6 + uVar13 * -0x28 + -0x28) == in_EDX) {
        lVar10 = (longlong)pauVar6 + uVar13 * -0x28;
        bVar14 = *(byte *)(lVar10 + -0x20);
        goto joined_r0x000140811ab5;
      }
      uVar8 = (ushort)(uVar4 - 1) & (ushort)uVar4;
      uVar4 = CONCAT22((short)(uVar4 - 1 >> 0x10),uVar8);
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
      if (in_RCX[2] == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual_state::PersistentVisualEntities>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual_state::PersistentVisualEntities>,std::hash::random::RandomState>_>
                  ();
        pauVar6 = (undefined1 (*) [16])*in_RCX;
        uVar7 = in_RCX[1];
        uVar9 = extraout_RAX & uVar7;
      }
      auVar17 = *(undefined1 (*) [16])(*pauVar6 + uVar9);
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
              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
              (ushort)(byte)(auVar17[0xf] >> 7) << 0xf;
      uVar4 = (uint)uVar8;
      if (uVar8 == 0) {
        lVar10 = 0x10;
        do {
          uVar9 = uVar9 + lVar10 & uVar7;
          auVar17 = *(undefined1 (*) [16])(*pauVar6 + uVar9);
          lVar10 = lVar10 + 0x10;
          uVar4 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
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
                                (ushort)(byte)(auVar17[0xf] >> 7) << 0xf);
        } while (uVar4 == 0);
      }
      uVar5 = 0;
      for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar5 = uVar5 + 1;
      }
      uVar12 = uVar5 + uVar9 & uVar7;
      bVar15 = (*pauVar6)[uVar12];
      if (-1 < (char)bVar15) {
        auVar17 = *pauVar6;
        uVar4 = 0;
        for (uVar5 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
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
                                   (ushort)(byte)(auVar17[0xf] >> 7) << 0xf); (uVar5 & 1) == 0;
            uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar12 = (ulonglong)uVar4;
        bVar15 = (*pauVar6)[uVar12];
      }
      bVar14 = bVar14 >> 1;
      (*pauVar6)[uVar12] = bVar14;
      pauVar6[1][uVar12 - 0x10 & uVar7] = bVar14;
      in_RCX[2] = in_RCX[2] - (ulonglong)(bVar15 & 1);
      lVar10 = (longlong)pauVar6 + uVar12 * -0x28;
      *(uint *)((longlong)pauVar6 + uVar12 * -0x28 + -0x28) = in_EDX;
      puVar2 = (ulonglong *)((longlong)pauVar6 + uVar12 * -0x28 + -0x24);
      *puVar2 = (ulonglong)in_EDX;
      puVar2[1] = 0;
      *(undefined4 *)((longlong)pauVar6 + uVar12 * -0x28 + -0x14) = 0;
      *(undefined8 *)((longlong)pauVar6 + uVar12 * -0x28 + -0x10) = 8;
      *(undefined8 *)((longlong)pauVar6 + uVar12 * -0x28 + -8) = 0;
      in_RCX[3] = in_RCX[3] + 1;
      bVar14 = *(byte *)(lVar10 + -0x20);
joined_r0x000140811ab5:
      if ((bVar14 & 1) != 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      lVar11 = *(longlong *)(lVar10 + -8);
      if (lVar11 == *(longlong *)(lVar10 + -0x18)) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      *(undefined8 *)(*(longlong *)(lVar10 + -0x10) + lVar11 * 8) = in_R8;
      *(longlong *)(lVar10 + -8) = lVar11 + 1;
      return;
    }
    lVar11 = uVar12 + lVar10;
    lVar10 = lVar10 + 0x10;
    uVar12 = lVar11 + 0x10U & uVar7;
  } while( true );
}

