// addr=0x140a7a5b0 name=uninit_clearing
// signature=void __cdecl uninit_clearing(void)


void __cdecl country_core::startup::clearing::uninit_clearing(void)

{
  undefined4 uVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined4 *puVar4;
  ulonglong uVar5;
  char *extraout_RAX;
  longlong in_RCX;
  undefined8 *puVar6;
  longlong *in_RDX;
  longlong *plVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *in_R8;
  longlong *in_R9;
  bool bVar10;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  char *local_b0;
  longlong local_a8;
  undefined4 local_a0;
  undefined1 uStack_9c;
  undefined1 uStack_9b;
  undefined1 uStack_9a;
  undefined1 local_99;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  char *pcStack_80;
  longlong local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  longlong *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  hashbrown::raw::RawTable::
  clear<tuple_<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple_<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>_>,alloc::alloc::Global>
            ();
  hashbrown::raw::RawTable::
  clear<tuple_<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>_>,alloc::alloc::Global>
            ();
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_108 = *in_RDX;
  uStack_100 = 1;
  do {
    plVar7 = &local_108;
    slotmap::basic::impl$9::
    next<country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle,utils::geometry::aabb::Aabb3>
              ();
  } while ((int)local_98 != 0);
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar2 = *in_R8;
  hashbrown::raw::RawTable::
  clear<tuple_<enum2_<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh>_>,alloc::alloc::Global>
            ();
  *(undefined1 *)(lVar2 + 0x38) = 1;
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar2 = *in_R9;
  for (lVar9 = 0; lVar9 != 0x1440; lVar9 = lVar9 + 0xc0) {
    if (-0x7fffffffffffffff < *(longlong *)(lVar2 + 8 + lVar9)) {
      *(undefined8 *)(lVar2 + 0x18 + lVar9) = 0;
      if (*(longlong *)(lVar2 + 0x38 + lVar9) != 0) {
        uVar3 = *(ulonglong *)(lVar2 + 0x28 + lVar9);
        if (uVar3 != 0) {
          plVar7 = (longlong *)CONCAT71((int7)((ulonglong)plVar7 >> 8),0xff);
          memset(*(void **)(lVar2 + 0x20 + lVar9),(int)plVar7,uVar3 + 0x11);
        }
        *(undefined8 *)(lVar2 + 0x38 + lVar9) = 0;
        uVar5 = (uVar3 + 1 & 0xfffffffffffffff8) - (uVar3 + 1 >> 3);
        if (uVar3 < 8) {
          uVar5 = uVar3;
        }
        *(ulonglong *)(lVar2 + 0x30 + lVar9) = uVar5;
      }
      *(undefined8 *)(lVar2 + 0xa8 + lVar9) = 0;
    }
  }
  *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar2 = *in_stack_00000030;
  puVar6 = *(undefined8 **)(lVar2 + 8);
  lVar9 = *(longlong *)(lVar2 + 0x10);
  *(undefined8 *)(lVar2 + 0x10) = 0;
  if (lVar9 != 0) {
    lVar9 = 1 - lVar9;
    do {
      plVar7 = (longlong *)*puVar6;
      LOCK();
      *plVar7 = *plVar7 + -1;
      UNLOCK();
      local_d0 = lVar9;
      local_58 = puVar6;
      if (*plVar7 == 0) {
        alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
      puVar6 = local_58 + 10;
      lVar9 = local_d0 + 1;
    } while (lVar9 != 1);
  }
  *(undefined1 *)(lVar2 + 0x18) = 0;
  lVar2 = *in_stack_00000028;
  if (lVar2 != 0) {
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    lVar9 = *(longlong *)(lVar2 + 0x10);
    if (lVar9 != 0) {
      lVar2 = *(longlong *)(lVar2 + 8);
      lVar8 = 0;
      do {
        *(undefined8 *)(lVar2 + 8 + lVar8) = 0;
        *(undefined4 *)(lVar2 + 0x10 + lVar8) = 0;
        lVar8 = lVar8 + 0x18;
      } while (lVar9 * 0x18 != lVar8);
    }
  }
  plVar7 = (longlong *)*in_stack_00000040;
  if ((int)*plVar7 == 2) {
    if (*(longlong *)*in_stack_00000038 == 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (char *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      builtin_strncpy(extraout_RAX,"snapshot:/Loading",0x11);
      local_98 = 0;
      local_88 = 0x11;
      local_78 = 0x11;
      uStack_70 = CONCAT71(uStack_70._1_7_,2);
      pcStack_80 = extraout_RAX;
    }
    else {
      resources::audio_resources::AudioCoreInner::instantiate_audio();
      if (local_98 == 2) {
        local_108 = lStack_90;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    local_a8 = local_78;
    local_a0 = (undefined4)uStack_70;
    uStack_9c = (undefined1)((ulonglong)uStack_70 >> 0x20);
    uStack_9b = (undefined1)((ulonglong)uStack_70 >> 0x28);
    uStack_9a = (undefined1)((ulonglong)uStack_70 >> 0x30);
    local_99 = uStack_70._7_1_;
    local_b8 = local_88;
    local_b0 = pcStack_80;
    local_c8 = local_98;
    lVar2 = local_c8;
    local_c0 = lStack_90;
    local_c8._0_4_ = (int)local_98;
    bVar10 = (int)local_c8 == 1;
    local_c8 = lVar2;
    if (bVar10) {
      local_49 = 1;
      libfmod::EventInstance::start();
      if (local_108 != -0x7ffffffffffffffc) {
        local_68 = local_d8;
        local_78 = local_e8;
        uStack_70 = uStack_e0;
        local_88 = local_f8;
        pcStack_80 = (char *)uStack_f0;
        local_98 = local_108;
        lStack_90 = uStack_100;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    local_78 = local_a8;
    uStack_70 = CONCAT17(local_99,CONCAT16(uStack_9a,
                                           CONCAT24(CONCAT11(uStack_9b,uStack_9c),local_a0)));
    local_88 = local_b8;
    pcStack_80 = local_b0;
    local_98 = local_c8;
    lStack_90 = local_c0;
    uVar1 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    puVar4 = (undefined4 *)in_stack_00000040[2];
    *puVar4 = uVar1;
    if ((int)*plVar7 != 2) {
      local_58 = plVar7;
      resources::audio_resources::impl$13::drop();
      plVar7 = local_58;
      if (local_58[2] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    plVar7[4] = local_78;
    plVar7[5] = uStack_70;
    plVar7[2] = local_88;
    plVar7[3] = (longlong)pcStack_80;
    *plVar7 = local_98;
    plVar7[1] = lStack_90;
    *puVar4 = uVar1;
    plVar7[6] = 1;
  }
  return;
}


