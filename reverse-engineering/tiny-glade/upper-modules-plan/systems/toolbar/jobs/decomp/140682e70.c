// 0x140682e70 load_configs
// signature: void __cdecl load_configs(void)

void __cdecl toolbar::mei::core::MeiContext::load_configs(void)

{
  bool bVar1;
  short *psVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  short **extraout_RAX_02;
  short **ppsVar3;
  undefined8 *extraout_RAX_03;
  longlong *extraout_RAX_04;
  short **ppsVar4;
  short **ppsVar5;
  int local_410;
  undefined8 uStack_378;
  undefined8 local_370;
  undefined8 uStack_368;
  longlong local_328;
  undefined8 local_2f0;
  undefined8 local_2e0;
  undefined8 local_2d8;
  undefined8 local_2d0;
  undefined8 uStack_2c8;
  undefined8 local_2c0;
  undefined8 local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  undefined8 local_2a0;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  longlong *local_278;
  short *psStack_270;
  short **local_268;
  short **ppsStack_260;
  short **local_258;
  undefined1 local_250;
  undefined4 local_24f;
  undefined3 uStack_24b;
  undefined8 local_248;
  undefined8 uStack_240;
  short **local_238;
  code *local_230;
  short ***local_228;
  code *local_220;
  undefined *local_218;
  undefined8 local_210;
  undefined8 **local_208;
  undefined8 local_200;
  longlong **local_1f8;
  undefined8 local_1f0;
  short **local_1e8;
  short **ppsStack_1e0;
  short **local_1d8;
  code *pcStack_1d0;
  char *local_1c8;
  char *pcStack_1c0;
  undefined8 local_1b8;
  undefined **ppuStack_1b0;
  undefined **local_1a8;
  short ***pppsStack_1a0;
  short ***local_198;
  undefined8 uStack_190;
  short **local_188;
  short **ppsStack_180;
  short **local_178;
  code *pcStack_170;
  undefined8 local_168;
  char *pcStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined **local_148;
  undefined8 uStack_140;
  short ***local_138;
  undefined8 uStack_130;
  short *local_128;
  short **local_120;
  short *local_118;
  short **local_110;
  short *local_108;
  short **local_100;
  longlong *local_f8;
  short *psStack_f0;
  short **local_e8;
  short **ppsStack_e0;
  short **local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  char *pcStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined **local_a8;
  undefined8 uStack_a0;
  short ***local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  longlong *local_68;
  short *local_60;
  longlong *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_e8 = (short **)&DAT_0000000a;
  ppsStack_e0 = (short **)0x0;
  local_d8 = (short **)0xffffffffffffffff;
  local_f8 = (longlong *)0x0;
  uStack_d0 = (code *)((ulonglong)uStack_d0 & 0xffffffffff000000);
  std::path::Path::to_path_buf();
  local_258 = local_d8;
  local_268 = local_e8;
  ppsStack_260 = ppsStack_e0;
  local_278 = local_f8;
  psStack_270 = psStack_f0;
  uStack_24b = (undefined3)((ulonglong)uStack_d0 >> 0x28);
  uStack_290 = uStack_378;
  local_288 = local_370;
  uStack_280 = uStack_368;
  local_250 = 0;
  local_2d0 = 0;
  uStack_2c8 = 0;
  local_2b8 = 0;
  uStack_2b0 = 0;
  local_248 = 0;
  uStack_240 = 0;
  local_2f0 = 0;
  local_2e0 = 0;
  local_2d8 = 8;
  local_2c0 = 8;
  local_2a8 = 8;
  local_2a0 = 0;
  local_24f = uStack_d0._1_4_;
  do {
    do {
      ppsVar4 = &psStack_f0;
      walkdir::impl$4::next();
      if (local_410 == 3) {
        ::core::ptr::drop_in_place<walkdir::IntoIter>();
        return;
      }
      ::core::ops::function::impls::impl$3::
      call_mut<tuple_<enum2_<core::result::Result<walkdir::dent::DirEntry,walkdir::error::Error>_>_>,toolbar::mei::config::impl_2::load_configs::closure_env_0>
                ();
    } while (local_f8 == (longlong *)0x2);
    memcpy(&uStack_378,ppsVar4,0x88);
    walkdir::dent::DirEntry::path();
    local_110 = ppsVar4;
    std::path::Path::extension();
    if ((((extraout_RAX != 0) &&
         (std::ffi::os_str::impl$41::try_from(), ((ulonglong)local_f8 & 1) == 0)) &&
        (local_e8 == (short **)0x3)) && ((char)psStack_f0[1] == 'n' && *psStack_f0 == 0x6f72)) {
      walkdir::dent::DirEntry::path();
      relative_path::path_ext::impl$3::relative_to<ref_<std::path::Path>_>();
      local_58 = local_f8;
      if (SBORROW8(0,(longlong)local_f8)) {
        local_188 = (short **)CONCAT71(local_188._1_7_,psStack_f0._0_1_);
                    // WARNING: Subroutine does not return
        ::core::result::unwrap_failed();
      }
      local_108 = psStack_f0;
      relative_path::RelativePath::to_relative_path_buf();
      ppsVar4 = local_e8;
      local_60 = psStack_f0;
      ppsVar3 = local_e8;
      relative_path::RelativePath::file_stem();
      psVar2 = local_60;
      if (extraout_RAX_00 != 0) {
        ppsVar5 = (short **)((extraout_RAX_00 + (longlong)ppsVar3) - (longlong)local_60);
        if (((ppsVar5 <= ppsVar4) &&
            (bVar1 = ppsVar5 < ppsVar4, ppsVar4 = ppsVar5,
            bVar1 && (short *)(extraout_RAX_00 + (longlong)ppsVar3) != local_60)) &&
           (*(char *)((longlong)local_60 + (longlong)ppsVar5) < -0x40)) {
                    // WARNING: Subroutine does not return
          ::core::panicking::panic();
        }
      }
      local_68 = local_f8;
      if (local_58 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_128 = psVar2;
      local_120 = ppsVar4;
      config::read_node_config_from_file();
      if (SBORROW8(0,(longlong)local_188)) {
        local_198 = (short ***)uStack_130;
        local_1a8 = (undefined **)uStack_140;
        pppsStack_1a0 = local_138;
        local_1b8 = uStack_150;
        ppuStack_1b0 = local_148;
        local_1c8 = pcStack_160;
        pcStack_1c0 = (char *)local_158;
        local_1d8 = (short **)pcStack_170;
        pcStack_1d0 = (code *)local_168;
        local_1e8 = ppsStack_180;
        ppsStack_1e0 = local_178;
        local_238 = &local_118;
        local_230 = ::core::fmt::impl$74::fmt<std::ffi::os_str::OsStr>;
        local_228 = &local_1e8;
        local_220 = ron::error::impl$0::fmt;
        local_f8 = (longlong *)0x2;
        local_e8 = (short **)0x2;
        local_d8 = (short **)0x0;
        uStack_d0 = (code *)0x20;
        local_c8 = CONCAT71(local_c8._1_7_,3);
        pcStack_c0 = (char *)0x2;
        uStack_b0 = 2;
        uStack_a0 = 1;
        local_98 = (short ***)0x400000020;
        uStack_90 = CONCAT71(uStack_90._1_7_,3);
        local_218 = &DAT_142a17b90;
        local_210 = 2;
        local_1f0 = 2;
        local_208 = &local_238;
        local_200 = 2;
        local_1f8 = &local_f8;
                    // WARNING: Subroutine does not return
        ::core::panicking::panic_fmt();
      }
      local_198 = local_138;
      uStack_190 = uStack_130;
      local_1a8 = local_148;
      pppsStack_1a0 = (short ***)uStack_140;
      local_1b8 = local_158;
      ppuStack_1b0 = (undefined **)uStack_150;
      local_1c8 = (char *)local_168;
      pcStack_1c0 = pcStack_160;
      local_1d8 = local_178;
      pcStack_1d0 = pcStack_170;
      local_1e8 = local_188;
      ppsStack_1e0 = ppsStack_180;
      if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_188 = &local_118;
        ppsStack_180 = (short **)::core::fmt::impl$74::fmt<std::ffi::os_str::OsStr>;
        local_178 = &local_128;
        pcStack_170 = ::core::fmt::impl$74::fmt<str_>;
        local_49 = 1;
        widgets::tweaker::TweakerView::as_inner_mut();
        local_d8 = (short **)*extraout_RAX_01;
        uStack_d0 = (code *)extraout_RAX_01[1];
        local_c8 = 5;
        pcStack_c0 = "toolbar::mei::config: ";
        local_b8 = 0x14;
        local_a8 = &PTR_s_Loaded_UI_config_from_as_142a17b40;
        uStack_a0 = 2;
        uStack_90 = 2;
        local_88 = 0;
        local_f8 = (longlong *)0x0;
        psStack_f0 = (short *)0x142a17b78;
        local_e8 = (short **)&DAT_00000014;
        ppsStack_e0 = (short **)0x0;
        uStack_b0 = CONCAT44(*(undefined4 *)(extraout_RAX_01 + 2),1);
        local_49 = 1;
        local_98 = &local_188;
        ::log::__private_api::impl$2::log();
      }
      ppsVar4 = local_120;
      psVar2 = local_128;
      if ((longlong)local_120 < 0) {
LAB_1406836b6:
        local_49 = 1;
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (local_120 == (short **)0x0) {
        ppsVar3 = (short **)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        ppsVar3 = extraout_RAX_02;
        if (extraout_RAX_02 == (short **)0x0) {
          local_100 = ppsVar4;
          goto LAB_1406836b6;
        }
      }
      memcpy(ppsVar3,psVar2,(size_t)ppsVar4);
      local_188 = ppsVar4;
      local_178 = ppsVar4;
      local_98 = local_198;
      uStack_90 = uStack_190;
      local_a8 = local_1a8;
      uStack_a0 = pppsStack_1a0;
      local_b8 = local_1b8;
      uStack_b0 = ppuStack_1b0;
      local_c8 = local_1c8;
      pcStack_c0 = pcStack_1c0;
      local_d8 = local_1d8;
      uStack_d0 = pcStack_1d0;
      local_e8 = local_1e8;
      ppsStack_e0 = ppsStack_1e0;
      local_f8 = (longlong *)0x1;
      psStack_f0 = (short *)0x1;
      ppsStack_180 = ppsVar3;
      local_100 = ppsVar3;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_03[0xc] = local_98;
      extraout_RAX_03[0xd] = uStack_90;
      extraout_RAX_03[10] = local_a8;
      extraout_RAX_03[0xb] = uStack_a0;
      extraout_RAX_03[8] = local_b8;
      extraout_RAX_03[9] = uStack_b0;
      extraout_RAX_03[6] = local_c8;
      extraout_RAX_03[7] = pcStack_c0;
      extraout_RAX_03[4] = local_d8;
      extraout_RAX_03[5] = uStack_d0;
      extraout_RAX_03[2] = local_e8;
      extraout_RAX_03[3] = ppsStack_e0;
      *extraout_RAX_03 = local_f8;
      extraout_RAX_03[1] = psStack_f0;
      hashbrown::map::HashMap::
      insert<alloc::string::String,alloc::sync::Arc<toolbar::mei::config::NodeConfig,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      local_f8 = extraout_RAX_04;
      if (extraout_RAX_04 != (longlong *)0x0) {
        LOCK();
        *extraout_RAX_04 = *extraout_RAX_04 + -1;
        UNLOCK();
        if (*extraout_RAX_04 == 0) {
          local_49 = 0;
          alloc::sync::Arc::drop_slow<toolbar::mei::config::NodeConfig,alloc::alloc::Global>();
        }
      }
      if (local_68 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    if (local_328 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  } while( true );
}

