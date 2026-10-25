// ADDRESS: 0x140a081e0
// SIZE: 149
// SIGNATURE: void __cdecl collision_radius_xz(void)

void __cdecl country_core::render::mesh::Mesh::collision_radius_xz(void)

{
  ulonglong uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  uint *local_20;
  uint *local_18;
  longlong local_10;
  ulonglong local_8;
  
  positions();
  if (local_20 == local_18) {
    return;
  }
  fVar2 = 0.0;
  do {
    uVar1 = (ulonglong)*local_20;
    if (local_8 <= uVar1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    local_20 = local_20 + 1;
    fVar4 = *(float *)(local_10 + uVar1 * 0xc);
    fVar3 = *(float *)(local_10 + 8 + uVar1 * 0xc);
    fVar3 = SQRT(fVar3 * fVar3 + fVar4 * fVar4);
    fVar4 = fVar3;
    if (fVar3 <= fVar2) {
      fVar4 = fVar2;
    }
    fVar2 = (float)(~-(uint)NAN(fVar2) & (uint)fVar4 | -(uint)NAN(fVar2) & (uint)fVar3);
  } while (local_20 != local_18);
  return;
}

