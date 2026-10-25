
void __cdecl system_critters::fish::fish(void)

{
  undefined8 *puVar1;
  uint *puVar2;
  undefined8 uVar3;
  char cVar4;
  uint uVar5;
  ulonglong *puVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  longlong extraout_RAX;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar19;
  longlong *in_RDX;
  ulonglong uVar20;
  ulonglong uVar21;
  undefined **ppuVar22;
  longlong lVar23;
  longlong lVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  ulonglong *in_R9;
  undefined8 *puVar27;
  float *pfVar28;
  undefined4 *puVar29;
  longlong *plVar30;
  uint *puVar31;
  longlong lVar32;
  longlong *plVar33;
  longlong lVar34;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000068;
  longlong *in_stack_00000070;
  longlong *in_stack_00000078;
  undefined8 *in_stack_00000080;
  undefined8 local_290;
  ulonglong local_288 [2];
  float local_278;
  undefined8 local_270;
  float local_268;
  undefined8 local_260;
  float local_258;
  undefined8 local_250;
  float local_248;
  longlong local_240;
  longlong local_238;
  longlong local_230;
  undefined8 local_228;
  longlong *local_220;
  undefined8 local_218;
  float local_210;
  undefined8 local_208;
  undefined8 local_200;
  ulonglong local_1f8;
  longlong local_1f0;
  undefined8 local_1e8;
  undefined8 local_1d8;
  float local_1d0;
  undefined8 local_1c8;
  longlong local_1c0;
  ulonglong local_1b8;
  undefined4 *local_1b0;
  longlong local_1a8;
  ulonglong *local_1a0;
  longlong local_198;
  undefined8 *local_190;
  code *pcStack_188;
  undefined8 *local_180;
  code *local_178;
  undefined8 local_170;
  uint *local_168;
  undefined8 local_160;
  undefined8 *local_158;
  longlong *local_150;
  float *local_148;
  longlong *local_140;
  undefined8 *local_138;
  float fStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_108;
  ulonglong *local_100;
  float *local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_168 = (uint *)*in_stack_00000050;
  local_1c8 = in_stack_00000050[4];
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  plVar30 = (longlong *)*in_stack_00000028;
  local_220 = in_RCX;
  local_1a0 = in_R9;
  local_150 = in_RDX;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  puVar1 = (undefined8 *)(extraout_RAX + 0x20);
  puVar27 = (undefined8 *)(extraout_RAX + 0x28);
  if (*(char *)(extraout_RAX + 0x298) == '\0') {
    puVar27 = (undefined8 *)(extraout_RAX + 0x10);
    puVar1 = (undefined8 *)(extraout_RAX + 8);
  }
  local_200 = *puVar27;
  local_208 = *puVar1;
  country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
  local_140 = plVar30;
  if ((ulonglong)plVar30[6] < 200) {
    lVar23 = *in_stack_00000070;
    puVar29 = (undefined4 *)in_stack_00000070[2];
    local_f8 = (float *)CONCAT44(local_f8._4_4_,*(undefined4 *)((longlong)in_stack_00000070 + 0x1c))
    ;
    local_148 = (float *)(plVar30 + 3);
    lVar24 = 0;
    local_1b0 = puVar29;
    do {
      if ((ulonglong)local_140[6] < 200) {
        local_288[0] = *(ulonglong *)(lVar23 + 0x10);
        local_290 = 0;
        if (local_288[0] == 0) {
          local_160 = 0;
          local_158 = &local_290;
          local_108 = 1;
          local_100 = local_288;
          local_190 = &local_160;
          pcStack_188 = core::ops::range::impl$75::fmt<ref_<usize>_>;
          local_180 = &local_108;
          local_178 = core::ops::range::impl$75::fmt<ref_<usize>_>;
          local_138 = (undefined8 *)&DAT_142badc80;
          fStack_130 = 2.8026e-45;
          uStack_12c = 0;
          local_118 = 0;
          uStack_128 = &local_190;
          local_120 = 2;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        uVar21 = *local_1a0;
        uVar18 = uVar21 + 0xa0761d6478bd642f;
        auVar7._8_8_ = 0;
        auVar7._0_8_ = uVar18 ^ 0xe7037ed1a0b428db;
        auVar12._8_8_ = 0;
        auVar12._0_8_ = uVar18;
        auVar8._8_8_ = 0;
        auVar8._0_8_ = SUB168(auVar7 * auVar12,0) ^ SUB168(auVar7 * auVar12,8);
        auVar13._8_8_ = 0;
        auVar13._0_8_ = local_288[0];
        uVar20 = SUB168(auVar8 * auVar13,8);
        uVar16 = SUB168(auVar8 * auVar13,0);
        *local_1a0 = uVar18;
        if (uVar16 < local_288[0]) {
          uVar17 = -local_288[0];
          if ((uVar17 | local_288[0]) >> 0x20 == 0) {
            uVar17 = (uVar17 & 0xffffffff) % (local_288[0] & 0xffffffff);
          }
          else {
            uVar17 = uVar17 % local_288[0];
          }
          if (uVar16 < uVar17) {
            uVar21 = uVar21 + 0x40ec3ac8f17ac85e;
            do {
              auVar9._8_8_ = 0;
              auVar9._0_8_ = uVar21 ^ 0xe7037ed1a0b428db;
              auVar14._8_8_ = 0;
              auVar14._0_8_ = uVar21;
              uVar18 = uVar18 + 0xa0761d6478bd642f;
              uVar16 = SUB168(auVar9 * auVar14,8) ^ SUB168(auVar9 * auVar14,0);
              uVar21 = uVar21 + 0xa0761d6478bd642f;
            } while (local_288[0] * uVar16 < uVar17);
            *local_1a0 = uVar18;
            auVar10._8_8_ = 0;
            auVar10._0_8_ = uVar16;
            auVar15._8_8_ = 0;
            auVar15._0_8_ = local_288[0];
            uVar20 = SUB168(auVar10 * auVar15,8);
          }
        }
        *puVar29 = local_f8._0_4_;
        if (*(ulonglong *)(lVar23 + 0x10) <= uVar20) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar34 = *(longlong *)(lVar23 + 8) + uVar20 * 0xc;
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        if (extraout_XMM0_Da <= 0.5) {
          *(undefined1 *)(lVar34 + 8) = 0;
        }
        else if (*(char *)(lVar34 + 8) == '\0') {
          *(undefined1 *)(lVar34 + 8) = 1;
          ::utils::swizzlers::impl$0::x0y();
          local_108 = CONCAT44(pcStack_188._0_4_,(int)local_190);
          ::utils::swizzlers::impl$1::xvy();
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          fVar45 = SUB84(local_138,0);
          uVar18 = (ulonglong)local_138 >> 0x20;
          fVar43 = (extraout_XMM0_Da_01 + -0.5) * 0.5 + fStack_130;
          ::utils::components::transform::impl$1::default();
          puVar1 = local_138;
          lVar34 = CONCAT44(uStack_12c,fStack_130);
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          bevy_ecs::system::commands::Commands::spawn_empty();
          uVar3 = local_118;
          plVar30 = (longlong *)CONCAT44(uStack_12c,fStack_130);
          if (plVar30 == (longlong *)0x0) {
            plVar30 = (longlong *)CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
          }
          lVar32 = plVar30[2];
          if ((ulonglong)(*plVar30 - lVar32) < 0x88) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          fVar45 = (extraout_XMM0_Da_00 + -0.5) * 0.5 + fVar45;
          fVar46 = (float)uVar18 + -0.25;
          lVar19 = plVar30[1];
          *(code **)(lVar19 + lVar32) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::resources::critters::Fish,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          plVar33 = (longlong *)(lVar19 + 8 + lVar32);
          *plVar33 = (longlong)puVar1;
          plVar33[1] = lVar34;
          *(ulonglong *)(lVar19 + 0x18 + lVar32) = CONCAT44(fVar46,fVar45);
          *(float *)(lVar19 + 0x20 + lVar32) = fVar43;
          *(undefined8 *)(lVar19 + 0x24 + lVar32) = 0x3f8000003f800000;
          *(undefined4 *)(lVar19 + 0x2c + lVar32) = 0x3f800000;
          *(undefined4 *)(lVar19 + 0x38 + lVar32) = 0;
          *(undefined8 *)(lVar19 + 0x40 + lVar32) = 0x2cae82919f90dd16;
          *(undefined1 *)(lVar19 + 0x48 + lVar32) = 3;
          *(undefined1 *)(lVar19 + 0x58 + lVar32) = 0;
          *(undefined8 *)(lVar19 + 0x68 + lVar32) = uVar3;
          *(code **)(lVar19 + 0x78 + lVar32) = bevy_ecs::error::handler::panic;
          plVar30[2] = lVar32 + 0x88;
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          plVar30 = local_140;
          lVar34 = local_140[2];
          if (lVar34 == *local_140) {
            alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
          }
          lVar32 = plVar30[1];
          *(ulonglong *)(lVar32 + lVar34 * 0x18) = CONCAT44(fVar46,fVar45);
          *(float *)(lVar32 + 8 + lVar34 * 0x18) = fVar43;
          *(ulonglong *)(lVar32 + 0xc + lVar34 * 0x18) =
               CONCAT44(extraout_XMM0_Da_03 + -0.5,extraout_XMM0_Da_02 + -0.5);
          *(float *)(lVar32 + 0x14 + lVar34 * 0x18) = extraout_XMM0_Da_04 + -0.5;
          plVar30[2] = lVar34 + 1;
          lVar34 = plVar30[5];
          if (lVar34 == plVar30[3]) {
            alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>()
            ;
          }
          lVar32 = local_140[4];
          lVar19 = lVar34 * 0x38;
          *(undefined4 *)(lVar32 + lVar19) = 0;
          *(undefined8 *)(lVar32 + 0x10 + lVar19) = uVar3;
          *(undefined8 *)(lVar32 + 0x18 + lVar19) = 0;
          *(undefined4 *)(lVar32 + 0x20 + lVar19) = 0x3f800000;
          *(undefined8 *)(lVar32 + 0x24 + lVar19) = 0;
          *(undefined4 *)(lVar32 + 0x2c + lVar19) = 0x3f800000;
          *(undefined4 *)(lVar32 + 0x30 + lVar19) = 0;
          local_140[5] = lVar34 + 1;
          local_140[6] = local_140[6] + 1;
          puVar29 = local_1b0;
        }
      }
      lVar24 = lVar24 + 1;
    } while (lVar24 != 0x14);
  }
  plVar30 = local_140;
  local_1b8 = local_140[2];
  auVar11._8_8_ = 0;
  auVar11._0_8_ = local_1b8;
  uVar18 = SUB168(auVar11 * ZEXT816(0x18),0);
  if ((SUB168(auVar11 * ZEXT816(0x18),8) != 0) || (0x7ffffffffffffffc < uVar18)) {
LAB_140f92794:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (uVar18 == 0) {
    local_198 = 4;
  }
  else {
    plVar33 = (longlong *)local_140[1];
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == 0) goto LAB_140f92794;
    local_198 = extraout_RAX_00;
    if (local_1b8 != 0) {
      uVar16 = 0;
      uVar21 = local_1b8;
      do {
        if (uVar18 == uVar16) break;
        uStack_128._4_4_ = *(undefined4 *)((longlong)plVar33 + 0x14);
        uStack_12c = (undefined4)*(undefined8 *)((longlong)plVar33 + 0xc);
        uStack_128._0_4_ = (undefined4)((ulonglong)*(undefined8 *)((longlong)plVar33 + 0xc) >> 0x20)
        ;
        fStack_130 = *(float *)(plVar33 + 1);
        lVar23 = plVar33[1];
        local_138 = (undefined8 *)*plVar33;
        plVar33 = plVar33 + 3;
        *(ulonglong *)(extraout_RAX_00 + 0x10 + uVar16) =
             CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
        *(longlong *)(extraout_RAX_00 + uVar16) = (longlong)local_138;
        ((longlong *)(extraout_RAX_00 + uVar16))[1] = lVar23;
        uVar16 = uVar16 + 0x18;
        uVar21 = uVar21 - 1;
      } while (uVar21 != 0);
    }
  }
  local_190 = (undefined8 *)0x0;
  pcStack_188 = (code *)0x8;
  local_180 = (undefined8 *)0x0;
  uVar5 = *local_168;
  lVar23 = 0;
  if (uVar5 - 6 < 0xf) {
    lVar23 = (ulonglong)uVar5 - 5;
  }
  fVar43 = 0.0;
  switch(lVar23) {
  case 0:
    if (uVar5 != 0) break;
  case 1:
  case 8:
  case 0xb:
  case 0xd:
    fVar43 = 2.0;
    break;
  case 2:
  case 3:
    fVar43 = *(float *)*in_stack_00000080 + 2.3;
    break;
  case 4:
  case 5:
  case 10:
  case 0xc:
    fVar43 = 3.98;
  }
  local_1c0 = plVar30[1];
  local_230 = plVar30[4];
  local_1f8 = plVar30[2];
  if ((ulonglong)plVar30[5] < (ulonglong)plVar30[2]) {
    local_1f8 = plVar30[5];
  }
  local_238 = *in_stack_00000078 + 0x58;
  local_170 = *in_stack_00000030;
  local_228 = *in_stack_00000068;
  local_1a8 = *local_150;
  local_1f0 = local_150[1];
  local_1e8 = *in_stack_00000040;
  local_168 = (uint *)0x0;
  plVar30 = (longlong *)0x0;
LAB_140f9150c:
  while( true ) {
    puVar2 = local_168;
    local_1b0 = local_168;
    if (local_1f8 <= local_168) {
      uStack_128._0_4_ = SUB84(local_180,0);
      uStack_128._4_4_ = (undefined4)((ulonglong)local_180 >> 0x20);
      local_138 = local_190;
      fStack_130 = SUB84(pcStack_188,0);
      uStack_12c = (undefined4)((ulonglong)pcStack_188 >> 0x20);
      boids::BoidFlock::remove<system_critters::fish::fish::closure_env_1>();
      local_140[6] = local_140[2];
      puVar6 = (ulonglong *)*in_stack_00000058;
      lVar23 = in_stack_00000058[1];
      uVar18 = *puVar6;
      uVar16 = 0;
      uVar21 = uVar18 - *(ulonglong *)(lVar23 + 0x18);
      if (uVar18 < *(ulonglong *)(lVar23 + 0x18)) {
        uVar21 = uVar16;
      }
      uVar20 = uVar18 - *(ulonglong *)(lVar23 + 0x38);
      if (uVar18 < *(ulonglong *)(lVar23 + 0x38)) {
        uVar20 = uVar16;
      }
      uVar18 = *(ulonglong *)(lVar23 + 0x10);
      uVar17 = *(ulonglong *)(lVar23 + 0x30);
      uVar26 = uVar17 - uVar20;
      if (uVar17 < uVar20) {
        uVar26 = uVar16;
      }
      uVar25 = uVar18 - uVar21;
      if (uVar18 < uVar21) {
        uVar25 = uVar16;
      }
      uVar16 = *(ulonglong *)(lVar23 + 0x40);
      *puVar6 = (uVar16 - uVar26) - uVar25;
      if (uVar18 >= uVar21 && uVar18 - uVar21 != 0 || uVar17 >= uVar20 && uVar17 - uVar20 != 0) {
        *puVar6 = uVar16;
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        if ((*(uint *)(extraout_RAX_02 + 600) < 2) &&
           (*(float *)(extraout_RAX_02 + 0x280) <= 0.3 && *(float *)(extraout_RAX_02 + 0x280) != 0.3
           )) {
          country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
          country_core::systems::wall::valid_enclosure::ShapeParameters::expand();
          lVar23 = local_140[1];
          uVar18 = local_140[2];
          if ((ulonglong)local_140[5] < (ulonglong)local_140[2]) {
            uVar18 = local_140[5];
          }
          uVar21 = 0;
          while (uVar16 = uVar21, uVar16 < uVar18) {
            fVar43 = *(float *)(lVar23 + uVar16 * 0x18);
            country_core::resources::glade_border::GladeBorder::is_pos_inside();
            uVar21 = uVar16 + 1;
            if (extraout_AL_01 != '\0') {
              pfVar28 = (float *)(lVar23 + uVar16 * 0x18);
              fVar45 = *pfVar28;
              fVar46 = pfVar28[2];
              country_core::systems::wall::valid_enclosure::ShapeParameters::center();
              local_160 = CONCAT44(fVar46 - fVar43,fVar45 - extraout_XMM0_Da_10);
              ::utils::swizzlers::impl$0::x0y();
              *(ulonglong *)(pfVar28 + 3) =
                   CONCAT44((float)((ulonglong)local_108 >> 0x20) * 8.0,(float)local_108 * 8.0);
              pfVar28[5] = local_100._0_4_ * 8.0;
            }
          }
        }
      }
      local_168 = (uint *)local_140[1];
      lVar23 = local_140[4];
      uVar18 = local_140[2];
      if ((ulonglong)local_140[5] < (ulonglong)local_140[2]) {
        uVar18 = local_140[5];
      }
      local_148 = (float *)*local_220;
      lVar24 = local_220[1];
      local_1c0 = CONCAT44(local_1c0._4_4_,*(undefined4 *)((longlong)local_220 + 0x14));
      uVar21 = 0;
      pfVar28 = local_148;
      while (uVar16 = uVar21, uVar16 < uVar18) {
        uVar21 = uVar16 + 1;
        uVar20 = *(ulonglong *)(lVar23 + 0x10 + uVar16 * 0x38);
        uVar17 = uVar20 & 0xffffffff;
        if ((uVar17 < *(ulonglong *)(lVar24 + 0x10)) &&
           (*(int *)(*(longlong *)(lVar24 + 8) + uVar17 * 0x14) == (int)(uVar20 >> 0x20))) {
          lVar34 = *(longlong *)(lVar24 + 8) + uVar17 * 0x14;
          uVar20 = (ulonglong)*(uint *)(lVar34 + 4);
          if ((uVar20 != 0xffffffff) &&
             ((uVar20 < *(ulonglong *)((longlong)pfVar28 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)((longlong)pfVar28 + 0x28) +
                 (ulonglong)(*(uint *)(lVar34 + 4) >> 6) * 8) >> (uVar20 & 0x3f) & 1) != 0)))) {
            plVar30 = (longlong *)((longlong)local_168 + uVar16 * 0x18);
            local_f8 = (float *)(ulonglong)*(uint *)(lVar34 + 0x10);
            lVar32 = *(longlong *)
                      (*(longlong *)(lVar24 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar34 + 0xc) * 0x48);
            lVar19 = ~*(ulonglong *)
                       (*(longlong *)
                         (*(longlong *)(lVar24 + 0x1a8) + 0x38 +
                         (ulonglong)*(uint *)(lVar34 + 0xc) * 0x48) +
                       *(longlong *)((longlong)pfVar28 + 0x110) * 8) * 0x30;
            lVar34 = *(longlong *)(lVar32 + 0x10 + lVar19);
            local_150 = *(longlong **)(lVar32 + 0x28 + lVar19);
            lVar32 = (longlong)local_f8 * 0x30;
            pfVar28 = (float *)(lVar34 + lVar32);
            fVar43 = *pfVar28;
            fVar45 = pfVar28[1];
            fVar46 = pfVar28[2];
            fVar41 = pfVar28[3];
            fVar37 = fVar41 * fVar41;
            fVar39 = fVar46 * fVar46 + fVar45 * fVar45 + fVar43 * fVar43;
            fVar44 = fVar46 * -1.0 + fVar45 * -0.0 + fVar43 * -0.0;
            fVar44 = fVar44 + fVar44;
            fVar41 = fVar41 + fVar41;
            local_210 = fVar41 * (fVar43 * -0.0 - fVar45 * -0.0) +
                        fVar44 * fVar46 + (fVar37 - fVar39) * -1.0;
            local_218 = CONCAT44(fVar41 * (fVar46 * -0.0 - fVar43 * -1.0) +
                                 fVar44 * fVar45 + (fVar37 - fVar39) * -0.0,
                                 fVar41 * (fVar45 * -1.0 - fVar46 * -0.0) +
                                 fVar44 * fVar43 + (fVar37 - fVar39) * -0.0);
            local_108 = CONCAT44(*(undefined4 *)((longlong)plVar30 + 0x14),
                                 *(undefined4 *)((longlong)plVar30 + 0xc));
            ::utils::swizzlers::impl$0::x0y();
            fVar43 = SUB84(local_138,0);
            fVar45 = (float)((ulonglong)local_138 >> 0x20);
            fVar46 = 1.0 / SQRT(fStack_130 * fStack_130 + fVar45 * fVar45 + fVar43 * fVar43);
            if ((int)fVar46 - 1U < 0x7fffff ||
                (int)ABS(fVar46) - 0x800000U < 0x7f000000 && -1 < (int)fVar46) {
              local_1d0 = fStack_130 * fVar46;
              local_1d8 = CONCAT44(fVar45 * fVar46,fVar43 * fVar46);
            }
            else {
              local_1d0 = 0.0;
              local_1d8 = 0;
            }
            plVar33 = (longlong *)(lVar34 + lVar32);
            ::utils::time_continuous_lerp_vec3();
            dolly::util::
            look_at<dolly::handedness::RightHanded,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>
                      ();
            *(undefined4 *)((longlong)local_150 + (longlong)local_f8 * 4) = (undefined4)local_1c0;
            *plVar33 = (longlong)local_138;
            plVar33[1] = CONCAT44(uStack_12c,fStack_130);
            fStack_130 = *(float *)(plVar30 + 1);
            local_138 = (undefined8 *)*plVar30;
            *(undefined4 *)((longlong)local_150 + (longlong)local_f8 * 4) = (undefined4)local_1c0;
            *(float *)(plVar33 + 3) = fStack_130;
            plVar33[2] = (longlong)local_138;
            pfVar28 = local_148;
          }
        }
      }
      if (local_1b8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    puVar1 = local_180;
    local_168 = (uint *)((longlong)local_1b0 + 1);
    local_150 = (longlong *)((longlong)plVar30 + 1);
    if (0.5 < extraout_XMM0_Da_05) break;
    if (local_180 == local_190) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    *(longlong **)((longlong)pcStack_188 + (longlong)puVar1 * 8) = plVar30;
    local_180 = (undefined8 *)((longlong)puVar1 + 1);
    plVar30 = local_150;
  }
  local_f8 = (float *)(local_1c0 + (longlong)puVar2 * 0x18);
  fVar45 = *local_f8;
  fVar46 = local_f8[2];
  country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
  ppuVar22 = (undefined **)((longlong)local_1b0 * 0x38 + local_230);
  local_138 = &local_208;
  boids::Boid::frame_update<system_critters::fish::fish::closure_env_0>();
  puVar1 = local_180;
  if (extraout_AL != '\0') {
    if (local_180 == local_190) {
      ppuVar22 = &PTR_anon_d50c582ad1069d4dab52714d73220c32_9_llvm_7307022832566353767_142badd40;
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    *(longlong **)((longlong)pcStack_188 + (longlong)puVar1 * 8) = plVar30;
    local_180 = (undefined8 *)((longlong)puVar1 + 1);
  }
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  fVar44 = extraout_XMM0_Da_06 + extraout_XMM0_Da_07 + extraout_XMM0_Da_08 + extraout_XMM0_Da_09;
  fVar41 = 0.0;
  if (0.0 <= fVar44) {
    fVar41 = fVar44;
  }
  fVar44 = 1.0;
  if (fVar41 <= 1.0) {
    fVar44 = fVar41;
  }
  if ((fVar44 < 0.1) &&
     (country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query(),
     ppuVar22 != (undefined **)0x0)) {
    lVar23 = 0;
    do {
      pfVar28 = (float *)(extraout_RAX_01 + 0x20 + lVar23);
      if ((*pfVar28 <= 0.2 && *pfVar28 != 0.2) &&
         (pfVar28 = (float *)(extraout_RAX_01 + 0x24 + lVar23), *pfVar28 <= 0.2 && *pfVar28 != 0.2))
      {
        fVar37 = (float)*(undefined8 *)(extraout_RAX_01 + lVar23);
        fVar39 = (float)((ulonglong)*(undefined8 *)(extraout_RAX_01 + lVar23) >> 0x20);
        uVar3 = *(undefined8 *)(extraout_RAX_01 + 8 + lVar23);
        fVar41 = (float)uVar3;
        fVar44 = (float)((ulonglong)uVar3 >> 0x20);
        fVar35 = fVar41 - fVar37;
        fVar36 = fVar44 - fVar39;
        fVar38 = fVar36 * fVar36 + fVar35 * fVar35;
        if (fVar38 <= 1e-20) {
          fVar38 = 1e-20;
        }
        fVar40 = ((fVar46 - fVar39) * fVar36 + (fVar45 - fVar37) * fVar35) / fVar38;
        fVar42 = 0.0;
        if (0.0 <= fVar40) {
          fVar42 = fVar40;
        }
        fVar40 = 1.0;
        if (fVar42 <= 1.0) {
          fVar40 = fVar42;
        }
        fVar42 = fVar45 - (fVar40 * fVar41 + (1.0 - fVar40) * fVar37);
        fVar40 = fVar46 - (fVar40 * fVar44 + (1.0 - fVar40) * fVar39);
        if (SQRT(fVar40 * fVar40 + fVar42 * fVar42) < 1.12) {
          fVar38 = (fVar36 * (local_f8[2] - fVar39) + fVar35 * (*local_f8 - fVar37)) / fVar38;
          fVar45 = 0.0;
          if (0.0 <= fVar38) {
            fVar45 = fVar38;
          }
          fVar46 = 1.0;
          if (fVar45 <= 1.0) {
            fVar46 = fVar45;
          }
          fVar41 = *local_f8 - (fVar41 * fVar46 + (1.0 - fVar46) * fVar37);
          fVar46 = local_f8[2] - (fVar44 * fVar46 + (1.0 - fVar46) * fVar39);
          fVar45 = 1.0 / SQRT(fVar46 * fVar46 + fVar41 * fVar41);
          local_138 = (undefined8 *)CONCAT44(fVar45 * fVar46,fVar45 * fVar41);
          ::utils::swizzlers::impl$0::x0y();
          local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_f8[5]);
          local_108 = *(undefined8 *)(local_f8 + 3);
          fVar45 = (float)*(longlong *)(local_f8 + 4);
          fVar46 = (float)((ulonglong)*(longlong *)(local_f8 + 4) >> 0x20);
          fVar45 = SQRT(fVar46 * fVar46 + local_f8[3] * local_f8[3] + fVar45 * fVar45);
          fStack_130 = fVar45 * local_248;
          local_138 = (undefined8 *)
                      CONCAT44((float)((ulonglong)local_250 >> 0x20) * fVar45,
                               (float)local_250 * fVar45);
          ::utils::time_continuous_lerp_vec3();
          local_f8[5] = local_158._0_4_;
          *(undefined8 *)(local_f8 + 3) = local_160;
          break;
        }
      }
      lVar23 = lVar23 + 0x38;
    } while ((longlong)ppuVar22 * 0x38 - lVar23 != 0);
  }
  local_108 = CONCAT44(local_f8[2],*local_f8);
  ::utils::swizzlers::impl$1::xvy();
  local_f8[2] = fStack_130;
  *(undefined8 **)local_f8 = local_138;
  puVar31 = *(uint **)(local_1a8 + 0xf8);
  puVar2 = puVar31 + *(longlong *)(local_1a8 + 0x100);
  cVar4 = *(char *)(local_1a8 + 0x108);
  local_148 = local_f8 + 3;
  lVar34 = 8;
  lVar23 = 0;
  uVar18 = 0;
  uVar21 = 0;
  lVar24 = 0;
  lVar32 = local_1f0;
  do {
    if (cVar4 == '\0') {
      if (uVar21 == uVar18) {
        do {
          if (puVar31 == puVar2) goto LAB_140f91d50;
          uVar5 = *puVar31;
          puVar31 = puVar31 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar32 + 0x100) + 0x58 + (ulonglong)uVar5 * 0xa0);
        } while (uVar18 == 0);
        lVar19 = *(longlong *)(lVar32 + 0x100) + (ulonglong)uVar5 * 0xa0;
        uVar21 = (ulonglong)*(uint *)(lVar19 + 0x94);
        lVar23 = *(longlong *)(lVar32 + 0x1a8);
        lVar24 = lVar23 + uVar21 * 0x48;
        lVar34 = *(longlong *)(lVar23 + 0x38 + uVar21 * 0x48);
        uVar21 = *(ulonglong *)(lVar23 + 0x40 + uVar21 * 0x48);
        if ((*(ulonglong *)(local_1a8 + 0x118) < uVar21) &&
           (uVar16 = *(ulonglong *)(lVar34 + *(ulonglong *)(local_1a8 + 0x118) * 8), uVar16 != 0)) {
          lVar23 = *(longlong *)(*(longlong *)(lVar24 + 0x18) + 0x10 + ~uVar16 * 0x30);
          uVar16 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar21 <= uVar16) goto LAB_140f91bb1;
LAB_140f91b8a:
          uVar21 = *(ulonglong *)(lVar34 + uVar16 * 8);
          if (uVar21 == 0) goto LAB_140f91bb1;
          lVar24 = *(longlong *)(*(longlong *)(lVar24 + 0x18) + 0x10 + ~uVar21 * 0x30);
        }
        else {
          lVar23 = 0;
          uVar16 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar16 < uVar21) goto LAB_140f91b8a;
LAB_140f91bb1:
          lVar24 = 0;
        }
        lVar34 = *(longlong *)(lVar19 + 0x50);
        uVar21 = 0;
      }
      uVar16 = (ulonglong)*(uint *)(lVar34 + 8 + uVar21 * 0x10);
    }
    else {
      if (uVar21 == uVar18) {
        do {
          if (puVar31 == puVar2) goto LAB_140f91d50;
          uVar5 = *puVar31;
          puVar31 = puVar31 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar32 + 0x1a8) + 0x10 + (ulonglong)uVar5 * 0x48);
        } while (uVar18 == 0);
        lVar24 = *(longlong *)(lVar32 + 0x1a8) + (ulonglong)uVar5 * 0x48;
        uVar21 = *(ulonglong *)(lVar24 + 0x40);
        if ((*(ulonglong *)(local_1a8 + 0x118) < uVar21) &&
           (uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar24 + 0x38) + *(ulonglong *)(local_1a8 + 0x118) * 8),
           uVar16 != 0)) {
          lVar23 = *(longlong *)(*(longlong *)(lVar24 + 0x18) + 0x10 + ~uVar16 * 0x30);
          uVar16 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar21 <= uVar16) goto LAB_140f91ba9;
