
void __cdecl
country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl
          (void)

{
  char *pcVar1;
  float fVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  longlong *plVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  longlong lVar11;
  undefined4 *puVar12;
  undefined8 uVar13;
  uint uVar14;
  uint uVar15;
  int iVar16;
  byte *pbVar17;
  uint extraout_EAX;
  ulonglong uVar18;
  byte *pbVar19;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *plVar20;
  undefined8 *in_RCX;
  byte *pbVar21;
  longlong *plVar22;
  longlong *in_RDX;
  byte *pbVar23;
  ulonglong uVar24;
  byte *pbVar25;
  longlong lVar26;
  longlong *in_R8;
  ulonglong uVar27;
  uint uVar28;
  longlong *in_R9;
  longlong lVar29;
  longlong lVar30;
  ushort uVar31;
  longlong lVar32;
  byte *pbVar33;
  ulonglong uVar34;
  bool bVar35;
  byte bVar36;
  char cVar37;
  char cVar42;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar38;
  char cVar40;
  char cVar41;
  undefined1 auVar39 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  float fVar51;
  undefined1 unaff_XMM10_Ba;
  char cVar52;
  undefined1 unaff_XMM10_Bb;
  char cVar53;
  undefined1 unaff_XMM10_Bc;
  char cVar54;
  undefined1 unaff_XMM10_Bd;
  char cVar55;
  undefined1 unaff_XMM10_Be;
  char cVar56;
  undefined1 unaff_XMM10_Bf;
  char cVar57;
  undefined1 unaff_XMM10_Bg;
  char cVar58;
  undefined1 unaff_XMM10_Bh;
  char cVar59;
  undefined1 unaff_XMM10_Bi;
  char cVar60;
  undefined1 unaff_XMM10_Bj;
  char cVar61;
  undefined1 unaff_XMM10_Bk;
  char cVar62;
  undefined1 unaff_XMM10_Bl;
  char cVar63;
  undefined1 unaff_XMM10_Bm;
  char cVar64;
  undefined1 unaff_XMM10_Bn;
  char cVar65;
  undefined1 unaff_XMM10_Bo;
  char cVar66;
  undefined1 unaff_XMM10_Bp;
  char cVar67;
  undefined8 local_168;
  undefined4 local_160;
  
  puVar6 = (ulonglong *)*in_RCX;
  lVar7 = in_RCX[1];
  uVar27 = *puVar6;
  uVar24 = 0;
  uVar18 = uVar27 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar18 = uVar24;
  }
  uVar34 = uVar27 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar34 = uVar24;
  }
  pbVar21 = (byte *)(uVar18 * 0x30 + *(longlong *)(lVar7 + 8));
  uVar27 = *(ulonglong *)(lVar7 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar18) {
    pbVar21 = &DAT_00000008;
    uVar27 = uVar24;
  }
  pbVar19 = (byte *)(uVar34 * 0x30 + *(longlong *)(lVar7 + 0x28));
  uVar18 = *(ulonglong *)(lVar7 + 0x30) - uVar34;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar34) {
    pbVar19 = &DAT_00000008;
    uVar18 = uVar24;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar18) - uVar27;
  pbVar33 = pbVar21 + uVar27 * 0x30;
  pbVar23 = pbVar19 + uVar18 * 0x30;
  plVar8 = (longlong *)*in_RDX;
  puVar9 = (undefined4 *)in_RDX[2];
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  uVar4 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar7 = *in_R8;
  puVar10 = (undefined4 *)in_R8[2];
  uVar5 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar11 = *in_R9;
  puVar12 = (undefined4 *)in_R9[2];
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bd,
                            CONCAT12(unaff_XMM10_Bc,CONCAT11(unaff_XMM10_Bb,unaff_XMM10_Ba))) ==
                  CONCAT13(unaff_XMM10_Bd,
                           CONCAT12(unaff_XMM10_Bc,CONCAT11(unaff_XMM10_Bb,unaff_XMM10_Ba))));
  cVar52 = (char)iVar16;
  cVar53 = (char)((uint)iVar16 >> 8);
  cVar54 = (char)((uint)iVar16 >> 0x10);
  cVar55 = (char)((uint)iVar16 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bh,
                            CONCAT12(unaff_XMM10_Bg,CONCAT11(unaff_XMM10_Bf,unaff_XMM10_Be))) ==
                  CONCAT13(unaff_XMM10_Bh,
                           CONCAT12(unaff_XMM10_Bg,CONCAT11(unaff_XMM10_Bf,unaff_XMM10_Be))));
  cVar56 = (char)iVar16;
  cVar57 = (char)((uint)iVar16 >> 8);
  cVar58 = (char)((uint)iVar16 >> 0x10);
  cVar59 = (char)((uint)iVar16 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bl,
                            CONCAT12(unaff_XMM10_Bk,CONCAT11(unaff_XMM10_Bj,unaff_XMM10_Bi))) ==
                  CONCAT13(unaff_XMM10_Bl,
                           CONCAT12(unaff_XMM10_Bk,CONCAT11(unaff_XMM10_Bj,unaff_XMM10_Bi))));
  cVar60 = (char)iVar16;
  cVar61 = (char)((uint)iVar16 >> 8);
  cVar62 = (char)((uint)iVar16 >> 0x10);
  cVar63 = (char)((uint)iVar16 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bp,
                            CONCAT12(unaff_XMM10_Bo,CONCAT11(unaff_XMM10_Bn,unaff_XMM10_Bm))) ==
                  CONCAT13(unaff_XMM10_Bp,
                           CONCAT12(unaff_XMM10_Bo,CONCAT11(unaff_XMM10_Bn,unaff_XMM10_Bm))));
  cVar64 = (char)iVar16;
  cVar65 = (char)((uint)iVar16 >> 8);
  cVar66 = (char)((uint)iVar16 >> 0x10);
  cVar67 = (char)((uint)iVar16 >> 0x18);
