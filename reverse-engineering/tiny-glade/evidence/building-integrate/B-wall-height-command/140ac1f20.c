
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
  float fVar17;
  byte *pbVar18;
  uint extraout_EAX;
  ulonglong uVar19;
  byte *pbVar20;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *plVar21;
  undefined8 *in_RCX;
  byte *pbVar22;
  longlong *plVar23;
  longlong *in_RDX;
  byte *pbVar24;
  ulonglong uVar25;
  byte *pbVar26;
  longlong lVar27;
  longlong *in_R8;
  ulonglong uVar28;
  uint uVar29;
  longlong *in_R9;
  longlong lVar30;
  longlong lVar31;
  ushort uVar32;
  longlong lVar33;
  byte *pbVar34;
  ulonglong uVar35;
  bool bVar36;
  byte bVar37;
  char cVar38;
  char cVar43;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar39;
  char cVar41;
  char cVar42;
  undefined1 auVar40 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  float fVar52;
  undefined1 unaff_XMM10_Ba;
  char cVar53;
  undefined1 unaff_XMM10_Bb;
  char cVar54;
  undefined1 unaff_XMM10_Bc;
  char cVar55;
  undefined1 unaff_XMM10_Bd;
  char cVar56;
  undefined1 unaff_XMM10_Be;
  char cVar57;
  undefined1 unaff_XMM10_Bf;
  char cVar58;
  undefined1 unaff_XMM10_Bg;
  char cVar59;
  undefined1 unaff_XMM10_Bh;
  char cVar60;
  undefined1 unaff_XMM10_Bi;
  char cVar61;
  undefined1 unaff_XMM10_Bj;
  char cVar62;
  undefined1 unaff_XMM10_Bk;
  char cVar63;
  undefined1 unaff_XMM10_Bl;
  char cVar64;
  undefined1 unaff_XMM10_Bm;
  char cVar65;
  undefined1 unaff_XMM10_Bn;
  char cVar66;
  undefined1 unaff_XMM10_Bo;
  char cVar67;
  undefined1 unaff_XMM10_Bp;
  char cVar68;
  undefined8 local_168;
  undefined4 local_160;
  
  puVar6 = (ulonglong *)*in_RCX;
  lVar7 = in_RCX[1];
  uVar28 = *puVar6;
  uVar25 = 0;
  uVar19 = uVar28 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar28 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar19 = uVar25;
  }
  uVar35 = uVar28 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar28 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar35 = uVar25;
  }
  pbVar22 = (byte *)(uVar19 * 0x30 + *(longlong *)(lVar7 + 8));
  uVar28 = *(ulonglong *)(lVar7 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar19) {
    pbVar22 = &DAT_00000008;
    uVar28 = uVar25;
  }
  pbVar20 = (byte *)(uVar35 * 0x30 + *(longlong *)(lVar7 + 0x28));
  uVar19 = *(ulonglong *)(lVar7 + 0x30) - uVar35;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar35) {
    pbVar20 = &DAT_00000008;
    uVar19 = uVar25;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar19) - uVar28;
  fVar17 = __real_3f800000;
  pbVar34 = pbVar22 + uVar28 * 0x30;
  pbVar24 = pbVar20 + uVar19 * 0x30;
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
  cVar53 = (char)iVar16;
  cVar54 = (char)((uint)iVar16 >> 8);
  cVar55 = (char)((uint)iVar16 >> 0x10);
  cVar56 = (char)((uint)iVar16 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bh,
                            CONCAT12(unaff_XMM10_Bg,CONCAT11(unaff_XMM10_Bf,unaff_XMM10_Be))) ==
                  CONCAT13(unaff_XMM10_Bh,
                           CONCAT12(unaff_XMM10_Bg,CONCAT11(unaff_XMM10_Bf,unaff_XMM10_Be))));
  cVar57 = (char)iVar16;
  cVar58 = (char)((uint)iVar16 >> 8);
  cVar59 = (char)((uint)iVar16 >> 0x10);
  cVar60 = (char)((uint)iVar16 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bl,
                            CONCAT12(unaff_XMM10_Bk,CONCAT11(unaff_XMM10_Bj,unaff_XMM10_Bi))) ==
                  CONCAT13(unaff_XMM10_Bl,
                           CONCAT12(unaff_XMM10_Bk,CONCAT11(unaff_XMM10_Bj,unaff_XMM10_Bi))));
  cVar61 = (char)iVar16;
  cVar62 = (char)((uint)iVar16 >> 8);
  cVar63 = (char)((uint)iVar16 >> 0x10);
  cVar64 = (char)((uint)iVar16 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM10_Bp,
                            CONCAT12(unaff_XMM10_Bo,CONCAT11(unaff_XMM10_Bn,unaff_XMM10_Bm))) ==
                  CONCAT13(unaff_XMM10_Bp,
                           CONCAT12(unaff_XMM10_Bo,CONCAT11(unaff_XMM10_Bn,unaff_XMM10_Bm))));
  cVar65 = (char)iVar16;
  cVar66 = (char)((uint)iVar16 >> 8);
  cVar67 = (char)((uint)iVar16 >> 0x10);
  cVar68 = (char)((uint)iVar16 >> 0x18);
