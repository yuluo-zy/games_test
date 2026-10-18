// capture_image @ 0x140589c90
// sig: void __cdecl capture_image(void)


void __cdecl tiny_glade::panic_reporter::panic_screenshot::capture_image(void)

{
  ulonglong uVar1;
  ulonglong extraout_RAX;
  ulonglong *extraout_RAX_00;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  bool bVar6;
  float fVar7;
  undefined4 local_380 [100];
  undefined1 local_1f0 [72];
  longlong local_1a8;
  undefined8 local_198;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  LOCK();
  bVar6 = DAT_14362b0f0 == '\0';
  if (bVar6) {
    DAT_14362b0f0 = '\x01';
  }
  UNLOCK();
  if (bVar6) {
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  }
  else {
    parking_lot::raw_mutex::RawMutex::lock_slow();
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  }
  if (DAT_14362b128._6_1_ != '\x05') {
    rhapsody::raw::texture::TextureDesc::extent_2d();
    fVar7 = SQRT(230400.0 / ((float)(extraout_RAX & 0xffffffff) / (float)(extraout_RAX >> 0x20)));
    iVar2 = 0;
    if (0.0 <= fVar7) {
      iVar2 = (int)(longlong)fVar7;
    }
    iVar4 = -1;
    if (fVar7 <= 4.294967e+09) {
      iVar4 = iVar2;
    }
    uVar5 = iVar4 + (uint)(iVar4 == 0);
    uVar1 = 0x38400 / (ulonglong)uVar5;
    uVar3 = (uint)uVar1;
    if ((uVar1 & 0x3fffe) != 0) {
      if (0x38400 < (ulonglong)(uVar3 & 0x3fffe) * (ulonglong)uVar5) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      rhapsody::rg::import_export::impl$1::import_unowned();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
      local_380[0] = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(local_380,local_1f0,400);
      country_core::render::pass_builder::compute_pass_builder::ComputePass::
      bind_external_storage_buffer();
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 != (ulonglong *)0x0) {
        *extraout_RAX_00 = CONCAT44(uVar5,uVar3) & 0xffffffff0003fffe;
        if (local_1a8 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_1a8 = 8;
        local_198 = 8;
        memcpy(local_380,local_1f0,400);
        country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d();
        rhapsody::rg::import_export::impl$1::export();
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar6 = DAT_14362b0f0 == '\x01';
        if (bVar6) {
          DAT_14362b0f0 = '\0';
        }
        UNLOCK();
        if (bVar6) {
          return;
        }
        parking_lot::raw_mutex::RawMutex::unlock_slow();
        return;
      }
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar6 = DAT_14362b0f0 == '\x01';
  if (bVar6) {
    DAT_14362b0f0 = '\0';
  }
  UNLOCK();
  if (bVar6) {
    return;
  }
  parking_lot::raw_mutex::RawMutex::unlock_slow();
  return;
}

