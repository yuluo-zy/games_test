
void __cdecl
itertools::tuple_impl::impl$7::
next<core::slice::iter::Iter<glam::f32::vec3::Vec3>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec3::Vec3>_>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  longlong *in_RCX;
  
  lVar2 = *in_RCX;
  if (lVar2 == in_RCX[1]) {
    return;
  }
  lVar1 = lVar2 + 0xc;
  *in_RCX = lVar1;
  if (in_RCX[2] != 0) {
    lVar1 = in_RCX[3];
    in_RCX[3] = lVar2;
    in_RCX[2] = lVar1;
    return;
  }
  if (lVar1 != in_RCX[1]) {
    *in_RCX = lVar2 + 0x18;
    in_RCX[2] = lVar2;
    in_RCX[3] = lVar1;
    return;
  }
  in_RCX[2] = 0;
  in_RCX[3] = lVar1;
  return;
}