LAB_140ac210e:
  do {
    if ((pbVar22 == (byte *)0x0) || (pbVar22 == pbVar34)) {
      if ((pbVar20 == (byte *)0x0) || (pbVar20 == pbVar24)) {
        return;
      }
      pbVar22 = (byte *)0x0;
      pbVar18 = pbVar20 + 0x30;
      pbVar26 = pbVar20;
    }
    else {
      pbVar18 = pbVar20;
      pbVar26 = pbVar22;
      pbVar22 = pbVar22 + 0x30;
    }
    pbVar20 = pbVar18;
    *puVar6 = *puVar6 + 1;
  } while (*(int *)(pbVar26 + 0x10) == 0);
  if (*(int *)(pbVar26 + 0x10) != 1) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar39 = *(float *)(pbVar26 + 0x14);
  lVar27 = *(longlong *)(pbVar26 + 0x18);
  if (plVar8[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar31 = *plVar8;
    bVar37 = (byte)(extraout_RAX >> 0x39);
    auVar40 = ZEXT216(CONCAT11(bVar37,bVar37));
    auVar40 = pshuflw(auVar40,auVar40,0);
    lVar30 = 0;
    uVar28 = extraout_RAX;
    while( true ) {
      uVar28 = plVar8[1] & uVar28;
      pcVar1 = (char *)(lVar31 + uVar28);
      cVar38 = auVar40[0];
      auVar48[0] = -(*pcVar1 == cVar38);
      cVar41 = auVar40[1];
      auVar48[1] = -(pcVar1[1] == cVar41);
      cVar42 = auVar40[2];
      auVar48[2] = -(pcVar1[2] == cVar42);
      cVar43 = auVar40[3];
      auVar48[3] = -(pcVar1[3] == cVar43);
      auVar48[4] = -(pcVar1[4] == cVar38);
      auVar48[5] = -(pcVar1[5] == cVar41);
      auVar48[6] = -(pcVar1[6] == cVar42);
      auVar48[7] = -(pcVar1[7] == cVar43);
      auVar48[8] = -(pcVar1[8] == cVar38);
      auVar48[9] = -(pcVar1[9] == cVar41);
      auVar48[10] = -(pcVar1[10] == cVar42);
      auVar48[0xb] = -(pcVar1[0xb] == cVar43);
      auVar48[0xc] = -(pcVar1[0xc] == cVar38);
      auVar48[0xd] = -(pcVar1[0xd] == cVar41);
      auVar48[0xe] = -(pcVar1[0xe] == cVar42);
      auVar48[0xf] = -(pcVar1[0xf] == cVar43);
      uVar32 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
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
      uVar29 = (uint)uVar32;
      while (uVar32 != 0) {
        uVar14 = 0;
        for (uVar15 = uVar29; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar14 = uVar14 + 1;
        }
        lVar33 = (uVar14 + uVar28 & plVar8[1]) * -0x78;
        if (*(longlong *)(lVar31 + -0x78 + lVar33) == lVar27) {
          fVar2 = *(float *)(lVar31 + -0x1c + lVar33);
          ui_adjust_wall_height_mode::clamp_wall_height();
          fVar52 = extraout_XMM0_Da;
          if ((extraout_RAX_00 & 1) == 0) {
            fVar52 = fVar39;
          }
          if (fVar52 == fVar2) goto LAB_140ac210e;
          if ((*pbVar26 & 1) != 0) {
            local_160 = *(undefined4 *)(pbVar26 + 0xc);
            local_168 = *(undefined8 *)(pbVar26 + 4);
            goto LAB_140ac23b6;
          }
          lVar27 = *(longlong *)(pbVar26 + 0x18);
          if (plVar8[3] == 0) goto LAB_140ac2741;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar31 = *plVar8;
          bVar37 = (byte)(extraout_RAX_01 >> 0x39);
          auVar40 = ZEXT216(CONCAT11(bVar37,bVar37));
          auVar40 = pshuflw(auVar40,auVar40,0);
          lVar30 = 0;
          uVar28 = extraout_RAX_01;
          goto LAB_140ac22df;
        }
        uVar32 = (ushort)(uVar29 - 1) & (ushort)uVar29;
        uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar32);
      }
      auVar44[0] = -(*pcVar1 == cVar53);
      auVar44[1] = -(pcVar1[1] == cVar54);
      auVar44[2] = -(pcVar1[2] == cVar55);
      auVar44[3] = -(pcVar1[3] == cVar56);
      auVar44[4] = -(pcVar1[4] == cVar57);
      auVar44[5] = -(pcVar1[5] == cVar58);
      auVar44[6] = -(pcVar1[6] == cVar59);
      auVar44[7] = -(pcVar1[7] == cVar60);
      auVar44[8] = -(pcVar1[8] == cVar61);
      auVar44[9] = -(pcVar1[9] == cVar62);
      auVar44[10] = -(pcVar1[10] == cVar63);
      auVar44[0xb] = -(pcVar1[0xb] == cVar64);
      auVar44[0xc] = -(pcVar1[0xc] == cVar65);
      auVar44[0xd] = -(pcVar1[0xd] == cVar66);
      auVar44[0xe] = -(pcVar1[0xe] == cVar67);
      auVar44[0xf] = -(pcVar1[0xf] == cVar68);
      if ((((((((((((((((SUB161(auVar44 >> 7,0) & 1) != 0 || (SUB161(auVar44 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar44 >> 0x17,0) & 1) != 0) || (SUB161(auVar44 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar44 >> 0x27,0) & 1) != 0) || (SUB161(auVar44 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar44 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar44 >> 0x3f,0) & 1) != 0) || (SUB161(auVar44 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar44 >> 0x4f,0) & 1) != 0) || (SUB161(auVar44 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar44 >> 0x5f,0) & 1) != 0) || (SUB161(auVar44 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar44 >> 0x6f,0) & 1) != 0) || (SUB161(auVar44 >> 0x77,0) & 1) != 0) ||
          auVar44[0xf] < '\0') break;
      uVar28 = uVar28 + lVar30 + 0x10;
      lVar30 = lVar30 + 0x10;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
LAB_140ac22df:
  uVar28 = plVar8[1] & uVar28;
  pcVar1 = (char *)(lVar31 + uVar28);
  cVar38 = auVar40[0];
  auVar49[0] = -(*pcVar1 == cVar38);
  cVar41 = auVar40[1];
  auVar49[1] = -(pcVar1[1] == cVar41);
  cVar42 = auVar40[2];
  auVar49[2] = -(pcVar1[2] == cVar42);
  cVar43 = auVar40[3];
  auVar49[3] = -(pcVar1[3] == cVar43);
  auVar49[4] = -(pcVar1[4] == cVar38);
  auVar49[5] = -(pcVar1[5] == cVar41);
  auVar49[6] = -(pcVar1[6] == cVar42);
  auVar49[7] = -(pcVar1[7] == cVar43);
  auVar49[8] = -(pcVar1[8] == cVar38);
  auVar49[9] = -(pcVar1[9] == cVar41);
  auVar49[10] = -(pcVar1[10] == cVar42);
  auVar49[0xb] = -(pcVar1[0xb] == cVar43);
  auVar49[0xc] = -(pcVar1[0xc] == cVar38);
  auVar49[0xd] = -(pcVar1[0xd] == cVar41);
  auVar49[0xe] = -(pcVar1[0xe] == cVar42);
  auVar49[0xf] = -(pcVar1[0xf] == cVar43);
  uVar32 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
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
           (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe | (ushort)(auVar49[0xf] >> 7) << 0xf;
  uVar29 = (uint)uVar32;
  if (uVar32 != 0) {
    while( true ) {
      uVar14 = 0;
      for (uVar15 = uVar29; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
        uVar14 = uVar14 + 1;
      }
      lVar33 = (uVar14 + uVar28 & plVar8[1]) * -0x78;
      if (*(longlong *)(lVar31 + -0x78 + lVar33) == lVar27) break;
      uVar32 = (ushort)(uVar29 - 1) & (ushort)uVar29;
      uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar32);
      if (uVar32 == 0) goto LAB_140ac2321;
    }
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
              ();
    uVar28 = *(ulonglong *)(lVar31 + -0x60 + lVar33);
    if (uVar28 <= extraout_EAX) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    if (uVar28 <= (ulonglong)extraout_EAX + 1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    ::utils::swizzlers::impl$0::x0y();
LAB_140ac23b6:
    resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    local_168 = CONCAT44(extraout_XMM0_Da_00 + local_168._4_4_,(undefined4)local_168);
    lVar27 = *(longlong *)(pbVar26 + 0x18);
    if (plVar8[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar37 = (byte)(extraout_RAX_02 >> 0x39);
      auVar40 = ZEXT216(CONCAT11(bVar37,bVar37));
      auVar40 = pshuflw(auVar40,auVar40,0);
      lVar31 = 0;
      uVar28 = extraout_RAX_02;
      while( true ) {
        uVar28 = plVar8[1] & uVar28;
        pcVar1 = (char *)(*plVar8 + uVar28);
        cVar38 = auVar40[0];
        auVar50[0] = -(*pcVar1 == cVar38);
        cVar41 = auVar40[1];
        auVar50[1] = -(pcVar1[1] == cVar41);
        cVar42 = auVar40[2];
        auVar50[2] = -(pcVar1[2] == cVar42);
        cVar43 = auVar40[3];
        auVar50[3] = -(pcVar1[3] == cVar43);
        auVar50[4] = -(pcVar1[4] == cVar38);
        auVar50[5] = -(pcVar1[5] == cVar41);
        auVar50[6] = -(pcVar1[6] == cVar42);
        auVar50[7] = -(pcVar1[7] == cVar43);
        auVar50[8] = -(pcVar1[8] == cVar38);
        auVar50[9] = -(pcVar1[9] == cVar41);
        auVar50[10] = -(pcVar1[10] == cVar42);
        auVar50[0xb] = -(pcVar1[0xb] == cVar43);
        auVar50[0xc] = -(pcVar1[0xc] == cVar38);
        auVar50[0xd] = -(pcVar1[0xd] == cVar41);
        auVar50[0xe] = -(pcVar1[0xe] == cVar42);
        auVar50[0xf] = -(pcVar1[0xf] == cVar43);
        uVar32 = (ushort)(SUB161(auVar50 >> 7,0) & 1) | (ushort)(SUB161(auVar50 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar50 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar50 >> 0x77,0) & 1) << 0xe | (ushort)(auVar50[0xf] >> 7) << 0xf
        ;
        uVar29 = (uint)uVar32;
        while (uVar32 != 0) {
          uVar14 = 0;
          for (uVar15 = uVar29; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          if (*(longlong *)(*plVar8 + -0x78 + (uVar14 + uVar28 & plVar8[1]) * -0x78) == lVar27) {
            *puVar9 = uVar3;
            resources::walls::inner_walls::InnerWalls::
            _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_height::closure_env_0>
                      ();
            lVar27 = *(longlong *)(pbVar26 + 0x18);
            if (plVar8[3] == 0) goto LAB_140ac2735;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar31 = *plVar8;
            bVar37 = (byte)(extraout_RAX_03 >> 0x39);
            auVar40 = ZEXT216(CONCAT11(bVar37,bVar37));
            auVar40 = pshuflw(auVar40,auVar40,0);
            lVar30 = 0;
            uVar28 = extraout_RAX_03;
            goto LAB_140ac2503;
          }
          uVar32 = (ushort)(uVar29 - 1) & (ushort)uVar29;
          uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar32);
        }
        auVar46[0] = -(*pcVar1 == cVar53);
        auVar46[1] = -(pcVar1[1] == cVar54);
        auVar46[2] = -(pcVar1[2] == cVar55);
        auVar46[3] = -(pcVar1[3] == cVar56);
        auVar46[4] = -(pcVar1[4] == cVar57);
        auVar46[5] = -(pcVar1[5] == cVar58);
        auVar46[6] = -(pcVar1[6] == cVar59);
        auVar46[7] = -(pcVar1[7] == cVar60);
        auVar46[8] = -(pcVar1[8] == cVar61);
        auVar46[9] = -(pcVar1[9] == cVar62);
        auVar46[10] = -(pcVar1[10] == cVar63);
        auVar46[0xb] = -(pcVar1[0xb] == cVar64);
        auVar46[0xc] = -(pcVar1[0xc] == cVar65);
        auVar46[0xd] = -(pcVar1[0xd] == cVar66);
        auVar46[0xe] = -(pcVar1[0xe] == cVar67);
        auVar46[0xf] = -(pcVar1[0xf] == cVar68);
        if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x1f,0) & 1) != 0) || (SUB161(auVar46 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x37,0) & 1) != 0) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
        uVar28 = uVar28 + lVar31 + 0x10;
        lVar31 = lVar31 + 0x10;
      }
    }
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
LAB_140ac2321:
  auVar45[0] = -(*pcVar1 == cVar53);
  auVar45[1] = -(pcVar1[1] == cVar54);
  auVar45[2] = -(pcVar1[2] == cVar55);
  auVar45[3] = -(pcVar1[3] == cVar56);
  auVar45[4] = -(pcVar1[4] == cVar57);
  auVar45[5] = -(pcVar1[5] == cVar58);
  auVar45[6] = -(pcVar1[6] == cVar59);
  auVar45[7] = -(pcVar1[7] == cVar60);
  auVar45[8] = -(pcVar1[8] == cVar61);
  auVar45[9] = -(pcVar1[9] == cVar62);
  auVar45[10] = -(pcVar1[10] == cVar63);
  auVar45[0xb] = -(pcVar1[0xb] == cVar64);
  auVar45[0xc] = -(pcVar1[0xc] == cVar65);
  auVar45[0xd] = -(pcVar1[0xd] == cVar66);
  auVar45[0xe] = -(pcVar1[0xe] == cVar67);
  auVar45[0xf] = -(pcVar1[0xf] == cVar68);
  if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 || (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar45 >> 0x17,0) & 1) != 0) || (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar45 >> 0x27,0) & 1) != 0) || (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar45 >> 0x37,0) & 1) != 0) || (SUB161(auVar45 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar45 >> 0x47,0) & 1) != 0) || (SUB161(auVar45 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar45 >> 0x57,0) & 1) != 0) || (SUB161(auVar45 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar45 >> 0x67,0) & 1) != 0) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') {
