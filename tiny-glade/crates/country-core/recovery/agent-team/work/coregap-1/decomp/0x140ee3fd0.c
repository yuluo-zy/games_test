// 0x140ee3fd0 add_region :: void __cdecl add_region(void)

void __cdecl system_clutter::autoclutter::AutoClutterState::add_region(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  float fVar3;
  longlong *plVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined1 (*pauVar8) [16];
  uint uVar9;
  ulonglong extraout_RAX;
  longlong in_RCX;
  uint uVar10;
  uint uVar11;
  longlong in_RDX;
  ulonglong uVar12;
  int *in_R8;
  longlong lVar13;
  longlong lVar14;
  ushort uVar15;
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
  undefined1 local_130 [8];
  undefined8 local_128;
  undefined8 uStack_120;
  undefined1 *local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  float local_e0;
  float local_dc;
  int *local_d0;
  undefined1 local_c1;
  undefined8 local_c0;
  
  local_c0 = 0xfffffffffffffffe;
  local_d0 = in_R8;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar13 = *(longlong *)(in_RCX + 0x60);
  bVar17 = (byte)(extraout_RAX >> 0x38);
  bVar18 = bVar17 >> 1;
  auVar20 = ZEXT216(CONCAT11(bVar18,bVar18));
  auVar20 = pshuflw(auVar20,auVar20,0);
  lVar14 = 0;
  uVar12 = extraout_RAX;
  do {
    uVar12 = *(ulonglong *)(in_RCX + 0x68) & uVar12;
    pcVar1 = (char *)(lVar13 + uVar12);
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
    uVar15 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
    uVar10 = (uint)uVar15;
    while (uVar15 != 0) {
      uVar11 = 0;
      for (uVar9 = uVar10; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar11 = uVar11 + 1;
      }
      uVar16 = uVar11 + uVar12 & *(ulonglong *)(in_RCX + 0x68);
      if (*(longlong *)(lVar13 + -0x48 + uVar16 * -0x48) == in_RDX) {
        lVar14 = lVar13 + uVar16 * -0x48;
        fVar3 = *(float *)(lVar13 + -0x18 + uVar16 * -0x48);
        if (((((((fVar3 != (float)local_d0[10]) || (NAN(fVar3) || NAN((float)local_d0[10]))) ||
               (*(float *)(lVar14 + -0x14) != (float)local_d0[0xb])) ||
              ((NAN(*(float *)(lVar14 + -0x14)) || NAN((float)local_d0[0xb]) ||
               (*(float *)(lVar14 + -0x10) != (float)local_d0[0xc])))) ||
             ((NAN(*(float *)(lVar14 + -0x10)) || NAN((float)local_d0[0xc]) ||
              ((*(float *)(lVar14 + -0x28) != (float)local_d0[6] ||
               (NAN(*(float *)(lVar14 + -0x28)) || NAN((float)local_d0[6]))))))) ||
            (*(float *)(lVar14 + -0x24) != (float)local_d0[7])) ||
           ((((NAN(*(float *)(lVar14 + -0x24)) || NAN((float)local_d0[7]) ||
              (*(float *)(lVar14 + -0x20) != (float)local_d0[8])) ||
             (NAN(*(float *)(lVar14 + -0x20)) || NAN((float)local_d0[8]))) ||
            (((*(float *)(lVar14 + -0x1c) != (float)local_d0[9] ||
              (NAN(*(float *)(lVar14 + -0x1c)) || NAN((float)local_d0[9]))) ||
             ((*(int *)(lVar14 + -0xc) != local_d0[0xd] ||
              ((*(char *)(lVar14 + -8) != (char)local_d0[0xe] ||
               (*(char *)(lVar14 + -7) != *(char *)((longlong)local_d0 + 0x39))))))))))) {
          uVar6 = *(undefined8 *)(in_RCX + 8);
          uVar7 = *(undefined8 *)(in_RCX + 0x10);
          local_dc = *(float *)(lVar14 + -0x1c) * *(float *)(lVar14 + -0x28) +
                     *(float *)(lVar14 + -0x20) * *(float *)(lVar14 + -0x24);
          local_e0 = *(float *)(lVar14 + -0x20) * *(float *)(lVar14 + -0x28) -
                     *(float *)(lVar14 + -0x1c) * *(float *)(lVar14 + -0x24);
          local_c1 = 1;
          ::utils::swizzlers::impl$0::x0y();
          local_c1 = 1;
          ::utils::geometry::rectangle::Rectangle2d::new();
          local_c1 = 1;
          ::utils::geometry::rectangle::Rectangle2d::aabb();
          local_118 = local_130;
          local_c1 = 1;
          local_128 = uVar6;
          uStack_120 = uVar7;
          country_core::utils::dirty_tiles::DirtyTiles::
          iter_world_space_rect_cells<system_clutter::autoclutter::impl_1::remove_region_from_lazy_grid::closure_env_0>
                    ();
          uVar6 = *(undefined8 *)(in_RCX + 8);
          uVar7 = *(undefined8 *)(in_RCX + 0x10);
          local_dc = (float)local_d0[9] * (float)local_d0[6] +
                     (float)local_d0[8] * (float)local_d0[7];
          local_e0 = (float)local_d0[8] * (float)local_d0[6] -
                     (float)local_d0[9] * (float)local_d0[7];
          local_c1 = 1;
          ::utils::swizzlers::impl$0::x0y();
          local_c1 = 1;
          ::utils::geometry::rectangle::Rectangle2d::new();
          local_c1 = 1;
          ::utils::geometry::rectangle::Rectangle2d::aabb();
          local_118 = local_130;
          local_c1 = 1;
          local_128 = uVar6;
          uStack_120 = uVar7;
          country_core::utils::dirty_tiles::DirtyTiles::
          iter_world_space_rect_cells<system_clutter::autoclutter::impl_1::add_region_to_lazy_grid::closure_env_0>
                    ();
          uVar15 = *(ushort *)(lVar14 + -6);
          if (uVar15 < 0xfffe) {
            lVar5 = *(longlong *)(in_RCX + 0x40);
            if (lVar5 == *(longlong *)(in_RCX + 0x30)) {
              local_c1 = 1;
              alloc::raw_vec::RawVec::grow_one<u16,alloc::alloc::Global>();
            }
            *(ushort *)(*(longlong *)(in_RCX + 0x38) + lVar5 * 2) = uVar15;
            *(longlong *)(in_RCX + 0x40) = lVar5 + 1;
          }
          lVar13 = lVar13 + uVar16 * -0x48;
          local_128 = *(undefined8 *)local_d0;
          uStack_120 = *(undefined8 *)(local_d0 + 2);
          local_118 = *(undefined1 **)(local_d0 + 4);
          uStack_110 = *(undefined8 *)(local_d0 + 6);
          local_108 = *(undefined8 *)(local_d0 + 8);
          uStack_100 = *(undefined8 *)(local_d0 + 10);
          local_f8 = *(undefined8 *)(local_d0 + 0xc);
          uStack_f0 = *(undefined8 *)(local_d0 + 0xe);
          if (*(int *)(lVar13 + -0x40) == 4) {
            plVar4 = *(longlong **)(lVar14 + -0x38);
            LOCK();
            *plVar4 = *plVar4 + -1;
            UNLOCK();
            if (*plVar4 == 0) {
              alloc::sync::Arc::drop_slow<slice2_<glam::f32::vec3::Vec3>,alloc::alloc::Global>();
            }
          }
          *(undefined8 *)(lVar13 + -0x10) = local_f8;
          *(undefined8 *)(lVar13 + -8) = uStack_f0;
          *(undefined8 *)(lVar13 + -0x20) = local_108;
          *(undefined8 *)(lVar13 + -0x18) = uStack_100;
          *(undefined1 **)(lVar13 + -0x30) = local_118;
          *(undefined8 *)(lVar13 + -0x28) = uStack_110;
          *(undefined8 *)(lVar13 + -0x40) = local_128;
          *(undefined8 *)(lVar13 + -0x38) = uStack_120;
          *(undefined1 *)(lVar14 + -4) = *(undefined1 *)(in_RCX + 0x90);
          local_c1 = 0;
          flush_candidates_pending_removal();
          return;
        }
        *(undefined1 *)(lVar14 + -4) = *(undefined1 *)(in_RCX + 0x90);
        local_c1 = 1;
        flush_candidates_pending_removal();
        if (*local_d0 != 4) {
          return;
        }
        plVar4 = *(longlong **)(local_d0 + 2);
        LOCK();
        *plVar4 = *plVar4 + -1;
        UNLOCK();
        if (*plVar4 != 0) {
          return;
        }
        alloc::sync::Arc::drop_slow<slice2_<glam::f32::vec3::Vec3>,alloc::alloc::Global>();
        return;
      }
      uVar15 = (ushort)(uVar10 - 1) & (ushort)uVar10;
      uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar15);
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
      if (*(longlong *)(in_RCX + 0x70) == 0) {
        local_c1 = 1;
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<system_clutter::autoclutter::ClutterRegionId,system_clutter::autoclutter::AutoClutterRegion>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<system_clutter::autoclutter::ClutterRegionId,system_clutter::autoclutter::AutoClutterRegion,std::hash::random::RandomState>_>
                  ();
      }
      uVar6 = *(undefined8 *)(in_RCX + 8);
      uVar7 = *(undefined8 *)(in_RCX + 0x10);
      local_dc = (float)local_d0[9] * (float)local_d0[6] + (float)local_d0[8] * (float)local_d0[7];
      local_e0 = (float)local_d0[8] * (float)local_d0[6] - (float)local_d0[9] * (float)local_d0[7];
      local_c1 = 1;
      ::utils::swizzlers::impl$0::x0y();
      local_c1 = 1;
      ::utils::geometry::rectangle::Rectangle2d::new();
      local_c1 = 1;
      ::utils::geometry::rectangle::Rectangle2d::aabb();
      local_118 = local_130;
      local_c1 = 1;
      local_128 = uVar6;
      uStack_120 = uVar7;
      country_core::utils::dirty_tiles::DirtyTiles::
      iter_world_space_rect_cells<system_clutter::autoclutter::impl_1::add_region_to_lazy_grid::closure_env_0>
                ();
      local_128 = *(undefined8 *)local_d0;
      uStack_120 = *(undefined8 *)(local_d0 + 2);
      local_118 = *(undefined1 **)(local_d0 + 4);
      uStack_110 = *(undefined8 *)(local_d0 + 6);
      local_108 = *(undefined8 *)(local_d0 + 8);
      uStack_100 = *(undefined8 *)(local_d0 + 10);
      local_f8 = *(undefined8 *)(local_d0 + 0xc);
      uStack_f0 = *(undefined8 *)(local_d0 + 0xe);
      pauVar8 = *(undefined1 (**) [16])(in_RCX + 0x60);
      uVar12 = *(ulonglong *)(in_RCX + 0x68);
      uVar16 = extraout_RAX & uVar12;
      auVar20 = *(undefined1 (*) [16])(*pauVar8 + uVar16);
      uVar15 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
      uVar10 = (uint)uVar15;
      if (uVar15 == 0) {
        lVar13 = 0x10;
        do {
          uVar16 = uVar16 + lVar13 & uVar12;
          auVar20 = *(undefined1 (*) [16])(*pauVar8 + uVar16);
          lVar13 = lVar13 + 0x10;
          uVar10 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
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
        } while (uVar10 == 0);
      }
      uVar11 = 0;
      for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
        uVar11 = uVar11 + 1;
      }
      uVar16 = uVar11 + uVar16 & uVar12;
      bVar18 = (*pauVar8)[uVar16];
      if (-1 < (char)bVar18) {
        auVar20 = *pauVar8;
        uVar10 = 0;
        for (uVar11 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
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
                                    (ushort)(byte)(auVar20[0xf] >> 7) << 0xf); (uVar11 & 1) == 0;
            uVar11 = uVar11 >> 1 | 0x80000000) {
          uVar10 = uVar10 + 1;
        }
        uVar16 = (ulonglong)uVar10;
        bVar18 = (*pauVar8)[uVar16];
      }
      bVar17 = bVar17 >> 1;
      (*pauVar8)[uVar16] = bVar17;
      pauVar8[1][uVar16 - 0x10 & uVar12] = bVar17;
      *(longlong *)(in_RCX + 0x70) = *(longlong *)(in_RCX + 0x70) - (ulonglong)(bVar18 & 1);
      *(longlong *)((longlong)pauVar8 + uVar16 * -0x48 + -0x48) = in_RDX;
      puVar2 = (undefined8 *)((longlong)pauVar8 + uVar16 * -0x48 + -0x40);
      *puVar2 = local_128;
      puVar2[1] = uStack_120;
      puVar2 = (undefined8 *)((longlong)pauVar8 + uVar16 * -0x48 + -0x30);
      *puVar2 = local_118;
      puVar2[1] = uStack_110;
      puVar2 = (undefined8 *)((longlong)pauVar8 + uVar16 * -0x48 + -0x20);
      *puVar2 = local_108;
      puVar2[1] = uStack_100;
      puVar2 = (undefined8 *)((longlong)pauVar8 + uVar16 * -0x48 + -0x10);
      *puVar2 = local_f8;
      puVar2[1] = uStack_f0;
      *(longlong *)(in_RCX + 0x78) = *(longlong *)(in_RCX + 0x78) + 1;
      *(undefined1 *)((longlong)pauVar8 + uVar16 * -0x48 + -4) = *(undefined1 *)(in_RCX + 0x90);
      local_c1 = 0;
      flush_candidates_pending_removal();
      return;
    }
    uVar12 = uVar12 + lVar14 + 0x10;
    lVar14 = lVar14 + 0x10;
  } while( true );
}


