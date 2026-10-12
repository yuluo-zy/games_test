
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
  float fVar15;
  float fVar16;
  longlong lVar17;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong lVar18;
  longlong lVar19;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong *puVar22;
  longlong *in_RDX;
  char *pcVar23;
  longlong lVar24;
  undefined8 *puVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  longlong *in_R9;
  longlong lVar29;
  longlong *plVar30;
  uint *puVar31;
  longlong lVar32;
  float fVar33;
  float fVar34;
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
  
  puVar22 = (ulonglong *)*in_stack_00000038;
  lVar7 = in_stack_00000038[1];
  uVar28 = *puVar22;
  uVar21 = 0;
  uVar27 = uVar28 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar28 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar27 = uVar21;
  }
  uVar26 = uVar28 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar28 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar26 = uVar21;
  }
  lVar20 = uVar27 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar28 = *(ulonglong *)(lVar7 + 0x10) - uVar27;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar27) {
    lVar20 = 8;
    uVar28 = uVar21;
  }
  lVar17 = uVar26 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar27 = *(ulonglong *)(lVar7 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar26) {
    lVar17 = 8;
    uVar27 = uVar21;
  }
  *puVar22 = (*(longlong *)(lVar7 + 0x40) - uVar27) - uVar28;
  lVar32 = uVar28 * 0x10 + lVar20;
  local_120 = uVar27 * 0x10 + lVar17;
  local_118 = *in_RDX;
  lVar7 = *in_stack_00000030;
  lVar8 = in_stack_00000030[1];
  do {
    do {
      do {
        fVar16 = __real_40200000;
        fVar15 = __real_3f800000;
        if ((lVar20 == 0) || (lVar20 == lVar32)) {
          if ((lVar17 == 0) || (lVar17 == local_120)) {
            lVar20 = *in_R9;
            lVar17 = in_R9[1];
            uVar6 = *(undefined4 *)((longlong)in_R9 + 0x14);
            puVar31 = *(uint **)(lVar20 + 0xf8);
            puVar3 = puVar31 + *(longlong *)(lVar20 + 0x100);
            cVar4 = *(char *)(lVar20 + 0x108);
            lVar32 = *in_stack_00000028;
            lVar18 = 0;
            local_130 = 0;
            local_e0 = 8;
            uVar28 = 0;
            uVar27 = 0;
            if (cVar4 != '\0') goto LAB_142322b40;
            do {
              if (uVar27 == uVar28) {
                if (puVar31 == puVar3) {
                  return;
                }
                while( true ) {
                  uVar28 = *(ulonglong *)
                            (*(longlong *)(lVar17 + 0x100) + 0x58 + (ulonglong)*puVar31 * 0xa0);
                  if (uVar28 != 0) break;
                  puVar31 = puVar31 + 1;
                  if (puVar31 == puVar3) {
                    return;
                  }
                }
                lVar19 = *(longlong *)(lVar17 + 0x100) + (ulonglong)*puVar31 * 0xa0;
                uVar27 = (ulonglong)*(uint *)(lVar19 + 0x94);
                lVar29 = *(longlong *)(*(longlong *)(lVar17 + 0x1a8) + 0x18 + uVar27 * 0x48);
                lVar18 = *(longlong *)(*(longlong *)(lVar17 + 0x1a8) + 0x38 + uVar27 * 0x48);
                lVar24 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar20 + 0x110) * 8) * 0x30;
                local_130 = *(longlong *)(lVar29 + 0x10 + lVar24);
                local_118 = *(longlong *)(lVar29 + 0x28 + lVar24);
                lVar24 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar20 + 0x118) * 8) * 0x30;
                lVar18 = *(longlong *)(lVar29 + 0x10 + lVar24);
                local_120 = *(longlong *)(lVar29 + 0x28 + lVar24);
                local_e0 = *(longlong *)(lVar19 + 0x50);
                puVar31 = puVar31 + 1;
                uVar27 = 0;
              }
              uVar21 = (ulonglong)*(uint *)(local_e0 + 8 + uVar27 * 0x10);
              while( true ) {
                puVar1 = (undefined4 *)(local_118 + uVar21 * 4);
                puVar22 = (ulonglong *)(uVar21 * 0x10 + local_130);
                puVar25 = (undefined8 *)(uVar21 * 0x30 + lVar18);
                uVar26 = *puVar22 & 0xffffffff;
                if ((uVar26 < *(ulonglong *)(lVar8 + 0x10)) &&
                   (*(int *)(*(longlong *)(lVar8 + 8) + uVar26 * 0x14) == (int)(*puVar22 >> 0x20)))
                {
                  lVar29 = *(longlong *)(lVar8 + 8) + uVar26 * 0x14;
                  uVar26 = (ulonglong)*(uint *)(lVar29 + 4);
                  if ((uVar26 != 0xffffffff) &&
                     ((uVar26 < *(ulonglong *)(lVar7 + 0x38) &&
                      ((*(ulonglong *)
                         (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar29 + 4) >> 6) * 8)
                        >> (uVar26 & 0x3f) & 1) != 0)))) {
                    lVar19 = *(longlong *)
                              (*(longlong *)
                                (*(longlong *)(lVar8 + 0x1a8) + 0x18 +
                                (ulonglong)*(uint *)(lVar29 + 0xc) * 0x48) + 0x10 +
                              ~*(ulonglong *)
                                (*(longlong *)
                                  (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
                                  (ulonglong)*(uint *)(lVar29 + 0xc) * 0x48) +
                                *(longlong *)(lVar7 + 0x118) * 8) * 0x30);
                    lVar29 = (ulonglong)*(uint *)(lVar29 + 0x10) * 0x30;
                    uVar9 = *(undefined8 *)(lVar19 + lVar29);
                    uVar10 = ((undefined8 *)(lVar19 + lVar29))[1];
                    puVar2 = (undefined8 *)(lVar19 + 0x10 + lVar29);
                    uVar11 = *puVar2;
                    uVar12 = puVar2[1];
                    puVar2 = (undefined8 *)(lVar19 + 0x20 + lVar29);
                    uVar13 = puVar2[1];
                    puVar25[4] = *puVar2;
                    puVar25[5] = uVar13;
                    puVar25[2] = uVar11;
                    puVar25[3] = uVar12;
                    *puVar25 = uVar9;
                    puVar25[1] = uVar10;
                    *(undefined4 *)(local_120 + uVar21 * 4) = uVar6;
                    *(undefined4 *)((longlong)puVar25 + 0x14) = 0;
                  }
                }
                fVar33 = *(float *)(lVar32 + 0x28);
                *puVar1 = uVar6;
                fVar33 = fVar33 / fVar16 + *(float *)(puVar22 + 1);
                uVar27 = uVar27 + 1;
                *(float *)(puVar22 + 1) = fVar33;
                if (fVar33 <= fVar15) {
                  fVar34 = 0.0;
                  if (0.0 <= fVar33) {
                    fVar34 = fVar33;
                  }
                  fVar33 = fVar15;
                  if (fVar34 <= fVar15) {
                    fVar33 = fVar34;
                  }
                  *puVar1 = uVar6;
                  *(float *)(puVar22 + 1) = fVar33;
                }
                else {
                  bevy_ecs::system::commands::EntityCommands::despawn();
                }
                if (cVar4 == '\0') break;
LAB_142322b40:
                if (uVar27 == uVar28) {
                  if (puVar31 == puVar3) {
                    return;
                  }
                  while( true ) {
                    uVar28 = *(ulonglong *)
                              (*(longlong *)(lVar17 + 0x1a8) + 0x10 + (ulonglong)*puVar31 * 0x48);
                    if (uVar28 != 0) break;
                    puVar31 = puVar31 + 1;
                    if (puVar31 == puVar3) {
                      return;
                    }
                  }
                  lVar18 = *(longlong *)(lVar17 + 0x1a8) + (ulonglong)*puVar31 * 0x48;
                  lVar29 = *(longlong *)(lVar18 + 0x38);
                  lVar19 = *(longlong *)(lVar18 + 0x18);
                  lVar18 = ~*(ulonglong *)(lVar29 + *(longlong *)(lVar20 + 0x110) * 8) * 0x30;
                  local_130 = *(longlong *)(lVar19 + 0x10 + lVar18);
                  local_118 = *(longlong *)(lVar19 + 0x28 + lVar18);
                  lVar29 = ~*(ulonglong *)(lVar29 + *(longlong *)(lVar20 + 0x118) * 8) * 0x30;
                  lVar18 = *(longlong *)(lVar19 + 0x10 + lVar29);
                  local_120 = *(longlong *)(lVar19 + 0x28 + lVar29);
                  puVar31 = puVar31 + 1;
                  uVar27 = 0;
                }
                uVar21 = uVar27 & 0xffffffff;
              }
            } while( true );
          }
          lVar20 = 0;
          lVar18 = lVar17 + 0x10;
          lVar29 = lVar17;
        }
        else {
          lVar18 = lVar17;
          lVar29 = lVar20;
          lVar20 = lVar20 + 0x10;
        }
        lVar17 = lVar18;
        *puVar22 = *puVar22 + 1;
        uVar28 = *(ulonglong *)(lVar29 + 8) & 0xffffffff;
      } while ((*(ulonglong *)(lVar8 + 0x10) <= uVar28) ||
              (*(int *)(*(longlong *)(lVar8 + 8) + uVar28 * 0x14) !=
               (int)(*(ulonglong *)(lVar29 + 8) >> 0x20)));
      lVar18 = *(longlong *)(lVar8 + 8) + uVar28 * 0x14;
      uVar28 = (ulonglong)*(uint *)(lVar18 + 4);
    } while ((uVar28 == 0xffffffff) ||
            ((*(ulonglong *)(lVar7 + 0x38) <= uVar28 ||
             ((*(ulonglong *)
                (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar18 + 4) >> 6) * 8) >>
               (uVar28 & 0x3f) & 1) == 0))));
    if (*(ulonglong *)(lVar7 + 0x118) <
        *(ulonglong *)
         (*(longlong *)(lVar8 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar18 + 0xc) * 0x48)) {
      lVar19 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*(uint *)(lVar18 + 0xc) * 0x48;
      uVar28 = *(ulonglong *)(*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lVar7 + 0x118) * 8);
      if (uVar28 == 0) goto LAB_14232281f;
      lVar19 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar28 * 0x30);
    }
    else {
LAB_14232281f:
      lVar19 = 0;
    }
    uVar5 = *(uint *)(lVar18 + 0x10);
    country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name();
    pcVar23 = "splash_particle";
    country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_00 & 1) == 0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    lVar18 = (ulonglong)uVar5 * 0x30;
    uVar9 = *(undefined8 *)(lVar19 + lVar18);
    uVar10 = ((undefined8 *)(lVar19 + lVar18))[1];
    puVar25 = (undefined8 *)(lVar19 + 0x10 + lVar18);
    uVar11 = *puVar25;
    uVar12 = puVar25[1];
    puVar25 = (undefined8 *)(lVar19 + 0x20 + lVar18);
    uVar13 = *puVar25;
    uVar14 = puVar25[1];
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
    uVar9 = *(undefined8 *)(lVar29 + 8);
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar30 = local_88;
    if (local_88 == (longlong *)0x0) {
      plVar30 = local_80;
    }
    lVar18 = plVar30[2];
    if ((ulonglong)(*plVar30 - lVar18) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar29 = plVar30[1];
    *(code **)(lVar29 + lVar18) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::SplashParticle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar29 + 8 + lVar18) = uVar9;
    *(undefined4 *)(lVar29 + 0x10 + lVar18) = 0;
    puVar25 = (undefined8 *)(lVar29 + 0x14 + lVar18);
    *puVar25 = CONCAT44(uStack_c4,local_c8);
    puVar25[1] = CONCAT44(uStack_bc,uStack_c0);
    puVar1 = (undefined4 *)(lVar29 + 0x24 + lVar18);
    *puVar1 = local_b8;
    puVar1[1] = uStack_b4;
    puVar1[2] = uStack_b0;
    puVar1[3] = uStack_ac;
    puVar25 = (undefined8 *)(lVar29 + 0x34 + lVar18);
    *puVar25 = CONCAT44(uStack_a4,local_a8);
    puVar25[1] = CONCAT44(uStack_9c,uStack_a0);
    *(undefined4 *)(lVar29 + 0x44 + lVar18) = uStack_98;
    *(undefined8 *)(lVar29 + 0x48 + lVar18) = extraout_RAX;
    *(char **)(lVar29 + 0x50 + lVar18) = pcVar23;
    *(undefined1 *)(lVar29 + 0x58 + lVar18) = 0;
    *(undefined8 *)(lVar29 + 0x68 + lVar18) = local_70;
    *(code **)(lVar29 + 0x78 + lVar18) = bevy_ecs::error::handler::panic;
    plVar30[2] = lVar18 + 0x88;
  } while( true );
}

