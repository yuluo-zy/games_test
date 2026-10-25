// find_best_fullscreen_video_mode
// sig: void __cdecl find_best_fullscreen_video_mode(void)
// size: 5090

void __cdecl tiny_glade::setup::find_best_fullscreen_video_mode(void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  char cVar4;
  char cVar5;
  char cVar6;
  char cVar7;
  char cVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  DWORD DVar22;
  DWORD DVar23;
  DWORD DVar24;
  longlong **pplVar25;
  undefined8 *puVar26;
  char extraout_AL;
  ushort uVar27;
  int iVar28;
  uint extraout_EAX;
  BOOL BVar29;
  int extraout_EAX_00;
  undefined4 extraout_EAX_01;
  longlong *extraout_RAX;
  void *_Dst;
  undefined8 *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong uVar30;
  longlong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  void *extraout_RAX_05;
  undefined8 *in_RCX;
  undefined8 *puVar31;
  ulonglong uVar32;
  undefined8 in_RDX;
  longlong lVar33;
  longlong *plVar34;
  longlong *plVar35;
  longlong *plVar36;
  void *_Dst_00;
  longlong lVar37;
  ulonglong uVar38;
  longlong lVar39;
  undefined8 *in_R8;
  DWORD iModeNum;
  size_t sVar40;
  uint uVar41;
  int iVar42;
  int iVar43;
  bool bVar44;
  byte bVar45;
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  char cVar48;
  char cVar49;
  char cVar50;
  char cVar51;
  undefined1 auVar52 [16];
  longlong *local_388;
  longlong *plStack_380;
  size_t local_378;
  longlong *plStack_370;
  longlong local_368;
  WCHAR aWStack_360 [12];
  undefined8 local_348;
  longlong *local_318;
  code *pcStack_310;
  undefined8 local_308;
  longlong local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined1 local_2d8 [16];
  undefined8 local_2c8;
  uint uStack_2c0;
  uint uStack_2bc;
  uint local_2b8;
  WCHAR aWStack_2b4 [4];
  uint uStack_2ac;
  uint local_2a8;
  uint uStack_2a4;
  WCHAR aWStack_2a0 [8];
  uint uStack_290;
  _union_682 _Stack_28c;
  undefined8 uStack_27c;
  WCHAR aWStack_274 [6];
  undefined1 local_268 [16];
  undefined1 local_258 [16];
  undefined1 local_248 [16];
  undefined8 local_238;
  DWORD DStack_230;
  DWORD DStack_22c;
  uint uStack_228;
  _union_685 _Stack_224;
  DWORD DStack_220;
  DWORD DStack_21c;
  undefined1 auStack_218 [12];
  DWORD local_20c;
  undefined1 auStack_208 [12];
  longlong *local_1f8;
  longlong *local_1f0;
  size_t local_1e8;
  undefined8 *local_1e0;
  ulonglong local_1d8;
  size_t local_1d0;
  longlong *local_1c8;
  ulonglong local_1c0;
  longlong *local_1b8;
  longlong *plStack_1b0;
  size_t local_1a8;
  longlong *plStack_1a0;
  undefined1 local_198 [9];
  WCHAR WStack_18f;
  uint uStack_18c;
  uint uStack_188;
  uint uStack_184;
  uint uStack_180;
  WCHAR WStack_17c;
  WCHAR WStack_17a;
  uint uStack_178;
  uint local_174;
  WCHAR aWStack_170 [4];
  uint uStack_168;
  uint uStack_164;
  uint uStack_160;
  WCHAR aWStack_15c [6];
  DWORD DStack_150;
  _union_682 _Stack_14c;
  DWORD DStack_13c;
  undefined8 uStack_138;
  WCHAR aWStack_130 [4];
  undefined1 local_128 [16];
  undefined1 local_118 [16];
  undefined1 local_108 [16];
  undefined1 local_f8 [16];
  undefined1 local_e8 [16];
  undefined1 auStack_d8 [12];
  DWORD local_cc;
  undefined1 auStack_c8 [12];
  undefined8 *local_b0;
  longlong local_a8;
  undefined8 *local_a0;
  undefined8 local_98;
  longlong *local_90;
  undefined1 local_85;
  undefined1 local_83;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  local_1e0 = in_RCX;
  local_90 = (longlong *)in_RDX;
  winit::platform_impl::platform::monitor::get_monitor_info();
  if (local_198._0_4_ == 1) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  uStack_27c._4_2_ = uStack_138._4_2_;
  uStack_27c._6_2_ = uStack_138._6_2_;
  aWStack_274[0] = aWStack_130[0];
  aWStack_274[1] = aWStack_130[1];
  _Stack_28c.field1.dmPosition.y = _Stack_14c.field1.dmDisplayOrientation;
  _Stack_28c.field1.dmDisplayOrientation = _Stack_14c.field1.dmDisplayFixedOutput;
  _Stack_28c.field1.dmDisplayFixedOutput = DStack_13c;
  uStack_27c._0_2_ = (short)uStack_138;
  uStack_27c._2_2_ = uStack_138._2_2_;
  aWStack_2a0[5] = aWStack_15c[5];
  aWStack_2a0[4] = aWStack_15c[4];
  aWStack_2a0[6] = (WCHAR)DStack_150;
  aWStack_2a0[7] = (WCHAR)(DStack_150 >> 0x10);
  uStack_290 = _Stack_14c.field1.dmPosition.x;
  _Stack_28c.field1.dmPosition.x = _Stack_14c.field1.dmPosition.y;
  uStack_2c0 = CONCAT22(WStack_17a,WStack_17c);
  local_2a8 = uStack_164;
  uStack_2a4 = uStack_160;
  aWStack_2a0[0] = aWStack_15c[0];
  aWStack_2a0[1] = aWStack_15c[1];
  aWStack_2a0[2] = aWStack_15c[2];
  aWStack_2a0[3] = aWStack_15c[3];
  local_2b8 = local_174;
  aWStack_2b4[0] = aWStack_170[0];
  aWStack_2b4[1] = aWStack_170[1];
  aWStack_2b4[2] = aWStack_170[2];
  aWStack_2b4[3] = aWStack_170[3];
  uStack_2ac = uStack_168;
  local_2c8._0_2_ = (undefined2)uStack_184;
  local_2c8._2_2_ = uStack_184._2_2_;
  local_2c8._4_4_ = uStack_180;
  uStack_2bc = uStack_178;
  local_2d8._0_4_ = local_198._4_4_;
  local_2d8._4_4_ = stack0xfffffffffffffe70;
  local_2d8._8_4_ = uStack_18c;
  local_2d8._12_4_ = uStack_188;
  winit::platform_impl::platform::util::decode_wide();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::os_str::wtf8::Slice::to_string_lossy();
  sVar40 = local_378;
  plVar34 = plStack_380;
  if ((longlong)local_378 < 0) {
LAB_140046717:
    local_90 = plVar34;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (local_378 == 0) {
    plVar35 = (longlong *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    plVar35 = extraout_RAX;
    if (extraout_RAX == (longlong *)0x0) goto LAB_140046717;
  }
  memcpy(plVar35,plVar34,sVar40);
  if (((ulonglong)local_388 & 0x7fffffffffffffff) == 0) {
    lVar39 = CONCAT44(local_198._4_4_,local_198._0_4_);
  }
  else {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    lVar39 = CONCAT44(local_198._4_4_,local_198._0_4_);
  }
  local_b0 = in_R8;
  if (lVar39 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_1d8 = sVar40;
  local_1e8 = sVar40;
  local_1b8 = (longlong *)0x0;
  local_1a8 = 0;
  local_83 = 1;
  local_1f8 = plVar35;
  local_1f0 = plVar35;
  winit::platform_impl::platform::monitor::get_monitor_info();
  if ((local_2d8._0_4_ & 1) == 0) {
    local_348 = CONCAT44(uStack_290,CONCAT22(aWStack_2a0[7],aWStack_2a0[6]));
    local_388 = (longlong *)CONCAT44(local_2d8._8_4_,local_2d8._4_4_);
    plStack_380 = (longlong *)CONCAT44((uint)local_2c8,local_2d8._12_4_);
    local_378 = CONCAT44(uStack_2c0,local_2c8._4_4_);
    plStack_370 = (longlong *)CONCAT44(local_2b8,uStack_2bc);
    local_368 = CONCAT44(aWStack_2b4._4_4_,aWStack_2b4._0_4_);
    iModeNum = 0;
    while( true ) {
      aWStack_2a0[4] = 0;
      aWStack_2a0[5] = 0;
      aWStack_2a0[7] = 0;
      uStack_290 = 0;
      _Stack_28c.field1.dmPosition.x = 0;
      auStack_208 = SUB1612((undefined1  [16])0x0,4);
      auStack_218 = SUB1612((undefined1  [16])0x0,0);
      local_20c = 0;
      uStack_228 = 0;
      _Stack_224.dmDisplayFlags = 0;
      DStack_220 = 0;
      DStack_21c = 0;
      local_238._0_2_ = L'\0';
      local_238._2_2_ = L'\0';
      local_238._4_2_ = L'\0';
      local_238._6_2_ = 0;
      DStack_230 = 0;
      DStack_22c = 0;
      local_248 = (undefined1  [16])0x0;
      local_258 = (undefined1  [16])0x0;
      local_268 = (undefined1  [16])0x0;
      uStack_27c._4_2_ = 0;
      uStack_27c._6_2_ = 0;
      aWStack_274[0] = L'\0';
      aWStack_274[1] = L'\0';
      aWStack_274[2] = L'\0';
      aWStack_274[3] = L'\0';
      aWStack_274[4] = L'\0';
      aWStack_274[5] = L'\0';
      _Stack_28c.field1.dmPosition.y = 0;
      _Stack_28c.field1.dmDisplayOrientation = 0;
      _Stack_28c.field1.dmDisplayFixedOutput = 0;
      uStack_27c._0_2_ = 0;
      uStack_27c._2_2_ = 0;
      local_2a8 = 0;
      uStack_2a4 = 0;
      aWStack_2a0[0] = L'\0';
      aWStack_2a0[1] = L'\0';
      aWStack_2a0[2] = L'\0';
      aWStack_2a0[3] = L'\0';
      local_2b8 = 0;
      aWStack_2b4[0] = L'\0';
      aWStack_2b4[1] = L'\0';
      aWStack_2b4[2] = L'\0';
      aWStack_2b4[3] = L'\0';
      uStack_2ac = 0;
      local_2c8._0_2_ = L'\0';
      local_2c8._2_2_ = L'\0';
      local_2c8._4_4_ = 0;
      uStack_2c0 = 0;
      uStack_2bc = 0;
      local_2d8._0_4_ = 0;
      local_2d8._4_4_ = 0;
      local_2d8._8_4_ = 0;
      local_2d8._12_4_ = 0;
      aWStack_2a0[6] = 0xdc;
      BVar29 = EnumDisplaySettingsExW(aWStack_360,iModeNum,(DEVMODEW *)local_2d8,0);
      DVar24 = DStack_220;
      uVar41 = uStack_228;
      DVar23 = DStack_22c;
      DVar22 = DStack_230;
      if (BVar29 == 0) break;
      if ((~uStack_290 & 0x5c0000) != 0) {
        local_83 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      memcpy(local_198,(DEVMODEW *)local_2d8,0xdc);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (void *)0x0) {
        local_83 = 1;
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      memcpy(_Dst,local_198,0xdc);
      uStack_180 = DVar24 * 1000;
      local_198._0_4_ = SUB84(_Dst,0);
      local_198._4_4_ = (undefined4)((ulonglong)_Dst >> 0x20);
      stack0xfffffffffffffe70 = DVar23;
      uStack_18c = uVar41;
      uStack_188 = (uint)local_90;
      uStack_184 = (uint)((ulonglong)local_90 >> 0x20);
      WStack_17c = (WCHAR)DVar22;
      local_83 = 1;
      alloc::collections::btree::map::BTreeMap::
      insert<winit::monitor::VideoMode,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
                ();
      iModeNum = iModeNum + 1;
    }
    local_198[0] = local_1b8 != (longlong *)0x0;
    plVar35 = plStack_1b0;
    sVar40 = 0;
    plVar34 = local_1b8;
    if ((bool)local_198[0]) {
      sVar40 = local_1a8;
    }
  }
  else {
    local_2f8 = CONCAT44(local_2d8._12_4_,local_2d8._8_4_);
    if (*(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E < 2) {
      sVar40 = 0;
      plVar34 = (longlong *)0x0;
      uVar41 = local_2d8._8_4_;
    }
    else {
      local_318 = &local_2f8;
      pcStack_310 = std::io::error::impl$9::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      _Stack_14c.field1.dmPosition.x = *(undefined4 *)(extraout_RAX_00 + 2);
      uStack_168 = 2;
      uStack_164 = 0;
      uStack_160 = 0x4292d710;
      aWStack_15c[0] = L'\x01';
      aWStack_15c[1] = L'\0';
      aWStack_15c[2] = L'\'';
      aWStack_15c[3] = L'\0';
      aWStack_15c[4] = 0;
      aWStack_15c[5] = 0;
      _Stack_14c.field1.dmPosition.y = 0x4292d6e8;
      _Stack_14c.field1.dmDisplayOrientation = 1;
      _Stack_14c.field1.dmDisplayFixedOutput = 1;
      DStack_13c = 0;
      uStack_138 = &local_318;
      aWStack_130[0] = L'\x01';
      aWStack_130[1] = L'\0';
      aWStack_130[2] = L'\0';
      aWStack_130[3] = L'\0';
      auVar46._8_8_ = 0;
      auVar46._0_8_ = local_128._8_8_;
      local_128 = auVar46 << 0x40;
      local_198._0_4_ = 0;
      local_198._4_4_ = 0;
      stack0xfffffffffffffe70 = 0x4292d710;
      uStack_18c = 1;
      uStack_188 = 0x27;
      uStack_184._0_2_ = L'\0';
      uStack_184._2_2_ = L'\0';
      uStack_180 = 0;
      WStack_17c = L'\0';
      WStack_17a = L'\0';
      uStack_178 = (uint)*extraout_RAX_00;
      local_174 = (uint)((ulonglong)*extraout_RAX_00 >> 0x20);
      aWStack_170._0_4_ = (undefined4)extraout_RAX_00[1];
      aWStack_170._4_4_ = (undefined4)((ulonglong)extraout_RAX_00[1] >> 0x20);
      DStack_150 = 1;
      ::log::__private_api::impl$2::log();
      plVar34 = local_1b8;
      plVar35 = plStack_1b0;
      sVar40 = local_1a8;
      uVar41 = (uint)local_2f8;
    }
    if ((uVar41 & 3) == 1) {
      local_a8 = local_2f8 + -1;
      local_98 = *(ulonglong *)(local_2f8 + -1);
      local_a0 = *(undefined8 **)(local_2f8 + 7);
      if ((code *)*local_a0 != (code *)0x0) {
        (*(code *)*local_a0)(local_98);
      }
      if (local_a0[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_198[0] = plVar34 != (longlong *)0x0;
    if (!(bool)local_198[0]) {
      sVar40 = 0;
    }
  }
  local_198._1_3_ = 0;
  local_198._4_4_ = 0;
  stack0xfffffffffffffe70 = 0;
  uStack_18c = 0;
  uStack_188 = (uint)plVar34;
  uStack_184 = (uint)((ulonglong)plVar34 >> 0x20);
  uStack_180 = (uint)plVar35;
  WStack_17c = (WCHAR)((ulonglong)plVar35 >> 0x20);
  WStack_17a = (WCHAR)((ulonglong)plVar35 >> 0x30);
  uStack_178 = (uint)(byte)local_198[0];
  local_174 = 0;
  aWStack_170[0] = L'\0';
  aWStack_170[1] = L'\0';
  aWStack_170[2] = L'\0';
  aWStack_170[3] = L'\0';
  aWStack_15c._0_4_ = (undefined4)((ulonglong)plVar35 >> 0x20);
  aWStack_15c._4_4_ = (undefined4)sVar40;
  aWStack_15c[4] = (WCHAR)(sVar40 >> 0x20);
  aWStack_15c[5] = (WCHAR)(sVar40 >> 0x30);
  uStack_168 = uStack_188;
  uStack_164 = uStack_184;
  uStack_160 = uStack_180;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<winit::monitor::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl_7::video_modes::closure_env_0>,winit::monitor::impl_5::video_modes::closure_env_0>_>
            ();
  if (local_1c0 < 2) {
    if (local_1c0 == 0) {
      if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_82 = 1;
        winit::platform_impl::platform::monitor::get_monitor_info();
        if ((local_198._0_4_ & 1) == 0) {
          plVar34 = (longlong *)CONCAT44(uStack_18c,stack0xfffffffffffffe70);
        }
        else {
          lVar39 = CONCAT44(uStack_18c,stack0xfffffffffffffe70);
          plVar34 = (longlong *)0x0;
          if ((stack0xfffffffffffffe70 & 3) == 1) {
            local_98 = lVar39 - 1;
            local_90 = *(longlong **)(lVar39 + -1);
            local_b0 = *(undefined8 **)(lVar39 + 7);
            if ((code *)*local_b0 != (code *)0x0) {
              (*(code *)*local_b0)(local_90);
            }
            if (local_b0[1] != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        local_2d8._0_8_ = &local_1f0;
        local_2d8._8_4_ = 0x405338a0;
        local_2d8._12_4_ = 1;
        local_2c8 = &local_388;
        uStack_2c0 = 0x40043630;
        uStack_2bc = 1;
        local_82 = 1;
        local_388 = plVar34;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        _Stack_14c.field1.dmPosition.x = *(undefined4 *)(extraout_RAX_03 + 2);
        uStack_168 = 2;
        uStack_164 = 0;
        uStack_160 = 0x4292d950;
        aWStack_15c[0] = L'\x01';
        aWStack_15c[1] = L'\0';
        aWStack_15c[2] = L'\x11';
        aWStack_15c[3] = L'\0';
        aWStack_15c[4] = 0;
        aWStack_15c[5] = 0;
        _Stack_14c.field1.dmPosition.y = 0x4292db18;
        _Stack_14c.field1.dmDisplayOrientation = 1;
        _Stack_14c.field1.dmDisplayFixedOutput = 3;
        DStack_13c = 0;
        uStack_138 = (longlong **)local_2d8;
        aWStack_130[0] = L'\x02';
        aWStack_130[1] = L'\0';
        aWStack_130[2] = L'\0';
        aWStack_130[3] = L'\0';
        auVar20._8_8_ = 0;
        auVar20._0_8_ = local_128._8_8_;
        local_128 = auVar20 << 0x40;
        local_198._0_4_ = 0;
        local_198._4_4_ = 0;
        stack0xfffffffffffffe70 = 0x4292d950;
        uStack_18c = 1;
        uStack_188 = 0x11;
        uStack_184 = 0;
        uStack_180 = 0;
        WStack_17c = L'\0';
        WStack_17a = L'\0';
        uStack_178 = (uint)*extraout_RAX_03;
        local_174 = (uint)((ulonglong)*extraout_RAX_03 >> 0x20);
        aWStack_170._0_4_ = (undefined4)extraout_RAX_03[1];
        aWStack_170._4_4_ = (undefined4)((ulonglong)extraout_RAX_03[1] >> 0x20);
        DStack_150 = 1;
        local_82 = 1;
        ::log::__private_api::impl$2::log();
      }
      uVar38 = local_1d8;
      *local_1e0 = 0;
      if (local_1d0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto joined_r0x000140045d7a;
    }
  }
  else if (local_1c0 < 0x15) {
    local_82 = 1;
    core::slice::sort::shared::smallsort::
    insertion_sort_shift_left<winit::monitor::VideoMode,bool_(*)(ref_<winit::monitor::VideoMode>,ref_<winit::monitor::VideoMode>)>
              ();
  }
  else {
    local_82 = 1;
    core::slice::sort::stable::
    driftsort_main<winit::monitor::VideoMode,bool_(*)(ref_<winit::monitor::VideoMode>,ref_<winit::monitor::VideoMode>),alloc::vec::Vec<winit::monitor::VideoMode,alloc::alloc::Global>_>
              ();
  }
  if (DAT_14362b040 != 2) {
    local_82 = 1;
    once_cell::imp::OnceCell::
    initialize<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<alloc::string::String,std::hash::random::RandomState>_>,once_cell::sync::impl_6::get_or_init::closure_env_0<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<alloc::string::String,std::hash::random::RandomState>_>,once_cell::sync::impl_11::force::closure_env_0<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<alloc::string::String,std::hash::random::RandomState>_>,std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<alloc::string::String,std::hash::random::RandomState>_>_(*)()>_>,enum2_<once_cell::sync::impl_6::get_or_init::Void>_>
              ();
  }
  LOCK();
  bVar44 = DAT_14362b008 == '\0';
  if (bVar44) {
    DAT_14362b008 = '\x01';
  }
  UNLOCK();
  if (!bVar44) {
    local_82 = 1;
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    local_98 = local_98 & 0xffffffff00000000;
    plVar34 = local_1f0;
    sVar40 = local_1e8;
  }
  else {
    local_82 = 1;
    std::panicking::panic_count::is_zero_slow_path();
    local_98 = CONCAT44(local_98._4_4_,extraout_EAX_01) ^ 1;
    plVar34 = local_1f0;
    sVar40 = local_1e8;
  }
  local_1f0 = plVar34;
  local_1e8 = sVar40;
  if (DAT_14362b009 != '\0') {
    local_198._0_2_ = L'뀈';
    local_198._2_2_ = L'䍢';
    local_198._4_2_ = L'\x01';
    local_198._6_2_ = L'\0';
    _WStack_18f = (undefined3)(unique0x10001fdc >> 8);
    local_198[8] = (char)local_98;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  puVar26 = local_a0;
  if (DAT_14362b028 != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    uVar32 = DAT_14362b018;
    lVar39 = DAT_14362b010;
    bVar45 = (byte)(extraout_RAX_01 >> 0x39);
    auVar46 = ZEXT216(CONCAT11(bVar45,bVar45));
    auVar46 = pshuflw(auVar46,auVar46,0);
    puVar31 = (undefined8 *)0x0;
    uVar38 = extraout_RAX_01;
    while( true ) {
      uVar38 = uVar32 & uVar38;
      pcVar1 = (char *)(lVar39 + uVar38);
      cVar4 = *pcVar1;
      cVar5 = pcVar1[1];
      cVar6 = pcVar1[2];
      cVar7 = pcVar1[3];
      cVar8 = pcVar1[4];
      cVar9 = pcVar1[5];
      cVar10 = pcVar1[6];
      cVar11 = pcVar1[7];
      cVar12 = pcVar1[8];
      cVar13 = pcVar1[9];
      cVar14 = pcVar1[10];
      cVar15 = pcVar1[0xb];
      cVar16 = pcVar1[0xc];
      cVar17 = pcVar1[0xd];
      cVar18 = pcVar1[0xe];
      cVar19 = pcVar1[0xf];
      cVar48 = auVar46[0];
      auVar47[0] = -(cVar4 == cVar48);
      cVar49 = auVar46[1];
      auVar47[1] = -(cVar5 == cVar49);
      cVar50 = auVar46[2];
      auVar47[2] = -(cVar6 == cVar50);
      cVar51 = auVar46[3];
      auVar47[3] = -(cVar7 == cVar51);
      auVar47[4] = -(cVar8 == cVar48);
      auVar47[5] = -(cVar9 == cVar49);
      auVar47[6] = -(cVar10 == cVar50);
      auVar47[7] = -(cVar11 == cVar51);
      auVar47[8] = -(cVar12 == cVar48);
      auVar47[9] = -(cVar13 == cVar49);
      auVar47[10] = -(cVar14 == cVar50);
      auVar47[0xb] = -(cVar15 == cVar51);
      auVar47[0xc] = -(cVar16 == cVar48);
      auVar47[0xd] = -(cVar17 == cVar49);
      auVar47[0xe] = -(cVar18 == cVar50);
      auVar47[0xf] = -(cVar19 == cVar51);
      uVar27 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf;
      uVar41 = (uint)uVar27;
      puVar26 = local_a0;
      while (local_a0 = puVar31, uVar27 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar41; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        uVar30 = uVar2 + uVar38 & uVar32;
        if (*(size_t *)(lVar39 + -8 + uVar30 * -0x18) == sVar40) {
          iVar28 = memcmp(plVar34,*(void **)(lVar39 + uVar30 * -0x18 + -0x10),sVar40);
          if (iVar28 == 0) goto LAB_140046014;
        }
        uVar27 = (ushort)(uVar41 - 1) & (ushort)uVar41;
        uVar41 = CONCAT22((short)(uVar41 - 1 >> 0x10),uVar27);
        puVar31 = local_a0;
        puVar26 = local_a0;
      }
      auVar52[0] = -(cVar4 == -1);
      auVar52[1] = -(cVar5 == -1);
      auVar52[2] = -(cVar6 == -1);
      auVar52[3] = -(cVar7 == -1);
      auVar52[4] = -(cVar8 == -1);
      auVar52[5] = -(cVar9 == -1);
      auVar52[6] = -(cVar10 == -1);
      auVar52[7] = -(cVar11 == -1);
      auVar52[8] = -(cVar12 == -1);
      auVar52[9] = -(cVar13 == -1);
      auVar52[10] = -(cVar14 == -1);
      auVar52[0xb] = -(cVar15 == -1);
      auVar52[0xc] = -(cVar16 == -1);
      auVar52[0xd] = -(cVar17 == -1);
      auVar52[0xe] = -(cVar18 == -1);
      auVar52[0xf] = -(cVar19 == -1);
      if ((((((((((((((((SUB161(auVar52 >> 7,0) & 1) != 0 || (SUB161(auVar52 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar52 >> 0x17,0) & 1) != 0) || (SUB161(auVar52 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar52 >> 0x27,0) & 1) != 0) || (SUB161(auVar52 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar52 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar52 >> 0x3f,0) & 1) != 0) || (SUB161(auVar52 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar52 >> 0x4f,0) & 1) != 0) || (SUB161(auVar52 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar52 >> 0x5f,0) & 1) != 0) || (SUB161(auVar52 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar52 >> 0x6f,0) & 1) != 0) || (SUB161(auVar52 >> 0x77,0) & 1) != 0) ||
          auVar52[0xf] < '\0') break;
      uVar38 = (longlong)local_a0 + uVar38 + 0x10;
      puVar31 = local_a0 + 2;
      local_a0 = puVar26;
    }
  }
  local_a0 = puVar26;
  uVar38 = local_1c0;
  if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    if (local_1c0 == 0) {
      lVar39 = 8;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      lVar39 = extraout_RAX_02;
      if (extraout_RAX_02 == 0) {
        local_81 = 1;
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
    }
    local_2d8._0_4_ = (undefined4)uVar38;
    local_2d8._4_4_ = (undefined4)(uVar38 >> 0x20);
    local_2d8._8_4_ = (undefined4)lVar39;
    local_2d8._12_4_ = (undefined4)((ulonglong)lVar39 >> 0x20);
    local_198._0_8_ = &local_2c8;
    local_2c8._0_2_ = L'\0';
    local_2c8._2_2_ = L'\0';
    local_2c8._4_2_ = L'\0';
    local_2c8._6_2_ = L'\0';
    stack0xfffffffffffffe70 = L'\0';
    stack0xfffffffffffffe72 = L'\0';
    uStack_18c._0_2_ = L'\0';
    uStack_18c._2_2_ = L'\0';
    uStack_188 = local_2d8._8_4_;
    uStack_184 = local_2d8._12_4_;
    core::iter::adapters::map::impl$2::
    fold<alloc::string::String,core::slice::iter::Iter<winit::monitor::VideoMode>,alloc::string::String_(*)(ref_<winit::monitor::VideoMode>),tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<alloc::string::String,alloc::vec::impl_20::extend_trusted::closure_env_0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<winit::monitor::VideoMode>,alloc::string::String_(*)(ref_<winit::monitor::VideoMode>)>_>_>_>
              ();
    local_388 = (longlong *)CONCAT44(local_2d8._4_4_,local_2d8._0_4_);
    plVar34 = (longlong *)CONCAT44(local_2d8._12_4_,local_2d8._8_4_);
    sVar40 = CONCAT44(local_2c8._4_4_,(uint)local_2c8);
    plStack_380 = plVar34;
    local_378 = sVar40;
    alloc::str::join_generic_copy<str_,u8,alloc::string::String>();
    local_2d8._0_8_ = &local_1f0;
    local_2d8._8_4_ = 0x40109e90;
    local_2d8._12_4_ = 1;
    uStack_2c0 = 0x4003bad0;
    uStack_2bc = 1;
    local_2c8 = &local_1b8;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    _Stack_14c.field1.dmPosition.x = *(undefined4 *)(extraout_RAX_04 + 2);
    uStack_168 = 4;
    uStack_164 = 0;
    uStack_160 = 0x4292d950;
    aWStack_15c[0] = L'\x01';
    aWStack_15c[1] = L'\0';
    aWStack_15c[2] = L'\x11';
    aWStack_15c[3] = L'\0';
    aWStack_15c[4] = 0;
    aWStack_15c[5] = 0;
    _Stack_14c.field1.dmPosition.y = 0x4292da78;
    _Stack_14c.field1.dmDisplayOrientation = 1;
    _Stack_14c.field1.dmDisplayFixedOutput = 3;
    DStack_13c = 0;
    uStack_138 = (longlong **)local_2d8;
    aWStack_130[0] = L'\x02';
    aWStack_130[1] = L'\0';
    aWStack_130[2] = L'\0';
    aWStack_130[3] = L'\0';
    auVar21._8_8_ = 0;
    auVar21._0_8_ = local_128._8_8_;
    local_128 = auVar21 << 0x40;
    local_198._0_4_ = 0;
    local_198._4_4_ = 0;
    stack0xfffffffffffffe70 = 0x4292d950;
    uStack_18c = 1;
    uStack_188 = 0x11;
    uStack_184 = 0;
    uStack_180 = 0;
    WStack_17c = L'\0';
    WStack_17a = L'\0';
    uStack_178 = (uint)*extraout_RAX_04;
    local_174 = (uint)((ulonglong)*extraout_RAX_04 >> 0x20);
    aWStack_170._0_4_ = (undefined4)extraout_RAX_04[1];
    aWStack_170._4_4_ = (undefined4)((ulonglong)extraout_RAX_04[1] >> 0x20);
    DStack_150 = 1;
    ::log::__private_api::impl$2::log();
    if (local_1b8 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (sVar40 != 0) {
      plVar34 = plVar34 + 1;
      do {
        if (plVar34[-1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        plVar34 = plVar34 + 3;
        sVar40 = sVar40 - 1;
      } while (sVar40 != 0);
    }
    plVar34 = local_1f0;
    sVar40 = local_1e8;
    if (local_388 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      plVar34 = local_1f0;
      sVar40 = local_1e8;
    }
  }
  if ((longlong)sVar40 < 0) {
LAB_14004676b:
    local_81 = 1;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (sVar40 == 0) {
    _Dst_00 = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst_00 = extraout_RAX_05;
    if (extraout_RAX_05 == (void *)0x0) goto LAB_14004676b;
  }
  memcpy(_Dst_00,plVar34,sVar40);
  local_198._0_4_ = (undefined4)sVar40;
  local_198._4_4_ = (undefined4)(sVar40 >> 0x20);
  stack0xfffffffffffffe70 = (uint)_Dst_00;
  uStack_18c = (uint)((ulonglong)_Dst_00 >> 0x20);
  local_81 = 1;
  uStack_188 = local_198._0_4_;
  uStack_184 = local_198._4_4_;
  hashbrown::map::HashMap::
  insert<alloc::string::String,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>();
LAB_140046014:
  puVar26 = local_b0;
  if (*(int *)(local_b0 + 6) == 0) {
    local_81 = 1;
    winit::platform_impl::platform::monitor::get_monitor_info();
    pplVar25 = uStack_138;
    if (local_198._0_4_ == 1) {
      lVar39 = CONCAT44(uStack_18c,stack0xfffffffffffffe70);
      if ((stack0xfffffffffffffe70 & 3) == 1) {
        local_a8 = lVar39 + -1;
        local_90 = *(longlong **)(lVar39 + -1);
        local_a0 = *(undefined8 **)(lVar39 + 7);
        if ((code *)*local_a0 != (code *)0x0) {
          (*(code *)*local_a0)(local_90);
        }
        if (local_a0[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      aWStack_274[0] = aWStack_130[0];
      aWStack_274[1] = aWStack_130[1];
      _Stack_28c.field1.dmPosition.y = _Stack_14c.field1.dmDisplayOrientation;
      _Stack_28c.field1.dmDisplayOrientation = _Stack_14c.field1.dmDisplayFixedOutput;
      _Stack_28c.field1.dmDisplayFixedOutput = DStack_13c;
      aWStack_2a0[5] = aWStack_15c[5];
      aWStack_2a0[4] = aWStack_15c[4];
      aWStack_2a0[6] = (WCHAR)DStack_150;
      aWStack_2a0[7] = (WCHAR)(DStack_150 >> 0x10);
      uStack_290 = _Stack_14c.field1.dmPosition.x;
      _Stack_28c.field1.dmPosition.x = _Stack_14c.field1.dmPosition.y;
      uStack_2c0 = CONCAT22(WStack_17a,WStack_17c);
      local_2a8 = uStack_164;
      uStack_2a4 = uStack_160;
      aWStack_2a0[0] = aWStack_15c[0];
      aWStack_2a0[1] = aWStack_15c[1];
      aWStack_2a0[2] = aWStack_15c[2];
      aWStack_2a0[3] = aWStack_15c[3];
      local_2b8 = local_174;
      aWStack_2b4[0] = aWStack_170[0];
      aWStack_2b4[1] = aWStack_170[1];
      aWStack_2b4[2] = aWStack_170[2];
      aWStack_2b4[3] = aWStack_170[3];
      uStack_2ac = uStack_168;
      local_2c8._0_4_ = uStack_184;
      local_2c8._4_4_ = uStack_180;
      uStack_2bc = uStack_178;
      local_2d8._0_4_ = local_198._4_4_;
      local_2d8._4_4_ = stack0xfffffffffffffe70;
      local_2d8._8_4_ = uStack_18c;
      local_2d8._12_4_ = uStack_188;
      aWStack_15c[2] = L'\0';
      aWStack_15c[3] = L'\0';
      aWStack_15c[5] = 0;
      DStack_150 = 0;
      _Stack_14c.field1.dmPosition.x = 0;
      auStack_c8 = SUB1612((undefined1  [16])0x0,4);
      auStack_d8 = SUB1612((undefined1  [16])0x0,0);
      local_cc = 0;
      local_e8 = (undefined1  [16])0x0;
      local_f8 = (undefined1  [16])0x0;
      local_108 = (undefined1  [16])0x0;
      local_118 = (undefined1  [16])0x0;
      local_128 = (undefined1  [16])0x0;
      uStack_138._0_2_ = 0;
      uStack_138._2_2_ = 0;
      uStack_138._4_2_ = 0;
      uStack_138._6_2_ = L'\0';
      aWStack_130[0] = L'\0';
      aWStack_130[1] = L'\0';
      aWStack_130[2] = L'\0';
      aWStack_130[3] = L'\0';
      _Stack_14c.field1.dmPosition.y = 0;
      _Stack_14c.field1.dmDisplayOrientation = 0;
      _Stack_14c.field1.dmDisplayFixedOutput = 0;
      DStack_13c = 0;
      uStack_168 = 0;
      uStack_164 = 0;
      uStack_160 = 0;
      aWStack_15c[0] = L'\0';
      aWStack_15c[1] = L'\0';
      uStack_178 = 0;
      local_174 = 0;
      aWStack_170[0] = L'\0';
      aWStack_170[1] = L'\0';
      aWStack_170[2] = L'\0';
      aWStack_170[3] = L'\0';
      uStack_188 = 0;
      uStack_184 = 0;
      uStack_180 = 0;
      WStack_17c = L'\0';
      WStack_17a = L'\0';
      local_198._0_4_ = 0;
      local_198._4_4_ = 0;
      stack0xfffffffffffffe70 = 0;
      uStack_18c = 0;
      aWStack_15c[4] = 0xdc;
      uStack_27c = pplVar25;
      BVar29 = EnumDisplaySettingsExW(aWStack_2b4 + 2,0xffffffff,(DEVMODEW *)local_198,0);
      if (BVar29 != 0) {
        iVar28 = local_e8._8_4_ * 1000;
        local_81 = 1;
        plVar34 = local_90;
        winit::platform_impl::platform::monitor::get_monitor_info();
        if (local_198._0_4_ == 1) {
          local_2d8._0_4_ = stack0xfffffffffffffe70;
          local_2d8._4_4_ = uStack_18c;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        if (local_1c0 != 0) {
          iVar42 = uStack_188 - stack0xfffffffffffffe70;
          iVar43 = uStack_184 - uStack_18c;
          plVar36 = local_1c8 + local_1c0 * 4;
          plVar35 = local_1c8;
          do {
            if (iVar28 == (int)plVar35[3]) {
              local_81 = 1;
              winit::platform_impl::platform::monitor::VideoMode::size();
              if ((extraout_EAX_00 == iVar42) && ((int)plVar34 == iVar43)) {
                local_81 = 1;
                impl$1::from_winit();
                local_2d8._0_8_ = CONCAT44(local_198._4_4_,local_198._0_4_);
                local_2d8._8_4_ = stack0xfffffffffffffe70;
                local_2d8._12_4_ = uStack_18c;
                *(undefined4 *)(puVar26 + 6) = 1;
                *(ulonglong *)((longlong)puVar26 + 0x34) = CONCAT44(local_198._4_4_,local_198._0_4_)
                ;
                *(ulonglong *)((longlong)puVar26 + 0x3c) =
                     CONCAT44(uStack_18c,stack0xfffffffffffffe70);
                break;
              }
            }
            plVar35 = plVar35 + 4;
          } while (plVar35 != plVar36);
        }
      }
    }
  }
  if ((*(byte *)(puVar26 + 6) & 1) != 0) {
    plStack_370 = local_1c8 + local_1c0 * 4;
    local_368 = (longlong)puVar26 + 0x34;
    local_388 = local_1c8;
    plStack_380 = local_1c8;
    local_378 = local_1d0;
    if (local_1c0 != 0) {
      plStack_380 = local_1c8 + 4;
      lVar39 = *local_1c8;
      if (lVar39 != 0) {
        sVar40 = local_1c8[3];
        local_1b8 = (longlong *)local_1c8[1];
        plStack_1b0 = (longlong *)local_1c8[2];
        local_198._0_4_ = (undefined4)lVar39;
        local_198._4_4_ = (undefined4)((ulonglong)lVar39 >> 0x20);
        stack0xfffffffffffffe70 = (uint)local_1b8;
        uStack_18c = (uint)((ulonglong)local_1b8 >> 0x20);
        uStack_188 = (uint)plStack_1b0;
        uStack_184 = (uint)((ulonglong)plStack_1b0 >> 0x20);
        uStack_180 = (uint)sVar40;
        WStack_17c = (WCHAR)(sVar40 >> 0x20);
        WStack_17a = (WCHAR)(sVar40 >> 0x30);
        local_1a8 = sVar40;
        winit::platform_impl::platform::monitor::VideoMode::size();
        uVar41 = (uint)sVar40;
        winit::platform_impl::platform::monitor::VideoMode::size();
        local_318 = (longlong *)CONCAT44(uStack_18c,stack0xfffffffffffffe70);
        pcStack_310 = (code *)CONCAT44(uStack_184,uStack_188);
        local_308 = CONCAT26(WStack_17a,CONCAT24(WStack_17c,uStack_180));
        if (CONCAT44(local_198._4_4_,local_198._0_4_) != 0) {
          lVar39 = -((ulonglong)(ushort)WStack_17c - (ulonglong)*(ushort *)(puVar26 + 8));
          if (lVar39 < 0) {
            lVar39 = (ulonglong)(ushort)WStack_17c - (ulonglong)*(ushort *)(puVar26 + 8);
          }
          lVar37 = (ulonglong)extraout_EAX * (ulonglong)uVar41 -
                   (ulonglong)*(uint *)(puVar26 + 7) *
                   (ulonglong)*(uint *)((longlong)puVar26 + 0x34);
          lVar33 = -lVar37;
          if (lVar33 < 0) {
            lVar33 = lVar37;
          }
          uVar32 = (ulonglong)uStack_180 - (ulonglong)*(uint *)((longlong)puVar26 + 0x3c);
          uVar38 = -uVar32;
          if ((longlong)uVar38 < 0) {
            uVar38 = uVar32;
          }
          local_2d8._0_8_ = (uVar38 & 0xffffffff) / 10 + lVar33 + lVar39 * 1000;
          local_2d8._8_4_ = local_198._0_4_;
          local_2d8._12_4_ = local_198._4_4_;
          local_2c8._0_4_ = stack0xfffffffffffffe70;
          local_2c8._4_4_ = uStack_18c;
          uStack_2c0 = uStack_188;
          uStack_2bc = uStack_184;
          local_2b8 = uStack_180;
          aWStack_2b4._0_4_ = (undefined4)((ulonglong)local_308 >> 0x20);
          local_1a8 = local_378;
          plStack_1a0 = plStack_370;
          local_1b8 = local_388;
          plStack_1b0 = plStack_380;
          local_85 = 0;
          alloc::vec::into_iter::impl$5::
          fold<winit::monitor::VideoMode,alloc::alloc::Global,tuple_<i64,winit::monitor::VideoMode>,core::iter::adapters::map::map_fold::closure_env_0<winit::monitor::VideoMode,tuple_<i64,winit::monitor::VideoMode>,tuple_<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env_0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env_1>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env_0<tuple_<i64,winit::monitor::VideoMode>,core::cmp::Ordering_(*)(ref_<tuple_<i64,winit::monitor::VideoMode>_>,ref_<tuple_<i64,winit::monitor::VideoMode>_>)>_>_>
                    ();
          local_1c8 = (longlong *)CONCAT44(local_198._4_4_,local_198._0_4_);
          plVar34 = (longlong *)CONCAT44(uStack_18c,stack0xfffffffffffffe70);
          local_2f8 = CONCAT44(uStack_184,uStack_188);
          uStack_2f0 = CONCAT26(WStack_17a,CONCAT24(WStack_17c,uStack_180));
          local_2e8 = CONCAT44(local_174,uStack_178);
          if (plVar34 == (longlong *)0x0) goto LAB_140046745;
          local_2c8._0_4_ = uStack_178;
          local_2c8._4_4_ = local_174;
          local_2d8._0_4_ = uStack_188;
          local_2d8._4_4_ = uStack_184;
          local_2d8._8_4_ = uStack_180;
          local_2d8._12_4_ = (undefined4)((ulonglong)uStack_2f0 >> 0x20);
          unique0x10002e79 = plVar34;
          goto LAB_1400462b6;
        }
      }
    }
    local_81 = 0;
    core::ptr::
    drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env_0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env_1>_>_>
              ();
LAB_140046745:
    local_81 = 0;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  plVar34 = local_1c8 + local_1c0 * 4;
  uStack_188 = (uint)local_1d0;
  uStack_184 = (uint)(local_1d0 >> 0x20);
  uStack_180 = (uint)plVar34;
  WStack_17c = (WCHAR)((ulonglong)plVar34 >> 0x20);
  WStack_17a = (WCHAR)((ulonglong)plVar34 >> 0x30);
  unique0x10002e61 = local_1c8;
  if (local_1c0 == 0) {
LAB_14004675a:
    local_198._0_8_ = local_1c8;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  unique0x00008e00 = local_1c8 + 4;
  plVar34 = (longlong *)*local_1c8;
  if (plVar34 == (longlong *)0x0) goto LAB_14004675a;
  local_2c8._0_4_ = (uint)local_1c8[3];
  local_2c8._4_4_ = (uint)((ulonglong)local_1c8[3] >> 0x20);
  local_2d8._0_4_ = (undefined4)local_1c8[1];
  local_2d8._4_4_ = (undefined4)((ulonglong)local_1c8[1] >> 0x20);
  local_2d8._8_4_ = (undefined4)local_1c8[2];
  local_2d8._12_4_ = (undefined4)((ulonglong)local_1c8[2] >> 0x20);
  if (local_1c0 != 1) {
    uVar38 = local_1c0 * 0x20 - 0x20 >> 5;
    do {
      local_198._0_8_ = local_1c8;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      uVar38 = uVar38 - 1;
      local_1c8 = (longlong *)local_198._0_8_;
    } while (uVar38 != 0);
  }
  if (local_1d0 != 0) {
    local_198._0_8_ = local_1c8;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    local_1c8 = (longlong *)local_198._0_8_;
  }
LAB_1400462b6:
  local_1e0[3] = CONCAT44(local_2c8._4_4_,(uint)local_2c8);
  local_1e0[1] = CONCAT44(local_2d8._4_4_,local_2d8._0_4_);
  local_1e0[2] = CONCAT44(local_2d8._12_4_,local_2d8._8_4_);
  *local_1e0 = plVar34;
  if (((char)local_98 == '\0') &&
     ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) != 0)) {
    local_82 = 0;
    local_198._0_8_ = local_1c8;
    std::panicking::panic_count::is_zero_slow_path();
    local_1c8 = (longlong *)local_198._0_8_;
    if (extraout_AL == '\0') {
      DAT_14362b009 = '\x01';
    }
  }
  cVar4 = DAT_14362b008;
  LOCK();
  DAT_14362b008 = '\0';
  UNLOCK();
  if (cVar4 == '\x02') {
    local_82 = 0;
    local_198._0_8_ = local_1c8;
    std::sys::sync::mutex::futex::Mutex::wake();
    local_1c8 = (longlong *)local_198._0_8_;
  }
  local_198._0_8_ = local_1c8;
  uVar38 = local_1d8;
joined_r0x000140045d7a:
  if ((uVar38 & 0x7fffffffffffffff) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

