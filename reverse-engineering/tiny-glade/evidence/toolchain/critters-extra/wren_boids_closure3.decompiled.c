
void __cdecl system_critters::wren_boids::sim_wren_boids::closure_3(void)

{
  float fVar1;
  undefined8 uVar2;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  float in_XMM2_Da;
  float fVar3;
  float fVar4;
  float fVar5;
  
  uVar2 = *in_RDX;
  fVar3 = in_XMM2_Da * (float)*(undefined8 *)((longlong)in_RDX + 0xc) + (float)uVar2;
  fVar4 = in_XMM2_Da * (float)((ulonglong)*(undefined8 *)((longlong)in_RDX + 0xc) >> 0x20) +
          (float)((ulonglong)uVar2 >> 0x20);
  if ((fVar4 <= __real_41f00000) && (0.0 <= fVar4)) {
    fVar1 = *(float *)(in_RDX + 1);
    fVar5 = *(float *)((longlong)in_RDX + 0x14) * in_XMM2_Da + fVar1;
    if ((*(char *)*in_RCX == '\0') || (SQRT(fVar5 * fVar5 + fVar3 * fVar3) <= __real_42b7d907)) {
      if (DEBUG_SHOW_BIRD_RAYCASTS != '\0') {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX = uVar2;
        *(float *)(extraout_RAX + 1) = fVar1;
        *(ulonglong *)((longlong)extraout_RAX + 0xc) = CONCAT44(fVar4,fVar3);
        *(float *)((longlong)extraout_RAX + 0x14) = fVar5;
        country_core::utils::debug_viz::DebugLine::new();
        country_core::utils::debug_viz::DebugLine::show();
      }
      country_core::systems::collision::world::Raycasting::raycast();
    }
  }
  return;
}

