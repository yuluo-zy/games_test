// duck_splash_particle 0x142322650 sig=void __cdecl duck_splash_particle(void)

void __cdecl system_critters::duck_splash_particle::duck_splash_particle(void)

{
  undefined4 *puVar1;
  undefined8 *puVar2;
  uint *puVar3;
  char cVar4;
  uint uVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  longlong lVar15;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  ulonglong uVar19;
  ulonglong *puVar20;
  longlong *in_RDX;
  char *pcVar21;
  longlong lVar22;
  undefined8 *puVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  longlong *in_R9;
  longlong lVar27;
  longlong *plVar28;
  uint *puVar29;
  longlong lVar30;
  float fVar31;
  float fVar32;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong local_130;
  longlong local_120;
  longlong local_118;
  longlong local_e0;
  undefined4 local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  longlong *local_88;
  longlong *local_80;
  undefined8 local_70;
  
  puVar20 = (ulonglong *)*in_stack_00000038;
  lVar7 = in_stack_00000038[1];
  uVar26 = *puVar20;
  uVar19 = 0;
  uVar25 = uVar26 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar26 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar25 = uVar19;
  }
  uVar24 = uVar26 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar26 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar24 = uVar19;
  }
  lVar18 = uVar25 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar26 = *(ulonglong *)(lVar7 + 0x10) - uVar25;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar25) {
    lVar18 = 8;
    uVar26 = uVar19;
  }
  lVar15 = uVar24 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar25 = *(ulonglong *)(lVar7 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar24) {
    lVar15 = 8;
    uVar25 = uVar19;
  }
  *puVar20 = (*(longlong *)(lVar7 + 0x40) - uVar25) - uVar26;
  lVar30 = uVar26 * 0x10 + lVar18;
  local_120 = uVar25 * 0x10 + lVar15;
  local_118 = *in_RDX;
  lVar7 = *in_stack_00000030;
  lVar8 = in_stack_00000030[1];
  do {
    do {
      do {
        if ((lVar18 == 0) || (lVar18 == lVar30)) {
          if ((lVar15 == 0) || (lVar15 == local_120)) {
            lVar18 = *in_R9;
            lVar15 = in_R9[1];
            uVar6 = *(undefined4 *)((longlong)in_R9 + 0x14);
            puVar29 = *(uint **)(lVar18 + 0xf8);
            puVar3 = puVar29 + *(longlong *)(lVar18 + 0x100);
            cVar4 = *(char *)(lVar18 + 0x108);
            lVar30 = *in_stack_00000028;
            lVar16 = 0;
            local_130 = 0;
            local_e0 = 8;
            uVar26 = 0;
            uVar25 = 0;
            if (cVar4 != '\0') goto LAB_142322b40;
            do {
              if (uVar25 == uVar26) {
                if (puVar29 == puVar3) {
                  return;
                }
                while( true ) {
                  uVar26 = *(ulonglong *)
                            (*(longlong *)(lVar15 + 0x100) + 0x58 + (ulonglong)*puVar29 * 0xa0);
                  if (uVar26 != 0) break;
                  puVar29 = puVar29 + 1;
                  if (puVar29 == puVar3) {
                    return;
                  }
                }
                lVar17 = *(longlong *)(lVar15 + 0x100) + (ulonglong)*puVar29 * 0xa0;
                uVar25 = (ulonglong)*(uint *)(lVar17 + 0x94);
                lVar27 = *(longlong *)(*(longlong *)(lVar15 + 0x1a8) + 0x18 + uVar25 * 0x48);
                lVar16 = *(longlong *)(*(longlong *)(lVar15 + 0x1a8) + 0x38 + uVar25 * 0x48);
                lVar22 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar18 + 0x110) * 8) * 0x30;
                local_130 = *(longlong *)(lVar27 + 0x10 + lVar22);
                local_118 = *(longlong *)(lVar27 + 0x28 + lVar22);
                lVar22 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar18 + 0x118) * 8) * 0x30;
                lVar16 = *(longlong *)(lVar27 + 0x10 + lVar22);
                local_120 = *(longlong *)(lVar27 + 0x28 + lVar22);
                local_e0 = *(longlong *)(lVar17 + 0x50);
                puVar29 = puVar29 + 1;
                uVar25 = 0;
              }
              uVar19 = (ulonglong)*(uint *)(local_e0 + 8 + uVar25 * 0x10);
              while( true ) {
                puVar1 = (undefined4 *)(local_118 + uVar19 * 4);
                puVar20 = (ulonglong *)(uVar19 * 0x10 + local_130);
                puVar23 = (undefined8 *)(uVar19 * 0x30 + lVar16);
                uVar24 = *puVar20 & 0xffffffff;
                if ((uVar24 < *(ulonglong *)(lVar8 + 0x10)) &&
                   (*(int *)(*(longlong *)(lVar8 + 8) + uVar24 * 0x14) == (int)(*puVar20 >> 0x20)))
                {
                  lVar27 = *(longlong *)(lVar8 + 8) + uVar24 * 0x14;
                  uVar24 = (ulonglong)*(uint *)(lVar27 + 4);
                  if ((uVar24 != 0xffffffff) &&
                     ((uVar24 < *(ulonglong *)(lVar7 + 0x38) &&
                      ((*(ulonglong *)
                         (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar27 + 4) >> 6) * 8)
                        >> (uVar24 & 0x3f) & 1) != 0)))) {
                    lVar17 = *(longlong *)
                              (*(longlong *)
                                (*(longlong *)(lVar8 + 0x1a8) + 0x18 +
                                (ulonglong)*(uint *)(lVar27 + 0xc) * 0x48) + 0x10 +
                              ~*(ulonglong *)
                                (*(longlong *)
                                  (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
                                  (ulonglong)*(uint *)(lVar27 + 0xc) * 0x48) +
                                *(longlong *)(lVar7 + 0x118) * 8) * 0x30);
                    lVar27 = (ulonglong)*(uint *)(lVar27 + 0x10) * 0x30;
                    uVar9 = *(undefined8 *)(lVar17 + lVar27);
                    uVar10 = ((undefined8 *)(lVar17 + lVar27))[1];
                    puVar2 = (undefined8 *)(lVar17 + 0x10 + lVar27);
                    uVar11 = *puVar2;
                    uVar12 = puVar2[1];
                    puVar2 = (undefined8 *)(lVar17 + 0x20 + lVar27);
                    uVar13 = puVar2[1];
                    puVar23[4] = *puVar2;
                    puVar23[5] = uVar13;
                    puVar23[2] = uVar11;
                    puVar23[3] = uVar12;
                    *puVar23 = uVar9;
                    puVar23[1] = uVar10;
                    *(undefined4 *)(local_120 + uVar19 * 4) = uVar6;
                    *(undefined4 *)((longlong)puVar23 + 0x14) = 0;
                  }
                }
                fVar31 = *(float *)(lVar30 + 0x28);
                *puVar1 = uVar6;
                fVar31 = fVar31 / 2.5 + *(float *)(puVar20 + 1);
                uVar25 = uVar25 + 1;
                *(float *)(puVar20 + 1) = fVar31;
                if (fVar31 <= 1.0) {
                  fVar32 = 0.0;
                  if (0.0 <= fVar31) {
                    fVar32 = fVar31;
                  }
                  fVar31 = 1.0;
                  if (fVar32 <= 1.0) {
                    fVar31 = fVar32;
                  }
                  *puVar1 = uVar6;
                  *(float *)(puVar20 + 1) = fVar31;
                }
                else {
                  bevy_ecs::system::commands::EntityCommands::despawn();
                }
                if (cVar4 == '\0') break;
LAB_142322b40:
                if (uVar25 == uVar26) {
                  if (puVar29 == puVar3) {
                    return;
                  }
                  while( true ) {
                    uVar26 = *(ulonglong *)
                              (*(longlong *)(lVar15 + 0x1a8) + 0x10 + (ulonglong)*puVar29 * 0x48);
                    if (uVar26 != 0) break;
                    puVar29 = puVar29 + 1;
                    if (puVar29 == puVar3) {
                      return;
                    }
                  }
                  lVar16 = *(longlong *)(lVar15 + 0x1a8) + (ulonglong)*puVar29 * 0x48;
                  lVar27 = *(longlong *)(lVar16 + 0x38);
                  lVar17 = *(longlong *)(lVar16 + 0x18);
                  lVar16 = ~*(ulonglong *)(lVar27 + *(longlong *)(lVar18 + 0x110) * 8) * 0x30;
                  local_130 = *(longlong *)(lVar17 + 0x10 + lVar16);
                  local_118 = *(longlong *)(lVar17 + 0x28 + lVar16);
                  lVar27 = ~*(ulonglong *)(lVar27 + *(longlong *)(lVar18 + 0x118) * 8) * 0x30;
                  lVar16 = *(longlong *)(lVar17 + 0x10 + lVar27);
                  local_120 = *(longlong *)(lVar17 + 0x28 + lVar27);
                  puVar29 = puVar29 + 1;
                  uVar25 = 0;
                }
                uVar19 = uVar25 & 0xffffffff;
              }
            } while( true );
          }
          lVar18 = 0;
          lVar16 = lVar15 + 0x10;
          lVar27 = lVar15;
        }
        else {
          lVar16 = lVar15;
          lVar27 = lVar18;
          lVar18 = lVar18 + 0x10;
        }
        lVar15 = lVar16;
        *puVar20 = *puVar20 + 1;
        uVar26 = *(ulonglong *)(lVar27 + 8) & 0xffffffff;
      } while ((*(ulonglong *)(lVar8 + 0x10) <= uVar26) ||
              (*(int *)(*(longlong *)(lVar8 + 8) + uVar26 * 0x14) !=
               (int)(*(ulonglong *)(lVar27 + 8) >> 0x20)));
      lVar16 = *(longlong *)(lVar8 + 8) + uVar26 * 0x14;
      uVar26 = (ulonglong)*(uint *)(lVar16 + 4);
    } while ((uVar26 == 0xffffffff) ||
            ((*(ulonglong *)(lVar7 + 0x38) <= uVar26 ||
             ((*(ulonglong *)
                (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar16 + 4) >> 6) * 8) >>
               (uVar26 & 0x3f) & 1) == 0))));
    if (*(ulonglong *)(lVar7 + 0x118) <
        *(ulonglong *)
         (*(longlong *)(lVar8 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48)) {
      lVar17 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48;
      uVar26 = *(ulonglong *)(*(longlong *)(lVar17 + 0x38) + *(ulonglong *)(lVar7 + 0x118) * 8);
      if (uVar26 == 0) goto LAB_14232281f;
      lVar17 = *(longlong *)(*(longlong *)(lVar17 + 0x18) + 0x10 + ~uVar26 * 0x30);
    }
    else {
LAB_14232281f:
      lVar17 = 0;
    }
    uVar5 = *(uint *)(lVar16 + 0x10);
    country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name();
    pcVar21 = "splash_particle";
    country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_00 & 1) == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar16 = (ulonglong)uVar5 * 0x30;
    uVar9 = *(undefined8 *)(lVar17 + lVar16);
    uVar10 = ((undefined8 *)(lVar17 + lVar16))[1];
    puVar23 = (undefined8 *)(lVar17 + 0x10 + lVar16);
    uVar11 = *puVar23;
    uVar12 = puVar23[1];
    puVar23 = (undefined8 *)(lVar17 + 0x20 + lVar16);
    uVar13 = *puVar23;
    uVar14 = puVar23[1];
    uStack_a4 = (undefined4)uVar13;
    uStack_a0 = (undefined4)((ulonglong)uVar13 >> 0x20);
    uStack_9c = (undefined4)uVar14;
    uStack_98 = (undefined4)((ulonglong)uVar14 >> 0x20);
    uStack_b4 = (undefined4)uVar11;
    uStack_b0 = (undefined4)((ulonglong)uVar11 >> 0x20);
    uStack_ac = (undefined4)uVar12;
    local_a8 = (undefined4)((ulonglong)uVar12 >> 0x20);
    uStack_c4 = (undefined4)uVar9;
    uStack_c0 = (undefined4)((ulonglong)uVar9 >> 0x20);
    uStack_bc = (undefined4)uVar10;
    local_b8 = (undefined4)((ulonglong)uVar10 >> 0x20);
    uVar9 = *(undefined8 *)(lVar27 + 8);
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar28 = local_88;
    if (local_88 == (longlong *)0x0) {
      plVar28 = local_80;
    }
    lVar16 = plVar28[2];
    if ((ulonglong)(*plVar28 - lVar16) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar27 = plVar28[1];
    *(code **)(lVar27 + lVar16) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::SplashParticle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar27 + 8 + lVar16) = uVar9;
    *(undefined4 *)(lVar27 + 0x10 + lVar16) = 0;
    puVar23 = (undefined8 *)(lVar27 + 0x14 + lVar16);
    *puVar23 = CONCAT44(uStack_c4,local_c8);
    puVar23[1] = CONCAT44(uStack_bc,uStack_c0);
    puVar1 = (undefined4 *)(lVar27 + 0x24 + lVar16);
    *puVar1 = local_b8;
    puVar1[1] = uStack_b4;
    puVar1[2] = uStack_b0;
    puVar1[3] = uStack_ac;
    puVar23 = (undefined8 *)(lVar27 + 0x34 + lVar16);
    *puVar23 = CONCAT44(uStack_a4,local_a8);
    puVar23[1] = CONCAT44(uStack_9c,uStack_a0);
    *(undefined4 *)(lVar27 + 0x44 + lVar16) = uStack_98;
    *(undefined8 *)(lVar27 + 0x48 + lVar16) = extraout_RAX;
    *(char **)(lVar27 + 0x50 + lVar16) = pcVar21;
    *(undefined1 *)(lVar27 + 0x58 + lVar16) = 0;
    *(undefined8 *)(lVar27 + 0x68 + lVar16) = local_70;
    *(code **)(lVar27 + 0x78 + lVar16) = bevy_ecs::error::handler::panic;
    plVar28[2] = lVar16 + 0x88;
  } while( true );
}

