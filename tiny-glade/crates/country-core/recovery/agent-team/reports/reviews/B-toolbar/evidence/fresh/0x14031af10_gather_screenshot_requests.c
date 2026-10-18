
void __cdecl tiny_glade::screenshot::gather_screenshot_requests(void)

{
  char *pcVar1;
  longlong *plVar2;
  uint uVar3;
  uint uVar4;
  undefined4 uVar5;
  longlong lVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  int *piVar9;
  undefined4 *puVar10;
  code *pcVar11;
  ulonglong uVar12;
  bool bVar13;
  byte extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte bVar14;
  byte extraout_AL_02;
  byte bVar15;
  ulonglong uVar16;
  undefined8 *extraout_RAX;
  longlong *extraout_RAX_00;
  void *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  ulonglong uVar17;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *plVar18;
  int iVar19;
  longlong *in_RCX;
  longlong lVar20;
  longlong lVar21;
  undefined8 *in_RDX;
  longlong lVar22;
  ulonglong uVar23;
  undefined8 *puVar24;
  undefined **ppuVar25;
  int iVar26;
  undefined8 *in_R8;
  longlong lVar27;
  longlong lVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  longlong lVar31;
  void *pvVar32;
  bool bVar33;
  bool bVar34;
  bool bVar35;
  bool bVar36;
  bool bVar37;
  float fVar38;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  char *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong *in_stack_00000058;
  longlong in_stack_00000060;
  longlong *in_stack_00000068;
  undefined8 *in_stack_00000070;
  undefined8 *in_stack_00000078;
  undefined8 *local_1e0;
  undefined **ppuStack_1d8;
  undefined8 *local_1d0;
  undefined **ppuStack_1c8;
  longlong *local_1c0;
  longlong *local_1b8;
  undefined *local_1a8;
  undefined8 local_1a0;
  longlong local_198 [3];
  undefined8 *local_180;
  longlong local_178;
  longlong *local_170;
  ulonglong *local_150;
  longlong *local_148;
  undefined8 local_140;
  code *pcStack_138;
  undefined **local_130;
  code *pcStack_128;
  undefined **local_120;
  undefined **ppuStack_118;
  undefined **local_110;
  undefined1 uStack_108;
  undefined5 uStack_107;
  undefined2 uStack_102;
  undefined6 uStack_100;
  undefined2 uStack_fa;
  undefined1 uStack_f8;
  undefined5 uStack_f7;
  undefined2 uStack_f2;
  undefined8 uStack_f0;
  undefined1 local_e8;
  undefined7 uStack_e7;
  undefined4 local_e0;
  int iStack_dc;
  int iStack_d8;
  char cStack_d4;
  undefined1 uStack_d3;
  undefined1 uStack_d2;
  undefined1 uStack_d1;
  undefined4 local_d0;
  undefined1 local_8a;
  longlong *local_88;
  undefined8 *local_80;
  ulonglong local_78;
  undefined **local_70;
  ulonglong local_68;
  undefined1 local_5a;
  longlong *local_58;
  undefined1 local_4f;
  undefined1 local_4c;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_150 = (ulonglong *)in_stack_00000070[5];
  lVar6 = in_stack_00000070[6];
  puVar7 = (ulonglong *)in_stack_00000070[10];
  lVar21 = in_stack_00000070[0xb];
  local_78 = 0;
  local_70 = (undefined **)0x8;
  local_68 = 0;
  plVar18 = (longlong *)*in_stack_00000028;
  local_180 = in_R8;
  local_88 = in_RCX;
  local_80 = in_RDX;
  if (plVar18 != (longlong *)0x0) {
    lVar28 = *plVar18;
    local_58 = (longlong *)(lVar28 + 0x10);
    LOCK();
    bVar34 = *(longlong *)(lVar28 + 0x10) == 0;
    if (bVar34) {
      *(longlong *)(lVar28 + 0x10) = 8;
    }
    UNLOCK();
    if (!bVar34) {
      local_49 = 1;
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    lVar20 = *(longlong *)(lVar28 + 0x28);
    if (lVar20 != 0) {
      lVar31 = *(longlong *)(lVar28 + 0x20);
      lVar27 = lVar20 * 8;
      lVar22 = 0;
      do {
        if (*(longlong *)(lVar31 + lVar22 * 8) == 0) {
          local_120 = *(undefined ***)(lVar31 + lVar22 * 8);
          *(undefined8 *)(lVar31 + lVar22 * 8) = *(undefined8 *)(lVar31 + -8 + lVar20 * 8);
          *(longlong *)(lVar28 + 0x28) = lVar20 + -1;
          if (local_120 != (undefined **)0x0) {
            LOCK();
            *local_120 = *local_120 + -1;
            UNLOCK();
            if (*local_120 == (undefined *)0x0) {
              alloc::sync::Arc::
              drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<enum2_<core::result::Result<tuple_<std::path::PathBuf,array_<u32,2>_>,alloc::boxed::Box<dyn_<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>_>_>_>_>_>,alloc::alloc::Global>
                        ();
            }
          }
          local_49 = 1;
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          local_49 = 1;
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          lVar21 = *local_58;
          if (lVar21 == 8) {
            *local_58 = 0;
          }
          UNLOCK();
          bVar14 = 1;
          if (lVar21 != 8) {
            local_49 = 1;
            parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
          }
          goto LAB_14031b1cf;
        }
        lVar22 = lVar22 + 1;
        lVar27 = lVar27 + -8;
      } while (lVar27 != 0);
    }
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    lVar28 = *local_58;
    if (lVar28 == 8) {
      *local_58 = 0;
    }
    UNLOCK();
    if (lVar28 != 8) {
      local_49 = 1;
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
  }
  uVar16 = *puVar7;
  uVar17 = 0;
  uVar30 = uVar16 - *(ulonglong *)(lVar21 + 0x18);
  if (uVar16 < *(ulonglong *)(lVar21 + 0x18)) {
    uVar30 = uVar17;
  }
  uVar29 = uVar16 - *(ulonglong *)(lVar21 + 0x38);
  if (uVar16 < *(ulonglong *)(lVar21 + 0x38)) {
    uVar29 = uVar17;
  }
  lVar28 = uVar30 * 0x10 + *(longlong *)(lVar21 + 8);
  uVar16 = *(ulonglong *)(lVar21 + 0x10);
  lVar20 = uVar29 * 0x10 + *(longlong *)(lVar21 + 0x28);
  uVar23 = uVar16 - uVar30;
  if (uVar16 < uVar30) {
    uVar23 = uVar17;
  }
  uVar8 = *(ulonglong *)(lVar21 + 0x30);
  if (uVar16 < uVar30) {
    lVar28 = 8;
    uVar30 = uVar16;
  }
  uVar12 = uVar8 - uVar29;
  if (uVar8 < uVar29) {
    lVar20 = 8;
    uVar12 = uVar17;
  }
  lVar21 = *(longlong *)(lVar21 + 0x40);
  if (uVar8 < uVar29) {
    uVar29 = uVar8;
  }
  *puVar7 = (lVar21 - uVar12) - uVar23;
  uVar16 = ((uVar29 + lVar21 + uVar30) - uVar8) - uVar16;
  lVar21 = lVar20;
  lVar27 = lVar28;
  do {
    uVar16 = uVar16 + 1;
    local_58 = plVar18;
    if ((lVar27 == 0) || (lVar27 == uVar23 * 0x10 + lVar28)) {
      if ((lVar21 == 0) || (lVar21 == uVar12 * 0x10 + lVar20)) {
        local_49 = 1;
        country_core::input::action_set::DigitalActionState::just_pressed();
        bVar14 = extraout_AL;
        goto LAB_14031b1cf;
      }
      lVar31 = 0;
      lVar22 = lVar21 + 0x10;
      lVar27 = lVar21;
    }
    else {
      lVar31 = lVar27 + 0x10;
      lVar22 = lVar21;
    }
    *puVar7 = uVar16;
    pcVar1 = (char *)(lVar27 + 8);
    lVar21 = lVar22;
    lVar27 = lVar31;
  } while (*pcVar1 != '\x04');
  bVar14 = 1;
LAB_14031b1cf:
  if (plVar18 != (longlong *)0x0) {
    lVar21 = *plVar18;
    plVar2 = (longlong *)(lVar21 + 0x10);
    LOCK();
    bVar34 = *plVar2 == 0;
    if (bVar34) {
      *plVar2 = 8;
    }
    UNLOCK();
    if (!bVar34) {
      local_49 = 1;
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    local_58 = (longlong *)(lVar21 + 0x10);
    alloc::vec::Vec::
    retain_mut<enum2_<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env_0>
              ();
    plVar2 = local_58;
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    local_49 = 1;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar34 = *plVar2 == 8;
    if (bVar34) {
      *plVar2 = 0;
    }
    UNLOCK();
    if (!bVar34) {
      local_49 = 1;
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
  }
  piVar9 = (int *)*in_stack_00000030;
  if (*in_stack_00000040 == '\0') {
    if ((bVar14 & *piVar9 == 0x13) == 0) {
LAB_14031b30a:
      if (bVar14 != 0) goto LAB_14031b313;
    }
    else {
      *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
      *(undefined1 *)(*in_stack_00000068 + 0x31) = 1;
      in_stack_00000040[0] = '\x01';
      in_stack_00000040[1] = '\x01';
    }
LAB_14031b582:
    local_49 = 1;
    country_core::input::action_set::DigitalActionState::just_pressed();
    if (extraout_AL_00 != '\0') {
      local_49 = 1;
      std::path::Path::to_path_buf();
      uVar16 = local_68;
      uStack_f8 = SUB81(local_130,0);
      uStack_f7 = (undefined5)((ulonglong)local_130 >> 8);
      uStack_f2 = (undefined2)((ulonglong)local_130 >> 0x30);
      uStack_f0 = pcStack_128;
      uStack_108 = SUB81(local_140,0);
      uStack_107 = (undefined5)((ulonglong)local_140 >> 8);
      uStack_102 = (undefined2)((ulonglong)local_140 >> 0x30);
      uStack_100 = SUB86(pcStack_138,0);
      uStack_fa = (undefined2)((ulonglong)pcStack_138 >> 0x30);
      cStack_d4 = '\0';
      local_e0 = 0;
      local_e8 = 100;
      local_120 = (undefined **)0x0;
      if (local_68 == local_78) {
        alloc::raw_vec::RawVec::
        grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
      }
      local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
      (local_70 + uVar16 * 10 + 8)[1] =
           (undefined *)
           CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))))
      ;
      local_70[uVar16 * 10 + 6] = uStack_f0;
      (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
      local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
      (local_70 + uVar16 * 10 + 4)[1] =
           (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8));
      local_70[uVar16 * 10 + 2] = (undefined *)local_110;
      (local_70 + uVar16 * 10 + 2)[1] =
           (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
      local_70[uVar16 * 10] = (undefined *)local_120;
      (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
      local_68 = uVar16 + 1;
    }
  }
  else {
    bVar15 = in_stack_00000040[1];
    if (bVar15 < 0x10) {
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      *(undefined1 *)(*in_stack_00000048 + 0x20) = 1;
      in_stack_00000040[1] = bVar15 + 1;
      goto LAB_14031b582;
    }
    if (bVar15 != 0x10) {
      *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
      *(undefined1 *)(*in_stack_00000068 + 0x31) = 0;
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      *(undefined1 *)(*in_stack_00000048 + 0x20) = 0;
      *in_stack_00000040 = '\0';
      goto LAB_14031b30a;
    }
    in_stack_00000040[1] = '\x11';
LAB_14031b313:
    local_178 = *in_stack_00000038;
    local_1a8 = &DAT_1429bc798;
    if (*(char *)(local_178 + 0x120) != '\0') {
      local_1a8 = &DAT_1429bc79b;
    }
    local_1a0 = 3;
    local_49 = 1;
    paths::config();
    local_49 = 1;
    puVar24 = extraout_RAX;
    std::path::Path::_join();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    ::utils::file::date_time_string_for_filename();
    pcStack_138 = alloc::string::impl$23::fmt;
    local_130 = &local_1a8;
    pcStack_128 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_120 = (undefined **)&DAT_1429bc7b0;
    ppuStack_118 = (undefined **)0x2;
    local_110 = (undefined **)&local_140;
    uStack_108 = 2;
    uStack_107 = 0;
    uStack_102 = 0;
    uStack_100 = 0;
    uStack_fa = 0;
    local_140 = local_198;
    alloc::fmt::format::format_inner();
    if (local_198[0] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_58 = local_1c0;
    local_148 = local_1b8;
    std::path::Path::_join();
    if (local_58 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_170 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (plVar18 == (longlong *)0x0) {
      ppuVar25 = (undefined **)0x0;
    }
    else {
      lVar21 = *plVar18;
      plVar18 = (longlong *)(lVar21 + 0x10);
      LOCK();
      bVar34 = *(longlong *)(lVar21 + 0x10) == 0;
      if (bVar34) {
        *(longlong *)(lVar21 + 0x10) = 8;
      }
      UNLOCK();
      if (!bVar34) {
        local_4a = 1;
        parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
      }
      local_4a = 1;
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      local_4a = 1;
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_148 = plVar18;
      if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_00 = 1;
      extraout_RAX_00[1] = 1;
      *(undefined1 *)(extraout_RAX_00 + 2) = 0;
      extraout_RAX_00[3] = -0x7fffffffffffffff;
      LOCK();
      lVar28 = *extraout_RAX_00;
      *extraout_RAX_00 = *extraout_RAX_00 + 1;
      UNLOCK();
      if (*extraout_RAX_00 == 0 || SCARRY8(lVar28,1) != *extraout_RAX_00 < 0) goto LAB_14031c67d;
      lVar28 = *(longlong *)(lVar21 + 0x28);
      local_58 = extraout_RAX_00;
      if (lVar28 == *(longlong *)(lVar21 + 0x18)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      ppuVar25 = local_130;
      pcVar11 = pcStack_138;
      *(longlong **)(*(longlong *)(lVar21 + 0x20) + lVar28 * 8) = local_58;
      *(longlong *)(lVar21 + 0x28) = lVar28 + 1;
      if ((longlong)local_130 < 0) {
LAB_14031c653:
        local_4c = 1;
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (local_130 == (undefined **)0x0) {
        pvVar32 = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        pvVar32 = extraout_RAX_01;
        if (extraout_RAX_01 == (void *)0x0) goto LAB_14031c653;
      }
      memcpy(pvVar32,pcVar11,(size_t)ppuVar25);
      uStack_100 = SUB86(local_58,0);
      uStack_fa = (undefined2)((ulonglong)local_58 >> 0x30);
      local_120 = ppuVar25;
      local_110 = ppuVar25;
      uStack_108 = pcStack_128._0_1_;
      ppuStack_118 = pvVar32;
      std::alloc::__default_lib_allocator::__rust_alloc();
      plVar18 = local_148;
      if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_02[4] = CONCAT26(uStack_fa,uStack_100);
      extraout_RAX_02[2] = local_110;
      extraout_RAX_02[3] = CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
      *extraout_RAX_02 = local_120;
      extraout_RAX_02[1] = ppuStack_118;
      local_4a = 1;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      local_4a = 1;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar34 = *plVar18 == 8;
      if (bVar34) {
        *plVar18 = 0;
      }
      UNLOCK();
      ppuVar25 = (undefined **)0x1;
      puVar24 = extraout_RAX_02;
      if (!bVar34) {
        local_4a = 1;
        parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
      }
    }
    uVar16 = local_68;
    cStack_d4 = '\x04' - (*piVar9 == 0x13);
    uStack_f8 = SUB81(local_130,0);
    uStack_f7 = (undefined5)((ulonglong)local_130 >> 8);
    uStack_f2 = (undefined2)((ulonglong)local_130 >> 0x30);
    uStack_f0 = pcStack_128;
    uStack_108 = SUB81(local_140,0);
    uStack_107 = (undefined5)((ulonglong)local_140 >> 8);
    uStack_102 = (undefined2)((ulonglong)local_140 >> 0x30);
    uStack_100 = SUB86(pcStack_138,0);
    uStack_fa = (undefined2)((ulonglong)pcStack_138 >> 0x30);
    local_e8 = 0x55;
    if (*(char *)(local_178 + 0x120) != '\0') {
      local_e8 = 100;
    }
    local_e0 = 0;
    local_110 = &anon_ed789b1e624f90e6ebdcc559d19c73c2_43_llvm_4495920109268975582;
    local_120 = ppuVar25;
    ppuStack_118 = (undefined **)puVar24;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
  }
  uVar16 = *local_150;
  uVar17 = 0;
  uVar30 = uVar16 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar16 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar30 = uVar17;
  }
  uVar29 = uVar16 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar16 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar29 = uVar17;
  }
  bVar34 = uVar29 <= *(ulonglong *)(lVar6 + 0x30);
  uVar29 = *(ulonglong *)(lVar6 + 0x30) - uVar29;
  bVar35 = uVar29 != 0;
  if (!bVar34) {
    uVar29 = uVar17;
  }
  bVar33 = uVar30 <= *(ulonglong *)(lVar6 + 0x10);
  uVar30 = *(ulonglong *)(lVar6 + 0x10) - uVar30;
  bVar36 = uVar30 != 0;
  if (!bVar33) {
    uVar30 = uVar17;
  }
  uVar16 = uVar30 + uVar29;
  if (bVar33 && bVar36 || bVar34 && bVar35) {
    uVar16 = uVar17;
  }
  *local_150 = *(longlong *)(lVar6 + 0x40) - uVar16;
  local_49 = 1;
  country_core::input::action_set::DigitalActionState::just_pressed();
  if (extraout_AL_01 == '\0') {
    bVar37 = false;
    bVar13 = false;
    if (*(int *)*in_stack_00000078 != 8) goto LAB_14031b8a3;
LAB_14031b8b2:
    bVar37 = bVar13;
    local_49 = 1;
    country_core::resources::gui::EscMenuState::is_closed();
    bVar14 = extraout_AL_02;
  }
  else {
    bVar37 = *(int *)*local_80 != 8;
    bVar13 = bVar37;
    if (*(int *)*in_stack_00000078 == 8) goto LAB_14031b8b2;
LAB_14031b8a3:
    bVar14 = 0;
  }
  if ((byte)(bVar37 | bVar14 | (bVar33 && bVar36 || bVar34 && bVar35)) == 1) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    uVar16 = local_68;
    if (extraout_RAX_03 == (longlong *)0x0) {
      local_49 = 1;
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_03 = 1;
    extraout_RAX_03[1] = 1;
    *(undefined1 *)(extraout_RAX_03 + 2) = 0;
    extraout_RAX_03[3] = -0x8000000000000000;
    LOCK();
    lVar6 = *extraout_RAX_03;
    *extraout_RAX_03 = *extraout_RAX_03 + 1;
    UNLOCK();
    if (*extraout_RAX_03 == 0 || SCARRY8(lVar6,1) != *extraout_RAX_03 < 0) goto LAB_14031c67d;
    cStack_d4 = '\x06';
    local_e0 = 0;
    uStack_108 = 0;
    uStack_107 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB86(extraout_RAX_03,0);
    uStack_fa = (undefined2)((ulonglong)extraout_RAX_03 >> 0x30);
    uStack_f8 = 0x3c;
    local_120 = (undefined **)0x0;
    ppuStack_118 = (undefined **)local_1d0;
    local_110 = ppuStack_1c8;
    local_170 = extraout_RAX_03;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
    local_120 = (undefined **)0x1;
    ppuStack_118 = (undefined **)0x1;
    local_110 = (undefined **)((ulonglong)local_110 & 0xffffffffffffff00);
    uStack_d2 = 7;
    std::alloc::__default_lib_allocator::__rust_alloc();
    uVar16 = local_68;
    if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    extraout_RAX_04[8] = CONCAT44(iStack_dc,local_e0);
    extraout_RAX_04[9] =
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    extraout_RAX_04[6] = (longlong)uStack_f0;
    extraout_RAX_04[7] = CONCAT71(uStack_e7,local_e8);
    extraout_RAX_04[4] = CONCAT26(uStack_fa,uStack_100);
    extraout_RAX_04[5] = CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8));
    extraout_RAX_04[2] = (longlong)local_110;
    extraout_RAX_04[3] = CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    *extraout_RAX_04 = (longlong)local_120;
    extraout_RAX_04[1] = (longlong)ppuStack_118;
    uVar3 = *(uint *)*in_stack_00000048;
    uVar4 = ((uint *)*in_stack_00000048)[1];
    LOCK();
    lVar6 = *extraout_RAX_04;
    *extraout_RAX_04 = *extraout_RAX_04 + 1;
    UNLOCK();
    local_140 = extraout_RAX_04;
    if (*extraout_RAX_04 == 0 || SCARRY8(lVar6,1) != *extraout_RAX_04 < 0) goto LAB_14031c67d;
    fVar38 = (float)uVar4 * 0.1;
    iStack_d8 = (int)(longlong)fVar38;
    if (fVar38 < 0.0) {
      iStack_d8 = 0;
    }
    if (4.294967e+09 < fVar38) {
      iStack_d8 = -1;
    }
    iStack_d8 = iStack_d8 + (uint)(iStack_d8 == 0);
    fVar38 = (float)uVar3 * 0.1;
    iStack_dc = (int)(longlong)fVar38;
    if (fVar38 < 0.0) {
      iStack_dc = 0;
    }
    if (4.294967e+09 < fVar38) {
      iStack_dc = -1;
    }
    iStack_dc = iStack_dc + (uint)(iStack_dc == 0);
    cStack_d4 = '\x06';
    local_e0 = 1;
    uStack_108 = 1;
    uStack_107 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB86(extraout_RAX_04,0);
    uStack_fa = (undefined2)((ulonglong)extraout_RAX_04 >> 0x30);
    local_120 = (undefined **)0x0;
    ppuStack_118 = (undefined **)local_1d0;
    local_110 = ppuStack_1c8;
    local_88 = extraout_RAX_04;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
    uVar5 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
    puVar10 = (undefined4 *)in_stack_00000050[2];
    *puVar10 = uVar5;
    local_58 = (longlong *)*in_stack_00000050;
    bVar15 = *(char *)((longlong)local_58 + 0x3e) - 7;
    bVar14 = 2;
    if (bVar15 < 2) {
      bVar14 = bVar15;
    }
    if (bVar14 != 0) {
      if (bVar14 == 1) {
        plVar18 = (longlong *)local_58[1];
        LOCK();
        *plVar18 = *plVar18 + -1;
        UNLOCK();
        if (*plVar18 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<rhapsody::raw::texture::Texture>_>_>,alloc::alloc::Global>
                    ();
        }
      }
      else {
        local_150 = (ulonglong *)(local_58 + 1);
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        rhapsody::raw::managed::handle::impl$6::drop();
      }
    }
    local_58[1] = (longlong)local_88;
    local_58[2] = (longlong)local_120;
    local_58[3] = (longlong)ppuStack_118;
    local_58[4] = (longlong)local_110;
    local_58[5] = CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    *(ulonglong *)((longlong)local_58 + 0x2e) = CONCAT62(uStack_100,uStack_102);
    *(ulonglong *)((longlong)local_58 + 0x36) = CONCAT53(uStack_f7,CONCAT12(uStack_f8,uStack_fa));
    *(undefined1 *)((longlong)local_58 + 0x3e) = 8;
    local_88 = local_170;
    *puVar10 = uVar5;
    plVar18 = (longlong *)*local_58;
    if (plVar18 != (longlong *)0x0) {
      LOCK();
      *plVar18 = *plVar18 + -1;
      UNLOCK();
      if (*plVar18 == 0) {
        alloc::sync::Arc::
        drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>,alloc::alloc::Global>
                  ();
      }
    }
    *local_58 = (longlong)local_88;
  }
  if ((*(int *)*local_80 == 10) && ((*(byte *)((int *)*local_80 + 2) & 1) == 0)) {
    *(undefined4 *)in_stack_00000078[2] = *(undefined4 *)((longlong)in_stack_00000078 + 0x1c);
    ppuStack_118 = (undefined **)CONCAT71(ppuStack_118._1_7_,1);
    local_120 = (undefined **)0xa;
    local_49 = 1;
    country_core::resources::gui::NextEscMenuState::set();
    fVar38 = (float)*(uint *)(*in_stack_00000048 + 4) * 0.4;
    iVar19 = 0;
    if (0.0 <= fVar38) {
      iVar19 = (int)(longlong)fVar38;
    }
    iVar26 = -1;
    if (fVar38 <= 4.294967e+09) {
      iVar26 = iVar19;
    }
    iVar26 = iVar26 + (uint)(iVar26 == 0);
    local_120 = (undefined **)0x1;
    ppuStack_118 = (undefined **)0x1;
    local_110 = (undefined **)((ulonglong)local_110 & 0xffffffffffffff00);
    uStack_d2 = 7;
    std::alloc::__default_lib_allocator::__rust_alloc();
    uVar16 = local_68;
    if (extraout_RAX_05 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    extraout_RAX_05[8] = CONCAT44(iStack_dc,local_e0);
    extraout_RAX_05[9] =
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    extraout_RAX_05[6] = (longlong)uStack_f0;
    extraout_RAX_05[7] = CONCAT71(uStack_e7,local_e8);
    extraout_RAX_05[4] = CONCAT26(uStack_fa,uStack_100);
    extraout_RAX_05[5] = CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8));
    extraout_RAX_05[2] = (longlong)local_110;
    extraout_RAX_05[3] = CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    *extraout_RAX_05 = (longlong)local_120;
    extraout_RAX_05[1] = (longlong)ppuStack_118;
    LOCK();
    lVar6 = *extraout_RAX_05;
    *extraout_RAX_05 = *extraout_RAX_05 + 1;
    UNLOCK();
    local_140 = extraout_RAX_05;
    if (*extraout_RAX_05 == 0 || SCARRY8(lVar6,1) != *extraout_RAX_05 < 0) goto LAB_14031c67d;
    cStack_d4 = '\x01';
    local_e0 = 1;
    uStack_108 = 1;
    uStack_107 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB86(extraout_RAX_05,0);
    uStack_fa = (undefined2)((ulonglong)extraout_RAX_05 >> 0x30);
    local_120 = (undefined **)0x0;
    iStack_dc = iVar26;
    iStack_d8 = iVar26;
    local_58 = extraout_RAX_05;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
    *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
    plVar18 = *(longlong **)(*in_stack_00000058 + 8);
    local_80 = (undefined8 *)(*in_stack_00000058 + 8);
    if (plVar18 != (longlong *)0x0) {
      LOCK();
      *plVar18 = *plVar18 + -1;
      UNLOCK();
      if (*plVar18 == 0) {
        alloc::sync::Arc::
        drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<rhapsody::raw::texture::Texture>_>_>,alloc::alloc::Global>
                  ();
      }
    }
    *local_80 = local_58;
  }
  uVar16 = local_68;
  if (bVar33 && bVar36 || bVar34 && bVar35) {
    plVar18 = *(longlong **)*local_180;
    LOCK();
    lVar6 = *plVar18;
    *plVar18 = *plVar18 + 1;
    UNLOCK();
    if (*plVar18 == 0 || SCARRY8(lVar6,1) != *plVar18 < 0) {
LAB_14031c67d:
                    // WARNING: Does not return
      pcVar11 = (code *)invalidInstructionException();
      (*pcVar11)();
    }
    cStack_d4 = '\x02';
    local_e0 = 0;
    uStack_108 = 1;
    uStack_107 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB86(plVar18,0);
    uStack_fa = (undefined2)((ulonglong)plVar18 >> 0x30);
    local_120 = (undefined **)0x0;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
  }
  lVar6 = in_stack_00000070[1];
  uVar16 = *(ulonglong *)*in_stack_00000070;
  uVar17 = 0;
  uVar30 = uVar16 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar16 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar30 = uVar17;
  }
  uVar29 = uVar16 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar16 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar29 = uVar17;
  }
  bVar34 = *(ulonglong *)(lVar6 + 0x10) < uVar30;
  uVar30 = *(ulonglong *)(lVar6 + 0x10) - uVar30;
  uVar16 = uVar30;
  if (bVar34) {
    uVar16 = uVar17;
  }
  bVar35 = *(ulonglong *)(lVar6 + 0x30) < uVar29;
  uVar29 = *(ulonglong *)(lVar6 + 0x30) - uVar29;
  uVar23 = uVar29;
  if (bVar35) {
    uVar23 = uVar17;
  }
  uVar16 = uVar23 + uVar16;
  if (!bVar34 && uVar30 != 0 || !bVar35 && uVar29 != 0) {
    uVar16 = uVar17;
  }
  *(ulonglong *)*in_stack_00000070 = *(longlong *)(lVar6 + 0x40) - uVar16;
  if (!bVar34 && uVar30 != 0 || !bVar35 && uVar29 != 0) {
    local_49 = 1;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    local_49 = 1;
    std::path::Path::_join();
    uVar16 = local_68;
    uStack_f8 = SUB81(local_130,0);
    uStack_f7 = (undefined5)((ulonglong)local_130 >> 8);
    uStack_f2 = (undefined2)((ulonglong)local_130 >> 0x30);
    uStack_f0 = pcStack_128;
    uStack_108 = SUB81(local_140,0);
    uStack_107 = (undefined5)((ulonglong)local_140 >> 8);
    uStack_102 = (undefined2)((ulonglong)local_140 >> 0x30);
    uStack_100 = SUB86(pcStack_138,0);
    uStack_fa = (undefined2)((ulonglong)pcStack_138 >> 0x30);
    cStack_d4 = 2;
    local_e0 = 1;
    iStack_dc = 0x200;
    iStack_d8 = 0x200;
    local_e8 = 0x55;
    local_120 = (undefined **)0x0;
    ppuStack_118 = (undefined **)local_1e0;
    local_110 = ppuStack_1d8;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
    uVar3 = ((uint *)*in_stack_00000048)[1];
    fVar38 = ((float)*(uint *)*in_stack_00000048 * 64.0) / (float)(uVar3 + (uVar3 == 0));
    if (fVar38 <= 1.0) {
      fVar38 = 1.0;
    }
    iVar19 = 0;
    if (0.0 <= fVar38) {
      iVar19 = (int)(longlong)fVar38;
    }
    iVar26 = -1;
    if (fVar38 <= 4.294967e+09) {
      iVar26 = iVar19;
    }
    local_49 = 1;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    local_49 = 1;
    std::path::Path::_join();
    uVar16 = local_68;
    uStack_f8 = SUB81(local_130,0);
    uStack_f7 = (undefined5)((ulonglong)local_130 >> 8);
    uStack_f2 = (undefined2)((ulonglong)local_130 >> 0x30);
    uStack_f0 = pcStack_128;
    uStack_108 = SUB81(local_140,0);
    uStack_107 = (undefined5)((ulonglong)local_140 >> 8);
    uStack_102 = (undefined2)((ulonglong)local_140 >> 0x30);
    uStack_100 = SUB86(pcStack_138,0);
    uStack_fa = (undefined2)((ulonglong)pcStack_138 >> 0x30);
    cStack_d4 = '\x02';
    local_e0 = 1;
    iStack_d8 = 0x40;
    local_e8 = 0x5f;
    local_120 = (undefined **)0x0;
    ppuStack_118 = (undefined **)local_1e0;
    local_110 = ppuStack_1d8;
    iStack_dc = iVar26;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
    }
    local_70[uVar16 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,local_e0);
    (local_70 + uVar16 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    local_70[uVar16 * 10 + 6] = uStack_f0;
    (local_70 + uVar16 * 10 + 6)[1] = (undefined *)CONCAT71(uStack_e7,local_e8);
    local_70[uVar16 * 10 + 4] = (undefined *)CONCAT26(uStack_fa,uStack_100);
    (local_70 + uVar16 * 10 + 4)[1] = (undefined *)CONCAT26(uStack_f2,CONCAT51(uStack_f7,uStack_f8))
    ;
    local_70[uVar16 * 10 + 2] = (undefined *)local_110;
    (local_70 + uVar16 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    local_70[uVar16 * 10] = (undefined *)local_120;
    (local_70 + uVar16 * 10)[1] = (undefined *)ppuStack_118;
    local_68 = uVar16 + 1;
  }
  LOCK();
  bVar34 = ACTIVE_SCREENSHOT_REQUESTS == '\0';
  if (bVar34) {
    ACTIVE_SCREENSHOT_REQUESTS = '\x01';
  }
  UNLOCK();
  if (!bVar34) {
    local_49 = 1;
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  local_49 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  ppuVar25 = local_70 + local_68 * 10;
  local_120 = local_70;
  ppuStack_118 = local_70;
  local_110 = (undefined **)local_78;
  uStack_108 = SUB81(ppuVar25,0);
  uStack_107 = (undefined5)((ulonglong)ppuVar25 >> 8);
  uStack_102 = (undefined2)((ulonglong)ppuVar25 >> 0x30);
  if ((ulonglong)(DAT_14362b058 - DAT_14362b068) < local_68) {
    local_5a = 1;
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  local_140 = &DAT_14362b068;
  pcStack_138 = (code *)DAT_14362b068;
  local_130 = (undefined **)DAT_14362b060;
  local_5a = 0;
  core::iter::adapters::map::impl$2::
  fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env_2,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl_20::extend_trusted::closure_env_0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env_2>_>_>_>
            ();
  country_core::meta_commander::MetaCommander::peek();
  if ((extraout_RAX_06 == (longlong *)0x0) || (*extraout_RAX_06 != -0x7ffffffffffffff7))
  goto LAB_14031c443;
  puVar24 = (undefined8 *)extraout_RAX_06[3];
  if ((longlong)puVar24 < 0) {
LAB_14031c62c:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar32 = (void *)extraout_RAX_06[2];
  if (puVar24 == (undefined8 *)0x0) {
    plVar18 = (longlong *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    plVar18 = extraout_RAX_07;
    if (extraout_RAX_07 == (longlong *)0x0) goto LAB_14031c62c;
  }
  local_80 = puVar24;
  local_58 = plVar18;
  memcpy(plVar18,pvVar32,(size_t)puVar24);
  lVar6 = extraout_RAX_06[4];
  **(undefined4 **)(in_stack_00000060 + 0x10) = *(undefined4 *)(in_stack_00000060 + 0x1c);
  local_4f = 1;
  country_core::meta_commander::MetaCommander::consume();
  if (local_120 == (undefined **)0x8000000000000014) goto LAB_14031c38d;
  uVar16 = 0xd;
  if (((ulonglong)local_120 ^ 0x8000000000000000) < 0x14) {
    uVar16 = (ulonglong)local_120 ^ 0x8000000000000000;
  }
  switch(uVar16) {
  case 0:
    plVar18 = (longlong *)CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108));
    if (plVar18 != (longlong *)0x0) {
      LOCK();
      *plVar18 = *plVar18 + -1;
      UNLOCK();
      if (*plVar18 == 0) {
        local_4f = 1;
        alloc::sync::Arc::
        drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                  ();
      }
    }
    break;
  case 8:
  case 9:
  case 0xc:
  case 0xe:
  case 0xf:
    ppuVar25 = ppuStack_118;
    goto joined_r0x00014031c4a2;
  case 0xd:
    if (local_120 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    ppuVar25 = (undefined **)
               (CONCAT26(uStack_102,CONCAT51(uStack_107,uStack_108)) & 0x7fffffffffffffff);
joined_r0x00014031c4a2:
    if (ppuVar25 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
LAB_14031c38d:
  lVar21 = DAT_14362b068;
  uStack_108 = SUB81(local_80,0);
  uStack_107 = (undefined5)((ulonglong)local_80 >> 8);
  uStack_102 = (undefined2)((ulonglong)local_80 >> 0x30);
  uStack_100 = SUB86(local_58,0);
  uStack_fa = (undefined2)((ulonglong)local_58 >> 0x30);
  uVar5 = (undefined4)local_140;
  uStack_f0._0_5_ = CONCAT41(uVar5,(char)lVar6);
  uStack_f0 = (code *)CONCAT44(local_140._3_4_,(undefined4)uStack_f0);
  local_120 = (undefined **)0x0;
  local_e8 = 0x55;
  local_e0 = 0;
  cStack_d4 = '\x04';
  local_d0 = 1;
  local_8a = 6;
  uStack_f8 = uStack_108;
  uStack_f7 = uStack_107;
  uStack_f2 = uStack_102;
  if (DAT_14362b068 == DAT_14362b058) {
    alloc::raw_vec::RawVec::
    grow_one<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>();
  }
  memmove((void *)(lVar21 * 0x98 + DAT_14362b060),&local_120,0x98);
  DAT_14362b068 = lVar21 + 1;
LAB_14031c443:
  local_49 = 0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar34 = ACTIVE_SCREENSHOT_REQUESTS == '\x01';
  if (bVar34) {
    ACTIVE_SCREENSHOT_REQUESTS = '\0';
  }
  UNLOCK();
  if (!bVar34) {
    local_49 = 0;
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  return;
}

