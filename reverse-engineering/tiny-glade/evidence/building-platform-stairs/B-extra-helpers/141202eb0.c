
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env_0>_>
          (void)

{
  ulonglong extraout_RAX;
  ulonglong uVar1;
  ulonglong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar2;
  
  uVar2 = in_RDX[1] - *in_RDX;
  if (0x7ffffffffffffffc < uVar2) {
LAB_141202f91:
                    /* WARNING: Subroutine does not return */
    raw_vec::handle_error();
  }
  if (in_RDX[1] == *in_RDX) {
    uVar1 = 4;
    uVar2 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) goto LAB_141202f91;
    uVar2 = uVar2 >> 2;
    uVar1 = extraout_RAX;
  }
  core::iter::adapters::map::impl$2::
  fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl_20::extend_trusted::closure_env_0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env_0>_>_>_>
            ();
  in_RCX[2] = 0;
  *in_RCX = uVar2;
  in_RCX[1] = uVar1;
  return;
}

