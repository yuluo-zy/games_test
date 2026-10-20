// name: ui_arch_controls
// addr: 0x140766e70
// size: 2622
// signature: void __cdecl ui_arch_controls(void)


void __cdecl system_wall_constructor::ui_arch_controls::ui_arch_controls(void)

{
  byte *pbVar1;
  float *pfVar2;
  undefined1 auVar3 [16];
  byte bVar4;
  byte bVar5;
  byte bVar6;
  int iVar7;
  undefined8 uVar8;
  longlong lVar9;
  longlong lVar10;
  undefined8 uVar11;
  longlong lVar12;
  ushort uVar13;
  undefined1 (*pauVar14) [16];
  uint uVar15;
  undefined1 (*pauVar16) [16];
  char extraout_AL;
  char extraout_AL_00;
  uint uVar17;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *plVar18;
  undefined8 *puVar19;
  undefined8 extraout_RAX_05;
  undefined1 (*extraout_RAX_06) [16];
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  undefined8 *in_RCX;
  undefined1 (*pauVar20) [16];
  undefined8 **ppuVar21;
  longlong *in_RDX;
  ulonglong uVar22;
  undefined8 **ppuVar23;
  undefined8 *in_R8;
  undefined *puVar24;
  code *pcVar25;
  undefined8 *in_R9;
  float fVar26;
  longlong *in_stack_00000028;
  undefined8 local_260;
  undefined8 **ppuStack_258;
  undefined8 *local_250;
  undefined **local_248;
  longlong local_240;
  undefined8 ***local_238;
  longlong local_230;
  ulonglong local_228;
  ulonglong local_220;
  ulonglong local_218;
  undefined8 **local_210;
  undefined8 *puStack_208;
  undefined8 **local_200;
  undefined8 *puStack_1f8;
  undefined8 local_198;
  undefined8 *local_170;
  undefined8 local_168;
  float local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined4 local_140;
  undefined8 local_138;
  undefined8 **ppuStack_130;
  undefined8 *local_128;
  undefined ***local_120;
  undefined8 **local_118;
  code *pcStack_110;
  undefined8 **local_108;
  undefined8 *puStack_100;
  float local_f8;
  undefined8 *local_f0;
  longlong local_e8;
  undefined8 local_e0;
  float local_d8;
  undefined8 local_d0;
  float local_c8;
  undefined8 local_c0;
  float local_b8;
  undefined8 *local_b0;
  undefined1 (*local_a0) [16];
  undefined1 (*local_98) [16];
  undefined8 *local_90;
  undefined8 **ppuStack_88;
  undefined8 *local_80;
  uint local_74;
  longlong *local_70;
  undefined1 local_63;
  char local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_198 = 0;
    iVar7 = *(int *)*in_RCX;
  }
  else {
    uVar22 = 0x3b;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar22 != 0) {
        if (uVar22 < 0x3b) {
          if ((char)(&DAT_142a7deb8)[uVar22] < -0x40) {
LAB_140766f00:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar22 != 0x3b) goto LAB_140766f00;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_01 == 1) && (uVar22 != 0xfffffffffffffffe)) {
        if (uVar22 + 2 < 0x3b) {
          if ((char)(&UNK_142a7deba)[uVar22] < -0x40) {
LAB_140766fbb:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar22 + 2 != 0x3b) goto LAB_140766fbb;
      }
    }
    puVar24 = &DAT_142a7e07d;
    do {
      bVar4 = puVar24[-1];
      uVar17 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar24[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar24[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar24 + -4;
            puVar24 = puVar24 + -4;
            uVar17 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar24 = puVar24 + -3;
            uVar17 = bVar6 & 0xf;
          }
          uVar17 = bVar5 & 0x3f | uVar17 << 6;
        }
        else {
          puVar24 = puVar24 + -2;
          uVar17 = bVar5 & 0x1f;
        }
        uVar17 = (uint)(bVar4 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_140767084;
LAB_14076709d:
        if (puVar24 == &UNK_142a7e045) break;
        if (puVar24 + -0x142a7e045 < (undefined *)0x37) {
          if (-0x41 < (char)puVar24[1]) break;
        }
        else if (puVar24 + -0x142a7e045 == (undefined *)0x37) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar24 = puVar24 + -1;
      if (uVar17 == 0x5c) goto LAB_14076709d;
LAB_140767084:
      if (uVar17 == 0x2f) goto LAB_14076709d;
    } while (puVar24 != &DAT_142a7e046);
    local_b0 = in_RCX;
    local_70 = in_RDX;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      pauVar20 = (undefined1 (*) [16])(extraout_RAX_02 + 1);
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), pauVar20 = extraout_RAX_06, extraout_RAX_06 == (undefined1 (*) [16])0x0))
    goto LAB_1407678dd;
    if (*(longlong *)*pauVar20 != 0) goto LAB_1407678e9;
    *(undefined8 *)*pauVar20 = 0xffffffffffffffff;
    local_98 = pauVar20;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)*local_98 = *(longlong *)*local_98 + 1;
    local_198 = 1;
    iVar7 = *(int *)*local_b0;
    in_RDX = local_70;
  }
  if (iVar7 == 0) {
    pauVar20 = *(undefined1 (**) [16])*in_R8;
    local_e8 = ((undefined8 *)*in_R8)[3];
    auVar3 = *pauVar20;
    local_158 = *in_RDX;
    local_150 = *in_R9;
    uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
    pauVar16 = pauVar20 + 1;
    pauVar14 = local_a0;
    uVar17 = local_74;
    local_62 = extraout_AL;
joined_r0x000140767204:
    local_a0 = pauVar16;
    local_74 = uVar15;
    if (local_e8 != 0) {
      do {
        if ((short)local_74 == 0) {
          do {
            auVar3 = *local_a0;
            uVar13 = (ushort)(SUB161(auVar3 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
            pauVar20 = pauVar20 + -0x20;
            local_a0 = local_a0 + 1;
          } while (uVar13 == 0xffff);
          uVar17 = ~(uint)uVar13;
          local_74 = uVar17 & -(uint)uVar13 - 2;
        }
        else {
          uVar17 = local_74;
          local_74 = local_74 - 1 & local_74;
        }
        iVar7 = 0;
        for (; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
          iVar7 = iVar7 + 1;
        }
        local_70 = (longlong *)((longlong)pauVar20 - (ulonglong)(uint)(iVar7 << 5));
        local_f0 = local_70 + -4;
        country_core::resources::walls::public_walls::PublicWalls::get();
        local_e8 = local_e8 + -1;
        uVar15 = local_74;
        pauVar16 = local_a0;
        pauVar14 = local_a0;
        uVar17 = local_74;
        if (extraout_RAX_04 == 0) goto joined_r0x000140767204;
        local_90 = (undefined8 *)0x0;
        ppuStack_88 = (undefined8 **)&DAT_00000004;
        local_80 = (undefined8 *)0x0;
        local_b0 = (undefined8 *)local_70[-2] + local_70[-1] * 0xd;
        plVar18 = (undefined8 *)local_70[-2];
        local_98 = pauVar20;
LAB_1407673a2:
        pauVar20 = local_98;
        if (plVar18 != local_b0) {
          local_70 = plVar18 + 0xd;
          local_248 = (undefined **)plVar18[1];
          local_218 = plVar18[2];
          local_240 = (longlong)local_248 + local_218 * 4;
          local_238 = (undefined8 ***)plVar18[4];
          local_220 = plVar18[5];
          local_230 = (longlong)local_238 + local_220 * 0xc;
          if (local_218 <= local_220) {
            local_220 = local_218;
          }
          local_210 = (undefined8 **)0x0;
          uVar22 = 0;
          while (plVar18 = local_70, uVar22 < local_220) {
            local_228 = uVar22 + 1;
            ppuVar21 = (undefined8 **)(uVar22 * 4 + (longlong)local_248);
            puVar19 = (undefined8 *)(uVar22 * 0xc + (longlong)local_238);
            ppuVar23 = local_200;
            pcVar25 = (code *)puStack_1f8;
            if (local_210 == (undefined8 **)0x0) {
              local_138 = 1;
              local_120 = &local_248;
              local_61 = 1;
              ppuStack_130 = ppuVar21;
              local_128 = puVar19;
              itertools::tuple_impl::impl$31::
              collect_from_iter_no_buf<tuple_<ref_<f32>,ref_<glam::f32::vec3::Vec3>_>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<ref_<f32>,ref_<glam::f32::vec3::Vec3>_>_>,ref_mut_<core::iter::adapters::zip::Zip<core::slice::iter::Iter<f32>,core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>_>_>
                        ();
              local_210 = local_118;
              puStack_208 = (undefined8 *)pcStack_110;
              local_200 = local_108;
              puStack_1f8 = puStack_100;
              puVar19 = puStack_100;
              plVar18 = local_70;
              ppuVar21 = local_108;
              ppuVar23 = local_118;
              pcVar25 = pcStack_110;
              if (local_118 == (undefined8 **)0x0) break;
            }
            local_b8 = *(float *)((longlong)pcVar25 + 8);
            local_c0 = *(undefined8 *)pcVar25;
            local_c8 = *(float *)(puVar19 + 1);
            local_d0 = *puVar19;
            fVar26 = *(float *)ppuVar23 * 0.0;
            local_d8 = local_b8 + fVar26;
            local_138 = CONCAT44(*(float *)ppuVar23 + (float)((ulonglong)local_c0 >> 0x20),
                                 fVar26 + (float)local_c0);
            fVar26 = *(float *)ppuVar21 * 0.0;
            local_160 = local_c8 + fVar26;
            local_168 = CONCAT44(*(float *)ppuVar21 + (float)((ulonglong)local_d0 >> 0x20),
                                 fVar26 + (float)local_d0);
            local_140 = *(undefined4 *)(puVar19 + 1);
            local_148 = *puVar19;
            ppuStack_130 = (undefined8 **)CONCAT44(ppuStack_130._4_4_,local_d8);
            local_61 = 1;
            local_210 = ppuVar23;
            puStack_208 = (undefined8 *)pcVar25;
            local_200 = ppuVar21;
            puStack_1f8 = puVar19;
            local_e0 = local_138;
            ::utils::geometry::intersect::Triangle::new();
            puVar19 = local_80;
            if (local_80 == local_90) {
              local_61 = 1;
              alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>
                        ();
            }
            *(float *)((longlong)ppuStack_88 + ((longlong)puVar19 * 9 + 8) * 4) = local_f8;
            pfVar2 = (float *)((longlong)ppuStack_88 + ((longlong)puVar19 * 9 + 4) * 4);
            *(undefined8 ***)pfVar2 = local_108;
            *(undefined8 **)(pfVar2 + 2) = puStack_100;
            pfVar2 = (float *)((longlong)ppuStack_88 + (longlong)puVar19 * 0x24);
            *(undefined8 ***)pfVar2 = local_118;
            *(code **)(pfVar2 + 2) = pcStack_110;
            local_80 = (undefined8 *)((longlong)puVar19 + 1);
            local_61 = 1;
            ::utils::geometry::intersect::Triangle::new();
            puVar19 = local_80;
            if (local_80 == local_90) {
              local_61 = 1;
              alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>
                        ();
            }
            *(float *)((longlong)ppuStack_88 + ((longlong)puVar19 * 9 + 8) * 4) = local_f8;
            pfVar2 = (float *)((longlong)ppuStack_88 + ((longlong)puVar19 * 9 + 4) * 4);
            *(undefined8 ***)pfVar2 = local_108;
            *(undefined8 **)(pfVar2 + 2) = puStack_100;
            pfVar2 = (float *)((longlong)ppuStack_88 + (longlong)puVar19 * 0x24);
            *(undefined8 ***)pfVar2 = local_118;
            *(code **)(pfVar2 + 2) = pcStack_110;
            local_80 = (undefined8 *)((longlong)puVar19 + 1);
            uVar22 = local_228;
          }
          goto LAB_1407673a2;
        }
        if (local_80 == (undefined8 *)0x0) goto LAB_1407677c0;
        local_118 = &local_f0;
        pcStack_110 = core::fmt::impl$74::fmt<country_core::resources::walls::WallId>;
        local_248 = &PTR_s_arch_height__142a7e0a8;
        local_240 = 1;
        local_230 = 1;
        local_228 = 0;
        local_61 = 1;
        local_238 = &local_118;
        alloc::fmt::format::format_inner();
        local_138 = local_260;
        ppuStack_130 = ppuStack_258;
        local_128 = local_250;
        puStack_100 = local_80;
        pcStack_110 = (code *)local_90;
        local_108 = ppuStack_88;
        local_118 = (undefined8 **)CONCAT71(local_118._1_7_,2);
        local_148 = CONCAT62(local_148._2_6_,0xc01);
        local_63 = 1;
        country_core::systems::ui::resources::widget_params::WidgetParams::new();
        local_63 = 0;
        puVar19 = &local_138;
        country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        local_61 = 0;
        local_170 = puVar19;
        country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
        if (extraout_AL_00 != '\0') {
          local_61 = 0;
          toolbar::mei::core::NodeContext::id();
          uVar8 = *local_f0;
          *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
          lVar9 = *in_stack_00000028;
          lVar10 = *(longlong *)(lVar9 + 0x30);
          uVar11 = *(undefined8 *)(lVar9 + 0x40);
          if (lVar10 == *(longlong *)(lVar9 + 0x20)) {
            local_61 = 0;
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          lVar12 = *(longlong *)(lVar9 + 0x28);
          *(undefined8 *)(lVar12 + lVar10 * 0x18) = extraout_RAX_05;
          *(undefined8 *)(lVar12 + 8 + lVar10 * 0x18) = uVar8;
          *(undefined8 *)(lVar12 + 0x10 + lVar10 * 0x18) = uVar11;
          *(longlong *)(lVar9 + 0x30) = lVar10 + 1;
          *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
          if (local_62 == '\0') {
            return;
          }
          goto LAB_140766f3e;
        }
        pauVar20 = local_98;
        pauVar14 = local_a0;
        uVar17 = local_74;
        if (local_e8 == 0) break;
      } while( true );
    }
    local_74 = uVar17;
    local_a0 = pauVar14;
    if (local_62 == '\0') {
      return;
    }
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar18 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
    goto LAB_1407678dd;
    if (*plVar18 != 0) goto LAB_1407678e9;
    *plVar18 = -1;
    local_70 = plVar18;
    puffin::ThreadProfiler::end_scope();
    goto LAB_140767254;
  }
  if (extraout_AL != '\0') {
LAB_140766f3e:
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_00 == 1) {
      plVar18 = extraout_RAX_00 + 1;
    }
    else if ((*extraout_RAX_00 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_1407678dd:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_1407678e9:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    local_70 = plVar18;
    puffin::ThreadProfiler::end_scope();
LAB_140767254:
    *local_70 = *local_70 + 1;
  }
  return;
LAB_1407677c0:
  uVar15 = local_74;
  pauVar16 = local_a0;
  pauVar14 = local_a0;
  uVar17 = local_74;
  if (local_90 != (undefined8 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    uVar15 = local_74;
    pauVar16 = local_a0;
    pauVar14 = local_a0;
    uVar17 = local_74;
  }
  goto joined_r0x000140767204;
}

