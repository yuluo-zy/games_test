void __cdecl despawn_prefab_contents(void)


// WARNING: Removing unreachable block (ram,0x000140843529)
// WARNING: Removing unreachable block (ram,0x00014084353a)
// WARNING: Removing unreachable block (ram,0x00014084355b)

void __cdecl country_core::systems::prefab::despawn_prefab_contents(void)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong lVar6;
  longlong in_RCX;
  longlong lVar7;
  int iVar8;
  longlong in_R8;
  longlong *plVar9;
  longlong in_R9;
  longlong lVar10;
  undefined4 uVar11;
  longlong local_a8;
  longlong *local_98;
  
  plVar2 = *(longlong **)(in_RCX + 8);
  lVar10 = *(longlong *)(in_RCX + 0x10);
  *(undefined8 *)(in_RCX + 0x10) = 0;
  plVar1 = (longlong *)((longlong)plVar2 + lVar10 * 0xc);
  if (lVar10 != 0) {
    do {
      plVar9 = (longlong *)((longlong)plVar2 + 0xc);
      lVar10 = *plVar2;
      if ((char)plVar2[1] == '\x1b') break;
      country_core::render::nani::gpu_world::GpuWorld::remove_instance();
      plVar2 = plVar9;
      local_a8 = lVar10;
    } while (plVar9 != plVar1);
  }
  lVar10 = *(longlong *)(in_RCX + 0x20);
  lVar3 = *(longlong *)(in_RCX + 0x28);
  *(undefined8 *)(in_RCX + 0x28) = 0;
  lVar7 = lVar3 * 0x10 + lVar10;
  if (lVar3 != 0) {
    do {
      lVar10 = lVar10 + 0x10;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::rt::RtMeshHandle>_>_>
                ();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<enum2_<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<enum2_<country_core::resources::rt::RtMeshHandle>,enum2_<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh>_>_>
                ();
      if (local_a8 != 3) {
        LOCK();
        *local_98 = *local_98 + -1;
        UNLOCK();
        if (*local_98 == 0) {
          alloc::sync::Arc::drop_slow<country_core::resources::rt::RtMesh,alloc::alloc::Global>();
        }
        *(undefined1 *)(in_R8 + 0x38) = 1;
      }
    } while (lVar10 != lVar7);
  }
  lVar10 = *(longlong *)(in_RCX + 0x40);
  *(undefined8 *)(in_RCX + 0x40) = 0;
  if (lVar10 != 0) {
    lVar3 = *(longlong *)(in_R9 + 8);
    if (lVar3 == 0) {
      lVar10 = lVar10 * 0x28;
      lVar3 = *(longlong *)(in_RCX + 0x38);
      do {
        if (*(longlong *)(lVar3 + 0x18) == 2) {
          return;
        }
        if (*(longlong *)(lVar3 + 0x18) != 0) goto LAB_14084365d;
        lVar10 = lVar10 + -0x28;
        lVar3 = lVar3 + 0x28;
      } while (lVar10 != 0);
    }
    else {
      uVar4 = *(ulonglong *)(in_R9 + 0x10);
      uVar11 = *(undefined4 *)(in_R9 + 0x18);
      iVar8 = *(int *)(in_R9 + 0x1c);
      lVar10 = lVar10 * 0x28;
      lVar7 = *(longlong *)(in_RCX + 0x38);
      do {
        if (*(longlong *)(lVar7 + 0x18) == 2) {
          return;
        }
        if (*(longlong *)(lVar7 + 0x18) != 0) {
LAB_14084365d:
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar5 = *(ulonglong *)(lVar7 + 0x20);
        if (uVar5 >> 0x20 < uVar4) {
          lVar6 = (uVar5 >> 0x20) * 0x1c;
          if (*(int *)(lVar3 + 0x18 + lVar6) == (int)uVar5) {
            *(undefined4 *)(lVar3 + lVar6) = uVar11;
            uVar11 = (undefined4)(uVar5 >> 0x20);
            *(undefined4 *)(in_R9 + 0x18) = uVar11;
            iVar8 = iVar8 + -1;
            *(int *)(in_R9 + 0x1c) = iVar8;
            *(int *)(lVar3 + 0x18 + lVar6) = (int)uVar5 + 1;
          }
        }
        lVar10 = lVar10 + -0x28;
        lVar7 = lVar7 + 0x28;
      } while (lVar10 != 0);
    }
  }
  return;
}

