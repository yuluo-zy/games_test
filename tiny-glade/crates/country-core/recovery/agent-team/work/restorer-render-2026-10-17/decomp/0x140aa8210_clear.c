// 0x140aa8210 clear size=901
// sig: void __cdecl clear(void)

void __cdecl country_core::resources::stairs::visual::MagicSupports::clear(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong lVar4;
  longlong lVar5;
  longlong in_RCX;
  longlong in_RDX;
  longlong in_R8;
  bool bVar6;
  
  if (*(longlong *)(in_RCX + 0x20) == 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 0;
    extraout_RAX[1] = 4;
    extraout_RAX[2] = 0;
    *(undefined1 *)(extraout_RAX + 3) = 0;
    lVar3 = *(longlong *)(in_RDX + 0x60);
    do {
      lVar4 = *(longlong *)(lVar3 + 8);
      while (lVar4 != -1) {
        if (lVar4 < 0) {
                    // WARNING: Subroutine does not return
          alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
        }
        LOCK();
        lVar5 = *(longlong *)(lVar3 + 8);
        bVar6 = lVar4 == lVar5;
        if (bVar6) {
          *(longlong *)(lVar3 + 8) = lVar4 + 1;
          lVar5 = lVar4;
        }
        UNLOCK();
        lVar4 = lVar5;
        if (bVar6) {
          *(longlong *)(in_RCX + 0x20) = lVar3;
          *(undefined8 **)(in_RCX + 0x28) = extraout_RAX;
          puVar1 = extraout_RAX;
          goto joined_r0x000140aa8456;
        }
      }
    } while( true );
  }
  puVar1 = *(undefined8 **)(in_RCX + 0x28);
joined_r0x000140aa8456:
  if (puVar1 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  LOCK();
  bVar6 = *(char *)(puVar1 + 3) == '\0';
  if (bVar6) {
    *(char *)(puVar1 + 3) = '\x01';
  }
  UNLOCK();
  if (!bVar6) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if ((puVar1[2] * 4 + puVar1[1] == 4 || puVar1[2] == 0) && (*(longlong *)(in_RDX + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
  clear<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  if (*(longlong *)(in_RCX + 0x30) != 0) {
    puVar2 = *(undefined8 **)(in_RCX + 0x38);
joined_r0x000140aa82e8:
    if (puVar2 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    LOCK();
    bVar6 = *(char *)(puVar2 + 3) == '\0';
    if (bVar6) {
      *(char *)(puVar2 + 3) = '\x01';
    }
    UNLOCK();
    if (!bVar6) {
      parking_lot::raw_mutex::RawMutex::lock_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    if ((puVar2[2] * 4 + puVar2[1] == 4 || puVar2[2] == 0) && (*(longlong *)(in_R8 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
    clear<country_core::geometry::instanced_wall::BrickTransformSsbo>();
    *(undefined8 *)(in_RCX + 0x10) = 0;
    *(undefined4 *)(in_RCX + 0x18) = 0;
    *(undefined1 *)(in_RCX + 0x40) = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar6 = *(char *)(puVar2 + 3) == '\x01';
    if (bVar6) {
      *(char *)(puVar2 + 3) = '\0';
    }
    UNLOCK();
    if (!bVar6) {
      parking_lot::raw_mutex::RawMutex::unlock_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar6 = *(char *)(puVar1 + 3) == '\x01';
    if (bVar6) {
      *(char *)(puVar1 + 3) = '\0';
    }
    UNLOCK();
    if (bVar6) {
      return;
    }
    parking_lot::raw_mutex::RawMutex::unlock_slow();
    return;
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX_00 = 0;
  extraout_RAX_00[1] = 4;
  extraout_RAX_00[2] = 0;
  *(undefined1 *)(extraout_RAX_00 + 3) = 0;
  lVar3 = *(longlong *)(in_R8 + 0x60);
  do {
    lVar4 = *(longlong *)(lVar3 + 8);
    while (lVar4 != -1) {
      if (lVar4 < 0) {
                    // WARNING: Subroutine does not return
        alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
      }
      LOCK();
      lVar5 = *(longlong *)(lVar3 + 8);
      bVar6 = lVar4 == lVar5;
      if (bVar6) {
        *(longlong *)(lVar3 + 8) = lVar4 + 1;
        lVar5 = lVar4;
      }
      UNLOCK();
      lVar4 = lVar5;
      if (bVar6) {
        *(longlong *)(in_RCX + 0x30) = lVar3;
        *(undefined8 **)(in_RCX + 0x38) = extraout_RAX_00;
        puVar2 = extraout_RAX_00;
        goto joined_r0x000140aa82e8;
      }
    }
  } while( true );
}


// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "acquire_resource", "address": "140c81c70"}, {"name": "clear<country_core::geometry::instanced_wall::BrickTransformSsbo>", "address": "1412e4760"}, {"name": "handle_alloc_error", "address": "1428d9000"}, {"name": "lock_slow", "address": "1428cd210"}, {"name": "panic_cold_display<ref$<str$>_>", "address": "1428a9a00"}, {"name": "release_resource", "address": "140c81e10"}, {"name": "unlock_slow", "address": "1428cdb90"}, {"name": "unwrap_failed", "address": "1428d9310"}]
