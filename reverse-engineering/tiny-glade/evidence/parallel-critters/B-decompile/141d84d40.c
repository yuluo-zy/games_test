
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar9;
  float fVar10;
  float fVar11;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar12;
  int iVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar14;
  longlong lVar15;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  ulonglong uVar16;
  longlong lVar17;
  longlong *plVar18;
  ulonglong in_RDX;
  uint *puVar19;
  undefined **ppuVar20;
  longlong lVar21;
  longlong *in_R9;
  longlong lVar22;
  bool bVar23;
  bool bVar24;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
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
  
  fVar27 = local_280;
  uVar16 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar24 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x33) {
          if ((char)(&DAT_142f67e70)[uVar16] < -0x40) {
LAB_141d84e0c:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 != 0x33) goto LAB_141d84e0c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar16 != 0xfffffffffffffffe)) {
        if (uVar16 + 2 < 0x33) {
          if ("stem_critters::butterfly::update_butterfly_states::f"[uVar16] < -0x40) {
LAB_141d84e61:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 + 2 != 0x33) goto LAB_141d84e61;
      }
    }
    ppuVar20 = &PTR_anon_3f7b249399f3d69a3d9a879e2ba2969d_15_llvm_14976041230053128051_142f67f98;
    do {
      bVar5 = *(byte *)((longlong)ppuVar20 + -1);
      uVar12 = (uint)(char)bVar5;
      if ((char)bVar5 < '\0') {
        bVar6 = *(byte *)((longlong)ppuVar20 + -2);
        if ((char)bVar6 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar20 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar20 + -4);
            ppuVar20 = (undefined **)((longlong)ppuVar20 + -4);
            uVar12 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar20 = (undefined **)((longlong)ppuVar20 + -3);
            uVar12 = bVar7 & 0xf;
          }
          uVar12 = bVar6 & 0x3f | uVar12 << 6;
        }
        else {
          ppuVar20 = (undefined **)((longlong)ppuVar20 + -2);
          uVar12 = bVar6 & 0x1f;
        }
        uVar12 = (uint)(bVar5 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_141d84f24;
LAB_141d84f39:
        if (ppuVar20 == (undefined **)&UNK_142f67f6f) break;
        if ((undefined *)((longlong)ppuVar20 + -0x142f67f6f) < (undefined *)0x28) {
          if (-0x41 < *(char *)((longlong)ppuVar20 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar20 + -0x142f67f6f) == (undefined *)0x28) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      ppuVar20 = (undefined **)((longlong)ppuVar20 + -1);
      if (uVar12 == 0x5c) goto LAB_141d84f39;
LAB_141d84f24:
      if (uVar12 == 0x2f) goto LAB_141d84f39;
    } while (ppuVar20 !=
             (undefined **)&anon_3f7b249399f3d69a3d9a879e2ba2969d_15_llvm_14976041230053128051);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_141d85c5e;
    if (*plVar18 != 0) goto LAB_141d85c6a;
    *plVar18 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar18 = *plVar18 + 1;
    bVar24 = true;
  }
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  fVar4 = *(float *)(*in_stack_00000030 + 0x28);
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
LAB_141d85ca8:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  if ((extraout_RAX_02[3] != 0x3f) ||
     (iVar13 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                      (void *)extraout_RAX_02[2],0x3f), iVar13 != 0)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
  }
  lVar14 = (**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
  if (lVar14 == 0) goto LAB_141d85ca8;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
  lVar14 = in_R9[1];
  puVar19 = *(uint **)(*in_R9 + 0xf8);
  puVar2 = puVar19 + *(longlong *)(*in_R9 + 0x100);
  local_280 = (float)((int)(float)lVar14 + 0x1a0);
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::sheep::Sheep>_>,tuple_<>_>,system_critters::butterfly::update_butterfly_states::closure_env_0>_>
            ();
  lVar22 = *in_RCX;
  lVar21 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Velocity>,ref_mut_<country_core::resources::critters::Butterfly>,ref_mut_<country_core::resources::critters::ButterflyAudio>,ref_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>
            ();
  fVar25 = __real_40400000;
  fVar11 = __real_40800000;
  fVar10 = __real_3f800000;
  fVar9 = __real_3f000000;
  local_178 = 8;
  local_148 = 0;
  local_140 = 0;
  local_118 = local_1f8;
  local_110 = local_1f0;
  fVar30 = __real_3f000000 * fVar4;
  if ((local_1d0 & 1) == 0) {
    if (local_1d8 != local_1e0) {
LAB_141d8563c:
      uVar16 = (ulonglong)*(uint *)(local_178 + 8 + local_1d8 * 0x10);
      goto LAB_141d85655;
    }
    if (puVar19 != puVar2) {
      do {
        if (*(longlong *)(*(longlong *)(lVar21 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0) != 0) {
          lVar15 = *(longlong *)(lVar21 + 0x100) + (ulonglong)*puVar19 * 0xa0;
          uVar16 = (ulonglong)*(uint *)(lVar15 + 0x94);
          lVar17 = *(longlong *)(lVar21 + 0x1a8) + uVar16 * 0x48;
          uVar16 = *(ulonglong *)(*(longlong *)(lVar21 + 0x1a8) + 0x40 + uVar16 * 0x48);
          if ((*(ulonglong *)(lVar22 + 0x110) < uVar16) &&
             (uVar8 = *(ulonglong *)
                       (*(longlong *)(lVar17 + 0x38) + *(ulonglong *)(lVar22 + 0x110) * 8),
             uVar8 != 0)) {
            local_118 = *(longlong *)(*(longlong *)(lVar17 + 0x18) + 0x10 + ~uVar8 * 0x30);
          }
          else {
            local_118 = 0;
          }
          if (*(ulonglong *)(lVar22 + 0x118) < uVar16) {
            uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar17 + 0x38) + *(ulonglong *)(lVar22 + 0x118) * 8);
            lVar21 = *(longlong *)(lVar17 + 0x18);
            if (uVar16 == 0) goto LAB_141d855ab;
            local_110 = *(longlong *)(lVar21 + 0x10 + ~uVar16 * 0x30);
          }
          else {
            lVar21 = *(longlong *)(lVar17 + 0x18);
LAB_141d855ab:
            local_110 = 0;
          }
          lVar22 = ~*(ulonglong *)(*(longlong *)(lVar17 + 0x38) + *(longlong *)(lVar22 + 0x120) * 8)
                   * 0x30;
          local_148 = *(longlong *)(lVar21 + 0x10 + lVar22);
          local_140 = *(longlong *)(lVar21 + 0x28 + lVar22);
          local_178 = *(longlong *)(lVar15 + 0x50);
          local_1d8 = 0;
          goto LAB_141d8563c;
        }
        puVar19 = puVar19 + 1;
      } while (puVar19 != puVar2);
    }
  }
  else {
    if (local_1d8 != local_1e0) {
LAB_141d85599:
      uVar16 = local_1d8 & 0xffffffff;
LAB_141d85655:
      puVar2 = (uint *)(local_148 + uVar16 * 0x18);
      local_118 = uVar16 * 0x30 + local_118;
      puVar3 = (undefined4 *)(local_140 + uVar16 * 4);
      country_core::systems::camera_rig::MainCamera::mode();
      if (extraout_AL_00 == '\x03') {
        country_core::systems::camera_rig::MainCamera::eye_position();
        fVar28 = *(float *)(local_118 + 0x10);
        fVar26 = *(float *)(local_118 + 0x18);
        fVar27 = (float)lVar14 - fVar28;
        fVar27 = SQRT((local_280 - fVar26) * (local_280 - fVar26) + fVar27 * fVar27);
        bVar24 = fVar25 == fVar27;
        bVar23 = fVar25 < fVar27;
      }
      else {
        fVar28 = *(float *)(local_118 + 0x10);
        fVar26 = *(float *)(local_118 + 0x18);
        fVar27 = (fVar26 - fVar27) * (fVar26 - fVar27) + (fVar28 - local_288) * (fVar28 - local_288)
        ;
        bVar24 = fVar11 == fVar27;
        bVar23 = fVar11 < fVar27;
      }
      lVar14 = 0;
      do {
        lVar22 = lVar14;
        if (local_160 << 3 == lVar22) break;
        fVar27 = fVar28 - (float)*(undefined8 *)(lVar22 + 0x3f);
        fVar29 = fVar26 - (float)((ulonglong)*(undefined8 *)(lVar22 + 0x3f) >> 0x20);
        lVar14 = lVar22 + 8;
      } while (fVar11 <= fVar29 * fVar29 + fVar27 * fVar27);
      fVar30 = (float)puVar2[4] - fVar30;
      if (fVar30 <= 0.0) {
        fVar30 = 0.0;
      }
      *puVar3 = 0;
      puVar2[4] = (uint)fVar30;
      if (!bVar23 && !bVar24 || local_160 << 3 != lVar22) {
        *puVar3 = 0;
        puVar2[4] = 0x3f800000;
        fVar30 = fVar10;
      }
      fVar27 = (float)puVar2[3];
      if ((fVar27 <= 0.0) || (0.0 < *(float *)(local_110 + uVar16 * 0xc + 4))) {
        if (fVar27 <= 0.0) {
          *puVar3 = 0;
          fVar27 = (float)puVar2[3];
          puVar2[3] = (uint)(fVar27 + fVar4);
          if (fVar27 + fVar4 <= 0.0) {
                    /* WARNING: Could not recover jumptable at 0x000141d85a20. Too many branches */
                    /* WARNING: Treating indirect jump as call */
            (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                      (-(uint)(0.0 < (float)puVar2[4]) & (uint)fVar11);
            return;
          }
          perchance::PerchanceContext::uniform_f32();
          fVar27 = extraout_XMM0_Da_00 * __real_3e99999a + __real_3e4ccccd;
          *puVar3 = 0;
          puVar2[3] = (uint)fVar27;
                    /* WARNING: Could not recover jumptable at 0x000141d8594f. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                    (-(uint)(0.0 < (float)puVar2[4]) & (uint)fVar11);
          return;
        }
LAB_141d85866:
        if (0.0 < fVar27) goto LAB_141d85955;
        bVar24 = false;
      }
      else {
        fVar28 = (float)((uint)(fVar9 - (float)puVar2[5]) & ___xmm_7fffffff7fffffff7fffffff7fffffff)
        ;
        if ((fVar28 < __real_3e4ccccd) || (__real_3ee66666 <= fVar28)) goto LAB_141d85866;
        *puVar3 = 0;
        fVar27 = (float)puVar2[3];
        puVar2[3] = (uint)(fVar27 - fVar4);
        bVar24 = true;
        if (0.0 < fVar27 - fVar4) {
                    /* WARNING: Could not recover jumptable at 0x000141d85850. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                    (-(uint)(0.0 < (float)puVar2[4]) & (uint)fVar11);
          return;
        }
      }
      perchance::PerchanceContext::uniform_f32();
      uVar12 = (uint)(extraout_XMM0_Da * __real_3fc00000 + fVar9) ^
               ___xmm_80000000800000008000000080000000;
      *puVar3 = 0;
      puVar2[3] = uVar12;
      fVar30 = (float)puVar2[4];
      if (bVar24) {
        fVar25 = fVar10;
      }
LAB_141d85955:
                    /* WARNING: Could not recover jumptable at 0x000141d85981. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_142f68008 + *(int *)(&DAT_142f68008 + (ulonglong)*puVar2 * 4)))
                (fVar25,fVar30,-(uint)(0.0 < fVar30) & (uint)fVar11);
      return;
    }
    if (puVar19 != puVar2) {
      do {
        if (*(longlong *)(*(longlong *)(lVar21 + 0x1a8) + 0x10 + (ulonglong)*puVar19 * 0x48) != 0) {
          lVar21 = *(longlong *)(lVar21 + 0x1a8) + (ulonglong)*puVar19 * 0x48;
          if ((*(ulonglong *)(lVar22 + 0x110) < *(ulonglong *)(lVar21 + 0x40)) &&
             (uVar16 = *(ulonglong *)
                        (*(longlong *)(lVar21 + 0x38) + *(ulonglong *)(lVar22 + 0x110) * 8),
             uVar16 != 0)) {
            local_118 = *(longlong *)(*(longlong *)(lVar21 + 0x18) + 0x10 + ~uVar16 * 0x30);
          }
          else {
            local_118 = 0;
          }
          if (*(ulonglong *)(lVar22 + 0x118) < *(ulonglong *)(lVar21 + 0x40)) {
            uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar21 + 0x38) + *(ulonglong *)(lVar22 + 0x118) * 8);
            lVar17 = *(longlong *)(lVar21 + 0x18);
            if (uVar16 == 0) goto LAB_141d8551a;
            local_110 = *(longlong *)(lVar17 + 0x10 + ~uVar16 * 0x30);
          }
          else {
            lVar17 = *(longlong *)(lVar21 + 0x18);
LAB_141d8551a:
            local_110 = 0;
          }
          lVar22 = ~*(ulonglong *)(*(longlong *)(lVar21 + 0x38) + *(longlong *)(lVar22 + 0x120) * 8)
                   * 0x30;
          local_148 = *(longlong *)(lVar17 + 0x10 + lVar22);
          local_140 = *(longlong *)(lVar17 + 0x28 + lVar22);
          local_1d8 = 0;
          goto LAB_141d85599;
        }
        puVar19 = puVar19 + 1;
      } while (puVar19 != puVar2);
    }
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  if (bVar24) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar18 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_141d85c5e:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_141d85c6a:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar18 = *plVar18 + 1;
  }
  return;
}

