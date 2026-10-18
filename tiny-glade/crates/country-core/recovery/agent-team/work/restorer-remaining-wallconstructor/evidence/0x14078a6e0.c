
void __cdecl
system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::
maintain_stone_column_colliders(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  char extraout_AL;
  uint uVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar17;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  void *_Dst;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *in_RCX;
  longlong *plVar18;
  undefined8 *in_RDX;
  undefined8 *puVar19;
  ulonglong uVar20;
  undefined8 *in_R8;
  undefined *puVar21;
  ulonglong uVar22;
  longlong *in_R9;
  longlong lVar23;
  ushort uVar24;
  ulonglong uVar25;
  longlong lVar26;
  ulonglong *puVar27;
  byte bVar28;
  char cVar29;
  char cVar31;
  char cVar32;
  char cVar33;
  undefined1 auVar30 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  longlong local_2b8;
  ulonglong *puStack_2b0;
  longlong local_2a8;
  longlong *plStack_290;
  longlong local_1f8;
  ulonglong local_1f0;
  longlong local_1e8;
  longlong lStack_1e0;
  longlong local_1d8;
  undefined8 uStack_1d0;
  longlong *plStack_1c0;
  longlong lStack_1b0;
  longlong local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_160;
  ulonglong local_120;
  longlong local_118;
  ulonglong local_110;
  ulonglong local_f8;
  ulonglong local_f0;
  longlong *local_e8;
  longlong local_e0;
  ulonglong *local_d8;
  longlong local_d0;
  undefined4 *local_c0;
  undefined8 local_b8;
  longlong local_b0;
  longlong local_a8;
  ulonglong *local_98;
  undefined4 *local_90;
  undefined4 local_84;
  longlong *local_80;
  char local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  puVar19 = in_RDX;
  local_e8 = in_R9;
  puffin::are_scopes_on();
  local_72 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar19 != (undefined8 *)0x0) {
        if (puVar19 < (undefined8 *)0x77) {
          if (*(char *)(puVar19 + 0x28550112) < -0x40) {
LAB_14078a775:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (puVar19 != (undefined8 *)0x77) goto LAB_14078a775;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar19 != (undefined8 *)0xfffffffffffffffe)) {
        if ((longlong)puVar19 + 2U < 0x77) {
          if (*(char *)((longlong)puVar19 + 0x142a80892) < -0x40) {
LAB_14078a7c2:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((longlong)puVar19 + 2U != 0x77) goto LAB_14078a7c2;
      }
    }
    puVar21 = &DAT_142a80a2b;
    do {
      bVar28 = puVar21[-1];
      uVar16 = (uint)(char)bVar28;
      if ((char)bVar28 < '\0') {
        bVar3 = puVar21[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar21[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar21 + -4;
            puVar21 = puVar21 + -4;
            uVar16 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar21 = puVar21 + -3;
            uVar16 = bVar4 & 0xf;
          }
          uVar16 = bVar3 & 0x3f | uVar16 << 6;
        }
        else {
          puVar21 = puVar21 + -2;
          uVar16 = bVar3 & 0x1f;
        }
        uVar16 = (uint)(bVar28 & 0x3f) | uVar16 << 6;
        if (uVar16 != 0x5c) goto LAB_14078a884;
LAB_14078a899:
        if (puVar21 == &UNK_142a809c7) break;
        if (puVar21 + -0x142a809c7 < (undefined *)0x63) {
          if (-0x41 < (char)puVar21[1]) break;
        }
        else if (puVar21 + -0x142a809c7 == (undefined *)0x63) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar21 = puVar21 + -1;
      if (uVar16 == 0x5c) goto LAB_14078a899;
LAB_14078a884:
      if (uVar16 == 0x2f) goto LAB_14078a899;
    } while (puVar21 != &DAT_142a809c8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14078b181;
    if (*plVar18 != 0) goto LAB_14078b18d;
    *plVar18 = -1;
    local_80 = plVar18;
    puffin::ThreadProfiler::begin_scope();
    *local_80 = *local_80 + 1;
  }
  puVar5 = (ulonglong *)*in_R8;
  lVar6 = in_R8[1];
  uVar7 = *puVar5;
  uVar25 = 0;
  uVar22 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar22 = uVar25;
  }
  uVar20 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar20 = uVar25;
  }
  puVar27 = (ulonglong *)(uVar22 * 0x10 + *(longlong *)(lVar6 + 8));
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar22;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar22) {
    puVar27 = (ulonglong *)&DAT_00000008;
    uVar7 = uVar25;
  }
  lVar17 = uVar20 * 0x10 + *(longlong *)(lVar6 + 0x28);
  uVar22 = *(ulonglong *)(lVar6 + 0x30) - uVar20;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar20) {
    lVar17 = 8;
    uVar22 = uVar25;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar22) - uVar7;
  local_98 = puVar27 + uVar7 * 2;
  local_a8 = uVar22 * 0x10 + lVar17;
  local_80 = (longlong *)CONCAT44(local_80._4_4_,*(undefined4 *)((longlong)in_RCX + 0x1c));
  local_e0 = *in_RCX;
  local_90 = (undefined4 *)in_RCX[2];
  local_84 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_b8 = *in_RDX;
  local_c0 = (undefined4 *)in_RDX[2];
  local_b0 = local_e0 + 0x48;
  do {
    if ((puVar27 == (ulonglong *)0x0) || (puVar27 == local_98)) {
      if ((lVar17 == 0) || (lVar17 == local_a8)) goto LAB_14078ab3e;
      lVar17 = lVar17 + 0x10;
      puVar27 = (ulonglong *)0x0;
    }
    else {
      puVar27 = puVar27 + 2;
    }
    *puVar5 = *puVar5 + 1;
    *local_90 = local_80._0_4_;
    local_1f8 = 0;
    local_1f0 = 4;
    local_1e8 = 0;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if ((!SBORROW8(0,local_2b8)) && (local_2b8 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *local_c0 = local_84;
    *local_90 = local_80._0_4_;
    country_core::systems::collision::world::RaycastWorld::
    remove_collider<country_core::resources::walls::WallId>();
    if ((char)local_1d8 != '\x11') {
      LOCK();
      *plStack_1c0 = *plStack_1c0 + -1;
      UNLOCK();
      if (*plStack_1c0 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
      }
    }
    uVar25 = 1;
  } while( true );
code_r0x00014078ac8a:
  uVar24 = (ushort)(uVar16 - 1) & (ushort)uVar16;
  uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar24);
  if (uVar24 == 0) goto LAB_14078aca0;
  goto LAB_14078ac6d;
LAB_14078ab3e:
  *local_90 = local_80._0_4_;
  local_1f8 = *(longlong *)(local_e0 + 8);
  lStack_1e0 = *(longlong *)(local_e0 + 0x10);
  *(undefined8 *)(local_e0 + 0x10) = 0;
  local_1f0 = local_1f8 + lStack_1e0 * 8;
  local_1e8 = local_e0;
  local_1d8 = 0;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::walls::WallId,alloc::vec::drain::Drain<country_core::resources::walls::WallId,alloc::alloc::Global>_>
            ();
  lVar6 = local_e0;
  local_d0 = local_2b8;
  local_d8 = puStack_2b0 + local_2a8;
  local_a8 = local_e0 + 0x38;
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  if (puStack_2b0 != local_d8) {
    while (uVar7 = *puStack_2b0, local_f0 = uVar7, *(longlong *)(lVar6 + 0x30) != 0) {
      local_98 = puStack_2b0 + 1;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar17 = *(longlong *)(lVar6 + 0x18);
      bVar28 = (byte)(extraout_RAX_02 >> 0x39);
      auVar30 = ZEXT216(CONCAT11(bVar28,bVar28));
      auVar30 = pshuflw(auVar30,auVar30,0);
      lVar23 = 0;
      uVar22 = extraout_RAX_02;
      while( true ) {
        uVar22 = *(ulonglong *)(lVar6 + 0x20) & uVar22;
        pcVar2 = (char *)(lVar17 + uVar22);
        cVar29 = auVar30[0];
        auVar35[0] = -(*pcVar2 == cVar29);
        cVar31 = auVar30[1];
        auVar35[1] = -(pcVar2[1] == cVar31);
        cVar32 = auVar30[2];
        auVar35[2] = -(pcVar2[2] == cVar32);
        cVar33 = auVar30[3];
        auVar35[3] = -(pcVar2[3] == cVar33);
        auVar35[4] = -(pcVar2[4] == cVar29);
        auVar35[5] = -(pcVar2[5] == cVar31);
        auVar35[6] = -(pcVar2[6] == cVar32);
        auVar35[7] = -(pcVar2[7] == cVar33);
        auVar35[8] = -(pcVar2[8] == cVar29);
        auVar35[9] = -(pcVar2[9] == cVar31);
        auVar35[10] = -(pcVar2[10] == cVar32);
        auVar35[0xb] = -(pcVar2[0xb] == cVar33);
        auVar35[0xc] = -(pcVar2[0xc] == cVar29);
        auVar35[0xd] = -(pcVar2[0xd] == cVar31);
        auVar35[0xe] = -(pcVar2[0xe] == cVar32);
        auVar35[0xf] = -(pcVar2[0xf] == cVar33);
        uVar24 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf
        ;
        uVar16 = (uint)uVar24;
        if (uVar24 != 0) break;
LAB_14078aca0:
        auVar34[0] = -(*pcVar2 == (char)iVar11);
        auVar34[1] = -(pcVar2[1] == (char)((uint)iVar11 >> 8));
        auVar34[2] = -(pcVar2[2] == (char)((uint)iVar11 >> 0x10));
        auVar34[3] = -(pcVar2[3] == (char)((uint)iVar11 >> 0x18));
        auVar34[4] = -(pcVar2[4] == (char)iVar12);
        auVar34[5] = -(pcVar2[5] == (char)((uint)iVar12 >> 8));
        auVar34[6] = -(pcVar2[6] == (char)((uint)iVar12 >> 0x10));
        auVar34[7] = -(pcVar2[7] == (char)((uint)iVar12 >> 0x18));
        auVar34[8] = -(pcVar2[8] == (char)iVar13);
        auVar34[9] = -(pcVar2[9] == (char)((uint)iVar13 >> 8));
        auVar34[10] = -(pcVar2[10] == (char)((uint)iVar13 >> 0x10));
        auVar34[0xb] = -(pcVar2[0xb] == (char)((uint)iVar13 >> 0x18));
        auVar34[0xc] = -(pcVar2[0xc] == (char)iVar14);
        auVar34[0xd] = -(pcVar2[0xd] == (char)((uint)iVar14 >> 8));
        auVar34[0xe] = -(pcVar2[0xe] == (char)((uint)iVar14 >> 0x10));
        auVar34[0xf] = -(pcVar2[0xf] == (char)((uint)iVar14 >> 0x18));
        if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 || (SUB161(auVar34 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x1f,0) & 1) != 0) || (SUB161(auVar34 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x37,0) & 1) != 0) || (SUB161(auVar34 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar34 >> 0x47,0) & 1) != 0) || (SUB161(auVar34 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar34 >> 0x57,0) & 1) != 0) || (SUB161(auVar34 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar34 >> 0x67,0) & 1) != 0) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') goto LAB_14078b141;
        uVar22 = uVar22 + lVar23 + 0x10;
        lVar23 = lVar23 + 0x10;
      }
