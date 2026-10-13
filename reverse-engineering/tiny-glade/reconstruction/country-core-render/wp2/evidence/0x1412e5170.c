// address 0x1412e5170  name new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>

void __cdecl
country_core::render::sparse_instance_buffer::impl$10::
new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>(void)

{
  longlong lVar1;
  int *piVar2;
  undefined8 *puVar3;
  void *_Dst;
  longlong extraout_RAX;
  longlong lVar4;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong in_R8;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  
  if (in_R8 != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  memset(_Dst,0,0x1000);
  *(undefined1 *)((longlong)_Dst + 0x1000) = 0x20;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  lVar4 = 0xf0;
  while( true ) {
    puVar3 = (undefined8 *)(extraout_RAX + -0xd0 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0xe0 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0xf0 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0xc0 + lVar4);
    *puVar3 = 0;
    puVar3[1] = 0x100000000;
    puVar3 = (undefined8 *)(extraout_RAX + -0xb0 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0xa0 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0x90 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0x80 + lVar4);
    *puVar3 = 0;
    puVar3[1] = 0x100000000;
    puVar3 = (undefined8 *)(extraout_RAX + -0x70 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0x60 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0x50 + lVar4);
    *puVar3 = 0x3f800000;
    puVar3[1] = 0;
    puVar3 = (undefined8 *)(extraout_RAX + -0x40 + lVar4);
    *puVar3 = 0;
    puVar3[1] = 0x100000000;
    if (lVar4 == 0x7ffff0) break;
    lVar1 = extraout_RAX + lVar4;
    *(undefined8 *)(lVar1 + -0x10) = 0x3f800000;
    *(undefined8 *)(lVar1 + -8) = 0;
    *(undefined8 *)(lVar1 + -0x20) = 0x3f800000;
    *(undefined8 *)(lVar1 + -0x18) = 0;
    *(undefined8 *)(lVar1 + -0x30) = 0x3f800000;
    *(undefined8 *)(lVar1 + -0x28) = 0;
    *(undefined8 *)(extraout_RAX + lVar4) = 0;
    ((undefined8 *)(extraout_RAX + lVar4))[1] = 0x100000000;
    lVar4 = lVar4 + 0x100;
  }
  *(undefined8 *)(extraout_RAX + 0x7fffe0) = 0x3f800000;
  *(undefined8 *)(extraout_RAX + 0x7fffe8) = 0;
  *(undefined8 *)(extraout_RAX + 0x7fffd0) = 0x3f800000;
  *(undefined8 *)(extraout_RAX + 0x7fffd8) = 0;
  *(undefined8 *)(extraout_RAX + 0x7fffc0) = 0x3f800000;
  *(undefined8 *)(extraout_RAX + 0x7fffc8) = 0;
  *(undefined8 *)(extraout_RAX + 0x7ffff0) = 0;
  *(undefined8 *)(extraout_RAX + 0x7ffff8) = 0x100000000;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != 0) {
    iVar7 = 0;
    iVar8 = 1;
    iVar9 = 2;
    iVar10 = 3;
    lVar4 = 0xc;
    do {
      piVar2 = (int *)(extraout_RAX_00 + -0x30 + lVar4 * 4);
      *piVar2 = 0xfff - iVar7;
      piVar2[1] = 0xfff - iVar8;
      piVar2[2] = 0xfff - iVar9;
      piVar2[3] = 0xfff - iVar10;
      piVar2 = (int *)(extraout_RAX_00 + -0x20 + lVar4 * 4);
      *piVar2 = 0xffb - iVar7;
      piVar2[1] = 0xffb - iVar8;
      piVar2[2] = 0xffb - iVar9;
      piVar2[3] = 0xffb - iVar10;
      piVar2 = (int *)(extraout_RAX_00 + -0x10 + lVar4 * 4);
      *piVar2 = 0xff7 - iVar7;
      piVar2[1] = 0xff7 - iVar8;
      piVar2[2] = 0xff7 - iVar9;
      piVar2[3] = 0xff7 - iVar10;
      piVar2 = (int *)(extraout_RAX_00 + lVar4 * 4);
      *piVar2 = 0xff3 - iVar7;
      piVar2[1] = 0xff3 - iVar8;
      piVar2[2] = 0xff3 - iVar9;
      piVar2[3] = 0xff3 - iVar10;
      iVar7 = iVar7 + 0x10;
      iVar8 = iVar8 + 0x10;
      iVar9 = iVar9 + 0x10;
      iVar10 = iVar10 + 0x10;
      lVar4 = lVar4 + 0x10;
    } while (lVar4 != 0x100c);
    ssbo::TypedStorageBuffer::new<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
    uVar5 = 0x40;
    alloc::boxed::iter::impl$13::
    from_iter<core::sync::atomic::AtomicUsize,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl_11::new::closure_0::closure_env_0<country_core::geometry::instanced_wall::BrickTransformSsbo>_>_>
              ();
    uVar6 = 0x40;
    alloc::boxed::iter::impl$13::
    from_iter<core::sync::atomic::AtomicUsize,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl_11::new::closure_0::closure_env_0<country_core::geometry::instanced_wall::BrickTransformSsbo>_>_>
              ();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_03 != (undefined8 *)0x0) {
      *extraout_RAX_03 = 1;
      extraout_RAX_03[1] = 1;
      *(undefined1 *)(extraout_RAX_03 + 2) = 0;
      extraout_RAX_03[3] = 0;
      extraout_RAX_03[4] = 8;
      extraout_RAX_03[5] = 0;
      in_RCX[0x11] = local_98;
      in_RCX[0xf] = local_a8;
      in_RCX[0x10] = uStack_a0;
      in_RCX[0xd] = local_b8;
      in_RCX[0xe] = uStack_b0;
      in_RCX[4] = _Dst;
      in_RCX[5] = 0x1001;
      in_RCX[6] = extraout_RAX_01;
      in_RCX[7] = uVar5;
      *in_RCX = 0;
      in_RCX[1] = 0x1000;
      in_RCX[2] = extraout_RAX_00;
      in_RCX[3] = 0x1000;
      in_RCX[8] = extraout_RAX;
      in_RCX[9] = 0x20000;
      in_RCX[10] = extraout_RAX_02;
      in_RCX[0xb] = uVar6;
      in_RCX[0xc] = extraout_RAX_03;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


