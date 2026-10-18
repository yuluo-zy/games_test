// verify_package_integrity_and_show_report @ 0x1402552b0
// sig: void __cdecl verify_package_integrity_and_show_report(void)


// WARNING: Type propagation algorithm not settling

void __cdecl tiny_glade::integrity_report_main::verify_package_integrity_and_show_report(void)

{
  undefined8 ******ppppppuVar1;
  char cVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined8 ****ppppuVar5;
  uint uVar6;
  undefined *puVar7;
  byte extraout_AL;
  byte bVar8;
  char extraout_AL_00;
  uint extraout_EAX;
  undefined8 *******extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  undefined8 *extraout_RAX_08;
  undefined8 *extraout_RAX_09;
  undefined8 *extraout_RAX_10;
  undefined8 *extraout_RAX_11;
  undefined4 in_EDX;
  undefined8 unaff_R15;
  bool bVar9;
  undefined8 *******local_538;
  code *local_530;
  undefined8 *******pppppppuStack_528;
  code *pcStack_520;
  undefined4 uStack_518;
  undefined8 ******local_450;
  undefined8 uStack_448;
  undefined8 local_440;
  undefined8 uStack_438;
  undefined8 local_430;
  undefined8 uStack_428;
  undefined8 local_420;
  undefined8 uStack_418;
  undefined8 local_410;
  longlong local_400;
  undefined4 local_3f8;
  undefined8 *******local_3f4;
  code *pcStack_3ec;
  undefined8 *******local_3e4;
  code *pcStack_3dc;
  undefined4 local_3d4;
  undefined8 local_3d0;
  undefined8 ******local_3b8;
  HWND pHStack_3b0;
  undefined8 local_3a8;
  undefined8 uStack_3a0;
  undefined8 *****local_390;
  undefined8 ******ppppppuStack_388;
  undefined8 local_380;
  undefined8 *****pppppuStack_378;
  undefined8 local_370;
  undefined8 uStack_368;
  undefined8 *****local_360;
  undefined8 uStack_358;
  undefined8 local_350;
  undefined8 ******local_348;
  HWND pHStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 *******local_328;
  HWND pHStack_320;
  undefined8 *******local_318;
  code *pcStack_310;
  undefined *local_308;
  code *pcStack_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined4 local_2d0;
  undefined4 local_2b8;
  undefined4 local_2a0;
  undefined4 local_288;
  undefined4 local_270;
  undefined8 local_258;
  undefined4 local_250;
  undefined1 local_24c;
  undefined4 local_24b;
  undefined1 local_247;
  undefined4 local_246;
  ulonglong local_1f8;
  undefined8 ******ppppppuStack_1f0;
  undefined2 local_1e8;
  undefined6 uStack_1e6;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 ******ppppppuStack_1b0;
  undefined8 ******ppppppuStack_1a8;
  undefined8 ******ppppppuStack_1a0;
  ulonglong local_198;
  undefined8 ******local_190;
  undefined8 local_188;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 local_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined8 ****local_158;
  char *pcStack_150;
  undefined8 local_148;
  undefined4 local_140;
  undefined4 local_13c;
  undefined **local_138;
  undefined8 local_130;
  undefined8 *******local_128;
  undefined8 local_120;
  undefined8 *******local_118;
  undefined8 uStack_110;
  ulonglong uStack_a0;
  undefined8 ******ppppppuStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined1 local_52;
  undefined1 local_51;
  undefined8 local_50;
  
  local_50 = 0xfffffffffffffffe;
  std::time::Instant::now();
  tg_package_integrity::verify_package_manifest();
  puVar7 = __imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
  if (local_390 == (undefined8 *****)0x8000000000000004) {
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_51 = 1;
      std::time::Instant::elapsed();
      pHStack_320 = (HWND)CONCAT44(pHStack_320._4_4_,in_EDX);
      local_538 = &local_328;
      local_530 = core::time::impl$12::fmt;
      local_51 = 1;
      local_328 = extraout_RAX;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_13c = *(undefined4 *)(extraout_RAX_00 + 2);
      local_158 = (undefined8 ****)0x3;
      pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
      local_148 = 0x21;
      local_138 = &PTR_s_Package_manifest_check_passed__T_1429a3ba8;
      local_130 = 1;
      local_128 = &local_538;
      local_120 = 1;
      local_118 = (undefined8 *******)0x0;
      local_188._0_4_ = 0;
      local_188._4_4_ = 0;
      uStack_180 = 0x429a3958;
      uStack_17c = 1;
      uStack_178 = 0x21;
      uStack_174 = 0;
      uStack_170 = 0;
      uStack_16c = 0;
      local_168 = (undefined4)*extraout_RAX_00;
      uStack_164 = (undefined4)((ulonglong)*extraout_RAX_00 >> 0x20);
      uStack_160 = (undefined4)extraout_RAX_00[1];
      uStack_15c = (undefined4)((ulonglong)extraout_RAX_00[1] >> 0x20);
      local_140 = 1;
      local_51 = 1;
      ::log::__private_api::impl$2::log();
    }
  }
  else {
    switch((ulonglong)local_390 ^ 0x8000000000000000) {
    case 0:
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_328 = (undefined8 *******)&UNK_1429a38d8;
        pHStack_320 = (HWND)anyhow::context::impl$3::
                            fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_01 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = (undefined **)&UNK_1429a3900;
        local_130 = 1;
        local_128 = &local_328;
        local_120 = 1;
        local_118 = (undefined8 *******)0x0;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_01;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_01 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_01[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_01[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      break;
    case 1:
      local_538 = &ppppppuStack_388;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_328 = (undefined8 *******)&UNK_1429a38d8;
        pHStack_320 = (HWND)anyhow::context::impl$3::
                            fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_318 = &local_538;
        pcStack_310 = core::fmt::impl$74::fmt<alloc::string::String>;
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_08 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = (undefined **)&UNK_1429a3990;
        local_130 = 2;
        local_128 = &local_328;
        local_120 = 2;
        local_118 = (undefined8 *******)0x0;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_08;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_08 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_08[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_08[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      break;
    case 2:
      local_538 = &ppppppuStack_388;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_328 = (undefined8 *******)&UNK_1429a38d8;
        pHStack_320 = (HWND)anyhow::context::impl$3::
                            fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_318 = &local_538;
        pcStack_310 = core::fmt::impl$74::fmt<alloc::string::String>;
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_06 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = (undefined **)&UNK_1429a39d8;
        local_130 = 2;
        local_128 = &local_328;
        local_120 = 2;
        local_118 = (undefined8 *******)0x0;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_06;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_06 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_06[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_06[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      break;
    case 3:
      local_538 = &ppppppuStack_388;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_328 = (undefined8 *******)&UNK_1429a38d8;
        pHStack_320 = (HWND)anyhow::context::impl$3::
                            fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_318 = &local_538;
        pcStack_310 = core::fmt::impl$76::fmt<u32>;
        local_308 = &UNK_1429a3a10;
        pcStack_300 = core::fmt::num::imp::impl$6::fmt;
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_07 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = (undefined **)&UNK_1429a3a30;
        local_130 = 3;
        local_128 = &local_328;
        local_120 = 3;
        local_118 = (undefined8 *******)0x0;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_07;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_07 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_07[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_07[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      break;
    default:
      ppppppuStack_1a0 = &local_390;
      ppppppuStack_1a8 = &pppppuStack_378;
      ppppppuStack_1b0 = &local_360;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_02 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = &PTR_s_File_integrity_check_failed__1429a3a98;
        local_130 = 1;
        local_128 = (undefined8 *******)&DAT_00000008;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        local_120 = 0;
        local_118 = (undefined8 *******)0x0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_02;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_02 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_02[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_02[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      if ((ppppppuStack_1a0[2] != (undefined8 *****)0x0) && (*(longlong *)puVar7 != 0)) {
        local_538 = &ppppppuStack_1a0;
        local_530 = core::fmt::impl$74::
                    fmt<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>_>;
        local_328 = (undefined8 *******)0x2;
        local_318 = (undefined8 *******)0x2;
        local_308 = (undefined *)0x0;
        pcStack_300 = (code *)0x400000020;
        local_2f8 = CONCAT71(local_2f8._1_7_,3);
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_03 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = &PTR_s_Missing_unreadable_files__1429a3ae0;
        local_130 = 1;
        local_128 = &local_538;
        local_120 = 1;
        local_118 = &local_328;
        uStack_110 = 1;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_03;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_03 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_03[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_03[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      if ((ppppppuStack_1a8[2] != (undefined8 *****)0x0) && (*(longlong *)puVar7 != 0)) {
        local_538 = &ppppppuStack_1a8;
        local_530 = core::fmt::impl$74::
                    fmt<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>_>;
        local_328 = (undefined8 *******)0x2;
        local_318 = (undefined8 *******)0x2;
        local_308 = (undefined *)0x0;
        pcStack_300 = (code *)0x400000020;
        local_2f8 = CONCAT71(local_2f8._1_7_,3);
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_04 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = &PTR_s_Incorrect_files__1429a3b20;
        local_130 = 1;
        local_128 = &local_538;
        local_120 = 1;
        local_118 = &local_328;
        uStack_110 = 1;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_04;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_04 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_04[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_04[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
      if ((ppppppuStack_1b0[2] != (undefined8 *****)0x0) && (*(longlong *)puVar7 != 0)) {
        local_538 = &ppppppuStack_1b0;
        local_530 = core::fmt::impl$74::
                    fmt<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>_>;
        local_328 = (undefined8 *******)0x2;
        local_318 = (undefined8 *******)0x2;
        local_308 = (undefined *)0x0;
        pcStack_300 = (code *)0x400000020;
        local_2f8 = CONCAT71(local_2f8._1_7_,3);
        local_51 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_13c = *(undefined4 *)(extraout_RAX_05 + 2);
        local_158 = (undefined8 ****)0x1;
        pcStack_150 = "tiny_glade::integrity_report_mainCould not read : ";
        local_148 = 0x21;
        local_138 = &PTR_s_Extra_files__1429a3b58;
        local_130 = 1;
        local_128 = &local_538;
        local_120 = 1;
        local_118 = &local_328;
        uStack_110 = 1;
        local_188._0_4_ = 0;
        local_188._4_4_ = 0;
        uStack_180 = 0x429a3958;
        uStack_17c = 1;
        uStack_178 = 0x21;
        uStack_174 = 0;
        uStack_170 = 0;
        uStack_16c = 0;
        local_168 = (undefined4)*extraout_RAX_05;
        uStack_164 = (undefined4)((ulonglong)*extraout_RAX_05 >> 0x20);
        uStack_160 = (undefined4)extraout_RAX_05[1];
        uStack_15c = (undefined4)((ulonglong)extraout_RAX_05[1] >> 0x20);
        local_140 = 1;
        local_51 = 1;
        ::log::__private_api::impl$2::log();
      }
    }
  }
  local_1f8 = 0;
  local_1e8 = 0x100;
  LOCK();
  cVar2 = *__imp__ZN5winit10event_loop18EVENT_LOOP_CREATED17h3040f7edfbf9ff64E;
  *__imp__ZN5winit10event_loop18EVENT_LOOP_CREATED17h3040f7edfbf9ff64E = 1;
  UNLOCK();
  if (cVar2 == '\0') {
    winit::platform_impl::platform::event_loop::EventLoop::new<tuple_<>_>();
    local_538 = (undefined8 *******)CONCAT44(uStack_178,uStack_17c);
    local_530 = (code *)CONCAT44(uStack_170,uStack_174);
    pppppppuStack_528 = (undefined8 *******)CONCAT44(local_168,uStack_16c);
    pcStack_520 = (code *)CONCAT44(uStack_160,uStack_164);
    local_308 = (undefined *)CONCAT44(local_308._4_4_,uStack_15c);
    uStack_518 = uStack_15c;
    lVar3 = CONCAT44(local_188._4_4_,(undefined4)local_188);
    local_328 = local_538;
    pHStack_320 = (HWND)local_530;
    local_318 = pppppppuStack_528;
    pcStack_310 = pcStack_520;
    uVar4 = uStack_180;
    ppppuVar5 = local_158;
    if (CONCAT44(local_188._4_4_,(undefined4)local_188) == 3) {
      lVar3 = 3;
      ppppuVar5 = (undefined8 ****)unaff_R15;
    }
  }
  else {
    lVar3 = 3;
    uVar4 = 3;
    ppppuVar5 = (undefined8 ****)unaff_R15;
  }
  local_198 = local_1f8;
  if (local_1f8 != 0) {
    local_190 = ppppppuStack_1f0;
    if (*ppppppuStack_1f0 != (undefined8 *****)0x0) {
      (*(code *)*ppppppuStack_1f0)(local_1f8);
    }
    if (local_190[1] != (undefined8 *****)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if ((int)lVar3 == 3) {
    local_188._4_4_ = SUB84(local_538,0);
    uStack_180 = (undefined4)((ulonglong)local_538 >> 0x20);
    uStack_17c = SUB84(local_530,0);
    uStack_178 = (undefined4)((ulonglong)local_530 >> 0x20);
    uStack_174 = SUB84(pppppppuStack_528,0);
    uStack_170 = (undefined4)((ulonglong)pppppppuStack_528 >> 0x20);
    uStack_16c = SUB84(pcStack_520,0);
    local_168 = (undefined4)((ulonglong)pcStack_520 >> 0x20);
    uStack_164 = uStack_518;
    local_188._0_4_ = uVar4;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_3f4 = local_538;
  pcStack_3ec = local_530;
  local_3e4 = pppppppuStack_528;
  pcStack_3dc = pcStack_520;
  local_3d4 = uStack_518;
  local_400 = lVar3;
  local_3f8 = uVar4;
  local_3d0 = ppppuVar5;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_09 == (undefined8 *)0x0) {
    local_52 = 1;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_09 = 0x69772074696e6977;
  *(undefined4 *)(extraout_RAX_09 + 1) = 0x776f646e;
  local_2e8 = CONCAT44(local_2e8._4_4_,2);
  local_2d0 = 2;
  local_2b8 = 2;
  local_2a0 = 2;
  local_24c = 1;
  local_250 = 7;
  pcStack_300 = (code *)0xc;
  uStack_2f0 = 0xc;
  local_24b = 0x100;
  local_247 = 1;
  local_258 = 0;
  local_288 = 2;
  local_246 = 0x1020100;
  local_270 = 0xf;
  local_328 = (undefined8 *******)0x0;
  winit::platform_impl::platform::impl$0::default();
  memcpy(&local_188,&local_328,0xe8);
  local_90 = CONCAT62(uStack_1e6,local_1e8);
  uStack_a0 = local_1f8;
  ppppppuStack_98 = ppppppuStack_1f0;
  uStack_88 = uStack_1e0;
  uStack_80 = local_1d8;
  uStack_78 = uStack_1d0;
  local_70 = local_1c8;
  uStack_68 = uStack_1c0;
  local_60 = local_1b8;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_10 != (undefined8 *)0x0) {
    extraout_RAX_10[2] = 0x6163696669726576;
    extraout_RAX_10[3] = 0x706572206e6f6974;
    *extraout_RAX_10 = 0x616c4720796e6954;
    extraout_RAX_10[1] = 0x20656c6966206564;
    *(undefined4 *)((longlong)extraout_RAX_10 + 0x1f) = 0x74726f70;
    if (CONCAT44(uStack_15c,uStack_160) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uStack_160 = 0x23;
    uStack_15c = 0;
    pcStack_150 = (char *)0x23;
    memcpy(&local_328,&local_188,0x130);
    winit::dpi::impl$2::from_f64();
    winit::dpi::impl$2::from_f64();
    local_2e8 = (ulonglong)extraout_EAX << 0x20;
    memcpy(&local_538,&local_328,0x130);
    memcpy(&local_188,&local_538,0xe8);
    local_2e8 = local_410;
    local_2f8 = local_420;
    uStack_2f0 = uStack_418;
    local_308 = (undefined *)local_430;
    pcStack_300 = (code *)uStack_428;
    local_318 = (undefined8 *******)local_440;
    pcStack_310 = (code *)uStack_438;
    local_328 = (undefined8 *******)local_450;
    pHStack_320 = (HWND)uStack_448;
    local_52 = 1;
    winit::platform_impl::platform::window::init<tuple_<>_>();
    pHStack_3b0 = (HWND)CONCAT62(uStack_1e6,local_1e8);
    local_3b8 = ppppppuStack_1f0;
    local_3a8 = uStack_1e0;
    uStack_3a0 = local_1d8;
    if ((local_1f8 & 1) != 0) {
      local_318 = (undefined8 *******)uStack_1e0;
      pcStack_310 = (code *)local_1d8;
      local_328 = (undefined8 *******)ppppppuStack_1f0;
      uStack_178 = (undefined4)uStack_1e0;
      uStack_174 = (undefined4)((ulonglong)uStack_1e0 >> 0x20);
      uStack_170 = (undefined4)local_1d8;
      uStack_16c = (undefined4)((ulonglong)local_1d8 >> 0x20);
      local_188._0_4_ = SUB84(ppppppuStack_1f0,0);
      local_188._4_4_ = (undefined4)((ulonglong)ppppppuStack_1f0 >> 0x20);
      uStack_180 = SUB84(pHStack_3b0,0);
      uStack_17c = (undefined4)((uint6)uStack_1e6 >> 0x10);
      pHStack_320 = pHStack_3b0;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_338 = uStack_1e0;
    uStack_330 = local_1d8;
    local_348 = ppppppuStack_1f0;
    ppppppuVar1 = ppppppuStack_1f0 + 2;
    LOCK();
    bVar9 = *(char *)(ppppppuStack_1f0 + 2) == '\0';
    if (bVar9) {
      *(char *)(ppppppuStack_1f0 + 2) = '\x01';
    }
    UNLOCK();
    pHStack_340 = pHStack_3b0;
    if (!bVar9) {
      std::sys::sync::mutex::futex::Mutex::lock_contended();
    }
    puVar7 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
    if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
        & 0x7fffffffffffffff) == 0) {
      bVar8 = 0;
      cVar2 = *(char *)((longlong)ppppppuStack_1f0 + 0x11);
    }
    else {
      std::panicking::panic_count::is_zero_slow_path();
      bVar8 = extraout_AL ^ 1;
      cVar2 = *(char *)((longlong)ppppppuStack_1f0 + 0x11);
    }
    if (cVar2 == '\0') {
      *(undefined1 *)((longlong)ppppppuStack_1f0 + 0xe4) = 1;
      if (((bVar8 == 0) && ((*(ulonglong *)puVar7 & 0x7fffffffffffffff) != 0)) &&
         (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
        *(undefined1 *)((longlong)ppppppuStack_1f0 + 0x11) = 1;
      }
      LOCK();
      cVar2 = *(char *)ppppppuVar1;
      *(char *)ppppppuVar1 = '\0';
      UNLOCK();
      if (cVar2 == '\x02') {
        std::sys::sync::mutex::futex::Mutex::wake();
      }
      RedrawWindow(pHStack_340,(RECT *)0x0,(HRGN)0x0,2);
      local_318 = (undefined8 *******)local_338;
      pcStack_310 = (code *)uStack_330;
      local_328 = (undefined8 *******)local_348;
      pHStack_320 = pHStack_340;
      local_168 = (undefined4)local_338;
      uStack_164 = (undefined4)((ulonglong)local_338 >> 0x20);
      uStack_160 = (undefined4)uStack_330;
      uStack_15c = (undefined4)((ulonglong)uStack_330 >> 0x20);
      uStack_178 = SUB84(local_348,0);
      uStack_174 = (undefined4)((ulonglong)local_348 >> 0x20);
      uStack_170 = SUB84(pHStack_340,0);
      uStack_16c = (undefined4)((ulonglong)pHStack_340 >> 0x20);
      local_188._0_4_ = 1;
      local_188._4_4_ = 0;
      uStack_180 = 1;
      uStack_17c = 0;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_11 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_11[4] = CONCAT44(uStack_164,local_168);
      extraout_RAX_11[5] = CONCAT44(uStack_15c,uStack_160);
      extraout_RAX_11[2] = CONCAT44(uStack_174,uStack_178);
      extraout_RAX_11[3] = CONCAT44(uStack_16c,uStack_170);
      *extraout_RAX_11 = CONCAT44(local_188._4_4_,(undefined4)local_188);
      extraout_RAX_11[1] = CONCAT44(uStack_17c,uStack_180);
      local_148 = local_350;
      local_158 = local_360;
      pcStack_150 = (char *)uStack_358;
      local_168 = (undefined4)local_370;
      uStack_164 = (undefined4)((ulonglong)local_370 >> 0x20);
      uStack_160 = (undefined4)uStack_368;
      uStack_15c = (undefined4)((ulonglong)uStack_368 >> 0x20);
      uStack_178 = (undefined4)local_380;
      uStack_174 = (undefined4)((ulonglong)local_380 >> 0x20);
      uStack_170 = SUB84(pppppuStack_378,0);
      uStack_16c = (undefined4)((ulonglong)pppppuStack_378 >> 0x20);
      local_188._0_4_ = SUB84(local_390,0);
      local_188._4_4_ = (undefined4)((ulonglong)local_390 >> 0x20);
      uStack_180 = SUB84(ppppppuStack_388,0);
      uStack_17c = (undefined4)((ulonglong)ppppppuStack_388 >> 0x20);
      local_52 = 0;
      tg_integrity_report::run_integrity_report_gui();
      local_51 = 0;
      core::ptr::drop_in_place<winit::event_loop::EventLoop<tuple_<>_>_>();
      return;
    }
    uVar6 = (uint)uStack_180 >> 8;
    uStack_180 = CONCAT31((int3)uVar6,bVar8);
    local_188 = ppppppuVar1;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

