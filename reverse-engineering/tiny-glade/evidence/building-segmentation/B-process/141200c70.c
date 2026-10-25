
void __cdecl
alloc::vec::Vec::
retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env_0>
          (void)

{
  longlong lVar1;
  float *pfVar2;
  longlong *plVar3;
  longlong *plVar4;
  longlong lVar5;
  float fVar6;
  longlong lVar7;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar8;
  longlong lVar9;
  float extraout_XMM0_Da;
  float fVar10;
  float extraout_XMM0_Da_00;
  
  lVar1 = *(longlong *)(in_RCX + 0x10);
  if (lVar1 != 0) {
    *(undefined8 *)(in_RCX + 0x10) = 0;
    fVar6 = __real_becccccd;
    pfVar2 = *(float **)(in_RDX + 0x10);
    plVar3 = *(longlong **)(in_RDX + 0x20);
    plVar4 = *(longlong **)(in_RDX + 0x18);
    lVar5 = *(longlong *)(in_RCX + 8);
    lVar8 = 0;
    lVar9 = 0;
    do {
      system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate
                ();
      fVar10 = extraout_XMM0_Da + fVar6;
      if (fVar10 <= 0.0) {
        fVar10 = 0.0;
      }
      if (*pfVar2 <= fVar10 && fVar10 != *pfVar2) {
        *pfVar2 = fVar10;
        lVar7 = *plVar3;
        *plVar4 = lVar7;
      }
      else {
        if (fVar10 <= 0.0) {
          lVar8 = 1;
          goto joined_r0x000141200d66;
        }
        lVar7 = *plVar3;
      }
      *plVar3 = lVar7 + 1;
      lVar9 = lVar9 + 1;
    } while (lVar9 != lVar1);
LAB_141200d68:
    *(longlong *)(in_RCX + 0x10) = lVar1 - lVar8;
  }
  return;
joined_r0x000141200d66:
  lVar9 = lVar9 + 1;
  if (lVar9 == lVar1) goto LAB_141200d68;
  system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate();
  fVar10 = extraout_XMM0_Da_00 + fVar6;
  if (fVar10 <= 0.0) {
    fVar10 = 0.0;
  }
  if (*pfVar2 <= fVar10 && fVar10 != *pfVar2) {
    *pfVar2 = fVar10;
    lVar7 = *plVar3;
    *plVar4 = lVar7;
  }
  else {
    if (fVar10 <= 0.0) {
      lVar8 = lVar8 + 1;
      goto joined_r0x000141200d66;
    }
    lVar7 = *plVar3;
  }
  *plVar3 = lVar7 + 1;
  *(undefined8 *)(lVar5 + (lVar9 - lVar8) * 8) = *(undefined8 *)(lVar5 + lVar9 * 8);
  goto joined_r0x000141200d66;
}

