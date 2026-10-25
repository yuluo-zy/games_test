// write_game_config_to_disk
// sig: void __cdecl write_game_config_to_disk(void)
// size: 693

void __cdecl tiny_glade::app_config::write_game_config_to_disk(void)

{
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong local_150 [4];
  longlong *local_130;
  code *pcStack_128;
  longlong *local_120;
  code *pcStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  longlong *local_e8;
  code *pcStack_e0;
  longlong *local_d8;
  code *pcStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined **local_98;
  undefined8 local_90;
  longlong **local_88;
  undefined8 local_80;
  undefined8 local_78;
  longlong local_68 [5];
  longlong *local_40;
  code *local_38;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  paths::config();
  std::path::Path::_join();
  ron::ser::impl$1::default();
  local_68[2] = 0;
  local_68[0] = 1;
  local_68[1] = 0x80;
  ron::options::Options::to_string_pretty<country_core::game_config::GameConfig>();
  if ((char)local_130 == '2') {
    local_38 = pcStack_128;
    local_40 = local_120;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::fs::write::inner();
    if (local_38 != (code *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (extraout_RAX != 0) {
      local_68[0] = extraout_RAX;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        pcStack_128 = std::path::impl$44::fmt;
        pcStack_118 = std::io::error::impl$0::fmt;
        local_130 = local_150;
        local_120 = local_68;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_c8 = *extraout_RAX_00;
        uStack_c0 = extraout_RAX_00[1];
        local_9c = *(undefined4 *)(extraout_RAX_00 + 2);
        local_b8 = 1;
        uStack_b0 = 0x1429c3950;
        local_a8 = 0x16;
        local_98 = &anon_6e172bbc46dc6f6cae4784a1bafa39bc_101_llvm_16494194562652537923;
        local_90 = 2;
        local_80 = 2;
        local_78 = 0;
        local_e8 = (longlong *)0x0;
        pcStack_e0 = (code *)0x1429c3950;
        local_d8 = (longlong *)0x16;
        pcStack_d0 = (code *)0x0;
        local_a0 = 1;
        local_88 = &local_130;
        ::log::__private_api::impl$2::log();
      }
      if (((uint)local_68[0] & 3) == 1) {
        local_68[3] = local_68[0] + -1;
        local_40 = *(longlong **)(local_68[0] + -1);
        local_38 = *(code **)(local_68[0] + 7);
        if (*(code **)local_38 != (code *)0x0) {
          (**(code **)local_38)(local_40);
        }
        if (*(longlong *)(local_38 + 8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    if (local_150[0] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
  local_a8 = local_f0;
  local_b8 = local_100;
  uStack_b0 = uStack_f8;
  local_c8 = local_110;
  uStack_c0 = uStack_108;
  local_d8 = local_120;
  pcStack_d0 = pcStack_118;
  local_e8 = local_130;
  pcStack_e0 = pcStack_128;
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

