
void __cdecl
bevy_ecs::event::writer::EventWriter::
write<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  longlong *in_RCX;
  longlong lVar13;
  undefined8 *in_RDX;
  
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  lVar2 = *in_RCX;
  lVar3 = *(longlong *)(lVar2 + 0x30);
  uVar4 = *(undefined8 *)(lVar2 + 0x40);
  if (lVar3 == *(longlong *)(lVar2 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform,utils::components::transform::Transform,u32>,alloc::alloc::Global>
              ();
  }
  lVar5 = *(longlong *)(lVar2 + 0x28);
  lVar13 = lVar3 * 0x70;
  uVar6 = in_RDX[0xb];
  puVar1 = (undefined8 *)(lVar5 + 0x50 + lVar13);
  *puVar1 = in_RDX[10];
  puVar1[1] = uVar6;
  uVar6 = in_RDX[9];
  puVar1 = (undefined8 *)(lVar5 + 0x40 + lVar13);
  *puVar1 = in_RDX[8];
  puVar1[1] = uVar6;
  uVar6 = *in_RDX;
  uVar7 = in_RDX[1];
  uVar8 = in_RDX[2];
  uVar9 = in_RDX[3];
  uVar10 = in_RDX[4];
  uVar11 = in_RDX[5];
  uVar12 = in_RDX[7];
  puVar1 = (undefined8 *)(lVar5 + 0x30 + lVar13);
  *puVar1 = in_RDX[6];
  puVar1[1] = uVar12;
  puVar1 = (undefined8 *)(lVar5 + 0x20 + lVar13);
  *puVar1 = uVar10;
  puVar1[1] = uVar11;
  puVar1 = (undefined8 *)(lVar5 + 0x10 + lVar13);
  *puVar1 = uVar8;
  puVar1[1] = uVar9;
  *(undefined8 *)(lVar5 + lVar13) = uVar6;
  ((undefined8 *)(lVar5 + lVar13))[1] = uVar7;
  *(undefined8 *)(lVar5 + 0x60 + lVar13) = uVar4;
  *(longlong *)(lVar2 + 0x30) = lVar3 + 1;
  *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
  return;
}


