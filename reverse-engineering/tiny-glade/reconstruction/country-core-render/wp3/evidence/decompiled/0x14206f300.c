// address 0x14206f300
// name nani_maintain_instances
// signature void __cdecl nani_maintain_instances(void)

void __cdecl country_core::render::nani::systems::nani_maintain_instances(void)

{
  byte *pbVar1;
  longlong *plVar2;
  char cVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  byte bVar6;
  byte bVar7;
  byte bVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  byte **ppbVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined2 extraout_AX;
  uint uVar13;
  uint uVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *plVar15;
  undefined8 *puVar16;
  longlong extraout_RAX_02;
  byte *pbVar17;
  longlong *extraout_RAX_03;
  byte **extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  undefined5 *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *in_RCX;
  byte *pbVar18;
  longlong lVar19;
  ulonglong uVar20;
  longlong *in_RDX;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong *puVar23;
  longlong *unaff_RDI;
  longlong lVar24;
  int iVar25;
  undefined8 *in_R8;
  undefined *puVar26;
  undefined *puVar27;
  ulonglong uVar28;
  undefined8 *in_R9;
  ulonglong uVar29;
  ulonglong *puVar30;
  uint *puVar31;
  ulonglong uVar32;
  uint *puVar33;
  longlong *unaff_R14;
  longlong lVar34;
  longlong *unaff_R15;
  bool bVar35;
  longlong *in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  byte *local_228;
  void *pvStack_220;
  ulonglong local_218;
  byte *local_1f8;
  longlong *local_1f0;
  longlong local_1e8;
  longlong local_1d8;
  longlong local_1d0;
  longlong local_1c8;
  undefined8 local_1c0;
  byte *local_1b8;
  void *pvStack_1b0;
  ulonglong local_1a8;
  undefined4 local_19c;
  undefined4 local_198;
  char local_194;
  undefined2 local_193;
  undefined1 local_191;
  byte *local_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  char *local_150;
  undefined8 local_148;
  undefined4 local_140;
  undefined4 local_13c;
  undefined *local_138;
  undefined8 local_130;
  byte ***local_128;
  undefined8 local_120;
  undefined8 local_118;
  longlong local_108;
  undefined4 *local_100;
  longlong *local_f8;
  longlong *local_e8;
  undefined1 *local_e0;
  byte **local_d8;
  byte **ppbStack_d0;
  byte *local_c8;
  code *local_c0;
  undefined4 local_b4;
  undefined2 local_b0;
  undefined1 local_ae;
  undefined2 local_ac;
  undefined1 local_aa;
  longlong local_a8;
  longlong *local_a0;
  undefined8 *local_98;
  byte **local_90;
  longlong *local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  local_1f0 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_1e8 = 0;
  }
  else {
    unaff_R15 = (longlong *)&DAT_142fd2ec7;
    unaff_R14 = (longlong *)&DAT_0000003c;
    uVar21 = 0x3c;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar21 != 0) {
        if (uVar21 < 0x3c) {
          if ((char)(&DAT_142fd2ec7)[uVar21] < -0x40) {
LAB_14206f392:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar21 != 0x3c) goto LAB_14206f392;
      }
      unaff_R15 = (longlong *)&DAT_142fd2ec7;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar21 != 0xfffffffffffffffe) {
          if (uVar21 + 2 < 0x3c) {
            if ((char)(&UNK_142fd2ec9)[uVar21] < -0x40) {
LAB_14206f3f0:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar21 + 2 != 0x3c) goto LAB_14206f3f0;
        }
        unaff_R14 = (longlong *)(0x3a - uVar21);
        unaff_R15 = (longlong *)(&UNK_142fd2ec9 + uVar21);
      }
    }
    puVar26 = &DAT_142fd345e;
    do {
      bVar6 = puVar26[-1];
      uVar13 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = puVar26[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar26[-3];
          if ((char)bVar8 < -0x40) {
            pbVar17 = puVar26 + -4;
            puVar26 = puVar26 + -4;
            uVar13 = bVar8 & 0x3f | (*pbVar17 & 7) << 6;
          }
          else {
            puVar26 = puVar26 + -3;
            uVar13 = bVar8 & 0xf;
          }
          uVar13 = bVar7 & 0x3f | uVar13 << 6;
        }
        else {
          puVar26 = puVar26 + -2;
          uVar13 = bVar7 & 0x1f;
        }
        uVar13 = (uint)(bVar6 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_14206f4b4;
LAB_14206f4c5:
        puVar27 = puVar26 + -0x142fd342f;
        if (puVar27 != (undefined *)0x0) {
          if (puVar27 < (undefined *)0x2e) {
            if ((char)puVar26[1] < -0x40) {
LAB_14206f4e1:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (puVar27 != (undefined *)0x2e) goto LAB_14206f4e1;
        }
        unaff_RDI = (longlong *)(0x2e - (longlong)puVar27);
        goto LAB_14206f511;
      }
      puVar26 = puVar26 + -1;
      if (uVar13 == 0x5c) goto LAB_14206f4c5;
LAB_14206f4b4:
      if (uVar13 == 0x2f) goto LAB_14206f4c5;
    } while (puVar26 != &DAT_142fd3430);
    unaff_RDI = (longlong *)0x2e;
LAB_14206f511:
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_11, extraout_RAX_11 == (longlong *)0x0)) {
LAB_1420705a4:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_1420705b0:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    local_88 = plVar15;
    puffin::ThreadProfiler::begin_scope();
    *local_88 = *local_88 + 1;
    local_1e8 = 1;
  }
  lVar34 = in_R8[2];
  if ((*(ulonglong *)*in_R8 < *(ulonglong *)(lVar34 + 0x40)) &&
     (uVar21 = *(ulonglong *)(*(longlong *)(lVar34 + 0x38) + *(ulonglong *)*in_R8 * 8), uVar21 != 0)
     ) {
    lVar34 = *(longlong *)(lVar34 + 8);
    uVar21 = ~uVar21;
    puVar30 = (ulonglong *)in_R8[1];
    uVar32 = *puVar30;
    uVar28 = 0;
    puVar23 = (ulonglong *)(lVar34 + 0x18 + uVar21 * 0x48);
    uVar20 = uVar32 - *puVar23;
    if (uVar32 < *puVar23) {
      uVar20 = uVar28;
    }
    puVar23 = (ulonglong *)(lVar34 + 0x38 + uVar21 * 0x48);
    uVar29 = uVar32 - *puVar23;
    if (uVar32 < *puVar23) {
      uVar29 = uVar28;
    }
    unaff_R14 = (longlong *)(uVar20 * 0x10 + *(longlong *)(lVar34 + 8 + uVar21 * 0x48));
    uVar32 = *(ulonglong *)(lVar34 + 0x10 + uVar21 * 0x48);
    uVar10 = *(ulonglong *)(lVar34 + 0x30 + uVar21 * 0x48);
    uVar11 = uVar32 - uVar20;
    if (uVar32 < uVar20) {
      unaff_R14 = (longlong *)&DAT_00000008;
      uVar11 = uVar28;
    }
    unaff_RDI = (longlong *)(uVar29 * 0x10 + *(longlong *)(lVar34 + 0x28 + uVar21 * 0x48));
    uVar32 = uVar10 - uVar29;
    if (uVar10 < uVar29) {
      unaff_RDI = (longlong *)&DAT_00000008;
      uVar32 = uVar28;
    }
    *puVar30 = (*(longlong *)(lVar34 + 0x40 + uVar21 * 0x48) - uVar32) - uVar11;
    local_88 = unaff_R14 + uVar11 * 2;
    local_90 = (byte **)(unaff_RDI + uVar32 * 2);
  }
  else {
    puVar30 = (ulonglong *)0x0;
  }
  local_b4 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  local_108 = *in_stack_00000028;
  local_100 = (undefined4 *)in_stack_00000028[2];
  local_a0 = (longlong *)(in_stack_00000030 + 0x20);
  puVar23 = (ulonglong *)0x0;
  local_e8 = in_RCX;
  local_98 = in_R9;
  do {
    plVar15 = in_RCX;
    if (puVar23 != (ulonglong *)0x0) goto LAB_14206f6b8;
LAB_14206f6a0:
    do {
      puVar23 = puVar30;
      puVar30 = (ulonglong *)0x0;
      unaff_R15 = unaff_RDI;
      plVar15 = unaff_R14;
      if (puVar23 == (ulonglong *)0x0) {
        local_e0 = (undefined1 *)*local_e8;
        puVar31 = *(uint **)(local_e0 + 0xf8);
        local_f8 = *(longlong **)(in_stack_00000038 + 8);
        if (local_f8 == (longlong *)0x0) {
          local_f8 = *(longlong **)(in_stack_00000038 + 0x10);
        }
        local_1d8 = local_e8[1];
        puVar33 = puVar31 + *(longlong *)(local_e0 + 0x100);
        local_e8 = (longlong *)CONCAT71(local_e8._1_7_,local_e0[0x108]);
        local_1c0 = *local_98;
        local_1d0 = 8;
        local_1c8 = 8;
        local_98 = (undefined8 *)0x0;
        local_a8 = 0;
        uVar21 = 0;
        local_88 = (longlong *)0x0;
        goto LAB_14206f845;
      }
LAB_14206f6b8:
      if ((plVar15 == (longlong *)0x0) || (in_RCX = plVar15 + 2, plVar15 == local_88)) {
        if ((unaff_R15 == (longlong *)0x0) || ((byte **)unaff_R15 == local_90)) goto LAB_14206f6a0;
        in_RCX = (longlong *)0x0;
        plVar15 = unaff_R15;
        unaff_R15 = unaff_R15 + 2;
      }
      *puVar23 = *puVar23 + 1;
    } while ((ulonglong)plVar15[1] >> 0x20 == 0);
    local_d8 = (byte **)plVar15[1];
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<bevy_ecs::entity::Entity,bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle>_>
              ();
    if ((char)local_178 == '\x1b') {
      *local_100 = local_b4;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    local_ac = local_178._1_2_;
    local_aa = local_178._3_1_;
    *local_100 = local_b4;
    local_188 = (undefined **)uStack_180;
    uStack_180 = (char *)CONCAT53(CONCAT41(uStack_180._4_4_,local_178._3_1_),(int3)local_178);
    gpu_world::GpuWorld::remove_instance();
  } while( true );
LAB_14206f845:
  if ((char)local_e8 == '\0') {
    if (local_88 == (longlong *)uVar21) {
      do {
        if (puVar31 == puVar33) goto LAB_1420700d0;
        uVar13 = *puVar31;
        puVar31 = puVar31 + 1;
        uVar21 = *(ulonglong *)(*(longlong *)(local_1d8 + 0x100) + 0x58 + (ulonglong)uVar13 * 0xa0);
      } while (uVar21 == 0);
      lVar22 = *(longlong *)(local_1d8 + 0x100) + (ulonglong)uVar13 * 0xa0;
      uVar32 = (ulonglong)*(uint *)(lVar22 + 0x94);
      lVar34 = *(longlong *)(local_1d8 + 0x1a8) + uVar32 * 0x48;
      uVar32 = *(ulonglong *)(*(longlong *)(local_1d8 + 0x1a8) + 0x40 + uVar32 * 0x48);
      if ((*(ulonglong *)(local_e0 + 0x110) < uVar32) &&
         (uVar20 = *(ulonglong *)
                    (*(longlong *)(lVar34 + 0x38) + *(ulonglong *)(local_e0 + 0x110) * 8),
         uVar20 != 0)) {
        local_98 = *(undefined8 **)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar20 * 0x30);
        uVar20 = *(ulonglong *)(local_e0 + 0x118);
        if (uVar32 <= uVar20) goto LAB_14206fa66;
LAB_14206fa14:
        uVar32 = *(ulonglong *)(*(longlong *)(lVar34 + 0x38) + uVar20 * 8);
        if (uVar32 == 0) goto LAB_14206fa66;
        local_a8 = *(longlong *)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar32 * 0x30);
      }
      else {
        local_98 = (undefined8 *)0x0;
        uVar20 = *(ulonglong *)(local_e0 + 0x118);
        if (uVar20 < uVar32) goto LAB_14206fa14;
LAB_14206fa66:
        local_a8 = 0;
      }
      local_1c8 = *(longlong *)(lVar22 + 0x50);
      local_88 = (longlong *)0x0;
    }
    puVar16 = (undefined8 *)(local_1c8 + (longlong)local_88 * 0x10);
    uVar32 = (ulonglong)*(uint *)(local_1c8 + 8 + (longlong)local_88 * 0x10);
  }
  else {
    if (local_88 == (longlong *)uVar21) {
      do {
        if (puVar31 == puVar33) goto LAB_1420700d0;
        uVar13 = *puVar31;
        puVar31 = puVar31 + 1;
        uVar21 = *(ulonglong *)(*(longlong *)(local_1d8 + 0x1a8) + 0x10 + (ulonglong)uVar13 * 0x48);
      } while (uVar21 == 0);
      lVar34 = *(longlong *)(local_1d8 + 0x1a8) + (ulonglong)uVar13 * 0x48;
      uVar32 = *(ulonglong *)(lVar34 + 0x40);
      if ((*(ulonglong *)(local_e0 + 0x110) < uVar32) &&
         (uVar20 = *(ulonglong *)
                    (*(longlong *)(lVar34 + 0x38) + *(ulonglong *)(local_e0 + 0x110) * 8),
         uVar20 != 0)) {
        local_98 = *(undefined8 **)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar20 * 0x30);
        uVar20 = *(ulonglong *)(local_e0 + 0x118);
        if (uVar32 <= uVar20) goto LAB_14206fa3e;
LAB_14206f9cc:
        uVar32 = *(ulonglong *)(*(longlong *)(lVar34 + 0x38) + uVar20 * 8);
        if (uVar32 == 0) goto LAB_14206fa3e;
        local_a8 = *(longlong *)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar32 * 0x30);
      }
      else {
        local_98 = (undefined8 *)0x0;
        uVar20 = *(ulonglong *)(local_e0 + 0x118);
        if (uVar20 < uVar32) goto LAB_14206f9cc;
LAB_14206fa3e:
        local_a8 = 0;
      }
      local_1d0 = *(longlong *)(lVar34 + 8);
      local_88 = (longlong *)0x0;
    }
    puVar16 = (undefined8 *)(local_1d0 + (longlong)local_88 * 8);
    uVar32 = (ulonglong)local_88 & 0xffffffff;
  }
  local_190 = (byte *)((longlong)local_98 + uVar32 * 0x18);
  lVar34 = uVar32 * 0x30 + local_a8;
  local_90 = (byte **)*puVar16;
  if ((*local_190 & 1) == 0) {
    pbVar17 = *(byte **)(local_190 + 8);
    local_1f8 = pbVar17;
    resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>
              ();
    lVar22 = (longlong)pbVar17 << 3;
    pbVar17 = (byte *)(extraout_RAX_02 + -8);
    do {
      if (lVar22 == 0) {
        local_d8 = &local_1f8;
        ppbStack_d0 = (byte **)::utils::name_hash::impl$7::fmt;
        local_c0 = gpu_world::impl$17::fmt;
        local_188 = &PTR_DAT_142fd3508;
        uStack_180 = (char *)0x3;
        local_168 = 0;
        local_178 = &local_d8;
        local_170 = 2;
        local_c8 = local_190 + 0x10;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      pbVar18 = pbVar17 + 8;
      lVar22 = lVar22 + -8;
      pbVar1 = pbVar17 + 0xc;
      pbVar17 = pbVar18;
    } while (*pbVar1 != local_190[0x10]);
  }
  else {
    pbVar18 = local_190 + 4;
  }
  uVar9 = *(undefined4 *)pbVar18;
  local_194 = '\x1b';
  *local_100 = local_b4;
  uVar5 = *(undefined8 *)(lVar34 + 0x10);
  uVar4 = *(undefined4 *)(lVar34 + 0x18);
  half::binary16::f16::from_f32();
  uStack_180 = (char *)CONCAT26((short)uVar9,CONCAT24(extraout_AX,uVar4));
  local_d8 = (byte **)&local_19c;
  local_188 = (undefined **)uVar5;
  gpu_world::GpuWorld::owned_subset_mut();
  local_a0 = extraout_RAX_03;
  gpu_world::OwnedGpuWorldSubset::extend_no_data();
  plVar15 = local_a0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar35 = *plVar15 == 8;
  if (bVar35) {
    *plVar15 = 0;
  }
  UNLOCK();
  if (!bVar35) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  cVar3 = local_194;
  uVar9 = local_198;
  uVar4 = local_19c;
  *(longlong *)(local_108 + 0x1440) = *(longlong *)(local_108 + 0x1440) + 1;
  local_88 = (longlong *)((longlong)local_88 + 1);
  if (local_194 == '\x1b') {
    local_d8 = &local_190;
    ppbStack_d0 = (byte **)core::fmt::impl$74::
                           fmt<country_core::components::drawable::NaniInstanceDesc>;
    local_188 = &PTR_s_Instance_limit_reached_while_try_142fd3580;
    uStack_180 = (char *)0x1;
    local_178 = &local_d8;
    local_170 = 1;
    local_168 = 0;
    alloc::fmt::format::format_inner();
    local_1b8 = local_228;
    pvStack_1b0 = pvStack_220;
    local_1a8 = local_218;
    if (DAT_14363ab28 != 3) {
      local_d8 = (byte **)CONCAT71(local_d8._1_7_,1);
      local_188 = (undefined **)&local_d8;
      std::sys::sync::once::futex::Once::call();
    }
    std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32>_>();
    if ((int)local_188 == 1) {
      local_d8 = (byte **)uStack_180;
      ppbStack_d0 = (byte **)CONCAT71(ppbStack_d0._1_7_,(char)local_178);
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_90 = (byte **)uStack_180;
    local_a0 = (longlong *)CONCAT71(local_a0._1_7_,(char)local_178);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_04 == (byte **)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined8 *)((longlong)extraout_RAX_04 + 7) = 0x2928202168746170;
    *extraout_RAX_04 = (byte *)0x705f656c75646f6d;
    local_188 = (undefined **)&DAT_0000000f;
    local_178 = (byte ***)0xf;
    uStack_180 = (char *)extraout_RAX_04;
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    builtin_strncpy((char *)((longlong)uStack_180 + (longlong)local_178),"$crate :: Level ",0x10);
    *(undefined8 *)((longlong)uStack_180 + (longlong)local_178 + 0xf) = 0x6e726157203a3a20;
    local_c8 = (byte *)((longlong)local_178 + 0x17);
    local_d8 = local_188;
    ppbStack_d0 = (byte **)uStack_180;
    if ((ulonglong)((longlong)local_188 - (longlong)local_c8) < local_1a8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    pbVar17 = local_c8;
    memcpy((undefined1 *)((longlong)ppbStack_d0 + (longlong)local_c8),pvStack_1b0,local_1a8);
    local_178 = (byte ***)((longlong)pbVar17 + local_1a8);
    local_188 = local_d8;
    uStack_180 = (char *)ppbStack_d0;
    local_c8 = (byte *)local_178;
    alloc::collections::btree::map::BTreeMap::
    insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
              ();
    if ((extraout_AL_00 == '\0') &&
       (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
      local_d8 = &local_1b8;
      ppbStack_d0 = (byte **)alloc::string::impl$23::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_168 = *extraout_RAX_05;
      uStack_160 = extraout_RAX_05[1];
      local_13c = *(undefined4 *)(extraout_RAX_05 + 2);
      local_158 = 2;
      local_150 = "country_core::render::nani::systems";
      local_148 = 0x23;
      local_138 = &DAT_142fd35d8;
      local_130 = 1;
      local_128 = &local_d8;
      local_120 = 1;
      local_118 = 0;
      local_188 = (byte **)0x0;
      uStack_180 = "country_core::render::nani::systems";
      local_178 = (byte ***)0x23;
      local_170 = 0;
      local_140 = 1;
      ::log::__private_api::impl$2::log();
    }
    ppbVar12 = local_90;
    if ((((char)local_a0 == '\0') &&
        ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
         & 0x7fffffffffffffff) != 0)) &&
       (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0')) {
      *(char *)((longlong)ppbVar12 + 1) = '\x01';
    }
    LOCK();
    cVar3 = *(char *)ppbVar12;
    *(char *)ppbVar12 = '\0';
    UNLOCK();
    if (cVar3 == '\x02') {
      std::sys::sync::mutex::futex::Mutex::wake();
    }
    if (local_1b8 != (byte *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    local_b0 = local_193;
    local_ae = local_191;
    uStack_180 = (char *)CONCAT71(uStack_180._1_7_,local_194);
    local_188 = (undefined **)CONCAT44(local_198,local_19c);
    hashbrown::map::HashMap::
    insert<bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    lVar34 = local_f8[2];
    if ((ulonglong)(*local_f8 - lVar34) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar22 = local_f8[1];
    *(code **)(lVar22 + lVar34) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniActiveInstance>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar22 + 8 + lVar34) = bevy_ecs::error::handler::panic;
    *(byte ***)(lVar22 + 0x10 + lVar34) = local_90;
    *(undefined4 *)(lVar22 + 0x18 + lVar34) = uVar4;
    *(undefined4 *)(lVar22 + 0x1c + lVar34) = uVar9;
    *(char *)(lVar22 + 0x20 + lVar34) = cVar3;
    *(undefined2 *)(lVar22 + 0x21 + lVar34) = local_b0;
    *(undefined1 *)(lVar22 + 0x23 + lVar34) = local_ae;
    *(undefined1 *)(lVar22 + 0x24 + lVar34) = 0;
    local_f8[2] = lVar34 + 0x28;
  }
  goto LAB_14206f845;
LAB_1420700d0:
  lVar34 = *local_1f0;
  lVar22 = local_1f0[1];
  iVar25 = *(int *)((longlong)local_1f0 + 0x14);
  puVar33 = *(uint **)(lVar34 + 0xf8);
  puVar31 = puVar33 + *(longlong *)(lVar34 + 0x100);
  local_98 = (undefined8 *)CONCAT71(local_98._1_7_,*(undefined1 *)(lVar34 + 0x108));
  uVar14 = iVar25 - (int)local_1f0[2];
  uVar13 = 0xc233b000;
  if (uVar14 < 0xc233b000) {
    uVar13 = uVar14;
  }
  local_88 = (longlong *)0x0;
  local_90 = (byte **)0x0;
  uVar21 = 0;
  uVar32 = 0;
  local_a0 = (longlong *)CONCAT44(local_a0._4_4_,iVar25);
  lVar24 = 0;
  plVar15 = (longlong *)8;
  goto LAB_14207013c;
LAB_142070440:
  plVar2 = local_88 + uVar20 * 3;
  local_e8 = plVar15;
  local_a8 = lVar24;
  if ((int)*plVar2 != 1) {
    lVar24 = plVar2[1];
    local_f8 = plVar2;
    local_e0 = (undefined1 *)((longlong)local_90 + uVar20 * 0xc);
    resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>
              ();
    if (lVar24 != 1) {
      local_188 = (undefined **)0x0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<usize,usize>();
    }
    local_d8 = (byte **)CONCAT35(local_d8._5_3_,*extraout_RAX_08);
    if (*(char *)((longlong)extraout_RAX_08 + 4) != (char)local_f8[2]) {
      local_188 = (undefined **)0x0;
                    // WARNING: Subroutine does not return
      core::panicking::
      assert_failed<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::render::nani::gpu_world::GpuWorldSubsetKind>
                ();
    }
  }
  uVar32 = uVar32 + 1;
  *local_100 = local_b4;
  gpu_world::GpuWorld::update_instance_mesh();
  lVar24 = local_a8;
  plVar15 = local_e8;
  iVar25 = (int)local_a0;
LAB_14207013c:
  if ((char)local_98 == '\0') {
    do {
      if (uVar32 == uVar21) {
        if (puVar33 == puVar31) goto LAB_142070501;
        uVar14 = *puVar33;
        puVar33 = puVar33 + 1;
        if (*(longlong *)(*(longlong *)(lVar22 + 0x100) + 0x58 + (ulonglong)uVar14 * 0xa0) == 0) {
          do {
            if (puVar33 == puVar31) goto LAB_142070501;
            uVar14 = *puVar33;
            puVar33 = puVar33 + 1;
          } while (*(longlong *)(*(longlong *)(lVar22 + 0x100) + 0x58 + (ulonglong)uVar14 * 0xa0) ==
                   0);
          lVar24 = *(longlong *)(lVar22 + 0x100) + (ulonglong)uVar14 * 0xa0;
        }
        else {
          lVar24 = *(longlong *)(lVar22 + 0x100) + (ulonglong)uVar14 * 0xa0;
        }
        lVar19 = *(longlong *)(lVar22 + 0x1a8) + (ulonglong)*(uint *)(lVar24 + 0x94) * 0x48;
        uVar21 = *(ulonglong *)
                  (*(longlong *)(lVar22 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar24 + 0x94) * 0x48
                  );
        if ((*(ulonglong *)(lVar34 + 0x110) < uVar21) &&
           (uVar32 = *(ulonglong *)
                      (*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lVar34 + 0x110) * 8),
           uVar32 != 0)) {
          local_88 = *(longlong **)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar32 * 0x30);
          uVar32 = *(ulonglong *)(lVar34 + 0x118);
          if (uVar21 <= uVar32) goto LAB_1420703f0;
LAB_1420703b7:
          uVar21 = *(ulonglong *)(*(longlong *)(lVar19 + 0x38) + uVar32 * 8);
          if (uVar21 == 0) goto LAB_1420703f0;
          local_90 = *(byte ***)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar21 * 0x30);
        }
        else {
          local_88 = (longlong *)0x0;
          uVar32 = *(ulonglong *)(lVar34 + 0x118);
          if (uVar32 < uVar21) goto LAB_1420703b7;
LAB_1420703f0:
          local_90 = (byte **)0x0;
        }
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        plVar15 = *(longlong **)(lVar24 + 0x50);
        uVar21 = *(ulonglong *)(lVar24 + 0x58);
        uVar32 = 0;
        lVar24 = extraout_RAX_07;
        iVar25 = (int)local_a0;
      }
      uVar20 = (ulonglong)*(uint *)((longlong)plVar15 + 8 + uVar32 * 0x10);
      if ((uint)(iVar25 - *(int *)(lVar24 + uVar20 * 4)) < uVar13) goto LAB_142070440;
      uVar32 = uVar32 + 1;
    } while( true );
  }
  local_e8 = plVar15;
  if (uVar32 != uVar21) goto LAB_1420702ab;
  while (puVar33 != puVar31) {
    uVar21 = (ulonglong)*puVar33;
    puVar33 = puVar33 + 1;
    lVar24 = *(longlong *)(lVar22 + 0x1a8);
    if (*(longlong *)(lVar24 + 0x10 + uVar21 * 0x48) == 0) {
      do {
        if (puVar33 == puVar31) goto LAB_142070501;
        uVar14 = *puVar33;
        puVar33 = puVar33 + 1;
      } while (*(longlong *)(*(longlong *)(lVar22 + 0x1a8) + 0x10 + (ulonglong)uVar14 * 0x48) == 0);
      lVar19 = *(longlong *)(lVar22 + 0x1a8) + (ulonglong)uVar14 * 0x48;
      puVar30 = (ulonglong *)(lVar19 + 0x10);
      uVar21 = *(ulonglong *)(lVar34 + 0x110);
      uVar32 = *(ulonglong *)(lVar19 + 0x40);
      if (uVar21 < uVar32) goto LAB_1420701ec;
LAB_142070230:
      local_88 = (longlong *)0x0;
      uVar21 = *(ulonglong *)(lVar34 + 0x118);
      if (uVar32 <= uVar21) goto LAB_142070280;
LAB_142070247:
      uVar21 = *(ulonglong *)(*(longlong *)(lVar19 + 0x38) + uVar21 * 8);
      if (uVar21 == 0) goto LAB_142070280;
      local_90 = *(byte ***)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar21 * 0x30);
    }
    else {
      lVar19 = lVar24 + uVar21 * 0x48;
      puVar30 = (ulonglong *)(lVar24 + uVar21 * 0x48 + 0x10);
      uVar21 = *(ulonglong *)(lVar34 + 0x110);
      uVar32 = *(ulonglong *)(lVar19 + 0x40);
      if (uVar32 <= uVar21) goto LAB_142070230;
LAB_1420701ec:
      uVar21 = *(ulonglong *)(*(longlong *)(lVar19 + 0x38) + uVar21 * 8);
      if (uVar21 == 0) goto LAB_142070230;
      local_88 = *(longlong **)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar21 * 0x30);
      uVar21 = *(ulonglong *)(lVar34 + 0x118);
      if (uVar21 < uVar32) goto LAB_142070247;
LAB_142070280:
      local_90 = (byte **)0x0;
    }
    bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
    uVar21 = *puVar30;
    uVar32 = 0;
    lVar24 = extraout_RAX_06;
    plVar15 = local_e8;
    iVar25 = (int)local_a0;
LAB_1420702ab:
    do {
      uVar20 = uVar32 & 0xffffffff;
      if ((uint)(iVar25 - *(int *)(lVar24 + uVar20 * 4)) < uVar13) goto LAB_142070440;
      uVar32 = uVar32 + 1;
    } while (uVar32 != uVar21);
  }
LAB_142070501:
  if (local_1e8 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_09 == 1) {
      plVar15 = extraout_RAX_09 + 1;
    }
    else if ((*extraout_RAX_09 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0))
    goto LAB_1420705a4;
    if (*plVar15 != 0) goto LAB_1420705b0;
    *plVar15 = -1;
    local_88 = plVar15;
    puffin::ThreadProfiler::end_scope();
    *local_88 = *local_88 + 1;
  }
  return;
}

