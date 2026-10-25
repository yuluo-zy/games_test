// addr=0x140a6ba50 name=rewire_enclosed_curve
// signature=void __cdecl rewire_enclosed_curve(void)


void __cdecl country_core::utils::geo::rewire_enclosed_curve(void)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong *in_RCX;
  undefined8 *puVar4;
  longlong in_RDX;
  ulonglong uVar5;
  bool bVar6;
  float in_XMM2_Da;
  float fVar7;
  float fVar8;
  longlong local_160;
  longlong lStack_158;
  longlong local_150;
  longlong lStack_148;
  longlong local_140;
  longlong lStack_138;
  longlong local_130;
  longlong local_110;
  undefined8 *local_108;
  longlong local_100;
  longlong local_f8;
  float local_e0;
  longlong local_d8;
  void *local_d0;
  ulonglong local_c8;
  longlong local_c0;
  float local_a8;
  
  if (((in_XMM2_Da == 0.0) && (!NAN(in_XMM2_Da))) || ((in_XMM2_Da == 1.0 && (!NAN(in_XMM2_Da))))) {
    *in_RCX = -0x8000000000000000;
  }
  else {
    ::utils::curve::Curve::crashy_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    ::utils::curve::Curve::crashy_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    if ((local_a8 < 0.1) || (local_e0 < 0.1)) {
      *in_RCX = -0x8000000000000000;
      if (local_110 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      bVar6 = local_f8 != 0;
      local_f8 = local_d8;
      if (bVar6) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      uVar5 = local_c8 - 1;
      if (local_c8 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (uVar5 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar2 = *(undefined8 *)((longlong)local_d0 + uVar5 * 8);
      uVar3 = *(undefined8 *)((longlong)local_d0 + local_c8 * 8 + -0x10);
      fVar7 = (float)uVar2 - (float)uVar3;
      fVar8 = (float)((ulonglong)uVar2 >> 0x20) - (float)((ulonglong)uVar3 >> 0x20);
      if (SQRT(fVar8 * fVar8 + fVar7 * fVar7) < 0.1) {
        *(undefined8 *)((longlong)local_d0 + local_c8 * 8 + -0x10) = uVar2;
        local_c8 = uVar5;
      }
      if (local_100 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (local_100 == 1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar2 = *local_108;
      fVar7 = (float)uVar2 - (float)local_108[1];
      fVar8 = (float)((ulonglong)uVar2 >> 0x20) - (float)((ulonglong)local_108[1] >> 0x20);
      if (SQRT(fVar8 * fVar8 + fVar7 * fVar7) < 0.1) {
        local_108[1] = uVar2;
        lVar1 = local_100 * 8;
        local_100 = local_100 + -1;
        memmove(local_108,local_108 + 1,lVar1 - 8);
      }
      if (*(longlong *)(in_RDX + 0x10) == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      puVar4 = *(undefined8 **)(in_RDX + 8) + *(longlong *)(in_RDX + 0x10) + -1;
      if (puVar4 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      uVar2 = **(undefined8 **)(in_RDX + 8);
      uVar3 = *puVar4;
      fVar7 = (float)uVar2 - (float)uVar3;
      fVar8 = (float)((ulonglong)uVar2 >> 0x20) - (float)((ulonglong)uVar3 >> 0x20);
      if (SQRT(fVar8 * fVar8 + fVar7 * fVar7) < 0.1) {
        local_100 = local_100 + -1;
      }
      if ((ulonglong)(local_110 - local_100) < local_c8) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      memcpy(local_108 + local_100,local_d0,local_c8 * 8);
      if (local_d8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
      if (SBORROW8(0,local_160)) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      in_RCX[6] = local_130;
      in_RCX[4] = local_140;
      in_RCX[5] = lStack_138;
      in_RCX[2] = local_150;
      in_RCX[3] = lStack_148;
      *in_RCX = local_160;
      in_RCX[1] = lStack_158;
    }
    if (local_f8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_c0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}


