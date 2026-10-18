
void __cdecl country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized(void)

{
  undefined4 *extraout_RAX;
  longlong *in_RCX;
  longlong in_RDX;
  float fVar1;
  float _Y;
  longlong local_70;
  longlong lStack_68;
  longlong local_60;
  longlong lStack_58;
  longlong local_50;
  longlong lStack_48;
  longlong local_40;
  
  fVar1 = *(float *)(in_RDX + 0x2c);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  _Y = (1.0 - fVar1) * 0.5 + fVar1 * 1.5;
  fVar1 = powf(0.0,_Y);
  *extraout_RAX = 0;
  extraout_RAX[1] = fVar1;
  fVar1 = powf(0.05263158,_Y);
  extraout_RAX[2] = 0x3d579436;
  extraout_RAX[3] = fVar1;
  fVar1 = powf(0.10526316,_Y);
  extraout_RAX[4] = 0x3dd79436;
  extraout_RAX[5] = fVar1;
  fVar1 = powf(0.15789473,_Y);
  extraout_RAX[6] = 0x3e21af28;
  extraout_RAX[7] = fVar1;
  fVar1 = powf(0.21052632,_Y);
  extraout_RAX[8] = 0x3e579436;
  extraout_RAX[9] = fVar1;
  fVar1 = powf(0.2631579,_Y);
  extraout_RAX[10] = 0x3e86bca2;
  extraout_RAX[0xb] = fVar1;
  fVar1 = powf(0.31578946,_Y);
  extraout_RAX[0xc] = 0x3ea1af28;
  extraout_RAX[0xd] = fVar1;
  fVar1 = powf(0.36842105,_Y);
  extraout_RAX[0xe] = 0x3ebca1af;
  extraout_RAX[0xf] = fVar1;
  fVar1 = powf(0.42105263,_Y);
  extraout_RAX[0x10] = 0x3ed79436;
  extraout_RAX[0x11] = fVar1;
  fVar1 = powf(0.47368422,_Y);
  extraout_RAX[0x12] = 0x3ef286bd;
  extraout_RAX[0x13] = fVar1;
  fVar1 = powf(0.5263158,_Y);
  extraout_RAX[0x14] = 0x3f06bca2;
  extraout_RAX[0x15] = fVar1;
  fVar1 = powf(0.57894737,_Y);
  extraout_RAX[0x16] = 0x3f1435e5;
  extraout_RAX[0x17] = fVar1;
  fVar1 = powf(0.6315789,_Y);
  extraout_RAX[0x18] = 0x3f21af28;
  extraout_RAX[0x19] = fVar1;
  fVar1 = powf(0.68421054,_Y);
  extraout_RAX[0x1a] = 0x3f2f286c;
  extraout_RAX[0x1b] = fVar1;
  fVar1 = powf(0.7368421,_Y);
  extraout_RAX[0x1c] = 0x3f3ca1af;
  extraout_RAX[0x1d] = fVar1;
  fVar1 = powf(0.7894737,_Y);
  extraout_RAX[0x1e] = 0x3f4a1af3;
  extraout_RAX[0x1f] = fVar1;
  fVar1 = powf(0.84210527,_Y);
  extraout_RAX[0x20] = 0x3f579436;
  extraout_RAX[0x21] = fVar1;
  fVar1 = powf(0.8947368,_Y);
  extraout_RAX[0x22] = 0x3f650d79;
  extraout_RAX[0x23] = fVar1;
  fVar1 = powf(0.94736844,_Y);
  extraout_RAX[0x24] = 0x3f7286bd;
  extraout_RAX[0x25] = fVar1;
  *(undefined8 *)(extraout_RAX + 0x26) = 0x3f8000003f800000;
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (!SBORROW8(0,local_70)) {
    in_RCX[6] = local_40;
    in_RCX[4] = local_50;
    in_RCX[5] = lStack_48;
    in_RCX[2] = local_60;
    in_RCX[3] = lStack_58;
    *in_RCX = local_70;
    in_RCX[1] = lStack_68;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

