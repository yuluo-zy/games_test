
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar16;
  float fVar17;
  undefined4 uVar18;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  longlong extraout_RAX;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar22;
  longlong *in_RDX;
  ulonglong uVar23;
  ulonglong uVar24;
  undefined **ppuVar25;
  longlong lVar26;
  longlong lVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  ulonglong *in_R9;
  undefined8 *puVar30;
  float *pfVar31;
  undefined4 *puVar32;
  longlong *plVar33;
  uint *puVar34;
  longlong lVar35;
  longlong *plVar36;
  longlong lVar37;
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
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
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
  plVar33 = (longlong *)*in_stack_00000028;
  local_220 = in_RCX;
  local_1a0 = in_R9;
  local_150 = in_RDX;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  puVar1 = (undefined8 *)(extraout_RAX + 0x20);
  puVar30 = (undefined8 *)(extraout_RAX + 0x28);
  if (*(char *)(extraout_RAX + 0x298) == '\0') {
    puVar30 = (undefined8 *)(extraout_RAX + 0x10);
    puVar1 = (undefined8 *)(extraout_RAX + 8);
  }
  local_200 = *puVar30;
  local_208 = *puVar1;
  country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
  fVar41 = __xmm_0000000000000000be80000000000000._4_4_;
  fVar51 = _UNK_142a171f4;
  fVar42 = ___xmm_0000000000000000bf000000bf000000;
  fVar17 = __real_bf000000;
  fVar16 = __real_3f000000;
  local_140 = plVar33;
  if ((ulonglong)plVar33[6] < 200) {
    lVar26 = *in_stack_00000070;
    puVar32 = (undefined4 *)in_stack_00000070[2];
    local_f8 = (float *)CONCAT44(local_f8._4_4_,*(undefined4 *)((longlong)in_stack_00000070 + 0x1c))
    ;
    local_148 = (float *)(plVar33 + 3);
    lVar27 = 0;
    local_1b0 = puVar32;
    do {
      if ((ulonglong)local_140[6] < 200) {
        local_288[0] = *(ulonglong *)(lVar26 + 0x10);
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
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        uVar24 = *local_1a0;
        uVar21 = uVar24 + 0xa0761d6478bd642f;
        auVar7._8_8_ = 0;
        auVar7._0_8_ = uVar21 ^ 0xe7037ed1a0b428db;
        auVar12._8_8_ = 0;
        auVar12._0_8_ = uVar21;
        auVar8._8_8_ = 0;
        auVar8._0_8_ = SUB168(auVar7 * auVar12,0) ^ SUB168(auVar7 * auVar12,8);
        auVar13._8_8_ = 0;
        auVar13._0_8_ = local_288[0];
        uVar23 = SUB168(auVar8 * auVar13,8);
        uVar19 = SUB168(auVar8 * auVar13,0);
        *local_1a0 = uVar21;
        if (uVar19 < local_288[0]) {
          uVar20 = -local_288[0];
          if ((uVar20 | local_288[0]) >> 0x20 == 0) {
            uVar20 = (uVar20 & 0xffffffff) % (local_288[0] & 0xffffffff);
          }
          else {
            uVar20 = uVar20 % local_288[0];
          }
          if (uVar19 < uVar20) {
            uVar24 = uVar24 + 0x40ec3ac8f17ac85e;
            do {
              auVar9._8_8_ = 0;
              auVar9._0_8_ = uVar24 ^ 0xe7037ed1a0b428db;
              auVar14._8_8_ = 0;
              auVar14._0_8_ = uVar24;
              uVar21 = uVar21 + 0xa0761d6478bd642f;
              uVar19 = SUB168(auVar9 * auVar14,8) ^ SUB168(auVar9 * auVar14,0);
              uVar24 = uVar24 + 0xa0761d6478bd642f;
            } while (local_288[0] * uVar19 < uVar20);
            *local_1a0 = uVar21;
            auVar10._8_8_ = 0;
            auVar10._0_8_ = uVar19;
            auVar15._8_8_ = 0;
            auVar15._0_8_ = local_288[0];
            uVar23 = SUB168(auVar10 * auVar15,8);
          }
        }
        *puVar32 = local_f8._0_4_;
        if (*(ulonglong *)(lVar26 + 0x10) <= uVar23) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        lVar37 = *(longlong *)(lVar26 + 8) + uVar23 * 0xc;
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        if (extraout_XMM0_Da <= fVar16) {
          *(undefined1 *)(lVar37 + 8) = 0;
        }
        else if (*(char *)(lVar37 + 8) == '\0') {
          *(undefined1 *)(lVar37 + 8) = 1;
          ::utils::swizzlers::impl$0::x0y();
          local_108 = CONCAT44(pcStack_188._0_4_,(int)local_190);
          ::utils::swizzlers::impl$1::xvy();
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          fVar50 = SUB84(local_138,0);
          uVar21 = (ulonglong)local_138 >> 0x20;
          fVar49 = (extraout_XMM0_Da_01 + fVar17) * fVar16 + fStack_130;
          ::utils::components::transform::impl$1::default();
          puVar1 = local_138;
          lVar37 = CONCAT44(uStack_12c,fStack_130);
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          bevy_ecs::system::commands::Commands::spawn_empty();
          uVar3 = local_118;
          plVar33 = (longlong *)CONCAT44(uStack_12c,fStack_130);
          if (plVar33 == (longlong *)0x0) {
            plVar33 = (longlong *)CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
          }
          lVar35 = plVar33[2];
          if ((ulonglong)(*plVar33 - lVar35) < 0x88) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          fVar50 = (extraout_XMM0_Da_00 + fVar17) * fVar16 + fVar50;
          fVar52 = fVar41 + (float)uVar21;
          lVar22 = plVar33[1];
          *(code **)(lVar22 + lVar35) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::resources::critters::Fish,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          plVar36 = (longlong *)(lVar22 + 8 + lVar35);
          *plVar36 = (longlong)puVar1;
          plVar36[1] = lVar37;
          *(ulonglong *)(lVar22 + 0x18 + lVar35) = CONCAT44(fVar52,fVar50);
          *(float *)(lVar22 + 0x20 + lVar35) = fVar49;
          *(undefined8 *)(lVar22 + 0x24 + lVar35) = 0x3f8000003f800000;
          *(undefined4 *)(lVar22 + 0x2c + lVar35) = 0x3f800000;
          *(undefined4 *)(lVar22 + 0x38 + lVar35) = 0;
          *(undefined8 *)(lVar22 + 0x40 + lVar35) = 0x2cae82919f90dd16;
          *(undefined1 *)(lVar22 + 0x48 + lVar35) = 3;
          *(undefined1 *)(lVar22 + 0x58 + lVar35) = 0;
          *(undefined8 *)(lVar22 + 0x68 + lVar35) = uVar3;
          *(code **)(lVar22 + 0x78 + lVar35) = bevy_ecs::error::handler::panic;
          plVar33[2] = lVar35 + 0x88;
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          plVar33 = local_140;
          lVar37 = local_140[2];
          if (lVar37 == *local_140) {
            alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
          }
          lVar35 = plVar33[1];
          *(ulonglong *)(lVar35 + lVar37 * 0x18) = CONCAT44(fVar52,fVar50);
          *(float *)(lVar35 + 8 + lVar37 * 0x18) = fVar49;
          *(ulonglong *)(lVar35 + 0xc + lVar37 * 0x18) =
               CONCAT44(extraout_XMM0_Da_03 + fVar51,extraout_XMM0_Da_02 + fVar42);
          *(float *)(lVar35 + 0x14 + lVar37 * 0x18) = extraout_XMM0_Da_04 + fVar17;
          plVar33[2] = lVar37 + 1;
          lVar37 = plVar33[5];
          if (lVar37 == plVar33[3]) {
            alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>()
            ;
          }
          lVar35 = local_140[4];
          lVar22 = lVar37 * 0x38;
          *(undefined4 *)(lVar35 + lVar22) = 0;
          *(undefined8 *)(lVar35 + 0x10 + lVar22) = uVar3;
          uVar3 = anon_ec2ae470d878d07d53d061983e87f5cc_9_llvm_8659414576431420266;
          *(undefined8 *)(lVar35 + 0x18 + lVar22) =
               anon_ec2ae470d878d07d53d061983e87f5cc_9_llvm_8659414576431420266;
          uVar18 = DAT_142f7d770;
          *(undefined4 *)(lVar35 + 0x20 + lVar22) = DAT_142f7d770;
          *(undefined8 *)(lVar35 + 0x24 + lVar22) = uVar3;
          *(undefined4 *)(lVar35 + 0x2c + lVar22) = uVar18;
          *(undefined4 *)(lVar35 + 0x30 + lVar22) = 0;
          local_140[5] = lVar37 + 1;
          local_140[6] = local_140[6] + 1;
          puVar32 = local_1b0;
        }
      }
      lVar27 = lVar27 + 1;
    } while (lVar27 != 0x14);
  }
  plVar33 = local_140;
  local_1b8 = local_140[2];
  auVar11._8_8_ = 0;
  auVar11._0_8_ = local_1b8;
  uVar21 = SUB168(auVar11 * ZEXT816(0x18),0);
  if ((SUB168(auVar11 * ZEXT816(0x18),8) != 0) || (0x7ffffffffffffffc < uVar21)) {
LAB_140f92794:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  if (uVar21 == 0) {
    local_198 = 4;
  }
  else {
    plVar36 = (longlong *)local_140[1];
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == 0) goto LAB_140f92794;
    local_198 = extraout_RAX_00;
    if (local_1b8 != 0) {
      uVar19 = 0;
      uVar24 = local_1b8;
      do {
        if (uVar21 == uVar19) break;
        uStack_128._4_4_ = *(undefined4 *)((longlong)plVar36 + 0x14);
        uStack_12c = (undefined4)*(undefined8 *)((longlong)plVar36 + 0xc);
        uStack_128._0_4_ = (undefined4)((ulonglong)*(undefined8 *)((longlong)plVar36 + 0xc) >> 0x20)
        ;
        fStack_130 = *(float *)(plVar36 + 1);
        lVar26 = plVar36[1];
        local_138 = (undefined8 *)*plVar36;
        plVar36 = plVar36 + 3;
        *(ulonglong *)(extraout_RAX_00 + 0x10 + uVar19) =
             CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
        *(longlong *)(extraout_RAX_00 + uVar19) = (longlong)local_138;
        ((longlong *)(extraout_RAX_00 + uVar19))[1] = lVar26;
        uVar19 = uVar19 + 0x18;
        uVar24 = uVar24 - 1;
      } while (uVar24 != 0);
    }
  }
  fVar42 = __real_3f8f5c29;
  fVar17 = __real_3e4ccccd;
  fVar16 = __real_3f800000;
  local_190 = (undefined8 *)0x0;
  pcStack_188 = (code *)0x8;
  local_180 = (undefined8 *)0x0;
  uVar5 = *local_168;
  lVar26 = 0;
  if (uVar5 - 6 < 0xf) {
    lVar26 = (ulonglong)uVar5 - 5;
  }
  fVar51 = 0.0;
  switch(lVar26) {
  case 0:
    if (uVar5 != 0) break;
  case 1:
  case 8:
  case 0xb:
  case 0xd:
    fVar51 = __real_40000000;
    break;
  case 2:
  case 3:
    fVar51 = *(float *)*in_stack_00000080 + __real_40133333;
    break;
  case 4:
  case 5:
  case 10:
  case 0xc:
    fVar51 = ___real_407eb852;
  }
  local_1c0 = plVar33[1];
  local_230 = plVar33[4];
  local_1f8 = plVar33[2];
  if ((ulonglong)plVar33[5] < (ulonglong)plVar33[2]) {
    local_1f8 = plVar33[5];
  }
  local_238 = *in_stack_00000078 + 0x58;
  local_170 = *in_stack_00000030;
  local_228 = *in_stack_00000068;
  local_1a8 = *local_150;
  local_1f0 = local_150[1];
  local_1e8 = *in_stack_00000040;
  local_168 = (uint *)0x0;
  plVar33 = (longlong *)0x0;
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
      lVar26 = in_stack_00000058[1];
      uVar21 = *puVar6;
      uVar19 = 0;
      uVar24 = uVar21 - *(ulonglong *)(lVar26 + 0x18);
      if (uVar21 < *(ulonglong *)(lVar26 + 0x18)) {
        uVar24 = uVar19;
      }
      uVar23 = uVar21 - *(ulonglong *)(lVar26 + 0x38);
      if (uVar21 < *(ulonglong *)(lVar26 + 0x38)) {
        uVar23 = uVar19;
      }
      uVar21 = *(ulonglong *)(lVar26 + 0x10);
      uVar20 = *(ulonglong *)(lVar26 + 0x30);
      uVar29 = uVar20 - uVar23;
      if (uVar20 < uVar23) {
        uVar29 = uVar19;
      }
      uVar28 = uVar21 - uVar24;
      if (uVar21 < uVar24) {
        uVar28 = uVar19;
      }
      uVar19 = *(ulonglong *)(lVar26 + 0x40);
      *puVar6 = (uVar19 - uVar29) - uVar28;
      if (uVar21 >= uVar24 && uVar21 - uVar24 != 0 || uVar20 >= uVar23 && uVar20 - uVar23 != 0) {
        *puVar6 = uVar19;
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_02 == 0) {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        if ((*(uint *)(extraout_RAX_02 + 600) < 2) &&
           (*(float *)(extraout_RAX_02 + 0x280) <= __real_3e99999a &&
            __real_3e99999a != *(float *)(extraout_RAX_02 + 0x280))) {
          country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
          country_core::systems::wall::valid_enclosure::ShapeParameters::expand();
          fVar17 = __real_41000000;
          lVar26 = local_140[1];
          uVar21 = local_140[2];
          if ((ulonglong)local_140[5] < (ulonglong)local_140[2]) {
            uVar21 = local_140[5];
          }
          uVar24 = 0;
          while (uVar19 = uVar24, uVar19 < uVar21) {
            fVar42 = *(float *)(lVar26 + uVar19 * 0x18);
            country_core::resources::glade_border::GladeBorder::is_pos_inside();
            uVar24 = uVar19 + 1;
            if (extraout_AL_01 != '\0') {
              pfVar31 = (float *)(lVar26 + uVar19 * 0x18);
              fVar51 = *pfVar31;
              fVar41 = pfVar31[2];
              country_core::systems::wall::valid_enclosure::ShapeParameters::center();
              local_160 = CONCAT44(fVar41 - fVar42,fVar51 - extraout_XMM0_Da_10);
              ::utils::swizzlers::impl$0::x0y();
              *(ulonglong *)(pfVar31 + 3) =
                   CONCAT44((float)((ulonglong)local_108 >> 0x20) * _UNK_142badbc4,
                            (float)local_108 * ___xmm_00000000000000004100000041000000);
              pfVar31[5] = local_100._0_4_ * fVar17;
            }
          }
        }
      }
      fVar40 = _UNK_1429a90a8;
      fVar39 = _UNK_1429a90a4;
      fVar52 = ___xmm_000000008000000080000000bf800000;
      fVar50 = _UNK_1429a9098;
      fVar49 = _UNK_1429a9094;
      fVar41 = ___xmm_0000000080000000bf80000080000000;
      fVar51 = _UNK_1429a9088;
      fVar42 = _UNK_1429a9084;
      fVar17 = ___xmm_00000000bf8000008000000080000000;
      local_168 = (uint *)local_140[1];
      lVar26 = local_140[4];
      uVar21 = local_140[2];
      if ((ulonglong)local_140[5] < (ulonglong)local_140[2]) {
        uVar21 = local_140[5];
      }
      local_148 = (float *)*local_220;
      lVar27 = local_220[1];
      local_1c0 = CONCAT44(local_1c0._4_4_,*(undefined4 *)((longlong)local_220 + 0x14));
      uVar24 = 0;
      pfVar31 = local_148;
      while (uVar19 = uVar24, uVar19 < uVar21) {
        uVar24 = uVar19 + 1;
        uVar23 = *(ulonglong *)(lVar26 + 0x10 + uVar19 * 0x38);
        uVar20 = uVar23 & 0xffffffff;
        if ((uVar20 < *(ulonglong *)(lVar27 + 0x10)) &&
           (*(int *)(*(longlong *)(lVar27 + 8) + uVar20 * 0x14) == (int)(uVar23 >> 0x20))) {
          lVar37 = *(longlong *)(lVar27 + 8) + uVar20 * 0x14;
          uVar23 = (ulonglong)*(uint *)(lVar37 + 4);
          if ((uVar23 != 0xffffffff) &&
             ((uVar23 < *(ulonglong *)((longlong)pfVar31 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)((longlong)pfVar31 + 0x28) +
                 (ulonglong)(*(uint *)(lVar37 + 4) >> 6) * 8) >> (uVar23 & 0x3f) & 1) != 0)))) {
            plVar33 = (longlong *)((longlong)local_168 + uVar19 * 0x18);
            local_f8 = (float *)(ulonglong)*(uint *)(lVar37 + 0x10);
            lVar35 = *(longlong *)
                      (*(longlong *)(lVar27 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar37 + 0xc) * 0x48);
            lVar22 = ~*(ulonglong *)
                       (*(longlong *)
                         (*(longlong *)(lVar27 + 0x1a8) + 0x38 +
                         (ulonglong)*(uint *)(lVar37 + 0xc) * 0x48) +
                       *(longlong *)((longlong)pfVar31 + 0x110) * 8) * 0x30;
            lVar37 = *(longlong *)(lVar35 + 0x10 + lVar22);
            local_150 = *(longlong **)(lVar35 + 0x28 + lVar22);
            lVar35 = (longlong)local_f8 * 0x30;
            pfVar31 = (float *)(lVar37 + lVar35);
            fVar38 = *pfVar31;
            fVar43 = pfVar31[1];
            fVar44 = pfVar31[2];
            fVar48 = pfVar31[3];
            fVar45 = fVar48 * fVar48;
            fVar46 = fVar44 * fVar44 + fVar43 * fVar43 + fVar38 * fVar38;
            fVar47 = fVar44 * fVar51 + fVar43 * fVar42 + fVar38 * fVar17;
            fVar47 = fVar47 + fVar47;
            fVar48 = fVar48 + fVar48;
            local_210 = fVar48 * (fVar38 * fVar40 - fVar43 * fVar50) +
                        fVar47 * fVar44 + (fVar45 - fVar46) * fVar51;
            local_218 = CONCAT44(fVar48 * (fVar44 * fVar39 - fVar38 * fVar49) +
                                 fVar47 * fVar43 + (fVar45 - fVar46) * fVar42,
                                 fVar48 * (fVar43 * fVar52 - fVar44 * fVar41) +
                                 fVar47 * fVar38 + (fVar45 - fVar46) * fVar17);
            local_108 = CONCAT44(*(undefined4 *)((longlong)plVar33 + 0x14),
                                 *(undefined4 *)((longlong)plVar33 + 0xc));
            ::utils::swizzlers::impl$0::x0y();
            fVar38 = SUB84(local_138,0);
            fVar43 = (float)((ulonglong)local_138 >> 0x20);
            fVar44 = fVar16 / SQRT(fStack_130 * fStack_130 + fVar43 * fVar43 + fVar38 * fVar38);
            if ((int)fVar44 - 1U < 0x7fffff ||
                (int)ABS(fVar44) - 0x800000U < 0x7f000000 && -1 < (int)fVar44) {
              local_1d0 = fStack_130 * fVar44;
              local_1d8 = CONCAT44(fVar43 * fVar44,fVar38 * fVar44);
            }
            else {
              local_1d0 = 0.0;
              local_1d8 = 0;
            }
            plVar36 = (longlong *)(lVar37 + lVar35);
            ::utils::time_continuous_lerp_vec3();
            dolly::util::
            look_at<dolly::handedness::RightHanded,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>
                      ();
            *(undefined4 *)((longlong)local_150 + (longlong)local_f8 * 4) = (undefined4)local_1c0;
            *plVar36 = (longlong)local_138;
            plVar36[1] = CONCAT44(uStack_12c,fStack_130);
            fStack_130 = *(float *)(plVar33 + 1);
            local_138 = (undefined8 *)*plVar33;
            *(undefined4 *)((longlong)local_150 + (longlong)local_f8 * 4) = (undefined4)local_1c0;
            *(float *)(plVar36 + 3) = fStack_130;
            plVar36[2] = (longlong)local_138;
            pfVar31 = local_148;
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
    local_150 = (longlong *)((longlong)plVar33 + 1);
    if (__real_3f000000 < extraout_XMM0_Da_05) break;
    if (local_180 == local_190) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    *(longlong **)((longlong)pcStack_188 + (longlong)puVar1 * 8) = plVar33;
    local_180 = (undefined8 *)((longlong)puVar1 + 1);
    plVar33 = local_150;
  }
  local_f8 = (float *)(local_1c0 + (longlong)puVar2 * 0x18);
  fVar41 = *local_f8;
  fVar49 = local_f8[2];
  country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
  ppuVar25 = (undefined **)((longlong)local_1b0 * 0x38 + local_230);
  local_138 = &local_208;
  boids::Boid::frame_update<system_critters::fish::fish::closure_env_0>();
  puVar1 = local_180;
  if (extraout_AL != '\0') {
    if (local_180 == local_190) {
      ppuVar25 = &PTR_anon_d50c582ad1069d4dab52714d73220c32_9_llvm_7307022832566353767_142badd40;
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    *(longlong **)((longlong)pcStack_188 + (longlong)puVar1 * 8) = plVar33;
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
  fVar52 = extraout_XMM0_Da_06 + extraout_XMM0_Da_07 + extraout_XMM0_Da_08 + extraout_XMM0_Da_09;
  fVar50 = 0.0;
  if (0.0 <= fVar52) {
    fVar50 = fVar52;
  }
  fVar52 = fVar16;
  if (fVar50 <= fVar16) {
    fVar52 = fVar50;
  }
  if ((fVar52 < __real_3dcccccd) &&
     (country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query(),
     ppuVar25 != (undefined **)0x0)) {
    lVar26 = 0;
    do {
      pfVar31 = (float *)(extraout_RAX_01 + 0x20 + lVar26);
      if ((*pfVar31 <= fVar17 && fVar17 != *pfVar31) &&
         (pfVar31 = (float *)(extraout_RAX_01 + 0x24 + lVar26),
         *pfVar31 <= fVar17 && fVar17 != *pfVar31)) {
        fVar40 = (float)*(undefined8 *)(extraout_RAX_01 + lVar26);
        fVar38 = (float)((ulonglong)*(undefined8 *)(extraout_RAX_01 + lVar26) >> 0x20);
        uVar3 = *(undefined8 *)(extraout_RAX_01 + 8 + lVar26);
        fVar52 = (float)uVar3;
        fVar39 = (float)((ulonglong)uVar3 >> 0x20);
        fVar43 = fVar52 - fVar40;
        fVar44 = fVar39 - fVar38;
        fVar50 = fVar44 * fVar44 + fVar43 * fVar43;
        if (fVar50 <= __real_1e3ce508) {
          fVar50 = __real_1e3ce508;
        }
        fVar47 = ((fVar49 - fVar38) * fVar44 + (fVar41 - fVar40) * fVar43) / fVar50;
        fVar48 = 0.0;
        if (0.0 <= fVar47) {
          fVar48 = fVar47;
        }
        fVar47 = fVar16;
        if (fVar48 <= fVar16) {
          fVar47 = fVar48;
        }
        fVar48 = fVar41 - (fVar47 * fVar52 + (fVar16 - fVar47) * fVar40);
        fVar47 = fVar49 - (fVar47 * fVar39 + (fVar16 - fVar47) * fVar38);
        if (SQRT(fVar47 * fVar47 + fVar48 * fVar48) < fVar42) {
          fVar50 = (fVar44 * (local_f8[2] - fVar38) + fVar43 * (*local_f8 - fVar40)) / fVar50;
          fVar41 = 0.0;
          if (0.0 <= fVar50) {
            fVar41 = fVar50;
          }
          fVar49 = fVar16;
          if (fVar41 <= fVar16) {
            fVar49 = fVar41;
          }
          fVar50 = *local_f8 - (fVar52 * fVar49 + (fVar16 - fVar49) * fVar40);
          fVar49 = local_f8[2] - (fVar39 * fVar49 + (fVar16 - fVar49) * fVar38);
          fVar41 = fVar16 / SQRT(fVar49 * fVar49 + fVar50 * fVar50);
          local_138 = (undefined8 *)CONCAT44(fVar41 * fVar49,fVar41 * fVar50);
          ::utils::swizzlers::impl$0::x0y();
          local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_f8[5]);
          local_108 = *(undefined8 *)(local_f8 + 3);
          fVar41 = (float)*(longlong *)(local_f8 + 4);
          fVar49 = (float)((ulonglong)*(longlong *)(local_f8 + 4) >> 0x20);
          fVar41 = SQRT(fVar49 * fVar49 + local_f8[3] * local_f8[3] + fVar41 * fVar41);
          fStack_130 = fVar41 * local_248;
          local_138 = (undefined8 *)
                      CONCAT44((float)((ulonglong)local_250 >> 0x20) * fVar41,
                               (float)local_250 * fVar41);
          ::utils::time_continuous_lerp_vec3();
          local_f8[5] = local_158._0_4_;
          *(undefined8 *)(local_f8 + 3) = local_160;
          break;
        }
      }
      lVar26 = lVar26 + 0x38;
    } while ((longlong)ppuVar25 * 0x38 - lVar26 != 0);
  }
  local_108 = CONCAT44(local_f8[2],*local_f8);
  ::utils::swizzlers::impl$1::xvy();
  local_f8[2] = fStack_130;
  *(undefined8 **)local_f8 = local_138;
  puVar34 = *(uint **)(local_1a8 + 0xf8);
  puVar2 = puVar34 + *(longlong *)(local_1a8 + 0x100);
  cVar4 = *(char *)(local_1a8 + 0x108);
  local_148 = local_f8 + 3;
  lVar37 = 8;
  lVar26 = 0;
  uVar21 = 0;
  uVar24 = 0;
  lVar27 = 0;
  lVar35 = local_1f0;
  do {
    if (cVar4 == '\0') {
      if (uVar24 == uVar21) {
        do {
          if (puVar34 == puVar2) goto LAB_140f91d50;
          uVar5 = *puVar34;
          puVar34 = puVar34 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar35 + 0x100) + 0x58 + (ulonglong)uVar5 * 0xa0);
        } while (uVar21 == 0);
        lVar22 = *(longlong *)(lVar35 + 0x100) + (ulonglong)uVar5 * 0xa0;
        uVar24 = (ulonglong)*(uint *)(lVar22 + 0x94);
        lVar26 = *(longlong *)(lVar35 + 0x1a8);
        lVar27 = lVar26 + uVar24 * 0x48;
        lVar37 = *(longlong *)(lVar26 + 0x38 + uVar24 * 0x48);
        uVar24 = *(ulonglong *)(lVar26 + 0x40 + uVar24 * 0x48);
        if ((*(ulonglong *)(local_1a8 + 0x118) < uVar24) &&
           (uVar19 = *(ulonglong *)(lVar37 + *(ulonglong *)(local_1a8 + 0x118) * 8), uVar19 != 0)) {
          lVar26 = *(longlong *)(*(longlong *)(lVar27 + 0x18) + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar24 <= uVar19) goto LAB_140f91bb1;
LAB_140f91b8a:
          uVar24 = *(ulonglong *)(lVar37 + uVar19 * 8);
          if (uVar24 == 0) goto LAB_140f91bb1;
          lVar27 = *(longlong *)(*(longlong *)(lVar27 + 0x18) + 0x10 + ~uVar24 * 0x30);
        }
        else {
          lVar26 = 0;
          uVar19 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar19 < uVar24) goto LAB_140f91b8a;
LAB_140f91bb1:
          lVar27 = 0;
        }
        lVar37 = *(longlong *)(lVar22 + 0x50);
        uVar24 = 0;
      }
      uVar19 = (ulonglong)*(uint *)(lVar37 + 8 + uVar24 * 0x10);
    }
    else {
      if (uVar24 == uVar21) {
        do {
          if (puVar34 == puVar2) goto LAB_140f91d50;
          uVar5 = *puVar34;
          puVar34 = puVar34 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar35 + 0x1a8) + 0x10 + (ulonglong)uVar5 * 0x48);
        } while (uVar21 == 0);
        lVar27 = *(longlong *)(lVar35 + 0x1a8) + (ulonglong)uVar5 * 0x48;
        uVar24 = *(ulonglong *)(lVar27 + 0x40);
        if ((*(ulonglong *)(local_1a8 + 0x118) < uVar24) &&
           (uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar27 + 0x38) + *(ulonglong *)(local_1a8 + 0x118) * 8),
           uVar19 != 0)) {
          lVar26 = *(longlong *)(*(longlong *)(lVar27 + 0x18) + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar24 <= uVar19) goto LAB_140f91ba9;
LAB_140f91b56:
          uVar24 = *(ulonglong *)(*(longlong *)(lVar27 + 0x38) + uVar19 * 8);
          if (uVar24 == 0) goto LAB_140f91ba9;
          lVar27 = *(longlong *)(*(longlong *)(lVar27 + 0x18) + 0x10 + ~uVar24 * 0x30);
        }
        else {
          lVar26 = 0;
          uVar19 = *(ulonglong *)(local_1a8 + 0x120);
          if (uVar19 < uVar24) goto LAB_140f91b56;
LAB_140f91ba9:
          lVar27 = 0;
        }
        uVar24 = 0;
      }
      uVar19 = uVar24 & 0xffffffff;
    }
    lVar22 = uVar19 * 0x30 + lVar26;
    uVar24 = uVar24 + 1;
    if (*(char *)(uVar19 + lVar27) == '\x02') {
      fVar41 = *(float *)(lVar22 + 0x10);
      fVar49 = *(float *)(lVar22 + 0x18);
      fVar50 = fVar41 - *local_f8;
      fVar52 = fVar49 - local_f8[2];
      if (SQRT(fVar52 * fVar52 + fVar50 * fVar50) < __real_40000000) {
        fVar41 = *local_f8 - fVar41;
        fVar49 = local_f8[2] - fVar49;
        fVar50 = fVar16 / SQRT(fVar49 * fVar49 + fVar41 * fVar41);
        local_138 = (undefined8 *)CONCAT44(fVar50 * fVar49,fVar50 * fVar41);
        local_240 = lVar27;
        ::utils::swizzlers::impl$0::x0y();
        local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_148[2]);
        local_108 = *(undefined8 *)local_148;
        fStack_130 = local_258 * __real_40a00000;
        local_138 = (undefined8 *)
                    CONCAT44((float)((ulonglong)local_260 >> 0x20) *
                             __xmm_000000000000000040a0000040a00000._4_4_,
                             (float)local_260 * (float)__xmm_000000000000000040a0000040a00000);
        ::utils::time_continuous_lerp_vec3();
        local_148[2] = local_158._0_4_;
        *(undefined8 *)local_148 = local_160;
        lVar27 = local_240;
        lVar35 = local_1f0;
      }
    }
  } while( true );
