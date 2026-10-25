// 0x142318fa0 move_critters
// SIG: void __cdecl move_critters(void)

void __cdecl system_critters::shared::move_critters(void)

{
  byte *pbVar1;
  float _Y;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  longlong lVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar13;
  int iVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *puVar15;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *in_RCX;
  ulonglong uVar16;
  longlong lVar17;
  float *pfVar18;
  longlong *plVar19;
  longlong *in_RDX;
  ulonglong uVar20;
  undefined8 uVar21;
  undefined1 (*pauVar22) [16];
  longlong *in_R8;
  undefined *puVar23;
  longlong lVar24;
  ulonglong uVar25;
  longlong lVar26;
  undefined8 *in_R9;
  undefined8 *puVar27;
  undefined4 *puVar28;
  uint *puVar29;
  uint *puVar30;
  float fVar31;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar32;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  undefined8 *in_stack_00000040;
  undefined8 local_348;
  float fStack_340;
  float fStack_33c;
  float local_330;
  float fStack_32c;
  float fStack_328;
  undefined4 uStack_324;
  uint *local_320;
  int local_318;
  longlong local_308;
  longlong *local_2f8;
  longlong *local_2f0;
  longlong local_2e8;
  byte local_2e0;
  byte local_2d0;
  byte local_2c0;
  longlong local_2b8;
  longlong local_2a8;
  undefined4 local_29c;
  longlong local_298;
  longlong local_290;
  longlong local_288;
  longlong local_278;
  undefined4 local_26c;
  byte local_268;
  byte local_260;
  undefined8 local_250;
  undefined8 local_248;
  undefined8 local_230;
  float local_228;
  undefined8 local_220;
  undefined8 local_218;
  undefined8 local_210;
  undefined8 local_208;
  undefined8 local_200;
  undefined4 *local_1f8;
  undefined4 *local_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined8 local_1d8;
  float fStack_1d0;
  float fStack_1cc;
  longlong local_1c8;
  uint *local_1b8;
  longlong local_1b0;
  undefined4 *local_1a8;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined4 local_194;
  longlong local_190;
  undefined *local_188;
  undefined4 local_180;
  float fStack_17c;
  float fStack_178;
  float local_174;
  longlong local_170;
  longlong local_168;
  longlong *local_160;
  longlong local_158;
  longlong local_150;
  longlong local_148;
  undefined4 local_13c;
  undefined8 local_138;
  float local_130;
  longlong *local_128;
  longlong *local_120;
  longlong local_118;
  longlong local_110;
  byte local_101;
  uint local_100;
  uint local_fc;
  uint local_f8;
  uint local_f4;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_1c8 = 0;
  }
  else {
    uVar20 = 0x26;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar20 != 0) {
        if (uVar20 < 0x26) {
          if ((char)(&DAT_14304af35)[uVar20] < -0x40) {
LAB_142319069:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 != 0x26) goto LAB_142319069;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar20 != 0xfffffffffffffffe)) {
        if (uVar20 + 2 < 0x26) {
          if ((char)(&UNK_14304af37)[uVar20] < -0x40) {
LAB_1423190c7:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 + 2 != 0x26) goto LAB_1423190c7;
      }
    }
    puVar23 = &DAT_14304b04d;
    do {
      bVar2 = puVar23[-1];
      uVar13 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar23[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar23[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar23 + -4;
            puVar23 = puVar23 + -4;
            uVar13 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar23 = puVar23 + -3;
            uVar13 = bVar4 & 0xf;
          }
          uVar13 = bVar3 & 0x3f | uVar13 << 6;
        }
        else {
          puVar23 = puVar23 + -2;
          uVar13 = bVar3 & 0x1f;
        }
        uVar13 = (uint)(bVar2 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_142319194;
LAB_1423191a5:
        if (puVar23 == &UNK_14304b027) break;
        if (puVar23 + -0x14304b027 < (undefined *)0x25) {
          if (-0x41 < (char)puVar23[1]) break;
        }
        else if (puVar23 + -0x14304b027 == (undefined *)0x25) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar23 = puVar23 + -1;
      if (uVar13 == 0x5c) goto LAB_1423191a5;
LAB_142319194:
      if (uVar13 == 0x2f) goto LAB_1423191a5;
    } while (puVar23 != &DAT_14304b028);
    local_128 = in_R8;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar19 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar19 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
LAB_14231a32e:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar19 != 0) {
LAB_14231a33a:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar19 = -1;
    local_120 = plVar19;
    puffin::ThreadProfiler::begin_scope();
    *local_120 = *local_120 + 1;
    local_1c8 = 1;
    in_R8 = local_128;
  }
  _Y = *(float *)(*in_RDX + 0x28);
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    local_188 = &anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650;
    local_180 = 0x3f;
    fStack_17c = 0.0;
    if ((extraout_RAX_02[3] != 0x3f) ||
       (iVar14 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                        (void *)extraout_RAX_02[2],0x3f), iVar14 != 0)) {
      local_348 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    puVar15 = (undefined8 *)(**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (puVar15 != (undefined8 *)0x0) {
      local_208 = *puVar15;
      local_200 = puVar15[1];
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      puVar15 = (undefined8 *)(extraout_RAX_03 + 0x20);
      puVar27 = (undefined8 *)(extraout_RAX_03 + 0x28);
      if (*(char *)(extraout_RAX_03 + 0x298) == '\0') {
        puVar27 = (undefined8 *)(extraout_RAX_03 + 0x10);
        puVar15 = (undefined8 *)(extraout_RAX_03 + 8);
      }
      local_248 = *puVar27;
      local_250 = *puVar15;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      puVar15 = (undefined8 *)(extraout_RAX_04 + 0x20);
      puVar27 = (undefined8 *)(extraout_RAX_04 + 0x28);
      if (*(char *)(extraout_RAX_04 + 0x298) == '\0') {
        puVar27 = (undefined8 *)(extraout_RAX_04 + 0x10);
        puVar15 = (undefined8 *)(extraout_RAX_04 + 8);
      }
      local_218 = *puVar27;
      local_220 = *puVar15;
      local_190 = *in_RCX;
      local_128 = (longlong *)in_RCX[1];
      bevy_ecs::query::iter::QueryIterationCursor::
      init<tuple_<ref_mut_<utils::components::transform::Transform>,ref_<enum2_<country_core::resources::critters::CritterLocomotion>_>,ref_<country_core::resources::critters::Velocity>,enum2_<core::option::Option<ref_<country_core::resources::critters::CritterVelocityMultiplier>_>_>,enum2_<core::option::Option<ref_<country_core::resources::critters::Duck>_>_>,enum2_<core::option::Option<ref_<country_core::resources::critters::sheep::Sheep>_>_>,enum2_<core::option::Option<ref_mut_<country_core::resources::critters::BirdSpots>_>_>_>,tuple_<>_>
                ();
      fVar31 = powf(0.01,_Y);
      puVar30 = (uint *)CONCAT44(uStack_324,fStack_328);
      local_1b8 = local_320;
      local_1b0 = local_308;
      local_120 = local_2f0;
      local_148 = local_2e8;
      local_f8 = (uint)local_2e0;
      local_100 = (uint)local_2d0;
      local_fc = (uint)local_2c0;
      local_170 = local_2b8;
      local_1e8 = *in_R8;
      local_210 = *in_R9;
      local_1e0 = *in_stack_00000040;
      fVar38 = 1.0 - fVar31;
      local_101 = local_260;
      local_168 = local_2a8;
      local_13c = local_29c;
      local_118 = local_298;
      local_110 = local_290;
      local_158 = local_288;
      local_150 = local_278;
      local_194 = local_26c;
      local_f4 = (uint)local_268;
      plVar19 = local_2f8;
LAB_142319506:
      if ((local_101 & 1) == 0) {
        if (local_120 == plVar19) {
          do {
            if (puVar30 == local_1b8) goto LAB_14231a245;
            uVar13 = *puVar30;
            puVar30 = puVar30 + 1;
            plVar19 = *(longlong **)(local_128[0x20] + 0x58 + (ulonglong)uVar13 * 0xa0);
          } while (plVar19 == (longlong *)0x0);
          lVar24 = local_128[0x20] + (ulonglong)uVar13 * 0xa0;
          uVar16 = (ulonglong)*(uint *)(lVar24 + 0x94);
          lVar17 = local_128[0x35];
          uVar20 = *(ulonglong *)(lVar17 + 0x40 + uVar16 * 0x48);
          lVar5 = *(longlong *)(lVar17 + 0x18 + uVar16 * 0x48);
          lVar17 = *(longlong *)(lVar17 + 0x38 + uVar16 * 0x48);
          if ((*(ulonglong *)(local_190 + 0x118) < uVar20) &&
             (uVar16 = *(ulonglong *)(lVar17 + *(ulonglong *)(local_190 + 0x118) * 8), uVar16 != 0))
          {
            local_118 = *(longlong *)(lVar5 + 0x10 + ~uVar16 * 0x30);
            uVar16 = *(ulonglong *)(local_190 + 0x120);
            if (uVar20 <= uVar16) goto LAB_1423197af;
LAB_1423196e7:
            uVar16 = *(ulonglong *)(lVar17 + uVar16 * 8);
            if (uVar16 == 0) goto LAB_1423197af;
            local_110 = *(longlong *)(lVar5 + 0x10 + ~uVar16 * 0x30);
            uVar16 = *(ulonglong *)(local_190 + 0x128);
            uVar25 = *(ulonglong *)(lVar24 + 0x88);
            if (uVar16 < uVar25) goto LAB_1423197d1;
LAB_1423198ed:
            local_f8 = 0;
            uVar20 = *(ulonglong *)(local_190 + 0x130);
            if (uVar20 < uVar25) goto LAB_142319907;
LAB_142319a11:
            local_100 = 0;
            uVar20 = *(ulonglong *)(local_190 + 0x138);
            if (uVar25 <= uVar20) goto LAB_14231992e;
LAB_142319a2b:
            local_fc = (uint)CONCAT71((int7)(uVar20 >> 8),
                                      *(longlong *)(*(longlong *)(lVar24 + 0x80) + uVar20 * 8) != 0)
            ;
            uVar20 = *(ulonglong *)(local_190 + 0x140);
            if (uVar25 <= uVar20) goto LAB_142319a8f;
LAB_142319a4e:
            if (*(longlong *)(*(longlong *)(lVar24 + 0x80) + uVar20 * 8) == 0) goto LAB_142319a8f;
            lVar26 = ~*(ulonglong *)(lVar17 + uVar20 * 8) * 0x30;
            local_158 = *(longlong *)(lVar5 + 0x10 + lVar26);
            local_150 = *(longlong *)(lVar5 + 0x28 + lVar26);
            local_f4 = (uint)CONCAT71((int7)((ulonglong)local_150 >> 8),1);
          }
          else {
            local_118 = 0;
            uVar16 = *(ulonglong *)(local_190 + 0x120);
            if (uVar16 < uVar20) goto LAB_1423196e7;
LAB_1423197af:
            local_110 = 0;
            uVar16 = *(ulonglong *)(local_190 + 0x128);
            uVar25 = *(ulonglong *)(lVar24 + 0x88);
            if (uVar25 <= uVar16) goto LAB_1423198ed;
LAB_1423197d1:
            if (*(longlong *)(*(longlong *)(lVar24 + 0x80) + uVar16 * 8) == 0) goto LAB_1423198ed;
            local_f8 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar24 + 0x80) >> 8),1);
            if ((uVar20 <= uVar16) || (uVar20 = *(ulonglong *)(lVar17 + uVar16 * 8), uVar20 == 0)) {
              local_148 = 0;
              uVar20 = *(ulonglong *)(local_190 + 0x130);
              if (uVar20 < uVar25) goto LAB_142319907;
              goto LAB_142319a11;
            }
            local_148 = *(longlong *)(lVar5 + 0x10 + ~uVar20 * 0x30);
            uVar20 = *(ulonglong *)(local_190 + 0x130);
            if (uVar25 <= uVar20) goto LAB_142319a11;
LAB_142319907:
            local_100 = (uint)CONCAT71((int7)(uVar20 >> 8),
                                       *(longlong *)(*(longlong *)(lVar24 + 0x80) + uVar20 * 8) != 0
                                      );
            uVar20 = *(ulonglong *)(local_190 + 0x138);
            if (uVar20 < uVar25) goto LAB_142319a2b;
LAB_14231992e:
            local_fc = 0;
            uVar20 = *(ulonglong *)(local_190 + 0x140);
            if (uVar20 < uVar25) goto LAB_142319a4e;
LAB_142319a8f:
            local_f4 = 0;
          }
          lVar17 = ~*(ulonglong *)(lVar17 + *(longlong *)(local_190 + 0x110) * 8) * 0x30;
          local_170 = *(longlong *)(lVar5 + 0x10 + lVar17);
          local_168 = *(longlong *)(lVar5 + 0x28 + lVar17);
          local_1b0 = *(longlong *)(lVar24 + 0x50);
          local_120 = (longlong *)0x0;
        }
        uVar20 = (ulonglong)*(uint *)(local_1b0 + 8 + (longlong)local_120 * 0x10);
        pauVar22 = (undefined1 (*) [16])(uVar20 * 0x30 + local_170);
        local_1a8 = (undefined4 *)(local_168 + uVar20 * 4);
        puVar29 = (uint *)(local_118 + uVar20 * 8);
        puVar15 = (undefined8 *)(local_110 + uVar20 * 0xc);
        pfVar18 = (float *)(local_148 + uVar20 * 4);
        if ((local_f8 & 1) == 0) {
          pfVar18 = (float *)0x0;
        }
        puVar28 = local_1f0;
        if ((local_f4 & 1) == 0) goto LAB_142319b79;
        local_160 = (longlong *)(local_158 + uVar20 * 0x18);
        puVar28 = (undefined4 *)(local_150 + uVar20 * 4);
        local_1f0 = puVar28;
      }
      else {
        if (local_120 == plVar19) {
          do {
            if (puVar30 == local_1b8) goto LAB_14231a245;
            uVar13 = *puVar30;
            puVar30 = puVar30 + 1;
            plVar19 = *(longlong **)(local_128[0x35] + 0x10 + (ulonglong)uVar13 * 0x48);
          } while (plVar19 == (longlong *)0x0);
          lVar17 = local_128[0x35] + (ulonglong)uVar13 * 0x48;
          uVar20 = *(ulonglong *)(lVar17 + 0x40);
          lVar5 = *(longlong *)(lVar17 + 0x18);
          lVar17 = *(longlong *)(lVar17 + 0x38);
          if ((*(ulonglong *)(local_190 + 0x118) < uVar20) &&
             (uVar16 = *(ulonglong *)(lVar17 + *(ulonglong *)(local_190 + 0x118) * 8), uVar16 != 0))
          {
            local_118 = *(longlong *)(lVar5 + 0x10 + ~uVar16 * 0x30);
            uVar16 = *(ulonglong *)(local_190 + 0x120);
            if (uVar20 <= uVar16) goto LAB_142319727;
LAB_142319696:
            uVar16 = *(ulonglong *)(lVar17 + uVar16 * 8);
            if (uVar16 == 0) goto LAB_142319727;
            local_110 = *(longlong *)(lVar5 + 0x10 + ~uVar16 * 0x30);
            uVar16 = *(ulonglong *)(local_190 + 0x128);
            if (uVar16 < uVar20) goto LAB_142319742;
LAB_14231982f:
            local_f8 = 0;
            uVar16 = *(ulonglong *)(local_190 + 0x130);
            if (uVar16 < uVar20) goto LAB_142319849;
LAB_142319780:
            local_100 = 0;
            uVar16 = *(ulonglong *)(local_190 + 0x138);
            if (uVar20 <= uVar16) goto LAB_142319869;
LAB_14231979a:
            local_fc = (uint)CONCAT71((int7)(uVar16 >> 8),*(longlong *)(lVar17 + uVar16 * 8) != 0);
          }
          else {
            local_118 = 0;
            uVar16 = *(ulonglong *)(local_190 + 0x120);
            if (uVar16 < uVar20) goto LAB_142319696;
LAB_142319727:
            local_110 = 0;
            uVar16 = *(ulonglong *)(local_190 + 0x128);
            if (uVar20 <= uVar16) goto LAB_14231982f;
LAB_142319742:
            uVar16 = *(ulonglong *)(lVar17 + uVar16 * 8);
            if (uVar16 == 0) goto LAB_14231982f;
            local_148 = *(longlong *)(lVar5 + 0x10 + ~uVar16 * 0x30);
            local_f8 = (uint)CONCAT71((int7)((ulonglong)local_148 >> 8),1);
            uVar16 = *(ulonglong *)(local_190 + 0x130);
            if (uVar20 <= uVar16) goto LAB_142319780;
LAB_142319849:
            local_100 = (uint)CONCAT71((int7)(uVar16 >> 8),*(longlong *)(lVar17 + uVar16 * 8) != 0);
            uVar16 = *(ulonglong *)(local_190 + 0x138);
            if (uVar16 < uVar20) goto LAB_14231979a;
LAB_142319869:
            local_fc = 0;
          }
          lVar24 = ~*(ulonglong *)(lVar17 + *(longlong *)(local_190 + 0x110) * 8) * 0x30;
          local_170 = *(longlong *)(lVar5 + 0x10 + lVar24);
          local_168 = *(longlong *)(lVar5 + 0x28 + lVar24);
          if ((*(ulonglong *)(local_190 + 0x140) < uVar20) &&
             (uVar20 = *(ulonglong *)(lVar17 + *(ulonglong *)(local_190 + 0x140) * 8), uVar20 != 0))
          {
            lVar17 = ~uVar20 * 0x30;
            local_158 = *(longlong *)(lVar5 + 0x10 + lVar17);
            local_150 = *(longlong *)(lVar5 + 0x28 + lVar17);
            local_f4 = (uint)CONCAT71((int7)((ulonglong)local_150 >> 8),1);
            local_120 = (longlong *)0x0;
          }
          else {
            local_120 = (longlong *)0x0;
            local_f4 = 0;
          }
        }
        uVar20 = (ulonglong)local_120 & 0xffffffff;
        pauVar22 = (undefined1 (*) [16])(uVar20 * 0x30 + local_170);
        local_1a8 = (undefined4 *)(local_168 + uVar20 * 4);
        puVar29 = (uint *)(local_118 + uVar20 * 8);
        puVar15 = (undefined8 *)(local_110 + uVar20 * 0xc);
        pfVar18 = (float *)(local_148 + uVar20 * 4);
        if ((local_f8 & 1) == 0) {
          pfVar18 = (float *)0x0;
        }
        puVar28 = local_1f8;
        if ((local_f4 & 1) == 0) {
LAB_142319b79:
          local_160 = (longlong *)0x0;
        }
        else {
          local_160 = (longlong *)(local_158 + uVar20 * 0x18);
          puVar28 = (undefined4 *)(local_150 + uVar20 * 4);
          local_1f8 = puVar28;
        }
      }
      local_120 = (longlong *)((longlong)local_120 + 1);
      fVar32 = *(float *)(pauVar22[1] + 4);
      if (fVar32 <= 0.0) {
        fVar32 = 0.0;
      }
      *local_1a8 = local_13c;
      *(float *)(pauVar22[1] + 4) = fVar32;
      fVar32 = 1.0;
      if (pfVar18 != (float *)0x0) goto code_r0x000142319bb2;
      goto LAB_142319bc1;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
