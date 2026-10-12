
void __cdecl utils::wall_space::WallSpaceCoord::get_tangent(void)

{
  uint extraout_EAX;
  float *in_RCX;
  longlong in_RDX;
  float fVar1;
  
  fVar1 = *in_RCX / *(float *)(in_RDX + 0x30);
  if (NAN(fVar1)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (ABS(fVar1) != INFINITY) {
    curve::Curve::
    get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
              ();
    if ((ulonglong)extraout_EAX + 1 < *(ulonglong *)(in_RDX + 0x10)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