LAB_140f91d50:
  country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
  fVar41 = *local_f8;
  fVar49 = local_f8[2];
  fVar50 = local_138._0_4_ - fVar41;
  fVar52 = fStack_130 - fVar49;
  if (SQRT(fVar52 * fVar52 + fVar50 * fVar50) < fVar51) {
    country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
    fVar41 = fVar41 - local_138._0_4_;
    fVar49 = fVar49 - fStack_130;
    fVar50 = fVar16 / SQRT(fVar49 * fVar49 + fVar41 * fVar41);
    local_138 = (undefined8 *)CONCAT44(fVar50 * fVar49,fVar50 * fVar41);
    ::utils::swizzlers::impl$0::x0y();
    local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_148[2]);
    local_108 = *(undefined8 *)local_148;
    fStack_130 = local_268 * __real_41000000;
    local_138 = (undefined8 *)
                CONCAT44((float)((ulonglong)local_270 >> 0x20) * _UNK_142badbc4,
                         (float)local_270 * ___xmm_00000000000000004100000041000000);
    ::utils::time_continuous_lerp_vec3();
    local_148[2] = local_158._0_4_;
    *(undefined8 *)local_148 = local_160;
  }
  country_core::systems::camera_rig::MainCamera::mode();
  plVar33 = local_150;
  if (extraout_AL_00 == '\x03') {
    country_core::systems::camera_rig::MainCamera::eye_position();
    fVar41 = *local_f8;
    fVar49 = local_f8[2];
    fVar50 = local_138._0_4_ - fVar41;
    fVar52 = fStack_130 - fVar49;
    plVar33 = local_150;
    if (SQRT(fVar52 * fVar52 + fVar50 * fVar50) < __real_40000000) {
      country_core::systems::camera_rig::MainCamera::eye_position();
      fVar41 = fVar41 - local_138._0_4_;
      fVar49 = fVar49 - fStack_130;
      fVar50 = fVar16 / SQRT(fVar49 * fVar49 + fVar41 * fVar41);
      local_138 = (undefined8 *)CONCAT44(fVar50 * fVar49,fVar50 * fVar41);
      ::utils::swizzlers::impl$0::x0y();
      local_100 = (ulonglong *)CONCAT44(local_100._4_4_,local_148[2]);
      local_108 = *(undefined8 *)local_148;
      fStack_130 = local_278 * __real_41000000;
      local_138 = (undefined8 *)
                  CONCAT44((float)(local_288[1] >> 0x20) * _UNK_142badbc4,
                           (float)local_288[1] * ___xmm_00000000000000004100000041000000);
      ::utils::time_continuous_lerp_vec3();
      local_148[2] = local_158._0_4_;
      *(undefined8 *)local_148 = local_160;
      plVar33 = local_150;
    }
  }
  goto LAB_140f9150c;
}

