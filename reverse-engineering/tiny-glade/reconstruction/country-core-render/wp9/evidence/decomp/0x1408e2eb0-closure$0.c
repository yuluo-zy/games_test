// signature: void __cdecl closure$0(void)

void __cdecl country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure_0(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined8 *in_RCX;
  longlong in_RDX;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  undefined8 local_98;
  float local_90;
  
  fVar1 = *(float *)(in_RDX + 0xc + (ulonglong)*(uint *)(in_RDX + 8) * 8);
  fVar2 = *(float *)(in_RDX + 0x10 + (ulonglong)*(uint *)(in_RDX + 8) * 8);
  fVar3 = *(float *)(in_RDX + 0x4c);
  ::utils::swizzlers::impl$0::x0y();
  fVar5 = *(float *)(in_RDX + 0x30);
  fVar4 = 0.0;
  if (0.1 < fVar5) {
    if ((*(byte *)(in_RDX + 8) & 1) == 0) {
      fVar4 = (*(float *)(in_RDX + 0x14) + -0.90000004) / 7.1;
      fVar7 = 8.0;
    }
    else {
      fVar4 = *(float *)(in_RDX + 0x1c);
      fVar7 = *(float *)(in_RDX + 0x20);
      fVar6 = fVar7;
      if (fVar7 <= fVar4) {
        fVar6 = fVar4;
      }
      fVar4 = ((float)(~-(uint)NAN(fVar4) & (uint)fVar6 | -(uint)NAN(fVar4) & (uint)fVar7) + -2.25)
              / 12.75;
      fVar7 = 9.0;
    }
    fVar6 = 0.0;
    if (0.0 <= fVar4 / 0.4) {
      fVar6 = fVar4 / 0.4;
    }
    fVar4 = 1.0;
    if (fVar6 <= 1.0) {
      fVar4 = fVar6;
    }
    fVar4 = (3.0 - (fVar4 + fVar4)) * fVar4 * fVar4;
    fVar4 = fVar4 * 12.0 + (1.0 - fVar4) * fVar7;
    if (fVar4 < 0.0) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    fVar7 = fVar5 * 12.0 + (1.0 - fVar5) * 0.4;
    fVar5 = 0.0;
    if (0.0 <= fVar7) {
      fVar5 = fVar7;
    }
    if (fVar5 <= fVar4) {
      fVar4 = fVar5;
    }
  }
  *in_RCX = CONCAT44(fVar4 + fVar3 + (float)((ulonglong)local_98 >> 0x20),
                     fVar4 * 0.0 + fVar1 + (float)local_98);
  *(float *)(in_RCX + 1) = fVar2 + local_90 + fVar4 * 0.0;
  return;
}