LAB_140ac2741:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar28 = uVar28 + lVar30 + 0x10;
  lVar30 = lVar30 + 0x10;
  goto LAB_140ac22df;
LAB_140ac2503:
  uVar28 = plVar8[1] & uVar28;
  pcVar1 = (char *)(lVar31 + uVar28);
  cVar38 = auVar40[0];
  auVar51[0] = -(*pcVar1 == cVar38);
  cVar41 = auVar40[1];
  auVar51[1] = -(pcVar1[1] == cVar41);
  cVar42 = auVar40[2];
  auVar51[2] = -(pcVar1[2] == cVar42);
  cVar43 = auVar40[3];
  auVar51[3] = -(pcVar1[3] == cVar43);
  auVar51[4] = -(pcVar1[4] == cVar38);
  auVar51[5] = -(pcVar1[5] == cVar41);
  auVar51[6] = -(pcVar1[6] == cVar42);
  auVar51[7] = -(pcVar1[7] == cVar43);
  auVar51[8] = -(pcVar1[8] == cVar38);
  auVar51[9] = -(pcVar1[9] == cVar41);
  auVar51[10] = -(pcVar1[10] == cVar42);
  auVar51[0xb] = -(pcVar1[0xb] == cVar43);
  auVar51[0xc] = -(pcVar1[0xc] == cVar38);
  auVar51[0xd] = -(pcVar1[0xd] == cVar41);
  auVar51[0xe] = -(pcVar1[0xe] == cVar42);
  auVar51[0xf] = -(pcVar1[0xf] == cVar43);
  uVar32 = (ushort)(SUB161(auVar51 >> 7,0) & 1) | (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar51 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar51 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar51 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar51 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar51 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar51 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar51 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar51 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar51 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar51 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar51 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe | (ushort)(auVar51[0xf] >> 7) << 0xf;
  uVar29 = (uint)uVar32;
  while (uVar32 != 0) {
    uVar14 = 0;
    for (uVar15 = uVar29; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar14 = uVar14 + 1;
    }
    lVar33 = (uVar14 + uVar28 & plVar8[1]) * -0x78;
    if (*(longlong *)(lVar31 + -0x78 + lVar33) == lVar27) {
      bVar37 = pbVar26[0x20];
      if ((*(uint *)(lVar31 + -0x38 + lVar33) < 2) && (bVar36 = bVar37 == 0, bVar37 = 1, bVar36)) {
        *puVar9 = uVar3;
        resources::walls::inner_walls::InnerWalls::
        _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_height::closure_env_0>
                  ();
        uVar13 = *(undefined8 *)(pbVar26 + 0x18);
        ::utils::inverse_lerp();
        *puVar12 = uVar5;
        lVar31 = *(longlong *)(lVar11 + 0x40);
        lVar27 = *(longlong *)(lVar11 + 0x30);
        if (lVar27 == *(longlong *)(lVar11 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        fVar39 = 0.0;
        if (0.0 <= extraout_XMM0_Da_01) {
          fVar39 = extraout_XMM0_Da_01;
        }
        fVar2 = fVar17;
        if (fVar39 <= fVar17) {
          fVar2 = fVar39;
        }
        lVar30 = *(longlong *)(lVar11 + 0x28);
        *(longlong *)(lVar30 + lVar27 * 0x28) = lVar31;
        *(undefined8 *)(lVar30 + 8 + lVar27 * 0x28) = uVar13;
        *(undefined8 *)(lVar30 + 0x10 + lVar27 * 0x28) = local_168;
        *(undefined4 *)(lVar30 + 0x18 + lVar27 * 0x28) = local_160;
        *(float *)(lVar30 + 0x1c + lVar27 * 0x28) = fVar2;
        *(undefined1 *)(lVar30 + 0x20 + lVar27 * 0x28) = 3;
        plVar21 = (longlong *)(lVar11 + 0x30);
        plVar23 = (longlong *)(lVar11 + 0x40);
      }
      else {
        uVar13 = *(undefined8 *)(pbVar26 + 0x18);
        *puVar10 = uVar4;
        lVar31 = *(longlong *)(lVar7 + 0x40);
        lVar27 = *(longlong *)(lVar7 + 0x30);
        if (lVar27 == *(longlong *)(lVar7 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar30 = *(longlong *)(lVar7 + 0x28);
        lVar33 = lVar27 * 0x20;
        *(longlong *)(lVar30 + lVar33) = lVar31;
        *(undefined8 *)(lVar30 + 8 + lVar33) = uVar13;
        *(undefined8 *)(lVar30 + 0x10 + lVar33) = local_168;
        *(undefined4 *)(lVar30 + 0x18 + lVar33) = local_160;
        *(undefined1 *)(lVar30 + 0x1c + lVar33) = 0;
        *(byte *)(lVar30 + 0x1d + lVar33) = bVar37;
        plVar21 = (longlong *)(lVar7 + 0x30);
        plVar23 = (longlong *)(lVar7 + 0x40);
      }
      *plVar21 = lVar27 + 1;
      *plVar23 = *plVar23 + 1;
      goto LAB_140ac210e;
    }
    uVar32 = (ushort)(uVar29 - 1) & (ushort)uVar29;
    uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar32);
  }
  auVar47[0] = -(*pcVar1 == cVar53);
  auVar47[1] = -(pcVar1[1] == cVar54);
  auVar47[2] = -(pcVar1[2] == cVar55);
  auVar47[3] = -(pcVar1[3] == cVar56);
  auVar47[4] = -(pcVar1[4] == cVar57);
  auVar47[5] = -(pcVar1[5] == cVar58);
  auVar47[6] = -(pcVar1[6] == cVar59);
  auVar47[7] = -(pcVar1[7] == cVar60);
  auVar47[8] = -(pcVar1[8] == cVar61);
  auVar47[9] = -(pcVar1[9] == cVar62);
  auVar47[10] = -(pcVar1[10] == cVar63);
  auVar47[0xb] = -(pcVar1[0xb] == cVar64);
  auVar47[0xc] = -(pcVar1[0xc] == cVar65);
  auVar47[0xd] = -(pcVar1[0xd] == cVar66);
  auVar47[0xe] = -(pcVar1[0xe] == cVar67);
  auVar47[0xf] = -(pcVar1[0xf] == cVar68);
  if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 || (SUB161(auVar47 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar47 >> 0x17,0) & 1) != 0) || (SUB161(auVar47 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar47 >> 0x27,0) & 1) != 0) || (SUB161(auVar47 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar47 >> 0x37,0) & 1) != 0) || (SUB161(auVar47 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar47 >> 0x47,0) & 1) != 0) || (SUB161(auVar47 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar47 >> 0x57,0) & 1) != 0) || (SUB161(auVar47 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar47 >> 0x67,0) & 1) != 0) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') {
LAB_140ac2735:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar28 = uVar28 + lVar30 + 0x10;
  lVar30 = lVar30 + 0x10;
  goto LAB_140ac2503;
}

