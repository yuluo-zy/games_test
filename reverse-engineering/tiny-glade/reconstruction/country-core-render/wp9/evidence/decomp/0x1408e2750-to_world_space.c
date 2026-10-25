// signature: void __cdecl to_world_space(void)

void __cdecl country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space(void)

{
  float fVar1;
  undefined4 *in_RCX;
  float *in_RDX;
  longlong in_R8;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined8 local_118;
  float local_110;
  float local_10c;
  float local_c8;
  
  fVar3 = *in_RDX;
  fVar6 = in_RDX[1];
  fVar4 = fVar6;
  if (fVar6 <= fVar3) {
    fVar4 = fVar3;
  }
  if ((float)(~-(uint)NAN(fVar3) & (uint)fVar4 | -(uint)NAN(fVar3) & (uint)fVar6) <= 0.1) {
    if ((*(byte *)(in_R8 + 8) & 1) != 0) {
      ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
    }
    roof_ridge::impl$0::to_world_space::closure_0();
    *in_RCX = 0;
    return;
  }
  if ((*(byte *)(in_R8 + 8) & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  fVar3 = (float)*(undefined8 *)(in_R8 + 0x1c);
  fVar6 = (float)((ulonglong)*(undefined8 *)(in_R8 + 0x1c) >> 0x20);
  fVar4 = (float)((ulonglong)local_118 >> 0x20);
  if (*(char *)(in_R8 + 0x3c) == '\x01') {
    ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
    local_c8 = (float)*(undefined8 *)(in_R8 + 0x14);
    local_10c = *(float *)(in_R8 + 0x18);
    fVar8 = *(float *)(in_R8 + 0x4c);
    ::utils::swizzlers::impl$0::x0y();
    fVar1 = *(float *)(in_R8 + 0x30);
    fVar7 = 0.0;
    if (0.1 < fVar1) {
      fVar7 = fVar6;
      if (fVar6 <= fVar3) {
        fVar7 = fVar3;
      }
      fVar2 = (((float)(~-(uint)NAN(fVar3) & (uint)fVar7 | -(uint)NAN(fVar3) & (uint)fVar6) + -2.25)
              / 12.75) / 0.4;
      fVar7 = 0.0;
      if (0.0 <= fVar2) {
        fVar7 = fVar2;
      }
      fVar2 = 1.0;
      if (fVar7 <= 1.0) {
        fVar2 = fVar7;
      }
      fVar7 = (3.0 - (fVar2 + fVar2)) * fVar2 * fVar2;
      fVar7 = fVar7 * 12.0 + (1.0 - fVar7) * 9.0;
      if (fVar7 < 0.0) {
LAB_1408e2e52:
                    // WARNING: Subroutine does not return
        core::f32::impl$0::clamp::do_panic::runtime();
      }
      fVar5 = fVar1 * 12.0 + (1.0 - fVar1) * 0.4;
      fVar2 = 0.0;
      if (0.0 <= fVar5) {
        fVar2 = fVar5;
      }
      if (fVar2 <= fVar7) {
        fVar7 = fVar2;
      }
    }
    ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
    ::utils::swizzlers::impl$0::x0y();
    fVar2 = 0.0;
    if (fVar1 <= 0.1) goto LAB_1408e2d8a;
    fVar2 = fVar6;
    if (fVar6 <= fVar3) {
      fVar2 = fVar3;
    }
    fVar6 = (((float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fVar6) + -2.25) /
            12.75) / 0.4;
    fVar3 = 0.0;
    if (0.0 <= fVar6) {
      fVar3 = fVar6;
    }
    fVar6 = 1.0;
    if (fVar3 <= 1.0) {
      fVar6 = fVar3;
    }
    fVar3 = (3.0 - (fVar6 + fVar6)) * fVar6 * fVar6;
    fVar2 = fVar3 * 12.0 + (1.0 - fVar3) * 9.0;
    if (fVar2 < 0.0) {
LAB_1408e2e64:
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    fVar6 = fVar1 * 12.0 + (1.0 - fVar1) * 0.4;
    fVar3 = 0.0;
    if (0.0 <= fVar6) {
      fVar3 = fVar6;
    }
  }
  else {
    ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
    local_c8 = (float)*(undefined8 *)(in_R8 + 0x14);
    local_10c = *(float *)(in_R8 + 0x18);
    fVar8 = *(float *)(in_R8 + 0x4c);
    ::utils::swizzlers::impl$0::x0y();
    fVar1 = *(float *)(in_R8 + 0x30);
    fVar7 = 0.0;
    if (0.1 < fVar1) {
      fVar7 = fVar6;
      if (fVar6 <= fVar3) {
        fVar7 = fVar3;
      }
      fVar2 = (((float)(~-(uint)NAN(fVar3) & (uint)fVar7 | -(uint)NAN(fVar3) & (uint)fVar6) + -2.25)
              / 12.75) / 0.4;
      fVar7 = 0.0;
      if (0.0 <= fVar2) {
        fVar7 = fVar2;
      }
      fVar2 = 1.0;
      if (fVar7 <= 1.0) {
        fVar2 = fVar7;
      }
      fVar7 = (3.0 - (fVar2 + fVar2)) * fVar2 * fVar2;
      fVar7 = fVar7 * 12.0 + (1.0 - fVar7) * 9.0;
      if (fVar7 < 0.0) goto LAB_1408e2e52;
      fVar5 = fVar1 * 12.0 + (1.0 - fVar1) * 0.4;
      fVar2 = 0.0;
      if (0.0 <= fVar5) {
        fVar2 = fVar5;
      }
      if (fVar2 <= fVar7) {
        fVar7 = fVar2;
      }
    }
    ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
    ::utils::swizzlers::impl$0::x0y();
    fVar2 = 0.0;
    if (fVar1 <= 0.1) goto LAB_1408e2d8a;
    fVar2 = fVar6;
    if (fVar6 <= fVar3) {
      fVar2 = fVar3;
    }
    fVar6 = (((float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fVar6) + -2.25) /
            12.75) / 0.4;
    fVar3 = 0.0;
    if (0.0 <= fVar6) {
      fVar3 = fVar6;
    }
    fVar6 = 1.0;
    if (fVar3 <= 1.0) {
      fVar6 = fVar3;
    }
    fVar3 = (3.0 - (fVar6 + fVar6)) * fVar6 * fVar6;
    fVar2 = fVar3 * 12.0 + (1.0 - fVar3) * 9.0;
    if (fVar2 < 0.0) goto LAB_1408e2e64;
    fVar6 = fVar1 * 12.0 + (1.0 - fVar1) * 0.4;
    fVar3 = 0.0;
    if (0.0 <= fVar6) {
      fVar3 = fVar6;
    }
  }
  if (fVar3 <= fVar2) {
    fVar2 = fVar3;
  }
LAB_1408e2d8a:
  *(ulonglong *)(in_RCX + 1) =
       CONCAT44(fVar7 + fVar4 + fVar8,fVar7 * 0.0 + (float)local_118 + local_c8);
  in_RCX[3] = local_110 + local_10c + fVar7 * 0.0;
  *(ulonglong *)(in_RCX + 4) =
       CONCAT44(fVar2 + fVar8 + fVar4,fVar2 * 0.0 + local_c8 + (float)local_118);
  in_RCX[6] = local_110 + local_10c + fVar2 * 0.0;
  *in_RCX = 1;
  return;
}

