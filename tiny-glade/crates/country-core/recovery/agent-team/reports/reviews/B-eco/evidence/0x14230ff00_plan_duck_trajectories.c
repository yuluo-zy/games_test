
void __cdecl system_critters::duck::plan_duck_trajectories(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  uint *puVar3;
  undefined8 uVar4;
  byte bVar5;
  byte bVar6;
  longlong lVar7;
  float *pfVar8;
  longlong lVar9;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  uint uVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar11;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  ulonglong uVar12;
  longlong *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong lVar13;
  uint *puVar14;
  undefined8 *puVar15;
  undefined4 *puVar16;
  longlong *plVar17;
  ulonglong uVar18;
  char *pcVar19;
  undefined4 *puVar20;
  longlong lVar21;
  longlong *plVar22;
  byte bVar23;
  longlong lVar24;
  ulonglong uVar25;
  undefined1 *puVar26;
  longlong *in_R8;
  undefined *puVar27;
  undefined8 *puVar28;
  undefined8 *in_R9;
  longlong lVar29;
  longlong lVar30;
  code *pcVar31;
  bool bVar32;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float fVar33;
  undefined4 extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  undefined4 extraout_XMM0_Da_04;
  undefined4 uVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000050;
  undefined8 local_4d8;
  float fStack_4d0;
  float fStack_4cc;
  float fStack_4c8;
  float fStack_4c4;
  ulonglong local_448;
  longlong local_440;
  ulonglong local_438;
  float local_3ec;
  undefined8 local_3d8;
  undefined8 uStack_3d0;
  undefined8 local_3c8;
  undefined8 uStack_3c0;
  float local_3b8;
  undefined4 uStack_3b4;
  undefined4 uStack_3b0;
  undefined4 uStack_3ac;
  int local_3a8;
  undefined4 local_2d8;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  undefined4 local_2b0;
  undefined8 local_2ac;
  undefined4 local_2a4;
  longlong local_298;
  char *local_290;
  undefined8 local_288;
  undefined4 local_280;
  undefined4 local_27c;
  undefined4 local_278;
  undefined8 local_270;
  undefined8 uStack_268;
  undefined4 local_260;
  undefined1 local_25c;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined4 local_240;
  undefined8 local_23c;
  undefined4 local_234;
  undefined8 *local_220;
  undefined8 *local_218;
  longlong local_210;
  longlong local_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 local_1ec [3];
  undefined8 local_1d0;
  undefined8 local_1c8;
  longlong local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  longlong local_1a0;
  undefined1 *local_190;
  undefined8 *local_188;
  undefined1 *local_180;
  undefined8 local_168;
  float local_160;
  longlong local_158;
  longlong local_150;
  longlong local_148;
  longlong local_140;
  undefined4 local_134;
  char *local_130;
  longlong local_128;
  undefined8 *local_120;
  longlong *local_118;
  longlong local_110;
  undefined1 *local_108;
  undefined8 *local_100;
  char local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
    puffin::are_scopes_on();
    local_100 = in_R9;
    local_f2 = extraout_AL;
    if (extraout_AL == '\0') {
      local_1c8 = 0;
    }
    else {
      uVar18 = 0x2d;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX == 1) {
        if (uVar18 != 0) {
          if (uVar18 < 0x2d) {
            if ((char)(&DAT_14304a97f)[uVar18] < -0x40) {
LAB_14230ffe5:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar18 != 0x2d) goto LAB_14230ffe5;
        }
        core::str::impl$0::rfind<ref_<str_>_>();
        if ((extraout_RAX_00 == 1) && (uVar18 != 0xfffffffffffffffe)) {
          if (uVar18 + 2 < 0x2d) {
            if ((char)(&UNK_14304a981)[uVar18] < -0x40) {
LAB_142310043:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar18 + 2 != 0x2d) goto LAB_142310043;
        }
      }
      puVar27 = &DAT_14304aa9b;
      do {
        bVar23 = puVar27[-1];
        uVar10 = (uint)(char)bVar23;
        if ((char)bVar23 < '\0') {
          bVar5 = puVar27[-2];
          if ((char)bVar5 < -0x40) {
            bVar6 = puVar27[-3];
            if ((char)bVar6 < -0x40) {
              pbVar1 = puVar27 + -4;
              puVar27 = puVar27 + -4;
              uVar10 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              puVar27 = puVar27 + -3;
              uVar10 = bVar6 & 0xf;
            }
            uVar10 = bVar5 & 0x3f | uVar10 << 6;
          }
          else {
            puVar27 = puVar27 + -2;
            uVar10 = bVar5 & 0x1f;
          }
          uVar10 = (uint)(bVar23 & 0x3f) | uVar10 << 6;
          if (uVar10 != 0x5c) goto LAB_142310104;
LAB_142310116:
          if (puVar27 == &UNK_14304aa77) break;
          if (puVar27 + -0x14304aa77 < (undefined *)0x23) {
            if (-0x41 < (char)puVar27[1]) break;
          }
          else if (puVar27 + -0x14304aa77 == (undefined *)0x23) break;
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
        puVar27 = puVar27 + -1;
        if (uVar10 == 0x5c) goto LAB_142310116;
LAB_142310104:
        if (uVar10 == 0x2f) goto LAB_142310116;
      } while (puVar27 != &DAT_14304aa78);
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_01 == 1) {
        plVar17 = extraout_RAX_01 + 1;
      }
      else if ((*extraout_RAX_01 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar17 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
      goto LAB_1423114ee;
      if (*plVar17 != 0) goto LAB_142311543;
      *plVar17 = -1;
      local_118 = plVar17;
      puffin::ThreadProfiler::begin_scope();
      *local_118 = *local_118 + 1;
      local_1c8 = 1;
    }
    puVar15 = local_100;
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    local_110 = *in_stack_00000030;
    local_118 = (longlong *)*in_R8;
    country_core::game_config::developer_hotkeys_enabled();
    if (extraout_AL_00 != '\0') {
      local_3d8 = (undefined8 *)CONCAT44(local_3d8._4_4_,0x1f);
      bevy_input::button_input::ButtonInput::just_pressed<enum2_<bevy_input::keyboard::KeyCode>_>();
      if (extraout_AL_01 != '\0') {
        country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
        std::alloc::__default_lib_allocator::__rust_alloc();
        lVar21 = local_110;
        if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *(float *)(extraout_RAX_02 + 1) = local_160;
        *extraout_RAX_02 = local_168;
        local_190 = (undefined1 *)0x1;
        local_180 = (undefined1 *)0x1;
        local_188 = extraout_RAX_02;
        while (local_108 = local_180, local_180 < (undefined1 *)0x3) {
          perchance::PerchanceContext::uniform_f32();
          uVar34 = extraout_XMM0_Da;
          ::utils::golden_spiral();
          local_4d8 = CONCAT44(uVar34,extraout_XMM0_Da_00);
          ::utils::swizzlers::impl$0::x0y();
          puVar26 = local_180;
          fVar33 = SUB84(local_3d8,0);
          uVar18 = (ulonglong)local_3d8 >> 0x20;
          fVar36 = (float)uStack_3d0 + local_160;
          if (local_180 == local_190) {
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          *(ulonglong *)((longlong)local_188 + (longlong)puVar26 * 0xc) =
               CONCAT44((float)uVar18 + (float)((ulonglong)local_168 >> 0x20),
                        fVar33 + (float)local_168);
          *(float *)((longlong)local_188 + (longlong)puVar26 * 0xc + 8) = fVar36;
          local_180 = puVar26 + 1;
        }
        uStack_3c0 = (longlong)local_188 + (longlong)local_180 * 0xc;
        local_3d8 = local_188;
        uStack_3d0 = local_188;
        local_3c8 = local_190;
        alloc::vec::in_place_collect::impl$1::
        from_iter<system_critters::duck::Trajectory,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,system_critters::spawn_ducks::spawn_ducks::closure_env_1>_>
                  ();
        perchance::PerchanceContext::uniform_f32();
        uVar34 = 0x3f800000;
        ::utils::uniform_sample_circle();
        puVar15 = local_100;
        local_3c8 = (undefined1 *)CONCAT44(fStack_4c4,fStack_4c8);
        uStack_3d0 = (undefined8 *)CONCAT44(fStack_4cc,fStack_4d0);
        local_3d8 = (undefined8 *)local_4d8;
        uStack_3c0 = CONCAT44(uVar34,extraout_XMM0_Da_01);
        uStack_3b4 = 0;
        uStack_3b0 = 0;
        lVar7 = *(longlong *)(lVar21 + 0x20);
        if (lVar7 == *(longlong *)(lVar21 + 0x10)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                    ();
        }
        lVar11 = *(longlong *)(lVar21 + 0x18);
        lVar13 = lVar7 * 0x30;
        puVar28 = (undefined8 *)(lVar11 + 0x20 + lVar13);
        *puVar28 = CONCAT44(uStack_3b4,local_3b8);
        puVar28[1] = CONCAT44(uStack_3ac,uStack_3b0);
        puVar16 = (undefined4 *)(lVar11 + 0x10 + lVar13);
        *puVar16 = (undefined4)local_3c8;
        puVar16[1] = local_3c8._4_4_;
        puVar16[2] = (float)uStack_3c0;
        puVar16[3] = uStack_3c0._4_4_;
        *(longlong *)(lVar11 + lVar13) = (longlong)local_3d8;
        ((longlong *)(lVar11 + lVar13))[1] = (longlong)uStack_3d0;
        *(longlong *)(lVar21 + 0x20) = lVar7 + 1;
      }
    }
    country_core::game_config::developer_hotkeys_enabled();
    if (extraout_AL_02 == '\0') {
      local_128 = *in_stack_00000038;
      lVar21 = local_110;
    }
    else {
      local_3d8 = (undefined8 *)CONCAT44(local_3d8._4_4_,0x27);
      bevy_input::button_input::ButtonInput::just_pressed<enum2_<bevy_input::keyboard::KeyCode>_>();
      local_128 = *in_stack_00000038;
      lVar21 = local_110;
      if (extraout_AL_03 != '\0') {
        lVar7 = in_stack_00000038[1];
        puVar14 = *(uint **)(local_128 + 0xf8);
        puVar3 = puVar14 + *(longlong *)(local_128 + 0x100);
        local_158 = local_110 + 0x10;
        local_140 = *(longlong *)(local_110 + 0x18);
        local_130 = (char *)(*(longlong *)(local_110 + 0x20) * 0x30);
        puVar28 = (undefined8 *)&DAT_00000008;
        uVar25 = 0;
        lVar13 = 0;
        lVar11 = 0;
        local_108 = &DAT_00000008;
        uVar18 = 0;
        do {
          puVar15 = local_100;
          if (*(char *)(local_128 + 0x108) == '\0') {
            if (uVar25 == uVar18) {
              if (puVar14 == puVar3) break;
              while( true ) {
                uVar18 = *(ulonglong *)
                          (*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar14 * 0xa0);
                local_120 = puVar28;
                if (uVar18 != 0) break;
                puVar14 = puVar14 + 1;
                if (puVar14 == puVar3) goto LAB_142310794;
              }
              lVar24 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar14 * 0xa0;
              uVar25 = (ulonglong)*(uint *)(lVar24 + 0x94);
              lVar13 = *(longlong *)(lVar7 + 0x1a8);
              lVar11 = lVar13 + uVar25 * 0x48;
              lVar30 = *(longlong *)(lVar13 + 0x38 + uVar25 * 0x48);
              if (*(ulonglong *)(local_128 + 0x110) < *(ulonglong *)(lVar13 + 0x40 + uVar25 * 0x48))
              {
                uVar25 = *(ulonglong *)(lVar30 + *(ulonglong *)(local_128 + 0x110) * 8);
                lVar13 = *(longlong *)(lVar11 + 0x18);
                if (uVar25 == 0) goto LAB_142310687;
                lVar11 = *(longlong *)(lVar13 + 0x10 + ~uVar25 * 0x30);
              }
              else {
                lVar13 = *(longlong *)(lVar11 + 0x18);
LAB_142310687:
                lVar11 = 0;
              }
              puVar14 = puVar14 + 1;
              lVar13 = *(longlong *)
                        (lVar13 + 0x10 +
                        ~*(ulonglong *)(lVar30 + *(longlong *)(local_128 + 0x120) * 8) * 0x30);
              local_108 = *(undefined1 **)(lVar24 + 0x50);
              uVar25 = 0;
            }
            plVar17 = (longlong *)(local_108 + uVar25 * 0x10);
            uVar12 = (ulonglong)*(uint *)(local_108 + uVar25 * 0x10 + 8);
          }
          else {
            if (uVar25 == uVar18) {
              if (puVar14 == puVar3) break;
              while( true ) {
                uVar18 = *(ulonglong *)
                          (*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar14 * 0x48);
                if (uVar18 != 0) break;
                puVar14 = puVar14 + 1;
                if (puVar14 == puVar3) goto LAB_142310794;
              }
              lVar30 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar14 * 0x48;
              if (*(ulonglong *)(local_128 + 0x110) < *(ulonglong *)(lVar30 + 0x40)) {
                uVar25 = *(ulonglong *)
                          (*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(local_128 + 0x110) * 8);
                lVar13 = *(longlong *)(lVar30 + 0x18);
                if (uVar25 == 0) goto LAB_142310648;
                lVar11 = *(longlong *)(lVar13 + 0x10 + ~uVar25 * 0x30);
              }
              else {
                lVar13 = *(longlong *)(lVar30 + 0x18);
LAB_142310648:
                lVar11 = 0;
              }
              puVar14 = puVar14 + 1;
              lVar13 = *(longlong *)
                        (lVar13 + 0x10 +
                        ~*(ulonglong *)
                          (*(longlong *)(lVar30 + 0x38) + *(longlong *)(local_128 + 0x120) * 8) *
                        0x30);
              puVar28 = *(undefined8 **)(lVar30 + 8);
              uVar25 = 0;
            }
            plVar17 = puVar28 + uVar25;
            uVar12 = uVar25 & 0xffffffff;
          }
          lVar30 = uVar12 * 0x30 + lVar11;
          uVar25 = uVar25 + 1;
          if ((*(char *)(uVar12 + lVar13) == '\x02') &&
             (*(float *)(lVar30 + 0x18) * *(float *)(lVar30 + 0x18) +
              *(float *)(lVar30 + 0x10) * *(float *)(lVar30 + 0x10) <= 25350.0)) {
            local_148 = CONCAT71(local_148._1_7_,*(char *)(local_128 + 0x108));
            lVar24 = *plVar17;
            lVar9 = local_140;
            pcVar19 = local_130;
            do {
              lVar29 = lVar9;
              local_120 = puVar28;
              if (pcVar19 == (char *)0x0) {
                std::alloc::__default_lib_allocator::__rust_alloc();
                if (extraout_RAX_06 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                  alloc::alloc::handle_alloc_error();
                }
                std::alloc::__default_lib_allocator::__rust_alloc();
                puVar15 = local_100;
                if (extraout_RAX_07 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                  alloc::alloc::handle_alloc_error();
                }
                *(undefined4 *)(extraout_RAX_07 + 1) = *(undefined4 *)(lVar30 + 0x18);
                *extraout_RAX_07 = *(undefined8 *)(lVar30 + 0x10);
                *extraout_RAX_06 = 1;
                extraout_RAX_06[1] = extraout_RAX_07;
                extraout_RAX_06[2] = 1;
                *(undefined1 *)(extraout_RAX_06 + 3) = 0;
                local_4d8 = 1;
                fStack_4d0 = SUB84(extraout_RAX_06,0);
                fStack_4cc = (float)((ulonglong)extraout_RAX_06 >> 0x20);
                fStack_4c8 = 1.4013e-45;
                fStack_4c4 = 0.0;
                perchance::PerchanceContext::uniform_f32();
                uVar34 = 0x3f800000;
                ::utils::uniform_sample_circle();
                lVar21 = local_110;
                local_3c8 = (undefined1 *)CONCAT44(fStack_4c4,fStack_4c8);
                uStack_3d0 = (undefined8 *)CONCAT44(fStack_4cc,fStack_4d0);
                local_3d8 = (undefined8 *)local_4d8;
                uStack_3c0 = CONCAT44(uVar34,extraout_XMM0_Da_04);
                local_3b8 = (float)lVar24;
                uStack_3b4 = (undefined4)((ulonglong)lVar24 >> 0x20);
                uStack_3b0 = 0;
                lVar7 = *(longlong *)(local_110 + 0x20);
                if (lVar7 == *(longlong *)(local_110 + 0x10)) {
                  alloc::raw_vec::RawVec::
                  grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                            ();
                }
                lVar11 = *(longlong *)(lVar21 + 0x18);
                lVar13 = lVar7 * 0x30;
                puVar28 = (undefined8 *)(lVar11 + 0x20 + lVar13);
                *puVar28 = CONCAT44(uStack_3b4,local_3b8);
                puVar28[1] = CONCAT44(uStack_3ac,uStack_3b0);
                puVar16 = (undefined4 *)(lVar11 + 0x10 + lVar13);
                *puVar16 = (undefined4)local_3c8;
                puVar16[1] = local_3c8._4_4_;
                puVar16[2] = (float)uStack_3c0;
                puVar16[3] = uStack_3c0._4_4_;
                puVar16 = (undefined4 *)(lVar11 + lVar13);
                *puVar16 = (undefined4)local_3d8;
                puVar16[1] = local_3d8._4_4_;
                puVar16[2] = (float)uStack_3d0;
                puVar16[3] = uStack_3d0._4_4_;
                *(longlong *)(lVar21 + 0x20) = lVar7 + 1;
                goto LAB_142310794;
              }
              pcVar19 = pcVar19 + -0x30;
              lVar9 = lVar29 + 0x30;
            } while ((*(int *)(lVar29 + 0x24) == 0) ||
                    (CONCAT44(*(int *)(lVar29 + 0x24),*(undefined4 *)(lVar29 + 0x20)) != lVar24));
          }
        } while( true );
      }
    }
LAB_142310794:
    local_1a0 = lVar21 + 0x10;
    local_148 = *(longlong *)(lVar21 + 0x18);
    local_1b8 = *(longlong *)(lVar21 + 0x20) * 0x30 + local_148;
    local_150 = in_stack_00000038[1];
    local_134 = *(undefined4 *)((longlong)in_stack_00000038 + 0x14);
    local_1b0 = *in_stack_00000028;
    local_1a8 = *puVar15;
    local_158 = *in_stack_00000050;
LAB_142310830:
    lVar21 = local_148;
    if (local_148 != local_1b8) {
      bVar32 = *(int *)(local_148 + 0x24) == 0;
      local_148 = local_148 + 0x30;
      pcVar31 = core::ops::function::FnOnce::
                call_once<system_critters::duck::plan_duck_trajectories::closure_env_5,tuple_<f32,f32>_>
      ;
      if (bVar32) {
        pcVar31 = core::ops::function::FnOnce::
                  call_once<system_critters::duck::plan_duck_trajectories::closure_env_4,tuple_<f32,f32>_>
        ;
      }
      uVar34 = 0x14;
      if (bVar32) {
        uVar34 = 1;
      }
      local_100 = (undefined8 *)CONCAT44(local_100._4_4_,uVar34);
      local_108 = (undefined1 *)((ulonglong)local_108 & 0xffffffff00000000);
LAB_14231089a:
      do {
        if ((uint)local_100 <= (uint)local_108) goto LAB_142310830;
        local_108 = (undefined1 *)CONCAT44(local_108._4_4_,(uint)local_108 + 1);
        plVar17 = *(longlong **)(lVar21 + 8);
        lVar7 = *(longlong *)(lVar21 + 0x10);
        plVar22 = plVar17 + lVar7 * 4;
        bVar23 = 1;
        uVar18 = 0;
        for (; plVar17 != plVar22; plVar17 = plVar17 + 4) {
          if ((char)plVar17[3] == '\0') {
            lVar11 = plVar17[2];
            if (lVar11 == 0) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            pfVar8 = (float *)plVar17[1];
            if (lVar11 == 1) {
              local_3d8 = (undefined8 *)
                          CONCAT44(pfVar8[2] +
                                   (float)((ulonglong)*(undefined8 *)(lVar21 + 0x18) >> 0x20) * 0.5,
                                   *pfVar8 + (float)*(undefined8 *)(lVar21 + 0x18) * 0.5);
              (*pcVar31)(0x3f000000);
              ::utils::swizzlers::impl$1::xvy();
              lVar11 = plVar17[2];
              if (lVar11 == *plVar17) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                puVar15 = local_1ec;
              }
              else {
                puVar15 = local_1ec;
              }
            }
            else {
              fVar33 = ((float)uVar18 / (float)lVar7) * 0.05 + 2.15;
              fVar36 = pfVar8[lVar11 * 3 + -6] * (1.0 - fVar33) +
                       (float)((ulonglong)*(undefined8 *)(pfVar8 + lVar11 * 3 + -4) >> 0x20) *
                       fVar33;
              fVar33 = pfVar8[lVar11 * 3 + -1] * fVar33 +
                       (float)*(undefined8 *)(pfVar8 + lVar11 * 3 + -4) * (1.0 - fVar33);
              local_1d0 = CONCAT44(fVar33,fVar36);
              if (plVar17[2] == 0) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              fVar36 = *(float *)plVar17[1] - fVar36;
              fVar33 = ((float *)plVar17[1])[2] - fVar33;
              (*pcVar31)(SQRT(fVar33 * fVar33 + fVar36 * fVar36));
              ::utils::swizzlers::impl$1::xvy();
              lVar11 = plVar17[2];
              if (lVar11 == *plVar17) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                puVar15 = &local_1f8;
              }
              else {
                puVar15 = &local_1f8;
              }
            }
            lVar13 = plVar17[1];
            *(undefined4 *)(lVar13 + 8 + lVar11 * 0xc) = *(undefined4 *)(puVar15 + 1);
            *(undefined8 *)(lVar13 + lVar11 * 0xc) = *puVar15;
            plVar17[2] = lVar11 + 1U;
            uVar25 = lVar11 - 1;
            if (lVar11 + 1U <= uVar25) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar30 = plVar17[1];
            fVar33 = *(float *)(lVar30 + 8 + uVar25 * 0xc);
            lVar13 = *(longlong *)(lVar30 + uVar25 * 0xc);
            fVar35 = (float)((ulonglong)lVar13 >> 0x20);
            uVar4 = *(undefined8 *)(lVar30 + lVar11 * 0xc);
            fVar36 = *(float *)(lVar30 + 8 + lVar11 * 0xc);
            *(int *)(lVar21 + 0x28) = *(int *)(lVar21 + 0x28) + 1;
            if (local_118 != (longlong *)0x0) {
              fVar38 = (float)uVar4 - (float)lVar13;
              fVar39 = (float)((ulonglong)uVar4 >> 0x20) - fVar35;
              fVar36 = fVar36 - fVar33;
              local_4d8 = lVar13;
              fStack_4d0 = fVar33;
              fStack_4cc = fVar38;
              fStack_4c8 = fVar39;
              fStack_4c4 = fVar36;
              country_core::systems::collision::world::RaycastWorld::legacy_raycast();
              if (local_3a8 != 2) {
                fVar35 = fVar35 - uStack_3c0._4_4_;
                fVar37 = (float)lVar13 - (float)uStack_3c0;
                if ((fVar33 - local_3b8) * (fVar33 - local_3b8) + fVar35 * fVar35 + fVar37 * fVar37
                    <= fVar36 * fVar36 + fVar39 * fVar39 + fVar38 * fVar38) {
                  perchance::PerchanceContext::uniform_f32();
                  uVar34 = 0x3f800000;
                  ::utils::uniform_sample_circle();
                  *(undefined4 *)(lVar21 + 0x18) = extraout_XMM0_Da_02;
                  *(undefined4 *)(lVar21 + 0x1c) = uVar34;
                  lVar7 = *(longlong *)(lVar21 + 0x10);
                  if (lVar7 != 0) {
                    lVar11 = *(longlong *)(lVar21 + 8);
                    lVar13 = 0;
                    do {
                      if (*(longlong *)(lVar11 + 0x10 + lVar13) != 0) {
                        *(undefined8 *)(lVar11 + 0x10 + lVar13) = 1;
                      }
                      *(undefined1 *)(lVar11 + 0x18 + lVar13) = 0;
                      lVar13 = lVar13 + 0x20;
                    } while (lVar7 << 5 != lVar13);
                  }
                  goto LAB_14231089a;
                }
              }
            }
            if (1 < (ulonglong)plVar17[2]) {
              lVar11 = plVar17[2] * 0xc + plVar17[1];
              if (lVar11 == 0xc) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              if (25350.0 < *(float *)(lVar11 + -4) * *(float *)(lVar11 + -4) +
                            *(float *)(lVar11 + -0xc) * *(float *)(lVar11 + -0xc)) {
                *(undefined1 *)(plVar17 + 3) = 1;
              }
            }
          }
          uVar18 = uVar18 + 1;
          bVar23 = bVar23 & *(byte *)(plVar17 + 3);
        }
      } while (bVar23 == 0);
      if (*(int *)(lVar21 + 0x24) == 0) {
        country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name();
        pcVar19 = "duck_shader";
        local_140 = extraout_RAX_03;
        country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
                  ();
        local_130 = pcVar19;
        if ((extraout_RAX_04 & 1) == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        puVar28 = *(undefined8 **)(lVar21 + 8);
        local_208 = *(longlong *)(lVar21 + 0x10);
        *(undefined8 *)(lVar21 + 0x10) = 0;
        uVar18 = local_208 * 0x20;
        local_218 = puVar28 + local_208 * 4;
        local_210 = lVar21;
        local_200 = 0;
        puVar15 = puVar28 + 5;
        local_120 = local_218;
        while (uVar18 = uVar18 - 0x20, puVar28 != local_120) {
          puVar2 = puVar28 + 4;
          local_220 = puVar2;
          if (SBORROW8(0,(longlong)*puVar28)) {
            if (local_120 != puVar2) {
              lVar21 = *(longlong *)(lVar21 + 8);
              uVar18 = uVar18 >> 5;
              puVar26 = (undefined1 *)((longlong)puVar15 + lVar21);
              do {
                if (*(longlong *)(puVar26 + (-8 - lVar21)) != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                puVar26 = puVar26 + 0x20;
                uVar18 = uVar18 - 1;
              } while (uVar18 != 0);
            }
            break;
          }
          local_108 = (undefined1 *)puVar28[1];
          uVar25 = puVar28[2];
          local_100 = (undefined8 *)*puVar28;
          perchance::PerchanceContext::uniform_f32();
          country_core::resources::critters::Duck::new();
          local_438 = 0;
          if (local_448 < uVar25) {
            local_f1 = 1;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          uVar12 = local_438;
          lVar7 = local_440;
          memcpy((void *)(local_440 + local_438 * 0xc),local_108,uVar25 * 0xc);
          local_438 = uVar12 + uVar25;
          if (1 < local_438) {
            uVar25 = local_438 >> 1;
            puVar16 = (undefined4 *)(lVar7 + local_438 * 0xc + -4);
            puVar20 = (undefined4 *)(lVar7 + 8);
            do {
              uVar4 = *(undefined8 *)(puVar20 + -2);
              *(undefined8 *)(puVar20 + -2) = *(undefined8 *)(puVar16 + -2);
              *(undefined8 *)(puVar16 + -2) = uVar4;
              uVar34 = *puVar20;
              *puVar20 = *puVar16;
              *puVar16 = uVar34;
              puVar20 = puVar20 + 3;
              puVar16 = puVar16 + -3;
              uVar25 = uVar25 - 1;
            } while (uVar25 != 0);
          }
          local_f1 = 1;
          perchance::PerchanceContext::uniform_f32();
          local_3ec = extraout_XMM0_Da_03 * -0.3;
          memcpy(&local_3d8,&local_4d8,0x100);
          local_2b8 = 0;
          local_2b0 = 0;
          local_240 = 0;
          local_248 = 0;
          local_25c = 0;
          local_2c8 = 0;
          uStack_2c0 = 0x3f80000000000000;
          local_2ac = 0x3f8000003f800000;
          local_2a4 = 0x3f800000;
          local_298 = local_140;
          local_290 = local_130;
          local_278 = 3;
          local_2d8 = 0;
          local_280 = 0;
          local_288 = 0;
          local_27c = 0x3ecccccd;
          local_270 = 0;
          uStack_268 = 0;
          local_260 = 0x3f800000;
          local_258 = 0;
          uStack_250 = 0x3f80000000000000;
          local_23c = 0x3f8000003f800000;
          local_234 = 0x3f800000;
          local_f1 = 0;
          bevy_ecs::system::commands::Commands::
          spawn<tuple_<country_core::resources::critters::Duck,enum2_<country_core::resources::critters::DuckState>,country_core::components::drawable::DrawableMeshBundle,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier,country_core::components::drawable::PrevTransform>_>
                    ();
          if (local_100 != (undefined8 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          puVar15 = puVar15 + 4;
          puVar28 = puVar2;
        }
      }
      else {
        uVar18 = *(ulonglong *)(lVar21 + 0x20) & 0xffffffff;
        if ((uVar18 < *(ulonglong *)(local_150 + 0x10)) &&
           (*(int *)(*(longlong *)(local_150 + 8) + uVar18 * 0x14) ==
            (int)(*(ulonglong *)(lVar21 + 0x20) >> 0x20))) {
          lVar7 = *(longlong *)(local_150 + 8) + uVar18 * 0x14;
          uVar18 = (ulonglong)*(uint *)(lVar7 + 4);
          if ((uVar18 != 0xffffffff) &&
             ((uVar18 < *(ulonglong *)(local_128 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)(local_128 + 0x28) + (ulonglong)(*(uint *)(lVar7 + 4) >> 6) * 8) >>
                (uVar18 & 0x3f) & 1) != 0)))) {
            uVar18 = (ulonglong)*(uint *)(lVar7 + 0x10);
            lVar11 = *(longlong *)
                      (*(longlong *)(local_150 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar7 + 0xc) * 0x48);
            lVar13 = *(longlong *)
                      (*(longlong *)(local_150 + 0x1a8) + 0x38 +
                      (ulonglong)*(uint *)(lVar7 + 0xc) * 0x48);
            lVar30 = ~*(ulonglong *)(lVar13 + *(longlong *)(local_128 + 0x118) * 8) * 0x30;
            lVar7 = *(longlong *)(lVar11 + 0x10 + lVar30);
            lVar30 = *(longlong *)(lVar11 + 0x28 + lVar30);
            lVar13 = ~*(ulonglong *)(lVar13 + *(longlong *)(local_128 + 0x120) * 8) * 0x30;
            puVar26 = *(undefined1 **)(lVar11 + 0x10 + lVar13);
            puVar15 = *(undefined8 **)(lVar11 + 0x28 + lVar13);
            *(undefined4 *)(lVar30 + uVar18 * 4) = local_134;
            lVar11 = *(longlong *)(lVar21 + 8);
            uStack_3c0 = *(longlong *)(lVar21 + 0x10);
            *(undefined8 *)(lVar21 + 0x10) = 0;
            lVar13 = uStack_3c0 * 0x20 + lVar11;
            local_3b8 = 0.0;
            uStack_3b4 = 0;
            local_3c8 = (undefined1 *)lVar21;
            uStack_3d0 = (undefined8 *)lVar13;
            local_3d8 = (undefined8 *)lVar11;
            if (uStack_3c0 != 0) {
              lVar21 = lVar13 + -0x20;
              pcVar19 = *(char **)(lVar13 + -0x20);
              uStack_3d0 = (undefined8 *)lVar21;
              if (!SBORROW8(0,(longlong)pcVar19)) {
                lVar7 = lVar7 + uVar18 * 0x100;
                local_120 = *(undefined8 **)(lVar13 + -0x18);
                uVar25 = *(ulonglong *)(lVar13 + -0x10);
                *(undefined8 *)(lVar7 + 0xa0) = 0;
                local_140 = lVar30;
                local_130 = pcVar19;
                local_108 = puVar26;
                local_100 = puVar15;
                if (*(ulonglong *)(lVar7 + 0x90) < uVar25) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  lVar13 = *(longlong *)(lVar7 + 0xa0);
                }
                else {
                  lVar13 = 0;
                }
                memcpy((void *)(lVar13 * 0xc + *(longlong *)(lVar7 + 0x98)),local_120,uVar25 * 0xc);
                *(ulonglong *)(lVar7 + 0xa0) = lVar13 + uVar25;
                if (local_130 != (char *)0x0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                puVar15 = local_100;
                puVar26 = local_108;
                lVar13 = local_140;
                if (lVar21 - lVar11 != 0) {
                  uVar25 = (ulonglong)(lVar21 - lVar11) >> 5;
                  lVar11 = lVar11 + 8;
                  do {
                    if (*(longlong *)(lVar11 + -8) != 0) {
                      std::alloc::__default_lib_allocator::__rust_dealloc();
                    }
                    lVar11 = lVar11 + 0x20;
                    uVar25 = uVar25 - 1;
                  } while (uVar25 != 0);
                }
                *(undefined4 *)(lVar13 + uVar18 * 4) = local_134;
                uVar25 = *(ulonglong *)(lVar7 + 0xa0);
                if (1 < uVar25) {
                  uVar12 = uVar25 >> 1;
                  puVar20 = (undefined4 *)(*(longlong *)(lVar7 + 0x98) + uVar25 * 0xc + -4);
                  puVar16 = (undefined4 *)(*(longlong *)(lVar7 + 0x98) + 8);
                  do {
                    uVar4 = *(undefined8 *)(puVar16 + -2);
                    *(undefined8 *)(puVar16 + -2) = *(undefined8 *)(puVar20 + -2);
                    *(undefined8 *)(puVar20 + -2) = uVar4;
                    uVar34 = *puVar16;
                    *puVar16 = *puVar20;
                    *puVar20 = uVar34;
                    puVar16 = puVar16 + 3;
                    puVar20 = puVar20 + -3;
                    uVar12 = uVar12 - 1;
                  } while (uVar12 != 0);
                }
                *(undefined4 *)(lVar13 + uVar18 * 4) = local_134;
                *(undefined4 *)(lVar7 + 0xec) = 0x3f800000;
                *(undefined4 *)((longlong)puVar15 + uVar18 * 4) = local_134;
                puVar26[uVar18] = 3;
                goto LAB_142310830;
              }
            }
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
        }
      }
      goto LAB_142310830;
    }
    alloc::vec::Vec::
    retain<system_critters::duck::TrajectoryPlan,alloc::alloc::Global,system_critters::duck::plan_duck_trajectories::closure_env_7>
              ();
    if (local_f2 != '\0') {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_05 == 1) {
        plVar17 = extraout_RAX_05 + 1;
      }
      else if ((*extraout_RAX_05 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar17 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
LAB_1423114ee:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      if (*plVar17 != 0) {
LAB_142311543:
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar17 = -1;
      local_118 = plVar17;
      puffin::ThreadProfiler::end_scope();
      *local_118 = *local_118 + 1;
    }
  }
  return;
}

