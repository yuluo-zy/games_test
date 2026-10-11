
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
  float fVar9;
  float fVar10;
  float fVar11;
  longlong lVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar14;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  ulonglong uVar15;
  longlong *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  undefined4 uVar16;
  longlong lVar17;
  uint *puVar18;
  undefined8 *puVar19;
  undefined4 *puVar20;
  longlong *plVar21;
  ulonglong uVar22;
  char *pcVar23;
  undefined4 *puVar24;
  longlong lVar25;
  longlong *plVar26;
  byte bVar27;
  longlong lVar28;
  ulonglong uVar29;
  undefined1 *puVar30;
  longlong *in_R8;
  undefined *puVar31;
  undefined8 *puVar32;
  undefined8 *in_R9;
  longlong lVar33;
  longlong lVar34;
  code *pcVar35;
  bool bVar36;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float fVar37;
  undefined4 extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  undefined4 extraout_XMM0_Da_04;
  undefined4 uVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
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
      uVar22 = 0x2d;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX == 1) {
        if (uVar22 != 0) {
          if (uVar22 < 0x2d) {
            if ((char)(&DAT_14304a97f)[uVar22] < -0x40) {
LAB_14230ffe5:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (uVar22 != 0x2d) goto LAB_14230ffe5;
        }
        core::str::impl$0::rfind<ref_<str_>_>();
        if ((extraout_RAX_00 == 1) && (uVar22 != 0xfffffffffffffffe)) {
          if (uVar22 + 2 < 0x2d) {
            if ((char)(&UNK_14304a981)[uVar22] < -0x40) {
LAB_142310043:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (uVar22 + 2 != 0x2d) goto LAB_142310043;
        }
      }
      puVar31 = &DAT_14304aa9b;
      do {
        bVar27 = puVar31[-1];
        uVar13 = (uint)(char)bVar27;
        if ((char)bVar27 < '\0') {
          bVar5 = puVar31[-2];
          if ((char)bVar5 < -0x40) {
            bVar6 = puVar31[-3];
            if ((char)bVar6 < -0x40) {
              pbVar1 = puVar31 + -4;
              puVar31 = puVar31 + -4;
              uVar13 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              puVar31 = puVar31 + -3;
              uVar13 = bVar6 & 0xf;
            }
            uVar13 = bVar5 & 0x3f | uVar13 << 6;
          }
          else {
            puVar31 = puVar31 + -2;
            uVar13 = bVar5 & 0x1f;
          }
          uVar13 = (uint)(bVar27 & 0x3f) | uVar13 << 6;
          if (uVar13 != 0x5c) goto LAB_142310104;
LAB_142310116:
          if (puVar31 == &UNK_14304aa77) break;
          if (puVar31 + -0x14304aa77 < (undefined *)0x23) {
            if (-0x41 < (char)puVar31[1]) break;
          }
          else if (puVar31 + -0x14304aa77 == (undefined *)0x23) break;
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
        puVar31 = puVar31 + -1;
        if (uVar13 == 0x5c) goto LAB_142310116;
LAB_142310104:
        if (uVar13 == 0x2f) goto LAB_142310116;
      } while (puVar31 != &DAT_14304aa78);
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_01 == 1) {
        plVar21 = extraout_RAX_01 + 1;
      }
      else if ((*extraout_RAX_01 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar21 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
      goto LAB_1423114ee;
      if (*plVar21 != 0) goto LAB_142311543;
      *plVar21 = -1;
      local_118 = plVar21;
      puffin::ThreadProfiler::begin_scope();
      *local_118 = *local_118 + 1;
      local_1c8 = 1;
    }
    puVar19 = local_100;
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
        lVar25 = local_110;
        if (extraout_RAX_02 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        *(float *)(extraout_RAX_02 + 1) = local_160;
        *extraout_RAX_02 = local_168;
        local_190 = (undefined1 *)0x1;
        local_180 = (undefined1 *)0x1;
        local_188 = extraout_RAX_02;
        while (local_108 = local_180, local_180 < (undefined1 *)0x3) {
          perchance::PerchanceContext::uniform_f32();
          uVar38 = extraout_XMM0_Da;
          ::utils::golden_spiral();
          local_4d8 = CONCAT44(uVar38,extraout_XMM0_Da_00);
          ::utils::swizzlers::impl$0::x0y();
          puVar30 = local_180;
          fVar39 = SUB84(local_3d8,0);
          uVar22 = (ulonglong)local_3d8 >> 0x20;
          fVar42 = (float)uStack_3d0 + local_160;
          if (local_180 == local_190) {
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          *(ulonglong *)((longlong)local_188 + (longlong)puVar30 * 0xc) =
               CONCAT44((float)uVar22 + (float)((ulonglong)local_168 >> 0x20),
                        fVar39 + (float)local_168);
          *(float *)((longlong)local_188 + (longlong)puVar30 * 0xc + 8) = fVar42;
          local_180 = puVar30 + 1;
        }
        uStack_3c0 = (longlong)local_188 + (longlong)local_180 * 0xc;
        local_3d8 = local_188;
        uStack_3d0 = local_188;
        local_3c8 = local_190;
        alloc::vec::in_place_collect::impl$1::
        from_iter<system_critters::duck::Trajectory,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,system_critters::spawn_ducks::spawn_ducks::closure_env_1>_>
                  ();
        perchance::PerchanceContext::uniform_f32();
        fVar39 = __real_3f800000;
        ::utils::uniform_sample_circle();
        puVar19 = local_100;
        local_3c8 = (undefined1 *)CONCAT44(fStack_4c4,fStack_4c8);
        uStack_3d0 = (undefined8 *)CONCAT44(fStack_4cc,fStack_4d0);
        local_3d8 = (undefined8 *)local_4d8;
        uStack_3c0 = CONCAT44(fVar39,extraout_XMM0_Da_01);
        uStack_3b4 = 0;
        uStack_3b0 = 0;
        lVar7 = *(longlong *)(lVar25 + 0x20);
        if (lVar7 == *(longlong *)(lVar25 + 0x10)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                    ();
        }
        lVar14 = *(longlong *)(lVar25 + 0x18);
        lVar17 = lVar7 * 0x30;
        puVar32 = (undefined8 *)(lVar14 + 0x20 + lVar17);
        *puVar32 = CONCAT44(uStack_3b4,local_3b8);
        puVar32[1] = CONCAT44(uStack_3ac,uStack_3b0);
        puVar20 = (undefined4 *)(lVar14 + 0x10 + lVar17);
        *puVar20 = (undefined4)local_3c8;
        puVar20[1] = local_3c8._4_4_;
        puVar20[2] = (float)uStack_3c0;
        puVar20[3] = uStack_3c0._4_4_;
        *(longlong *)(lVar14 + lVar17) = (longlong)local_3d8;
        ((longlong *)(lVar14 + lVar17))[1] = (longlong)uStack_3d0;
        *(longlong *)(lVar25 + 0x20) = lVar7 + 1;
      }
    }
    country_core::game_config::developer_hotkeys_enabled();
    if (extraout_AL_02 == '\0') {
      local_128 = *in_stack_00000038;
      lVar25 = local_110;
    }
    else {
      local_3d8 = (undefined8 *)CONCAT44(local_3d8._4_4_,0x27);
      bevy_input::button_input::ButtonInput::just_pressed<enum2_<bevy_input::keyboard::KeyCode>_>();
      local_128 = *in_stack_00000038;
      lVar25 = local_110;
      if (extraout_AL_03 != '\0') {
        lVar7 = in_stack_00000038[1];
        puVar18 = *(uint **)(local_128 + 0xf8);
        puVar3 = puVar18 + *(longlong *)(local_128 + 0x100);
        local_158 = local_110 + 0x10;
        local_140 = *(longlong *)(local_110 + 0x18);
        local_130 = (char *)(*(longlong *)(local_110 + 0x20) * 0x30);
        puVar32 = (undefined8 *)&DAT_00000008;
        uVar29 = 0;
        lVar17 = 0;
        lVar14 = 0;
        local_108 = &DAT_00000008;
        uVar22 = 0;
        do {
          puVar19 = local_100;
          if (*(char *)(local_128 + 0x108) == '\0') {
            if (uVar29 == uVar22) {
              if (puVar18 == puVar3) break;
              while( true ) {
                uVar22 = *(ulonglong *)
                          (*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
                local_120 = puVar32;
                if (uVar22 != 0) break;
                puVar18 = puVar18 + 1;
                if (puVar18 == puVar3) goto LAB_142310794;
              }
              lVar28 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar18 * 0xa0;
              uVar29 = (ulonglong)*(uint *)(lVar28 + 0x94);
              lVar17 = *(longlong *)(lVar7 + 0x1a8);
              lVar14 = lVar17 + uVar29 * 0x48;
              lVar34 = *(longlong *)(lVar17 + 0x38 + uVar29 * 0x48);
              if (*(ulonglong *)(local_128 + 0x110) < *(ulonglong *)(lVar17 + 0x40 + uVar29 * 0x48))
              {
                uVar29 = *(ulonglong *)(lVar34 + *(ulonglong *)(local_128 + 0x110) * 8);
                lVar17 = *(longlong *)(lVar14 + 0x18);
                if (uVar29 == 0) goto LAB_142310687;
                lVar14 = *(longlong *)(lVar17 + 0x10 + ~uVar29 * 0x30);
              }
              else {
                lVar17 = *(longlong *)(lVar14 + 0x18);
LAB_142310687:
                lVar14 = 0;
              }
              puVar18 = puVar18 + 1;
              lVar17 = *(longlong *)
                        (lVar17 + 0x10 +
                        ~*(ulonglong *)(lVar34 + *(longlong *)(local_128 + 0x120) * 8) * 0x30);
              local_108 = *(undefined1 **)(lVar28 + 0x50);
              uVar29 = 0;
            }
            plVar21 = (longlong *)(local_108 + uVar29 * 0x10);
            uVar15 = (ulonglong)*(uint *)(local_108 + uVar29 * 0x10 + 8);
          }
          else {
            if (uVar29 == uVar22) {
              if (puVar18 == puVar3) break;
              while( true ) {
                uVar22 = *(ulonglong *)
                          (*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
                if (uVar22 != 0) break;
                puVar18 = puVar18 + 1;
                if (puVar18 == puVar3) goto LAB_142310794;
              }
              lVar34 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
              if (*(ulonglong *)(local_128 + 0x110) < *(ulonglong *)(lVar34 + 0x40)) {
                uVar29 = *(ulonglong *)
                          (*(longlong *)(lVar34 + 0x38) + *(ulonglong *)(local_128 + 0x110) * 8);
                lVar17 = *(longlong *)(lVar34 + 0x18);
                if (uVar29 == 0) goto LAB_142310648;
                lVar14 = *(longlong *)(lVar17 + 0x10 + ~uVar29 * 0x30);
              }
              else {
                lVar17 = *(longlong *)(lVar34 + 0x18);
LAB_142310648:
                lVar14 = 0;
              }
              puVar18 = puVar18 + 1;
              lVar17 = *(longlong *)
                        (lVar17 + 0x10 +
                        ~*(ulonglong *)
                          (*(longlong *)(lVar34 + 0x38) + *(longlong *)(local_128 + 0x120) * 8) *
                        0x30);
              puVar32 = *(undefined8 **)(lVar34 + 8);
              uVar29 = 0;
            }
            plVar21 = puVar32 + uVar29;
            uVar15 = uVar29 & 0xffffffff;
          }
          lVar34 = uVar15 * 0x30 + lVar14;
          uVar29 = uVar29 + 1;
          if ((*(char *)(uVar15 + lVar17) == '\x02') &&
             (*(float *)(lVar34 + 0x18) * *(float *)(lVar34 + 0x18) +
              *(float *)(lVar34 + 0x10) * *(float *)(lVar34 + 0x10) <= __real_46c60c00)) {
            local_148 = CONCAT71(local_148._1_7_,*(char *)(local_128 + 0x108));
            lVar28 = *plVar21;
            lVar12 = local_140;
            pcVar23 = local_130;
            do {
              lVar33 = lVar12;
              local_120 = puVar32;
              if (pcVar23 == (char *)0x0) {
                std::alloc::__default_lib_allocator::__rust_alloc();
                if (extraout_RAX_06 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                  alloc::alloc::handle_alloc_error();
                }
                std::alloc::__default_lib_allocator::__rust_alloc();
                puVar19 = local_100;
                if (extraout_RAX_07 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                  alloc::alloc::handle_alloc_error();
                }
                *(undefined4 *)(extraout_RAX_07 + 1) = *(undefined4 *)(lVar34 + 0x18);
                *extraout_RAX_07 = *(undefined8 *)(lVar34 + 0x10);
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
                fVar39 = __real_3f800000;
                ::utils::uniform_sample_circle();
                lVar25 = local_110;
                local_3c8 = (undefined1 *)CONCAT44(fStack_4c4,fStack_4c8);
                uStack_3d0 = (undefined8 *)CONCAT44(fStack_4cc,fStack_4d0);
                local_3d8 = (undefined8 *)local_4d8;
                uStack_3c0 = CONCAT44(fVar39,extraout_XMM0_Da_04);
                local_3b8 = (float)lVar28;
                uStack_3b4 = (undefined4)((ulonglong)lVar28 >> 0x20);
                uStack_3b0 = 0;
                lVar7 = *(longlong *)(local_110 + 0x20);
                if (lVar7 == *(longlong *)(local_110 + 0x10)) {
                  alloc::raw_vec::RawVec::
                  grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                            ();
                }
                lVar14 = *(longlong *)(lVar25 + 0x18);
                lVar17 = lVar7 * 0x30;
                puVar32 = (undefined8 *)(lVar14 + 0x20 + lVar17);
                *puVar32 = CONCAT44(uStack_3b4,local_3b8);
                puVar32[1] = CONCAT44(uStack_3ac,uStack_3b0);
                puVar20 = (undefined4 *)(lVar14 + 0x10 + lVar17);
                *puVar20 = (undefined4)local_3c8;
                puVar20[1] = local_3c8._4_4_;
                puVar20[2] = (float)uStack_3c0;
                puVar20[3] = uStack_3c0._4_4_;
                puVar20 = (undefined4 *)(lVar14 + lVar17);
                *puVar20 = (undefined4)local_3d8;
                puVar20[1] = local_3d8._4_4_;
                puVar20[2] = (float)uStack_3d0;
                puVar20[3] = uStack_3d0._4_4_;
                *(longlong *)(lVar25 + 0x20) = lVar7 + 1;
                goto LAB_142310794;
              }
              pcVar23 = pcVar23 + -0x30;
              lVar12 = lVar33 + 0x30;
            } while ((*(int *)(lVar33 + 0x24) == 0) ||
                    (CONCAT44(*(int *)(lVar33 + 0x24),*(undefined4 *)(lVar33 + 0x20)) != lVar28));
          }
        } while( true );
      }
    }
LAB_142310794:
    fVar11 = __real_4009999a;
    fVar10 = __real_3d4ccccd;
    fVar9 = __real_3f800000;
    fVar42 = __xmm_00000000000000003f0000003f000000._4_4_;
    fVar39 = (float)__xmm_00000000000000003f0000003f000000;
    uVar38 = __real_3f000000;
    local_1a0 = lVar25 + 0x10;
    local_148 = *(longlong *)(lVar25 + 0x18);
    local_1b8 = *(longlong *)(lVar25 + 0x20) * 0x30 + local_148;
    local_150 = in_stack_00000038[1];
    local_134 = *(undefined4 *)((longlong)in_stack_00000038 + 0x14);
    local_1b0 = *in_stack_00000028;
    local_1a8 = *puVar19;
    local_158 = *in_stack_00000050;
LAB_142310830:
    lVar25 = local_148;
    if (local_148 != local_1b8) {
      bVar36 = *(int *)(local_148 + 0x24) == 0;
      local_148 = local_148 + 0x30;
      pcVar35 = core::ops::function::FnOnce::
                call_once<system_critters::duck::plan_duck_trajectories::closure_env_5,tuple_<f32,f32>_>
      ;
      if (bVar36) {
        pcVar35 = core::ops::function::FnOnce::
                  call_once<system_critters::duck::plan_duck_trajectories::closure_env_4,tuple_<f32,f32>_>
        ;
      }
      uVar16 = 0x14;
      if (bVar36) {
        uVar16 = 1;
      }
      local_100 = (undefined8 *)CONCAT44(local_100._4_4_,uVar16);
      local_108 = (undefined1 *)((ulonglong)local_108 & 0xffffffff00000000);
LAB_14231089a:
      do {
        if ((uint)local_100 <= (uint)local_108) goto LAB_142310830;
        local_108 = (undefined1 *)CONCAT44(local_108._4_4_,(uint)local_108 + 1);
        plVar21 = *(longlong **)(lVar25 + 8);
        lVar7 = *(longlong *)(lVar25 + 0x10);
        plVar26 = plVar21 + lVar7 * 4;
        bVar27 = 1;
        uVar22 = 0;
        for (; plVar21 != plVar26; plVar21 = plVar21 + 4) {
          if ((char)plVar21[3] == '\0') {
            lVar14 = plVar21[2];
            if (lVar14 == 0) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            pfVar8 = (float *)plVar21[1];
            if (lVar14 == 1) {
              local_3d8 = (undefined8 *)
                          CONCAT44(pfVar8[2] +
                                   (float)((ulonglong)*(undefined8 *)(lVar25 + 0x18) >> 0x20) *
                                   fVar42,*pfVar8 + (float)*(undefined8 *)(lVar25 + 0x18) * fVar39);
              (*pcVar35)(uVar38);
              ::utils::swizzlers::impl$1::xvy();
              lVar14 = plVar21[2];
              if (lVar14 == *plVar21) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                puVar19 = local_1ec;
              }
              else {
                puVar19 = local_1ec;
              }
            }
            else {
              fVar37 = ((float)uVar22 / (float)lVar7) * fVar10 + fVar11;
              fVar40 = fVar9 - fVar37;
              fVar41 = pfVar8[lVar14 * 3 + -6] * fVar40 +
                       (float)((ulonglong)*(undefined8 *)(pfVar8 + lVar14 * 3 + -4) >> 0x20) *
                       fVar37;
              fVar37 = pfVar8[lVar14 * 3 + -1] * fVar37 +
                       (float)*(undefined8 *)(pfVar8 + lVar14 * 3 + -4) * fVar40;
              local_1d0 = CONCAT44(fVar37,fVar41);
              if (plVar21[2] == 0) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              fVar41 = *(float *)plVar21[1] - fVar41;
              fVar37 = ((float *)plVar21[1])[2] - fVar37;
              (*pcVar35)(SQRT(fVar37 * fVar37 + fVar41 * fVar41));
              ::utils::swizzlers::impl$1::xvy();
              lVar14 = plVar21[2];
              if (lVar14 == *plVar21) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                puVar19 = &local_1f8;
              }
              else {
                puVar19 = &local_1f8;
              }
            }
            lVar17 = plVar21[1];
            *(undefined4 *)(lVar17 + 8 + lVar14 * 0xc) = *(undefined4 *)(puVar19 + 1);
            *(undefined8 *)(lVar17 + lVar14 * 0xc) = *puVar19;
            plVar21[2] = lVar14 + 1U;
            uVar29 = lVar14 - 1;
            if (lVar14 + 1U <= uVar29) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            lVar34 = plVar21[1];
            fVar37 = *(float *)(lVar34 + 8 + uVar29 * 0xc);
            lVar17 = *(longlong *)(lVar34 + uVar29 * 0xc);
            fVar40 = (float)((ulonglong)lVar17 >> 0x20);
            uVar4 = *(undefined8 *)(lVar34 + lVar14 * 0xc);
            fVar41 = *(float *)(lVar34 + 8 + lVar14 * 0xc);
            *(int *)(lVar25 + 0x28) = *(int *)(lVar25 + 0x28) + 1;
            if (local_118 != (longlong *)0x0) {
              fVar44 = (float)uVar4 - (float)lVar17;
              fVar45 = (float)((ulonglong)uVar4 >> 0x20) - fVar40;
              fVar41 = fVar41 - fVar37;
              local_4d8 = lVar17;
              fStack_4d0 = fVar37;
              fStack_4cc = fVar44;
              fStack_4c8 = fVar45;
              fStack_4c4 = fVar41;
              country_core::systems::collision::world::RaycastWorld::legacy_raycast();
              if (local_3a8 != 2) {
                fVar40 = fVar40 - uStack_3c0._4_4_;
                fVar43 = (float)lVar17 - (float)uStack_3c0;
                if ((fVar37 - local_3b8) * (fVar37 - local_3b8) + fVar40 * fVar40 + fVar43 * fVar43
                    <= fVar41 * fVar41 + fVar45 * fVar45 + fVar44 * fVar44) {
                  perchance::PerchanceContext::uniform_f32();
                  fVar37 = fVar9;
                  ::utils::uniform_sample_circle();
                  *(undefined4 *)(lVar25 + 0x18) = extraout_XMM0_Da_02;
                  *(float *)(lVar25 + 0x1c) = fVar37;
                  lVar7 = *(longlong *)(lVar25 + 0x10);
                  if (lVar7 != 0) {
                    lVar14 = *(longlong *)(lVar25 + 8);
                    lVar17 = 0;
                    do {
                      if (*(longlong *)(lVar14 + 0x10 + lVar17) != 0) {
                        *(undefined8 *)(lVar14 + 0x10 + lVar17) = 1;
                      }
                      *(undefined1 *)(lVar14 + 0x18 + lVar17) = 0;
                      lVar17 = lVar17 + 0x20;
                    } while (lVar7 << 5 != lVar17);
                  }
                  goto LAB_14231089a;
                }
              }
            }
            if (1 < (ulonglong)plVar21[2]) {
              lVar14 = plVar21[2] * 0xc + plVar21[1];
              if (lVar14 == 0xc) {
                    /* WARNING: Subroutine does not return */
                core::option::unwrap_failed();
              }
              if (__real_46c60c00 <
                  *(float *)(lVar14 + -4) * *(float *)(lVar14 + -4) +
                  *(float *)(lVar14 + -0xc) * *(float *)(lVar14 + -0xc)) {
                *(undefined1 *)(plVar21 + 3) = 1;
              }
            }
          }
          uVar22 = uVar22 + 1;
          bVar27 = bVar27 & *(byte *)(plVar21 + 3);
        }
      } while (bVar27 == 0);
      if (*(int *)(lVar25 + 0x24) == 0) {
        country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name();
        pcVar23 = "duck_shader";
        local_140 = extraout_RAX_03;
        country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
                  ();
        local_130 = pcVar23;
        if ((extraout_RAX_04 & 1) == 0) {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        puVar32 = *(undefined8 **)(lVar25 + 8);
        local_208 = *(longlong *)(lVar25 + 0x10);
        *(undefined8 *)(lVar25 + 0x10) = 0;
        uVar22 = local_208 * 0x20;
        local_218 = puVar32 + local_208 * 4;
        local_210 = lVar25;
        local_200 = 0;
        puVar19 = puVar32 + 5;
        local_120 = local_218;
        while (uVar22 = uVar22 - 0x20, puVar32 != local_120) {
          puVar2 = puVar32 + 4;
          local_220 = puVar2;
          if (SBORROW8(0,(longlong)*puVar32)) {
            if (local_120 != puVar2) {
              lVar25 = *(longlong *)(lVar25 + 8);
              uVar22 = uVar22 >> 5;
              puVar30 = (undefined1 *)((longlong)puVar19 + lVar25);
              do {
                if (*(longlong *)(puVar30 + (-8 - lVar25)) != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                puVar30 = puVar30 + 0x20;
                uVar22 = uVar22 - 1;
              } while (uVar22 != 0);
            }
            break;
          }
          local_108 = (undefined1 *)puVar32[1];
          uVar29 = puVar32[2];
          local_100 = (undefined8 *)*puVar32;
          perchance::PerchanceContext::uniform_f32();
          country_core::resources::critters::Duck::new();
          local_438 = 0;
          if (local_448 < uVar29) {
            local_f1 = 1;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          uVar15 = local_438;
          lVar7 = local_440;
          memcpy((void *)(local_440 + local_438 * 0xc),local_108,uVar29 * 0xc);
          local_438 = uVar15 + uVar29;
          if (1 < local_438) {
            uVar29 = local_438 >> 1;
            puVar20 = (undefined4 *)(lVar7 + local_438 * 0xc + -4);
            puVar24 = (undefined4 *)(lVar7 + 8);
            do {
              uVar4 = *(undefined8 *)(puVar24 + -2);
              *(undefined8 *)(puVar24 + -2) = *(undefined8 *)(puVar20 + -2);
              *(undefined8 *)(puVar20 + -2) = uVar4;
              uVar16 = *puVar24;
              *puVar24 = *puVar20;
              *puVar20 = uVar16;
              puVar24 = puVar24 + 3;
              puVar20 = puVar20 + -3;
              uVar29 = uVar29 - 1;
            } while (uVar29 != 0);
          }
          local_f1 = 1;
          perchance::PerchanceContext::uniform_f32();
          local_3ec = extraout_XMM0_Da_03 * __real_be99999a;
          memcpy(&local_3d8,&local_4d8,0x100);
          local_2b8 = 0;
          local_2b0 = 0;
          local_240 = 0;
          local_248 = 0;
          local_25c = 0;
          local_2c8 = __xmm_3f800000000000000000000000000000;
          uStack_2c0 = DAT_14293e0c8;
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
          local_258 = __xmm_3f800000000000000000000000000000;
          uStack_250 = DAT_14293e0c8;
          local_23c = 0x3f8000003f800000;
          local_234 = 0x3f800000;
          local_f1 = 0;
          bevy_ecs::system::commands::Commands::
          spawn<tuple_<country_core::resources::critters::Duck,enum2_<country_core::resources::critters::DuckState>,country_core::components::drawable::DrawableMeshBundle,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier,country_core::components::drawable::PrevTransform>_>
                    ();
          if (local_100 != (undefined8 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          puVar19 = puVar19 + 4;
          puVar32 = puVar2;
        }
      }
      else {
        uVar22 = *(ulonglong *)(lVar25 + 0x20) & 0xffffffff;
        if ((uVar22 < *(ulonglong *)(local_150 + 0x10)) &&
           (*(int *)(*(longlong *)(local_150 + 8) + uVar22 * 0x14) ==
            (int)(*(ulonglong *)(lVar25 + 0x20) >> 0x20))) {
          lVar7 = *(longlong *)(local_150 + 8) + uVar22 * 0x14;
          uVar22 = (ulonglong)*(uint *)(lVar7 + 4);
          if ((uVar22 != 0xffffffff) &&
             ((uVar22 < *(ulonglong *)(local_128 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)(local_128 + 0x28) + (ulonglong)(*(uint *)(lVar7 + 4) >> 6) * 8) >>
                (uVar22 & 0x3f) & 1) != 0)))) {
            uVar22 = (ulonglong)*(uint *)(lVar7 + 0x10);
            lVar14 = *(longlong *)
                      (*(longlong *)(local_150 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar7 + 0xc) * 0x48);
            lVar17 = *(longlong *)
                      (*(longlong *)(local_150 + 0x1a8) + 0x38 +
                      (ulonglong)*(uint *)(lVar7 + 0xc) * 0x48);
            lVar34 = ~*(ulonglong *)(lVar17 + *(longlong *)(local_128 + 0x118) * 8) * 0x30;
            lVar7 = *(longlong *)(lVar14 + 0x10 + lVar34);
            lVar34 = *(longlong *)(lVar14 + 0x28 + lVar34);
            lVar17 = ~*(ulonglong *)(lVar17 + *(longlong *)(local_128 + 0x120) * 8) * 0x30;
            puVar30 = *(undefined1 **)(lVar14 + 0x10 + lVar17);
            puVar19 = *(undefined8 **)(lVar14 + 0x28 + lVar17);
            *(undefined4 *)(lVar34 + uVar22 * 4) = local_134;
            lVar14 = *(longlong *)(lVar25 + 8);
            uStack_3c0 = *(longlong *)(lVar25 + 0x10);
            *(undefined8 *)(lVar25 + 0x10) = 0;
            lVar17 = uStack_3c0 * 0x20 + lVar14;
            local_3b8 = 0.0;
            uStack_3b4 = 0;
            local_3c8 = (undefined1 *)lVar25;
            uStack_3d0 = (undefined8 *)lVar17;
            local_3d8 = (undefined8 *)lVar14;
            if (uStack_3c0 != 0) {
              lVar25 = lVar17 + -0x20;
              pcVar23 = *(char **)(lVar17 + -0x20);
              uStack_3d0 = (undefined8 *)lVar25;
              if (!SBORROW8(0,(longlong)pcVar23)) {
                lVar7 = lVar7 + uVar22 * 0x100;
                local_120 = *(undefined8 **)(lVar17 + -0x18);
                uVar29 = *(ulonglong *)(lVar17 + -0x10);
                *(undefined8 *)(lVar7 + 0xa0) = 0;
                local_140 = lVar34;
                local_130 = pcVar23;
                local_108 = puVar30;
                local_100 = puVar19;
                if (*(ulonglong *)(lVar7 + 0x90) < uVar29) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  lVar17 = *(longlong *)(lVar7 + 0xa0);
                }
                else {
                  lVar17 = 0;
                }
                memcpy((void *)(lVar17 * 0xc + *(longlong *)(lVar7 + 0x98)),local_120,uVar29 * 0xc);
                *(ulonglong *)(lVar7 + 0xa0) = lVar17 + uVar29;
                if (local_130 != (char *)0x0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                puVar19 = local_100;
                puVar30 = local_108;
                lVar17 = local_140;
                if (lVar25 - lVar14 != 0) {
                  uVar29 = (ulonglong)(lVar25 - lVar14) >> 5;
                  lVar14 = lVar14 + 8;
                  do {
                    if (*(longlong *)(lVar14 + -8) != 0) {
                      std::alloc::__default_lib_allocator::__rust_dealloc();
                    }
                    lVar14 = lVar14 + 0x20;
                    uVar29 = uVar29 - 1;
                  } while (uVar29 != 0);
                }
                *(undefined4 *)(lVar17 + uVar22 * 4) = local_134;
                uVar29 = *(ulonglong *)(lVar7 + 0xa0);
                if (1 < uVar29) {
                  uVar15 = uVar29 >> 1;
                  puVar24 = (undefined4 *)(*(longlong *)(lVar7 + 0x98) + uVar29 * 0xc + -4);
                  puVar20 = (undefined4 *)(*(longlong *)(lVar7 + 0x98) + 8);
                  do {
                    uVar4 = *(undefined8 *)(puVar20 + -2);
                    *(undefined8 *)(puVar20 + -2) = *(undefined8 *)(puVar24 + -2);
                    *(undefined8 *)(puVar24 + -2) = uVar4;
                    uVar16 = *puVar20;
                    *puVar20 = *puVar24;
                    *puVar24 = uVar16;
                    puVar20 = puVar20 + 3;
                    puVar24 = puVar24 + -3;
                    uVar15 = uVar15 - 1;
                  } while (uVar15 != 0);
                }
                *(undefined4 *)(lVar17 + uVar22 * 4) = local_134;
                *(undefined4 *)(lVar7 + 0xec) = 0x3f800000;
                *(undefined4 *)((longlong)puVar19 + uVar22 * 4) = local_134;
                puVar30[uVar22] = 3;
                goto LAB_142310830;
              }
            }
                    /* WARNING: Subroutine does not return */
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
        plVar21 = extraout_RAX_05 + 1;
      }
      else if ((*extraout_RAX_05 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar21 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
LAB_1423114ee:
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      if (*plVar21 != 0) {
LAB_142311543:
                    /* WARNING: Subroutine does not return */
        core::cell::panic_already_borrowed();
      }
      *plVar21 = -1;
      local_118 = plVar21;
      puffin::ThreadProfiler::end_scope();
      *local_118 = *local_118 + 1;
    }
  }
  return;
}

