
void __cdecl system_decorator::decorator_visual::stairs::find_best_neighbor_alignments(void)

{
  longlong *plVar1;
  float fVar2;
  undefined4 uVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong lVar6;
  undefined4 *puVar7;
  longlong *in_RCX;
  undefined4 *in_RDX;
  longlong *plVar8;
  longlong in_R8;
  byte in_R9B;
  byte bVar9;
  float fVar10;
  float fVar11;
  longlong local_c8;
  undefined4 *puStack_c0;
  undefined4 local_b8;
  undefined4 uStack_b4;
  float local_b0;
  longlong local_98;
  undefined4 *puStack_90;
  longlong local_88;
  
  if (in_R8 == 0) {
    *in_RCX = 0;
    in_RCX[1] = 4;
    in_RCX[2] = 0;
    *(undefined4 *)(in_RCX + 3) = 0;
  }
  else {
    local_98 = 0;
    puStack_90 = (undefined4 *)&DAT_00000004;
    local_88 = 0;
    plVar4 = *(longlong **)(in_RDX + 4);
    lVar5 = *(longlong *)(in_RDX + 6);
    plVar8 = plVar4;
    fVar11 = -3.4028235e+38;
    while (plVar8 != plVar4 + lVar5 * 3) {
      bVar9 = 1;
      if ((char)*plVar8 != '\0') {
        if ((char)*plVar8 != '\x01') {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        bVar9 = ((float)((ulonglong)*(undefined8 *)((longlong)plVar8 + 0xc) >> 0x20) *
                 (float)*(undefined8 *)((longlong)plVar8 + 4) <=
                (float)*(undefined8 *)((longlong)plVar8 + 0xc) *
                (float)((ulonglong)*(undefined8 *)((longlong)plVar8 + 4) >> 0x20)) * '\x02' + 2;
      }
      fVar2 = *(float *)((longlong)plVar8 + 0x14);
      find_best_neighbor_alignments();
      fVar10 = 0.0;
      if ((bVar9 & in_R9B) != 0) {
        fVar10 = -10.0;
      }
      plVar1 = plVar8 + 3;
      fVar10 = fVar2 + local_b0 + fVar10;
      if (fVar11 < fVar10) {
        local_88 = CONCAT44(uStack_b4,local_b8);
        if (local_98 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_98 = local_c8;
        puStack_90 = puStack_c0;
        uVar3 = *in_RDX;
        local_b8 = (undefined4)plVar8[2];
        lVar6 = *plVar8;
        puVar7 = (undefined4 *)plVar8[1];
        if (local_88 == local_c8) {
          alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
        }
        if (local_88 != 0) {
          memmove(puStack_c0 + 6,puStack_c0,local_88 * 0x18);
        }
        *puStack_c0 = uVar3;
        *(longlong *)(puStack_c0 + 1) = lVar6;
        *(undefined4 **)(puStack_c0 + 3) = puVar7;
        puStack_c0[5] = local_b8;
        local_88 = local_88 + 1;
        plVar8 = plVar1;
        puStack_c0 = puVar7;
        local_c8 = lVar6;
        fVar11 = fVar10;
      }
      else {
        plVar8 = plVar1;
        if (local_c8 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
    }
    in_RCX[2] = local_88;
    *in_RCX = local_98;
    in_RCX[1] = (longlong)puStack_90;
    *(float *)(in_RCX + 3) = fVar11;
  }
  return;
}

