
/* WARNING: Removing unreachable block (ram,0x0001411f88ca) */
/* WARNING: Removing unreachable block (ram,0x0001411f88f8) */
/* WARNING: Removing unreachable block (ram,0x0001411f8902) */
/* WARNING: Removing unreachable block (ram,0x0001411f8904) */
/* WARNING: Removing unreachable block (ram,0x0001411f8910) */
/* WARNING: Removing unreachable block (ram,0x0001411f8912) */
/* WARNING: Removing unreachable block (ram,0x0001411f8919) */
/* WARNING: Removing unreachable block (ram,0x0001411f891e) */
/* WARNING: Removing unreachable block (ram,0x0001411f8923) */
/* WARNING: Removing unreachable block (ram,0x0001411f88f3) */

void __cdecl
utils::curve::Curve::circular_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>(void)

{
  float in_XMM2_Da;
  float in_XMM3_Da;
  longlong local_e8;
  void *local_e0;
  longlong local_d8;
  longlong local_d0;
  
  if ((0.0 <= in_XMM2_Da) && (in_XMM2_Da <= __real_3f800000)) {
    if (!NAN(in_XMM2_Da)) {
      if ((in_XMM3_Da < 0.0) || (__real_3f800000 < in_XMM3_Da)) goto LAB_1411f89bf;
      if (!NAN(in_XMM3_Da)) {
        if (in_XMM2_Da < in_XMM3_Da) {
          slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
                    ();
          try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
        }
        else {
          if ((in_XMM2_Da != __real_3f800000) || (NAN(in_XMM2_Da) || NAN(__real_3f800000))) {
            slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
                      ();
          }
          if ((in_XMM3_Da != 0.0) || (NAN(in_XMM3_Da))) {
            slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
                      ();
            try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
            if (!SBORROW8(0,local_e8)) {
              if (local_d8 != 0) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              memcpy(&DAT_00000004,local_e0,local_d8 * 8);
              if (local_e8 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_d0 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
          }
          try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
        }
        return;
      }
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
LAB_1411f89bf:
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

