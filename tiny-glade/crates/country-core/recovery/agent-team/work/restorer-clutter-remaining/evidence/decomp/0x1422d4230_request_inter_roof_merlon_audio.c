
void __cdecl
system_clutter::inter_roof_merlons::inter_roof_merlons_audio::request_inter_roof_merlon_audio(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong *in_R8;
  longlong lVar12;
  ulonglong uVar13;
  longlong *plVar14;
  longlong in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined7 uStack_5b;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 local_4c;
  undefined8 uStack_48;
  
  if (*(int *)*in_RCX == 0xd) {
    return;
  }
  uVar13 = *(ulonglong *)(in_stack_00000028 + 0x10);
  lVar5 = *in_R8;
  if (uVar13 < *(ulonglong *)(lVar5 + 0x10)) {
    country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar6 = *in_RDX;
    lVar12 = *(longlong *)(lVar6 + 0x30);
    uVar4 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar12 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar8 = *(longlong *)(lVar6 + 0x28);
    *(undefined8 *)(lVar8 + lVar12 * 0x28) = uVar4;
    puVar1 = (undefined8 *)(lVar8 + 0x14 + lVar12 * 0x28);
    *puVar1 = CONCAT44(local_4c,uStack_50);
    puVar1[1] = uStack_48;
    puVar1 = (undefined8 *)(lVar8 + 8 + lVar12 * 0x28);
    *puVar1 = CONCAT71(uStack_5b,8);
    puVar1[1] = CONCAT44(uStack_50,uStack_54);
    *(longlong *)(lVar6 + 0x30) = lVar12 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
    plVar14 = (longlong *)*in_stack_00000030;
    if (*(longlong *)(in_stack_00000028 + 0x18) == *plVar14) goto LAB_1422d427c;
  }
  else {
    plVar14 = (longlong *)*in_stack_00000030;
    if (*(longlong *)(in_stack_00000028 + 0x18) == *plVar14) {
LAB_1422d427c:
      lVar6 = *(longlong *)(lVar5 + 8);
      uVar7 = *(ulonglong *)(lVar5 + 0x10);
      goto joined_r0x0001422d4287;
    }
  }
  country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar6 = *in_RDX;
  lVar12 = *(longlong *)(lVar6 + 0x30);
  uVar4 = *(undefined8 *)(lVar6 + 0x40);
  if (lVar12 == *(longlong *)(lVar6 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
              ();
  }
  lVar8 = *(longlong *)(lVar6 + 0x28);
  *(undefined8 *)(lVar8 + lVar12 * 0x28) = uVar4;
  puVar1 = (undefined8 *)(lVar8 + 0x14 + lVar12 * 0x28);
  *puVar1 = CONCAT44(local_4c,uStack_50);
  puVar1[1] = uStack_48;
  puVar1 = (undefined8 *)(lVar8 + 8 + lVar12 * 0x28);
  *puVar1 = CONCAT71(uStack_5b,9);
  puVar1[1] = CONCAT44(uStack_50,uStack_54);
  *(longlong *)(lVar6 + 0x30) = lVar12 + 1;
  *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  lVar6 = *(longlong *)(lVar5 + 8);
  uVar7 = *(ulonglong *)(lVar5 + 0x10);
joined_r0x0001422d4287:
  if (uVar7 <= uVar13) {
    *(ulonglong *)(in_stack_00000028 + 0x10) = uVar7;
    uVar13 = uVar7;
  }
  if (uVar13 != 0) {
    lVar5 = *(longlong *)(in_stack_00000028 + 8);
    lVar12 = 0x34;
    do {
      uVar3 = *(undefined4 *)(lVar6 + lVar12);
      puVar1 = (undefined8 *)(lVar6 + -0x34 + lVar12);
      uVar4 = puVar1[1];
      puVar2 = (undefined8 *)(lVar5 + -0x34 + lVar12);
      *puVar2 = *puVar1;
      puVar2[1] = uVar4;
      uVar4 = *(undefined8 *)(lVar6 + -8 + lVar12);
      puVar1 = (undefined8 *)(lVar6 + -0x24 + lVar12);
      uVar9 = *puVar1;
      uVar10 = puVar1[1];
      puVar1 = (undefined8 *)(lVar6 + -0x18 + lVar12);
      uVar11 = puVar1[1];
      puVar2 = (undefined8 *)(lVar5 + -0x18 + lVar12);
      *puVar2 = *puVar1;
      puVar2[1] = uVar11;
      puVar1 = (undefined8 *)(lVar5 + -0x24 + lVar12);
      *puVar1 = uVar9;
      puVar1[1] = uVar10;
      *(undefined8 *)(lVar5 + -8 + lVar12) = uVar4;
      *(undefined4 *)(lVar5 + lVar12) = uVar3;
      lVar12 = lVar12 + 0x38;
      uVar13 = uVar13 - 1;
    } while (uVar13 != 0);
  }
  alloc::vec::spec_extend::impl$3::
  spec_extend<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsDescription,core::slice::iter::Iter<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsDescription>,alloc::alloc::Global>
            ();
  *(longlong *)(in_stack_00000028 + 0x18) = *plVar14;
  return;
}

