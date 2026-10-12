
void __cdecl
utils::threshold_range::
threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env_7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env_8>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong in_RCX;
  ulonglong in_RDX;
  longlong *in_R9;
  longlong lVar6;
  ulonglong uVar7;
  longlong *plVar8;
  float in_XMM2_Da;
  longlong *in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 local_180;
  undefined4 local_178;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined4 local_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined4 local_c0;
  undefined8 *local_b8;
  longlong local_b0;
  longlong local_a8;
  undefined8 *local_a0;
  longlong *local_98;
  undefined8 local_8c;
  undefined4 local_84;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  if (in_RDX != 0) {
    local_b8 = (undefined8 *)(in_stack_00000030 + 0x78);
    plVar8 = (longlong *)in_R9[1];
    local_a0 = (undefined8 *)(*in_R9 + 0x78);
    local_b0 = in_R9[2];
    local_a8 = in_R9[3];
    uVar4 = 0;
    local_98 = plVar8;
    do {
      uVar3 = uVar4 + 1;
      if (uVar4 + 1 < in_RDX) {
        uVar3 = in_RDX;
      }
      uVar5 = uVar4;
      uVar7 = uVar4;
      if (*(float *)(in_RCX + uVar4 * 4) <= in_XMM2_Da) {
        do {
          if (uVar3 - 1 == uVar5) {
            uVar5 = uVar3 - uVar4;
            uVar7 = uVar3;
            goto joined_r0x0001411ffe6d;
          }
          uVar7 = uVar5 + 1;
          lVar1 = uVar5 * 4;
          uVar5 = uVar7;
        } while (*(float *)(in_RCX + 4 + lVar1) <= in_XMM2_Da);
        uVar5 = uVar7 - uVar4;
joined_r0x0001411ffe6d:
        if (1 < uVar5) {
          lVar1 = in_stack_00000028[2];
          if (lVar1 == *in_stack_00000028) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          lVar2 = in_stack_00000028[1];
          lVar6 = lVar1 * 0x20;
          *(ulonglong *)(lVar2 + lVar6) = uVar4;
          *(ulonglong *)(lVar2 + 8 + lVar6) = uVar7;
          *(undefined8 *)(lVar2 + 0x10 + lVar6) = *local_b8;
          *(undefined4 *)(lVar2 + 0x18 + lVar6) = *(undefined4 *)(local_b8 + 1);
          in_stack_00000028[2] = lVar1 + 1;
          plVar8 = local_98;
        }
      }
      else {
        do {
          if (uVar3 - 1 == uVar7) {
            uVar4 = uVar3 - uVar4;
            uVar7 = uVar3;
            goto joined_r0x0001411ffd6d;
          }
          lVar1 = uVar7 * 4;
          uVar7 = uVar7 + 1;
        } while (in_XMM2_Da < *(float *)(in_RCX + 4 + lVar1));
        uVar4 = uVar7 - uVar4;
joined_r0x0001411ffd6d:
        if (1 < uVar4) {
          local_178 = *(undefined4 *)(local_a0 + 1);
          local_180 = *local_a0;
          loop_range::LoopRange::subslice();
          local_d8 = *(undefined8 *)(local_b0 + 0x90);
          uStack_d0 = *(undefined8 *)(local_b0 + 0x98);
          local_c0 = local_84;
          local_c8 = local_8c;
          local_f8 = *(undefined8 *)(local_a8 + 8);
          uStack_f0 = *(undefined8 *)(local_a8 + 0x10);
          local_e8 = local_8c;
          local_e0 = local_84;
          system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment()
          ;
          lVar1 = plVar8[2];
          if (lVar1 == *plVar8) {
            alloc::raw_vec::RawVec::
            grow_one<enum2_<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>();
          }
          memmove((void *)(lVar1 * 0x88 + plVar8[1]),&local_180,0x88);
          plVar8[2] = lVar1 + 1;
        }
      }
      uVar4 = uVar7;
    } while (uVar7 < in_RDX);
  }
  return;
}

