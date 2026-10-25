
void __cdecl rhapsody::rg::import_export::impl$0::import_unowned(void)

{
  int *piVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined7 uVar11;
  undefined4 *in_RCX;
  undefined8 *in_RDX;
  longlong *in_R8;
  undefined1 in_R9B;
  undefined1 uStack_a1;
  undefined6 uStack_76;
  
  lVar3 = in_RDX[1];
  LOCK();
  piVar1 = (int *)(lVar3 + 8);
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  uVar11 = *(undefined7 *)(in_RDX + 2);
  uVar4 = *in_RDX;
  uVar7 = in_RDX[5];
  uVar8 = in_RDX[6];
  uVar9 = in_RDX[3];
  uVar10 = in_RDX[4];
  lVar5 = in_R8[2];
  if (lVar5 == *in_R8) {
    alloc::raw_vec::RawVec::
    grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
              ();
  }
  lVar6 = in_R8[1];
  *(undefined8 *)(lVar6 + 0x40 + lVar5 * 0x48) = uVar8;
  puVar2 = (undefined8 *)(lVar6 + 0x30 + lVar5 * 0x48);
  *puVar2 = uVar10;
  puVar2[1] = uVar7;
  puVar2 = (undefined8 *)(lVar6 + 0x20 + lVar5 * 0x48);
  *puVar2 = CONCAT17(uStack_a1,uVar11);
  puVar2[1] = uVar9;
  puVar2 = (undefined8 *)(lVar6 + 0x10 + lVar5 * 0x48);
  *puVar2 = uVar4;
  puVar2[1] = lVar3;
  puVar2 = (undefined8 *)(lVar6 + lVar5 * 0x48);
  *puVar2 = 1;
  puVar2[1] = (ulonglong)CONCAT61(uStack_76,in_R9B) << 8;
  in_R8[2] = lVar5 + 1;
  lVar3 = in_R8[8];
  if (lVar3 == in_R8[6]) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar6 = in_R8[7];
  *(undefined4 *)(lVar6 + lVar3 * 0x10) = 0xffffffff;
  *(undefined8 *)(lVar6 + 8 + lVar3 * 0x10) = 0;
  in_R8[8] = lVar3 + 1;
  uVar4 = in_RDX[3];
  uVar7 = in_RDX[4];
  uVar8 = in_RDX[6];
  *(undefined8 *)(in_RCX + 5) = in_RDX[5];
  *(undefined8 *)(in_RCX + 7) = uVar8;
  *(undefined8 *)(in_RCX + 1) = uVar4;
  *(undefined8 *)(in_RCX + 3) = uVar7;
  *in_RCX = (int)lVar5;
  return;
}

