// name: wooden_floor_borders
// addr: 0x141244310
// size: 3135
// signature: void __cdecl wooden_floor_borders(void)


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  undefined1 auVar3 [16];
  longlong lVar4;
  longlong lVar5;
  uint uVar6;
  uint uVar7;
  undefined1 auVar8 [12];
  undefined8 uVar9;
  float fVar10;
  float fVar11;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  float *extraout_RAX_02;
  byte *extraout_RAX_03;
  longlong *in_RCX;
  longlong lVar12;
  float *pfVar13;
  ulonglong uVar14;
  longlong *in_R9;
  longlong lVar15;
  uint uVar16;
  ushort uVar17;
  float *pfVar18;
  float *pfVar19;
  longlong *plVar20;
  byte bVar21;
  char cVar22;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  char cVar24;
  char cVar25;
  undefined1 auVar23 [16];
  char cVar26;
  undefined1 extraout_XMM0 [16];
  float fVar27;
  float fVar28;
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  longlong local_2e0;
  longlong lStack_2d8;
  undefined1 local_2d0 [16];
  longlong local_288;
  longlong lStack_280;
  longlong local_258;
  float fStack_250;
  undefined4 uStack_24c;
  undefined7 uStack_248;
  undefined1 uStack_241;
  float fStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined7 uStack_230;
  undefined1 local_229;
  undefined7 uStack_228;
  undefined1 uStack_221;
  undefined7 uStack_220;
  undefined1 local_219;
  undefined1 uStack_218;
  undefined6 uStack_217;
  undefined8 uStack_211;
  longlong local_1c8;
  undefined1 local_1c0;
  float local_1b8;
  float fStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  longlong *local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  longlong local_168;
  longlong local_160;
  float *local_158;
  int local_150;
  float fStack_14c;
  float fStack_148;
  float fStack_144;
  undefined8 uStack_140;
  float fStack_138;
  undefined4 local_130;
  float fStack_12c;
  float fStack_128;
  float fStack_124;
  undefined8 uStack_120;
  float *local_118;
  float *local_108;
  longlong local_100;
  longlong local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  lVar4 = *in_RCX;
  country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  if (extraout_RAX >> 0x20 == 0) {
    return;
  }
  lVar15 = in_stack_00000028[1];
  uVar14 = extraout_RAX & 0xffffffff;
  if (*(ulonglong *)(lVar15 + 0x10) <= uVar14) {
    return;
  }
  if (*(int *)(*(longlong *)(lVar15 + 8) + uVar14 * 0x14) != (int)(extraout_RAX >> 0x20)) {
    return;
  }
  lVar12 = *(longlong *)(lVar15 + 8) + uVar14 * 0x14;
  uVar14 = (ulonglong)*(uint *)(lVar12 + 4);
  if (uVar14 == 0xffffffff) {
    return;
  }
  lVar5 = *in_stack_00000028;
  if (*(ulonglong *)(lVar5 + 0x38) <= uVar14) {
    return;
  }
  if ((*(ulonglong *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)(*(uint *)(lVar12 + 4) >> 6) * 8) >>
       (uVar14 & 0x3f) & 1) == 0) {
    return;
  }
  lVar15 = *(longlong *)
            (*(longlong *)
              (*(longlong *)(lVar15 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48) +
             0x10 + ~*(ulonglong *)
                      (*(longlong *)
                        (*(longlong *)(lVar15 + 0x1a8) + 0x38 +
                        (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48) +
                      *(longlong *)(lVar5 + 0x110) * 8) * 0x30);
  lVar12 = (ulonglong)*(uint *)(lVar12 + 0x10) * 0x58;
  cVar22 = *(char *)(lVar15 + 0x40 + lVar12);
  if (cVar22 == '\0') {
    country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
    uVar14 = extraout_RAX_00;
  }
  else {
    if (cVar22 == '\x01') {
      return;
    }
    uVar14 = (ulonglong)*(byte *)(lVar15 + 0x41 + lVar12);
  }
  if ((uVar14 & 1) == 0) {
    return;
  }
  if (in_R9[3] == 0) {
    return;
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  local_118 = (float *)in_R9[1];
  bVar21 = (byte)(extraout_RAX_01 >> 0x39);
  auVar23 = ZEXT216(CONCAT11(bVar21,bVar21));
  auVar23 = pshuflw(auVar23,auVar23,0);
  lVar15 = 0;
  uVar14 = extraout_RAX_01;
  do {
    uVar14 = (ulonglong)local_118 & uVar14;
    auVar3 = *(undefined1 (*) [16])(*in_R9 + uVar14);
    cVar22 = auVar23[0];
    auVar30[0] = -(auVar3[0] == cVar22);
    cVar24 = auVar23[1];
    auVar30[1] = -(auVar3[1] == cVar24);
    cVar25 = auVar23[2];
    auVar30[2] = -(auVar3[2] == cVar25);
    cVar26 = auVar23[3];
    auVar30[3] = -(auVar3[3] == cVar26);
    auVar30[4] = -(auVar3[4] == cVar22);
    auVar30[5] = -(auVar3[5] == cVar24);
    auVar30[6] = -(auVar3[6] == cVar25);
    auVar30[7] = -(auVar3[7] == cVar26);
    auVar30[8] = -(auVar3[8] == cVar22);
    auVar30[9] = -(auVar3[9] == cVar24);
    auVar30[10] = -(auVar3[10] == cVar25);
    auVar30[0xb] = -(auVar3[0xb] == cVar26);
    auVar30[0xc] = -(auVar3[0xc] == cVar22);
    auVar30[0xd] = -(auVar3[0xd] == cVar24);
    auVar30[0xe] = -(auVar3[0xe] == cVar25);
    auVar30[0xf] = -(auVar3[0xf] == cVar26);
    uVar17 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe | (ushort)(auVar30[0xf] >> 7) << 0xf;
    uVar16 = (uint)uVar17;
    while (uVar17 != 0) {
      uVar6 = 0;
      for (uVar7 = uVar16; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
        uVar6 = uVar6 + 1;
      }
      if (lVar4 == *(longlong *)(*in_R9 + -0x68 + (uVar6 + uVar14 & (ulonglong)local_118) * -0x68))
      {
        country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized();
        country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates();
        pfVar13 = local_118;
        pfVar19 = extraout_RAX_02;
        goto LAB_141244573;
      }
      uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
      uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
    }
    auVar29[0] = -(auVar3[0] == -1);
    auVar29[1] = -(auVar3[1] == -1);
    auVar29[2] = -(auVar3[2] == -1);
    auVar29[3] = -(auVar3[3] == -1);
    auVar29[4] = -(auVar3[4] == -1);
    auVar29[5] = -(auVar3[5] == -1);
    auVar29[6] = -(auVar3[6] == -1);
    auVar29[7] = -(auVar3[7] == -1);
    auVar29[8] = -(auVar3[8] == -1);
    auVar29[9] = -(auVar3[9] == -1);
    auVar29[10] = -(auVar3[10] == -1);
    auVar29[0xb] = -(auVar3[0xb] == -1);
    auVar29[0xc] = -(auVar3[0xc] == -1);
    auVar29[0xd] = -(auVar3[0xd] == -1);
    auVar29[0xe] = -(auVar3[0xe] == -1);
    auVar29[0xf] = -(auVar3[0xf] == -1);
    if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x17,0) & 1) != 0) || (SUB161(auVar29 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x27,0) & 1) != 0) || (SUB161(auVar29 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x37,0) & 1) != 0) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') {
      return;
    }
    uVar14 = uVar14 + lVar15 + 0x10;
    lVar15 = lVar15 + 0x10;
  } while( true );
