
void __cdecl utils::geometry::aabb::Aabb2::from_center_dims(void)

{
  float *in_RCX;
  float fVar1;
  float in_XMM1_Da;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar2;
  float in_stack_00000028;
  
  if (0.0 <= SQRT(in_stack_00000028 * in_stack_00000028 + in_XMM3_Da * in_XMM3_Da)) {
    fVar2 = in_XMM3_Da * __real_3f000000;
    fVar1 = in_stack_00000028 * __real_3f000000;
    *in_RCX = in_XMM1_Da - fVar2;
    in_RCX[1] = in_XMM2_Da - fVar1;
    in_RCX[2] = in_XMM1_Da + fVar2;
    in_RCX[3] = in_XMM2_Da + fVar1;
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

