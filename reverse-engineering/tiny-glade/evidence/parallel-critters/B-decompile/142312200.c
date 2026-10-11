
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar17;
  char extraout_AL;
  uint uVar18;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar19;
  undefined8 *puVar20;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  byte bVar21;
  longlong *in_RCX;
  longlong *plVar22;
  undefined8 *puVar23;
  ulonglong uVar24;
  longlong *in_RDX;
  ulonglong uVar25;
  longlong lVar26;
  longlong lVar27;
  undefined *puVar28;
  float *pfVar29;
  ulonglong *in_R9;
  longlong lVar30;
  undefined4 *puVar31;
  undefined4 *puVar32;
  undefined4 *puVar33;
  float *pfVar34;
  float *pfVar35;
  undefined4 *puVar36;
  float extraout_XMM0_Da;
  float fVar37;
  float fVar38;
  float extraout_XMM0_Da_00;
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
  float unaff_XMM8_Db;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar56;
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  float fVar57;
  longlong *in_stack_00000028;
  ulonglong uVar58;
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
    uVar25 = 0x34;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar25 != 0) {
        if (uVar25 < 0x34) {
          if ((char)(&DAT_14304a948)[uVar25] < -0x40) {
LAB_1423122c8:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 != 0x34) goto LAB_1423122c8;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar25 != 0xfffffffffffffffe)) {
        if (uVar25 + 2 < 0x34) {
          if ((char)(&UNK_14304a94a)[uVar25] < -0x40) {
LAB_142312326:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 + 2 != 0x34) goto LAB_142312326;
      }
    }
    puVar28 = &DAT_14304aa9b;
    do {
      bVar21 = puVar28[-1];
      uVar18 = (uint)(char)bVar21;
      if ((char)bVar21 < '\0') {
        bVar5 = puVar28[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar28[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar18 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar18 = bVar6 & 0xf;
          }
          uVar18 = bVar5 & 0x3f | uVar18 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar18 = bVar5 & 0x1f;
        }
        uVar18 = (uint)(bVar21 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto LAB_1423123f4;
LAB_142312405:
        if (puVar28 == &UNK_14304aa77) break;
        if (puVar28 + -0x14304aa77 < (undefined *)0x23) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x14304aa77 == (undefined *)0x23) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar18 == 0x5c) goto LAB_142312405;
LAB_1423123f4:
      if (uVar18 == 0x2f) goto LAB_142312405;
    } while (puVar28 != &DAT_14304aa78);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar22 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar22 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_1423137e4;
    if (*plVar22 != 0) goto LAB_1423137f0;
    *plVar22 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar22 = *plVar22 + 1;
    bVar16 = true;
  }
  lVar8 = *in_RDX;
  lVar9 = in_RDX[1];
  uVar58 = (ulonglong)*(uint *)((longlong)in_RDX + 0x14);
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<bevy_ecs::entity::Entity,ref_mut_<country_core::resources::critters::Duck>,ref_mut_<enum2_<country_core::resources::critters::DuckState>_>,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::Velocity>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush>_>,tuple_<>_>
            ();
  fVar17 = __real_3f800000;
  local_220 = local_388;
  local_218 = local_378;
  lVar10 = *in_RCX;
  uVar25 = *in_R9;
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
                uVar18 = *local_398;
                local_398 = local_398 + 1;
              } while (*(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar18 * 0xa0)
                       == 0);
              lVar19 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar18 * 0xa0;
              lVar27 = *(longlong *)
                        (*(longlong *)(lVar9 + 0x1a8) + 0x18 +
                        (ulonglong)*(uint *)(lVar19 + 0x94) * 0x48);
              lVar26 = *(longlong *)
                        (*(longlong *)(lVar9 + 0x1a8) + 0x38 +
                        (ulonglong)*(uint *)(lVar19 + 0x94) * 0x48);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
              local_1a0 = *(longlong *)(lVar27 + 0x10 + lVar30);
              local_198 = *(longlong *)(lVar27 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
              local_190 = *(longlong *)(lVar27 + 0x10 + lVar30);
              local_188 = *(longlong *)(lVar27 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
              local_180 = *(longlong *)(lVar27 + 0x10 + lVar30);
              local_178 = *(longlong *)(lVar27 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x128) * 8) * 0x30;
              local_170 = *(longlong *)(lVar27 + 0x10 + lVar30);
              local_168 = *(longlong *)(lVar27 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x130) * 8) * 0x30;
              local_160 = *(longlong *)(lVar27 + 0x10 + lVar30);
              local_158 = *(longlong *)(lVar27 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x138) * 8) * 0x30;
              local_150 = *(longlong *)(lVar27 + 0x10 + lVar30);
              local_148 = *(longlong *)(lVar27 + 0x28 + lVar30);
              lVar26 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x140) * 8) * 0x30;
              local_140 = *(longlong *)(lVar27 + 0x10 + lVar26);
              local_138 = *(longlong *)(lVar27 + 0x28 + lVar26);
              local_218 = *(longlong *)(lVar19 + 0x50);
              local_108 = *(ulonglong *)(lVar19 + 0x58);
              local_280 = 0;
            }
            puVar20 = (undefined8 *)(local_218 + local_280 * 0x10);
            uVar24 = (ulonglong)*(uint *)(local_218 + 8 + local_280 * 0x10);
            local_110 = (undefined4 *)(local_198 + uVar24 * 4);
            local_100 = (byte *)(local_190 + uVar24);
            local_1b0 = (undefined4 *)(local_188 + uVar24 * 4);
            puVar32 = (undefined4 *)(local_178 + uVar24 * 4);
            puVar33 = (undefined4 *)(local_168 + uVar24 * 4);
            local_128 = (undefined4 *)(local_160 + uVar24 * 4);
            local_1b8 = (undefined4 *)(local_158 + uVar24 * 4);
            local_120 = (undefined4 *)(local_148 + uVar24 * 4);
            pfVar29 = (float *)(local_140 + uVar24 * 4);
            puVar31 = (undefined4 *)(local_138 + uVar24 * 4);
            lVar27 = uVar24 * 0x100 + local_1a0;
            puVar23 = (undefined8 *)(local_170 + uVar24 * 0xc);
            pfVar34 = (float *)(uVar24 * 0x30 + local_180);
            puVar36 = (undefined4 *)(uVar24 * 0x10 + local_150);
          }
          else {
            if (local_280 == local_108) {
              do {
                if (local_398 == local_390) goto LAB_1423136fb;
                uVar18 = *local_398;
                local_398 = local_398 + 1;
              } while (*(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar18 * 0x48)
                       == 0);
              lVar27 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar18 * 0x48;
              lVar26 = *(longlong *)(lVar27 + 0x38);
              lVar19 = *(longlong *)(lVar27 + 0x18);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
              local_1a0 = *(longlong *)(lVar19 + 0x10 + lVar30);
              local_198 = *(longlong *)(lVar19 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
              local_190 = *(longlong *)(lVar19 + 0x10 + lVar30);
              local_188 = *(longlong *)(lVar19 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
              local_180 = *(longlong *)(lVar19 + 0x10 + lVar30);
              local_178 = *(longlong *)(lVar19 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x128) * 8) * 0x30;
              local_170 = *(longlong *)(lVar19 + 0x10 + lVar30);
              local_168 = *(longlong *)(lVar19 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x130) * 8) * 0x30;
              local_160 = *(longlong *)(lVar19 + 0x10 + lVar30);
              local_158 = *(longlong *)(lVar19 + 0x28 + lVar30);
              lVar30 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x138) * 8) * 0x30;
              local_150 = *(longlong *)(lVar19 + 0x10 + lVar30);
              local_148 = *(longlong *)(lVar19 + 0x28 + lVar30);
              lVar26 = ~*(ulonglong *)(lVar26 + *(longlong *)(lVar8 + 0x140) * 8) * 0x30;
              local_140 = *(longlong *)(lVar19 + 0x10 + lVar26);
              local_138 = *(longlong *)(lVar19 + 0x28 + lVar26);
              local_220 = *(longlong *)(lVar27 + 8);
              local_108 = *(ulonglong *)(lVar27 + 0x10);
              local_280 = 0;
            }
            uVar24 = local_280 & 0xffffffff;
            local_110 = (undefined4 *)(local_198 + uVar24 * 4);
            local_100 = (byte *)(local_190 + uVar24);
            local_1b0 = (undefined4 *)(local_188 + uVar24 * 4);
            puVar32 = (undefined4 *)(local_178 + uVar24 * 4);
            puVar33 = (undefined4 *)(local_168 + uVar24 * 4);
            local_128 = (undefined4 *)(local_160 + uVar24 * 4);
            local_1b8 = (undefined4 *)(local_158 + uVar24 * 4);
            local_120 = (undefined4 *)(local_148 + uVar24 * 4);
            pfVar29 = (float *)(local_140 + uVar24 * 4);
            puVar31 = (undefined4 *)(local_138 + uVar24 * 4);
            lVar27 = uVar24 * 0x100 + local_1a0;
            puVar23 = (undefined8 *)(local_170 + uVar24 * 0xc);
            pfVar34 = (float *)(uVar24 * 0x30 + local_180);
            puVar36 = (undefined4 *)(uVar24 * 0x10 + local_150);
            puVar20 = (undefined8 *)(local_220 + local_280 * 8);
          }
          uVar14 = *puVar20;
          local_280 = local_280 + 1;
          if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0')
          break;
          bevy_ecs::system::commands::EntityCommands::despawn();
          local_3b8 = uVar25;
          local_3b0 = uVar11;
        }
        lVar26 = 0;
        if (1 < (ulonglong)*local_100) {
          lVar26 = (ulonglong)*local_100 - 1;
        }
      } while ((lVar26 == 1) || (lVar19 = *(longlong *)(lVar27 + 0xa0), lVar19 == 0));
      if (lVar19 == 1) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      pfVar15 = *(float **)(lVar27 + 0x98);
      pfVar35 = pfVar15 + lVar19 * 3;
      if (pfVar35 == (float *)&DAT_0000000c) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      fVar37 = fVar17;
      if ((lVar26 != 0) && (fVar37 = __real_bf800000, lVar26 != 2)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      fVar41 = fVar37 * __real_3f000000 + __real_3f000000;
      fVar48 = 0.0;
      if (0.0 <= *(float *)(lVar27 + 0xec)) {
        fVar48 = *(float *)(lVar27 + 0xec);
      }
      fVar57 = fVar17;
      if (fVar48 <= fVar17) {
        fVar57 = fVar48;
      }
      if ((fVar57 != fVar41) || (NAN(fVar57) || NAN(fVar41))) break;
      bVar21 = 2;
      if (lVar26 != 0) {
        if (lVar26 != 2) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        bevy_ecs::system::commands::EntityCommands::despawn();
        bVar21 = 3;
        local_3b8 = uVar25;
        local_3b0 = uVar11;
      }
      *local_1b0 = local_32c;
      *local_100 = bVar21;
      uVar14 = *(undefined8 *)(pfVar34 + 4);
      fVar37 = pfVar34[6];
      ::utils::swizzlers::impl$0::x0y();
      unaff_XMM8_Db = (float)((ulonglong)uVar14 >> 0x20) + (float)(local_3b8 >> 0x20);
      *local_120 = local_2ac;
      *puVar36 = 1;
      *(ulonglong *)(puVar36 + 1) = CONCAT44(unaff_XMM8_Db,(float)uVar14 + (float)local_3b8);
      puVar36[3] = fVar37 + (float)local_3b0;
      *local_110 = local_34c;
      *(undefined2 *)(lVar27 + 0xe0) = 1;
      *(undefined8 *)(lVar27 + 0xe4) = __xmm_00000000000000004000000040800000;
      *local_1b8 = local_2cc;
      *local_128 = 0x3f800000;
    }
    fVar57 = SQRT((pfVar35[-1] - pfVar15[2]) * (pfVar35[-1] - pfVar15[2]) +
                  (pfVar35[-3] - *pfVar15) * (pfVar35[-3] - *pfVar15));
    *local_1b8 = local_2cc;
    *local_128 = 0;
    fVar41 = fVar37 * __real_41400000;
    fVar48 = *(float *)(lVar10 + 0x28);
    *local_110 = local_34c;
    fVar41 = (fVar41 * fVar48) / fVar57 + *(float *)(lVar27 + 0xec);
    *(float *)(lVar27 + 0xec) = fVar41;
    fVar48 = (float)*(longlong *)(lVar27 + 0xa0) + __real_bf800000;
    if (fVar48 < 0.0) {
                    /* WARNING: Subroutine does not return */
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    uVar18 = -(uint)(0.0 < fVar37) & (uint)fVar41;
    fVar39 = (float)(~-(uint)(0.0 < fVar37) & (uint)(fVar41 * fVar41) | uVar18) * fVar48;
    fVar41 = 0.0;
    if (0.0 <= fVar39) {
      fVar41 = fVar39;
    }
    if (fVar41 <= fVar48) {
      fVar48 = fVar41;
    }
    truncf(CONCAT44(unaff_XMM8_Db,fVar48),
           (longlong)(fVar48 - __real_5f000000) & (longlong)fVar48 >> 0x3f | (longlong)fVar48,uVar18
           ,puVar31,uVar58);
    ::utils::curve::CurveInternalCoord::new_for_point_count<utils::curve::CurveCoordSemantic3D>();
    uVar24 = (ulonglong)extraout_EAX;
    if (*(ulonglong *)(lVar27 + 0xa0) <= uVar24) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    uVar2 = uVar24 + 1;
    if (*(ulonglong *)(lVar27 + 0xa0) <= uVar2) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    lVar26 = *(longlong *)(lVar27 + 0x98);
    uVar3 = *(undefined8 *)(lVar26 + uVar24 * 0xc);
    fVar48 = *(float *)(lVar26 + 8 + uVar24 * 0xc);
    uVar4 = *(undefined8 *)(lVar26 + uVar2 * 0xc);
    fVar39 = (float)uVar4;
    fVar52 = (float)((ulonglong)uVar4 >> 0x20);
    fVar41 = *(float *)(lVar26 + 8 + uVar2 * 0xc);
    unaff_XMM8_Db = 0.0;
    fVar53 = (float)uVar3;
    fVar56 = (float)((ulonglong)uVar3 >> 0x20);
    if (fVar37 <= 0.0) {
      uVar24 = CONCAT44(fVar56 - fVar52,fVar53 - fVar39);
      fVar37 = fVar48 - fVar41;
    }
    else {
      ::utils::swizzlers::impl$0::x0y();
      uVar24 = local_3b8;
      fVar37 = (float)local_3b0;
    }
    fVar49 = (float)uVar24;
    fVar50 = (float)(uVar24 >> 0x20);
    fVar51 = fVar17 / SQRT(fVar37 * fVar37 + fVar50 * fVar50 + fVar49 * fVar49);
    fVar49 = fVar51 * fVar49;
    fVar50 = fVar51 * fVar50;
    fVar51 = fVar51 * fVar37;
    fVar37 = fVar17 - extraout_XMM0_Da;
    ::utils::swizzlers::impl$0::x0y();
    fVar38 = fVar49 - fVar50 * 0.0;
    fVar43 = fVar50 * 0.0 - fVar51;
    fVar40 = fVar51 * 0.0 - fVar49 * 0.0;
    fVar46 = fVar17 / SQRT(fVar38 * fVar38 + fVar40 * fVar40 + fVar43 * fVar43);
    fVar43 = fVar43 * fVar46;
    fVar40 = fVar40 * fVar46;
    fVar38 = fVar38 * fVar46;
    fVar46 = fVar50 * fVar43 - fVar49 * fVar40;
    fVar42 = fVar51 * fVar40 - fVar50 * fVar38;
    fVar44 = fVar38 * fVar49 - fVar51 * fVar43;
    fVar45 = (float)((uint)fVar49 ^ ___xmm_80000000800000008000000080000000);
    fVar47 = *(float *)(lVar10 + 0x28) + *(float *)(lVar10 + 0x28);
    auVar54._8_8_ = 0;
    auVar54._0_8_ = local_3b8;
    auVar55._4_4_ = fVar47;
    auVar55._0_4_ = fVar47;
    auVar55._8_8_ = 0;
    auVar55 = divps(auVar54,auVar55);
    *puVar33 = local_2ec;
    *puVar23 = auVar55._0_8_;
    *(float *)(puVar23 + 1) = (float)local_3b0 / fVar47;
    if (0.0 <= fVar51) {
      fVar44 = fVar44 - fVar43;
      if (fVar44 <= 0.0) {
        fVar47 = (fVar51 + fVar17) - fVar44;
        fVar43 = fVar38 - fVar49;
        fVar44 = fVar42 + fVar40;
        fVar40 = fVar50 + fVar46;
        fVar49 = fVar47;
        goto LAB_142313312;
      }
      fVar44 = fVar51 + fVar17 + fVar44;
      fVar43 = __real_3f000000 / SQRT(fVar44);
      fVar47 = fVar43 * (fVar40 + fVar42);
      fVar44 = fVar43 * fVar44;
      fVar49 = fVar43 * (fVar46 - fVar50);
      fVar43 = fVar43 * (fVar45 - fVar38);
    }
    else {
      fVar43 = fVar43 + fVar44;
      if (fVar43 <= 0.0) {
        fVar47 = fVar38 - fVar49;
        fVar44 = fVar46 - fVar50;
        fVar43 = (fVar17 - fVar51) - fVar43;
        fVar40 = fVar40 - fVar42;
        fVar49 = fVar43;
LAB_142313312:
        fVar38 = __real_3f000000 / SQRT(fVar49);
        fVar47 = fVar38 * fVar47;
        fVar44 = fVar38 * fVar44;
        fVar49 = fVar38 * fVar43;
        fVar43 = fVar38 * fVar40;
      }
      else {
        fVar43 = (fVar17 - fVar51) + fVar43;
        fVar51 = __real_3f000000 / SQRT(fVar43);
        fVar47 = fVar51 * (fVar50 + fVar46);
        fVar44 = fVar51 * (fVar45 - fVar38);
        fVar49 = fVar51 * (fVar40 - fVar42);
        fVar43 = fVar43 * fVar51;
      }
    }
    *puVar32 = local_30c;
    *pfVar34 = fVar47;
    pfVar34[1] = fVar44;
    pfVar34[2] = fVar49;
    pfVar34[3] = fVar43;
    *(ulonglong *)(pfVar34 + 4) =
         CONCAT44(fVar37 * fVar56 + extraout_XMM0_Da * fVar52,
                  fVar37 * fVar53 + extraout_XMM0_Da * fVar39);
    pfVar34[6] = fVar41 * extraout_XMM0_Da + fVar48 * fVar37;
    pfVar34[7] = 1.0;
    pfVar34[8] = 1.0;
    pfVar34[9] = 1.0;
    fVar48 = __real_c1a00000 / fVar57;
    ::utils::inverse_lerp();
    pfVar35 = pfVar34 + 4;
    fVar37 = 0.0;
    if (0.0 <= extraout_XMM0_Da_00) {
      fVar37 = extraout_XMM0_Da_00;
    }
    fVar41 = fVar17;
    if (fVar37 <= fVar17) {
      fVar41 = fVar37;
    }
    *local_110 = local_34c;
    *(float *)(lVar27 + 0xf4) = fVar41;
    fVar39 = fVar41 + __real_bf000000 + fVar41 + __real_bf000000;
    fVar37 = 0.0;
    if (0.0 <= fVar39) {
      fVar37 = fVar39;
    }
    fVar39 = fVar17;
    if (fVar37 <= fVar17) {
      fVar39 = fVar37;
    }
    fVar37 = (__real_40400000 - (fVar39 + fVar39)) * fVar39 * fVar39;
    *puVar31 = local_28c;
    *pfVar29 = fVar37 + fVar37;
    if ((fVar41 <= __real_3e99999a) || (1 < *local_100)) {
      if ((__real_3f000000 < fVar41) && (*local_100 == 3)) {
        country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
        local_3b0 = (ulonglong)(uint)pfVar34[6];
        local_3b8 = *(ulonglong *)pfVar35;
        country_core::resources::audio_resources::impl$10::set_position();
        uVar58 = 0x43020000;
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
                  ();
        country_core::resources::audio_resources::impl$10::set_volume();
      }
    }
    else {
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      local_3b0 = (ulonglong)(uint)pfVar34[6];
      local_3b8 = *(ulonglong *)pfVar35;
      country_core::resources::audio_resources::impl$10::set_position();
      uVar58 = 0x43020000;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
                ();
      country_core::resources::audio_resources::impl$10::set_volume();
    }
    if (((*(float *)(lVar27 + 0xec) < __real_3c23d70a) ||
        (__real_3f7d70a4 <= *(float *)(lVar27 + 0xec))) || (*local_100 == 2)) {
      *local_110 = local_34c;
      country_core::utils::audio_utils::AudioLoop::stop();
    }
    else {
      *local_110 = local_34c;
      country_core::utils::audio_utils::AudioLoop::start_if_not_playing();
      country_core::utils::audio_utils::AudioLoop::current();
      local_3b0 = (ulonglong)(uint)pfVar34[6];
      local_3b8 = *(ulonglong *)pfVar35;
      country_core::resources::audio_resources::impl$10::set_position();
      country_core::utils::audio_utils::AudioLoop::current();
      country_core::resources::audio_resources::impl$10::set_parameter();
      country_core::utils::audio_utils::AudioLoop::current();
      country_core::resources::audio_resources::impl$10::set_parameter();
    }
    if ((__real_3e800000 < fVar41) && (*local_100 < 2 && (*local_100 & 1) == 0)) {
      *local_1b0 = local_32c;
      *local_100 = 1;
      *puVar13 = uVar7;
      lVar26 = *(longlong *)(lVar12 + 0x30);
      uVar3 = *(undefined8 *)(lVar12 + 0x40);
      if (lVar26 == *(longlong *)(lVar12 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar19 = *(longlong *)(lVar12 + 0x28);
      *(undefined8 *)(lVar19 + lVar26 * 0x10) = uVar3;
      *(undefined8 *)(lVar19 + 8 + lVar26 * 0x10) = uVar14;
      *(longlong *)(lVar12 + 0x30) = lVar26 + 1;
      *(longlong *)(lVar12 + 0x40) = *(longlong *)(lVar12 + 0x40) + 1;
    }
    fVar37 = fVar17 - __real_42480000 / fVar57;
    fVar48 = (*(float *)(lVar27 + 0xec) - fVar37) / ((fVar48 + fVar17) - fVar37);
    fVar37 = 0.0;
    if (0.0 <= fVar48) {
      fVar37 = fVar48;
    }
    fVar48 = fVar17;
    if (fVar37 <= fVar17) {
      fVar48 = fVar37;
    }
    fVar48 = (__real_40400000 - (fVar48 + fVar48)) * fVar48 * fVar48;
    fVar41 = fVar48 * __real_40800000;
    fVar37 = *(float *)(lVar10 + 0x28);
    *local_110 = local_34c;
    *(float *)(lVar27 + 0xf0) = (fVar41 + (fVar17 - fVar48)) * fVar37 + *(float *)(lVar27 + 0xf0);
  } while( true );
LAB_1423136fb:
  if (bVar16) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar22 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar22 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_1423137e4:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar22 != 0) {
LAB_1423137f0:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar22 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar22 = *plVar22 + 1;
  }
  return;
}

