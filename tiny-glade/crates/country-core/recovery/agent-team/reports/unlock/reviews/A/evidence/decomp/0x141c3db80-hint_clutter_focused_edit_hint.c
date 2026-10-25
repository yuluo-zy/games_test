// 0x141c3db80 sig void __cdecl hint_clutter_focused_edit_hint(void)

void __cdecl system_onboarding::hint_clutter_focus_edit::hint_clutter_focused_edit_hint(void)

{
  undefined8 *puVar1;
  char *pcVar2;
  ulonglong *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  longlong lVar8;
  longlong lVar9;
  char *extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar10;
  undefined8 *in_RDX;
  longlong *in_R8;
  undefined8 *in_R9;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  int local_f8;
  undefined4 local_f4;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined4 local_9e;
  undefined2 local_9a;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_68;
  int iStack_64;
  
  pcVar2 = (char *)*in_RDX;
  if (*pcVar2 == '\0') {
    puVar3 = (ulonglong *)*in_R9;
    lVar4 = in_R9[1];
    uVar5 = *puVar3;
    uVar12 = 0;
    uVar13 = uVar5 - *(ulonglong *)(lVar4 + 0x18);
    if (uVar5 < *(ulonglong *)(lVar4 + 0x18)) {
      uVar13 = uVar12;
    }
    uVar11 = uVar5 - *(ulonglong *)(lVar4 + 0x38);
    if (uVar5 < *(ulonglong *)(lVar4 + 0x38)) {
      uVar11 = uVar12;
    }
    uVar5 = *(ulonglong *)(lVar4 + 0x30);
    uVar14 = uVar5 - uVar11;
    if (uVar5 < uVar11) {
      uVar14 = uVar12;
    }
    uVar6 = *(ulonglong *)(lVar4 + 0x10);
    uVar15 = uVar6 - uVar13;
    if (uVar6 < uVar13) {
      uVar15 = uVar12;
    }
    uVar12 = *(ulonglong *)(lVar4 + 0x40);
    *puVar3 = uVar12 - (uVar15 + uVar14);
    if (uVar6 >= uVar13 && uVar6 - uVar13 != 0 || uVar5 >= uVar11 && uVar5 - uVar11 != 0) {
      *puVar3 = uVar12;
      *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
      pcVar2[1] = '\x01';
    }
    _ZN120__LT_bevy_ecs__system__schedule_system__InfallibleSystemWrapper_LT_S_GT__u20_as_u20_bevy_ecs__system__system__System_GT_16component_access17h024209426c3fd862E_llvm_6988214418030372215
              ();
    if (((*extraout_RAX != '\x02') && (pcVar2[1] != '\0')) && (*(int *)*in_RCX == 0)) {
      country_core::systems::collision::world::RaycastWorld::raycast();
      if (iStack_64 != 2) {
        country_core::systems::ui_edit_focus::geo_src_to_edit_subject_w_deco_clutter();
        if (local_f8 == 3) {
          make_focus_or_camera_binding_hint();
          local_d8 = CONCAT62((int6)((ulonglong)local_88 >> 0x10),0x100);
          *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
          lVar4 = *in_R8;
          uVar7 = *(undefined8 *)(lVar4 + 0x40);
          lVar8 = *(longlong *)(lVar4 + 0x30);
          if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                      ();
          }
          lVar9 = *(longlong *)(lVar4 + 0x28);
          lVar10 = lVar8 * 0x58;
          *(undefined8 *)(lVar9 + 0x50 + lVar10) = uVar7;
          puVar1 = (undefined8 *)(lVar9 + 0x40 + lVar10);
          *puVar1 = local_a8;
          puVar1[1] = CONCAT26(local_9a,CONCAT42(local_9e,0x100));
          puVar1 = (undefined8 *)(lVar9 + 0x30 + lVar10);
          *puVar1 = local_b8;
          puVar1[1] = uStack_b0;
          puVar1 = (undefined8 *)(lVar9 + 0x20 + lVar10);
          *puVar1 = local_c8;
          puVar1[1] = uStack_c0;
          puVar1 = (undefined8 *)(lVar9 + 0x10 + lVar10);
          *puVar1 = local_d8;
          puVar1[1] = uStack_d0;
          *(undefined8 *)(lVar9 + lVar10) = CONCAT44(local_f4,3);
          ((undefined8 *)(lVar9 + lVar10))[1] = uStack_90;
          *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
          *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
        }
      }
    }
  }
  return;
}