LAB_140ac210e:
  do {
    if ((pbVar21 == (byte *)0x0) || (pbVar21 == pbVar33)) {
      if ((pbVar19 == (byte *)0x0) || (pbVar19 == pbVar23)) {
        return;
      }
      pbVar21 = (byte *)0x0;
      pbVar17 = pbVar19 + 0x30;
      pbVar25 = pbVar19;
    }
    else {
      pbVar17 = pbVar19;
      pbVar25 = pbVar21;
      pbVar21 = pbVar21 + 0x30;
    }
    pbVar19 = pbVar17;
    *puVar6 = *puVar6 + 1;
  } while (*(int *)(pbVar25 + 0x10) == 0);
  if (*(int *)(pbVar25 + 0x10) != 1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar38 = *(float *)(pbVar25 + 0x14);
  lVar26 = *(longlong *)(pbVar25 + 0x18);
  if (plVar8[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar30 = *plVar8;
    bVar36 = (byte)(extraout_RAX >> 0x39);
    auVar39 = ZEXT216(CONCAT11(bVar36,bVar36));
    auVar39 = pshuflw(auVar39,auVar39,0);
    lVar29 = 0;
    uVar27 = extraout_RAX;
    while( true ) {
      uVar27 = plVar8[1] & uVar27;
      pcVar1 = (char *)(lVar30 + uVar27);
      cVar37 = auVar39[0];
      auVar47[0] = -(*pcVar1 == cVar37);
      cVar40 = auVar39[1];
      auVar47[1] = -(pcVar1[1] == cVar40);
      cVar41 = auVar39[2];
      auVar47[2] = -(pcVar1[2] == cVar41);
      cVar42 = auVar39[3];
      auVar47[3] = -(pcVar1[3] == cVar42);
      auVar47[4] = -(pcVar1[4] == cVar37);
      auVar47[5] = -(pcVar1[5] == cVar40);
      auVar47[6] = -(pcVar1[6] == cVar41);
      auVar47[7] = -(pcVar1[7] == cVar42);
      auVar47[8] = -(pcVar1[8] == cVar37);
      auVar47[9] = -(pcVar1[9] == cVar40);
      auVar47[10] = -(pcVar1[10] == cVar41);
      auVar47[0xb] = -(pcVar1[0xb] == cVar42);
      auVar47[0xc] = -(pcVar1[0xc] == cVar37);
      auVar47[0xd] = -(pcVar1[0xd] == cVar40);
      auVar47[0xe] = -(pcVar1[0xe] == cVar41);
      auVar47[0xf] = -(pcVar1[0xf] == cVar42);
      uVar31 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf;
      uVar28 = (uint)uVar31;
      while (uVar31 != 0) {
        uVar14 = 0;
        for (uVar15 = uVar28; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar14 = uVar14 + 1;
        }
        lVar32 = (uVar14 + uVar27 & plVar8[1]) * -0x78;
        if (*(longlong *)(lVar30 + -0x78 + lVar32) == lVar26) {
          fVar2 = *(float *)(lVar30 + -0x1c + lVar32);
          ui_adjust_wall_height_mode::clamp_wall_height();
          fVar51 = extraout_XMM0_Da;
          if ((extraout_RAX_00 & 1) == 0) {
            fVar51 = fVar38;
          }
          if (fVar51 == fVar2) goto LAB_140ac210e;
          if ((*pbVar25 & 1) != 0) {
            local_160 = *(undefined4 *)(pbVar25 + 0xc);
            local_168 = *(undefined8 *)(pbVar25 + 4);
            goto LAB_140ac23b6;
          }
          lVar26 = *(longlong *)(pbVar25 + 0x18);
          if (plVar8[3] == 0) goto LAB_140ac2741;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar30 = *plVar8;
          bVar36 = (byte)(extraout_RAX_01 >> 0x39);
          auVar39 = ZEXT216(CONCAT11(bVar36,bVar36));
          auVar39 = pshuflw(auVar39,auVar39,0);
          lVar29 = 0;
          uVar27 = extraout_RAX_01;
          goto LAB_140ac22df;
        }
        uVar31 = (ushort)(uVar28 - 1) & (ushort)uVar28;
        uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar31);
      }
      auVar43[0] = -(*pcVar1 == cVar52);
      auVar43[1] = -(pcVar1[1] == cVar53);
      auVar43[2] = -(pcVar1[2] == cVar54);
      auVar43[3] = -(pcVar1[3] == cVar55);
      auVar43[4] = -(pcVar1[4] == cVar56);
      auVar43[5] = -(pcVar1[5] == cVar57);
      auVar43[6] = -(pcVar1[6] == cVar58);
      auVar43[7] = -(pcVar1[7] == cVar59);
      auVar43[8] = -(pcVar1[8] == cVar60);
      auVar43[9] = -(pcVar1[9] == cVar61);
      auVar43[10] = -(pcVar1[10] == cVar62);
      auVar43[0xb] = -(pcVar1[0xb] == cVar63);
      auVar43[0xc] = -(pcVar1[0xc] == cVar64);
      auVar43[0xd] = -(pcVar1[0xd] == cVar65);
      auVar43[0xe] = -(pcVar1[0xe] == cVar66);
      auVar43[0xf] = -(pcVar1[0xf] == cVar67);
      if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 || (SUB161(auVar43 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar43 >> 0x17,0) & 1) != 0) || (SUB161(auVar43 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar43 >> 0x27,0) & 1) != 0) || (SUB161(auVar43 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar43 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar43 >> 0x3f,0) & 1) != 0) || (SUB161(auVar43 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar43 >> 0x4f,0) & 1) != 0) || (SUB161(auVar43 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar43 >> 0x5f,0) & 1) != 0) || (SUB161(auVar43 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar43 >> 0x6f,0) & 1) != 0) || (SUB161(auVar43 >> 0x77,0) & 1) != 0) ||
          auVar43[0xf] < '\0') break;
      uVar27 = uVar27 + lVar29 + 0x10;
      lVar29 = lVar29 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_140ac22df:
  uVar27 = plVar8[1] & uVar27;
  pcVar1 = (char *)(lVar30 + uVar27);
  cVar37 = auVar39[0];
  auVar48[0] = -(*pcVar1 == cVar37);
  cVar40 = auVar39[1];
  auVar48[1] = -(pcVar1[1] == cVar40);
  cVar41 = auVar39[2];
  auVar48[2] = -(pcVar1[2] == cVar41);
  cVar42 = auVar39[3];
  auVar48[3] = -(pcVar1[3] == cVar42);
  auVar48[4] = -(pcVar1[4] == cVar37);
  auVar48[5] = -(pcVar1[5] == cVar40);
  auVar48[6] = -(pcVar1[6] == cVar41);
  auVar48[7] = -(pcVar1[7] == cVar42);
  auVar48[8] = -(pcVar1[8] == cVar37);
  auVar48[9] = -(pcVar1[9] == cVar40);
  auVar48[10] = -(pcVar1[10] == cVar41);
  auVar48[0xb] = -(pcVar1[0xb] == cVar42);
  auVar48[0xc] = -(pcVar1[0xc] == cVar37);
  auVar48[0xd] = -(pcVar1[0xd] == cVar40);
  auVar48[0xe] = -(pcVar1[0xe] == cVar41);
  auVar48[0xf] = -(pcVar1[0xf] == cVar42);
  uVar31 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar48 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar48 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar48 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar48 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar48 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar48 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar48 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar48 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar48 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar48 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar48 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe | (ushort)(auVar48[0xf] >> 7) << 0xf;
  uVar28 = (uint)uVar31;
  if (uVar31 != 0) {
    while( true ) {
      uVar14 = 0;
      for (uVar15 = uVar28; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
        uVar14 = uVar14 + 1;
      }
      lVar32 = (uVar14 + uVar27 & plVar8[1]) * -0x78;
      if (*(longlong *)(lVar30 + -0x78 + lVar32) == lVar26) break;
      uVar31 = (ushort)(uVar28 - 1) & (ushort)uVar28;
      uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar31);
      if (uVar31 == 0) goto LAB_140ac2321;
    }
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
              ();
    uVar27 = *(ulonglong *)(lVar30 + -0x60 + lVar32);
    if (uVar27 <= extraout_EAX) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (uVar27 <= (ulonglong)extraout_EAX + 1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    ::utils::swizzlers::impl$0::x0y();
LAB_140ac23b6:
    resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    local_168 = CONCAT44(extraout_XMM0_Da_00 + local_168._4_4_,(undefined4)local_168);
    lVar26 = *(longlong *)(pbVar25 + 0x18);
    if (plVar8[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar36 = (byte)(extraout_RAX_02 >> 0x39);
      auVar39 = ZEXT216(CONCAT11(bVar36,bVar36));
      auVar39 = pshuflw(auVar39,auVar39,0);
      lVar30 = 0;
      uVar27 = extraout_RAX_02;
      while( true ) {
        uVar27 = plVar8[1] & uVar27;
        pcVar1 = (char *)(*plVar8 + uVar27);
        cVar37 = auVar39[0];
        auVar49[0] = -(*pcVar1 == cVar37);
        cVar40 = auVar39[1];
        auVar49[1] = -(pcVar1[1] == cVar40);
        cVar41 = auVar39[2];
        auVar49[2] = -(pcVar1[2] == cVar41);
        cVar42 = auVar39[3];
        auVar49[3] = -(pcVar1[3] == cVar42);
        auVar49[4] = -(pcVar1[4] == cVar37);
        auVar49[5] = -(pcVar1[5] == cVar40);
        auVar49[6] = -(pcVar1[6] == cVar41);
        auVar49[7] = -(pcVar1[7] == cVar42);
        auVar49[8] = -(pcVar1[8] == cVar37);
        auVar49[9] = -(pcVar1[9] == cVar40);
        auVar49[10] = -(pcVar1[10] == cVar41);
        auVar49[0xb] = -(pcVar1[0xb] == cVar42);
        auVar49[0xc] = -(pcVar1[0xc] == cVar37);
        auVar49[0xd] = -(pcVar1[0xd] == cVar40);
        auVar49[0xe] = -(pcVar1[0xe] == cVar41);
        auVar49[0xf] = -(pcVar1[0xf] == cVar42);
        uVar31 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe | (ushort)(auVar49[0xf] >> 7) << 0xf
        ;
        uVar28 = (uint)uVar31;
        while (uVar31 != 0) {
          uVar14 = 0;
          for (uVar15 = uVar28; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          if (*(longlong *)(*plVar8 + -0x78 + (uVar14 + uVar27 & plVar8[1]) * -0x78) == lVar26) {
            *puVar9 = uVar3;
            resources::walls::inner_walls::InnerWalls::
            _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_height::closure_env_0>
                      ();
            lVar26 = *(longlong *)(pbVar25 + 0x18);
            if (plVar8[3] == 0) goto LAB_140ac2735;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar30 = *plVar8;
            bVar36 = (byte)(extraout_RAX_03 >> 0x39);
            auVar39 = ZEXT216(CONCAT11(bVar36,bVar36));
            auVar39 = pshuflw(auVar39,auVar39,0);
            lVar29 = 0;
            uVar27 = extraout_RAX_03;
            goto LAB_140ac2503;
          }
          uVar31 = (ushort)(uVar28 - 1) & (ushort)uVar28;
          uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar31);
        }
        auVar45[0] = -(*pcVar1 == cVar52);
        auVar45[1] = -(pcVar1[1] == cVar53);
        auVar45[2] = -(pcVar1[2] == cVar54);
        auVar45[3] = -(pcVar1[3] == cVar55);
        auVar45[4] = -(pcVar1[4] == cVar56);
        auVar45[5] = -(pcVar1[5] == cVar57);
        auVar45[6] = -(pcVar1[6] == cVar58);
        auVar45[7] = -(pcVar1[7] == cVar59);
        auVar45[8] = -(pcVar1[8] == cVar60);
        auVar45[9] = -(pcVar1[9] == cVar61);
        auVar45[10] = -(pcVar1[10] == cVar62);
        auVar45[0xb] = -(pcVar1[0xb] == cVar63);
        auVar45[0xc] = -(pcVar1[0xc] == cVar64);
        auVar45[0xd] = -(pcVar1[0xd] == cVar65);
        auVar45[0xe] = -(pcVar1[0xe] == cVar66);
        auVar45[0xf] = -(pcVar1[0xf] == cVar67);
        if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 || (SUB161(auVar45 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar45 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar45 >> 0x1f,0) & 1) != 0) || (SUB161(auVar45 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar45 >> 0x37,0) & 1) != 0) || (SUB161(auVar45 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar45 >> 0x47,0) & 1) != 0) || (SUB161(auVar45 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar45 >> 0x57,0) & 1) != 0) || (SUB161(auVar45 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar45 >> 0x67,0) & 1) != 0) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') break;
        uVar27 = uVar27 + lVar30 + 0x10;
        lVar30 = lVar30 + 0x10;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_140ac2321:
  auVar44[0] = -(*pcVar1 == cVar52);
  auVar44[1] = -(pcVar1[1] == cVar53);
  auVar44[2] = -(pcVar1[2] == cVar54);
  auVar44[3] = -(pcVar1[3] == cVar55);
  auVar44[4] = -(pcVar1[4] == cVar56);
  auVar44[5] = -(pcVar1[5] == cVar57);
  auVar44[6] = -(pcVar1[6] == cVar58);
  auVar44[7] = -(pcVar1[7] == cVar59);
  auVar44[8] = -(pcVar1[8] == cVar60);
  auVar44[9] = -(pcVar1[9] == cVar61);
  auVar44[10] = -(pcVar1[10] == cVar62);
  auVar44[0xb] = -(pcVar1[0xb] == cVar63);
  auVar44[0xc] = -(pcVar1[0xc] == cVar64);
  auVar44[0xd] = -(pcVar1[0xd] == cVar65);
  auVar44[0xe] = -(pcVar1[0xe] == cVar66);
  auVar44[0xf] = -(pcVar1[0xf] == cVar67);
  if ((((((((((((((((SUB161(auVar44 >> 7,0) & 1) != 0 || (SUB161(auVar44 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar44 >> 0x17,0) & 1) != 0) || (SUB161(auVar44 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar44 >> 0x27,0) & 1) != 0) || (SUB161(auVar44 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar44 >> 0x37,0) & 1) != 0) || (SUB161(auVar44 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar44 >> 0x47,0) & 1) != 0) || (SUB161(auVar44 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar44 >> 0x57,0) & 1) != 0) || (SUB161(auVar44 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar44 >> 0x67,0) & 1) != 0) || (SUB161(auVar44 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar44 >> 0x77,0) & 1) != 0) || auVar44[0xf] < '\0') {
LAB_140ac2741:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar27 = uVar27 + lVar29 + 0x10;
  lVar29 = lVar29 + 0x10;
  goto LAB_140ac22df;
LAB_140ac2503:
  uVar27 = plVar8[1] & uVar27;
  pcVar1 = (char *)(lVar30 + uVar27);
  cVar37 = auVar39[0];
  auVar50[0] = -(*pcVar1 == cVar37);
  cVar40 = auVar39[1];
  auVar50[1] = -(pcVar1[1] == cVar40);
  cVar41 = auVar39[2];
  auVar50[2] = -(pcVar1[2] == cVar41);
  cVar42 = auVar39[3];
  auVar50[3] = -(pcVar1[3] == cVar42);
  auVar50[4] = -(pcVar1[4] == cVar37);
  auVar50[5] = -(pcVar1[5] == cVar40);
  auVar50[6] = -(pcVar1[6] == cVar41);
  auVar50[7] = -(pcVar1[7] == cVar42);
  auVar50[8] = -(pcVar1[8] == cVar37);
  auVar50[9] = -(pcVar1[9] == cVar40);
  auVar50[10] = -(pcVar1[10] == cVar41);
  auVar50[0xb] = -(pcVar1[0xb] == cVar42);
  auVar50[0xc] = -(pcVar1[0xc] == cVar37);
  auVar50[0xd] = -(pcVar1[0xd] == cVar40);
  auVar50[0xe] = -(pcVar1[0xe] == cVar41);
  auVar50[0xf] = -(pcVar1[0xf] == cVar42);
  uVar31 = (ushort)(SUB161(auVar50 >> 7,0) & 1) | (ushort)(SUB161(auVar50 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar50 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar50 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar50 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar50 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar50 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar50 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar50 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar50 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar50 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar50 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar50 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar50 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar50 >> 0x77,0) & 1) << 0xe | (ushort)(auVar50[0xf] >> 7) << 0xf;
  uVar28 = (uint)uVar31;
  while (uVar31 != 0) {
    uVar14 = 0;
    for (uVar15 = uVar28; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar14 = uVar14 + 1;
    }
    lVar32 = (uVar14 + uVar27 & plVar8[1]) * -0x78;
    if (*(longlong *)(lVar30 + -0x78 + lVar32) == lVar26) {
      bVar36 = pbVar25[0x20];
      if ((*(uint *)(lVar30 + -0x38 + lVar32) < 2) && (bVar35 = bVar36 == 0, bVar36 = 1, bVar35)) {
        *puVar9 = uVar3;
        resources::walls::inner_walls::InnerWalls::
        _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_height::closure_env_0>
                  ();
        uVar13 = *(undefined8 *)(pbVar25 + 0x18);
        ::utils::inverse_lerp();
        *puVar12 = uVar5;
        lVar30 = *(longlong *)(lVar11 + 0x40);
        lVar26 = *(longlong *)(lVar11 + 0x30);
        if (lVar26 == *(longlong *)(lVar11 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        fVar38 = 0.0;
        if (0.0 <= extraout_XMM0_Da_01) {
          fVar38 = extraout_XMM0_Da_01;
        }
        fVar2 = 1.0;
        if (fVar38 <= 1.0) {
          fVar2 = fVar38;
        }
        lVar29 = *(longlong *)(lVar11 + 0x28);
        *(longlong *)(lVar29 + lVar26 * 0x28) = lVar30;
        *(undefined8 *)(lVar29 + 8 + lVar26 * 0x28) = uVar13;
        *(undefined8 *)(lVar29 + 0x10 + lVar26 * 0x28) = local_168;
        *(undefined4 *)(lVar29 + 0x18 + lVar26 * 0x28) = local_160;
        *(float *)(lVar29 + 0x1c + lVar26 * 0x28) = fVar2;
        *(undefined1 *)(lVar29 + 0x20 + lVar26 * 0x28) = 3;
        plVar20 = (longlong *)(lVar11 + 0x30);
        plVar22 = (longlong *)(lVar11 + 0x40);
      }
      else {
        uVar13 = *(undefined8 *)(pbVar25 + 0x18);
        *puVar10 = uVar4;
        lVar30 = *(longlong *)(lVar7 + 0x40);
        lVar26 = *(longlong *)(lVar7 + 0x30);
        if (lVar26 == *(longlong *)(lVar7 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar29 = *(longlong *)(lVar7 + 0x28);
        lVar32 = lVar26 * 0x20;
        *(longlong *)(lVar29 + lVar32) = lVar30;
        *(undefined8 *)(lVar29 + 8 + lVar32) = uVar13;
        *(undefined8 *)(lVar29 + 0x10 + lVar32) = local_168;
        *(undefined4 *)(lVar29 + 0x18 + lVar32) = local_160;
        *(undefined1 *)(lVar29 + 0x1c + lVar32) = 0;
        *(byte *)(lVar29 + 0x1d + lVar32) = bVar36;
        plVar20 = (longlong *)(lVar7 + 0x30);
        plVar22 = (longlong *)(lVar7 + 0x40);
      }
      *plVar20 = lVar26 + 1;
      *plVar22 = *plVar22 + 1;
      goto LAB_140ac210e;
    }
    uVar31 = (ushort)(uVar28 - 1) & (ushort)uVar28;
    uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar31);
  }
  auVar46[0] = -(*pcVar1 == cVar52);
  auVar46[1] = -(pcVar1[1] == cVar53);
  auVar46[2] = -(pcVar1[2] == cVar54);
  auVar46[3] = -(pcVar1[3] == cVar55);
  auVar46[4] = -(pcVar1[4] == cVar56);
  auVar46[5] = -(pcVar1[5] == cVar57);
  auVar46[6] = -(pcVar1[6] == cVar58);
  auVar46[7] = -(pcVar1[7] == cVar59);
  auVar46[8] = -(pcVar1[8] == cVar60);
  auVar46[9] = -(pcVar1[9] == cVar61);
  auVar46[10] = -(pcVar1[10] == cVar62);
  auVar46[0xb] = -(pcVar1[0xb] == cVar63);
  auVar46[0xc] = -(pcVar1[0xc] == cVar64);
  auVar46[0xd] = -(pcVar1[0xd] == cVar65);
  auVar46[0xe] = -(pcVar1[0xe] == cVar66);
  auVar46[0xf] = -(pcVar1[0xf] == cVar67);
  if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar46 >> 0x17,0) & 1) != 0) || (SUB161(auVar46 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar46 >> 0x27,0) & 1) != 0) || (SUB161(auVar46 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x37,0) & 1) != 0) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') {
LAB_140ac2735:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar27 = uVar27 + lVar29 + 0x10;
  lVar29 = lVar29 + 0x10;
  goto LAB_140ac2503;
}

