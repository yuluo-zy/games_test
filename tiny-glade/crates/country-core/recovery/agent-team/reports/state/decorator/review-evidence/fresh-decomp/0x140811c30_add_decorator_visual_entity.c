
void __cdecl system_decorator::decorator_visual_state::impl$5::add_decorator_visual_entity(void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  ulonglong extraout_RAX;
  longlong lVar5;
  ulonglong uVar6;
  undefined8 *in_RCX;
  undefined1 (*pauVar7) [16];
  int in_EDX;
  ulonglong uVar8;
  undefined8 in_R8;
  ushort uVar9;
  longlong lVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  byte bVar13;
  undefined1 (*pauVar14) [16];
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
  pauVar7 = (undefined1 (*) [16])*in_RCX;
  uVar8 = in_RCX[1];
  uVar11 = uVar8 & extraout_RAX;
  bVar13 = (byte)(extraout_RAX >> 0x38);
  bVar15 = bVar13 >> 1;
  auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
  auVar17 = pshuflw(auVar17,auVar17,0);
  lVar10 = 0;
  uVar6 = uVar11;
  while( true ) {
    pcVar1 = *pauVar7 + uVar6;
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
    uVar9 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
    uVar3 = (uint)uVar9;
    while (uVar9 != 0) {
      uVar4 = 0;
      for (uVar2 = uVar3; (uVar2 & 1) == 0; uVar2 = uVar2 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      uVar12 = uVar4 + uVar6 & uVar8;
      if (*(int *)pauVar7[uVar12 * -2 + -2] == in_EDX) {
        pauVar14 = pauVar7 + uVar12 * -2;
        lVar10 = *(longlong *)(pauVar14[-1] + 8);
        if (lVar10 == *(longlong *)(pauVar14[-2] + 8)) goto LAB_140811d41;
        goto LAB_140811d51;
      }
      uVar9 = (ushort)(uVar3 - 1) & (ushort)uVar3;
      uVar3 = CONCAT22((short)(uVar3 - 1 >> 0x10),uVar9);
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
        (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') break;
    lVar5 = uVar6 + lVar10;
    lVar10 = lVar10 + 0x10;
    uVar6 = lVar5 + 0x10U & uVar8;
  }
  if (in_RCX[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<country_core::resources::clutter_meshes::DisplayChildAutoClutterInfo,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<country_core::resources::clutter_meshes::DisplayChildAutoClutterInfo,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
    pauVar7 = (undefined1 (*) [16])*in_RCX;
    uVar8 = in_RCX[1];
    uVar11 = extraout_RAX & uVar8;
    auVar17 = *(undefined1 (*) [16])(*pauVar7 + uVar11);
    uVar3 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
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
    if (uVar3 != 0) goto LAB_140811d8e;
  }
  else {
    auVar17 = *(undefined1 (*) [16])(*pauVar7 + uVar11);
    uVar9 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
    uVar3 = (uint)uVar9;
    if (uVar9 != 0) goto LAB_140811d8e;
  }
  lVar10 = 0x10;
  do {
    uVar11 = uVar11 + lVar10 & uVar8;
    auVar17 = *(undefined1 (*) [16])(*pauVar7 + uVar11);
    lVar10 = lVar10 + 0x10;
    uVar3 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
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
  } while (uVar3 == 0);
LAB_140811d8e:
  uVar4 = 0;
  for (; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
    uVar4 = uVar4 + 1;
  }
  uVar6 = uVar4 + uVar11 & uVar8;
  bVar15 = (*pauVar7)[uVar6];
  if (-1 < (char)bVar15) {
    auVar17 = *pauVar7;
    uVar3 = 0;
    for (uVar4 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
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
                               (ushort)(byte)(auVar17[0xf] >> 7) << 0xf); (uVar4 & 1) == 0;
        uVar4 = uVar4 >> 1 | 0x80000000) {
      uVar3 = uVar3 + 1;
    }
    uVar6 = (ulonglong)uVar3;
    bVar15 = (*pauVar7)[uVar6];
  }
  bVar13 = bVar13 >> 1;
  (*pauVar7)[uVar6] = bVar13;
  pauVar7[1][uVar6 - 0x10 & uVar8] = bVar13;
  pauVar14 = pauVar7 + uVar6 * -2;
  in_RCX[2] = in_RCX[2] - (ulonglong)(bVar15 & 1);
  *(int *)pauVar7[uVar6 * -2 + -2] = in_EDX;
  *(undefined8 *)(pauVar14[-2] + 8) = 0;
  *(undefined8 *)pauVar14[-1] = 8;
  *(undefined8 *)(pauVar14[-1] + 8) = 0;
  in_RCX[3] = in_RCX[3] + 1;
  lVar10 = *(longlong *)(pauVar14[-1] + 8);
  if (lVar10 == *(longlong *)(pauVar14[-2] + 8)) {
LAB_140811d41:
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
  }
LAB_140811d51:
  *(undefined8 *)(*(longlong *)pauVar14[-1] + lVar10 * 8) = in_R8;
  *(longlong *)(pauVar14[-1] + 8) = lVar10 + 1;
  return;
}

