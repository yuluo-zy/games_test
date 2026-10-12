
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
  float fVar22;
  float fVar23;
  float fVar24;
  ulonglong uVar25;
  longlong lVar26;
  longlong *in_RCX;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  longlong *in_RDX;
  uint *puVar30;
  ulonglong uVar31;
  longlong lVar32;
  longlong lVar33;
  longlong *plVar34;
  undefined8 *puVar35;
  float fVar36;
  float extraout_XMM0_Da;
  float fVar37;
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
  
  fVar24 = __real_c0000000;
  fVar23 = __real_40000000;
  fVar22 = __real_3f800000;
  lVar10 = *in_RCX;
  lVar11 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar30 = *(uint **)(lVar10 + 0xf8);
  puVar1 = puVar30 + *(longlong *)(lVar10 + 0x100);
  cVar5 = *(char *)(lVar10 + 0x108);
  lVar12 = *in_RDX;
  lVar13 = in_RDX[1];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  lVar14 = *in_stack_00000028;
  local_180 = 8;
  local_a8 = 0;
  local_b0 = 0;
  local_c8 = 0;
  uVar31 = 0;
  local_c0 = 0;
  do {
    do {
      do {
        if (cVar5 == '\0') {
          if (local_c0 == uVar31) {
            if (puVar30 == puVar1) {
              return;
            }
            while( true ) {
              uVar31 = *(ulonglong *)
                        (*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)*puVar30 * 0xa0);
              if (uVar31 != 0) break;
              puVar30 = puVar30 + 1;
              if (puVar30 == puVar1) {
                return;
              }
            }
            lVar28 = *(longlong *)(lVar11 + 0x100) + (ulonglong)*puVar30 * 0xa0;
            uVar27 = (ulonglong)*(uint *)(lVar28 + 0x94);
            lVar33 = *(longlong *)(lVar11 + 0x1a8) + uVar27 * 0x48;
            uVar27 = *(ulonglong *)(*(longlong *)(lVar11 + 0x1a8) + 0x40 + uVar27 * 0x48);
            if ((*(ulonglong *)(lVar10 + 0x110) < uVar27) &&
               (uVar25 = *(ulonglong *)
                          (*(longlong *)(lVar33 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
               uVar25 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar33 + 0x18) + 0x10 + ~uVar25 * 0x30);
              uVar25 = *(ulonglong *)(lVar10 + 0x118);
              lVar26 = *(longlong *)(lVar33 + 0x38);
              if (uVar27 <= uVar25) goto LAB_142320f43;
LAB_142321003:
              uVar27 = *(ulonglong *)(lVar26 + uVar25 * 8);
              lVar33 = *(longlong *)(lVar33 + 0x18);
              if (uVar27 == 0) goto LAB_142321029;
              local_a8 = *(longlong *)(lVar33 + 0x10 + ~uVar27 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar25 = *(ulonglong *)(lVar10 + 0x118);
              lVar26 = *(longlong *)(lVar33 + 0x38);
              if (uVar25 < uVar27) goto LAB_142321003;
LAB_142320f43:
              lVar33 = *(longlong *)(lVar33 + 0x18);
LAB_142321029:
              local_a8 = 0;
            }
            puVar30 = puVar30 + 1;
            lVar26 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar10 + 0x120) * 8) * 0x30;
            local_c8 = *(longlong *)(lVar33 + 0x10 + lVar26);
            local_140 = *(longlong *)(lVar33 + 0x28 + lVar26);
            local_180 = *(longlong *)(lVar28 + 0x50);
            local_c0 = 0;
          }
          uVar27 = (ulonglong)*(uint *)(local_180 + 8 + local_c0 * 0x10);
        }
        else {
          if (local_c0 == uVar31) {
            if (puVar30 == puVar1) {
              return;
            }
            while( true ) {
              uVar31 = *(ulonglong *)
                        (*(longlong *)(lVar11 + 0x1a8) + 0x10 + (ulonglong)*puVar30 * 0x48);
              if (uVar31 != 0) break;
              puVar30 = puVar30 + 1;
              if (puVar30 == puVar1) {
                return;
              }
            }
            lVar33 = *(longlong *)(lVar11 + 0x1a8) + (ulonglong)*puVar30 * 0x48;
            uVar27 = *(ulonglong *)(lVar33 + 0x40);
            if ((*(ulonglong *)(lVar10 + 0x110) < uVar27) &&
               (uVar25 = *(ulonglong *)
                          (*(longlong *)(lVar33 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
               uVar25 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar33 + 0x18) + 0x10 + ~uVar25 * 0x30);
              uVar25 = *(ulonglong *)(lVar10 + 0x118);
              lVar28 = *(longlong *)(lVar33 + 0x38);
              if (uVar27 <= uVar25) goto LAB_142320eca;
LAB_142320f6b:
              uVar27 = *(ulonglong *)(lVar28 + uVar25 * 8);
              lVar33 = *(longlong *)(lVar33 + 0x18);
              if (uVar27 == 0) goto LAB_142320f91;
              local_a8 = *(longlong *)(lVar33 + 0x10 + ~uVar27 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar25 = *(ulonglong *)(lVar10 + 0x118);
              lVar28 = *(longlong *)(lVar33 + 0x38);
              if (uVar25 < uVar27) goto LAB_142320f6b;
LAB_142320eca:
              lVar33 = *(longlong *)(lVar33 + 0x18);
LAB_142320f91:
              local_a8 = 0;
            }
            puVar30 = puVar30 + 1;
            lVar28 = ~*(ulonglong *)(lVar28 + *(longlong *)(lVar10 + 0x120) * 8) * 0x30;
            local_c8 = *(longlong *)(lVar33 + 0x10 + lVar28);
            local_140 = *(longlong *)(lVar33 + 0x28 + lVar28);
            local_c0 = 0;
          }
          uVar27 = local_c0 & 0xffffffff;
        }
        puVar2 = (undefined8 *)(local_c8 + uVar27 * 0x18);
        puVar35 = (undefined8 *)(uVar27 * 0x30 + local_b0);
        iVar9 = *(int *)(local_a8 + uVar27 * 8);
        if (*(int *)((longlong)puVar2 + 0x14) == 0) {
          uVar16 = *puVar35;
          uVar17 = puVar35[1];
          uVar18 = puVar35[2];
          uVar25 = puVar35[3];
          uVar19 = puVar35[5];
          uVar20 = *puVar2;
          uVar21 = puVar2[1];
          bevy_ecs::system::commands::Commands::spawn_empty();
          plVar34 = (longlong *)CONCAT44(uStack_ec,uStack_f0);
          if ((longlong *)CONCAT44(uStack_ec,uStack_f0) == (longlong *)0x0) {
            plVar34 = local_e8;
          }
          lVar33 = plVar34[2];
          if ((ulonglong)(*plVar34 - lVar33) < 0x78) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar28 = plVar34[1];
          *(code **)(lVar28 + lVar33) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          uStack_1a0 = (undefined4)uVar19;
          uStack_19c = (undefined4)((ulonglong)uVar19 >> 0x20);
          puVar3 = (undefined8 *)(lVar28 + 8 + lVar33);
          *puVar3 = uVar16;
          puVar3[1] = uVar17;
          puVar3 = (undefined8 *)(lVar28 + 0x18 + lVar33);
          *puVar3 = uVar18;
          puVar3[1] = uVar25 & 0xffffffff;
          puVar4 = (undefined4 *)(lVar28 + 0x28 + lVar33);
          *puVar4 = 0;
          puVar4[1] = 0;
          puVar4[2] = uStack_1a0;
          puVar4[3] = uStack_19c;
          puVar3 = (undefined8 *)(lVar28 + 0x38 + lVar33);
          *puVar3 = uVar20;
          puVar3[1] = uVar21;
          *(undefined1 *)(lVar28 + 0x48 + lVar33) = 0;
          *(ulonglong *)(lVar28 + 0x58 + lVar33) = local_d8;
          *(code **)(lVar28 + 0x68 + lVar33) = bevy_ecs::error::handler::panic;
          plVar34[2] = lVar33 + 0x78;
          if ((ulonglong)(*plVar34 - (lVar33 + 0x78)) < 0x28) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar28 = plVar34[1];
          *(code **)(lVar28 + 0x78 + lVar33) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::critters::sheep::SheepAccessory>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar28 + 0x80 + lVar33) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(lVar28 + 0x88 + lVar33) = local_d8;
          *(undefined4 *)(lVar28 + 0x90 + lVar33) = 0;
          *(bool *)(lVar28 + 0x94 + lVar33) = iVar9 == 4;
          *(undefined1 *)(lVar28 + 0x98 + lVar33) = 0;
          plVar34[2] = lVar33 + 0xa0;
          *(undefined4 *)(local_140 + uVar27 * 4) = uVar7;
          puVar2[2] = local_d8;
          uVar27 = local_d8;
          if (local_d8 >> 0x20 == 0) {
                    /* WARNING: Subroutine does not return */
            core::option::unwrap_failed();
          }
        }
        else {
          uVar27 = puVar2[2];
        }
        local_c0 = local_c0 + 1;
        uVar25 = uVar27 & 0xffffffff;
      } while ((*(ulonglong *)(lVar13 + 0x10) <= uVar25) ||
              (*(int *)(*(longlong *)(lVar13 + 8) + uVar25 * 0x14) != (int)(uVar27 >> 0x20)));
      lVar33 = *(longlong *)(lVar13 + 8) + uVar25 * 0x14;
      uVar27 = (ulonglong)*(uint *)(lVar33 + 4);
    } while ((uVar27 == 0xffffffff) ||
            ((*(ulonglong *)(lVar12 + 0x38) <= uVar27 ||
             ((*(ulonglong *)
                (*(longlong *)(lVar12 + 0x28) + (ulonglong)(*(uint *)(lVar33 + 4) >> 6) * 8) >>
               (uVar27 & 0x3f) & 1) == 0))));
    uVar27 = (ulonglong)*(uint *)(lVar33 + 0x10);
    lVar28 = *(longlong *)
              (*(longlong *)(lVar13 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar33 + 0xc) * 0x48);
    lVar33 = *(longlong *)
              (*(longlong *)(lVar13 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar33 + 0xc) * 0x48);
    lVar29 = ~*(ulonglong *)(lVar33 + *(longlong *)(lVar12 + 0x110) * 8) * 0x30;
    lVar26 = ~*(ulonglong *)(lVar33 + *(longlong *)(lVar12 + 0x118) * 8) * 0x30;
    lVar33 = *(longlong *)(lVar28 + 0x10 + lVar26);
    lVar26 = *(longlong *)(lVar28 + 0x28 + lVar26);
    lVar32 = uVar27 * 0x30;
    lVar15 = *(longlong *)(lVar28 + 0x10 + lVar29);
    lVar28 = *(longlong *)(lVar28 + 0x28 + lVar29);
    uStack_f0 = *(undefined4 *)(puVar35 + 3);
    uVar16 = puVar35[2];
    *(undefined4 *)(lVar26 + uVar27 * 4) = uVar8;
    *(undefined4 *)(lVar33 + 0x18 + lVar32) = uStack_f0;
    *(undefined8 *)(lVar33 + 0x10 + lVar32) = uVar16;
    uVar17 = *puVar35;
    uVar18 = puVar35[1];
    *(undefined4 *)(lVar26 + uVar27 * 4) = uVar8;
    *(undefined8 *)(lVar33 + lVar32) = uVar17;
    ((undefined8 *)(lVar33 + lVar32))[1] = uVar18;
    cVar6 = *(char *)(lVar15 + 4 + uVar27 * 8);
    local_f8 = (int)uVar16;
    if ((iVar9 == 4) || (cVar6 == '\0')) {
      if ((iVar9 != 4) || (cVar6 != '\0')) goto LAB_142321516;
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_f8 == 2) {
                    /* WARNING: Subroutine does not return */
        core::result::unwrap_failed();
      }
      uStack_f0 = *(undefined4 *)(puVar35 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
      if (local_e8 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined4 *)(lVar28 + uVar27 * 4) = uVar8;
      *(undefined1 *)(lVar15 + 4 + uVar27 * 8) = 1;
LAB_14232153a:
      fVar37 = fVar23;
    }
    else {
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_f8 == 2) {
                    /* WARNING: Subroutine does not return */
        core::result::unwrap_failed();
      }
      uStack_f0 = *(undefined4 *)(puVar35 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
      if (local_e8 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
LAB_142321516:
      *(undefined4 *)(lVar28 + uVar27 * 4) = uVar8;
      *(bool *)(lVar15 + 4 + uVar27 * 8) = iVar9 == 4;
      fVar37 = fVar24;
      if (iVar9 == 4) goto LAB_14232153a;
    }
    lVar33 = lVar33 + lVar32;
    fVar36 = *(float *)(lVar14 + 0x28);
    *(undefined4 *)(lVar28 + uVar27 * 4) = uVar8;
    fVar36 = fVar37 * fVar36 + *(float *)(lVar15 + uVar27 * 8);
    fVar37 = 0.0;
    if (0.0 <= fVar36) {
      fVar37 = fVar36;
    }
    fVar36 = fVar22;
    if (fVar37 <= fVar22) {
      fVar36 = fVar37;
    }
    *(float *)(lVar15 + uVar27 * 8) = fVar36;
    ::utils::easing_functions::EasingFunction::apply();
    fVar37 = extraout_XMM0_Da * *(float *)((longlong)puVar35 + 0x1c);
    *(undefined4 *)(lVar26 + uVar27 * 4) = uVar8;
    *(float *)(lVar33 + 0x1c) = fVar37;
    *(float *)(lVar33 + 0x20) = fVar37;
    *(float *)(lVar33 + 0x24) = fVar37;
  } while( true );
}

