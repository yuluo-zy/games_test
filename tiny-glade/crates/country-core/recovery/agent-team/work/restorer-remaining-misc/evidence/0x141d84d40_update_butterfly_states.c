
void __cdecl system_critters::butterfly::update_butterfly_states(void)

{
  byte *pbVar1;
  uint *puVar2;
  undefined4 *puVar3;
  float fVar4;
  byte bVar5;
  byte bVar6;
  byte bVar7;
  ulonglong uVar8;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar9;
  int iVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar11;
  longlong lVar12;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  ulonglong uVar13;
  longlong lVar14;
  longlong *plVar15;
  ulonglong in_RDX;
  uint *puVar16;
  undefined **ppuVar17;
  longlong lVar18;
  longlong *in_R9;
  longlong lVar19;
  bool bVar20;
  bool bVar21;
  float fVar22;
  float extraout_XMM0_Da;
  undefined4 uVar23;
  float extraout_XMM0_Da_00;
  float fVar24;
  float fVar25;
  float fVar26;
  longlong *in_stack_00000030;
  float local_288;
  float local_280;
  longlong local_1f8;
  longlong local_1f0;
  ulonglong local_1e0;
  ulonglong local_1d8;
  byte local_1d0;
  longlong local_178;
  longlong local_160;
  longlong local_148;
  longlong local_140;
  longlong local_118;
  longlong local_110;
  
  fVar25 = local_280;
  uVar13 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar21 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x33) {
          if ((char)(&DAT_142f67e70)[uVar13] < -0x40) {
LAB_141d84e0c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 != 0x33) goto LAB_141d84e0c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar13 != 0xfffffffffffffffe)) {
        if (uVar13 + 2 < 0x33) {
          if ("stem_critters::butterfly::update_butterfly_states::f"[uVar13] < -0x40) {
LAB_141d84e61:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 + 2 != 0x33) goto LAB_141d84e61;
      }
    }
    ppuVar17 = &PTR_anon_3f7b249399f3d69a3d9a879e2ba2969d_15_llvm_14976041230053128051_142f67f98;
    do {
      bVar5 = *(byte *)((longlong)ppuVar17 + -1);
      uVar9 = (uint)(char)bVar5;
      if ((char)bVar5 < '\0') {
        bVar6 = *(byte *)((longlong)ppuVar17 + -2);
        if ((char)bVar6 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar17 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar17 + -4);
            ppuVar17 = (undefined **)((longlong)ppuVar17 + -4);
            uVar9 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar17 = (undefined **)((longlong)ppuVar17 + -3);
            uVar9 = bVar7 & 0xf;
          }
          uVar9 = bVar6 & 0x3f | uVar9 << 6;
        }
        else {
          ppuVar17 = (undefined **)((longlong)ppuVar17 + -2);
          uVar9 = bVar6 & 0x1f;
        }
        uVar9 = (uint)(bVar5 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_141d84f24;
LAB_141d84f39:
        if (ppuVar17 == (undefined **)&UNK_142f67f6f) break;
        if ((undefined *)((longlong)ppuVar17 + -0x142f67f6f) < (undefined *)0x28) {
          if (-0x41 < *(char *)((longlong)ppuVar17 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar17 + -0x142f67f6f) == (undefined *)0x28) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar17 = (undefined **)((longlong)ppuVar17 + -1);
      if (uVar9 == 0x5c) goto LAB_141d84f39;
LAB_141d84f24:
      if (uVar9 == 0x2f) goto LAB_141d84f39;
    } while (ppuVar17 !=
             (undefined **)&anon_3f7b249399f3d69a3d9a879e2ba2969d_15_llvm_14976041230053128051);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_141d85c5e;
    if (*plVar15 != 0) goto LAB_141d85c6a;
    *plVar15 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar15 = *plVar15 + 1;
    bVar21 = true;
  }
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  fVar4 = *(float *)(*in_stack_00000030 + 0x28);
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
LAB_141d85ca8:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if ((extraout_RAX_02[3] != 0x3f) ||
     (iVar10 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                      (void *)extraout_RAX_02[2],0x3f), iVar10 != 0)) {
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
  }
  lVar11 = (**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
  if (lVar11 == 0) goto LAB_141d85ca8;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
  lVar11 = in_R9[1];
  puVar16 = *(uint **)(*in_R9 + 0xf8);
  puVar2 = puVar16 + *(longlong *)(*in_R9 + 0x100);
  local_280 = (float)((int)(float)lVar11 + 0x1a0);
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::sheep::Sheep>_>,tuple_<>_>,system_critters::butterfly::update_butterfly_states::closure_env_0>_>
            ();
  lVar19 = *in_RCX;
  lVar18 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Velocity>,ref_mut_<country_core::resources::critters::Butterfly>,ref_mut_<country_core::resources::critters::ButterflyAudio>,ref_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>
            ();
  local_178 = 8;
  local_148 = 0;
  local_140 = 0;
  local_118 = local_1f8;
  local_110 = local_1f0;
  if ((local_1d0 & 1) == 0) {
    if (local_1d8 != local_1e0) {
LAB_141d8563c:
      uVar13 = (ulonglong)*(uint *)(local_178 + 8 + local_1d8 * 0x10);
      goto LAB_141d85655;
    }
    if (puVar16 != puVar2) {
      do {
        if (*(longlong *)(*(longlong *)(lVar18 + 0x100) + 0x58 + (ulonglong)*puVar16 * 0xa0) != 0) {
          lVar12 = *(longlong *)(lVar18 + 0x100) + (ulonglong)*puVar16 * 0xa0;
          uVar13 = (ulonglong)*(uint *)(lVar12 + 0x94);
          lVar14 = *(longlong *)(lVar18 + 0x1a8) + uVar13 * 0x48;
          uVar13 = *(ulonglong *)(*(longlong *)(lVar18 + 0x1a8) + 0x40 + uVar13 * 0x48);
          if ((*(ulonglong *)(lVar19 + 0x110) < uVar13) &&
             (uVar8 = *(ulonglong *)
                       (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar19 + 0x110) * 8),
             uVar8 != 0)) {
            local_118 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar8 * 0x30);
          }
          else {
            local_118 = 0;
          }
          if (*(ulonglong *)(lVar19 + 0x118) < uVar13) {
            uVar13 = *(ulonglong *)
                      (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar19 + 0x118) * 8);
            lVar18 = *(longlong *)(lVar14 + 0x18);
            if (uVar13 == 0) goto LAB_141d855ab;
            local_110 = *(longlong *)(lVar18 + 0x10 + ~uVar13 * 0x30);
          }
          else {
            lVar18 = *(longlong *)(lVar14 + 0x18);
LAB_141d855ab:
            local_110 = 0;
          }
          lVar19 = ~*(ulonglong *)(*(longlong *)(lVar14 + 0x38) + *(longlong *)(lVar19 + 0x120) * 8)
                   * 0x30;
          local_148 = *(longlong *)(lVar18 + 0x10 + lVar19);
          local_140 = *(longlong *)(lVar18 + 0x28 + lVar19);
          local_178 = *(longlong *)(lVar12 + 0x50);
          local_1d8 = 0;
          goto LAB_141d8563c;
        }
        puVar16 = puVar16 + 1;
      } while (puVar16 != puVar2);
    }
  }
  else {
    if (local_1d8 != local_1e0) {
LAB_141d85599:
      uVar13 = local_1d8 & 0xffffffff;
LAB_141d85655:
      puVar2 = (uint *)(local_148 + uVar13 * 0x18);
      local_118 = uVar13 * 0x30 + local_118;
      puVar3 = (undefined4 *)(local_140 + uVar13 * 4);
      country_core::systems::camera_rig::MainCamera::mode();
      if (extraout_AL_00 == '\x03') {
        country_core::systems::camera_rig::MainCamera::eye_position();
        fVar22 = *(float *)(local_118 + 0x10);
        fVar24 = *(float *)(local_118 + 0x18);
        fVar25 = (float)lVar11 - fVar22;
        fVar25 = SQRT((local_280 - fVar24) * (local_280 - fVar24) + fVar25 * fVar25);
        bVar21 = fVar25 == 3.0;
        bVar20 = 3.0 < fVar25;
      }
      else {
        fVar22 = *(float *)(local_118 + 0x10);
        fVar24 = *(float *)(local_118 + 0x18);
        fVar25 = (fVar24 - fVar25) * (fVar24 - fVar25) + (fVar22 - local_288) * (fVar22 - local_288)
        ;
        bVar21 = fVar25 == 4.0;
        bVar20 = 4.0 < fVar25;
      }
      lVar11 = 0;
      do {
        lVar19 = lVar11;
        if (local_160 << 3 == lVar19) break;
        fVar25 = fVar22 - (float)*(undefined8 *)(lVar19 + 0x3f);
        fVar26 = fVar24 - (float)((ulonglong)*(undefined8 *)(lVar19 + 0x3f) >> 0x20);
        lVar11 = lVar19 + 8;
      } while (4.0 <= fVar26 * fVar26 + fVar25 * fVar25);
      fVar25 = (float)puVar2[4] - fVar4 * 0.5;
      if (fVar25 <= 0.0) {
        fVar25 = 0.0;
      }
      *puVar3 = 0;
      puVar2[4] = (uint)fVar25;
      if (!bVar20 && !bVar21 || local_160 << 3 != lVar19) {
        *puVar3 = 0;
        puVar2[4] = 0x3f800000;
        fVar25 = 1.0;
      }
      fVar22 = (float)puVar2[3];
      if ((fVar22 <= 0.0) || (0.0 < *(float *)(local_110 + uVar13 * 0xc + 4))) {
        if (fVar22 <= 0.0) {
          *puVar3 = 0;
          fVar25 = (float)puVar2[3];
          puVar2[3] = (uint)(fVar25 + fVar4);
          if (fVar25 + fVar4 <= 0.0) {
                    // WARNING: Could not recover jumptable at 0x000141d85a20. Too many branches
                    // WARNING: Treating indirect jump as call
            (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                      (-(uint)(0.0 < (float)puVar2[4]) & 0x40800000);
            return;
          }
          perchance::PerchanceContext::uniform_f32();
          *puVar3 = 0;
          puVar2[3] = (uint)(extraout_XMM0_Da_00 * 0.3 + 0.2);
                    // WARNING: Could not recover jumptable at 0x000141d8594f. Too many branches
                    // WARNING: Treating indirect jump as call
          (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                    (-(uint)(0.0 < (float)puVar2[4]) & 0x40800000);
          return;
        }
LAB_141d85866:
        uVar23 = 0x40400000;
        if (0.0 < fVar22) goto LAB_141d85955;
        bVar21 = false;
      }
      else {
        if ((ABS(0.5 - (float)puVar2[5]) < 0.2) || (0.45 <= ABS(0.5 - (float)puVar2[5])))
        goto LAB_141d85866;
        *puVar3 = 0;
        fVar25 = (float)puVar2[3];
        puVar2[3] = (uint)(fVar25 - fVar4);
        bVar21 = true;
        if (0.0 < fVar25 - fVar4) {
                    // WARNING: Could not recover jumptable at 0x000141d85850. Too many branches
                    // WARNING: Treating indirect jump as call
          (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                    (-(uint)(0.0 < (float)puVar2[4]) & 0x40800000);
          return;
        }
      }
      perchance::PerchanceContext::uniform_f32();
      *puVar3 = 0;
      puVar2[3] = (uint)-(extraout_XMM0_Da * 1.5 + 0.5);
      fVar25 = (float)puVar2[4];
      if (bVar21) {
        uVar23 = 0x3f800000;
      }
      else {
        uVar23 = 0x40400000;
      }
LAB_141d85955:
                    // WARNING: Could not recover jumptable at 0x000141d85981. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                (uVar23,fVar25,-(uint)(0.0 < fVar25) & 0x40800000);
      return;
    }
    if (puVar16 != puVar2) {
      do {
        if (*(longlong *)(*(longlong *)(lVar18 + 0x1a8) + 0x10 + (ulonglong)*puVar16 * 0x48) != 0) {
          lVar18 = *(longlong *)(lVar18 + 0x1a8) + (ulonglong)*puVar16 * 0x48;
          if ((*(ulonglong *)(lVar19 + 0x110) < *(ulonglong *)(lVar18 + 0x40)) &&
             (uVar13 = *(ulonglong *)
                        (*(longlong *)(lVar18 + 0x38) + *(ulonglong *)(lVar19 + 0x110) * 8),
             uVar13 != 0)) {
            local_118 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar13 * 0x30);
          }
          else {
            local_118 = 0;
          }
          if (*(ulonglong *)(lVar19 + 0x118) < *(ulonglong *)(lVar18 + 0x40)) {
            uVar13 = *(ulonglong *)
                      (*(longlong *)(lVar18 + 0x38) + *(ulonglong *)(lVar19 + 0x118) * 8);
            lVar14 = *(longlong *)(lVar18 + 0x18);
            if (uVar13 == 0) goto LAB_141d8551a;
            local_110 = *(longlong *)(lVar14 + 0x10 + ~uVar13 * 0x30);
          }
          else {
            lVar14 = *(longlong *)(lVar18 + 0x18);
LAB_141d8551a:
            local_110 = 0;
          }
          lVar19 = ~*(ulonglong *)(*(longlong *)(lVar18 + 0x38) + *(longlong *)(lVar19 + 0x120) * 8)
                   * 0x30;
          local_148 = *(longlong *)(lVar14 + 0x10 + lVar19);
          local_140 = *(longlong *)(lVar14 + 0x28 + lVar19);
          local_1d8 = 0;
          goto LAB_141d85599;
        }
        puVar16 = puVar16 + 1;
      } while (puVar16 != puVar2);
    }
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  if (bVar21) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar15 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_141d85c5e:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_141d85c6a:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar15 = *plVar15 + 1;
  }
  return;
}

