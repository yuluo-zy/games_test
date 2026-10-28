
void __cdecl rhapsody::raw::device::GraphicsDevice::validate_swapchain_and_acquire_image(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  ulonglong uVar1;
  undefined8 *in_RCX;
  undefined8 *in_R8;
  ulonglong uVar2;
  undefined8 local_180;
  undefined8 uStack_178;
  ulonglong local_170;
  ulonglong local_168;
  undefined8 local_160;
  undefined4 *local_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined6 local_128;
  char local_122;
  undefined1 uStack_121;
  undefined8 local_120;
  longlong lStack_118;
  undefined4 **ppuStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  longlong lStack_f0;
  undefined8 uStack_e8;
  undefined4 **ppuStack_98;
  code *pcStack_90;
  undefined8 *local_88;
  ulonglong uStack_80;
  undefined4 *puStack_78;
  code *pcStack_70;
  undefined4 ***pppuStack_68;
  code *pcStack_60;
  ulonglong uStack_58;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  swapchain::Swapchain::acquire_next_image();
  if (local_122 != '\a') {
    in_RCX[0xc] = local_120;
    in_RCX[10] = uStack_130;
    in_RCX[0xb] = CONCAT17(uStack_121,CONCAT16(local_122,local_128));
    in_RCX[8] = uStack_140;
    in_RCX[9] = uStack_138;
    in_RCX[6] = uStack_150;
    in_RCX[7] = uStack_148;
    in_RCX[4] = local_160;
    in_RCX[5] = local_158;
    in_RCX[2] = local_170;
    in_RCX[3] = local_168;
    *in_RCX = local_180;
    in_RCX[1] = uStack_178;
    return;
  }
  uVar1 = local_168 & 0xffffffff;
  local_88 = in_R8;
  switch(local_180) {
  case 2:
    goto code_r0x000140bfda60;
  default:
    puStack_78 = local_158;
    local_160._0_4_ = (undefined4)local_168;
    uStack_80 = uVar1;
    if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      uStack_58 = local_170;
      ppuStack_98 = &puStack_78;
      pcStack_90 = alloc::string::impl$23::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      uStack_f8 = *extraout_RAX_00;
      lStack_f0 = extraout_RAX_00[1];
      uStack_e8 = 4;
      lStack_118 = 0;
      ppuStack_110 = (undefined4 **)&DAT_142b16d70;
      uStack_108 = 0x18;
      uStack_100 = 0;
      ::log::__private_api::impl$2::log();
      local_160._0_4_ = (undefined4)uStack_80;
      local_170 = uStack_58;
    }
    uStack_178 = local_180;
    if (puStack_78 != (undefined4 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      local_160._0_4_ = (undefined4)uStack_80;
    }
    break;
  case 4:
    if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      uStack_80 = uVar1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      uStack_f8 = *extraout_RAX;
      lStack_f0 = extraout_RAX[1];
      uStack_e8 = 4;
      lStack_118 = 0;
      ppuStack_110 = (undefined4 **)&DAT_142b16d70;
      uStack_108 = 0x18;
      uStack_100 = 0;
      ::log::__private_api::impl$2::log();
      uVar1 = uStack_80;
    }
    local_170 = local_168 & 0xffffffff00000000 | uVar1;
    break;
  case 5:
    uStack_4c = (undefined4)local_168;
    puStack_78 = &uStack_4c;
    pcStack_70 = ash::vk::enums::impl$253::fmt;
    pppuStack_68 = &ppuStack_98;
    pcStack_60 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  create_swapchain();
  if (lStack_118 == 6) {
    managed::handle::impl$6::drop();
    local_88[2] = uStack_100;
    local_88[3] = uStack_f8;
    *local_88 = ppuStack_110;
    local_88[1] = uStack_108;
    swapchain::Swapchain::acquire_next_image();
    switch(uStack_178) {
    case 2:
code_r0x000140bfda60:
      *(undefined1 *)((longlong)in_RCX + 0x5e) = 7;
      return;
    default:
      puStack_78 = local_158;
      if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        ppuStack_98 = &puStack_78;
        pcStack_90 = alloc::string::impl$23::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        ::log::__private_api::impl$2::log();
      }
      *(undefined1 *)((longlong)in_RCX + 0x5e) = 7;
      if (puStack_78 == (undefined4 *)0x0) {
        return;
      }
      break;
    case 4:
      log_gpu_crash_info();
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    case 5:
      pcStack_90 = (code *)local_170;
      uStack_4c = (undefined4)local_160;
      log_gpu_crash_info();
      puStack_78 = &uStack_4c;
      pcStack_70 = ash::vk::enums::impl$253::fmt;
      pppuStack_68 = &ppuStack_98;
      pcStack_60 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
  }
  else {
    uVar2 = lStack_118 - 2;
    uVar1 = 1;
    if (uVar2 < 4) {
      uVar1 = uVar2;
    }
    if (1 < uVar1) {
      uStack_58 = uStack_e8;
      local_88 = (undefined8 *)lStack_f0;
      uStack_80 = uVar2;
      if (uVar1 != 2) {
        uStack_4c = (undefined4)uStack_100;
        ppuStack_98 = ppuStack_110;
        pcStack_90 = (code *)uStack_108;
        log_gpu_crash_info();
        puStack_78 = &uStack_4c;
        pcStack_70 = ash::vk::enums::impl$253::fmt;
        pppuStack_68 = &ppuStack_98;
        pcStack_60 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
        ;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      log_gpu_crash_info();
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    *(undefined1 *)((longlong)in_RCX + 0x5e) = 7;
    if (lStack_f0 == 0 || uVar2 != 1 && uVar2 < 4) {
      return;
    }
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}

