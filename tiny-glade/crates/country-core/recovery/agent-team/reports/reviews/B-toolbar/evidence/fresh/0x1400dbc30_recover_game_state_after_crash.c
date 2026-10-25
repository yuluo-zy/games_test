
void __cdecl tiny_glade::panic_reporter::recover_game_state_after_crash(void)

{
  undefined8 ***extraout_RAX;
  undefined8 ***extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 **ppuVar1;
  undefined8 *in_RCX;
  undefined8 **ppuVar2;
  undefined8 ***pppuVar3;
  undefined8 uVar4;
  undefined8 local_3a8;
  longlong lStack_3a0;
  undefined8 local_398;
  undefined8 *puStack_390;
  undefined8 local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 **local_340;
  code *pcStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined *local_320;
  longlong lStack_318;
  undefined8 **local_310;
  undefined8 uStack_308;
  undefined **local_2f8;
  undefined8 uStack_2f0;
  undefined8 **local_2e8;
  undefined8 uStack_2e0;
  undefined8 *local_2d8;
  undefined8 uStack_2d0;
  longlong lStack_2c8;
  undefined8 local_2c0;
  undefined8 *puStack_2b8;
  undefined8 local_2b0;
  undefined8 uStack_2a8;
  undefined8 local_2a0;
  undefined8 uStack_298;
  undefined8 local_290;
  undefined8 uStack_288;
  undefined8 local_280;
  undefined8 uStack_278;
  undefined8 local_270;
  undefined *local_268;
  longlong lStack_260;
  undefined8 **local_258;
  char *local_248;
  undefined8 uStack_240;
  undefined8 **local_238;
  undefined8 uStack_230;
  char *local_228;
  undefined8 uStack_220;
  undefined8 **local_218;
  undefined8 uStack_210;
  char *local_208;
  undefined8 uStack_200;
  undefined8 **local_1f8;
  undefined8 uStack_1f0;
  char *local_1e8;
  undefined8 uStack_1e0;
  undefined8 **local_1d8;
  undefined8 uStack_1d0;
  char *local_1c8;
  undefined8 uStack_1c0;
  undefined8 **local_1b8;
  undefined8 uStack_1b0;
  char *local_1a8;
  undefined8 uStack_1a0;
  undefined8 **local_198;
  undefined8 uStack_190;
  char *local_188;
  undefined8 uStack_180;
  undefined8 **local_178;
  undefined8 uStack_170;
  undefined8 **local_160;
  code *pcStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 **local_140;
  undefined8 **ppuStack_138;
  code *pcStack_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 *local_118;
  undefined8 **ppuStack_110;
  undefined8 **local_108;
  undefined8 uStack_100;
  undefined8 **local_f8;
  undefined8 local_f0;
  undefined8 **local_e8;
  char *local_e0;
  undefined8 *local_d8;
  undefined8 **ppuStack_d0;
  code *local_c8;
  undefined1 uStack_c0;
  undefined7 uStack_bf;
  undefined8 **local_b8;
  undefined8 uStack_b0;
  undefined8 **local_a8;
  undefined8 uStack_a0;
  undefined8 *local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 **local_48;
  undefined1 local_3c;
  undefined1 local_3b;
  undefined1 local_3a;
  undefined1 local_39;
  undefined1 local_38;
  undefined1 local_37;
  undefined1 local_36;
  undefined1 local_35;
  undefined1 local_34;
  undefined1 local_33;
  undefined1 local_32;
  undefined1 local_31;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined1 local_2d;
  undefined1 local_2c;
  undefined1 local_2b;
  undefined1 local_2a;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  ::utils::file::date_time_string_for_filename();
  pcStack_158 = alloc::string::impl$23::fmt;
  local_2f8 = &PTR_s_tmp_panics_panic__14293aaa8;
  uStack_2f0 = 1;
  local_2e8 = &local_160;
  uStack_2e0 = 1;
  local_2d8 = (undefined8 **)0x0;
  local_160 = &local_b8;
  alloc::fmt::format::format_inner();
  if (local_b8 != (undefined8 **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_d8 = local_118;
  ppuStack_d0 = ppuStack_110;
  local_c8 = (code *)local_108;
  uStack_c0 = 1;
  local_2f8 = (undefined **)CONCAT71(local_2f8._1_7_,1);
  local_29 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_29 = 1;
  std::fs::DirBuilder::_create();
  if (extraout_RAX == (undefined8 ***)0x0) {
    local_48 = (undefined8 ***)0x0;
  }
  else {
    local_3c = 1;
    std::backtrace::Backtrace::capture();
    local_98 = local_2d8;
    uStack_90 = uStack_2d0;
    local_a8 = local_2e8;
    uStack_a0 = uStack_2e0;
    local_b8 = (undefined8 **)local_2f8;
    uStack_b0 = uStack_2f0;
    local_2f8 = (undefined **)0x14293aab8;
    uStack_2f0 = 0x21;
    local_3c = 0;
    local_2e8 = extraout_RAX;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
    local_48 = extraout_RAX_00;
  }
  local_2c = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_2c = 1;
  panic_dump::dump_save();
  local_2a = 1;
  local_2b = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_2a = 1;
  local_2b = 1;
  panic_screenshot::dump_screenshot();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_2a = 1;
    local_2b = 1;
    recover_game_state_after_crash::logging_map_error();
    local_188 = (char *)0x8000000000000000;
  }
  else {
    local_178 = local_a8;
    uStack_170 = uStack_a0;
    local_188 = (char *)local_b8;
    uStack_180 = uStack_b0;
  }
  local_2d = 1;
  local_2e = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_2d = 1;
  local_2e = 1;
  panic_dump::dump_rhapsody_log();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_2d = 1;
    local_2e = 1;
    recover_game_state_after_crash::logging_map_error();
    local_1a8 = (char *)0x8000000000000000;
  }
  else {
    local_198 = local_a8;
    uStack_190 = uStack_a0;
    local_1a8 = (char *)local_b8;
    uStack_1a0 = uStack_b0;
  }
  local_2f = 1;
  local_30 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_2f = 1;
  local_30 = 1;
  panic_dump::dump_inputs();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_2f = 1;
    local_30 = 1;
    recover_game_state_after_crash::logging_map_error();
    local_1c8 = (char *)0x8000000000000000;
  }
  else {
    local_1b8 = local_a8;
    uStack_1b0 = uStack_a0;
    local_1c8 = (char *)local_b8;
    uStack_1c0 = uStack_b0;
  }
  local_31 = 1;
  local_32 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_31 = 1;
  local_32 = 1;
  panic_dump::dump_log_file();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_31 = 1;
    local_32 = 1;
    recover_game_state_after_crash::logging_map_error();
    local_1e8 = (char *)0x8000000000000000;
  }
  else {
    local_1d8 = local_a8;
    uStack_1d0 = uStack_a0;
    local_1e8 = (char *)local_b8;
    uStack_1e0 = uStack_b0;
  }
  local_33 = 1;
  local_34 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_33 = 1;
  local_34 = 1;
  panic_dump::dump_settings_file();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_33 = 1;
    local_34 = 1;
    recover_game_state_after_crash::logging_map_error();
    local_208 = (char *)0x8000000000000000;
  }
  else {
    local_1f8 = local_a8;
    uStack_1f0 = uStack_a0;
    local_208 = (char *)local_b8;
    uStack_200 = uStack_b0;
  }
  local_35 = 1;
  local_36 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_35 = 1;
  local_36 = 1;
  panic_dump::dump_profile_file();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_35 = 1;
    local_36 = 1;
    recover_game_state_after_crash::logging_map_error();
    local_228 = (char *)0x8000000000000000;
  }
  else {
    local_218 = local_a8;
    uStack_210 = uStack_a0;
    local_228 = (char *)local_b8;
    uStack_220 = uStack_b0;
  }
  local_37 = 1;
  local_38 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_37 = 1;
  local_38 = 1;
  panic_dump::dump_gpu_crash_dump_file();
  if (SBORROW8(0,(longlong)local_b8)) {
    local_37 = 1;
    local_38 = 1;
    recover_game_state_after_crash::logging_map_error();
    local_248 = (char *)0x8000000000000000;
  }
  else {
    local_238 = local_a8;
    uStack_230 = uStack_a0;
    local_248 = (char *)local_b8;
    uStack_240 = uStack_b0;
  }
  if ((undefined8 ***)local_48 == (undefined8 ***)0x0) {
    local_118 = (undefined8 *)0x0;
    ppuStack_110 = (undefined8 **)0x8;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == 0) {
      local_39 = 1;
      local_3a = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_118 = (undefined8 *)0x1;
    ppuStack_110 = (undefined8 **)extraout_RAX_01;
  }
  local_b8 = &local_108;
  local_108 = (undefined8 ***)0x0;
  uStack_b0 = 0;
  local_a8 = ppuStack_110;
  core::iter::adapters::map::impl$2::
  fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref_<anyhow::Error>_>,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env_7,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl_20::extend_trusted::closure_env_0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref_<anyhow::Error>_>,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env_7>_>_>_>
            ();
  pppuVar3 = (undefined8 ***)local_48;
  local_268 = (undefined *)local_118;
  lStack_260 = (longlong)ppuStack_110;
  local_258 = local_108;
  local_140 = (undefined8 **)local_d8;
  ppuStack_138 = ppuStack_d0;
  if ((undefined8 ***)local_48 == (undefined8 ***)0x0) {
    pppuVar3 = (undefined8 ***)ppuStack_d0;
    if (!SBORROW8(0,(longlong)local_d8)) {
      pcStack_130 = local_c8;
      uStack_128 = CONCAT71(uStack_bf,uStack_c0);
      goto LAB_1400dc40a;
    }
  }
  else if ((undefined8 **)local_d8 != (undefined8 **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_140 = &local_e8;
  ppuStack_138 = (undefined8 **)anyhow::error::impl$4::fmt;
  local_b8 = (undefined8 **)0x2;
  local_a8 = (undefined8 **)0x2;
  local_98 = (undefined8 **)0x0;
  uStack_90 = 0x400000020;
  local_88 = CONCAT71(local_88._1_7_,3);
  local_118 = (undefined8 *)&DAT_14293ab20;
  ppuStack_110 = (undefined8 **)0x1;
  local_108 = &local_140;
  uStack_100 = 1;
  local_f0 = 1;
  local_f8 = &local_b8;
  local_e8 = pppuVar3;
  alloc::fmt::format::format_inner();
  ppuStack_138 = local_160;
  pcStack_130 = pcStack_158;
  uStack_128 = local_150;
  local_3b = 1;
  anyhow::error::impl$6::drop();
  local_140 = (undefined8 **)0x8000000000000000;
LAB_1400dc40a:
  ppuVar1 = (undefined8 **)-(longlong)local_2f8;
  ppuVar2 = (undefined8 **)local_2f8;
  uVar4 = uStack_2f0;
  if (!SBORROW8(0,(longlong)local_2f8)) {
    if (SBORROW8(0,(longlong)local_2d8)) {
      ppuVar1 = (undefined8 **)0x8000000000000000;
    }
    else {
      local_120 = uStack_2f0;
      local_e0 = (char *)local_2f8;
      local_b8 = (undefined8 **)local_2d8;
      uStack_b0 = uStack_2d0;
      local_a8 = (undefined8 **)lStack_2c8;
      uStack_a0 = local_2c0;
      local_98 = puStack_2b8;
      uStack_90 = local_2b0;
      local_88 = uStack_2a8;
      local_80 = local_2a0;
      uStack_78 = uStack_298;
      local_70 = local_290;
      uStack_68 = uStack_288;
      local_60 = local_280;
      uStack_58 = uStack_278;
      local_50 = local_270;
      country_core::resources::session::SessionId::save_folder_path();
      country_core::resources::session::SessionId::backup_folder_path();
      local_3a8 = uStack_b0;
      lStack_3a0 = (longlong)local_a8;
      local_398 = uStack_a0;
      puStack_390 = local_98;
      local_388 = uStack_90;
      uStack_380 = local_88;
      local_378 = local_80;
      uStack_370 = uStack_78;
      local_368 = local_70;
      uStack_360 = uStack_68;
      local_358 = local_60;
      uStack_350 = uStack_58;
      local_348 = local_50;
      local_330 = local_150;
      uStack_328 = uStack_148;
      local_340 = local_160;
      pcStack_338 = pcStack_158;
      local_310 = local_108;
      uStack_308 = uStack_100;
      local_320 = (undefined *)local_118;
      lStack_318 = (longlong)ppuStack_110;
      ppuVar1 = local_b8;
      ppuVar2 = (undefined8 **)local_e0;
      uVar4 = local_120;
    }
  }
  local_b8 = local_2e8;
  uStack_b0 = uStack_2e0;
  in_RCX[0x27] = local_1b8;
  in_RCX[0x28] = uStack_1b0;
  in_RCX[0x25] = local_1c8;
  in_RCX[0x26] = uStack_1c0;
  in_RCX[0x2b] = local_1d8;
  in_RCX[0x2c] = uStack_1d0;
  in_RCX[0x29] = local_1e8;
  in_RCX[0x2a] = uStack_1e0;
  in_RCX[0x33] = local_1f8;
  in_RCX[0x34] = uStack_1f0;
  in_RCX[0x31] = local_208;
  in_RCX[0x32] = uStack_200;
  in_RCX[0x2f] = local_218;
  in_RCX[0x30] = uStack_210;
  in_RCX[0x2d] = local_228;
  in_RCX[0x2e] = uStack_220;
  in_RCX[0x35] = local_248;
  in_RCX[0x36] = uStack_240;
  in_RCX[0x37] = local_238;
  in_RCX[0x38] = uStack_230;
  in_RCX[0x39] = local_1a8;
  in_RCX[0x3a] = uStack_1a0;
  in_RCX[0x3b] = local_198;
  in_RCX[0x3c] = uStack_190;
  *in_RCX = local_268;
  in_RCX[1] = lStack_260;
  in_RCX[2] = local_258;
  in_RCX[5] = pcStack_130;
  in_RCX[6] = uStack_128;
  in_RCX[3] = local_140;
  in_RCX[4] = ppuStack_138;
  in_RCX[7] = local_188;
  in_RCX[8] = uStack_180;
  in_RCX[9] = local_178;
  in_RCX[10] = uStack_170;
  in_RCX[0xb] = ppuVar2;
  in_RCX[0xc] = uVar4;
  in_RCX[0xd] = local_2e8;
  in_RCX[0xe] = uStack_2e0;
  in_RCX[0xf] = ppuVar1;
  memcpy(in_RCX + 0x10,&local_3a8,0xa8);
  return;
}

