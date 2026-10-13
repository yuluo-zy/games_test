// address 0x141fd3700  name gpu_write_ranges_nonoverlapping<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// signature: void __cdecl gpu_write_ranges_nonoverlapping<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>(void)

void __cdecl
country_core::render::ssbo::TypedStorageBuffer::
gpu_write_ranges_nonoverlapping<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>(void)

{
  longlong lVar1;
  undefined *puVar2;
  ulonglong uVar3;
  ulonglong extraout_RAX;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong in_stack_00000028;
  ulonglong in_stack_00000030;
  int local_f8;
  ulonglong *local_48;
  
  if (in_stack_00000030 == 0) {
    return;
  }
  bumpalo::Bump::new();
  if (in_stack_00000030 >> 0x3b != 0) {
                    // WARNING: Subroutine does not return
    bumpalo::oom();
  }
  uVar5 = in_stack_00000030 * 0x10;
  uVar3 = local_48[4] & 0xfffffffffffffff8;
  if (uVar5 <= uVar3 - *local_48 && *local_48 <= uVar3) {
    uVar3 = uVar3 + in_stack_00000030 * -0x10;
    local_48[4] = uVar3;
    if (uVar3 != 0) goto LAB_141fd379e;
  }
  bumpalo::Bump::alloc_layout_slow<1>();
  uVar3 = extraout_RAX;
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    bumpalo::oom();
  }
LAB_141fd379e:
  uVar4 = 0;
  do {
    if (uVar5 == uVar4) {
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
    lVar1 = ((longlong *)(in_stack_00000028 + uVar4))[1];
    *(longlong *)(uVar3 + uVar4) = *(longlong *)(in_stack_00000028 + uVar4) << 6;
    ((longlong *)(uVar3 + uVar4))[1] = lVar1 << 6;
    uVar4 = uVar4 + 0x10;
    in_stack_00000030 = in_stack_00000030 - 1;
  } while (in_stack_00000030 != 0);
  rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
  puVar2 = __imp__ZN7bumpalo11EMPTY_CHUNK17h9156b7dbfe37bdb3E;
  if (local_f8 == 0xb) {
    while (local_48 != (ulonglong *)puVar2) {
      local_48 = (ulonglong *)local_48[3];
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


