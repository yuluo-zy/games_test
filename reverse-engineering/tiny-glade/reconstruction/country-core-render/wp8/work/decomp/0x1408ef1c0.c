
void __cdecl country_core::resources::mouse_terrain_raycast::camera_mouse_ray(void)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  ulonglong *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  longlong *in_R9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  float fVar13;
  float fVar14;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 local_30;
  undefined8 uStack_28;
  undefined8 local_20;
  
  ::utils::geometry::ray::Ray::from_mouse_ss();
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar2 = (undefined8 *)*in_R9;
  puVar2[2] = local_20;
  *puVar2 = local_30;
  puVar2[1] = uStack_28;
  puVar3 = (ulonglong *)*in_stack_00000028;
  lVar4 = in_stack_00000028[1];
  uVar5 = *puVar3;
  uVar9 = 0;
  uVar10 = uVar5 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar10 = uVar9;
  }
  uVar8 = uVar5 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar8 = uVar9;
  }
  uVar5 = *(ulonglong *)(lVar4 + 0x30);
  uVar11 = uVar5 - uVar8;
  if (uVar8 > uVar5) {
    uVar11 = uVar9;
  }
  uVar6 = *(ulonglong *)(lVar4 + 0x10);
  uVar12 = uVar6 - uVar10;
  if (uVar10 > uVar6) {
    uVar12 = uVar9;
  }
  uVar9 = *(ulonglong *)(lVar4 + 0x40);
  *puVar3 = uVar9 - (uVar12 + uVar11);
  if (uVar10 <= uVar6 && uVar6 - uVar10 != 0 || uVar8 <= uVar5 && uVar5 - uVar8 != 0) {
    *puVar3 = uVar9;
  }
  else {
    uVar1 = *(undefined8 *)(*in_stack_00000030 + 0x158);
    fVar13 = (float)uVar1;
    fVar14 = (float)((ulonglong)uVar1 >> 0x20);
    if ((SQRT(fVar14 * fVar14 + fVar13 * fVar13) <= 0.0) &&
       (uVar1 = *(undefined8 *)(*in_stack_00000030 + 0x150), fVar13 = (float)uVar1,
       fVar14 = (float)((ulonglong)uVar1 >> 0x20), SQRT(fVar14 * fVar14 + fVar13 * fVar13) <= 0.0))
    {
      return;
    }
  }
  *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar4 = *in_stack_00000038;
  lVar7 = *(longlong *)(lVar4 + 0x30);
  uVar1 = *(undefined8 *)(lVar4 + 0x40);
  if (lVar7 == *(longlong *)(lVar4 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(lVar4 + 0x28) + lVar7 * 8) = uVar1;
  *(longlong *)(lVar4 + 0x30) = lVar7 + 1;
  *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  return;
}


