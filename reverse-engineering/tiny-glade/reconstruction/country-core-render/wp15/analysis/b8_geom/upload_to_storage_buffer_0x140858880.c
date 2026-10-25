// addr=0x140858880 name=upload_to_storage_buffer
// signature=void __cdecl upload_to_storage_buffer(void)


void __cdecl country_core::geometry::instanced_wall::InstancedWall::upload_to_storage_buffer(void)

{
  longlong lVar1;
  longlong lVar2;
  char extraout_AL;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar3;
  longlong lVar4;
  bool bVar5;
  
  lVar1 = *(longlong *)(in_RCX + 8);
  if (!SBORROW8(0,lVar1)) {
    lVar4 = *(longlong *)(in_RCX + 0x10);
    lVar3 = *(longlong *)(in_RCX + 0x18);
    *(undefined8 *)(in_RCX + 8) = 0x8000000000000000;
    if (*(longlong *)(in_RCX + 0x20) == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar2 = *(longlong *)(in_RCX + 0x28);
    if (lVar2 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    LOCK();
    bVar5 = *(char *)(lVar2 + 0x18) == '\0';
    if (bVar5) {
      *(char *)(lVar2 + 0x18) = '\x01';
    }
    UNLOCK();
    if (!bVar5) {
      parking_lot::raw_mutex::RawMutex::lock_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    if ((*(longlong *)(lVar2 + 0x10) * 4 + *(longlong *)(lVar2 + 8) == 4 ||
         *(longlong *)(lVar2 + 0x10) == 0) && (*(longlong *)(in_RDX + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
    clear<country_core::geometry::instanced_wall::BrickTransformSsbo>();
    lVar3 = lVar3 * 0x60 + lVar4;
    do {
      if (lVar4 == lVar3) goto LAB_140858aae;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
      lVar4 = lVar4 + 0x60;
    } while (extraout_AL == '\0');
    if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
LAB_140858aae:
    if (lVar1 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar5 = *(char *)(lVar2 + 0x18) == '\x01';
    if (bVar5) {
      *(char *)(lVar2 + 0x18) = '\0';
    }
    UNLOCK();
    if (!bVar5) {
      parking_lot::raw_mutex::RawMutex::unlock_slow();
    }
  }
  return;
}


