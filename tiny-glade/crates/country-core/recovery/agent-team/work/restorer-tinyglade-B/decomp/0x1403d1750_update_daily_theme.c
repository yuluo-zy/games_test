// update_daily_theme @ 0x1403d1750
// sig: void __cdecl update_daily_theme(void)


void __cdecl tiny_glade::theme_fetcher_plugin::update_daily_theme(void)

{
  char cVar1;
  undefined4 uVar2;
  char *pcVar3;
  undefined4 *puVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined *puVar7;
  undefined *puVar8;
  undefined *puVar9;
  undefined7 uVar10;
  longlong *plVar11;
  char extraout_AL;
  char extraout_AL_00;
  undefined **extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined *puVar12;
  undefined *puVar13;
  bool bVar14;
  undefined **local_2f8;
  longlong lStack_2f0;
  undefined *local_2c8;
  undefined *puStack_2c0;
  undefined *local_2b8;
  longlong lStack_2b0;
  undefined **local_2a8;
  longlong lStack_2a0;
  undefined *local_298;
  undefined *puStack_290;
  longlong local_288;
  longlong lStack_280;
  undefined *local_278;
  undefined *puStack_270;
  undefined *local_268;
  undefined *puStack_260;
  longlong local_258;
  undefined **ppuStack_250;
  undefined **local_248;
  longlong lStack_240;
  longlong local_238;
  longlong lStack_230;
  longlong local_228;
  undefined4 local_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined *local_1e8;
  undefined *puStack_1e0;
  undefined *local_1d8;
  undefined *puStack_1d0;
  undefined *local_1c8;
  longlong lStack_1c0;
  undefined **local_1b8;
  undefined **ppuStack_1b0;
  longlong local_1a8;
  longlong lStack_1a0;
  longlong local_198;
  longlong lStack_190;
  undefined *local_188;
  longlong lStack_180;
  undefined *local_178;
  undefined *local_170;
  undefined *local_168;
  undefined *puStack_160;
  longlong local_158;
  longlong lStack_150;
  undefined *local_138;
  undefined *local_130;
  undefined *local_128;
  undefined *local_120;
  undefined *local_118;
  undefined *puStack_110;
  undefined *local_108;
  undefined *puStack_100;
  undefined *local_f8;
  undefined1 uStack_f0;
  undefined7 uStack_ef;
  undefined **local_e8;
  undefined **ppuStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  longlong local_c8;
  longlong lStack_c0;
  undefined **local_b8;
  undefined *local_b0;
  undefined *local_a8;
  undefined *puStack_a0;
  undefined *local_98;
  undefined4 local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined *local_78;
  undefined *local_70;
  undefined *local_68;
  longlong *local_60;
  undefined *local_58;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  pcVar3 = (char *)*in_RCX;
  LOCK();
  cVar1 = *pcVar3;
  if (cVar1 == '\0') {
    *pcVar3 = '\x01';
  }
  UNLOCK();
  if (cVar1 != '\0') {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_60 = (longlong *)pcVar3;
  if (*(longlong *)(pcVar3 + 8) == 0) {
    std::sync::mpmc::array::Channel::
    try_recv<enum2_<core::result::Result<theme_defs::ActiveEvent,enum2_<theme_defs::EventDataError>_>_>_>
              ();
  }
  else if ((int)*(longlong *)(pcVar3 + 8) == 1) {
    std::sync::mpmc::list::Channel::
    try_recv<enum2_<core::result::Result<theme_defs::ActiveEvent,enum2_<theme_defs::EventDataError>_>_>_>
              ();
  }
  else {
    std::sync::mpmc::zero::Channel::
    try_recv<enum2_<core::result::Result<theme_defs::ActiveEvent,enum2_<theme_defs::EventDataError>_>_>_>
              ();
  }
  plVar11 = local_60;
  uVar10 = uStack_ef;
  puVar9 = local_f8;
  puVar13 = puStack_100;
  puVar8 = local_108;
  puVar12 = puStack_110;
  puVar7 = local_118;
  if (local_118 == (undefined *)0x8000000000000001) {
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar14 = (char)*plVar11 == '\x01';
    if (bVar14) {
      *(char *)plVar11 = '\0';
    }
    UNLOCK();
    if (bVar14) {
      return;
    }
    parking_lot::raw_mutex::RawMutex::unlock_slow();
    return;
  }
  uVar6 = CONCAT71(uStack_ef,uStack_f0);
  local_49 = 1;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar14 = (char)*plVar11 == '\x01';
  if (bVar14) {
    *(char *)plVar11 = '\0';
  }
  UNLOCK();
  if (!bVar14) {
    local_49 = 1;
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  uVar2 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar4 = (undefined4 *)in_RDX[2];
  *puVar4 = uVar2;
  local_60 = (longlong *)*in_RDX;
  if (SBORROW8(0,*local_60)) {
    lVar5 = local_60[1];
    if ((-0x7ffffffffffffffe < lVar5 || lVar5 == -0x7fffffffffffffff) && (lVar5 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    core::ptr::drop_in_place<country_core::resources::daily::DailyThemeInfo>();
  }
  plVar11 = local_60;
  *local_60 = -0x8000000000000000;
  local_60[1] = -0x8000000000000000;
  local_1f8 = SUB84(puVar9,0);
  local_7c = local_1f8;
  uStack_1f4 = (undefined4)((ulonglong)puVar9 >> 0x20);
  local_80 = uStack_1f4;
  uStack_1f0 = (undefined4)uVar6;
  local_84 = uStack_1f0;
  uStack_1ec = (undefined4)((uint7)uVar10 >> 0x18);
  local_88 = uStack_1ec;
  if (SBORROW8(0,(longlong)puVar7)) {
    local_78 = (undefined *)0x8000000000000000;
    local_98 = puVar8;
    goto LAB_1403d1f0e;
  }
  local_138 = puVar7;
  local_130 = puVar12;
  local_128 = puVar8;
  local_120 = puVar13;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read::inner();
  if (local_118 != (undefined *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar13 = puStack_160;
  if (SBORROW8(0,(longlong)local_168)) {
    local_188 = puStack_160;
    local_4a = 1;
    std::backtrace::Backtrace::capture();
    local_1c8 = local_f8;
    local_1d8 = local_108;
    puStack_1d0 = puStack_100;
    local_1e8 = local_118;
    puStack_1e0 = puStack_110;
    local_118 = &DAT_1429d1d82;
    puStack_110 = (undefined *)0x12;
    local_108 = puStack_160;
    local_4a = 0;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
  }
  else {
    local_58 = local_168;
    local_68 = puStack_160;
    serde_json::de::from_slice<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>_>();
    puVar12 = puStack_a0;
    if (SBORROW8(0,(longlong)local_a8)) {
      local_168 = puStack_a0;
      local_4b = 1;
      std::backtrace::Backtrace::capture();
      local_1c8 = local_f8;
      local_1d8 = local_108;
      puStack_1d0 = puStack_100;
      local_1e8 = local_118;
      puStack_1e0 = puStack_110;
      local_118 = &DAT_1429d1d94;
      puStack_110 = (undefined *)0x12;
      local_108 = puVar12;
      local_4b = 0;
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
      if (local_58 != (undefined *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      local_78 = local_a8;
      if (local_58 != (undefined *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::path::Path::_join();
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      country_core::resources::session::SessionMeta::try_load_from_path();
      if (SBORROW8(0,(longlong)local_118)) {
        local_188 = &DAT_1429d1daf;
        lStack_180 = 0x14;
        local_178 = puStack_110;
        local_168 = (undefined *)0x3;
        anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,anyhow::Error>_>();
        if (local_a8 != (undefined *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      else {
        local_198 = local_c8;
        lStack_190 = lStack_c0;
        lStack_1c0 = CONCAT71(uStack_ef,uStack_f0);
        local_1a8 = local_d8;
        lStack_1a0 = lStack_d0;
        local_1b8 = local_e8;
        ppuStack_1b0 = ppuStack_e0;
        local_1c8 = local_f8;
        local_1d8 = local_108;
        puStack_1d0 = puStack_100;
        local_1e8 = local_118;
        puStack_1e0 = puStack_110;
        if (local_a8 != (undefined *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::path::Path::_join();
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::path::Path::is_file();
        if (extraout_AL != '\0') {
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::path::Path::_join();
          std::fs::read_to_string<std::path::PathBuf>();
          if (SBORROW8(0,(longlong)local_a8)) {
            core::ptr::drop_in_place<std::io::error::Error>();
LAB_1403d2120:
            local_b0 = (undefined *)0x8000000000000000;
            local_70 = (undefined *)0x8000000000000000;
            local_b8 = (undefined **)&DAT_8000000000000000;
          }
          else {
            local_58 = local_a8;
            local_68 = puStack_a0;
            serde_json::de::from_str<country_core::resources::daily::DailyThemeCredits>();
            local_70 = local_118;
            if (local_118 == (undefined *)0x8000000000000001) {
              core::ptr::drop_in_place<serde_json::error::Error>();
              local_b8 = extraout_RAX;
              local_b0 = puVar13;
            }
            else {
              local_a8 = puStack_110;
              puStack_a0 = local_108;
              lStack_180 = CONCAT71(uStack_ef,uStack_f0);
              local_188 = local_f8;
              local_2f8 = ppuStack_e0;
              lStack_2f0 = local_d8;
              local_b8 = local_e8;
              local_b0 = puStack_100;
            }
            if (local_58 != (undefined *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_70 == (undefined *)0x8000000000000001) goto LAB_1403d2120;
            local_2c8 = local_a8;
            puStack_2c0 = puStack_a0;
            local_2b8 = local_188;
            lStack_2b0 = lStack_180;
            local_2a8 = local_2f8;
            lStack_2a0 = lStack_2f0;
          }
          puVar13 = local_1e8;
          local_278 = puStack_1e0;
          puStack_270 = local_1d8;
          local_268 = puStack_1d0;
          puStack_260 = local_1c8;
          local_258 = lStack_1c0;
          ppuStack_250 = local_1b8;
          local_248 = ppuStack_1b0;
          lStack_240 = local_1a8;
          local_238 = lStack_1a0;
          lStack_230 = local_198;
          local_228 = lStack_190;
          local_298 = local_168;
          puStack_290 = puStack_160;
          local_288 = local_158;
          lStack_280 = lStack_150;
          if (local_138 != (undefined *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          goto LAB_1403d1f0e;
        }
        local_118 = &DAT_1429d1de8;
        puStack_110 = (undefined *)0x1;
        local_108 = (undefined *)0x8;
        puStack_100 = (undefined *)0x0;
        local_f8 = (undefined *)0x0;
        anyhow::__private::format_err();
        if (local_168 != (undefined *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        bevy_ecs::component::ComponentDescriptor::
        drop_ptr<country_core::resources::session::ActiveSession>();
      }
      if (local_98 != (undefined *)0x0) {
        puVar12 = puVar12 + 8;
        puVar13 = local_98;
        do {
          if (*(longlong *)(puVar12 + -8) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          puVar12 = puVar12 + 0x18;
          puVar13 = puVar13 + -1;
        } while (puVar13 != (undefined *)0x0);
      }
      if (local_78 != (undefined *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  if (local_138 != (undefined *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_1e8 = (undefined *)0x0;
  puStack_1e0 = (undefined *)0x1;
  local_1d8 = (undefined *)0x0;
  local_118 = (undefined *)0x0;
  local_108 = (undefined *)0x0;
  local_f8 = (undefined *)0x20;
  uStack_f0 = 3;
  local_e8 = &local_1e8;
  ppuStack_e0 = &
                PTR_drop_in_place<image::buffer_::ImageBuffer<image::color::Rgb<u8>,alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_1429d18d8
  ;
  anyhow::error::impl$4::fmt();
  puVar13 = local_1d8;
  puVar7 = puStack_1e0;
  puVar12 = local_1e8;
  if (extraout_AL_00 != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_49 = 0;
  anyhow::error::impl$6::drop();
  local_78 = (undefined *)0x8000000000000000;
  local_98 = puVar7;
LAB_1403d1f0e:
  *puVar4 = uVar2;
  local_170 = puVar13;
  local_68 = local_98;
  local_58 = puVar12;
  if (SBORROW8(0,*plVar11)) {
    lVar5 = plVar11[1];
    if ((-0x7ffffffffffffffe < lVar5 || lVar5 == -0x7fffffffffffffff) && (lVar5 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    core::ptr::drop_in_place<country_core::resources::daily::DailyThemeInfo>();
  }
  *local_60 = (longlong)local_78;
  local_60[1] = (longlong)local_58;
  local_60[2] = (longlong)local_68;
  local_60[3] = (longlong)local_170;
  local_60[4] = (longlong)local_278;
  local_60[5] = (longlong)puStack_270;
  local_60[6] = (longlong)local_268;
  local_60[7] = (longlong)puStack_260;
  local_60[8] = local_258;
  local_60[9] = (longlong)ppuStack_250;
  local_60[10] = (longlong)local_248;
  local_60[0xb] = lStack_240;
  local_60[0xc] = local_238;
  local_60[0xd] = lStack_230;
  local_60[0xe] = local_228;
  local_60[0xf] = (longlong)local_298;
  local_60[0x10] = (longlong)puStack_290;
  local_60[0x11] = local_288;
  local_60[0x12] = lStack_280;
  local_60[0x13] = (longlong)local_70;
  local_60[0x14] = (longlong)local_2c8;
  local_60[0x15] = (longlong)puStack_2c0;
  local_60[0x16] = (longlong)local_b0;
  local_60[0x17] = (longlong)local_2b8;
  local_60[0x18] = lStack_2b0;
  local_60[0x19] = (longlong)local_b8;
  local_60[0x1a] = (longlong)local_2a8;
  local_60[0x1b] = lStack_2a0;
  *(undefined4 *)(local_60 + 0x1c) = local_7c;
  *(undefined4 *)((longlong)local_60 + 0xe4) = local_80;
  *(undefined4 *)(local_60 + 0x1d) = local_84;
  *(undefined4 *)((longlong)local_60 + 0xec) = local_88;
  return;
}

