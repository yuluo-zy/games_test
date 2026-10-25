
void __cdecl system_critters::wren::update_roof_bird_spots(void)

{
  undefined4 uVar1;
  longlong lVar2;
  ulonglong *puVar3;
  longlong lVar4;
  undefined4 *puVar5;
  undefined8 uVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  char extraout_AL;
  char extraout_AL_00;
  float extraout_EAX;
  undefined4 extraout_EAX_00;
  ulonglong uVar10;
  ulonglong extraout_RAX;
  longlong lVar11;
  float extraout_var;
  longlong lVar12;
  bool bVar13;
  ulonglong uVar14;
  undefined8 *in_RDX;
  ulonglong uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  undefined8 *in_R8;
  longlong lVar19;
  longlong *in_R9;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong *puVar22;
  longlong lVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
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
  
  puVar22 = (ulonglong *)*in_RDX;
  lVar2 = in_RDX[1];
  uVar14 = *puVar22;
  uVar21 = 0;
  uVar10 = uVar14 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar14 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar10 = uVar21;
  }
  uVar15 = uVar14 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar14 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar15 = uVar21;
  }
  local_100 = uVar10 * 0x20 + *(longlong *)(lVar2 + 8);
  uVar14 = *(ulonglong *)(lVar2 + 0x10) - uVar10;
  if (*(ulonglong *)(lVar2 + 0x10) < uVar10) {
    local_100 = 8;
    uVar14 = uVar21;
  }
  local_108 = uVar15 * 0x20 + *(longlong *)(lVar2 + 0x28);
  uVar10 = *(ulonglong *)(lVar2 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar2 + 0x30) < uVar15) {
    local_108 = 8;
    uVar10 = uVar21;
  }
  *puVar22 = (*(longlong *)(lVar2 + 0x40) - uVar10) - uVar14;
  lVar16 = uVar14 * 0x20 + local_100;
  lVar17 = uVar10 * 0x20 + local_108;
  puVar3 = (ulonglong *)*in_R8;
  lVar2 = in_R8[1];
  uVar14 = *puVar3;
  uVar10 = uVar14 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar14 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar10 = uVar21;
  }
  uVar15 = uVar14 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar14 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar15 = uVar21;
  }
  lVar23 = uVar10 * 0x10 + *(longlong *)(lVar2 + 8);
  uVar14 = *(ulonglong *)(lVar2 + 0x10) - uVar10;
  if (*(ulonglong *)(lVar2 + 0x10) < uVar10) {
    lVar23 = 8;
    uVar14 = uVar21;
  }
  lVar18 = uVar15 * 0x10 + *(longlong *)(lVar2 + 0x28);
  uVar10 = *(ulonglong *)(lVar2 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar2 + 0x30) < uVar15) {
    lVar18 = 8;
    uVar10 = uVar21;
  }
  *puVar3 = (*(longlong *)(lVar2 + 0x40) - uVar10) - uVar14;
  fVar9 = __real_3dcccccd;
  fVar8 = __real_3f800000;
  fVar7 = __real_3f000000;
  lVar20 = uVar14 * 0x10 + lVar23;
  lVar19 = uVar10 * 0x10 + lVar18;
  lVar2 = *in_R9;
  lVar4 = in_R9[1];
  uVar1 = *(undefined4 *)(in_stack_00000028 + 0x1c);
  puVar5 = *(undefined4 **)(in_stack_00000028 + 0x10);
  uVar6 = *(undefined8 *)(in_stack_00000030 + 8);
LAB_14231d8f6:
  do {
    if (puVar22 == (ulonglong *)0x0) goto LAB_14231d9d2;
    if ((local_100 == 0) || (local_100 == lVar16)) {
      if ((local_108 == 0) || (local_108 == lVar17)) {
        local_100 = 0;
        goto LAB_14231d9d2;
      }
      local_100 = 0;
      lVar11 = local_108;
      local_108 = local_108 + 0x20;
    }
    else {
      lVar11 = local_100;
      local_100 = local_100 + 0x20;
    }
    *puVar22 = *puVar22 + 1;
    uVar14 = *(ulonglong *)(lVar11 + 8);
    if (uVar14 >> 0x20 == 0) goto LAB_14231d9d2;
    uVar10 = uVar14 & 0xffffffff;
  } while (*(ulonglong *)(lVar4 + 0x10) <= uVar10);
  goto LAB_14231da28;
