
// WARNING: Removing unreachable block (ram,0x00014078881c)
// WARNING: Removing unreachable block (ram,0x000140788873)
// WARNING: Removing unreachable block (ram,0x000140788887)

void __cdecl system_wall_constructor::construct_elevation_supports::construct_circle_supports(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  char *pcVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  longlong *plVar9;
  undefined4 *puVar10;
  undefined8 *puVar11;
  undefined4 *puVar12;
  undefined8 uVar13;
  longlong lVar14;
  uint uVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  byte extraout_AL;
  uint uVar22;
  uint uVar23;
  longlong lVar24;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *in_RCX;
  undefined1 (*pauVar25) [16];
  ulonglong uVar26;
  byte bVar27;
  longlong lVar28;
  longlong *in_R8;
  longlong lVar29;
  ulonglong uVar30;
  ushort uVar31;
  longlong lVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  undefined1 (*pauVar35) [16];
  char cVar36;
  byte bVar37;
  char cVar39;
  char cVar40;
  char cVar41;
  undefined1 auVar38 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar46;
  undefined1 unaff_XMM6_Bc;
  char cVar47;
  undefined1 unaff_XMM6_Bd;
  char cVar48;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar49;
  undefined1 unaff_XMM6_Bg;
  char cVar50;
  undefined1 unaff_XMM6_Bh;
  char cVar51;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar52;
  undefined1 unaff_XMM6_Bk;
  char cVar53;
  undefined1 unaff_XMM6_Bl;
  char cVar54;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar55;
  undefined1 unaff_XMM6_Bo;
  char cVar56;
  undefined1 unaff_XMM6_Bp;
  char cVar57;
  undefined8 *in_stack_00000038;
  longlong in_stack_00000040;
  longlong *in_stack_00000050;
  longlong local_130;
  int local_7c;
  
  puVar7 = (ulonglong *)*in_RCX;
  lVar8 = in_RCX[1];
  uVar26 = *puVar7;
  uVar30 = 0;
  uVar33 = uVar26 - *(ulonglong *)(lVar8 + 0x18);
  if (uVar26 < *(ulonglong *)(lVar8 + 0x18)) {
    uVar33 = uVar30;
  }
  uVar34 = uVar26 - *(ulonglong *)(lVar8 + 0x38);
  if (uVar26 < *(ulonglong *)(lVar8 + 0x38)) {
    uVar34 = uVar30;
  }
  local_130 = uVar33 * 0x28 + *(longlong *)(lVar8 + 8);
  uVar26 = *(ulonglong *)(lVar8 + 0x10) - uVar33;
  if (*(ulonglong *)(lVar8 + 0x10) < uVar33) {
    local_130 = 8;
    uVar26 = uVar30;
  }
  lVar24 = uVar34 * 0x28 + *(longlong *)(lVar8 + 0x28);
  uVar33 = *(ulonglong *)(lVar8 + 0x30) - uVar34;
  if (*(ulonglong *)(lVar8 + 0x30) < uVar34) {
    lVar24 = 8;
    uVar33 = uVar30;
  }
  *puVar7 = (*(longlong *)(lVar8 + 0x40) - uVar33) - uVar26;
  lVar8 = local_130 + uVar26 * 0x28;
  lVar1 = lVar24 + uVar33 * 0x28;
  plVar9 = (longlong *)*in_R8;
  uVar5 = *(undefined4 *)(in_stack_00000040 + 0x1c);
  puVar10 = *(undefined4 **)(in_stack_00000040 + 0x10);
  uVar6 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  puVar11 = (undefined8 *)*in_stack_00000038;
  puVar12 = (undefined4 *)in_stack_00000038[2];
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar46 = (char)((uint)iVar16 >> 8);
  cVar47 = (char)((uint)iVar16 >> 0x10);
  cVar48 = (char)((uint)iVar16 >> 0x18);
  iVar17 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar49 = (char)((uint)iVar17 >> 8);
  cVar50 = (char)((uint)iVar17 >> 0x10);
  cVar51 = (char)((uint)iVar17 >> 0x18);
  iVar18 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar52 = (char)((uint)iVar18 >> 8);
  cVar53 = (char)((uint)iVar18 >> 0x10);
  cVar54 = (char)((uint)iVar18 >> 0x18);
  iVar19 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar55 = (char)((uint)iVar19 >> 8);
  cVar56 = (char)((uint)iVar19 >> 0x10);
  cVar57 = (char)((uint)iVar19 >> 0x18);
LAB_14078832c:
  if ((local_130 == 0) || (local_130 == lVar8)) {
    if ((lVar24 == 0) || (lVar24 == lVar1)) {
      return;
    }
    local_130 = 0;
    lVar28 = lVar24;
    lVar24 = lVar24 + 0x28;
  }
  else {
    lVar28 = local_130;
    local_130 = local_130 + 0x28;
  }
  *puVar7 = *puVar7 + 1;
  lVar28 = *(longlong *)(lVar28 + 0x18);
  if (plVar9[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar27 = (byte)(extraout_RAX >> 0x39);
    auVar38 = ZEXT216(CONCAT11(bVar27,bVar27));
    auVar38 = pshuflw(auVar38,auVar38,0);
    lVar29 = 0;
    uVar26 = extraout_RAX;
    while( true ) {
      uVar26 = plVar9[1] & uVar26;
      pcVar4 = (char *)(*plVar9 + uVar26);
      cVar36 = auVar38[0];
      auVar44[0] = -(*pcVar4 == cVar36);
      cVar39 = auVar38[1];
      auVar44[1] = -(pcVar4[1] == cVar39);
      cVar40 = auVar38[2];
      auVar44[2] = -(pcVar4[2] == cVar40);
      cVar41 = auVar38[3];
      auVar44[3] = -(pcVar4[3] == cVar41);
      auVar44[4] = -(pcVar4[4] == cVar36);
      auVar44[5] = -(pcVar4[5] == cVar39);
      auVar44[6] = -(pcVar4[6] == cVar40);
      auVar44[7] = -(pcVar4[7] == cVar41);
      auVar44[8] = -(pcVar4[8] == cVar36);
      auVar44[9] = -(pcVar4[9] == cVar39);
      auVar44[10] = -(pcVar4[10] == cVar40);
      auVar44[0xb] = -(pcVar4[0xb] == cVar41);
      auVar44[0xc] = -(pcVar4[0xc] == cVar36);
      auVar44[0xd] = -(pcVar4[0xd] == cVar39);
      auVar44[0xe] = -(pcVar4[0xe] == cVar40);
      auVar44[0xf] = -(pcVar4[0xf] == cVar41);
      uVar31 = (ushort)(SUB161(auVar44 >> 7,0) & 1) | (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe | (ushort)(auVar44[0xf] >> 7) << 0xf;
      uVar22 = (uint)uVar31;
      while (uVar31 != 0) {
        uVar23 = 0;
        for (uVar15 = uVar22; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar23 = uVar23 + 1;
        }
        if (lVar28 == *(longlong *)(*plVar9 + -0x2a8 + (uVar23 + uVar26 & plVar9[1]) * -0x2a8)) {
          country_core::resources::walls::public_wall_state::PublicWallState::is_circle();
          if (local_7c != 1) goto LAB_14078832c;
          *puVar10 = uVar5;
          country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage::
          clear_and_get_mut();
          circle_categorize::circle_categorize();
          if (extraout_AL == 0) {
            circle_turret::construct_turret();
          }
          else {
            circle_bottom_layer::construct_circle_bottom_layer();
            *(undefined4 *)in_stack_00000050[2] =
                 *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
            lVar29 = *in_stack_00000050;
            lVar32 = *(longlong *)(lVar29 + 0x30);
            uVar13 = *(undefined8 *)(lVar29 + 0x40);
            if (lVar32 == *(longlong *)(lVar29 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                        ();
            }
            lVar14 = *(longlong *)(lVar29 + 0x28);
            *(undefined8 *)(lVar14 + lVar32 * 0x28) = 0;
            *(undefined8 *)(lVar14 + 8 + lVar32 * 0x28) = 4;
            *(undefined8 *)(lVar14 + 0x10 + lVar32 * 0x28) = 0;
            *(longlong *)(lVar14 + 0x18 + lVar32 * 0x28) = lVar28;
            *(undefined8 *)(lVar14 + 0x20 + lVar32 * 0x28) = uVar13;
            *(longlong *)(lVar29 + 0x30) = lVar32 + 1;
            *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
            circle_pillars::construct_circle_pillars();
          }
          *puVar12 = uVar6;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          pauVar25 = (undefined1 (*) [16])*puVar11;
          uVar26 = puVar11[1];
          uVar30 = uVar26 & extraout_RAX_01;
          bVar27 = (byte)(extraout_RAX_01 >> 0x38);
          bVar37 = bVar27 >> 1;
          auVar38 = ZEXT216(CONCAT11(bVar37,bVar37));
          auVar38 = pshuflw(auVar38,auVar38,0);
          lVar29 = 0;
          uVar33 = uVar30;
          goto LAB_1407886cb;
        }
        uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
        uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
      }
      auVar42[0] = -(*pcVar4 == (char)iVar16);
      auVar42[1] = -(pcVar4[1] == cVar46);
      auVar42[2] = -(pcVar4[2] == cVar47);
      auVar42[3] = -(pcVar4[3] == cVar48);
      auVar42[4] = -(pcVar4[4] == (char)iVar17);
      auVar42[5] = -(pcVar4[5] == cVar49);
      auVar42[6] = -(pcVar4[6] == cVar50);
      auVar42[7] = -(pcVar4[7] == cVar51);
      auVar42[8] = -(pcVar4[8] == (char)iVar18);
      auVar42[9] = -(pcVar4[9] == cVar52);
      auVar42[10] = -(pcVar4[10] == cVar53);
      auVar42[0xb] = -(pcVar4[0xb] == cVar54);
      auVar42[0xc] = -(pcVar4[0xc] == (char)iVar19);
      auVar42[0xd] = -(pcVar4[0xd] == cVar55);
      auVar42[0xe] = -(pcVar4[0xe] == cVar56);
      auVar42[0xf] = -(pcVar4[0xf] == cVar57);
      if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar42 >> 0x17,0) & 1) != 0) || (SUB161(auVar42 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar42 >> 0x27,0) & 1) != 0) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar42 >> 0x3f,0) & 1) != 0) || (SUB161(auVar42 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar42 >> 0x4f,0) & 1) != 0) || (SUB161(auVar42 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar42 >> 0x5f,0) & 1) != 0) || (SUB161(auVar42 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar42 >> 0x6f,0) & 1) != 0) || (SUB161(auVar42 >> 0x77,0) & 1) != 0) ||
          auVar42[0xf] < '\0') break;
      uVar26 = uVar26 + lVar29 + 0x10;
      lVar29 = lVar29 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_1407886cb:
  pcVar4 = *pauVar25 + uVar33;
  cVar36 = auVar38[0];
  auVar45[0] = -(*pcVar4 == cVar36);
  cVar39 = auVar38[1];
  auVar45[1] = -(pcVar4[1] == cVar39);
  cVar40 = auVar38[2];
  auVar45[2] = -(pcVar4[2] == cVar40);
  cVar41 = auVar38[3];
  auVar45[3] = -(pcVar4[3] == cVar41);
  auVar45[4] = -(pcVar4[4] == cVar36);
  auVar45[5] = -(pcVar4[5] == cVar39);
  auVar45[6] = -(pcVar4[6] == cVar40);
  auVar45[7] = -(pcVar4[7] == cVar41);
  auVar45[8] = -(pcVar4[8] == cVar36);
  auVar45[9] = -(pcVar4[9] == cVar39);
  auVar45[10] = -(pcVar4[10] == cVar40);
  auVar45[0xb] = -(pcVar4[0xb] == cVar41);
  auVar45[0xc] = -(pcVar4[0xc] == cVar36);
  auVar45[0xd] = -(pcVar4[0xd] == cVar39);
  auVar45[0xe] = -(pcVar4[0xe] == cVar40);
  auVar45[0xf] = -(pcVar4[0xf] == cVar41);
  uVar31 = (ushort)(SUB161(auVar45 >> 7,0) & 1) | (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe | (ushort)(auVar45[0xf] >> 7) << 0xf;
  uVar22 = (uint)uVar31;
  while (uVar31 != 0) {
    uVar23 = 0;
    for (uVar15 = uVar22; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar23 = uVar23 + 1;
    }
    uVar34 = uVar23 + uVar33 & uVar26;
    if (*(longlong *)pauVar25[uVar34 * -4 + -4] == lVar28) {
      pauVar35 = pauVar25 + uVar34 * -4;
      goto LAB_1407888c2;
    }
    uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
    uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
  }
  auVar43[0] = -(*pcVar4 == (char)iVar16);
  auVar43[1] = -(pcVar4[1] == cVar46);
  auVar43[2] = -(pcVar4[2] == cVar47);
  auVar43[3] = -(pcVar4[3] == cVar48);
  auVar43[4] = -(pcVar4[4] == (char)iVar17);
  auVar43[5] = -(pcVar4[5] == cVar49);
  auVar43[6] = -(pcVar4[6] == cVar50);
  auVar43[7] = -(pcVar4[7] == cVar51);
  auVar43[8] = -(pcVar4[8] == (char)iVar18);
  auVar43[9] = -(pcVar4[9] == cVar52);
  auVar43[10] = -(pcVar4[10] == cVar53);
  auVar43[0xb] = -(pcVar4[0xb] == cVar54);
  auVar43[0xc] = -(pcVar4[0xc] == (char)iVar19);
  auVar43[0xd] = -(pcVar4[0xd] == cVar55);
  auVar43[0xe] = -(pcVar4[0xe] == cVar56);
  auVar43[0xf] = -(pcVar4[0xf] == cVar57);
  if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 || (SUB161(auVar43 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar43 >> 0x17,0) & 1) != 0) || (SUB161(auVar43 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar43 >> 0x27,0) & 1) != 0) || (SUB161(auVar43 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar43 >> 0x37,0) & 1) != 0) || (SUB161(auVar43 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar43 >> 0x47,0) & 1) != 0) || (SUB161(auVar43 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar43 >> 0x57,0) & 1) != 0) || (SUB161(auVar43 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar43 >> 0x67,0) & 1) != 0) || (SUB161(auVar43 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar43 >> 0x77,0) & 1) != 0) || auVar43[0xf] < '\0') goto LAB_140788770;
  lVar32 = uVar33 + lVar29;
  lVar29 = lVar29 + 0x10;
  uVar33 = lVar32 + 0x10U & uVar26;
  goto LAB_1407886cb;
