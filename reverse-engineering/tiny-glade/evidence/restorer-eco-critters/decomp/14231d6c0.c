// 0x14231d6c0 update_roof_bird_spots
// SIG: void __cdecl update_roof_bird_spots(void)

void __cdecl system_critters::wren::update_roof_bird_spots(void)

{
  undefined4 uVar1;
  longlong lVar2;
  ulonglong *puVar3;
  longlong lVar4;
  undefined4 *puVar5;
  undefined8 uVar6;
  char extraout_AL;
  char extraout_AL_00;
  float extraout_EAX;
  undefined4 extraout_EAX_00;
  ulonglong uVar7;
  ulonglong extraout_RAX;
  longlong lVar8;
  float extraout_var;
  longlong lVar9;
  bool bVar10;
  ulonglong uVar11;
  undefined8 *in_RDX;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  undefined8 *in_R8;
  longlong lVar16;
  longlong *in_R9;
  longlong lVar17;
  ulonglong uVar18;
  ulonglong *puVar19;
  longlong lVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong local_248;
  longlong local_240;
  longlong local_238;
  longlong local_1c0;
  undefined8 local_188;
  undefined4 uStack_180;
  longlong local_158;
  undefined8 local_138;
  longlong local_108;
  longlong local_100;
  
  puVar19 = (ulonglong *)*in_RDX;
  lVar2 = in_RDX[1];
  uVar11 = *puVar19;
  uVar18 = 0;
  uVar7 = uVar11 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar11 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar7 = uVar18;
  }
  uVar12 = uVar11 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar11 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar12 = uVar18;
  }
  local_100 = uVar7 * 0x20 + *(longlong *)(lVar2 + 8);
  uVar11 = *(ulonglong *)(lVar2 + 0x10) - uVar7;
  if (*(ulonglong *)(lVar2 + 0x10) < uVar7) {
    local_100 = 8;
    uVar11 = uVar18;
  }
  local_108 = uVar12 * 0x20 + *(longlong *)(lVar2 + 0x28);
  uVar7 = *(ulonglong *)(lVar2 + 0x30) - uVar12;
  if (*(ulonglong *)(lVar2 + 0x30) < uVar12) {
    local_108 = 8;
    uVar7 = uVar18;
  }
  *puVar19 = (*(longlong *)(lVar2 + 0x40) - uVar7) - uVar11;
  lVar13 = uVar11 * 0x20 + local_100;
  lVar14 = uVar7 * 0x20 + local_108;
  puVar3 = (ulonglong *)*in_R8;
  lVar2 = in_R8[1];
  uVar11 = *puVar3;
  uVar7 = uVar11 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar11 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar7 = uVar18;
  }
  uVar12 = uVar11 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar11 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar12 = uVar18;
  }
  lVar20 = uVar7 * 0x10 + *(longlong *)(lVar2 + 8);
  uVar11 = *(ulonglong *)(lVar2 + 0x10) - uVar7;
  if (*(ulonglong *)(lVar2 + 0x10) < uVar7) {
    lVar20 = 8;
    uVar11 = uVar18;
  }
  lVar15 = uVar12 * 0x10 + *(longlong *)(lVar2 + 0x28);
  uVar7 = *(ulonglong *)(lVar2 + 0x30) - uVar12;
  if (*(ulonglong *)(lVar2 + 0x30) < uVar12) {
    lVar15 = 8;
    uVar7 = uVar18;
  }
  *puVar3 = (*(longlong *)(lVar2 + 0x40) - uVar7) - uVar11;
  lVar17 = uVar11 * 0x10 + lVar20;
  lVar16 = uVar7 * 0x10 + lVar15;
  lVar2 = *in_R9;
  lVar4 = in_R9[1];
  uVar1 = *(undefined4 *)(in_stack_00000028 + 0x1c);
  puVar5 = *(undefined4 **)(in_stack_00000028 + 0x10);
  uVar6 = *(undefined8 *)(in_stack_00000030 + 8);
LAB_14231d8f6:
  do {
    if (puVar19 == (ulonglong *)0x0) goto LAB_14231d9d2;
    if ((local_100 == 0) || (local_100 == lVar13)) {
      if ((local_108 == 0) || (local_108 == lVar14)) {
        local_100 = 0;
        goto LAB_14231d9d2;
      }
      local_100 = 0;
      lVar8 = local_108;
      local_108 = local_108 + 0x20;
    }
    else {
      lVar8 = local_100;
      local_100 = local_100 + 0x20;
    }
    *puVar19 = *puVar19 + 1;
    uVar11 = *(ulonglong *)(lVar8 + 8);
    if (uVar11 >> 0x20 == 0) goto LAB_14231d9d2;
    uVar7 = uVar11 & 0xffffffff;
  } while (*(ulonglong *)(lVar4 + 0x10) <= uVar7);
  goto LAB_14231da28;
LAB_14231d9d2:
  do {
    if ((lVar20 == 0) || (lVar20 == lVar17)) {
      if ((lVar15 == 0) || (lVar15 == lVar16)) {
        *puVar5 = uVar1;
        country_core::resources::critters::BirdSpotManager::
        recycle<system_critters::wren::update_roof_bird_spots::closure_env_3>();
        return;
      }
      lVar15 = lVar15 + 0x10;
      lVar20 = 0;
    }
    else {
      lVar20 = lVar20 + 0x10;
    }
    *puVar3 = *puVar3 + 1;
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  } while (extraout_RAX >> 0x20 == 0);
  puVar19 = (ulonglong *)0x0;
  uVar7 = extraout_RAX & 0xffffffff;
  uVar11 = extraout_RAX;
  if (*(ulonglong *)(lVar4 + 0x10) <= uVar7) goto LAB_14231d8f6;