LAB_140f91b56:
          uVar21 = *(ulonglong *)(*(longlong *)(lVar24 + 0x38) + uVar16 * 8);
          if (uVar21 == 0) goto LAB_140f91ba9;
          lVar24 = *(longlong *)(*(longlong *)(lVar24 + 0x18) + 0x10 + ~uVar21 * 0x30);
        }
        else {
          lVar23 = 0;
          uVar16 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar16 < uVar21) goto LAB_140f91b56;
LAB_140f91ba9:
          lVar24 = 0;
        }
        uVar21 = 0;
      }
      uVar16 = uVar21 & 0xffffffff;
    }
    lVar19 = uVar16 * 0x30 + lVar23;
    uVar21 = uVar21 + 1;
    if (*(char *)(uVar16 + lVar24) == '\x02') {
      fVar45 = *(float *)(lVar19 + 0x10);
      fVar46 = *(float *)(lVar19 + 0x18);
      fVar41 = fVar45 - *local_f8;
      fVar44 = fVar46 - local_f8[2];
      if (SQRT(fVar44 * fVar44 + fVar41 * fVar41) < 2.0) {
        fVar45 = *local_f8 - fVar45;
        fVar46 = local_f8[2] - fVar46;
        fVar41 = 1.0 / SQRT(fVar46 * fVar46 + fVar45 * fVar45);
        local_138 = (undefined8 *)CONCAT44(fVar41 * fVar46,fVar41 * fVar45);
        local_240 = lVar24;
        ::utils::swizzlers::impl$0::x0y();
        local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_148[2]);
        local_108 = *(undefined8 *)local_148;
        fStack_130 = local_258 * 5.0;
        local_138 = (undefined8 *)
                    CONCAT44((float)((ulonglong)local_260 >> 0x20) * 5.0,(float)local_260 * 5.0);
        ::utils::time_continuous_lerp_vec3();
        local_148[2] = local_158._0_4_;
        *(undefined8 *)local_148 = local_160;
        lVar24 = local_240;
        lVar32 = local_1f0;
      }
    }
  } while( true );
