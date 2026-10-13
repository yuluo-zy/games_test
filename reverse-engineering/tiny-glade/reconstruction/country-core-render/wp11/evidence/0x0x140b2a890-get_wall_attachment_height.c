// 0x140b2a890 get_wall_attachment_height
// sig: void __cdecl get_wall_attachment_height(void)

void __cdecl
country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height(void)

{
  uint extraout_EAX;
  uint extraout_EAX_00;
  longlong in_RCX;
  float in_XMM1_Da;
  float fVar1;
  
  if ((*(byte *)(in_RCX + 0x108) & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (1 < *(uint *)(in_RCX + 600)) {
    fVar1 = in_XMM1_Da * *(float *)(in_RCX + 0x30);
    if (NAN(fVar1)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (ABS(fVar1) == INFINITY) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
              ();
    if (*(ulonglong *)(in_RCX + 0x10) <= (ulonglong)extraout_EAX) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (*(ulonglong *)(in_RCX + 0x10) <= (ulonglong)extraout_EAX + 1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    ::utils::curve::Curve::
    get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>()
    ;
    if (*(ulonglong *)(in_RCX + 0xe8) <= (ulonglong)extraout_EAX_00) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (*(ulonglong *)(in_RCX + 0xe8) <= (ulonglong)extraout_EAX_00 + 1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
  }
  return;
}

