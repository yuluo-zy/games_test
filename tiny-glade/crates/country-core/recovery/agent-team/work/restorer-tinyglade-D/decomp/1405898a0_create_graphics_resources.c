// 1405898a0 void __cdecl create_graphics_resources(void)

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources(void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  longlong lVar3;
  bool bVar4;
  uint local_158;
  undefined4 uStack_154;
  longlong lStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  longlong lStack_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 uStack_120;
  char cStack_d2;
  
  rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu();
  rhapsody::raw::device::GraphicsDevice::create_mapped_buffer();
  if (local_158 == 0xb) {
    LOCK();
    bVar4 = DAT_14362b0f0 == '\0';
    if (bVar4) {
      DAT_14362b0f0 = '\x01';
    }
    UNLOCK();
    if (!bVar4) {
      parking_lot::raw_mutex::RawMutex::lock_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    uVar2 = DAT_14362b128;
    DAT_14362b0f8 = local_128;
    DAT_14362b100 = uStack_120;
    _DAT_14362b108 = lStack_150;
    uRam000000014362b110 = uStack_148;
    _DAT_14362b118 = uStack_140;
    lRam000000014362b120 = lStack_138;
    DAT_14362b128 = local_130;
    cStack_d2 = (char)((ulonglong)uVar2 >> 0x30);
    if (cStack_d2 != '\x05') {
      rhapsody::raw::managed::handle::impl$6::drop();
      core::ptr::
      drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>_>_>_>
                ();
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar4 = DAT_14362b0f0 == '\x01';
    if (bVar4) {
      DAT_14362b0f0 = '\0';
    }
    UNLOCK();
    if (!bVar4) {
      parking_lot::raw_mutex::RawMutex::unlock_slow();
    }
  }
  else {
    uVar1 = CONCAT44(uStack_154,local_158);
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
    lVar3 = 0;
    if (uVar1 - 9 < 2) {
      lVar3 = uVar1 - 8;
    }
    if (lVar3 == 0) {
      if (((uVar1 < 6) && ((0x32UL >> ((ulonglong)local_158 & 0x3f) & 1) != 0)) && (lStack_150 != 0)
         ) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      lStack_138 = lStack_150;
      if (lVar3 == 1) {
        core::ptr::drop_in_place<backtrace::capture::Backtrace>();
        return;
      }
    }
    if (lStack_138 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

