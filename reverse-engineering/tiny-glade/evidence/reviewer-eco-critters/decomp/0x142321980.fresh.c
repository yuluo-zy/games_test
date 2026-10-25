// sheep_umbrellas 0x142321980 sig=void __cdecl sheep_umbrellas(void)

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
  byte bVar20;
  ulonglong uVar21;
  longlong lVar22;
  uint uVar23;
  longlong *in_RCX;
  ulonglong uVar24;
  longlong lVar25;
  longlong lVar26;
  longlong *in_RDX;
  longlong lVar27;
  uint *puVar28;
  ulonglong uVar29;
  longlong lVar30;
  longlong *plVar31;
  undefined8 *puVar32;
  float fVar33;
  float fVar34;
  float extraout_XMM0_Da;
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
  
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar28 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar28 + *(longlong *)(lVar9 + 0x100);
  cVar5 = *(char *)(lVar9 + 0x108);
  lVar11 = *in_RDX;
  lVar12 = in_RDX[1];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  lVar13 = *in_stack_00000028;
  local_180 = 8;
  local_a8 = 0;
  local_b0 = 0;
  local_d0 = 0;
  uVar29 = 0;
  local_c0 = 0;
  do {
    do {
      do {
        if (cVar5 == '\0') {
          if (local_c0 == uVar29) {
            if (puVar28 == puVar1) {
              return;
            }
            while( true ) {
              uVar29 = *(ulonglong *)
                        (*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)*puVar28 * 0xa0);
              if (uVar29 != 0) break;
              puVar28 = puVar28 + 1;
              if (puVar28 == puVar1) {
                return;
              }
            }
            lVar25 = *(longlong *)(lVar10 + 0x100) + (ulonglong)*puVar28 * 0xa0;
            uVar24 = (ulonglong)*(uint *)(lVar25 + 0x94);
            lVar30 = *(longlong *)(lVar10 + 0x1a8) + uVar24 * 0x48;
            uVar24 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar24 * 0x48);
            if ((*(ulonglong *)(lVar9 + 0x110) < uVar24) &&
               (uVar21 = *(ulonglong *)
                          (*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
               uVar21 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar21 * 0x30);
              uVar21 = *(ulonglong *)(lVar9 + 0x118);
              lVar27 = *(longlong *)(lVar30 + 0x38);
              if (uVar24 <= uVar21) goto LAB_142321be0;
LAB_142321ca7:
              uVar24 = *(ulonglong *)(lVar27 + uVar21 * 8);
              lVar30 = *(longlong *)(lVar30 + 0x18);
              if (uVar24 == 0) goto LAB_142321ccd;
              local_a8 = *(longlong *)(lVar30 + 0x10 + ~uVar24 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar21 = *(ulonglong *)(lVar9 + 0x118);
              lVar27 = *(longlong *)(lVar30 + 0x38);
              if (uVar21 < uVar24) goto LAB_142321ca7;
LAB_142321be0:
              lVar30 = *(longlong *)(lVar30 + 0x18);
LAB_142321ccd:
              local_a8 = 0;
            }
            puVar28 = puVar28 + 1;
            lVar27 = ~*(ulonglong *)(lVar27 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
            local_d0 = *(longlong *)(lVar30 + 0x10 + lVar27);
            local_150 = *(longlong *)(lVar30 + 0x28 + lVar27);
            local_180 = *(longlong *)(lVar25 + 0x50);
            local_c0 = 0;
          }
          uVar24 = (ulonglong)*(uint *)(local_180 + 8 + local_c0 * 0x10);
        }
        else {
          if (local_c0 == uVar29) {
            if (puVar28 == puVar1) {
              return;
            }
            while( true ) {
              uVar29 = *(ulonglong *)
                        (*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)*puVar28 * 0x48);
              if (uVar29 != 0) break;
              puVar28 = puVar28 + 1;
              if (puVar28 == puVar1) {
                return;
              }
            }
            lVar30 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)*puVar28 * 0x48;
            uVar24 = *(ulonglong *)(lVar30 + 0x40);
            if ((*(ulonglong *)(lVar9 + 0x110) < uVar24) &&
               (uVar21 = *(ulonglong *)
                          (*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
               uVar21 != 0)) {
              local_b0 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar21 * 0x30);
              uVar21 = *(ulonglong *)(lVar9 + 0x118);
              lVar25 = *(longlong *)(lVar30 + 0x38);
              if (uVar24 <= uVar21) goto LAB_142321b6a;
LAB_142321c08:
              uVar24 = *(ulonglong *)(lVar25 + uVar21 * 8);
              lVar30 = *(longlong *)(lVar30 + 0x18);
              if (uVar24 == 0) goto LAB_142321c2e;
              local_a8 = *(longlong *)(lVar30 + 0x10 + ~uVar24 * 0x30);
            }
            else {
              local_b0 = 0;
              uVar21 = *(ulonglong *)(lVar9 + 0x118);
              lVar25 = *(longlong *)(lVar30 + 0x38);
              if (uVar21 < uVar24) goto LAB_142321c08;
LAB_142321b6a:
              lVar30 = *(longlong *)(lVar30 + 0x18);
LAB_142321c2e:
              local_a8 = 0;
            }
            puVar28 = puVar28 + 1;
            lVar25 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
            local_d0 = *(longlong *)(lVar30 + 0x10 + lVar25);
            local_150 = *(longlong *)(lVar30 + 0x28 + lVar25);
            local_c0 = 0;
          }
          uVar24 = local_c0 & 0xffffffff;
        }
        puVar2 = (undefined8 *)(local_d0 + uVar24 * 0x18);
        puVar32 = (undefined8 *)(uVar24 * 0x30 + local_b0);
        uVar23 = *(int *)(local_a8 + uVar24 * 8) - 2;
        bVar20 = (byte)uVar23;
        if (4 < uVar23) {
          bVar20 = 3;
        }
        bVar20 = 0x1a >> (bVar20 & 0x1f);
        if (*(int *)((longlong)puVar2 + 0x14) == 0) {
          uVar14 = *puVar32;
          uVar15 = puVar32[1];
          uVar16 = puVar32[2];
          uVar21 = puVar32[3];
          uVar17 = puVar32[5];
          uVar18 = *puVar2;
          uVar19 = puVar2[1];
          bevy_ecs::system::commands::Commands::spawn_empty();
          plVar31 = (longlong *)CONCAT44(uStack_f4,uStack_f8);
          if ((longlong *)CONCAT44(uStack_f4,uStack_f8) == (longlong *)0x0) {
            plVar31 = local_f0;
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
          uStack_1b0 = (undefined4)uVar17;
          uStack_1ac = (undefined4)((ulonglong)uVar17 >> 0x20);
          puVar3 = (undefined8 *)(lVar25 + 8 + lVar30);
          *puVar3 = uVar14;
          puVar3[1] = uVar15;
          puVar3 = (undefined8 *)(lVar25 + 0x18 + lVar30);
          *puVar3 = uVar16;
          puVar3[1] = uVar21 & 0xffffffff;
          puVar4 = (undefined4 *)(lVar25 + 0x28 + lVar30);
          *puVar4 = 0;
          puVar4[1] = 0;
          puVar4[2] = uStack_1b0;
          puVar4[3] = uStack_1ac;
          puVar3 = (undefined8 *)(lVar25 + 0x38 + lVar30);
          *puVar3 = uVar18;
          puVar3[1] = uVar19;
          *(undefined1 *)(lVar25 + 0x48 + lVar30) = 0;
          *(ulonglong *)(lVar25 + 0x58 + lVar30) = local_e0;
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
          *(ulonglong *)(lVar25 + 0x88 + lVar30) = local_e0;
          *(undefined4 *)(lVar25 + 0x90 + lVar30) = 0;
          *(byte *)(lVar25 + 0x94 + lVar30) = bVar20 & 1;
          *(undefined1 *)(lVar25 + 0x98 + lVar30) = 0;
          plVar31[2] = lVar30 + 0xa0;
          *(undefined4 *)(local_150 + uVar24 * 4) = uVar7;
          puVar2[2] = local_e0;
          uVar24 = local_e0;
          if (local_e0 >> 0x20 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
        }
        else {
          uVar24 = puVar2[2];
        }
        local_c0 = local_c0 + 1;
        uVar21 = uVar24 & 0xffffffff;
      } while ((*(ulonglong *)(lVar12 + 0x10) <= uVar21) ||
              (*(int *)(*(longlong *)(lVar12 + 8) + uVar21 * 0x14) != (int)(uVar24 >> 0x20)));
      lVar30 = *(longlong *)(lVar12 + 8) + uVar21 * 0x14;
      uVar24 = (ulonglong)*(uint *)(lVar30 + 4);
    } while ((uVar24 == 0xffffffff) ||
            ((*(ulonglong *)(lVar11 + 0x38) <= uVar24 ||
             ((*(ulonglong *)
                (*(longlong *)(lVar11 + 0x28) + (ulonglong)(*(uint *)(lVar30 + 4) >> 6) * 8) >>
               (uVar24 & 0x3f) & 1) == 0))));
    uVar24 = (ulonglong)*(uint *)(lVar30 + 0x10);
    lVar25 = *(longlong *)
              (*(longlong *)(lVar12 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar30 + 0xc) * 0x48);
    lVar30 = *(longlong *)
              (*(longlong *)(lVar12 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar30 + 0xc) * 0x48);
    lVar26 = ~*(ulonglong *)(lVar30 + *(longlong *)(lVar11 + 0x110) * 8) * 0x30;
    lVar27 = *(longlong *)(lVar25 + 0x10 + lVar26);
    lVar26 = *(longlong *)(lVar25 + 0x28 + lVar26);
    lVar22 = ~*(ulonglong *)(lVar30 + *(longlong *)(lVar11 + 0x118) * 8) * 0x30;
    lVar30 = *(longlong *)(lVar25 + 0x10 + lVar22);
    lVar25 = *(longlong *)(lVar25 + 0x28 + lVar22);
    lVar22 = uVar24 * 0x30;
    uStack_f8 = *(undefined4 *)(puVar32 + 3);
    uVar14 = puVar32[2];
    *(undefined4 *)(lVar25 + uVar24 * 4) = uVar8;
    *(undefined4 *)(lVar30 + 0x18 + lVar22) = uStack_f8;
    *(undefined8 *)(lVar30 + 0x10 + lVar22) = uVar14;
    uVar15 = *puVar32;
    uVar16 = puVar32[1];
    *(undefined4 *)(lVar25 + uVar24 * 4) = uVar8;
    *(undefined8 *)(lVar30 + lVar22) = uVar15;
    ((undefined8 *)(lVar30 + lVar22))[1] = uVar16;
    cVar6 = *(char *)(lVar27 + 4 + uVar24 * 8);
    local_100 = (int)uVar14;
    if (((bVar20 & 1) == 0) && (cVar6 != '\0')) {
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_100 == 2) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      uStack_f8 = *(undefined4 *)(puVar32 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
LAB_1423221a3:
      if (local_f0 != (longlong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else if (((bVar20 & 1) != 0) && (cVar6 == '\0')) {
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if (local_100 == 2) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      uStack_f8 = *(undefined4 *)(puVar32 + 3);
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::impl$13::drop();
      goto LAB_1423221a3;
    }
    *(undefined4 *)(lVar26 + uVar24 * 4) = uVar8;
    fVar33 = 2.0;
    *(byte *)(lVar27 + 4 + uVar24 * 8) = bVar20 & 1;
    if ((bVar20 & 1) == 0) {
      fVar33 = -2.0;
    }
    lVar30 = lVar30 + lVar22;
    fVar34 = *(float *)(lVar13 + 0x28);
    *(undefined4 *)(lVar26 + uVar24 * 4) = uVar8;
    fVar34 = fVar33 * fVar34 + *(float *)(lVar27 + uVar24 * 8);
    fVar33 = 0.0;
    if (0.0 <= fVar34) {
      fVar33 = fVar34;
    }
    fVar34 = 1.0;
    if (fVar33 <= 1.0) {
      fVar34 = fVar33;
    }
    *(float *)(lVar27 + uVar24 * 8) = fVar34;
    ::utils::easing_functions::EasingFunction::apply();
    fVar33 = extraout_XMM0_Da * *(float *)((longlong)puVar32 + 0x1c);
    *(undefined4 *)(lVar25 + uVar24 * 4) = uVar8;
    *(float *)(lVar30 + 0x1c) = fVar33;
    *(float *)(lVar30 + 0x20) = fVar33;
    *(float *)(lVar30 + 0x24) = fVar33;
  } while( true );
}