LAB_140788770:
  if (puVar11[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports,std::hash::random::RandomState>_>
              ();
    pauVar25 = (undefined1 (*) [16])*puVar11;
    uVar26 = puVar11[1];
    uVar30 = extraout_RAX_01 & uVar26;
  }
  auVar38 = *(undefined1 (*) [16])(*pauVar25 + uVar30);
  uVar31 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar38[0xf] >> 7) << 0xf
  ;
  uVar22 = (uint)uVar31;
  if (uVar31 == 0) {
    lVar29 = 0x10;
    do {
      uVar30 = uVar30 + lVar29 & uVar26;
      auVar38 = *(undefined1 (*) [16])(*pauVar25 + uVar30);
      lVar29 = lVar29 + 0x10;
      uVar22 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                              (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                              (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                              (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                              (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                              (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                              (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                              (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                              (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                              (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                              (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                              (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                              (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                              (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                             (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
    } while (uVar22 == 0);
  }
  uVar23 = 0;
  for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
    uVar23 = uVar23 + 1;
  }
  uVar33 = uVar23 + uVar30 & uVar26;
  bVar37 = (*pauVar25)[uVar33];
  if (-1 < (char)bVar37) {
    auVar38 = *pauVar25;
    uVar22 = 0;
    for (uVar23 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar38[0xf] >> 7) << 0xf); (uVar23 & 1) == 0;
        uVar23 = uVar23 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    uVar33 = (ulonglong)uVar22;
    bVar37 = (*pauVar25)[uVar33];
  }
  bVar27 = bVar27 >> 1;
  (*pauVar25)[uVar33] = bVar27;
  pauVar25[1][uVar33 - 0x10 & uVar26] = bVar27;
  puVar11[2] = puVar11[2] - (ulonglong)(bVar37 & 1);
  pauVar35 = pauVar25 + uVar33 * -4;
  *(longlong *)pauVar25[uVar33 * -4 + -4] = lVar28;
  *(undefined8 *)(pauVar35[-4] + 8) = 0;
  *(undefined8 *)pauVar35[-3] = 4;
  *(undefined8 *)(pauVar35[-3] + 8) = 0;
  *(undefined8 *)pauVar35[-2] = 0;
  *(undefined8 *)(pauVar35[-2] + 8) = 0;
  *(undefined8 *)pauVar35[-1] = 0;
  pauVar35[-1][8] = 0;
  puVar11[3] = puVar11[3] + 1;
LAB_1407888c2:
  *(undefined8 *)pauVar35[-1] = 0;
  pauVar35[-1][8] = extraout_AL ^ 1;
  uVar26 = *(ulonglong *)(extraout_RAX_00 + 0x10);
  if (uVar26 == 0) {
    lVar29 = 4;
    lVar28 = *(longlong *)(pauVar35[-4] + 8);
  }
  else {
    lVar28 = *(longlong *)(extraout_RAX_00 + 8);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (uVar26 == 1) {
      uVar33 = 0;
    }
    else {
      lVar29 = 0x20;
      uVar33 = 0;
      do {
        puVar2 = (undefined8 *)(lVar28 + -0x20 + lVar29);
        uVar13 = *puVar2;
        uVar20 = puVar2[1];
        puVar2 = (undefined8 *)(lVar28 + -0x10 + lVar29);
        uVar21 = puVar2[1];
        puVar3 = (undefined8 *)(extraout_RAX_02 + -0x10 + lVar29);
        *puVar3 = *puVar2;
        puVar3[1] = uVar21;
        puVar2 = (undefined8 *)(extraout_RAX_02 + -0x20 + lVar29);
        *puVar2 = uVar13;
        puVar2[1] = uVar20;
        uVar13 = ((undefined8 *)(lVar28 + lVar29))[1];
        puVar2 = (undefined8 *)(lVar28 + 0x10 + lVar29);
        uVar20 = *puVar2;
        uVar21 = puVar2[1];
        *(undefined8 *)(extraout_RAX_02 + lVar29) = *(undefined8 *)(lVar28 + lVar29);
        ((undefined8 *)(extraout_RAX_02 + lVar29))[1] = uVar13;
        puVar2 = (undefined8 *)(extraout_RAX_02 + 0x10 + lVar29);
        *puVar2 = uVar20;
        puVar2[1] = uVar21;
        uVar33 = uVar33 + 2;
        lVar29 = lVar29 + 0x40;
      } while ((uVar26 & 0xfffffffffffffffe) != uVar33);
    }
    if ((uVar26 & 1) != 0) {
      lVar29 = uVar33 * 0x20;
      uVar13 = *(undefined8 *)(lVar28 + lVar29);
      uVar20 = ((undefined8 *)(lVar28 + lVar29))[1];
      puVar2 = (undefined8 *)(lVar28 + 0x10 + lVar29);
      uVar21 = puVar2[1];
      puVar3 = (undefined8 *)(extraout_RAX_02 + 0x10 + lVar29);
      *puVar3 = *puVar2;
      puVar3[1] = uVar21;
      *(undefined8 *)(extraout_RAX_02 + lVar29) = uVar13;
      ((undefined8 *)(extraout_RAX_02 + lVar29))[1] = uVar20;
    }
    lVar28 = *(longlong *)(pauVar35[-4] + 8);
    lVar29 = extraout_RAX_02;
  }
  if (lVar28 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(ulonglong *)(pauVar35[-4] + 8) = uVar26;
  *(longlong *)pauVar35[-3] = lVar29;
  *(ulonglong *)(pauVar35[-3] + 8) = uVar26;
  goto LAB_14078832c;
}

