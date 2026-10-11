
void __cdecl system_critters::sheep::sheep_umbrella::sheep_umbrellas(void)

{
  uint *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined4 *puVar4;
  char cVar5;
  char cVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  byte bVar23;
  ulonglong uVar24;
  longlong lVar25;
  uint uVar26;
  longlong *in_RCX;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  longlong *in_RDX;
  longlong lVar30;
  uint *puVar31;
  ulonglong uVar32;
  longlong lVar33;
  longlong *plVar34;
  undefined8 *puVar35;
  float fVar36;
  float extraout_XMM0_Da;
  float fVar37;
  longlong *in_stack_00000028;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  longlong local_180;
  longlong local_150;
  int local_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  longlong *local_f0;
  ulonglong local_e0;
  longlong local_d0;
  ulonglong local_c0;
  longlong local_b0;
  longlong local_a8;
  
  fVar22 = __real_c0000000;
  fVar21 = __real_40000000;
  fVar20 = __real_3f800000;
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar31 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar31 + *(longlong *)(lVar9 + 0x100);
  cVar5 = *(char *)(lVar9 + 0x108);
  lVar11 = *in_RDX;
  lVar12 = in_RDX[1];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  lVar13 = *in_stack_00000028;
  local_180 = 8;
  local_a8 = 0;
  local_b0 = 0;
  local_d0 = 0;
  uVar32 = 0;
  local_c0 = 0;
  do {
    do {
      do {
        if (cVar5 == '\0') {
          if (local_c0 == uVar32) {
            if (puVar31 == puVar1) {
              return;
            }
            while( true ) {
              uVar32 = *(ulonglong *)
                        (*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)*puVar31 * 0xa0);
              if (uVar32 != 0) break;
              puVar31 = puVar31 + 1;
              if (puVar31 == puVar1) {
                return;
              }
            }
            lVar28 = *(longlong *)(lVar10 + 0x100) + (ulonglong)*puVar31 * 0xa0;
            uVar27 = (ulonglong)*(uint *)(lVar28 + 0x94);
            lVar33 = *(longlong *)(lVar10 + 0x1a8) + uVar27 * 0x48;
            uVar27 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar27 * 0x48);
            if ((*(ulonglong *)(lVar9 + 0x110) < uVar27) &&
               (uVar24 = *(ulonglong *)
                          (*(longlong *)(lVar33 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
               uVar24 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar33 + 0x18) + 0x10 + ~uVar24 * 0x30);
              uVar24 = *(ulonglong *)(lVar9 + 0x118);
              lVar30 = *(longlong *)(lVar33 + 0x38);
              if (uVar27 <= uVar24) goto LAB_142321be0;
LAB_142321ca7:
              uVar27 = *(ulonglong *)(lVar30 + uVar24 * 8);
              lVar33 = *(longlong *)(lVar33 + 0x18);
              if (uVar27 == 0) goto LAB_142321ccd;
              local_a8 = *(longlong *)(lVar33 + 0x10 + ~uVar27 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar24 = *(ulonglong *)(lVar9 + 0x118);
              lVar30 = *(longlong *)(lVar33 + 0x38);
              if (uVar24 < uVar27) goto LAB_142321ca7;
LAB_142321be0:
              lVar33 = *(longlong *)(lVar33 + 0x18);
LAB_142321ccd:
              local_a8 = 0;
            }
            puVar31 = puVar31 + 1;
            lVar30 = ~*(ulonglong *)(lVar30 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
            local_d0 = *(longlong *)(lVar33 + 0x10 + lVar30);
            local_150 = *(longlong *)(lVar33 + 0x28 + lVar30);
            local_180 = *(longlong *)(lVar28 + 0x50);
            local_c0 = 0;
          }
          uVar27 = (ulonglong)*(uint *)(local_180 + 8 + local_c0 * 0x10);
        }
        else {
          if (local_c0 == uVar32) {
            if (puVar31 == puVar1) {
              return;
            }
            while( true ) {
              uVar32 = *(ulonglong *)
                        (*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)*puVar31 * 0x48);
              if (uVar32 != 0) break;
              puVar31 = puVar31 + 1;
              if (puVar31 == puVar1) {
                return;
              }
            }
            lVar33 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)*puVar31 * 0x48;
            uVar27 = *(ulonglong *)(lVar33 + 0x40);
            if ((*(ulonglong *)(lVar9 + 0x110) < uVar27) &&
               (uVar24 = *(ulonglong *)
                          (*(longlong *)(lVar33 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
               uVar24 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar33 + 0x18) + 0x10 + ~uVar24 * 0x30);
              uVar24 = *(ulonglong *)(lVar9 + 0x118);
              lVar28 = *(longlong *)(lVar33 + 0x38);
              if (uVar27 <= uVar24) goto LAB_142321b6a;
LAB_142321c08:
              uVar27 = *(ulonglong *)(lVar28 + uVar24 * 8);
              lVar33 = *(longlong *)(lVar33 + 0x18);
              if (uVar27 == 0) goto LAB_142321c2e;
              local_a8 = *(longlong *)(lVar33 + 0x10 + ~uVar27 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar24 = *(ulonglong *)(lVar9 + 0x118);
              lVar28 = *(longlong *)(lVar33 + 0x38);
              if (uVar24 < uVar27) goto LAB_142321c08;
LAB_142321b6a:
              lVar33 = *(longlong *)(lVar33 + 0x18);
LAB_142321c2e:
              local_a8 = 0;
            }
            puVar31 = puVar31 + 1;
            lVar28 = ~*(ulonglong *)(lVar28 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
            local_d0 = *(longlong *)(lVar33 + 0x10 + lVar28);
            local_150 = *(longlong *)(lVar33 + 0x28 + lVar28);
            local_c0 = 0;
          }
          uVar27 = local_c0 & 0xffffffff;
        }
        puVar2 = (undefined8 *)(local_d0 + uVar27 * 0x18);
        puVar35 = (undefined8 *)(uVar27 * 0x30 + local_b0);
        uVar26 = *(int *)(local_a8 + uVar27 * 8) - 2;
        bVar23 = (byte)uVar26;
        if (4 < uVar26) {
          bVar23 = 3;
        }
        bVar23 = 0x1a >> (bVar23 & 0x1f);
        if (*(int *)((longlong)puVar2 + 0x14) == 0) {
          uVar14 = *puVar35;
          uVar15 = puVar35[1];
          uVar16 = puVar35[2];
          uVar24 = puVar35[3];
          uVar17 = puVar35[5];
          uVar18 = *puVar2;
          uVar19 = puVar2[1];
          bevy_ecs::system::commands::Commands::spawn_empty();
          plVar34 = (longlong *)CONCAT44(uStack_f4,uStack_f8);
          if ((longlong *)CONCAT44(uStack_f4,uStack_f8) == (longlong *)0x0) {
            plVar34 = local_f0;
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
          uStack_1b0 = (undefined4)uVar17;
          uStack_1ac = (undefined4)((ulonglong)uVar17 >> 0x20);
          puVar3 = (undefined8 *)(lVar28 + 8 + lVar33);
          *puVar3 = uVar14;
          puVar3[1] = uVar15;
          puVar3 = (undefined8 *)(lVar28 + 0x18 + lVar33);
          *puVar3 = uVar16;
          puVar3[1] = uVar24 & 0xffffffff;
          puVar4 = (undefined4 *)(lVar28 + 0x28 + lVar33);
          *puVar4 = 0;
          puVar4[1] = 0;
          puVar4[2] = uStack_1b0;
          puVar4[3] = uStack_1ac;
          puVar3 = (undefined8 *)(lVar28 + 0x38 + lVar33);
          *puVar3 = uVar18;
          puVar3[1] = uVar19;
          *(undefined1 *)(lVar28 + 0x48 + lVar33) = 0;
          *(ulonglong *)(lVar28 + 0x58 + lVar33) = local_e0;
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
          *(ulonglong *)(lVar28 + 0x88 + lVar33) = local_e0;
          *(undefined4 *)(lVar28 + 0x90 + lVar33) = 0;
          *(byte *)(lVar28 + 0x94 + lVar33) = bVar23 & 1;
          *(undefined1 *)(lVar28 + 0x98 + lVar33) = 0;
          plVar34[2] = lVar33 + 0xa0;
          *(undefined4 *)(local_150 + uVar27 * 4) = uVar7;
          puVar2[2] = local_e0;
          uVar27 = local_e0;
          if (local_e0 >> 0x20 == 0) {
                    /* WARNING: Subroutine does not return */
            core::option::unwrap_failed();
          }
        }
        else {
          uVar27 = puVar2[2];
        }
        local_c0 = local_c0 + 1;
        uVar24 = uVar27 & 0xffffffff;
      } while ((*(ulonglong *)(lVar12 + 0x10) <= uVar24) ||
              (*(int *)(*(longlong *)(lVar12 + 8) + uVar24 * 0x14) != (int)(uVar27 >> 0x20)));
      lVar33 = *(longlong *)(lVar12 + 8) + uVar24 * 0x14;
      uVar27 = (ulonglong)*(uint *)(lVar33 + 4);
    } while ((uVar27 == 0xffffffff) ||
            ((*(ulonglong *)(lVar11 + 0x38) <= uVar27 ||
             ((*(ulonglong *)
                (*(longlong *)(lVar11 + 0x28) + (ulonglong)(*(uint *)(lVar33 + 4) >> 6) * 8) >>
               (uVar27 & 0x3f) & 1) == 0))));
    uVar27 = (ulonglong)*(uint *)(lVar33 + 0x10);
    lVar28 = *(longlong *)
              (*(longlong *)(lVar12 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar33 + 0xc) * 0x48);
    lVar33 = *(longlong *)
              (*(longlong *)(lVar12 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar33 + 0xc) * 0x48);
    lVar29 = ~*(ulonglong *)(lVar33 + *(longlong *)(lVar11 + 0x110) * 8) * 0x30;
    lVar30 = *(longlong *)(lVar28 + 0x10 + lVar29);
    lVar29 = *(longlong *)(lVar28 + 0x28 + lVar29);
    lVar25 = ~*(ulonglong *)(lVar33 + *(longlong *)(lVar11 + 0x118) * 8) * 0x30;
    lVar33 = *(longlong *)(lVar28 + 0x10 + lVar25);
    lVar28 = *(longlong *)(lVar28 + 0x28 + lVar25);
    lVar25 = uVar27 * 0x30;
    uStack_f8 = *(undefined4 *)(puVar35 + 3);
    uVar14 = puVar35[2];
    *(undefined4 *)(lVar28 + uVar27 * 4) = uVar8;
    *(undefined4 *)(lVar33 + 0x18 + lVar25) = uStack_f8;
    *(undefined8 *)(lVar33 + 0x10 + lVar25) = uVar14;
    uVar15 = *puVar35;
    uVar16 = puVar35[1];
    *(undefined4 *)(lVar28 + uVar27 * 4) = uVar8;
    *(undefined8 *)(lVar33 + lVar25) = uVar15;
    ((undefined8 *)(lVar33 + lVar25))[1] = uVar16;
    cVar6 = *(char *)(lVar30 + 4 + uVar27 * 8);
    local_100 = (int)uVar14;
    if (((bVar23 & 1) == 0) && (cVar6 != '\0')) {
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_100 == 2) {
                    /* WARNING: Subroutine does not return */
        core::result::unwrap_failed();
      }
      uStack_f8 = *(undefined4 *)(puVar35 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
LAB_1423221a3:
      if (local_f0 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else if (((bVar23 & 1) != 0) && (cVar6 == '\0')) {
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_100 == 2) {
                    /* WARNING: Subroutine does not return */
        core::result::unwrap_failed();
      }
      uStack_f8 = *(undefined4 *)(puVar35 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
      goto LAB_1423221a3;
    }
    *(undefined4 *)(lVar29 + uVar27 * 4) = uVar8;
    *(byte *)(lVar30 + 4 + uVar27 * 8) = bVar23 & 1;
    fVar37 = fVar21;
    if ((bVar23 & 1) == 0) {
      fVar37 = fVar22;
    }
    lVar33 = lVar33 + lVar25;
    fVar36 = *(float *)(lVar13 + 0x28);
    *(undefined4 *)(lVar29 + uVar27 * 4) = uVar8;
    fVar36 = fVar37 * fVar36 + *(float *)(lVar30 + uVar27 * 8);
    fVar37 = 0.0;
    if (0.0 <= fVar36) {
      fVar37 = fVar36;
    }
    fVar36 = fVar20;
    if (fVar37 <= fVar20) {
      fVar36 = fVar37;
    }
    *(float *)(lVar30 + uVar27 * 8) = fVar36;
    ::utils::easing_functions::EasingFunction::apply();
    fVar37 = extraout_XMM0_Da * *(float *)((longlong)puVar35 + 0x1c);
    *(undefined4 *)(lVar28 + uVar27 * 4) = uVar8;
    *(float *)(lVar33 + 0x1c) = fVar37;
    *(float *)(lVar33 + 0x20) = fVar37;
    *(float *)(lVar33 + 0x24) = fVar37;
  } while( true );
}

