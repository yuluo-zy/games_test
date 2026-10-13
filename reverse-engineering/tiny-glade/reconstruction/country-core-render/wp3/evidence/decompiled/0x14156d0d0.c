// address 0x14156d0d0
// name create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>
// signature void __cdecl create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>(void)

void __cdecl
country_core::render::nani::gpu_world::GpuWorld::
create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env_0>
          (void)

{
  ulonglong uVar1;
  char *pcVar2;
  undefined8 *puVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined **ppuVar16;
  char extraout_AL;
  char extraout_AL_00;
  longlong *extraout_RAX;
  longlong lVar17;
  ulonglong extraout_RAX_00;
  code *extraout_RAX_01;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong in_RCX;
  longlong lVar22;
  ulonglong *in_RDX;
  ulonglong uVar23;
  longlong in_R8;
  ulonglong uVar24;
  uint uVar25;
  ulonglong in_R9;
  ushort uVar26;
  undefined8 *puVar27;
  bool bVar28;
  char cVar29;
  byte bVar30;
  char cVar32;
  char cVar33;
  char cVar34;
  undefined1 auVar31 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
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
  longlong in_stack_00000028;
  ulonglong in_stack_00000030;
  longlong *in_stack_00000038;
  undefined *local_180;
  void *pvStack_178;
  ulonglong local_170;
  int local_128;
  char local_118;
  undefined *local_a8;
  void *pvStack_a0;
  ulonglong local_98;
  char *local_90;
  undefined **local_88;
  code *pcStack_80;
  undefined **local_78;
  code *local_70;
  char local_62;
  byte local_61 [33];
  
  local_61[1] = 0xfe;
  local_61[2] = 0xff;
  local_61[3] = 0xff;
  local_61[4] = 0xff;
  local_61[5] = 0xff;
  local_61[6] = 0xff;
  local_61[7] = 0xff;
  local_61[8] = 0xff;
  bVar30 = (byte)in_RDX;
  owned_subset_mut();
  local_61[0] = bVar30;
  if (in_R9 != 0) {
    if (in_R9 <= in_stack_00000030) {
      in_stack_00000030 = in_R9;
    }
    iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    uVar24 = 0;
    if (in_stack_00000030 != 0) {
      do {
        while (uVar1 = uVar24 + 1,
              in_RDX[0x14] < *(ulonglong *)(&DAT_142c77010 + (ulonglong)local_61[0] * 8)) {
          puVar27 = (undefined8 *)(uVar24 * 0x10 + in_R8);
          local_90 = (char *)(uVar24 * 0x30 + in_stack_00000028);
LAB_14156d280:
          iVar4 = (int)in_RDX[0x16];
          *(int *)(in_RDX + 0x16) = iVar4 + 1;
          if (in_RDX[6] != 0) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                      ();
            bVar30 = (byte)(extraout_RAX_00 >> 0x39);
            auVar31 = ZEXT216(CONCAT11(bVar30,bVar30));
            auVar31 = pshuflw(auVar31,auVar31,0);
            lVar17 = 0;
            uVar24 = extraout_RAX_00;
            while( true ) {
              uVar24 = in_RDX[4] & uVar24;
              pcVar2 = (char *)(in_RDX[3] + uVar24);
              cVar29 = auVar31[0];
              auVar36[0] = -(*pcVar2 == cVar29);
              cVar32 = auVar31[1];
              auVar36[1] = -(pcVar2[1] == cVar32);
              cVar33 = auVar31[2];
              auVar36[2] = -(pcVar2[2] == cVar33);
              cVar34 = auVar31[3];
              auVar36[3] = -(pcVar2[3] == cVar34);
              auVar36[4] = -(pcVar2[4] == cVar29);
              auVar36[5] = -(pcVar2[5] == cVar32);
              auVar36[6] = -(pcVar2[6] == cVar33);
              auVar36[7] = -(pcVar2[7] == cVar34);
              auVar36[8] = -(pcVar2[8] == cVar29);
              auVar36[9] = -(pcVar2[9] == cVar32);
              auVar36[10] = -(pcVar2[10] == cVar33);
              auVar36[0xb] = -(pcVar2[0xb] == cVar34);
              auVar36[0xc] = -(pcVar2[0xc] == cVar29);
              auVar36[0xd] = -(pcVar2[0xd] == cVar32);
              auVar36[0xe] = -(pcVar2[0xe] == cVar33);
              auVar36[0xf] = -(pcVar2[0xf] == cVar34);
              uVar26 = (ushort)(SUB161(auVar36 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar36[0xf] >> 7) << 0xf;
              uVar25 = (uint)uVar26;
              while (uVar26 != 0) {
                uVar5 = 0;
                for (uVar6 = uVar25; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                  uVar5 = uVar5 + 1;
                }
                if (*(int *)(in_RDX[3] + (uVar5 + uVar24 & in_RDX[4]) * -8 + -8) == iVar4)
                goto LAB_14156d280;
                uVar26 = (ushort)(uVar25 - 1) & (ushort)uVar25;
                uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar26);
              }
              auVar35[0] = -(*pcVar2 == (char)iVar7);
              auVar35[1] = -(pcVar2[1] == (char)((uint)iVar7 >> 8));
              auVar35[2] = -(pcVar2[2] == (char)((uint)iVar7 >> 0x10));
              auVar35[3] = -(pcVar2[3] == (char)((uint)iVar7 >> 0x18));
              auVar35[4] = -(pcVar2[4] == (char)iVar8);
              auVar35[5] = -(pcVar2[5] == (char)((uint)iVar8 >> 8));
              auVar35[6] = -(pcVar2[6] == (char)((uint)iVar8 >> 0x10));
              auVar35[7] = -(pcVar2[7] == (char)((uint)iVar8 >> 0x18));
              auVar35[8] = -(pcVar2[8] == (char)iVar9);
              auVar35[9] = -(pcVar2[9] == (char)((uint)iVar9 >> 8));
              auVar35[10] = -(pcVar2[10] == (char)((uint)iVar9 >> 0x10));
              auVar35[0xb] = -(pcVar2[0xb] == (char)((uint)iVar9 >> 0x18));
              auVar35[0xc] = -(pcVar2[0xc] == (char)iVar10);
              auVar35[0xd] = -(pcVar2[0xd] == (char)((uint)iVar10 >> 8));
              auVar35[0xe] = -(pcVar2[0xe] == (char)((uint)iVar10 >> 0x10));
              auVar35[0xf] = -(pcVar2[0xf] == (char)((uint)iVar10 >> 0x18));
              if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar35 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar35 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar35 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar35 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar35 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar35 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar35 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar35 >> 0x4f,0) & 1) != 0) || (SUB161(auVar35 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar35 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar35 >> 0x67,0) & 1) != 0) || (SUB161(auVar35 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar35 >> 0x77,0) & 1) != 0) || auVar35[0xf] < '\0') break;
              uVar24 = uVar24 + lVar17 + 0x10;
              lVar17 = lVar17 + 0x10;
            }
          }
          uVar24 = in_RDX[2];
          if (uVar24 == *in_RDX) {
            alloc::raw_vec::RawVec::
            grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
          }
          *(int *)(in_RDX[1] + uVar24 * 4) = iVar4;
          in_RDX[2] = uVar24 + 1;
          hashbrown::map::HashMap::
          insert<country_core::render::nani::gpu_world::InstanceKey,u32,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          bVar30 = local_61[0];
          lVar17 = in_stack_00000038[2];
          if (lVar17 == *in_stack_00000038) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                      ();
          }
          uVar23 = uVar24 & 0xffffffff;
          lVar20 = in_stack_00000038[1];
          *(int *)(lVar20 + lVar17 * 0xc) = iVar4;
          *(int *)(lVar20 + 4 + lVar17 * 0xc) = (int)uVar24;
          *(byte *)(lVar20 + 8 + lVar17 * 0xc) = bVar30;
          in_stack_00000038[2] = lVar17 + 1;
          uVar24 = in_RDX[0xc];
          if (uVar24 <= uVar23) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          uVar11 = puVar27[1];
          puVar3 = (undefined8 *)(in_RDX[0xb] + uVar23 * 0x10);
          *puVar3 = *puVar27;
          puVar3[1] = uVar11;
          uVar18 = in_RDX[9];
          do {
            uVar21 = uVar23;
            if (uVar18 <= uVar23) {
              uVar21 = uVar18;
            }
            LOCK();
            uVar19 = in_RDX[9];
            bVar28 = uVar18 == uVar19;
            if (bVar28) {
              in_RDX[9] = uVar21;
              uVar19 = uVar18;
            }
            UNLOCK();
            uVar18 = uVar19;
          } while (!bVar28);
          uVar18 = in_RDX[10];
          do {
            uVar21 = uVar23;
            if (uVar23 < uVar18) {
              uVar21 = uVar18;
            }
            LOCK();
            uVar19 = in_RDX[10];
            bVar28 = uVar18 == uVar19;
            if (bVar28) {
              in_RDX[10] = uVar21;
              uVar19 = uVar18;
            }
            UNLOCK();
            uVar18 = uVar19;
          } while (!bVar28);
          lVar17 = (**(code **)(in_RDX[0x13] + 0x28))(in_RDX[0x12]);
          lVar20 = (**(code **)(uVar24 + 0x18))(lVar17);
          if (((lVar17 == 0) || (lVar20 != -0x990b32ccf5b0500)) || (uVar24 != 0x2095664170371110)) {
            local_a8 = &anon_107183261458dea85466985370debbb6_18_llvm_3641509820080252961;
            pvStack_a0 = (void *)0x3a;
            local_88 = (undefined **)local_61;
            pcStack_80 = gpu_world::impl$17::fmt;
            local_78 = &local_a8;
            local_70 = core::fmt::impl$74::fmt<str_>;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if (*(ulonglong *)(lVar17 + 0x30) <= uVar23) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar20 = *(longlong *)(lVar17 + 0x28);
          lVar22 = uVar23 * 0x30;
          uVar11 = *(undefined8 *)local_90;
          uVar12 = *(undefined8 *)(local_90 + 8);
          uVar13 = *(undefined8 *)(local_90 + 0x10);
          uVar14 = *(undefined8 *)(local_90 + 0x18);
          uVar15 = *(undefined8 *)(local_90 + 0x28);
          puVar27 = (undefined8 *)(lVar20 + 0x20 + lVar22);
          *puVar27 = *(undefined8 *)(local_90 + 0x20);
          puVar27[1] = uVar15;
          puVar27 = (undefined8 *)(lVar20 + 0x10 + lVar22);
          *puVar27 = uVar13;
          puVar27[1] = uVar14;
          *(undefined8 *)(lVar20 + lVar22) = uVar11;
          ((undefined8 *)(lVar20 + lVar22))[1] = uVar12;
          uVar24 = *(ulonglong *)(lVar17 + 0x18);
          do {
            uVar18 = uVar23;
            if (uVar24 <= uVar23) {
              uVar18 = uVar24;
            }
            LOCK();
            uVar21 = *(ulonglong *)(lVar17 + 0x18);
            bVar28 = uVar24 == uVar21;
            if (bVar28) {
              *(ulonglong *)(lVar17 + 0x18) = uVar18;
              uVar21 = uVar24;
            }
            UNLOCK();
            uVar24 = uVar21;
          } while (!bVar28);
          uVar24 = *(ulonglong *)(lVar17 + 0x20);
          do {
            uVar18 = uVar23;
            if (uVar23 < uVar24) {
              uVar18 = uVar24;
            }
            LOCK();
            uVar21 = *(ulonglong *)(lVar17 + 0x20);
            bVar28 = uVar24 == uVar21;
            if (bVar28) {
              *(ulonglong *)(lVar17 + 0x20) = uVar18;
              uVar21 = uVar24;
            }
            UNLOCK();
            uVar24 = uVar21;
          } while (!bVar28);
          in_RDX[0x14] = in_RDX[0x14] + 1;
          uVar24 = uVar1;
          if (in_stack_00000030 <= uVar1) goto LAB_14156d15c;
        }
        local_88 = (undefined **)local_61;
        pcStack_80 = gpu_world::impl$17::fmt;
        local_128 = 0x42c76830;
        alloc::fmt::format::format_inner();
        local_a8 = local_180;
        pvStack_a0 = pvStack_178;
        local_98 = local_170;
        if (OwnedGpuWorldSubset::extend::ONCE != 3) {
          local_88 = (undefined **)CONCAT71(local_88._1_7_,1);
          local_128 = (int)&local_88;
          std::sys::sync::once::futex::Once::call();
        }
        std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32>_>();
        local_118 = (char)&local_88;
        if (local_128 == 1) {
          local_88 = (undefined **)0x1;
          pcStack_80 = (code *)CONCAT71(pcStack_80._1_7_,local_118);
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        local_90 = (char *)0x1;
        local_62 = local_118;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_01 == (code *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        *(undefined8 *)(extraout_RAX_01 + 7) = 0x2928202168746170;
        *(undefined8 *)extraout_RAX_01 = 0x705f656c75646f6d;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        *(undefined8 *)(extraout_RAX_01 + 0xf) = 0x3a20657461726324;
        *(undefined8 *)(extraout_RAX_01 + 0x17) = 0x206c6576654c203a;
        *(undefined8 *)(extraout_RAX_01 + 0x1e) = 0x6e726157203a3a20;
        local_78 = (undefined **)0x26;
        local_88 = (undefined **)&DAT_0000000f;
        pcStack_80 = extraout_RAX_01;
        if (0xffffffffffffffe9 < local_98) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        ppuVar16 = local_78;
        memcpy(pcStack_80 + (longlong)local_78,pvStack_a0,local_98);
        local_78 = (undefined **)((longlong)ppuVar16 + local_98);
        alloc::collections::btree::map::BTreeMap::
        insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
                  ();
        if ((extraout_AL == '\0') &&
           (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
          local_88 = &local_a8;
          pcStack_80 = alloc::string::impl$23::fmt;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          ::log::__private_api::impl$2::log();
        }
        pcVar2 = local_90;
        if (((local_62 == '\0') &&
            ((*(ulonglong *)
               __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
             0x7fffffffffffffff) != 0)) &&
           (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
          pcVar2[1] = '\x01';
        }
        LOCK();
        cVar29 = *pcVar2;
        *pcVar2 = '\0';
        UNLOCK();
        if (cVar29 == '\x02') {
          std::sys::sync::mutex::futex::Mutex::wake();
        }
        if (local_a8 != (undefined *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        uVar24 = uVar1;
      } while (uVar1 < in_stack_00000030);
    }
  }
LAB_14156d15c:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar17 = *extraout_RAX;
  if (lVar17 == 8) {
    *extraout_RAX = 0;
  }
  UNLOCK();
  if (lVar17 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  *(longlong *)(in_RCX + 0x1440) = *(longlong *)(in_RCX + 0x1440) + in_R9;
  return;
}