LAB_14078ac6d:
      uVar9 = 0;
      for (uVar10 = uVar16; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      uVar25 = uVar9 + uVar22 & *(ulonglong *)(lVar6 + 0x20);
      if (uVar7 != *(ulonglong *)(lVar17 + -0x20 + uVar25 * -0x20)) goto code_r0x00014078ac8a;
      lVar26 = uVar25 * -0x20;
      lVar23 = *(longlong *)(lVar17 + -8 + lVar26);
      if (lVar23 == 0) {
        *local_c0 = local_84;
        *local_90 = local_80._0_4_;
        country_core::systems::collision::world::RaycastWorld::
        remove_collider<country_core::resources::walls::WallId>();
        if ((char)local_1d8 != '\x11') {
          LOCK();
          *plStack_1c0 = *plStack_1c0 + -1;
          UNLOCK();
          if (*plStack_1c0 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
      }
      else {
        lStack_1b0 = *(longlong *)(lVar17 + lVar26 + -0x10);
        local_1a8 = lVar23 * 0x20 + lStack_1b0;
        local_1f8 = 1;
        local_1f0 = 0;
        uStack_1d0 = 0;
        uStack_1a0 = 0;
        local_160 = 0;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env_0>,array_<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env_1>_>
                  ();
        uVar22 = local_f8;
        if (local_f8 < 3) {
          local_118 = 4;
          uVar25 = 0;
        }
        else {
          uVar25 = local_f8 / 3;
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_03 == 0) {
            local_71 = 1;
                    // WARNING: Subroutine does not return
            alloc::raw_vec::handle_error();
          }
          if (uVar22 - 3 < 3) {
            uVar22 = 0;
          }
          else {
            lVar17 = 0;
            uVar22 = 0;
            do {
              iVar15 = (int)lVar17;
              *(int *)(extraout_RAX_03 + lVar17 * 4) = iVar15;
              *(int *)(extraout_RAX_03 + 4 + lVar17 * 4) = iVar15 + 1;
              *(int *)(extraout_RAX_03 + 8 + lVar17 * 4) = iVar15 + 2;
              uVar22 = uVar22 + 2;
              *(int *)(extraout_RAX_03 + 0xc + lVar17 * 4) = iVar15 + 3;
              *(int *)(extraout_RAX_03 + 0x10 + lVar17 * 4) = iVar15 + 4;
              *(int *)(extraout_RAX_03 + 0x14 + lVar17 * 4) = iVar15 + 5;
              lVar17 = lVar17 + 6;
            } while ((uVar25 & 0x7fffffffffffffe) != uVar22);
          }
          local_118 = extraout_RAX_03;
          if ((uVar25 & 1) != 0) {
            iVar15 = (int)uVar22;
            *(int *)(extraout_RAX_03 + uVar22 * 0xc) = iVar15 * 3;
            *(int *)(extraout_RAX_03 + 4 + uVar22 * 0xc) = iVar15 * 3 + 1;
            *(int *)(extraout_RAX_03 + 8 + uVar22 * 0xc) = iVar15 * 3 + 2;
          }
        }
        *local_c0 = local_84;
        *local_90 = local_80._0_4_;
        local_1f8 = -0x8000000000000000;
        local_71 = 0;
        local_120 = uVar25;
        local_110 = uVar25;
        country_core::systems::collision::world::RaycastMeshData::new_indexed();
        local_1f8 = 1;
        local_1f0 = 1;
        plStack_1c0 = plStack_290;
        local_1d8 = local_2a8;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        memcpy(_Dst,&local_1f8,0x90);
        local_1f8 = CONCAT71(local_1f8._1_7_,6);
        local_71 = 0;
        local_1f0 = uVar7;
        country_core::systems::collision::world::RaycastMesh::new();
        local_1f0 = local_1f0 & 0xffffffff00000000;
        local_1f8 = 0;
        local_1e8 = 0;
        lStack_1e0 = 0x3f80000000000000;
        local_71 = 0;
        country_core::systems::collision::world::RaycastWorld::
        upsert_collider<country_core::resources::walls::WallId,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
                  ();
      }
      uVar25 = 1;
      puStack_2b0 = local_98;
      if (local_98 == local_d8) goto LAB_14078b061;
    }
LAB_14078b141:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_14078b061:
  if (local_d0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (uVar25 != 0) {
    *(undefined4 *)local_e8[2] = *(undefined4 *)((longlong)local_e8 + 0x1c);
    lVar6 = *local_e8;
    lVar17 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar17 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar6 + 0x28) + lVar17 * 8) = uVar8;
    *(longlong *)(lVar6 + 0x30) = lVar17 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  }
  if (local_72 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar18 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_14078b181:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_14078b18d:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    local_80 = plVar18;
    puffin::ThreadProfiler::end_scope();
    *local_80 = *local_80 + 1;
  }
  return;
}