code_r0x000142319bb2:
  fVar32 = *pfVar18;
  if ((fVar32 != 0.0) || (NAN(fVar32))) {
LAB_142319bc1:
    local_130 = *(float *)(puVar15 + 1) * _Y * fVar32;
    local_138 = CONCAT44(fVar32 * (float)((ulonglong)*puVar15 >> 0x20) * _Y,
                         fVar32 * (float)*puVar15 * _Y);
    local_1d8 = puVar28;
    ::utils::geometry::circle::Circle2d::new_rotated();
    puVar15 = &local_348;
    ::utils::geometry::line_relation::LineInterval::line_segment();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    if (*puVar29 == 2) {
      uVar21 = local_200;
      country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz();
      puVar15 = (undefined8 *)CONCAT71((int7)((ulonglong)uVar21 >> 8),0.0 < extraout_XMM0_Da);
      if ((extraout_AL_00 != '\0' || 0.0 < extraout_XMM0_Da) ||
         ((extraout_XMM0_Da <= 0.0 & (byte)local_100) != 0)) goto LAB_142319506;
    }
    else if ((extraout_XMM0_Da <= 0.0) && ((local_100 & 1) != 0)) goto LAB_142319506;
    if (((local_fc & 1) != 0) || ((local_100 & 1) != 0)) {
      if ((6 < *puVar29) || ((99U >> (*puVar29 & 0x1f) & 1) == 0)) {
        local_180 = *(undefined4 *)(pauVar22[1] + 8);
        local_188 = *(undefined **)pauVar22[1];
        fStack_17c = (float)local_138;
        fStack_178 = (float)((ulonglong)local_138 >> 0x20);
        local_174 = 1.0 / SQRT(fStack_178 * fStack_178 + fStack_17c * fStack_17c +
                               local_130 * local_130);
        fStack_17c = local_174 * fStack_17c;
        fStack_178 = local_174 * fStack_178;
        local_174 = local_174 * local_130;
        country_core::systems::collision::world::RaycastWorld::legacy_spherecast();
        if ((local_318 != 2) &&
           (fVar32 = fStack_32c - *(float *)(pauVar22[1] + 4),
           fVar36 = local_330 - *(float *)pauVar22[1],
           fVar35 = fStack_328 - *(float *)(pauVar22[1] + 8),
           SQRT(fVar35 * fVar35 + fVar32 * fVar32 + fVar36 * fVar36) < 1.0)) goto LAB_142319506;
        if (((local_fc & 1) != 0) && (*puVar29 != 4)) {
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          if ((extraout_XMM0_Da_01 <= 0.0) && (0.0 < extraout_XMM0_Da_02)) goto LAB_142319506;
        }
      }
LAB_142319dbf:
      *local_1a8 = local_13c;
      *(ulonglong *)pauVar22[1] =
           CONCAT44((float)((ulonglong)*(undefined8 *)pauVar22[1] >> 0x20) +
                    (float)((ulonglong)local_138 >> 0x20),
                    (float)*(undefined8 *)pauVar22[1] + (float)local_138);
      *(float *)(pauVar22[1] + 8) = local_130 + *(float *)(pauVar22[1] + 8);
      ::utils::swizzlers::impl$3::x0z();
      fVar32 = (float)local_348;
      fVar36 = (float)((ulonglong)local_348 >> 0x20);
      fVar35 = 1.0 / SQRT(fStack_340 * fStack_340 + fVar36 * fVar36 + fVar32 * fVar32);
      if ((int)fVar35 - 1U < 0x7fffff ||
          (int)ABS(fVar35) - 0x800000U < 0x7f000000 && -1 < (int)fVar35) {
        local_228 = fStack_340 * fVar35;
        local_230 = CONCAT44(fVar36 * fVar35,fVar32 * fVar35);
        local_188 = (undefined *)0x8000000080000000;
        local_180 = 0xbf800000;
        glam::f32::sse2::quat::Quat::from_rotation_arc();
        fVar32 = fStack_33c * *(float *)(*pauVar22 + 0xc) +
                 local_348._4_4_ * *(float *)(*pauVar22 + 4) +
                 fStack_340 * *(float *)(*pauVar22 + 8) + (float)local_348 * *(float *)*pauVar22;
        auVar33._0_4_ =
             (float)((uint)fVar32 & 0x80000000 ^ (uint)*(float *)*pauVar22) * fVar31 +
             fVar38 * (float)local_348;
        auVar33._4_4_ =
             (float)((uint)fVar32 & 0x80000000 ^ (uint)*(float *)(*pauVar22 + 4)) * fVar31 +
             fVar38 * local_348._4_4_;
        auVar33._8_4_ =
             (float)((uint)fVar32 & 0x80000000 ^ (uint)*(float *)(*pauVar22 + 8)) * fVar31 +
             fVar38 * fStack_340;
        auVar33._12_4_ =
             (float)((uint)fVar32 & 0x80000000 ^ (uint)*(float *)(*pauVar22 + 0xc)) * fVar31 +
             fVar38 * fStack_33c;
        auVar34._0_4_ =
             auVar33._12_4_ * auVar33._12_4_ + auVar33._4_4_ * auVar33._4_4_ +
             auVar33._8_4_ * auVar33._8_4_ + auVar33._0_4_ * auVar33._0_4_;
        auVar34._4_4_ = auVar34._0_4_;
        auVar34._8_4_ = auVar34._0_4_;
        auVar34._12_4_ = auVar34._0_4_;
        auVar34 = sqrtps(auVar34,auVar34);
        auVar33 = divps(auVar33,auVar34);
        *local_1a8 = local_13c;
        *pauVar22 = auVar33;
      }
      if (local_160 != (longlong *)0x0) {
        fVar32 = *(float *)(pauVar22[1] + 0xc);
        fVar36 = *(float *)pauVar22[2];
        fVar35 = *(float *)(pauVar22[2] + 4);
        fVar6 = *(float *)*pauVar22;
        fVar7 = *(float *)(*pauVar22 + 4);
        fVar8 = *(float *)(*pauVar22 + 8);
        fVar9 = *(float *)(*pauVar22 + 0xc);
        fVar10 = *(float *)pauVar22[1];
        fVar11 = *(float *)(pauVar22[1] + 4);
        fVar12 = *(float *)(pauVar22[1] + 8);
        *local_1d8 = local_194;
        lVar17 = local_160[1];
        if (lVar17 != 0) {
          fVar40 = fVar8 + fVar8;
          fVar41 = fVar7 + fVar7;
          local_1d8 = (undefined4 *)CONCAT44(fVar38,fVar38);
          fVar39 = fVar9 * (fVar6 + fVar6);
          local_1a8 = (undefined4 *)(ulonglong)(uint)_Y;
          fVar37 = fVar6 * (fVar6 + fVar6);
          lVar5 = *local_160;
          lVar24 = 0;
          do {
            *(ulonglong *)(lVar5 + 8 + lVar24) =
                 CONCAT44(fVar11 + (fVar40 * fVar7 - fVar39) * fVar35 * -0.495 +
                                   (1.0 - (fVar37 + fVar40 * fVar8)) * fVar36 * 1.567 +
                                   (fVar9 * fVar40 + fVar41 * fVar6) * fVar32 * 0.0,
                          fVar10 + (fVar40 * fVar6 + fVar9 * fVar41) * fVar35 * -0.495 +
                                   (fVar41 * fVar6 - fVar9 * fVar40) * fVar36 * 1.567 +
                                   (1.0 - (fVar40 * fVar8 + fVar7 * fVar41)) * fVar32 * 0.0);
            *(float *)(lVar5 + 0x10 + lVar24) =
                 fVar12 + (1.0 - (fVar37 + fVar7 * fVar41)) * fVar35 * -0.495 +
                          (fVar40 * fVar7 + fVar39) * fVar36 * 1.567 +
                          (fVar40 * fVar6 - fVar9 * fVar41) * fVar32 * 0.0;
            lVar24 = lVar24 + 0x18;
            uStack_19c = 0;
            uStack_1a0 = 0;
            fStack_1cc = fVar38;
            fStack_1d0 = fVar38;
          } while (lVar17 * 0x18 != lVar24);
        }
      }
      goto LAB_142319506;
    }
    country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query();
    lVar17 = extraout_RAX_05;
LAB_142319d00:
    do {
      if (lVar17 == (longlong)puVar15 * 0x38 + extraout_RAX_05) goto LAB_142319dbf;
      lVar17 = lVar17 + 0x38;
      if (*puVar29 != 3) {
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_06 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable();
        if (extraout_XMM0_Da_00 < *(float *)(pauVar22[1] + 4)) goto LAB_142319d00;
      }
      ::utils::geometry::circle::Circle2d::new_rotated();
      ::utils::geometry::line_relation::LineInterval::line_segment();
      ::utils::geometry::line_relation::LineInterval::relate();
    } while ((int)local_188 != 0);
  }
  goto LAB_142319506;
LAB_14231a245:
  if (local_1c8 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_07 == 1) {
      plVar19 = extraout_RAX_07 + 1;
    }
    else if ((*extraout_RAX_07 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar19 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
    goto LAB_14231a32e;
    if (*plVar19 != 0) goto LAB_14231a33a;
    *plVar19 = -1;
    local_128 = plVar19;
    puffin::ThreadProfiler::end_scope();
    *local_128 = *local_128 + 1;
  }
  return;
}

