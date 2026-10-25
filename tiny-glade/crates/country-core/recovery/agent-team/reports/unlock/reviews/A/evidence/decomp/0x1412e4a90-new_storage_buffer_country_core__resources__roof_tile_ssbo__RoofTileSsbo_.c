// addr 0x1412e4a90 sig void __cdecl new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void) size 1089

void __cdecl
country_core::render::sparse_instance_buffer::impl$10::
new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

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
  memset(_Dst,0,0x2000);
  *(undefined1 *)((longlong)_Dst + 0x2000) = 0x20;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  lVar2 = 0;
  do {
    *(undefined1 (*) [16])(extraout_RAX + 0x30 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x20 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x10 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x40 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x50 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x60 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x70 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x80 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x90 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0xa0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0xb0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0xc0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0xd0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0xe0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0xf0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x130 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x120 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x110 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x100 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x170 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x160 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x150 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x140 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x180 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 400 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x1a0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x1b0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x1c0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x1d0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x1e0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x1f0 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x200 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x210 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x220 + lVar2) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(extraout_RAX + 0x230 + lVar2) = (undefined1  [16])0x0;
    lVar2 = lVar2 + 0x240;
  } while (lVar2 != 0xffffc0);
  *(undefined1 (*) [16])(extraout_RAX + 0xfffff0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xffffe0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xffffd0) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(extraout_RAX + 0xffffc0) = (undefined1  [16])0x0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != 0) {
    iVar5 = 0;
    iVar6 = 1;
    iVar7 = 2;
    iVar8 = 3;
    lVar2 = 0xc;
    do {
      piVar1 = (int *)(extraout_RAX_00 + -0x30 + lVar2 * 4);
      *piVar1 = 0x1fff - iVar5;
      piVar1[1] = 0x1fff - iVar6;
      piVar1[2] = 0x1fff - iVar7;
      piVar1[3] = 0x1fff - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + -0x20 + lVar2 * 4);
      *piVar1 = 0x1ffb - iVar5;
      piVar1[1] = 0x1ffb - iVar6;
      piVar1[2] = 0x1ffb - iVar7;
      piVar1[3] = 0x1ffb - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + -0x10 + lVar2 * 4);
      *piVar1 = 0x1ff7 - iVar5;
      piVar1[1] = 0x1ff7 - iVar6;
      piVar1[2] = 0x1ff7 - iVar7;
      piVar1[3] = 0x1ff7 - iVar8;
      piVar1 = (int *)(extraout_RAX_00 + lVar2 * 4);
      *piVar1 = 0x1ff3 - iVar5;
      piVar1[1] = 0x1ff3 - iVar6;
      piVar1[2] = 0x1ff3 - iVar7;
      piVar1[3] = 0x1ff3 - iVar8;
      iVar5 = iVar5 + 0x10;
      iVar6 = iVar6 + 0x10;
      iVar7 = iVar7 + 0x10;
      iVar8 = iVar8 + 0x10;
      lVar2 = lVar2 + 0x10;
    } while (lVar2 != 0x200c);
    ssbo::TypedStorageBuffer::new<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
    uVar3 = 0x80;
    alloc::boxed::iter::impl$13::
    from_iter<core::sync::atomic::AtomicUsize,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl_11::new::closure_0::closure_env_0<country_core::geometry::instanced_wall::BrickTransformSsbo>_>_>
              ();
    uVar4 = 0x80;
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
      in_RCX[5] = 0x2001;
      in_RCX[6] = extraout_RAX_01;
      in_RCX[7] = uVar3;
      *in_RCX = 0;
      in_RCX[1] = 0x2000;
      in_RCX[2] = extraout_RAX_00;
      in_RCX[3] = 0x2000;
      in_RCX[8] = extraout_RAX;
      in_RCX[9] = 0x40000;
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


// CALLEES: [{"address": "140613c10", "name": "__rust_alloc"}, {"address": "1416969a0", "name": "from_iter<core::sync::atomic::AtomicUsize,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl$11::new::closure$0::closure_env$0<country_core::geometry::instanced_wall::BrickTransformSsbo>_>_>"}, {"address": "1428d9000", "name": "handle_alloc_error"}, {"address": "1428d8fe3", "name": "handle_error"}, {"address": "142923c20", "name": "memset"}, {"address": "1416191a0", "name": "new<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"}, {"address": "1428d9430", "name": "panic"}]