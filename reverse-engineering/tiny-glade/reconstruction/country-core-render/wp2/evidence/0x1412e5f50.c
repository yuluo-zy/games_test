// address 0x1412e5f50  name new_storage_buffer<country_core::resources::sparkle::SparkleSsbo>

void __cdecl
country_core::render::sparse_instance_buffer::impl$10::
new_storage_buffer<country_core::resources::sparkle::SparkleSsbo>(void)

{
  int *piVar1;
  void *_Dst;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong in_R8;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  
  if (in_R8 != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  memset(_Dst,0,0x300);
  *(undefined1 *)((longlong)_Dst + 0x300) = 0x20;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  lVar2 = 0;
  do {
    *(undefined1 (*) [16])(extraout_RAX + 0x10 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x20 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x30 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x40 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x50 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x60 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x70 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x80 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x90 + lVar2) = (undefined1  [16])0x0;
    lVar2 = lVar2 + 0xa0;
  } while (lVar2 != 0xbffe0);
  *(undefined1 (*) [16])(extraout_RAX + 0xbfff0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xbffe0) = (undefined1  [16])0x0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != 0) {
    iVar5 = 0;
    iVar6 = 1;
    iVar7 = 2;
    iVar8 = 3;
    lVar2 = 0x14;
    do {
      piVar1 = (int *)(extraout_RAX_00 + -0x50 + lVar2 * 4);
      *piVar1 = 0x2ff - iVar5;
      piVar1[1] = 0x2ff - iVar6;
      piVar1[2] = 0x2ff - iVar7;
      piVar1[3] = 0x2ff - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + -0x40 + lVar2 * 4);
      *piVar1 = 0x2fb - iVar5;
      piVar1[1] = 0x2fb - iVar6;
      piVar1[2] = 0x2fb - iVar7;
      piVar1[3] = 0x2fb - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + -0x30 + lVar2 * 4);
      *piVar1 = 0x2f7 - iVar5;
      piVar1[1] = 0x2f7 - iVar6;
      piVar1[2] = 0x2f7 - iVar7;
      piVar1[3] = 0x2f7 - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + -0x20 + lVar2 * 4);
      *piVar1 = 0x2f3 - iVar5;
      piVar1[1] = 0x2f3 - iVar6;
      piVar1[2] = 0x2f3 - iVar7;
      piVar1[3] = 0x2f3 - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + -0x10 + lVar2 * 4);
      *piVar1 = 0x2ef - iVar5;
      piVar1[1] = 0x2ef - iVar6;
      piVar1[2] = 0x2ef - iVar7;
      piVar1[3] = 0x2ef - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + lVar2 * 4);
      *piVar1 = 0x2eb - iVar5;
      piVar1[1] = 0x2eb - iVar6;
      piVar1[2] = 0x2eb - iVar7;
      piVar1[3] = 0x2eb - iVar8;
      iVar5 = iVar5 + 0x18;
      iVar6 = iVar6 + 0x18;
      iVar7 = iVar7 + 0x18;
      iVar8 = iVar8 + 0x18;
      lVar2 = lVar2 + 0x18;
    } while (lVar2 != 0x314);
    ssbo::TypedStorageBuffer::new<country_core::resources::sparkle::SparkleSsbo>();
    uVar3 = 0xc;
    alloc::boxed::iter::impl$13::
    from_iter<core::sync::atomic::AtomicUsize,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl_11::new::closure_0::closure_env_0<country_core::geometry::instanced_wall::BrickTransformSsbo>_>_>
              ();
    uVar4 = 0xc;
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
      in_RCX[0x11] = local_b8;
      in_RCX[0xf] = local_c8;
      in_RCX[0x10] = uStack_c0;
      in_RCX[0xd] = local_d8;
      in_RCX[0xe] = uStack_d0;
      in_RCX[4] = _Dst;
      in_RCX[5] = 0x301;
      in_RCX[6] = extraout_RAX_01;
      in_RCX[7] = uVar3;
      *in_RCX = 0;
      in_RCX[1] = 0x300;
      in_RCX[2] = extraout_RAX_00;
      in_RCX[3] = 0x300;
      in_RCX[8] = extraout_RAX;
      in_RCX[9] = 0x6000;
      in_RCX[10] = extraout_RAX_02;
      in_RCX[0xb] = uVar4;
      in_RCX[0xc] = extraout_RAX_03;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


