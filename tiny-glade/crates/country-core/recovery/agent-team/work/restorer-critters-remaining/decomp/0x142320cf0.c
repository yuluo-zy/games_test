
void __cdecl system_critters::sheep::sheep_floatie::sheep_floaties(void)

{
  uint *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined4 *puVar4;
  char cVar5;
  char cVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  int iVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong *in_RCX;
  ulonglong uVar24;
  longlong lVar25;
  longlong lVar26;
  longlong *in_RDX;
  uint *puVar27;
  ulonglong uVar28;
  longlong lVar29;
  longlong lVar30;
  longlong *plVar31;
  undefined8 *puVar32;
  float fVar33;
  float fVar34;
  float extraout_XMM0_Da;
  longlong *in_stack_00000028;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  longlong local_180;
  longlong local_140;
  int local_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  longlong *local_e8;
  ulonglong local_d8;
  longlong local_c8;
  ulonglong local_c0;
  longlong local_b0;
  longlong local_a8;
  
  lVar10 = *in_RCX;
  lVar11 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar27 = *(uint **)(lVar10 + 0xf8);
  puVar1 = puVar27 + *(longlong *)(lVar10 + 0x100);
  cVar5 = *(char *)(lVar10 + 0x108);
  lVar12 = *in_RDX;
  lVar13 = in_RDX[1];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  lVar14 = *in_stack_00000028;
  local_180 = 8;
  local_a8 = 0;
  local_b0 = 0;
  local_c8 = 0;
  uVar28 = 0;
  local_c0 = 0;
  do {
    do {
      do {
        if (cVar5 == '\0') {
          if (local_c0 == uVar28) {
            if (puVar27 == puVar1) {
              return;
            }
            while( true ) {
              uVar28 = *(ulonglong *)
                        (*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)*puVar27 * 0xa0);
              if (uVar28 != 0) break;
              puVar27 = puVar27 + 1;
              if (puVar27 == puVar1) {
                return;
              }
            }
            lVar25 = *(longlong *)(lVar11 + 0x100) + (ulonglong)*puVar27 * 0xa0;
            uVar24 = (ulonglong)*(uint *)(lVar25 + 0x94);
            lVar30 = *(longlong *)(lVar11 + 0x1a8) + uVar24 * 0x48;
            uVar24 = *(ulonglong *)(*(longlong *)(lVar11 + 0x1a8) + 0x40 + uVar24 * 0x48);
            if ((*(ulonglong *)(lVar10 + 0x110) < uVar24) &&
               (uVar22 = *(ulonglong *)
                          (*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
               uVar22 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar22 * 0x30);
              uVar22 = *(ulonglong *)(lVar10 + 0x118);
              lVar23 = *(longlong *)(lVar30 + 0x38);
              if (uVar24 <= uVar22) goto LAB_142320f43;
LAB_142321003:
              uVar24 = *(ulonglong *)(lVar23 + uVar22 * 8);
              lVar30 = *(longlong *)(lVar30 + 0x18);
              if (uVar24 == 0) goto LAB_142321029;
              local_a8 = *(longlong *)(lVar30 + 0x10 + ~uVar24 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar22 = *(ulonglong *)(lVar10 + 0x118);
              lVar23 = *(longlong *)(lVar30 + 0x38);
              if (uVar22 < uVar24) goto LAB_142321003;
LAB_142320f43:
              lVar30 = *(longlong *)(lVar30 + 0x18);
LAB_142321029:
              local_a8 = 0;
            }
            puVar27 = puVar27 + 1;
            lVar23 = ~*(ulonglong *)(lVar23 + *(longlong *)(lVar10 + 0x120) * 8) * 0x30;
            local_c8 = *(longlong *)(lVar30 + 0x10 + lVar23);
            local_140 = *(longlong *)(lVar30 + 0x28 + lVar23);
            local_180 = *(longlong *)(lVar25 + 0x50);
            local_c0 = 0;
          }
          uVar24 = (ulonglong)*(uint *)(local_180 + 8 + local_c0 * 0x10);
        }
        else {
          if (local_c0 == uVar28) {
            if (puVar27 == puVar1) {
              return;
            }
            while( true ) {
              uVar28 = *(ulonglong *)
                        (*(longlong *)(lVar11 + 0x1a8) + 0x10 + (ulonglong)*puVar27 * 0x48);
              if (uVar28 != 0) break;
              puVar27 = puVar27 + 1;
              if (puVar27 == puVar1) {
                return;
              }
            }
            lVar30 = *(longlong *)(lVar11 + 0x1a8) + (ulonglong)*puVar27 * 0x48;
            uVar24 = *(ulonglong *)(lVar30 + 0x40);
            if ((*(ulonglong *)(lVar10 + 0x110) < uVar24) &&
               (uVar22 = *(ulonglong *)
                          (*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
               uVar22 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar22 * 0x30);
              uVar22 = *(ulonglong *)(lVar10 + 0x118);
              lVar25 = *(longlong *)(lVar30 + 0x38);
              if (uVar24 <= uVar22) goto LAB_142320eca;
LAB_142320f6b:
              uVar24 = *(ulonglong *)(lVar25 + uVar22 * 8);
              lVar30 = *(longlong *)(lVar30 + 0x18);
              if (uVar24 == 0) goto LAB_142320f91;
              local_a8 = *(longlong *)(lVar30 + 0x10 + ~uVar24 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar22 = *(ulonglong *)(lVar10 + 0x118);
              lVar25 = *(longlong *)(lVar30 + 0x38);
              if (uVar22 < uVar24) goto LAB_142320f6b;
LAB_142320eca:
              lVar30 = *(longlong *)(lVar30 + 0x18);
LAB_142320f91:
              local_a8 = 0;
            }
            puVar27 = puVar27 + 1;
            lVar25 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar10 + 0x120) * 8) * 0x30;
            local_c8 = *(longlong *)(lVar30 + 0x10 + lVar25);
            local_140 = *(longlong *)(lVar30 + 0x28 + lVar25);
            local_c0 = 0;
          }
          uVar24 = local_c0 & 0xffffffff;
        }
        puVar2 = (undefined8 *)(local_c8 + uVar24 * 0x18);
        puVar32 = (undefined8 *)(uVar24 * 0x30 + local_b0);
        iVar9 = *(int *)(local_a8 + uVar24 * 8);
        if (*(int *)((longlong)puVar2 + 0x14) == 0) {
          uVar16 = *puVar32;
          uVar17 = puVar32[1];
          uVar18 = puVar32[2];
          uVar22 = puVar32[3];
          uVar19 = puVar32[5];
          uVar20 = *puVar2;
          uVar21 = puVar2[1];
          bevy_ecs::system::commands::Commands::spawn_empty();
          plVar31 = (longlong *)CONCAT44(uStack_ec,uStack_f0);
          if ((longlong *)CONCAT44(uStack_ec,uStack_f0) == (longlong *)0x0) {
            plVar31 = local_e8;
          }
          lVar30 = plVar31[2];
          if ((ulonglong)(*plVar31 - lVar30) < 0x78) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar25 = plVar31[1];
          *(code **)(lVar25 + lVar30) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          uStack_1a0 = (undefined4)uVar19;
          uStack_19c = (undefined4)((ulonglong)uVar19 >> 0x20);
          puVar3 = (undefined8 *)(lVar25 + 8 + lVar30);
          *puVar3 = uVar16;
          puVar3[1] = uVar17;
          puVar3 = (undefined8 *)(lVar25 + 0x18 + lVar30);
          *puVar3 = uVar18;
          puVar3[1] = uVar22 & 0xffffffff;
          puVar4 = (undefined4 *)(lVar25 + 0x28 + lVar30);
          *puVar4 = 0;
          puVar4[1] = 0;
          puVar4[2] = uStack_1a0;
          puVar4[3] = uStack_19c;
          puVar3 = (undefined8 *)(lVar25 + 0x38 + lVar30);
          *puVar3 = uVar20;
          puVar3[1] = uVar21;
          *(undefined1 *)(lVar25 + 0x48 + lVar30) = 0;
          *(ulonglong *)(lVar25 + 0x58 + lVar30) = local_d8;
          *(code **)(lVar25 + 0x68 + lVar30) = bevy_ecs::error::handler::panic;
          plVar31[2] = lVar30 + 0x78;
          if ((ulonglong)(*plVar31 - (lVar30 + 0x78)) < 0x28) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar25 = plVar31[1];
          *(code **)(lVar25 + 0x78 + lVar30) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::critters::sheep::SheepAccessory>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar25 + 0x80 + lVar30) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(lVar25 + 0x88 + lVar30) = local_d8;
          *(undefined4 *)(lVar25 + 0x90 + lVar30) = 0;
          *(bool *)(lVar25 + 0x94 + lVar30) = iVar9 == 4;
          *(undefined1 *)(lVar25 + 0x98 + lVar30) = 0;
          plVar31[2] = lVar30 + 0xa0;
          *(undefined4 *)(local_140 + uVar24 * 4) = uVar7;
          puVar2[2] = local_d8;
          uVar24 = local_d8;
          if (local_d8 >> 0x20 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
        }
        else {
          uVar24 = puVar2[2];
        }
        local_c0 = local_c0 + 1;
        uVar22 = uVar24 & 0xffffffff;
      } while ((*(ulonglong *)(lVar13 + 0x10) <= uVar22) ||
              (*(int *)(*(longlong *)(lVar13 + 8) + uVar22 * 0x14) != (int)(uVar24 >> 0x20)));
      lVar30 = *(longlong *)(lVar13 + 8) + uVar22 * 0x14;
      uVar24 = (ulonglong)*(uint *)(lVar30 + 4);
    } while ((uVar24 == 0xffffffff) ||
            ((*(ulonglong *)(lVar12 + 0x38) <= uVar24 ||
             ((*(ulonglong *)
                (*(longlong *)(lVar12 + 0x28) + (ulonglong)(*(uint *)(lVar30 + 4) >> 6) * 8) >>
               (uVar24 & 0x3f) & 1) == 0))));
    uVar24 = (ulonglong)*(uint *)(lVar30 + 0x10);
    lVar25 = *(longlong *)
              (*(longlong *)(lVar13 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar30 + 0xc) * 0x48);
    lVar30 = *(longlong *)
              (*(longlong *)(lVar13 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar30 + 0xc) * 0x48);
    lVar26 = ~*(ulonglong *)(lVar30 + *(longlong *)(lVar12 + 0x110) * 8) * 0x30;
    lVar23 = ~*(ulonglong *)(lVar30 + *(longlong *)(lVar12 + 0x118) * 8) * 0x30;
    lVar30 = *(longlong *)(lVar25 + 0x10 + lVar23);
    lVar23 = *(longlong *)(lVar25 + 0x28 + lVar23);
    lVar29 = uVar24 * 0x30;
    lVar15 = *(longlong *)(lVar25 + 0x10 + lVar26);
    lVar25 = *(longlong *)(lVar25 + 0x28 + lVar26);
    uStack_f0 = *(undefined4 *)(puVar32 + 3);
    uVar16 = puVar32[2];
    *(undefined4 *)(lVar23 + uVar24 * 4) = uVar8;
    *(undefined4 *)(lVar30 + 0x18 + lVar29) = uStack_f0;
    *(undefined8 *)(lVar30 + 0x10 + lVar29) = uVar16;
    uVar17 = *puVar32;
    uVar18 = puVar32[1];
    *(undefined4 *)(lVar23 + uVar24 * 4) = uVar8;
    *(undefined8 *)(lVar30 + lVar29) = uVar17;
    ((undefined8 *)(lVar30 + lVar29))[1] = uVar18;
    cVar6 = *(char *)(lVar15 + 4 + uVar24 * 8);
    local_f8 = (int)uVar16;
    if ((iVar9 == 4) || (cVar6 == '\0')) {
      if ((iVar9 != 4) || (cVar6 != '\0')) goto LAB_142321516;
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_f8 == 2) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      uStack_f0 = *(undefined4 *)(puVar32 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
      if (local_e8 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined4 *)(lVar25 + uVar24 * 4) = uVar8;
      *(undefined1 *)(lVar15 + 4 + uVar24 * 8) = 1;
LAB_14232153a:
      fVar33 = 2.0;
    }
    else {
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_f8 == 2) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      uStack_f0 = *(undefined4 *)(puVar32 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
      if (local_e8 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
LAB_142321516:
      *(undefined4 *)(lVar25 + uVar24 * 4) = uVar8;
      fVar33 = -2.0;
      *(bool *)(lVar15 + 4 + uVar24 * 8) = iVar9 == 4;
      if (iVar9 == 4) goto LAB_14232153a;
    }
    lVar30 = lVar30 + lVar29;
    fVar34 = *(float *)(lVar14 + 0x28);
    *(undefined4 *)(lVar25 + uVar24 * 4) = uVar8;
    fVar34 = fVar33 * fVar34 + *(float *)(lVar15 + uVar24 * 8);
    fVar33 = 0.0;
    if (0.0 <= fVar34) {
      fVar33 = fVar34;
    }
    fVar34 = 1.0;
    if (fVar33 <= 1.0) {
      fVar34 = fVar33;
    }
    *(float *)(lVar15 + uVar24 * 8) = fVar34;
    ::utils::easing_functions::EasingFunction::apply();
    fVar33 = extraout_XMM0_Da * *(float *)((longlong)puVar32 + 0x1c);
    *(undefined4 *)(lVar23 + uVar24 * 4) = uVar8;
    *(float *)(lVar30 + 0x1c) = fVar33;
    *(float *)(lVar30 + 0x20) = fVar33;
    *(float *)(lVar30 + 0x24) = fVar33;
  } while( true );
}