LAB_140f91d50:
  country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
  fVar45 = *local_f8;
  fVar46 = local_f8[2];
  fVar41 = local_138._0_4_ - fVar45;
  fVar44 = fStack_130 - fVar46;
  if (SQRT(fVar44 * fVar44 + fVar41 * fVar41) < fVar43) {
    country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
    fVar45 = fVar45 - local_138._0_4_;
    fVar46 = fVar46 - fStack_130;
    fVar41 = 1.0 / SQRT(fVar46 * fVar46 + fVar45 * fVar45);
    local_138 = (undefined8 *)CONCAT44(fVar41 * fVar46,fVar41 * fVar45);
    ::utils::swizzlers::impl$0::x0y();
    local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_148[2]);
    local_108 = *(undefined8 *)local_148;
    fStack_130 = local_268 * 8.0;
    local_138 = (undefined8 *)
                CONCAT44((float)((ulonglong)local_270 >> 0x20) * 8.0,(float)local_270 * 8.0);
    ::utils::time_continuous_lerp_vec3();
    local_148[2] = local_158._0_4_;
    *(undefined8 *)local_148 = local_160;
  }
  country_core::systems::camera_rig::MainCamera::mode();
  plVar30 = local_150;
  if (extraout_AL_00 == '\x03') {
    country_core::systems::camera_rig::MainCamera::eye_position();
    fVar45 = *local_f8;
    fVar46 = local_f8[2];
    fVar41 = local_138._0_4_ - fVar45;
    fVar44 = fStack_130 - fVar46;
    plVar30 = local_150;
    if (SQRT(fVar44 * fVar44 + fVar41 * fVar41) < 2.0) {
      country_core::systems::camera_rig::MainCamera::eye_position();
      fVar45 = fVar45 - local_138._0_4_;
      fVar46 = fVar46 - fStack_130;
      fVar41 = 1.0 / SQRT(fVar46 * fVar46 + fVar45 * fVar45);
      local_138 = (undefined8 *)CONCAT44(fVar41 * fVar46,fVar41 * fVar45);
      ::utils::swizzlers::impl$0::x0y();
      local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_148[2]);
      local_108 = *(undefined8 *)local_148;
      fStack_130 = local_278 * 8.0;
      local_138 = (undefined8 *)
                  CONCAT44((float)(local_288[1] >> 0x20) * 8.0,(float)local_288[1] * 8.0);
      ::utils::time_continuous_lerp_vec3();
      local_148[2] = local_158._0_4_;
      *(undefined8 *)local_148 = local_160;
      plVar30 = local_150;
    }
  }
  goto LAB_140f9150c;
}

