
void __cdecl
utils::threshold_range::
threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env_1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env_2>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
  void *_Src;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong in_RCX;
  ulonglong in_RDX;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong in_R9;
  longlong lVar7;
  float in_XMM2_Da;
  ulonglong local_6c;
  
  if (in_RDX != 0) {
    lVar1 = *(longlong *)(in_R9 + 0x10);
    plVar2 = *(longlong **)(in_R9 + 0x18);
    uVar5 = 0;
    do {
      uVar3 = uVar5;
      if (*(float *)(in_RCX + uVar5 * 4) <= in_XMM2_Da) {
        do {
          uVar5 = uVar3 + 1;
          if (in_RDX <= uVar5) {
            return;
          }
          lVar7 = uVar3 * 4;
          uVar3 = uVar5;
        } while (*(float *)(in_RCX + 4 + lVar7) <= in_XMM2_Da);
      }
      else {
        uVar3 = uVar5 + 1;
        if (uVar5 + 1 < in_RDX) {
          uVar3 = in_RDX;
        }
        uVar6 = uVar5;
        do {
          if (uVar3 - 1 == uVar6) {
            uVar4 = uVar3 - uVar5;
            uVar5 = uVar3;
            goto joined_r0x0001411ff89d;
          }
          lVar7 = uVar6 * 4;
          uVar6 = uVar6 + 1;
        } while (in_XMM2_Da < *(float *)(in_RCX + 4 + lVar7));
        uVar4 = uVar6 - uVar5;
        uVar5 = uVar6;
joined_r0x0001411ff89d:
        if (1 < uVar4) {
          loop_range::LoopRange::subslice();
          _Src = *(void **)(lVar1 + 0x98);
          system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment()
          ;
          country_core::resources::walls::public_wall_state::PublicWallState::u_space();
          system_wall_constructor::segmentation::wall_path_segment::
          calculate_wall_path_segments_impl();
          lVar7 = plVar2[2];
          if ((ulonglong)(*plVar2 - lVar7) < local_6c) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            lVar7 = plVar2[2];
          }
          memcpy((void *)(lVar7 * 0x68 + plVar2[1]),_Src,local_6c * 0x68);
          plVar2[2] = local_6c + lVar7;
          core::ptr::
          drop_in_place<alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>_>
                    ();
          core::ptr::
          drop_in_place<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask>
                    ();
        }
      }
    } while (uVar5 < in_RDX);
  }
  return;
}

