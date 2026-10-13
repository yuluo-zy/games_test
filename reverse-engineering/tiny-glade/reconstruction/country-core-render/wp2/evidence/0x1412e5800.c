// address 0x1412e5800  name new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>

void __cdecl
country_core::render::sparse_instance_buffer::impl$10::
new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  longlong lVar1;
  int *piVar2;
  void *_Dst;
  longlong extraout_RAX;
  longlong lVar3;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong in_R8;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
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
  memset(_Dst,0,0x10000);
  *(undefined1 *)((longlong)_Dst + 0x10000) = 0x20;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  lVar3 = 0x16c;
  while( true ) {
    *(undefined1 (*) [16])(extraout_RAX + -0x13c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0x14c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0x15c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0x16c + lVar3) = (undefined1  [16])0x0;
    *(undefined8 *)(extraout_RAX + -300 + lVar3) = 0xc2c80000c2c80000;
    *(undefined4 *)(extraout_RAX + -0x124 + lVar3) = 0xc2c80000;
    *(undefined1 (*) [16])(extraout_RAX + -0x120 + lVar3) = (undefined1  [16])0x0;
    *(undefined4 *)(extraout_RAX + -0x110 + lVar3) = 0;
    *(undefined1 (*) [16])(extraout_RAX + -0x10c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0xfc + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0xec + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0xdc + lVar3) = (undefined1  [16])0x0;
    *(undefined8 *)(extraout_RAX + -0xcc + lVar3) = 0xc2c80000c2c80000;
    *(undefined4 *)(extraout_RAX + -0xc4 + lVar3) = 0xc2c80000;
    *(undefined1 (*) [16])(extraout_RAX + -0xc0 + lVar3) = (undefined1  [16])0x0;
    *(undefined4 *)(extraout_RAX + -0xb0 + lVar3) = 0;
    *(undefined1 (*) [16])(extraout_RAX + -0x7c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0x8c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0x9c + lVar3) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + -0xac + lVar3) = (undefined1  [16])0x0;
    *(undefined8 *)(extraout_RAX + -0x6c + lVar3) = 0xc2c80000c2c80000;
    *(undefined4 *)(extraout_RAX + -100 + lVar3) = 0xc2c80000;
    *(undefined1 (*) [16])(extraout_RAX + -0x60 + lVar3) = (undefined1  [16])0x0;
    *(undefined4 *)(extraout_RAX + -0x50 + lVar3) = 0;
    if (lVar3 == 0xbffffec) break;
    lVar1 = extraout_RAX + lVar3;
    *(undefined1 (*) [16])(lVar1 + -0x1c) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(lVar1 + -0x2c) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(lVar1 + -0x3c) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(lVar1 + -0x4c) = (undefined1  [16])0x0;
    *(undefined8 *)(extraout_RAX + -0xc + lVar3) = 0xc2c80000c2c80000;
    *(undefined4 *)(extraout_RAX + -4 + lVar3) = 0xc2c80000;
    *(undefined1 (*) [16])(extraout_RAX + lVar3) = (undefined1  [16])0x0;
    *(undefined4 *)(extraout_RAX + 0x10 + lVar3) = 0;
    lVar3 = lVar3 + 0x180;
  }
  *(undefined1 (*) [16])(extraout_RAX + 0xbffffd0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xbffffc0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xbffffb0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xbffffa0) = (undefined1  [16])0x0;
  *(undefined8 *)(extraout_RAX + 0xbffffe0) = 0xc2c80000c2c80000;
  *(undefined4 *)(extraout_RAX + 0xbffffe8) = 0xc2c80000;
  *(undefined1 (*) [16])(extraout_RAX + 0xbffffec) = (undefined1  [16])0x0;
  *(undefined4 *)(extraout_RAX + 0xbfffffc) = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != 0) {
    iVar6 = 0;
    iVar7 = 1;
    iVar8 = 2;
    iVar9 = 3;
    lVar3 = 0xc;
    do {
      piVar2 = (int *)(extraout_RAX_00 + -0x30 + lVar3 * 4);
      *piVar2 = 0xffff - iVar6;
      piVar2[1] = 0xffff - iVar7;
      piVar2[2] = 0xffff - iVar8;
      piVar2[3] = 0xffff - iVar9;
      piVar2 = (int *)(extraout_RAX_00 + -0x20 + lVar3 * 4);
      *piVar2 = 0xfffb - iVar6;
      piVar2[1] = 0xfffb - iVar7;
      piVar2[2] = 0xfffb - iVar8;
      piVar2[3] = 0xfffb - iVar9;
      piVar2 = (int *)(extraout_RAX_00 + -0x10 + lVar3 * 4);
      *piVar2 = 0xfff7 - iVar6;
      piVar2[1] = 0xfff7 - iVar7;
      piVar2[2] = 0xfff7 - iVar8;
      piVar2[3] = 0xfff7 - iVar9;
      piVar2 = (int *)(extraout_RAX_00 + lVar3 * 4);
      *piVar2 = 0xfff3 - iVar6;
      piVar2[1] = 0xfff3 - iVar7;
      piVar2[2] = 0xfff3 - iVar8;
      piVar2[3] = 0xfff3 - iVar9;
      iVar6 = iVar6 + 0x10;
      iVar7 = iVar7 + 0x10;
      iVar8 = iVar8 + 0x10;
      iVar9 = iVar9 + 0x10;
      lVar3 = lVar3 + 0x10;
    } while (lVar3 != 0x1000c);
    ssbo::TypedStorageBuffer::new<country_core::geometry::instanced_wall::BrickTransformSsbo>();
    uVar4 = 0x400;
    alloc::boxed::iter::impl$13::
    from_iter<core::sync::atomic::AtomicUsize,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl_11::new::closure_0::closure_env_0<country_core::geometry::instanced_wall::BrickTransformSsbo>_>_>
              ();
    uVar5 = 0x400;
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
      in_RCX[5] = 0x10001;
      in_RCX[6] = extraout_RAX_01;
      in_RCX[7] = uVar4;
      *in_RCX = 0;
      in_RCX[1] = 0x10000;
      in_RCX[2] = extraout_RAX_00;
      in_RCX[3] = 0x10000;
      in_RCX[8] = extraout_RAX;
      in_RCX[9] = 0x200000;
      in_RCX[10] = extraout_RAX_02;
      in_RCX[0xb] = uVar5;
      in_RCX[0xc] = extraout_RAX_03;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


