
void __cdecl system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay(void)

{
  uint *puVar1;
  uint uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  ulonglong uVar6;
  longlong extraout_RAX;
  longlong lVar7;
  longlong lVar8;
  undefined4 *puVar9;
  longlong *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *in_RCX;
  undefined8 *puVar10;
  longlong *in_RDX;
  ulonglong uVar11;
  longlong lVar12;
  undefined4 *puVar13;
  undefined8 *puVar14;
  undefined8 *in_R8;
  longlong *plVar15;
  longlong *in_R9;
  uint *puVar16;
  longlong local_e8;
  longlong local_e0;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined8 *local_d0;
  undefined8 *local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  undefined4 local_a0;
  undefined4 local_9c;
  ulonglong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  undefined8 *local_78;
  longlong lStack_70;
  longlong local_68;
  undefined4 local_60;
  undefined4 local_5c [4];
  char local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  uVar11 = *(ulonglong *)(*in_RDX + 0x40) ^ 0x8000000000000000;
  uVar6 = 1;
  if (uVar11 < 5) {
    uVar6 = uVar11;
  }
  if (uVar6 < 2) {
    alloc::vec::spec_from_iter::impl$0::
    from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref_<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>_>,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env_0>_>
              ();
    country_core::input::input_stream::InputStream::last_frame_mut();
    if (extraout_RAX_00 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    if (*extraout_RAX_00 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    extraout_RAX_00[2] = local_68;
    *extraout_RAX_00 = (longlong)local_78;
    extraout_RAX_00[1] = lStack_70;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref_<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>_>,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env_1>_>
              ();
    country_core::input::input_stream::InputStream::last_frame_mut();
    if (extraout_RAX_01 == 0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    if (*(longlong *)(extraout_RAX_01 + 0x18) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(longlong *)(extraout_RAX_01 + 0x28) = local_68;
    *(undefined8 **)(extraout_RAX_01 + 0x18) = local_78;
    *(longlong *)(extraout_RAX_01 + 0x20) = lStack_70;
  }
  else if ((1 < uVar6 - 3) && ((*(char *)(*in_R9 + 0xfb) != '\0' || (*(char *)*in_R8 == '\x01')))) {
    country_core::input::input_stream::impl$2::index();
    local_a8 = *in_RCX;
    lVar5 = in_RCX[1];
    local_a0 = (int)in_RCX[2];
    local_9c = *(undefined4 *)((longlong)in_RCX + 0x14);
    puVar16 = *(uint **)(local_a8 + 0xf8);
    puVar1 = puVar16 + *(longlong *)(local_a8 + 0x100);
    local_49 = *(char *)(local_a8 + 0x108);
    local_d0 = *(undefined8 **)(extraout_RAX + 8);
    local_c8 = (undefined8 *)((longlong)local_d0 + *(longlong *)(extraout_RAX + 0x10) * 0xc);
    local_b8 = *(longlong *)(extraout_RAX + 0x20);
    local_c0 = *(longlong *)(extraout_RAX + 0x20) + *(longlong *)(extraout_RAX + 0x28) * 4;
    local_b0 = 8;
    local_80 = 0;
    uVar6 = 0;
    local_98 = 0;
    if (local_49 == '\0') goto LAB_14231b940;
    do {
      if (local_98 == uVar6) {
        do {
          if (puVar16 == puVar1) {
            return;
          }
          uVar2 = *puVar16;
          puVar16 = puVar16 + 1;
          uVar6 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)uVar2 * 0x48);
        } while (uVar6 == 0);
        lVar7 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)uVar2 * 0x48;
        lVar8 = *(longlong *)(lVar7 + 0x18);
        lVar7 = ~*(ulonglong *)(*(longlong *)(lVar7 + 0x38) + *(longlong *)(local_a8 + 0x110) * 8) *
                0x30;
        local_80 = *(longlong *)(lVar8 + 0x10 + lVar7);
        local_90 = *(longlong *)(lVar8 + 0x20 + lVar7);
        local_88 = *(longlong *)(lVar8 + 0x28 + lVar7);
        local_98 = 0;
      }
      uVar11 = local_98 & 0xffffffff;
      puVar14 = (undefined8 *)(uVar11 * 0x30 + local_80);
      local_e8 = local_90 + uVar11 * 4;
      local_e0 = local_88 + uVar11 * 4;
      puVar9 = &local_d8;
      puVar13 = &local_d4;
      plVar15 = &local_e8;
      puVar10 = local_d0;
      uVar11 = local_98;
      while( true ) {
        *puVar9 = local_a0;
        *puVar13 = local_9c;
        if (puVar14 == (undefined8 *)0x0) {
          return;
        }
        if (puVar10 == local_c8) {
          return;
        }
        if (local_b8 == local_c0) {
          return;
        }
        puVar9 = (undefined4 *)plVar15[1];
        uVar3 = *(undefined4 *)((longlong)plVar15 + 0x14);
        local_98 = uVar11 + 1;
        local_d0 = (undefined8 *)((longlong)puVar10 + 0xc);
        uVar4 = *(undefined4 *)(puVar10 + 1);
        lStack_70 = CONCAT44(lStack_70._4_4_,uVar4);
        local_78 = (undefined8 *)*puVar10;
        *puVar9 = uVar3;
        *(undefined4 *)(puVar14 + 3) = uVar4;
        puVar14[2] = local_78;
        local_b8 = local_b8 + 4;
        glam::euler::impl$46::from_euler_angles();
        *puVar9 = uVar3;
        *puVar14 = local_78;
        puVar14[1] = lStack_70;
        if (local_49 != '\0') break;
LAB_14231b940:
        if (local_98 == uVar6) {
          do {
            if (puVar16 == puVar1) {
              return;
            }
            uVar2 = *puVar16;
            puVar16 = puVar16 + 1;
            uVar6 = *(ulonglong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)uVar2 * 0xa0);
          } while (uVar6 == 0);
          lVar8 = *(longlong *)(lVar5 + 0x100) + (ulonglong)uVar2 * 0xa0;
          uVar11 = (ulonglong)*(uint *)(lVar8 + 0x94);
          lVar7 = *(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x18 + uVar11 * 0x48);
          lVar12 = ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x38 + uVar11 * 0x48) +
                     *(longlong *)(local_a8 + 0x110) * 8) * 0x30;
          local_80 = *(longlong *)(lVar7 + 0x10 + lVar12);
          local_90 = *(longlong *)(lVar7 + 0x20 + lVar12);
          local_88 = *(longlong *)(lVar7 + 0x28 + lVar12);
          local_b0 = *(longlong *)(lVar8 + 0x50);
          local_98 = 0;
        }
        uVar11 = (ulonglong)*(uint *)(local_b0 + 8 + local_98 * 0x10);
        puVar14 = (undefined8 *)(uVar11 * 0x30 + local_80);
        local_78 = puVar14;
        lStack_70 = local_90 + uVar11 * 4;
        local_68 = local_88 + uVar11 * 4;
        puVar9 = &local_60;
        puVar13 = local_5c;
        plVar15 = &lStack_70;
        puVar10 = local_d0;
        uVar11 = local_98;
      }
    } while( true );
  }
  return;
}

