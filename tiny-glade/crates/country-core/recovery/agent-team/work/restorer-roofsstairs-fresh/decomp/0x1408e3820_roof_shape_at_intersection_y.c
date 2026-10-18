
void __cdecl country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y(void)

{
  float fVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 *in_RCX;
  longlong in_RDX;
  float _X;
  float fVar5;
  float in_XMM2_Da;
  float fVar6;
  float fVar7;
  float fVar8;
  
  fVar6 = *(float *)(in_RDX + 0x30);
  fVar1 = *(float *)(in_RDX + 0x4c);
  fVar5 = 0.0;
  if (0.1 < fVar6) {
    if ((*(byte *)(in_RDX + 8) & 1) == 0) {
      fVar5 = (*(float *)(in_RDX + 0x14) + -0.90000004) / 7.1;
      fVar8 = 8.0;
    }
    else {
      fVar5 = *(float *)(in_RDX + 0x1c);
      fVar8 = *(float *)(in_RDX + 0x20);
      fVar7 = fVar8;
      if (fVar8 <= fVar5) {
        fVar7 = fVar5;
      }
      fVar5 = ((float)(~-(uint)NAN(fVar5) & (uint)fVar7 | -(uint)NAN(fVar5) & (uint)fVar8) + -2.25)
              / 12.75;
      fVar8 = 9.0;
    }
    fVar7 = 0.0;
    if (0.0 <= fVar5 / 0.4) {
      fVar7 = fVar5 / 0.4;
    }
    fVar5 = 1.0;
    if (fVar7 <= 1.0) {
      fVar5 = fVar7;
    }
    fVar5 = (3.0 - (fVar5 + fVar5)) * fVar5 * fVar5;
    fVar5 = fVar5 * 12.0 + (1.0 - fVar5) * fVar8;
    if (fVar5 < 0.0) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    fVar8 = fVar6 * 12.0 + (1.0 - fVar6) * 0.4;
    fVar6 = 0.0;
    if (0.0 <= fVar8) {
      fVar6 = fVar8;
    }
    if (fVar6 <= fVar5) {
      fVar5 = fVar6;
    }
  }
  if ((fVar1 != in_XMM2_Da) || (NAN(fVar1) || NAN(in_XMM2_Da))) {
    if ((in_XMM2_Da < fVar1) || (fVar5 + fVar1 <= in_XMM2_Da)) {
      *(undefined4 *)in_RCX = 2;
    }
    else {
      fVar6 = *(float *)(in_RDX + 0x2c);
      ::utils::inverse_lerp();
      powf(_X,1.0 / (fVar6 * 1.5 + (1.0 - fVar6) * 0.5));
      if ((*(byte *)(in_RDX + 8) & 1) == 0) {
        ::utils::geometry::circle::Circle2d::new_rotated();
        *(undefined4 *)in_RCX = 0;
      }
      else {
        ::utils::geometry::rectangle::Rectangle2d::new();
        *(undefined4 *)in_RCX = 1;
      }
    }
  }
  else {
    uVar2 = *(undefined8 *)(in_RDX + 8);
    uVar3 = *(undefined8 *)(in_RDX + 0x10);
    uVar4 = *(undefined8 *)(in_RDX + 0x1c);
    *(undefined8 *)((longlong)in_RCX + 0xc) = *(undefined8 *)(in_RDX + 0x14);
    *(undefined8 *)((longlong)in_RCX + 0x14) = uVar4;
    *in_RCX = uVar2;
    in_RCX[1] = uVar3;
  }
  return;
}