LAB_14231da28:
  if (*(int *)(*(longlong *)(lVar4 + 8) + uVar7 * 0x14) != (int)(uVar11 >> 0x20))
  goto LAB_14231d8f6;
  lVar8 = *(longlong *)(lVar4 + 8) + uVar7 * 0x14;
  uVar11 = (ulonglong)*(uint *)(lVar8 + 4);
  if (((uVar11 == 0xffffffff) || (*(ulonglong *)(lVar2 + 0x38) <= uVar11)) ||
     ((*(ulonglong *)(*(longlong *)(lVar2 + 0x28) + (ulonglong)(*(uint *)(lVar8 + 4) >> 6) * 8) >>
       (uVar11 & 0x3f) & 1) == 0)) goto LAB_14231d8f6;
  if (*(ulonglong *)(lVar2 + 0x110) <
      *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar8 + 0xc) * 0x48)
     ) {
    lVar9 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)*(uint *)(lVar8 + 0xc) * 0x48;
    uVar11 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + *(ulonglong *)(lVar2 + 0x110) * 8);
    if (uVar11 != 0) {
      lVar9 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar11 * 0x30);
      goto LAB_14231daf6;
    }
  }
  lVar9 = 0;
LAB_14231daf6:
  lVar8 = (ulonglong)*(uint *)(lVar8 + 0x10) * 0x58;
  if (*(int *)(lVar9 + 8 + lVar8) == 1) {
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (extraout_AL == '\0') {
      country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls();
      ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
      ::utils::geometry::rectangle::Rectangle2d::new_axis_aligned();
      country_core::resources::roofs::roof_shape::Roof::is_gable();
      if ((extraout_AL_00 == '\0') || (*(char *)(lVar9 + lVar8 + 0x54) != '\0')) {
        country_core::resources::walls::decorator::ClutterVersion::is_initial();
        ::utils::geometry::rectangle::Rectangle2d::get_edge_pts();
        fVar21 = extraout_EAX;
        fVar23 = extraout_var;
      }
      else {
        country_core::resources::walls::decorator::ClutterVersion::is_initial();
        ::utils::geometry::rectangle::Rectangle2d::get_edge_pts();
        fVar22 = (float)local_138;
        fVar24 = (float)((ulonglong)local_138 >> 0x20);
        fVar26 = 0.1 / SQRT((extraout_var - fVar24) * (extraout_var - fVar24) +
                            (extraout_EAX - fVar22) * (extraout_EAX - fVar22));
        fVar25 = 1.0 - fVar26;
        fVar21 = fVar22 * fVar26 + extraout_EAX * fVar25;
        fVar23 = fVar24 * fVar26 + extraout_var * fVar25;
        local_138 = CONCAT44(extraout_var * fVar26 + fVar24 * fVar25,
                             extraout_EAX * fVar26 + fVar22 * fVar25);
      }
      fVar21 = fVar21 - (float)local_138;
      fVar23 = fVar23 - (float)((ulonglong)local_138 >> 0x20);
      if (0.5 < SQRT(fVar23 * fVar23 + fVar21 * fVar21)) {
        local_158 = 0;
        ::utils::rng::init_rng();
        ::utils::random_splits();
        lVar8 = local_240 + local_238 * 4;
        bVar10 = true;
        lVar9 = local_240;
        while (lVar9 != lVar8) {
          local_1c0 = lVar9 + 4;
          if (bVar10) {
            if (local_1c0 == lVar8) break;
            local_1c0 = lVar9 + 8;
          }
          fastrand::Rng::f32();
          country_core::resources::roofs::roof_shape::Roof::height();
          ::utils::swizzlers::impl$1::xvy();
          country_core::resources::roofs::roof_noise::apply_roof_noise();
          *puVar5 = uVar1;
          country_core::resources::critters::BirdSpotManager::gen_id();
          if (local_158 == 0) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          *(undefined4 *)(local_158 * 0x18 + 0xc) = 0;
          *(undefined8 *)(local_158 * 0x18 + 0x10) = local_188;
          *(undefined4 *)(local_158 * 0x18 + 0x18) = uStack_180;
          *(undefined4 *)(local_158 * 0x18 + 0x1c) = extraout_EAX_00;
          local_158 = local_158 + 1;
          bVar10 = false;
          lVar9 = local_1c0;
        }
        country_core::resources::critters::BirdSpotManager::create_spots();
        bevy_ecs::system::commands::EntityCommands::
        queue<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::critters::BirdSpots>,tuple_<>,enum2_<core::result::Result<tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>_>
                  ();
        local_138 = uVar6;
        if (local_248 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        goto LAB_14231d8f6;
      }
    }
  }
  local_188 = 0;
  uStack_180 = 8;
  country_core::resources::critters::BirdSpotManager::create_spots();
  bevy_ecs::system::commands::EntityCommands::
  queue<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::critters::BirdSpots>,tuple_<>,enum2_<core::result::Result<tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>_>
            ();
  local_138 = uVar6;
  goto LAB_14231d8f6;
}