LAB_14231d9d2:
  do {
    if ((lVar23 == 0) || (lVar23 == lVar20)) {
      if ((lVar18 == 0) || (lVar18 == lVar19)) {
        *puVar5 = uVar1;
        country_core::resources::critters::BirdSpotManager::
        recycle<system_critters::wren::update_roof_bird_spots::closure_env_3>();
        return;
      }
      lVar18 = lVar18 + 0x10;
      lVar23 = 0;
    }
    else {
      lVar23 = lVar23 + 0x10;
    }
    *puVar3 = *puVar3 + 1;
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  } while (extraout_RAX >> 0x20 == 0);
  puVar22 = (ulonglong *)0x0;
  uVar10 = extraout_RAX & 0xffffffff;
  uVar14 = extraout_RAX;
  if (*(ulonglong *)(lVar4 + 0x10) <= uVar10) goto LAB_14231d8f6;
LAB_14231da28:
  if (*(int *)(*(longlong *)(lVar4 + 8) + uVar10 * 0x14) != (int)(uVar14 >> 0x20))
  goto LAB_14231d8f6;
  lVar11 = *(longlong *)(lVar4 + 8) + uVar10 * 0x14;
  uVar14 = (ulonglong)*(uint *)(lVar11 + 4);
  if (((uVar14 == 0xffffffff) || (*(ulonglong *)(lVar2 + 0x38) <= uVar14)) ||
     ((*(ulonglong *)(*(longlong *)(lVar2 + 0x28) + (ulonglong)(*(uint *)(lVar11 + 4) >> 6) * 8) >>
       (uVar14 & 0x3f) & 1) == 0)) goto LAB_14231d8f6;
  if (*(ulonglong *)(lVar2 + 0x110) <
      *(ulonglong *)
       (*(longlong *)(lVar4 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar11 + 0xc) * 0x48)) {
    lVar12 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)*(uint *)(lVar11 + 0xc) * 0x48;
    uVar14 = *(ulonglong *)(*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar2 + 0x110) * 8);
    if (uVar14 != 0) {
      lVar12 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar14 * 0x30);
      goto LAB_14231daf6;
    }
  }
  lVar12 = 0;
LAB_14231daf6:
  lVar11 = (ulonglong)*(uint *)(lVar11 + 0x10) * 0x58;
  if (*(int *)(lVar12 + 8 + lVar11) == 1) {
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (extraout_AL == '\0') {
      country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls();
      ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
      ::utils::geometry::rectangle::Rectangle2d::new_axis_aligned();
      country_core::resources::roofs::roof_shape::Roof::is_gable();
      if ((extraout_AL_00 == '\0') || (*(char *)(lVar12 + lVar11 + 0x54) != '\0')) {
        country_core::resources::walls::decorator::ClutterVersion::is_initial();
        ::utils::geometry::rectangle::Rectangle2d::get_edge_pts();
        fVar24 = extraout_EAX;
        fVar26 = extraout_var;
      }
      else {
        country_core::resources::walls::decorator::ClutterVersion::is_initial();
        ::utils::geometry::rectangle::Rectangle2d::get_edge_pts();
        fVar25 = (float)local_138;
        fVar27 = (float)((ulonglong)local_138 >> 0x20);
        fVar29 = fVar9 / SQRT((extraout_var - fVar27) * (extraout_var - fVar27) +
                              (extraout_EAX - fVar25) * (extraout_EAX - fVar25));
        fVar28 = fVar8 - fVar29;
        fVar24 = fVar25 * fVar29 + extraout_EAX * fVar28;
        fVar26 = fVar27 * fVar29 + extraout_var * fVar28;
        local_138 = CONCAT44(extraout_var * fVar29 + fVar27 * fVar28,
                             extraout_EAX * fVar29 + fVar25 * fVar28);
      }
      fVar24 = fVar24 - (float)local_138;
      fVar26 = fVar26 - (float)((ulonglong)local_138 >> 0x20);
      if (fVar7 < SQRT(fVar26 * fVar26 + fVar24 * fVar24)) {
        local_158 = 0;
        ::utils::rng::init_rng();
        ::utils::random_splits();
        lVar11 = local_240 + local_238 * 4;
        bVar13 = true;
        lVar12 = local_240;
        while (lVar12 != lVar11) {
          local_1c0 = lVar12 + 4;
          if (bVar13) {
            if (local_1c0 == lVar11) break;
            local_1c0 = lVar12 + 8;
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
          bVar13 = false;
          lVar12 = local_1c0;
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

