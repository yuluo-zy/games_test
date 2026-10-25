
void __cdecl
country_core::systems::terrain::delayed_terrain_changed_msg::delayed_terrain_changed_msg(void)

{
  int *piVar1;
  char cVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined4 *puVar8;
  undefined8 uVar9;
  byte bVar10;
  int iVar11;
  undefined8 *extraout_RAX;
  longlong lVar12;
  undefined8 *extraout_RAX_00;
  longlong lVar13;
  longlong *in_RDX;
  longlong lVar14;
  longlong *in_R8;
  ulonglong uVar15;
  longlong *in_R9;
  int *piVar16;
  longlong lVar17;
  longlong lVar18;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  
  lVar14 = in_R8[1];
  lVar18 = in_R8[2];
  if (lVar18 != 0) {
    uVar3 = *(undefined4 *)(in_stack_00000028 + 0x1c);
    puVar6 = *(undefined4 **)(in_stack_00000028 + 0x10);
    uVar4 = *(undefined4 *)(in_stack_00000030 + 0x1c);
    puVar7 = *(undefined4 **)(in_stack_00000030 + 0x10);
    uVar5 = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar13 = *in_R9;
    puVar8 = (undefined4 *)in_R9[2];
    lVar17 = 0;
    bVar10 = 0;
    do {
      piVar1 = (int *)(lVar14 + lVar17);
      *piVar1 = *piVar1 + -1;
      if (!(bool)(*piVar1 != 0 | bVar10)) {
        frozen::FrozenMap::
        get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                  ();
        if (extraout_RAX == (undefined8 *)0x0) {
LAB_140aac1c6:
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        if ((extraout_RAX[3] != 0x3a) ||
           (iVar11 = memcmp(&anon_107183261458dea85466985370debbb6_78_llvm_3641509820080252961,
                            (void *)extraout_RAX[2],0x3a), iVar11 != 0)) {
LAB_140aac193:
                    /* WARNING: Subroutine does not return */
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        lVar12 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
        if (lVar12 == 0) goto LAB_140aac1c6;
        *puVar6 = uVar3;
        resources::terrain_height::TerrainHeightsData::
        copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo>();
        *puVar8 = uVar5;
        lVar12 = *(longlong *)(lVar13 + 0x30);
        uVar9 = *(undefined8 *)(lVar13 + 0x40);
        if (lVar12 == *(longlong *)(lVar13 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(lVar13 + 0x28) + lVar12 * 8) = uVar9;
        *(longlong *)(lVar13 + 0x30) = lVar12 + 1;
        *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
        frozen::FrozenMap::
        get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                  ();
        if (extraout_RAX_00 == (undefined8 *)0x0) {
LAB_140aac1d2:
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        if ((extraout_RAX_00[3] != 0x40) ||
           (iVar11 = memcmp(&anon_107183261458dea85466985370debbb6_88_llvm_3641509820080252961,
                            (void *)extraout_RAX_00[2],0x40), iVar11 != 0)) goto LAB_140aac193;
        lVar12 = (**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
        if (lVar12 == 0) goto LAB_140aac1d2;
        *puVar7 = uVar4;
        resources::terrain_height::TerrainHeightsData::
        copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo>();
        bVar10 = 1;
      }
      lVar17 = lVar17 + 4;
    } while (lVar18 * 4 - lVar17 != 0);
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  cVar2 = *(char *)(*in_RDX + 8);
  *(undefined1 *)(*in_RDX + 8) = 0;
  if (cVar2 != '\0') {
    if (lVar18 == *in_R8) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      lVar14 = in_R8[1];
    }
    *(undefined4 *)(lVar14 + lVar18 * 4) = 2;
    lVar18 = lVar18 + 1;
    in_R8[2] = lVar18;
  }
  if (lVar18 != 0) {
    piVar1 = (int *)in_R8[1];
    lVar13 = 1;
    lVar14 = 1;
    if (*piVar1 != 0) {
      lVar17 = 1;
      do {
        if (lVar18 == lVar17) {
          lVar13 = 0;
          goto LAB_140aac178;
        }
        lVar14 = lVar17 + 1;
        piVar16 = piVar1 + lVar17;
        lVar17 = lVar14;
      } while (*piVar16 != 0);
    }
    uVar15 = lVar18 - lVar14;
    if (uVar15 != 0) {
      if (lVar18 == lVar14 + 1) {
        lVar13 = 1;
      }
      else {
        piVar16 = piVar1 + lVar14 + 1;
        lVar13 = 1;
        lVar17 = 0;
        do {
          while (lVar12 = lVar17, piVar16[-1] == 0) {
            lVar13 = lVar13 + 1;
            iVar11 = *piVar16;
            if (iVar11 == 0) goto LAB_140aac14a;
LAB_140aac100:
            piVar16[-lVar13] = iVar11;
            piVar16 = piVar16 + 2;
            lVar17 = lVar12 + -2;
            if (-(lVar12 + -2) == (uVar15 & 0xfffffffffffffffe)) goto LAB_140aac15a;
          }
          piVar16[-1 - lVar13] = piVar16[-1];
          iVar11 = *piVar16;
          if (iVar11 != 0) goto LAB_140aac100;
LAB_140aac14a:
          lVar13 = lVar13 + 1;
          piVar16 = piVar16 + 2;
          lVar17 = lVar12 + -2;
        } while (-(lVar12 + -2) != (uVar15 & 0xfffffffffffffffe));
LAB_140aac15a:
        lVar14 = lVar14 - (lVar12 + -2);
      }
      if ((uVar15 & 1) != 0) {
        if (piVar1[lVar14] == 0) {
          lVar13 = lVar13 + 1;
        }
        else {
          piVar1[lVar14 - lVar13] = piVar1[lVar14];
        }
      }
    }
LAB_140aac178:
    in_R8[2] = lVar18 - lVar13;
  }
  return;
}

