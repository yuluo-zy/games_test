
void __cdecl rhapsody::rg::import_export::impl$0::import_owned(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined4 *in_RCX;
  undefined8 *in_RDX;
  longlong *in_R8;
  undefined1 in_R9B;
  undefined6 uStack_b6;
  
  uVar2 = in_RDX[3];
  uVar6 = in_RDX[4];
  uVar7 = in_RDX[6];
  *(undefined8 *)(in_RCX + 5) = in_RDX[5];
  *(undefined8 *)(in_RCX + 7) = uVar7;
  *(undefined8 *)(in_RCX + 1) = uVar2;
  *(undefined8 *)(in_RCX + 3) = uVar6;
  uVar2 = in_RDX[6];
  uVar6 = *in_RDX;
  uVar7 = in_RDX[1];
  uVar8 = in_RDX[2];
  uVar9 = in_RDX[3];
  uVar10 = in_RDX[4];
  uVar11 = in_RDX[5];
  lVar3 = in_R8[2];
  if (lVar3 == *in_R8) {
    alloc::raw_vec::RawVec::
    grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
              ();
  }
  lVar4 = in_R8[1];
  *(undefined8 *)(lVar4 + 0x40 + lVar3 * 0x48) = uVar2;
  puVar1 = (undefined8 *)(lVar4 + 0x30 + lVar3 * 0x48);
  *puVar1 = uVar10;
  puVar1[1] = uVar11;
  puVar1 = (undefined8 *)(lVar4 + 0x20 + lVar3 * 0x48);
  *puVar1 = uVar8;
  puVar1[1] = uVar9;
  puVar1 = (undefined8 *)(lVar4 + 0x10 + lVar3 * 0x48);
  *puVar1 = uVar6;
  puVar1[1] = uVar7;
  puVar1 = (undefined8 *)(lVar4 + lVar3 * 0x48);
  *puVar1 = 2;
  puVar1[1] = (ulonglong)CONCAT61(uStack_b6,in_R9B) << 8;
  in_R8[2] = lVar3 + 1;
  lVar4 = in_R8[8];
  if (lVar4 == in_R8[6]) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar5 = in_R8[7];
  *(undefined4 *)(lVar5 + lVar4 * 0x10) = 0xffffffff;
  *(undefined8 *)(lVar5 + 8 + lVar4 * 0x10) = 0;
  in_R8[8] = lVar4 + 1;
  *in_RCX = (int)lVar3;
  return;
}

