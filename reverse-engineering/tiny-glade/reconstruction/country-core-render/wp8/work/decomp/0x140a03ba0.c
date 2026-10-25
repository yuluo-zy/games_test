
void __cdecl country_core::systems::terrain::draw_terrain_stroke::terrain_stroke_changed(void)

{
  ulonglong *puVar1;
  longlong lVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *in_R8;
  longlong *in_R9;
  ulonglong uVar11;
  ulonglong uVar12;
  
  puVar1 = (ulonglong *)*in_RCX;
  lVar2 = in_RCX[1];
  uVar3 = *puVar1;
  uVar12 = 0;
  uVar9 = uVar3 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar3 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar9 = uVar12;
  }
  uVar11 = uVar3 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar3 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar11 = uVar12;
  }
  uVar3 = *(ulonglong *)(lVar2 + 0x30);
  uVar10 = uVar3 - uVar11;
  if (uVar3 < uVar11) {
    uVar10 = uVar12;
  }
  uVar4 = *(ulonglong *)(lVar2 + 0x10);
  uVar8 = uVar4 - uVar9;
  if (uVar4 < uVar9) {
    uVar8 = uVar12;
  }
  uVar12 = *(ulonglong *)(lVar2 + 0x40);
  *puVar1 = uVar12 - (uVar8 + uVar10);
  if (uVar4 >= uVar9 && uVar4 - uVar9 != 0 || uVar3 >= uVar11 && uVar3 - uVar11 != 0) {
    *puVar1 = uVar12;
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar2 = *in_R8;
    lVar5 = *(longlong *)(lVar2 + 0x30);
    uVar6 = *(undefined8 *)(lVar2 + 0x40);
    if (lVar5 == *(longlong *)(lVar2 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar2 + 0x28) + lVar5 * 8) = uVar6;
    *(longlong *)(lVar2 + 0x30) = lVar5 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
    LOCK();
    *(longlong *)*in_RDX = *(longlong *)*in_RDX + 1;
    UNLOCK();
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar2 = *in_R9;
    lVar5 = *(longlong *)(lVar2 + 0x30);
    uVar6 = *(undefined8 *)(lVar2 + 0x40);
    if (lVar5 == *(longlong *)(lVar2 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar7 = *(longlong *)(lVar2 + 0x28);
    *(undefined8 *)(lVar7 + lVar5 * 0x28) = uVar6;
    *(undefined4 *)(lVar7 + 8 + lVar5 * 0x28) = 0;
    *(longlong *)(lVar2 + 0x30) = lVar5 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
  }
  return;
}


