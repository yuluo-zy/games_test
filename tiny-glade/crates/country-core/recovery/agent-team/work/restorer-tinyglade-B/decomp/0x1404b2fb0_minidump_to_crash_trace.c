// minidump_to_crash_trace @ 0x1404b2fb0
// sig: void __cdecl minidump_to_crash_trace(void)


// WARNING: Function: __chkstk replaced with injection: alloca_probe
// WARNING: Type propagation algorithm not settling

void __cdecl tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace(void)

{
  longlong ***ppplVar1;
  longlong ***ppplVar2;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 uVar3;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *in_RCX;
  undefined4 uVar4;
  int local_1d38 [344];
  longlong ***local_17d8;
  longlong ***ppplStack_17d0;
  char ****ppppcStack_17c8;
  char ****ppppcStack_17c0;
  char ****ppppcStack_17b8;
  undefined8 *puStack_17b0;
  undefined8 local_17a8;
  undefined8 uStack_17a0;
  undefined8 local_1798;
  undefined8 uStack_1790;
  undefined8 local_1788;
  undefined8 uStack_1780;
  undefined8 local_1778;
  undefined1 uStack_1770;
  undefined8 local_1768;
  char local_171c;
  undefined2 local_171b;
  undefined1 local_1719;
  char ****local_16c0;
  char ****local_16b8;
  undefined2 local_16ae;
  longlong ***local_988;
  longlong ***local_980;
  longlong ***ppplStack_978;
  longlong ***ppplStack_970;
  longlong ***local_968;
  undefined8 uStack_960;
  longlong ***local_478;
  longlong ***local_470;
  undefined1 local_468 [232];
  longlong ***local_380;
  undefined8 local_378;
  longlong ***local_370;
  longlong ***ppplStack_368;
  longlong ***local_360;
  char local_2c4;
  undefined2 local_2c3;
  undefined1 local_2c1;
  longlong ***local_2a8;
  longlong ***ppplStack_2a0;
  longlong ***local_298;
  longlong ***ppplStack_290;
  longlong local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 local_260;
  undefined8 uStack_258;
  undefined8 local_250;
  longlong local_248;
  undefined8 local_230;
  longlong ***local_228;
  longlong ***local_220;
  longlong ***ppplStack_218;
  longlong ***local_210;
  longlong lStack_208;
  undefined8 local_200;
  longlong ***local_1f8;
  longlong ***local_1f0;
  longlong ***local_1e8;
  longlong ***ppplStack_1e0;
  longlong ***local_1d8;
  undefined8 uStack_1d0;
  longlong ***local_1c8;
  longlong ***local_1c0;
  longlong ***local_1b8;
  longlong ***ppplStack_1b0;
  longlong local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 local_140;
  longlong ***local_138;
  longlong ***ppplStack_130;
  longlong ***local_128;
  longlong lStack_120;
  undefined8 local_118;
  longlong ***local_e8;
  longlong ***ppplStack_e0;
  longlong ***local_d8;
  longlong ***ppplStack_d0;
  longlong ***local_c8;
  undefined8 uStack_c0;
  longlong ***local_a8;
  longlong ***local_a0;
  longlong ***local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined4 local_80;
  undefined1 local_75;
  undefined1 local_74;
  undefined1 local_73;
  undefined1 local_72;
  undefined1 local_71;
  undefined8 local_70;
  undefined8 uStack_48;
  
  uStack_48 = 0x1404b2fc6;
  local_70 = 0xfffffffffffffffe;
  minidump::minidump::Minidump::read_path<ref_<std::path::Path>_>();
  ppplVar2 = ppplStack_17d0;
  ppplVar1 = local_17d8;
  if (local_17d8 == (longlong ***)0x2) {
    local_968 = (longlong ***)ppppcStack_17c0;
    ppplStack_978 = ppplStack_17d0;
    ppplStack_970 = (longlong ***)ppppcStack_17c8;
    std::backtrace::Backtrace::capture();
    local_988 = (longlong ***)&DAT_1429e8e58;
    local_980 = (longlong ***)0x10;
    anyhow::Error::
    construct<anyhow::error::ContextError<ref_<str_>,enum2_<minidump::minidump::Error>_>_>();
    in_RCX[1] = extraout_RAX_04;
    *in_RCX = 3;
    return;
  }
  memcpy(local_468,&ppppcStack_17c8,0xe8);
  local_478 = ppplVar1;
  local_470 = ppplVar2;
  minidump::minidump::Minidump::get_stream<memmap2::Mmap,minidump::minidump::MinidumpSystemInfo>();
  if (local_17d8 == (longlong ***)0x8000000000000001) {
    local_968 = (longlong ***)ppppcStack_17c0;
    ppplStack_978 = ppplStack_17d0;
    ppplStack_970 = (longlong ***)ppppcStack_17c8;
    std::backtrace::Backtrace::capture();
    local_988 = (longlong ***)&DAT_1429e8e68;
    local_980 = (longlong ***)0x12;
    anyhow::Error::
    construct<anyhow::error::ContextError<ref_<str_>,enum2_<minidump::minidump::Error>_>_>();
    in_RCX[1] = extraout_RAX;
    *in_RCX = 3;
    goto LAB_1404b39f4;
  }
  local_1b8 = (longlong ***)ppppcStack_17c8;
  ppplStack_1b0 = (longlong ***)ppppcStack_17c0;
  local_1a8 = (longlong)ppppcStack_17b8;
  uStack_1a0 = puStack_17b0;
  local_198 = local_17a8;
  uStack_190 = uStack_17a0;
  local_188 = local_1798;
  uStack_180 = uStack_1790;
  local_178 = local_1788;
  uStack_170 = uStack_1780;
  local_168 = local_1778;
  local_158 = local_1768;
  local_1c8 = local_17d8;
  local_1c0 = ppplStack_17d0;
  minidump::minidump::Minidump::get_stream<memmap2::Mmap,minidump::minidump::MinidumpMiscInfo>();
  if ((int)local_17d8 == 5) {
    local_1d38[0] = 5;
  }
  else {
    memcpy(local_1d38,&local_17d8,0x560);
  }
  minidump::minidump::Minidump::get_stream<memmap2::Mmap,minidump::minidump::MinidumpModuleList>();
  if (SBORROW8(0,(longlong)local_988)) {
    local_360 = ppplStack_970;
    local_370 = local_980;
    ppplStack_368 = ppplStack_978;
    std::backtrace::Backtrace::capture();
    local_380 = (longlong ***)&DAT_1429e8e7a;
    local_378 = 0x12;
    anyhow::Error::
    construct<anyhow::error::ContextError<ref_<str_>,enum2_<minidump::minidump::Error>_>_>();
    in_RCX[1] = extraout_RAX_00;
    *in_RCX = 3;
  }
  else {
    local_1e8 = ppplStack_978;
    ppplStack_1e0 = ppplStack_970;
    local_1d8 = local_968;
    uStack_1d0 = uStack_960;
    local_1f8 = local_988;
    local_1f0 = local_980;
    minidump::minidump::Minidump::get_stream<memmap2::Mmap,minidump::minidump::MinidumpException>();
    ppplVar1 = local_17d8;
    if (local_171c == '\x02') {
      std::backtrace::Backtrace::capture();
      local_e8 = (longlong ***)&DAT_1429e8e8c;
      ppplStack_e0 = (longlong ***)0x11;
      local_d8 = local_17d8;
      ppplStack_d0 = ppplStack_17d0;
      local_c8 = (longlong ***)ppppcStack_17c8;
      anyhow::Error::
      construct<anyhow::error::ContextError<ref_<str_>,enum2_<minidump::minidump::Error>_>_>();
      uVar3 = extraout_RAX_01;
LAB_1404b3348:
      in_RCX[1] = uVar3;
      *in_RCX = 3;
    }
    else {
      memcpy(&local_378,&ppplStack_17d0,0xb4);
      local_2c3 = local_171b;
      local_2c1 = local_1719;
      local_380 = ppplVar1;
      local_2c4 = local_171c;
      minidump::minidump::MinidumpException::context();
      ppplVar2 = ppplStack_17d0;
      ppplVar1 = local_17d8;
      if ((int)local_17d8 == 10) {
        std::backtrace::Backtrace::capture();
        anyhow::Error::construct<anyhow::wrapper::DisplayError<ref_<str_>_>_>();
        uVar3 = extraout_RAX_02;
        goto LAB_1404b3348;
      }
      memcpy(&ppplStack_978,&ppppcStack_17c8,0x500);
      local_988 = ppplVar1;
      local_980 = ppplVar2;
      local_72 = 1;
      minidump::minidump::Minidump::
      get_stream<memmap2::Mmap,minidump::minidump::MinidumpMemoryListBase<minidump_common::format::MINIDUMP_MEMORY_DESCRIPTOR>_>
                ();
      if (!SBORROW8(0,(longlong)local_17d8)) {
        local_118 = puStack_17b0;
        local_128 = (longlong ***)ppppcStack_17c0;
        lStack_120 = (longlong)ppppcStack_17b8;
        local_230 = 0;
LAB_1404b3589:
        local_200 = local_118;
        local_210 = local_128;
        lStack_208 = lStack_120;
        local_220 = ppplStack_17d0;
        ppplStack_218 = (longlong ***)ppppcStack_17c8;
        local_228 = local_17d8;
        local_138 = ppplStack_17d0;
        ppplStack_130 = (longlong ***)ppppcStack_17c8;
        minidump::context::MinidumpContext::get_stack_pointer();
        local_73 = 1;
        minidump::minidump::UnifiedMemoryList::memory_at_address();
        if ((int)local_988 == 9) {
          local_73 = 0;
          alloc::borrow::impl$1::to_owned<minidump::context::MinidumpContext>();
        }
        else {
          local_17d8 = local_988;
          ppplStack_17d0 = local_980;
          memcpy(&ppppcStack_17c8,&ppplStack_978,0x500);
        }
        local_73 = 0;
        minidump_unwind::CallStack::with_context();
        local_150 = 0;
        local_148 = 8;
        local_140 = 0;
        local_16b8 = (char ****)&local_1f8;
        local_16ae = 0x100;
        local_71 = 1;
        local_16c0 = (char ****)&local_1c8;
        futures_lite::future::
        block_on<minidump_unwind::symbols::debuginfo::DebugInfoSymbolProvider,enum2_<minidump_unwind::symbols::debuginfo::impl_12::build::async_fn_env_0>_>
                  ();
        alloc::alloc::exchange_malloc();
        extraout_RAX_05[4] = local_c8;
        extraout_RAX_05[5] = uStack_c0;
        extraout_RAX_05[2] = local_d8;
        extraout_RAX_05[3] = ppplStack_d0;
        *extraout_RAX_05 = local_e8;
        extraout_RAX_05[1] = ppplStack_e0;
        local_71 = 1;
        minidump_unwind::symbols::MultiSymbolProvider::add();
        uVar3 = local_198;
        uVar4 = (undefined4)local_158;
        local_71 = 1;
        minidump::minidump::MinidumpSystemInfo::cpu_info();
        if (local_248 == -0x7fffffffffffffff) {
          local_17d8 = (longlong ***)0x8000000000000000;
        }
        else {
          local_71 = 1;
          alloc::borrow::Cow::into_owned<str_>();
        }
        local_90 = uVar3;
        local_d8 = (longlong ***)0x8000000000000000;
        uStack_c0 = 0x8000000000000000;
        local_a8 = local_17d8;
        local_a0 = ppplStack_17d0;
        local_98 = (longlong ***)ppppcStack_17c8;
        local_e8 = (longlong ***)0x0;
        local_88 = 1;
        local_17a8 = 0;
        ppppcStack_17c8 = (char ****)&local_138;
        ppppcStack_17c0 = (char ****)&local_1f8;
        uStack_1770 = 0;
        ppppcStack_17b8 = (char ****)&local_e8;
        puStack_17b0 = &local_150;
        local_80 = uVar4;
        futures_lite::future::
        block_on<tuple_<>,enum2_<minidump_unwind::walk_stack::async_fn_env_0<minidump_unwind::symbols::MultiSymbolProvider,tuple_<>_>_>_>
                  ();
        local_71 = 1;
        core::ptr::drop_in_place<minidump_unwind::system_info::SystemInfo>();
        ppplStack_d0 = ppplStack_130 + (longlong)local_128 * 0xda;
        local_e8 = ppplStack_130;
        ppplStack_e0 = ppplStack_130;
        local_d8 = local_138;
        local_74 = 1;
        minidump::minidump::MinidumpException::get_crash_reason();
        ppppcStack_17c8 = (char ****)local_d8;
        ppppcStack_17c0 = (char ****)ppplStack_d0;
        local_17d8 = local_e8;
        ppplStack_17d0 = ppplStack_e0;
        local_74 = 0;
        alloc::vec::in_place_collect::impl$1::
        from_iter<sentry_types::protocol::v7::Frame,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env_2>_>
                  ();
        local_74 = 0;
        sentry_types::protocol::v7::Stacktrace::from_frames_reversed();
        local_1798 = local_268;
        local_17a8 = local_278;
        uStack_17a0 = uStack_270;
        ppppcStack_17b8 = (char ****)local_288;
        puStack_17b0 = (undefined8 *)uStack_280;
        ppppcStack_17c8 = (char ****)local_298;
        ppppcStack_17c0 = (char ****)ppplStack_290;
        local_17d8 = local_2a8;
        ppplStack_17d0 = ppplStack_2a0;
        uStack_1780 = local_250;
        uStack_1790 = local_260;
        local_1788 = uStack_258;
        in_RCX[6] = local_278;
        in_RCX[7] = uStack_270;
        in_RCX[4] = local_288;
        in_RCX[5] = uStack_280;
        in_RCX[2] = local_298;
        in_RCX[3] = ppplStack_290;
        *in_RCX = local_2a8;
        in_RCX[1] = ppplStack_2a0;
        in_RCX[10] = uStack_258;
        in_RCX[0xb] = local_250;
        in_RCX[8] = local_268;
        in_RCX[9] = local_260;
        local_75 = 0;
        core::ptr::
        drop_in_place<alloc::vec::Vec<alloc::boxed::Box<dyn_<bevy_app::plugin::Plugin>,alloc::alloc::Global>,alloc::alloc::Global>_>
                  ();
        if ((!SBORROW8(0,lStack_120)) && (lStack_120 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        core::ptr::drop_in_place<enum2_<minidump::minidump::UnifiedMemoryList>_>();
        core::ptr::drop_in_place<minidump::minidump::MinidumpModuleList>();
        core::ptr::drop_in_place<minidump::minidump::MinidumpSystemInfo>();
        goto LAB_1404b39f4;
      }
      local_72 = 1;
      minidump::minidump::Minidump::
      get_stream<memmap2::Mmap,minidump::minidump::MinidumpMemoryListBase<minidump_common::format::MINIDUMP_MEMORY_DESCRIPTOR64>_>
                ();
      if (!SBORROW8(0,(longlong)local_17d8)) {
        local_118 = puStack_17b0;
        local_128 = (longlong ***)ppppcStack_17c0;
        lStack_120 = (longlong)ppppcStack_17b8;
        local_230 = 1;
        goto LAB_1404b3589;
      }
      local_138 = (longlong ***)ppppcStack_17c8;
      ppplStack_130 = (longlong ***)ppppcStack_17c0;
      local_d8 = ppplStack_17d0;
      ppplStack_d0 = (longlong ***)ppppcStack_17c8;
      local_c8 = (longlong ***)ppppcStack_17c0;
      local_72 = 1;
      std::backtrace::Backtrace::capture();
      local_e8 = (longlong ***)0x1429e8eae;
      ppplStack_e0 = (longlong ***)0xb;
      local_72 = 1;
      anyhow::Error::
      construct<anyhow::error::ContextError<ref_<str_>,enum2_<minidump::minidump::Error>_>_>();
      in_RCX[1] = extraout_RAX_03;
      *in_RCX = 3;
      if ((int)local_988 != 9) {
        core::ptr::drop_in_place<minidump::context::MinidumpContext>();
      }
    }
    core::ptr::drop_in_place<minidump::minidump::MinidumpModuleList>();
  }
  if ((!SBORROW8(0,(longlong)local_1c8)) && (local_1c8 != (longlong ***)0x0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((!SBORROW8(0,(longlong)ppplStack_1b0)) && (ppplStack_1b0 != (longlong ***)0x0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_1404b39f4:
  core::ptr::drop_in_place<minidump::minidump::Minidump<memmap2::Mmap>_>();
  return;
}

