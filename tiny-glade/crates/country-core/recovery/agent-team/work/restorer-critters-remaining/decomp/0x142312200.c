
void __cdecl system_critters::duck::move_ducks_along_trajectories(void)

{
  byte *pbVar1;
  ulonglong uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  undefined4 *puVar13;
  undefined8 uVar14;
  float *pfVar15;
  bool bVar16;
  char extraout_AL;
  uint uVar17;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar18;
  undefined8 *puVar19;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  byte bVar20;
  longlong *in_RCX;
  longlong *plVar21;
  undefined8 *puVar22;
  ulonglong uVar23;
  longlong *in_RDX;
  ulonglong uVar24;
  longlong lVar25;
  longlong lVar26;
  undefined *puVar27;
  float *pfVar28;
  ulonglong *in_R9;
  longlong lVar29;
  undefined4 *puVar30;
  undefined4 *puVar31;
  undefined4 *puVar32;
  float *pfVar33;
  float *pfVar34;
  undefined4 *puVar35;
  float extraout_XMM0_Da;
  float fVar36;
  float extraout_XMM0_Da_00;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float unaff_XMM8_Db;
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar54;
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  float fVar55;
  longlong *in_stack_00000028;
  ulonglong uVar56;
  ulonglong local_3b8;
  ulonglong local_3b0;
  uint *local_398;
  uint *local_390;
  longlong local_388;
  longlong local_378;
  longlong local_368;
  longlong local_358;
  undefined4 local_34c;
  longlong local_348;
  longlong local_338;
  undefined4 local_32c;
  longlong local_328;
  longlong local_318;
  undefined4 local_30c;
  longlong local_308;
  longlong local_2f8;
  undefined4 local_2ec;
  longlong local_2e8;
  longlong local_2d8;
  undefined4 local_2cc;
  longlong local_2c8;
  longlong local_2b8;
  undefined4 local_2ac;
  longlong local_2a8;
  longlong local_298;
  undefined4 local_28c;
  ulonglong local_288;
  ulonglong local_280;
  byte local_278;
  longlong local_220;
  longlong local_218;
  undefined4 *local_1b8;
  undefined4 *local_1b0;
  longlong local_1a0;
  longlong local_198;
  longlong local_190;
  longlong local_188;
  longlong local_180;
  longlong local_178;
  longlong local_170;
  longlong local_168;
  longlong local_160;
  longlong local_158;
  longlong local_150;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  undefined4 *local_128;
  undefined4 *local_120;
  undefined4 *local_110;
  ulonglong local_108;
  byte *local_100;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar16 = false;
  }
  else {
    uVar24 = 0x34;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar24 != 0) {
        if (uVar24 < 0x34) {
          if ((char)(&DAT_14304a948)[uVar24] < -0x40) {
LAB_1423122c8:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar24 != 0x34) goto LAB_1423122c8;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar24 != 0xfffffffffffffffe)) {
        if (uVar24 + 2 < 0x34) {
          if ((char)(&UNK_14304a94a)[uVar24] < -0x40) {
LAB_142312326:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar24 + 2 != 0x34) goto LAB_142312326;
      }
    }
    puVar27 = &DAT_14304aa9b;
    do {
      bVar20 = puVar27[-1];
      uVar17 = (uint)(char)bVar20;
      if ((char)bVar20 < '\0') {
        bVar5 = puVar27[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar27[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar27 + -4;
            puVar27 = puVar27 + -4;
            uVar17 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar27 = puVar27 + -3;
            uVar17 = bVar6 & 0xf;
          }
          uVar17 = bVar5 & 0x3f | uVar17 << 6;
        }
        else {
          puVar27 = puVar27 + -2;
          uVar17 = bVar5 & 0x1f;
        }
        uVar17 = (uint)(bVar20 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_1423123f4;
LAB_142312405:
        if (puVar27 == &UNK_14304aa77) break;
        if (puVar27 + -0x14304aa77 < (undefined *)0x23) {
          if (-0x41 < (char)puVar27[1]) break;
        }
        else if (puVar27 + -0x14304aa77 == (undefined *)0x23) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar27 = puVar27 + -1;
      if (uVar17 == 0x5c) goto LAB_142312405;
LAB_1423123f4:
      if (uVar17 == 0x2f) goto LAB_142312405;
    } while (puVar27 != &DAT_14304aa78);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar21 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_1423137e4;
    if (*plVar21 != 0) goto LAB_1423137f0;
    *plVar21 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar21 = *plVar21 + 1;
    bVar16 = true;
  }
  lVar8 = *in_RDX;
  lVar9 = in_RDX[1];
  uVar56 = (ulonglong)*(uint *)((longlong)in_RDX + 0x14);
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<bevy_ecs::entity::Entity,ref_mut_<country_core::resources::critters::Duck>,ref_mut_<enum2_<country_core::resources::critters::DuckState>_>,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::Velocity>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush>_>,tuple_<>_>
            ();
  local_220 = local_388;
  local_218 = local_378;
  lVar10 = *in_RCX;
  uVar24 = *in_R9;
  uVar11 = in_R9[1];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar12 = *in_stack_00000028;
  puVar13 = (undefined4 *)in_stack_00000028[2];
  local_1a0 = local_368;
  local_198 = local_358;
  local_190 = local_348;
  local_188 = local_338;
  local_180 = local_328;
  local_178 = local_318;
  local_170 = local_308;
  local_168 = local_2f8;
  local_160 = local_2e8;
  local_158 = local_2d8;
  local_150 = local_2c8;
  local_148 = local_2b8;
  local_140 = local_2a8;
  local_138 = local_298;
  local_108 = local_288;
  do {
    while( true ) {
      do {
        while( true ) {
          if ((local_278 & 1) == 0) {
            if (local_280 == local_108) {
              do {
                if (local_398 == local_390) goto LAB_1423136fb;
                uVar17 = *local_398;
                local_398 = local_398 + 1;
              } while (*(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar17 * 0xa0)
                       == 0);
              lVar18 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar17 * 0xa0;
              lVar26 = *(longlong *)
                        (*(longlong *)(lVar9 + 0x1a8) + 0x18 +
                        (ulonglong)*(uint *)(lVar18 + 0x94) * 0x48);
              lVar25 = *(longlong *)
                        (*(longlong *)(lVar9 + 0x1a8) + 0x38 +
                        (ulonglong)*(uint *)(lVar18 + 0x94) * 0x48);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
              local_1a0 = *(longlong *)(lVar26 + 0x10 + lVar29);
              local_198 = *(longlong *)(lVar26 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
              local_190 = *(longlong *)(lVar26 + 0x10 + lVar29);
              local_188 = *(longlong *)(lVar26 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
              local_180 = *(longlong *)(lVar26 + 0x10 + lVar29);
              local_178 = *(longlong *)(lVar26 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x128) * 8) * 0x30;
              local_170 = *(longlong *)(lVar26 + 0x10 + lVar29);
              local_168 = *(longlong *)(lVar26 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x130) * 8) * 0x30;
              local_160 = *(longlong *)(lVar26 + 0x10 + lVar29);
              local_158 = *(longlong *)(lVar26 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x138) * 8) * 0x30;
              local_150 = *(longlong *)(lVar26 + 0x10 + lVar29);
              local_148 = *(longlong *)(lVar26 + 0x28 + lVar29);
              lVar25 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x140) * 8) * 0x30;
              local_140 = *(longlong *)(lVar26 + 0x10 + lVar25);
              local_138 = *(longlong *)(lVar26 + 0x28 + lVar25);
              local_218 = *(longlong *)(lVar18 + 0x50);
              local_108 = *(ulonglong *)(lVar18 + 0x58);
              local_280 = 0;
            }
            puVar19 = (undefined8 *)(local_218 + local_280 * 0x10);
            uVar23 = (ulonglong)*(uint *)(local_218 + 8 + local_280 * 0x10);
            local_110 = (undefined4 *)(local_198 + uVar23 * 4);
            local_100 = (byte *)(local_190 + uVar23);
            local_1b0 = (undefined4 *)(local_188 + uVar23 * 4);
            puVar31 = (undefined4 *)(local_178 + uVar23 * 4);
            puVar32 = (undefined4 *)(local_168 + uVar23 * 4);
            local_128 = (undefined4 *)(local_160 + uVar23 * 4);
            local_1b8 = (undefined4 *)(local_158 + uVar23 * 4);
            local_120 = (undefined4 *)(local_148 + uVar23 * 4);
            pfVar28 = (float *)(local_140 + uVar23 * 4);
            puVar30 = (undefined4 *)(local_138 + uVar23 * 4);
            lVar26 = uVar23 * 0x100 + local_1a0;
            puVar22 = (undefined8 *)(local_170 + uVar23 * 0xc);
            pfVar33 = (float *)(uVar23 * 0x30 + local_180);
            puVar35 = (undefined4 *)(uVar23 * 0x10 + local_150);
          }
          else {
            if (local_280 == local_108) {
              do {
                if (local_398 == local_390) goto LAB_1423136fb;
                uVar17 = *local_398;
                local_398 = local_398 + 1;
              } while (*(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar17 * 0x48)
                       == 0);
              lVar26 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar17 * 0x48;
              lVar25 = *(longlong *)(lVar26 + 0x38);
              lVar18 = *(longlong *)(lVar26 + 0x18);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
              local_1a0 = *(longlong *)(lVar18 + 0x10 + lVar29);
              local_198 = *(longlong *)(lVar18 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
              local_190 = *(longlong *)(lVar18 + 0x10 + lVar29);
              local_188 = *(longlong *)(lVar18 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
              local_180 = *(longlong *)(lVar18 + 0x10 + lVar29);
              local_178 = *(longlong *)(lVar18 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x128) * 8) * 0x30;
              local_170 = *(longlong *)(lVar18 + 0x10 + lVar29);
              local_168 = *(longlong *)(lVar18 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x130) * 8) * 0x30;
              local_160 = *(longlong *)(lVar18 + 0x10 + lVar29);
              local_158 = *(longlong *)(lVar18 + 0x28 + lVar29);
              lVar29 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x138) * 8) * 0x30;
              local_150 = *(longlong *)(lVar18 + 0x10 + lVar29);
              local_148 = *(longlong *)(lVar18 + 0x28 + lVar29);
              lVar25 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar8 + 0x140) * 8) * 0x30;
              local_140 = *(longlong *)(lVar18 + 0x10 + lVar25);
              local_138 = *(longlong *)(lVar18 + 0x28 + lVar25);
              local_220 = *(longlong *)(lVar26 + 8);
              local_108 = *(ulonglong *)(lVar26 + 0x10);
              local_280 = 0;
            }
            uVar23 = local_280 & 0xffffffff;
            local_110 = (undefined4 *)(local_198 + uVar23 * 4);
            local_100 = (byte *)(local_190 + uVar23);
            local_1b0 = (undefined4 *)(local_188 + uVar23 * 4);
            puVar31 = (undefined4 *)(local_178 + uVar23 * 4);
            puVar32 = (undefined4 *)(local_168 + uVar23 * 4);
            local_128 = (undefined4 *)(local_160 + uVar23 * 4);
            local_1b8 = (undefined4 *)(local_158 + uVar23 * 4);
            local_120 = (undefined4 *)(local_148 + uVar23 * 4);
            pfVar28 = (float *)(local_140 + uVar23 * 4);
            puVar30 = (undefined4 *)(local_138 + uVar23 * 4);
            lVar26 = uVar23 * 0x100 + local_1a0;
            puVar22 = (undefined8 *)(local_170 + uVar23 * 0xc);
            pfVar33 = (float *)(uVar23 * 0x30 + local_180);
            puVar35 = (undefined4 *)(uVar23 * 0x10 + local_150);
            puVar19 = (undefined8 *)(local_220 + local_280 * 8);
          }
          uVar14 = *puVar19;
          local_280 = local_280 + 1;
          if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0')
          break;
          bevy_ecs::system::commands::EntityCommands::despawn();
          local_3b8 = uVar24;
          local_3b0 = uVar11;
        }
        lVar25 = 0;
        if (1 < (ulonglong)*local_100) {
          lVar25 = (ulonglong)*local_100 - 1;
        }
      } while ((lVar25 == 1) || (lVar18 = *(longlong *)(lVar26 + 0xa0), lVar18 == 0));
      if (lVar18 == 1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      pfVar15 = *(float **)(lVar26 + 0x98);
      pfVar34 = pfVar15 + lVar18 * 3;
      if (pfVar34 == (float *)&DAT_0000000c) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      fVar46 = 1.0;
      if (lVar25 != 0) {
        if (lVar25 != 2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar46 = -1.0;
      }
      fVar39 = fVar46 * 0.5 + 0.5;
      fVar45 = 0.0;
      if (0.0 <= *(float *)(lVar26 + 0xec)) {
        fVar45 = *(float *)(lVar26 + 0xec);
      }
      fVar55 = 1.0;
      if (fVar45 <= 1.0) {
        fVar55 = fVar45;
      }
      if ((fVar55 != fVar39) || (NAN(fVar55) || NAN(fVar39))) break;
      bVar20 = 2;
      if (lVar25 != 0) {
        if (lVar25 != 2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        bevy_ecs::system::commands::EntityCommands::despawn();
        bVar20 = 3;
        local_3b8 = uVar24;
        local_3b0 = uVar11;
      }
      *local_1b0 = local_32c;
      *local_100 = bVar20;
      uVar14 = *(undefined8 *)(pfVar33 + 4);
      fVar46 = pfVar33[6];
      ::utils::swizzlers::impl$0::x0y();
      unaff_XMM8_Db = (float)((ulonglong)uVar14 >> 0x20) + (float)(local_3b8 >> 0x20);
      *local_120 = local_2ac;
      *puVar35 = 1;
      *(ulonglong *)(puVar35 + 1) = CONCAT44(unaff_XMM8_Db,(float)uVar14 + (float)local_3b8);
      puVar35[3] = fVar46 + (float)local_3b0;
      *local_110 = local_34c;
      *(undefined2 *)(lVar26 + 0xe0) = 1;
      *(undefined8 *)(lVar26 + 0xe4) = 0x4000000040800000;
      *local_1b8 = local_2cc;
      *local_128 = 0x3f800000;
    }
    fVar55 = SQRT((pfVar34[-1] - pfVar15[2]) * (pfVar34[-1] - pfVar15[2]) +
                  (pfVar34[-3] - *pfVar15) * (pfVar34[-3] - *pfVar15));
    *local_1b8 = local_2cc;
    *local_128 = 0;
    fVar45 = *(float *)(lVar10 + 0x28);
    *local_110 = local_34c;
    fVar39 = (fVar46 * 12.0 * fVar45) / fVar55 + *(float *)(lVar26 + 0xec);
    *(float *)(lVar26 + 0xec) = fVar39;
    fVar45 = (float)*(longlong *)(lVar26 + 0xa0) + -1.0;
    if (fVar45 < 0.0) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    uVar17 = -(uint)(0.0 < fVar46) & (uint)fVar39;
    fVar37 = (float)(~-(uint)(0.0 < fVar46) & (uint)(fVar39 * fVar39) | uVar17) * fVar45;
    fVar39 = 0.0;
    if (0.0 <= fVar37) {
      fVar39 = fVar37;
    }
    if (fVar39 <= fVar45) {
      fVar45 = fVar39;
    }
    truncf(CONCAT44(unaff_XMM8_Db,fVar45),
           (longlong)(fVar45 - 9.223372e+18) & (longlong)fVar45 >> 0x3f | (longlong)fVar45,uVar17,
           puVar30,uVar56);
    ::utils::curve::CurveInternalCoord::new_for_point_count<utils::curve::CurveCoordSemantic3D>();
    uVar23 = (ulonglong)extraout_EAX;
    if (*(ulonglong *)(lVar26 + 0xa0) <= uVar23) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    uVar2 = uVar23 + 1;
    if (*(ulonglong *)(lVar26 + 0xa0) <= uVar2) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    lVar25 = *(longlong *)(lVar26 + 0x98);
    uVar3 = *(undefined8 *)(lVar25 + uVar23 * 0xc);
    fVar45 = *(float *)(lVar25 + 8 + uVar23 * 0xc);
    uVar4 = *(undefined8 *)(lVar25 + uVar2 * 0xc);
    fVar37 = (float)uVar4;
    fVar50 = (float)((ulonglong)uVar4 >> 0x20);
    fVar39 = *(float *)(lVar25 + 8 + uVar2 * 0xc);
    unaff_XMM8_Db = 0.0;
    fVar51 = (float)uVar3;
    fVar54 = (float)((ulonglong)uVar3 >> 0x20);
    if (fVar46 <= 0.0) {
      uVar23 = CONCAT44(fVar54 - fVar50,fVar51 - fVar37);
      fVar46 = fVar45 - fVar39;
    }
    else {
      ::utils::swizzlers::impl$0::x0y();
      uVar23 = local_3b8;
      fVar46 = (float)local_3b0;
    }
    fVar47 = (float)uVar23;
    fVar48 = (float)(uVar23 >> 0x20);
    fVar49 = 1.0 / SQRT(fVar46 * fVar46 + fVar48 * fVar48 + fVar47 * fVar47);
    fVar47 = fVar49 * fVar47;
    fVar48 = fVar49 * fVar48;
    fVar49 = fVar49 * fVar46;
    fVar46 = 1.0 - extraout_XMM0_Da;
    ::utils::swizzlers::impl$0::x0y();
    fVar36 = fVar47 - fVar48 * 0.0;
    fVar41 = fVar48 * 0.0 - fVar49;
    fVar38 = fVar49 * 0.0 - fVar47 * 0.0;
    fVar43 = 1.0 / SQRT(fVar36 * fVar36 + fVar38 * fVar38 + fVar41 * fVar41);
    fVar41 = fVar41 * fVar43;
    fVar38 = fVar38 * fVar43;
    fVar36 = fVar36 * fVar43;
    fVar43 = fVar48 * fVar41 - fVar47 * fVar38;
    fVar40 = fVar49 * fVar38 - fVar48 * fVar36;
    fVar42 = fVar36 * fVar47 - fVar49 * fVar41;
    fVar44 = *(float *)(lVar10 + 0x28) + *(float *)(lVar10 + 0x28);
    auVar52._8_8_ = 0;
    auVar52._0_8_ = local_3b8;
    auVar53._4_4_ = fVar44;
    auVar53._0_4_ = fVar44;
    auVar53._8_8_ = 0;
    auVar53 = divps(auVar52,auVar53);
    *puVar32 = local_2ec;
    *puVar22 = auVar53._0_8_;
    *(float *)(puVar22 + 1) = (float)local_3b0 / fVar44;
    if (0.0 <= fVar49) {
      fVar42 = fVar42 - fVar41;
      if (fVar42 <= 0.0) {
        fVar44 = (fVar49 + 1.0) - fVar42;
        fVar41 = fVar36 - fVar47;
        fVar42 = fVar40 + fVar38;
        fVar38 = fVar48 + fVar43;
        fVar47 = fVar44;
        goto LAB_142313312;
      }
      fVar42 = fVar49 + 1.0 + fVar42;
      fVar41 = 0.5 / SQRT(fVar42);
      fVar44 = fVar41 * (fVar38 + fVar40);
      fVar42 = fVar41 * fVar42;
      fVar48 = fVar41 * (fVar43 - fVar48);
      fVar41 = fVar41 * (-fVar47 - fVar36);
    }
    else {
      fVar41 = fVar41 + fVar42;
      if (fVar41 <= 0.0) {
        fVar44 = fVar36 - fVar47;
        fVar42 = fVar43 - fVar48;
        fVar41 = (1.0 - fVar49) - fVar41;
        fVar38 = fVar38 - fVar40;
        fVar47 = fVar41;
LAB_142313312:
        fVar47 = 0.5 / SQRT(fVar47);
        fVar44 = fVar47 * fVar44;
        fVar42 = fVar47 * fVar42;
        fVar48 = fVar47 * fVar41;
        fVar41 = fVar47 * fVar38;
      }
      else {
        fVar41 = (1.0 - fVar49) + fVar41;
        fVar49 = 0.5 / SQRT(fVar41);
        fVar44 = fVar49 * (fVar48 + fVar43);
        fVar42 = fVar49 * (-fVar47 - fVar36);
        fVar48 = fVar49 * (fVar38 - fVar40);
        fVar41 = fVar41 * fVar49;
      }
    }
    *puVar31 = local_30c;
    *pfVar33 = fVar44;
    pfVar33[1] = fVar42;
    pfVar33[2] = fVar48;
    pfVar33[3] = fVar41;
    *(ulonglong *)(pfVar33 + 4) =
         CONCAT44(fVar46 * fVar54 + extraout_XMM0_Da * fVar50,
                  fVar46 * fVar51 + extraout_XMM0_Da * fVar37);
    pfVar33[6] = fVar39 * extraout_XMM0_Da + fVar45 * fVar46;
    pfVar33[7] = 1.0;
    pfVar33[8] = 1.0;
    pfVar33[9] = 1.0;
    ::utils::inverse_lerp();
    pfVar34 = pfVar33 + 4;
    fVar46 = 0.0;
    if (0.0 <= extraout_XMM0_Da_00) {
      fVar46 = extraout_XMM0_Da_00;
    }
    fVar45 = 1.0;
    if (fVar46 <= 1.0) {
      fVar45 = fVar46;
    }
    *local_110 = local_34c;
    *(float *)(lVar26 + 0xf4) = fVar45;
    fVar39 = fVar45 + -0.5 + fVar45 + -0.5;
    fVar46 = 0.0;
    if (0.0 <= fVar39) {
      fVar46 = fVar39;
    }
    fVar39 = 1.0;
    if (fVar46 <= 1.0) {
      fVar39 = fVar46;
    }
    fVar46 = (3.0 - (fVar39 + fVar39)) * fVar39 * fVar39;
    *puVar30 = local_28c;
    *pfVar28 = fVar46 + fVar46;
    if ((fVar45 <= 0.3) || (1 < *local_100)) {
      if ((0.5 < fVar45) && (*local_100 == 3)) {
        country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
        local_3b0 = (ulonglong)(uint)pfVar33[6];
        local_3b8 = *(ulonglong *)pfVar34;
        country_core::resources::audio_resources::impl$10::set_position();
        uVar56 = 0x43020000;
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
                  ();
        country_core::resources::audio_resources::impl$10::set_volume();
      }
    }
    else {
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      local_3b0 = (ulonglong)(uint)pfVar33[6];
      local_3b8 = *(ulonglong *)pfVar34;
      country_core::resources::audio_resources::impl$10::set_position();
      uVar56 = 0x43020000;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
                ();
      country_core::resources::audio_resources::impl$10::set_volume();
    }
    if (((*(float *)(lVar26 + 0xec) < 0.01) || (0.99 <= *(float *)(lVar26 + 0xec))) ||
       (*local_100 == 2)) {
      *local_110 = local_34c;
      country_core::utils::audio_utils::AudioLoop::stop();
    }
    else {
      *local_110 = local_34c;
      country_core::utils::audio_utils::AudioLoop::start_if_not_playing();
      country_core::utils::audio_utils::AudioLoop::current();
      local_3b0 = (ulonglong)(uint)pfVar33[6];
      local_3b8 = *(ulonglong *)pfVar34;
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::utils::audio_utils::AudioLoop::current();
      country_core::resources::audio_resources::impl$10::set_parameter();
      country_core::utils::audio_utils::AudioLoop::current();
      country_core::resources::audio_resources::impl$10::set_parameter();
    }
    if ((0.25 < fVar45) && (*local_100 < 2 && (*local_100 & 1) == 0)) {
      *local_1b0 = local_32c;
      *local_100 = 1;
      *puVar13 = uVar7;
      lVar25 = *(longlong *)(lVar12 + 0x30);
      uVar3 = *(undefined8 *)(lVar12 + 0x40);
      if (lVar25 == *(longlong *)(lVar12 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar18 = *(longlong *)(lVar12 + 0x28);
      *(undefined8 *)(lVar18 + lVar25 * 0x10) = uVar3;
      *(undefined8 *)(lVar18 + 8 + lVar25 * 0x10) = uVar14;
      *(longlong *)(lVar12 + 0x30) = lVar25 + 1;
      *(longlong *)(lVar12 + 0x40) = *(longlong *)(lVar12 + 0x40) + 1;
    }
    fVar46 = 1.0 - 50.0 / fVar55;
    fVar45 = (*(float *)(lVar26 + 0xec) - fVar46) / ((-20.0 / fVar55 + 1.0) - fVar46);
    fVar46 = 0.0;
    if (0.0 <= fVar45) {
      fVar46 = fVar45;
    }
    fVar45 = 1.0;
    if (fVar46 <= 1.0) {
      fVar45 = fVar46;
    }
    fVar45 = (3.0 - (fVar45 + fVar45)) * fVar45 * fVar45;
    fVar46 = *(float *)(lVar10 + 0x28);
    *local_110 = local_34c;
    *(float *)(lVar26 + 0xf0) = (fVar45 * 4.0 + (1.0 - fVar45)) * fVar46 + *(float *)(lVar26 + 0xf0)
    ;
  } while( true );
LAB_1423136fb:
  if (bVar16) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar21 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_1423137e4:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
LAB_1423137f0:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar21 = *plVar21 + 1;
  }
  return;
}

