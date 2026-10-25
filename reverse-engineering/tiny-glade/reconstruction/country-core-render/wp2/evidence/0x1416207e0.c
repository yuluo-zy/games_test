// address 0x1416207e0  name new_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>
// signature: void __cdecl new_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>(void)

void __cdecl
country_core::render::ssbo::impl$7::
new_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>(void)

{
  void *_Dst;
  ulonglong *in_RCX;
  ulonglong in_R8;
  ulonglong in_R9;
  undefined8 local_1f8;
  ulonglong uStack_1f0;
  ulonglong local_1e8;
  ulonglong uStack_1e0;
  ulonglong local_1d8;
  ulonglong local_38;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  if (in_R8 == 0) {
    local_29 = 0;
    nani::buffers::impl$9::default_value();
    if ((int)local_1f8 == 1) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (void *)0x0) {
        local_29 = 0;
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      memcpy(_Dst,(void *)((longlong)&local_1f8 + 4),0x1b0);
      local_29 = 1;
      TypedStorageBuffer::new<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>();
      in_RCX[5] = local_1d8;
      in_RCX[3] = local_1e8;
      in_RCX[4] = uStack_1e0;
      in_RCX[1] = local_1f8;
      in_RCX[2] = uStack_1f0;
      *in_RCX = 1;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    else {
      local_29 = 0;
      TypedStorageBuffer::new<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>();
      in_RCX[5] = local_1d8;
      in_RCX[3] = local_1e8;
      in_RCX[4] = uStack_1e0;
      in_RCX[1] = local_1f8;
      in_RCX[2] = uStack_1f0;
      *in_RCX = 0;
    }
  }
  else {
    local_38 = in_R9 % 0x1b0;
    if ((local_38 != 0) || (local_38 = in_R8 & 3, local_38 != 0)) {
      local_1f8 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<usize,usize>();
    }
    local_29 = 0;
    TypedStorageBuffer::new<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>();
    in_RCX[5] = local_1d8;
    in_RCX[3] = local_1e8;
    in_RCX[4] = uStack_1e0;
    in_RCX[1] = local_1f8;
    in_RCX[2] = uStack_1f0;
    *in_RCX = in_R9 / 0x1b0;
  }
  return;
}


