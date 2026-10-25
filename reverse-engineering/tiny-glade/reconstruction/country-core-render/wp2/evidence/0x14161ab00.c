// address 0x14161ab00  name gpu_write_ranges_nonoverlapping<country_core::geometry::instanced_wall::BrickTransformSsbo>
// signature: void __cdecl gpu_write_ranges_nonoverlapping<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

void __cdecl
country_core::render::ssbo::TypedStorageBuffer::
gpu_write_ranges_nonoverlapping<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined *puVar3;
  ulonglong uVar4;
  ulonglong extraout_RAX;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong *in_stack_00000028;
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
  uVar6 = in_stack_00000030 * 0x10;
  uVar4 = local_48[4] & 0xfffffffffffffff8;
  if (uVar6 <= uVar4 - *local_48 && *local_48 <= uVar4) {
    uVar4 = uVar4 + in_stack_00000030 * -0x10;
    local_48[4] = uVar4;
    if (uVar4 != 0) goto LAB_14161ab9e;
  }
  bumpalo::Bump::alloc_layout_slow<1>();
  uVar4 = extraout_RAX;
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    bumpalo::oom();
  }
LAB_14161ab9e:
  uVar5 = 0;
  do {
    if (uVar6 == uVar5) {
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
    lVar1 = *in_stack_00000028;
    lVar2 = in_stack_00000028[1];
    in_stack_00000028 = in_stack_00000028 + 2;
    *(longlong *)(uVar4 + uVar5) = lVar1 * 0x60;
    *(longlong *)(uVar4 + 8 + uVar5) = lVar2 * 0x60;
    uVar5 = uVar5 + 0x10;
    in_stack_00000030 = in_stack_00000030 - 1;
  } while (in_stack_00000030 != 0);
  rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
  puVar3 = __imp__ZN7bumpalo11EMPTY_CHUNK17h9156b7dbfe37bdb3E;
  if (local_f8 == 0xb) {
    while (local_48 != (ulonglong *)puVar3) {
      local_48 = (ulonglong *)local_48[3];
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