LAB_141244573:
  while( true ) {
    do {
      pfVar18 = pfVar19;
      if (pfVar18 == pfVar13) {
                    // WARNING: Read-only address (ram,0x0001429258f0) is written
        return;
      }
      pfVar19 = pfVar18 + 5;
    } while ((*(char *)(pfVar18 + 4) != '\x04') && (*(char *)(pfVar18 + 4) != '\x01'));
    country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
    if ((*extraout_RAX_03 & 1) != 0) break;
    local_108 = pfVar19;
    country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length();
    ::utils::circular_f32::CircularF32::as_f32();
    ::utils::swizzlers::impl$1::xvy();
    country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz();
    country_core::resources::walls::decorator::DecoratorRank::new();
    country_core::components::drawable::impl$1::from();
    fStack_240 = 0.0;
    uStack_248 = 0;
    uStack_241 = 0;
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    local_258 = 0;
    fStack_250 = 0.0;
    uStack_24c = 1.0;
    uStack_23c = 0x3f800000;
    uStack_238 = 0x3f800000;
    uStack_234 = 0x3f800000;
    uStack_218 = 0x18;
    local_1c0 = 0;
    local_1c8 = lVar4;
    bevy_ecs::system::commands::Commands::spawn_empty();
    uVar9 = local_198;
    plVar20 = (longlong *)CONCAT44(fStack_1ac,fStack_1b0);
    if ((longlong *)CONCAT44(fStack_1ac,fStack_1b0) == (longlong *)0x0) {
      plVar20 = local_1a8;
    }
    lVar15 = plVar20[2];
    if ((ulonglong)(*plVar20 - lVar15) < 0xd8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar12 = plVar20[1];
    *(code **)(lVar12 + lVar15) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::resources::walls::curved_halftimber::CurvedHalfTimber>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    memcpy((void *)(lVar12 + lVar15 + 8),&local_258,0xa0);
    *(undefined1 *)(lVar12 + 0xa8 + lVar15) = 0;
    *(undefined8 *)(lVar12 + 0xb8 + lVar15) = uVar9;
    *(code **)(lVar12 + 200 + lVar15) = bevy_ecs::error::handler::panic;
    plVar20[2] = lVar15 + 0xd8;
    lVar15 = in_stack_00000038[2];
    if (lVar15 == *in_stack_00000038) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    *(undefined8 *)(in_stack_00000038[1] + lVar15 * 8) = uVar9;
    in_stack_00000038[2] = lVar15 + 1;
    pfVar13 = local_118;
    pfVar19 = local_108;
  }
  ::utils::geometry::aabb::Aabb2::width();
  ::utils::geometry::rectangle::Rectangle2d::perimeter();
  if (extraout_XMM0._0_4_ + 0.05 <= extraout_XMM0_Da) {
    fVar31 = *pfVar18;
    ::utils::geometry::rectangle::Rectangle2d::perimeter();
    if (NAN(fVar31 / extraout_XMM0_Da_00)) {
LAB_141244f39:
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar32 = pfVar18[2];
    ::utils::geometry::rectangle::Rectangle2d::perimeter();
    if (NAN(fVar32 / extraout_XMM0_Da_01)) goto LAB_141244f39;
    fmodf(fVar31 / extraout_XMM0_Da_00,1.0);
    fmodf(fVar32 / extraout_XMM0_Da_01,1.0);
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
    ::utils::curve::Curve::circular_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>
              ();
    if (local_258 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (CONCAT44(uStack_23c,fStack_240) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    ::utils::geometry::rectangle::Rectangle2d::try_as_curve2();
  }
  if (!SBORROW8(0,local_2e0)) {
    local_108 = pfVar19;
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
    local_100 = local_258;
    local_f8 = CONCAT44(uStack_24c,fStack_250);
    lVar15 = CONCAT17(uStack_241,uStack_248);
    if (CONCAT44(uStack_23c,fStack_240) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_288 = local_2d0._0_8_;
    local_160 = lStack_2d8 + local_288 * 8;
    local_150 = 0;
    local_158 = pfVar18;
    if (local_288 != 0) {
      lVar15 = lVar15 << 3;
      local_168 = lStack_2d8;
      do {
        local_168 = local_168 + 8;
        ::utils::swizzlers::impl$1::xvy();
        if (local_150 == 1) {
          fStack_14c = (float)uStack_140;
          fStack_148 = (float)((ulonglong)uStack_140 >> 0x20);
          uStack_140 = CONCAT44(fStack_128,fStack_12c);
          fVar31 = fStack_124;
          fVar32 = fStack_12c;
          fVar33 = fStack_128;
          fVar34 = fStack_138;
        }
        else {
          fStack_1ac = fStack_124;
          fStack_1b4 = fStack_12c;
          fStack_1b0 = fStack_128;
          local_1b8 = 1.4013e-45;
          local_1a8 = &local_168;
          itertools::tuple_impl::impl$31::
          collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut_<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders::closure_env_5>_>_>_>
                    ();
          auVar8._4_7_ = uStack_248;
          auVar8._0_4_ = uStack_24c;
          auVar8[0xb] = uStack_241;
          uStack_140 = auVar8._4_8_;
          fStack_138 = fStack_240;
          local_150 = (int)local_258;
          fStack_14c = (float)((ulonglong)local_258 >> 0x20);
          fStack_148 = fStack_250;
          fStack_144 = uStack_24c;
          if (local_150 != 1) break;
          fVar32 = (float)uStack_248;
          fVar33 = auVar8._8_4_;
          fVar31 = fStack_240;
          fVar34 = uStack_24c;
        }
        fVar11 = fStack_148;
        fVar10 = fStack_14c;
        lVar12 = 0;
        do {
          if (lVar15 == lVar12) break;
          fVar27 = (float)*(undefined8 *)(local_f8 + lVar12) - fStack_14c;
          fVar28 = (float)((ulonglong)*(undefined8 *)(local_f8 + lVar12) >> 0x20) - fVar34;
          lVar12 = lVar12 + 8;
        } while (0.05 <= SQRT(fVar28 * fVar28 + fVar27 * fVar27));
        lVar12 = 0;
        do {
          if (lVar15 == lVar12) break;
          fVar27 = (float)*(undefined8 *)(local_f8 + lVar12) - fVar32;
          fVar28 = (float)((ulonglong)*(undefined8 *)(local_f8 + lVar12) >> 0x20) - fVar31;
          lVar12 = lVar12 + 8;
        } while (0.05 <= SQRT(fVar28 * fVar28 + fVar27 * fVar27));
        fStack_144 = fVar34;
        fStack_138 = fVar31;
        country_core::resources::walls::decorator::DecoratorRank::new();
        country_core::components::drawable::impl$1::from();
        local_178._0_4_ = local_1b8;
        local_178._4_4_ = fStack_1b4;
        uStack_170 = CONCAT44(fStack_1ac,fStack_1b0);
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        uVar9 = uStack_170;
        fVar28 = local_178._4_4_;
        fVar27 = (float)local_178;
        local_178 = CONCAT44((fVar11 + fVar33) * 0.5,(fVar10 + fVar32) * 0.5);
        uStack_170 = CONCAT44(uStack_170._4_4_,(fVar34 + fVar31) * 0.5);
        country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight();
        fStack_128 = fVar27;
        fStack_124 = fVar28;
        uStack_120 = uVar9;
        local_219 = (undefined1)local_188;
        uStack_218 = (undefined1)((ulonglong)local_188 >> 8);
        uStack_217 = (undefined6)((ulonglong)local_188 >> 0x10);
        uStack_211 = uStack_180;
        local_229 = (undefined1)local_198;
        uStack_228 = (undefined7)((ulonglong)local_198 >> 8);
        uStack_221 = (undefined1)uStack_190;
        uStack_220 = (undefined7)((ulonglong)uStack_190 >> 8);
        uStack_23c._3_1_ = SUB81(local_1a8,0);
        uStack_238 = (undefined4)((ulonglong)local_1a8 >> 8);
        uStack_234._0_3_ = (undefined3)((ulonglong)local_1a8 >> 0x28);
        uStack_234._3_1_ = (undefined1)uStack_1a0;
        uStack_230 = (undefined7)((ulonglong)uStack_1a0 >> 8);
        uStack_24c._3_1_ = SUB41(local_1b8,0);
        uStack_248 = (undefined7)(CONCAT44(fStack_1b4,local_1b8) >> 8);
        uStack_241 = SUB41(fStack_1b0,0);
        fStack_240 = (float)(CONCAT44(fStack_1ac,fStack_1b0) >> 8);
        uStack_23c._0_3_ = (undefined3)((uint)fStack_1ac >> 8);
        bevy_ecs::system::commands::Commands::spawn_empty();
        uVar9 = local_198;
        plVar20 = (longlong *)CONCAT44(fStack_1ac,fStack_1b0);
        if ((longlong *)CONCAT44(fStack_1ac,fStack_1b0) == (longlong *)0x0) {
          plVar20 = local_1a8;
        }
        lVar12 = plVar20[2];
        if ((ulonglong)(*plVar20 - lVar12) < 0xd8) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar5 = plVar20[1];
        *(code **)(lVar5 + lVar12) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::resources::walls::curved_halftimber::CurvedHalfTimber>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        puVar1 = (undefined8 *)(lVar5 + 8 + lVar12);
        *puVar1 = 0;
        puVar1[1] = 0x3f80000000000000;
        *(undefined8 *)(lVar5 + 0x18 + lVar12) = 0;
        *(undefined4 *)(lVar5 + 0x20 + lVar12) = 0;
        *(undefined8 *)(lVar5 + 0x24 + lVar12) = 0x3f8000003f800000;
        *(undefined4 *)(lVar5 + 0x2c + lVar12) = 0x3f800000;
        puVar1 = (undefined8 *)(lVar5 + 0x30 + lVar12);
        *puVar1 = CONCAT44(fStack_12c,local_130);
        puVar1[1] = CONCAT44(fStack_124,fStack_128);
        *(undefined8 *)(lVar5 + 0x40 + lVar12) = uStack_120;
        *(undefined1 *)(lVar5 + 0x48 + lVar12) = 0x18;
        auVar23[7] = uStack_241;
        auVar23._0_7_ = uStack_248;
        auVar23._8_4_ = fStack_240;
        auVar23._12_4_ = uStack_23c;
        auVar3._4_4_ = uStack_234;
        auVar3._0_4_ = uStack_238;
        auVar3._8_7_ = uStack_230;
        auVar3[0xf] = local_229;
        plVar2 = (longlong *)(lVar5 + 0x49 + lVar12);
        *plVar2 = local_258;
        plVar2[1] = CONCAT44(uStack_24c,fStack_250);
        *(undefined1 (*) [16])(lVar5 + 0x59 + lVar12) = auVar23;
        *(undefined1 (*) [16])(lVar5 + 0x69 + lVar12) = auVar3;
        puVar1 = (undefined8 *)(lVar5 + 0x79 + lVar12);
        *puVar1 = CONCAT17(uStack_221,uStack_228);
        puVar1[1] = CONCAT17(local_219,uStack_220);
        puVar1 = (undefined8 *)(lVar5 + 0x88 + lVar12);
        *puVar1 = CONCAT62(uStack_217,CONCAT11(uStack_218,local_219));
        puVar1[1] = uStack_211;
        *(longlong *)(lVar5 + 0x98 + lVar12) = lVar4;
        *(undefined1 *)(lVar5 + 0xa0 + lVar12) = 0;
        *(undefined1 *)(lVar5 + 0xa8 + lVar12) = 0;
        *(undefined8 *)(lVar5 + 0xb8 + lVar12) = uVar9;
        *(code **)(lVar5 + 200 + lVar12) = bevy_ecs::error::handler::panic;
        plVar20[2] = lVar12 + 0xd8;
        lVar12 = in_stack_00000038[2];
        if (lVar12 == *in_stack_00000038) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(undefined8 *)(in_stack_00000038[1] + lVar12 * 8) = uVar9;
        in_stack_00000038[2] = lVar12 + 1;
      } while (local_168 != local_160);
    }
    if (local_100 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    pfVar19 = local_108;
    pfVar13 = local_118;
    if (local_2e0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lStack_280 = local_2d0._8_8_;
    if (lStack_280 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  goto LAB_141244573;
}

