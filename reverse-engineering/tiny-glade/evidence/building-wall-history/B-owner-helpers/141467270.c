
void __cdecl
utils::curve::Curve::get_pos_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>(void)

{
  uint extraout_EAX;
  longlong in_RCX;
  
  get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
            ();
  if (*(ulonglong *)(in_RCX + 0x10) <= (ulonglong)extraout_EAX) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if ((ulonglong)extraout_EAX + 1 < *(ulonglong *)(in_RCX + 0x10)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_bounds_check();
}

