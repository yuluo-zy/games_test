// name: ui_lintel_controls
// addr: 0x140771a60
// size: 2719
// signature: void __cdecl ui_lintel_controls(void)


void __cdecl system_wall_constructor::ui_lintel_controls::ui_lintel_controls(void)

{
  byte *pbVar1;
  longlong lVar2;
  longlong lVar3;
  float *pfVar4;
  undefined1 auVar5 [16];
  byte bVar6;
  byte bVar7;
  undefined4 uVar8;
  undefined4 *puVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 uVar12;
  void *_Src;
  float *pfVar13;
  undefined1 auVar14 [16];
  ushort uVar15;
  int iVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  char extraout_AL;
  byte extraout_AL_00;
  byte bVar28;
  char extraout_AL_01;
  uint uVar29;
  uint uVar30;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar31;
  code *pcVar32;
  undefined8 extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong lVar33;
  undefined8 *puVar34;
  ulonglong uVar35;
  undefined8 **extraout_RAX_13;
  float *pfVar36;
  undefined8 *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong *extraout_RAX_16;
  longlong *extraout_RAX_17;
  longlong *extraout_RAX_18;
  undefined8 *in_RCX;
  longlong *plVar37;
  undefined8 **ppuVar38;
  float *pfVar39;
  ulonglong in_RDX;
  undefined8 **ppuVar40;
  ulonglong *puVar41;
  int *piVar42;
  ulonglong uVar43;
  undefined *puVar44;
  code *pcVar45;
  float *pfVar46;
  longlong *in_R9;
  longlong lVar47;
  undefined1 (*pauVar48) [16];
  undefined1 (*pauVar49) [16];
  bool bVar50;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  float fVar55;
  float fVar56;
  float fVar57;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  longlong *in_stack_00000060;
  ulonglong local_3d8;
  undefined8 **ppuStack_3d0;
  code *pcStack_3c8;
  longlong *plStack_3c0;
  int local_348;
  undefined2 uStack_344;
  undefined2 uStack_342;
  float local_340;
  undefined2 uStack_33c;
  undefined2 uStack_33a;
  byte local_338;
  byte bStack_337;
  undefined2 uStack_336;
  float fStack_334;
  code *local_330;
  code *local_328;
  code *local_320;
  code *local_318;
  undefined8 **local_310;
  code *pcStack_308;
  undefined8 **local_300;
  code *pcStack_2f8;
  undefined8 uStack_248;
  longlong local_230;
  undefined1 (*local_228) [16];
  undefined4 *local_220;
  longlong local_218;
  undefined4 *local_210;
  undefined8 local_208;
  ulonglong local_200;
  float local_1f8;
  undefined8 local_1f0;
  undefined4 local_1e8;
  undefined8 local_1e0;
  float local_1d8;
  longlong local_1d0;
  undefined4 uStack_1c0;
  undefined4 local_1bc;
  undefined8 **ppuStack_1b8;
  undefined8 local_1b0;
  undefined8 **local_1a8;
  code *pcStack_1a0;
  undefined8 **local_198;
  code *pcStack_190;
  float local_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  code *pcStack_178;
  undefined8 local_170;
  ulonglong *local_168;
  undefined1 (*local_160) [16];
  undefined4 local_158;
  undefined4 local_154;
  longlong *local_150;
  code *local_148;
  undefined8 **ppuStack_140;
  code *local_138;
  undefined8 local_130;
  ulonglong local_128;
  undefined8 **ppuStack_120;
  code *local_118;
  int *local_110;
  uint local_fc;
  undefined8 *local_f8;
  longlong *local_f0;
  undefined1 uStack_e6;
  undefined1 local_e5;
  undefined1 uStack_e4;
  undefined1 uStack_e3;
  undefined1 uStack_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_1d0 = 0;
    if (*(int *)*in_RCX != 0) {
      return;
    }
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x3f) {
          if ((char)(&DAT_142a7edc0)[in_RDX] < -0x40) {
LAB_140771b1e:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x3f) goto LAB_140771b1e;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x3f) {
          if ("stem_wall_constructor::ui_lintel_controls::ui_lintel_controls::f"[in_RDX] < -0x40) {
LAB_140771b87:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x3f) goto LAB_140771b87;
      }
    }
    puVar44 = &DAT_142a7ef27;
    do {
      bVar28 = puVar44[-1];
      uVar29 = (uint)(char)bVar28;
      if ((char)bVar28 < '\0') {
        bVar6 = puVar44[-2];
        if ((char)bVar6 < -0x40) {
          bVar7 = puVar44[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = puVar44 + -4;
            puVar44 = puVar44 + -4;
            uVar29 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar44 = puVar44 + -3;
            uVar29 = bVar7 & 0xf;
          }
          uVar29 = bVar6 & 0x3f | uVar29 << 6;
        }
        else {
          puVar44 = puVar44 + -2;
          uVar29 = bVar6 & 0x1f;
        }
        uVar29 = (uint)(bVar28 & 0x3f) | uVar29 << 6;
        if (uVar29 != 0x5c) goto LAB_140771c54;
LAB_140771c6b:
        if (puVar44 == &UNK_142a7eeed) break;
        if (puVar44 + -0x142a7eeed < (undefined *)0x39) {
          if (-0x41 < (char)puVar44[1]) break;
        }
        else if (puVar44 + -0x142a7eeed == (undefined *)0x39) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar44 = puVar44 + -1;
      if (uVar29 == 0x5c) goto LAB_140771c6b;
LAB_140771c54:
      if (uVar29 == 0x2f) goto LAB_140771c6b;
    } while (puVar44 != &DAT_142a7eeee);
    local_f0 = in_R9;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar37 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar37 = extraout_RAX_16, extraout_RAX_16 == (longlong *)0x0))
    goto LAB_140772fde;
    if (*plVar37 != 0) goto LAB_140772fea;
    *plVar37 = -1;
    local_150 = plVar37;
    puffin::ThreadProfiler::begin_scope();
    *local_150 = *local_150 + 1;
    local_1d0 = 1;
    in_R9 = local_f0;
    if (*(int *)*in_RCX != 0) {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_02 == 1) {
        plVar37 = extraout_RAX_02 + 1;
      }
      else if ((*extraout_RAX_02 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar37 = extraout_RAX_17, extraout_RAX_17 == (longlong *)0x0))
      goto LAB_140772fde;
      if (*plVar37 != 0) goto LAB_140772fea;
      *plVar37 = -1;
      local_f0 = plVar37;
      puffin::ThreadProfiler::end_scope();
      goto LAB_140772f2e;
    }
  }
  pauVar48 = *(undefined1 (**) [16])*in_stack_00000050;
  local_230 = ((undefined8 *)*in_stack_00000050)[3];
  auVar5 = *pauVar48;
  local_130 = *in_stack_00000028;
  local_154 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
  local_208 = *in_stack_00000058;
  local_210 = (undefined4 *)in_stack_00000058[2];
  local_158 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  local_1b0 = *in_R9;
  local_220 = (undefined4 *)in_R9[2];
  local_1bc = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
  local_218 = *in_stack_00000060;
  puVar9 = (undefined4 *)in_stack_00000060[2];
  local_160 = pauVar48 + 1;
  uVar29 = ~(uint)(ushort)((ushort)(SUB161(auVar5 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar5[0xf] >> 7) << 0xf);
  while (local_230 != 0) {
    if ((short)uVar29 == 0) {
      do {
        auVar5 = *local_160;
        uVar15 = (ushort)(SUB161(auVar5 >> 7,0) & 1) | (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar5[0xf] >> 7) << 0xf;
        pauVar48 = pauVar48 + -0x20;
        local_160 = local_160 + 1;
      } while (uVar15 == 0xffff);
      uVar29 = ~(uint)uVar15;
      local_fc = uVar29 & -(uint)uVar15 - 2;
      pauVar49 = pauVar48;
    }
    else {
      local_fc = uVar29 - 1 & uVar29;
      pauVar49 = pauVar48;
    }
    iVar16 = 0;
    for (; (uVar29 & 1) == 0; uVar29 = uVar29 >> 1 | 0x80000000) {
      iVar16 = iVar16 + 1;
    }
    uVar30 = iVar16 << 5;
    local_f8 = (undefined8 *)((longlong)pauVar49 + (-0x20 - (ulonglong)uVar30));
    local_228 = pauVar49;
    country_core::resources::walls::public_walls::PublicWalls::get();
    local_230 = local_230 + -1;
    pauVar48 = local_228;
    uVar29 = local_fc;
    if (extraout_RAX_03 != 0) {
      local_148 = (code *)0x0;
      ppuStack_140 = (undefined8 **)&DAT_00000004;
      local_138 = (code *)0x0;
      lVar47 = *(longlong *)((longlong)pauVar49 + (-0x10 - (ulonglong)uVar30));
      lVar33 = *(longlong *)((longlong)pauVar49 + (-8 - (ulonglong)uVar30));
      lVar31 = lVar47;
LAB_140771fc3:
      if (lVar31 != lVar33 * 0x68 + lVar47) {
        lVar2 = lVar31 + 0x68;
        lVar10 = *(longlong *)(lVar31 + 8);
        local_318 = *(code **)(lVar31 + 0x10);
        lVar3 = lVar10 + (longlong)local_318 * 4;
        lVar11 = *(longlong *)(lVar31 + 0x20);
        local_320 = *(code **)(lVar31 + 0x28);
        local_330 = (code *)(lVar11 + (longlong)local_320 * 0xc);
        if (local_318 <= local_320) {
          local_320 = local_318;
        }
        local_348 = (int)lVar10;
        uStack_344 = (undefined2)((ulonglong)lVar10 >> 0x20);
        uStack_342 = (undefined2)((ulonglong)lVar10 >> 0x30);
        local_340 = (float)lVar3;
        uStack_33c = (undefined2)((ulonglong)lVar3 >> 0x20);
        uStack_33a = (undefined2)((ulonglong)lVar3 >> 0x30);
        local_338 = (byte)lVar11;
        bStack_337 = (byte)((ulonglong)lVar11 >> 8);
        uStack_336 = (undefined2)((ulonglong)lVar11 >> 0x10);
        fStack_334 = (float)((ulonglong)lVar11 >> 0x20);
        local_310 = (undefined8 **)0x0;
        pcVar32 = (code *)0x0;
        while (lVar31 = lVar2, pcVar32 < local_320) {
          local_328 = pcVar32 + 1;
          ppuVar38 = (undefined8 **)
                     ((longlong)pcVar32 * 4 + CONCAT26(uStack_342,CONCAT24(uStack_344,local_348)));
          pcVar32 = (code *)((longlong)pcVar32 * 0xc +
                            CONCAT44(fStack_334,CONCAT22(uStack_336,CONCAT11(bStack_337,local_338)))
                            );
          ppuVar40 = local_300;
          pcVar45 = pcStack_2f8;
          if (local_310 == (undefined8 **)0x0) {
            local_128 = 1;
            local_110 = &local_348;
            local_e1 = 1;
            ppuStack_120 = ppuVar38;
            local_118 = pcVar32;
            itertools::tuple_impl::impl$31::
            collect_from_iter_no_buf<tuple_<ref_<f32>,ref_<glam::f32::vec3::Vec3>_>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<ref_<f32>,ref_<glam::f32::vec3::Vec3>_>_>,ref_mut_<core::iter::adapters::zip::Zip<core::slice::iter::Iter<f32>,core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>_>_>
                      ();
            local_310 = local_1a8;
            pcStack_308 = pcStack_1a0;
            local_300 = local_198;
            pcStack_2f8 = pcStack_190;
            pcVar32 = pcStack_190;
            ppuVar38 = local_198;
            ppuVar40 = local_1a8;
            pcVar45 = pcStack_1a0;
            if (local_1a8 == (undefined8 **)0x0) break;
          }
          local_1d8 = *(float *)(pcVar45 + 8);
          local_1e0 = *(undefined8 *)pcVar45;
          local_1e8 = *(undefined4 *)(pcVar32 + 8);
          local_1f0 = *(undefined8 *)pcVar32;
          fVar52 = *(float *)ppuVar40 * 0.0;
          local_1f8 = local_1d8 + fVar52;
          local_200 = CONCAT44(*(float *)ppuVar40 + (float)((ulonglong)local_1e0 >> 0x20),
                               fVar52 + (float)local_1e0);
          local_168 = (ulonglong *)CONCAT44(local_168._4_4_,*(undefined4 *)(pcVar32 + 8));
          local_170 = *(undefined8 *)pcVar32;
          ppuStack_120 = (undefined8 **)CONCAT44(ppuStack_120._4_4_,local_1f8);
          local_e1 = 1;
          local_310 = ppuVar40;
          pcStack_308 = pcVar45;
          local_300 = ppuVar38;
          pcStack_2f8 = pcVar32;
          local_128 = local_200;
          ::utils::geometry::intersect::Triangle::new();
          pcVar32 = local_138;
          if (local_138 == local_148) {
            local_e1 = 1;
            alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>()
            ;
          }
          *(float *)((longlong)ppuStack_140 + ((longlong)pcVar32 * 9 + 8) * 4) = local_188;
          pfVar13 = (float *)((longlong)ppuStack_140 + ((longlong)pcVar32 * 9 + 4) * 4);
          *(undefined8 ***)pfVar13 = local_198;
          *(code **)(pfVar13 + 2) = pcStack_190;
          pfVar13 = (float *)((longlong)ppuStack_140 + (longlong)pcVar32 * 0x24);
          *(undefined8 ***)pfVar13 = local_1a8;
          *(code **)(pfVar13 + 2) = pcStack_1a0;
          local_138 = pcVar32 + 1;
          local_e1 = 1;
          ::utils::geometry::intersect::Triangle::new();
          pcVar32 = local_138;
          if (local_138 == local_148) {
            local_e1 = 1;
            alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>()
            ;
          }
          *(float *)((longlong)ppuStack_140 + ((longlong)pcVar32 * 9 + 8) * 4) = local_188;
          pfVar13 = (float *)((longlong)ppuStack_140 + ((longlong)pcVar32 * 9 + 4) * 4);
          *(undefined8 ***)pfVar13 = local_198;
          *(code **)(pfVar13 + 2) = pcStack_190;
          pfVar13 = (float *)((longlong)ppuStack_140 + (longlong)pcVar32 * 0x24);
          *(undefined8 ***)pfVar13 = local_1a8;
          *(code **)(pfVar13 + 2) = pcStack_1a0;
          local_138 = pcVar32 + 1;
          pcVar32 = local_328;
        }
        goto LAB_140771fc3;
      }
      if (local_138 == (code *)0x0) {
        pauVar48 = local_228;
        uVar29 = local_fc;
        if (local_148 != (code *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
          pauVar48 = local_228;
          uVar29 = local_fc;
        }
      }
      else {
        local_1a8 = &local_f8;
        pcStack_1a0 = core::fmt::impl$74::fmt<country_core::resources::walls::WallId>;
        local_348 = 0x42a7ef50;
        uStack_344 = 1;
        uStack_342 = 0;
        local_340 = 1.4013e-45;
        uStack_33c = 0;
        uStack_33a = 0;
        local_338 = (byte)&local_1a8;
        bStack_337 = (byte)((ulonglong)&local_1a8 >> 8);
        uStack_336 = (undefined2)((ulonglong)&local_1a8 >> 0x10);
        fStack_334 = (float)((ulonglong)&local_1a8 >> 0x20);
        local_330 = (code *)0x1;
        local_328 = (code *)0x0;
        local_e1 = 1;
        alloc::fmt::format::format_inner();
        local_128 = local_3d8;
        ppuStack_120 = ppuStack_3d0;
        local_118 = pcStack_3c8;
        pcStack_190 = local_138;
        pcStack_1a0 = local_148;
        local_198 = ppuStack_140;
        local_1a8 = (undefined8 **)CONCAT71(local_1a8._1_7_,2);
        local_170 = CONCAT62(local_170._2_6_,0xc01);
        local_e5 = 1;
        country_core::systems::ui::resources::widget_params::WidgetParams::new();
        local_e5 = 0;
        puVar41 = &local_128;
        country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        local_e1 = 0;
        local_170 = extraout_RAX_04;
        local_168 = puVar41;
        country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
        bVar28 = extraout_AL_00 - 2;
        if (2 < bVar28) {
          bVar28 = 3;
        }
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
        switch(bVar28) {
        case 1:
          local_e1 = 0;
          country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting();
          if (local_348 == 2) {
            local_e1 = 0;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          pcStack_178 = local_318;
          local_1a8 = (undefined8 **)CONCAT26(uStack_342,CONCAT24(uStack_344,local_348));
          pcStack_1a0 = (code *)CONCAT26(uStack_33a,CONCAT24(uStack_33c,local_340));
          local_198 = (undefined8 **)
                      CONCAT44(fStack_334,CONCAT22(uStack_336,CONCAT11(bStack_337,local_338)));
          local_188 = SUB84(local_328,0);
          uStack_184 = (undefined4)((ulonglong)local_328 >> 0x20);
          uStack_180 = SUB84(local_320,0);
          uStack_17c = (undefined4)((ulonglong)local_320 >> 0x20);
          pcStack_190 = local_330;
          *(undefined4 *)(in_stack_00000030 + 1) = uStack_17c;
          *in_stack_00000030 = CONCAT44(uStack_180,uStack_184);
          local_e1 = 0;
          country_core::resources::walls::public_walls::PublicWalls::get();
          if (extraout_RAX_05 == 0) {
            local_e1 = 0;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          *(undefined4 *)((longlong)in_stack_00000030 + 0xc) =
               *(undefined4 *)(extraout_RAX_05 + 0x288);
          break;
        case 2:
          local_e1 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size();
          local_e1 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
          transform_screen_space();
          local_340 = *(float *)(in_stack_00000030 + 1);
          uVar12 = *in_stack_00000030;
          local_348 = (int)uVar12;
          uStack_344 = (undefined2)((ulonglong)uVar12 >> 0x20);
          uStack_342 = (undefined2)((ulonglong)uVar12 >> 0x30);
          uStack_33c = 0;
          uStack_33a = 0;
          local_338 = 0;
          bStack_337 = 0;
          uStack_336 = 0x3f80;
          fStack_334 = 0.0;
          local_128 = 0x7f7fffffff7fffff;
          ppuStack_120 = (undefined8 **)((ulonglong)ppuStack_120 & 0xffffffffffffff00);
          local_e1 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
          project_onto_ray();
          fVar52 = uStack_248._4_4_ - *(float *)((longlong)in_stack_00000030 + 4);
          local_e1 = 0;
          country_core::resources::walls::inner_walls::InnerWalls::get();
          if (extraout_RAX_06 == 0) {
            local_e1 = 0;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          fVar53 = *(float *)(extraout_RAX_06 + 0x5c);
          fVar52 = fVar52 + *(float *)((longlong)in_stack_00000030 + 0xc);
          fVar51 = 0.5;
          if (0.5 <= fVar52) {
            fVar51 = fVar52;
          }
          fVar52 = 14.0;
          if (fVar51 <= 14.0) {
            fVar52 = fVar51;
          }
          *local_220 = local_158;
          local_e1 = 0;
          country_core::resources::walls::inner_walls::InnerWalls::update_arch_height();
          if (fVar53 != fVar52) {
            uVar8 = *(undefined4 *)(in_stack_00000030 + 1);
            uStack_33a = (undefined2)uVar8;
            local_338 = (byte)((uint)uVar8 >> 0x10);
            bStack_337 = (byte)((uint)uVar8 >> 0x18);
            uVar12 = *in_stack_00000030;
            uStack_342 = (undefined2)uVar12;
            local_340 = (float)((ulonglong)uVar12 >> 0x10);
            uStack_33c = (undefined2)((ulonglong)uVar12 >> 0x30);
            *puVar9 = local_1bc;
            lVar47 = *(longlong *)(local_218 + 0x30);
            uVar12 = *(undefined8 *)(local_218 + 0x40);
            if (lVar47 == *(longlong *)(local_218 + 0x20)) {
              local_e1 = 0;
              alloc::raw_vec::RawVec::
              grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                        ();
            }
            lVar33 = *(longlong *)(local_218 + 0x28);
            *(undefined8 *)(lVar33 + lVar47 * 0x28) = uVar12;
            *(undefined2 *)(lVar33 + 8 + lVar47 * 0x28) = 10;
            puVar34 = (undefined8 *)(lVar33 + 10 + lVar47 * 0x28);
            *puVar34 = CONCAT26(uStack_342,CONCAT24(uStack_344,local_348));
            puVar34[1] = CONCAT26(uStack_33a,CONCAT24(uStack_33c,local_340));
            *(ushort *)(lVar33 + 0x1a + lVar47 * 0x28) = CONCAT11(bStack_337,local_338);
            *(undefined4 *)(lVar33 + 0x1c + lVar47 * 0x28) = 0x3f800000;
            *(undefined1 *)(lVar33 + 0x20 + lVar47 * 0x28) = 0;
            *(longlong *)(local_218 + 0x30) = lVar47 + 1;
            *(longlong *)(local_218 + 0x40) = *(longlong *)(local_218 + 0x40) + 1;
          }
          break;
        case 3:
          local_e1 = 0;
          country_core::resources::walls::public_walls::PublicWalls::get();
          if (extraout_RAX_07 == 0) {
            local_e1 = 0;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          fVar52 = *(float *)(extraout_RAX_07 + 0x288);
          fVar53 = *(float *)((longlong)in_stack_00000030 + 0xc);
          fVar51 = ABS(fVar52 - fVar53);
          if ((fVar51 < 0.05 & extraout_AL_00) == 0) {
            if (0.05 <= fVar51) {
              local_e1 = 0;
              country_core::resources::history::wall_compound_edit::CompoundEditBuilder::
              new_edit_sequence();
              uVar12 = *local_f8;
              local_340 = (float)uVar12;
              uStack_33c = (undefined2)((ulonglong)uVar12 >> 0x20);
              uStack_33a = (undefined2)((ulonglong)uVar12 >> 0x30);
              local_338 = SUB41(fVar53,0);
              bStack_337 = (byte)((uint)fVar53 >> 8);
              uStack_336 = (undefined2)((uint)fVar53 >> 0x10);
              local_348 = 6;
              uStack_344 = 0;
              uStack_342 = 0x8000;
              uStack_e3 = 1;
              piVar42 = &local_348;
              fStack_334 = fVar52;
              country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next();
              *local_210 = local_154;
              local_338 = (byte)local_198;
              bStack_337 = (byte)((ulonglong)local_198 >> 8);
              uStack_336 = (undefined2)((ulonglong)local_198 >> 0x10);
              fStack_334 = (float)((ulonglong)local_198 >> 0x20);
              local_348 = (int)local_1a8;
              uStack_344 = (undefined2)((ulonglong)local_1a8 >> 0x20);
              uStack_342 = (undefined2)((ulonglong)local_1a8 >> 0x30);
              local_340 = SUB84(pcStack_1a0,0);
              uStack_33c = (undefined2)((ulonglong)pcStack_1a0 >> 0x20);
              uStack_33a = (undefined2)((ulonglong)pcStack_1a0 >> 0x30);
              uStack_e3 = 0;
              country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build();
              local_340 = (float)extraout_RAX_10;
              uStack_33c = (undefined2)((ulonglong)extraout_RAX_10 >> 0x20);
              uStack_33a = (undefined2)((ulonglong)extraout_RAX_10 >> 0x30);
              local_338 = (byte)piVar42;
              bStack_337 = (byte)((ulonglong)piVar42 >> 8);
              uStack_336 = (undefined2)((ulonglong)piVar42 >> 0x10);
              fStack_334 = (float)((ulonglong)piVar42 >> 0x20);
              local_348 = 7;
              uStack_344 = 0;
              uStack_342 = 0;
              uStack_e3 = 0;
              country_core::resources::history::History::add();
            }
          }
          else {
            local_e1 = 0;
            country_core::resources::walls::public_walls::PublicWalls::get();
            if (extraout_RAX_08 == 0) {
              local_e1 = 0;
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            bVar28 = *(byte *)(extraout_RAX_08 + 0x294);
            *local_220 = local_158;
            local_e1 = 0;
            country_core::resources::walls::inner_walls::InnerWalls::update_arch_type();
            local_e1 = 0;
            country_core::resources::history::wall_compound_edit::CompoundEditBuilder::
            new_edit_sequence();
            uVar12 = *local_f8;
            local_340 = (float)uVar12;
            uStack_33c = (undefined2)((ulonglong)uVar12 >> 0x20);
            uStack_33a = (undefined2)((ulonglong)uVar12 >> 0x30);
            local_348 = 7;
            uStack_344 = 0;
            uStack_342 = 0x8000;
            uStack_e4 = 1;
            piVar42 = &local_348;
            local_338 = bVar28;
            bStack_337 = bVar28 ^ 1;
            country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next();
            *local_210 = local_154;
            local_338 = (byte)local_198;
            bStack_337 = (byte)((ulonglong)local_198 >> 8);
            uStack_336 = (undefined2)((ulonglong)local_198 >> 0x10);
            fStack_334 = (float)((ulonglong)local_198 >> 0x20);
            local_348 = (int)local_1a8;
            uStack_344 = (undefined2)((ulonglong)local_1a8 >> 0x20);
            uStack_342 = (undefined2)((ulonglong)local_1a8 >> 0x30);
            local_340 = SUB84(pcStack_1a0,0);
            uStack_33c = (undefined2)((ulonglong)pcStack_1a0 >> 0x20);
            uStack_33a = (undefined2)((ulonglong)pcStack_1a0 >> 0x30);
            uStack_e4 = 0;
            country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build();
            local_340 = (float)extraout_RAX_09;
            uStack_33c = (undefined2)((ulonglong)extraout_RAX_09 >> 0x20);
            uStack_33a = (undefined2)((ulonglong)extraout_RAX_09 >> 0x30);
            local_338 = (byte)piVar42;
            bStack_337 = (byte)((ulonglong)piVar42 >> 8);
            uStack_336 = (undefined2)((ulonglong)piVar42 >> 0x10);
            fStack_334 = (float)((ulonglong)piVar42 >> 0x20);
            local_348 = 7;
            uStack_344 = 0;
            uStack_342 = 0;
            uStack_e4 = 0;
            country_core::resources::history::History::add();
          }
        }
        local_e1 = 0;
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_11 == 0) {
          local_e1 = 0;
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        uStack_1c0 = *(undefined4 *)(extraout_RAX_11 + 0x288);
        local_e1 = 0;
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_12 == 0) {
          local_e1 = 0;
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        uStack_e6 = *(code *)(extraout_RAX_12 + 0x294);
        local_e1 = 0;
        country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
        pauVar48 = local_228;
        uVar29 = local_fc;
        if (extraout_AL_01 != '\0') {
          lVar47 = *(longlong *)((longlong)pauVar49 + (-0x10 - (ulonglong)uVar30));
          lVar33 = *(longlong *)((longlong)pauVar49 + (-8 - (ulonglong)uVar30)) * 0x68 + lVar47;
          puVar34 = (undefined8 *)0x0;
          while (pauVar48 = local_228, uVar29 = local_fc, lVar47 != lVar33) {
            pcVar32 = *(code **)(lVar47 + 0x28);
            auVar5._8_8_ = 0;
            auVar5._0_8_ = pcVar32;
            uVar35 = SUB168(auVar5 * ZEXT816(0xc),0);
            uStack_248 = puVar34;
            local_f0 = puVar34;
            if ((SUB168(auVar5 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar35)) {
LAB_140773037:
              local_e1 = 0;
                    // WARNING: Subroutine does not return
              alloc::raw_vec::handle_error();
            }
            _Src = *(void **)(lVar47 + 0x20);
            if (uVar35 == 0) {
              ppuVar38 = (undefined8 **)&DAT_00000004;
              uVar43 = 0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              ppuVar38 = extraout_RAX_13;
              uVar43 = (ulonglong)pcVar32;
              if (extraout_RAX_13 == (undefined8 **)0x0) goto LAB_140773037;
            }
            memcpy(ppuVar38,_Src,uVar35);
            pfVar13 = *(float **)(lVar47 + 8);
            pfVar36 = *(float **)(lVar47 + 0x20);
            uVar35 = *(ulonglong *)(lVar47 + 0x28);
            if (*(ulonglong *)(lVar47 + 0x10) < *(ulonglong *)(lVar47 + 0x28)) {
              uVar35 = *(ulonglong *)(lVar47 + 0x10);
            }
            ppuStack_1b8 = (undefined8 **)lVar47;
            local_128 = uVar43;
            ppuStack_120 = ppuVar38;
            if (uVar43 - (longlong)pcVar32 < uVar35) {
              uStack_e2 = 1;
              local_118 = pcVar32;
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              pcVar32 = local_118;
joined_r0x000140772d35:
              if (0xf < uVar35) {
                auVar14._8_8_ = 0;
                auVar14._0_8_ = uVar35 - 1;
                lVar47 = SUB168(auVar14 * ZEXT816(0xc),0);
                pfVar39 = (float *)((longlong)ppuStack_120 + (longlong)pcVar32 * 0xc);
                if (((((float *)((longlong)ppuStack_120 + ((longlong)pcVar32 * 3 + 1) * 4) <=
                       (float *)((longlong)pfVar39 + lVar47 + 4)) &&
                     (SUB168(auVar14 * ZEXT816(0xc),8) == 0)) &&
                    ((float *)((longlong)ppuStack_120 + ((longlong)pcVar32 * 3 + 2) * 4) <=
                     (float *)((longlong)pfVar39 + lVar47 + 8))) &&
                   (pfVar39 <= (float *)(lVar47 + (longlong)pfVar39))) {
                  pfVar46 = (float *)((longlong)ppuStack_120 + (longlong)pcVar32 * 0xc);
                  if ((pfVar13 + uVar35 <= pfVar46 || pfVar46 + uVar35 * 3 <= pfVar13) &&
                     (pfVar36 + uVar35 * 3 <= pfVar46 || pfVar46 + uVar35 * 3 <= pfVar36)) {
                    uVar43 = uVar35 & 0xfffffffffffffffc;
                    pcVar32 = (code *)((longlong)pcVar32 + uVar43);
                    pfVar46 = pfVar36 + uVar35 * 3 + -0xc;
                    lVar47 = 0;
                    do {
                      pfVar4 = pfVar13 + uVar35 + lVar47 + -4;
                      fVar52 = *pfVar4;
                      fVar53 = pfVar4[1];
                      fVar51 = pfVar4[3];
                      fVar17 = *pfVar46;
                      fVar18 = pfVar46[1];
                      fVar19 = pfVar46[2];
                      fVar20 = pfVar46[3];
                      fVar21 = pfVar46[4];
                      fVar22 = pfVar46[5];
                      fVar23 = pfVar46[6];
                      fVar24 = pfVar46[8];
                      fVar25 = pfVar46[9];
                      fVar26 = pfVar46[10];
                      fVar27 = pfVar46[0xb];
                      fVar54 = fVar52 * 0.0;
                      fVar55 = fVar53 * 0.0;
                      fVar56 = pfVar4[2] * 0.0;
                      fVar57 = fVar51 * 0.0;
                      pfVar39[4] = pfVar46[7] + pfVar4[2];
                      pfVar39[5] = fVar56 + fVar24;
                      pfVar39[6] = fVar20 + fVar55;
                      pfVar39[7] = fVar21 + fVar53;
                      *pfVar39 = fVar25 + fVar57;
                      pfVar39[1] = fVar26 + fVar51;
                      pfVar39[2] = fVar57 + fVar27;
                      pfVar39[3] = fVar23 + fVar56;
                      pfVar39[8] = fVar55 + fVar22;
                      pfVar39[9] = fVar17 + fVar54;
                      pfVar39[10] = fVar18 + fVar52;
                      pfVar39[0xb] = fVar54 + fVar19;
                      pfVar46 = pfVar46 + -0xc;
                      lVar47 = lVar47 + -4;
                      pfVar39 = pfVar39 + 0xc;
                    } while (-lVar47 != uVar43);
                    bVar50 = uVar35 == uVar43;
                    uVar35 = (ulonglong)((uint)uVar35 & 3);
                    if (bVar50) goto LAB_140772b0b;
                  }
                }
              }
              pfVar39 = (float *)((longlong)ppuStack_120 + ((longlong)pcVar32 * 3 + 2) * 4);
              pfVar36 = pfVar36 + uVar35 * 3 + -1;
              lVar47 = 0;
              do {
                fVar53 = pfVar13[uVar35 + lVar47 + -1] * 0.0;
                fVar52 = *pfVar36;
                *(ulonglong *)(pfVar39 + -2) =
                     CONCAT44(pfVar13[uVar35 + lVar47 + -1] +
                              (float)((ulonglong)*(undefined8 *)(pfVar36 + -2) >> 0x20),
                              fVar53 + (float)*(undefined8 *)(pfVar36 + -2));
                *pfVar39 = fVar52 + fVar53;
                lVar47 = lVar47 + -1;
                pfVar39 = pfVar39 + 3;
                pfVar36 = pfVar36 + -3;
              } while (-lVar47 != uVar35);
              pcVar32 = (code *)((longlong)pcVar32 - lVar47);
            }
            else if (uVar35 != 0) goto joined_r0x000140772d35;
LAB_140772b0b:
            local_118 = pcVar32;
            if (pcVar32 == (code *)0x0) {
              uStack_e2 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            puVar34 = *ppuStack_120;
            local_348 = (int)puVar34;
            uStack_344 = (undefined2)((ulonglong)puVar34 >> 0x20);
            uStack_342 = (undefined2)((ulonglong)puVar34 >> 0x30);
            local_340 = *(float *)(ppuStack_120 + 1);
            if (pcVar32 == (code *)local_128) {
              uStack_e2 = 1;
              alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
            }
            *(float *)((longlong)ppuStack_120 + ((longlong)pcVar32 * 3 + 2) * 4) = local_340;
            *(ulonglong *)((longlong)ppuStack_120 + (longlong)pcVar32 * 0xc) =
                 CONCAT26(uStack_342,CONCAT24(uStack_344,local_348));
            local_118 = (code *)((longlong)pcVar32 + 1);
            local_348 = (int)&local_f8;
            uStack_344 = (undefined2)((ulonglong)&local_f8 >> 0x20);
            uStack_342 = (undefined2)((ulonglong)&local_f8 >> 0x30);
            local_340 = 10.05455;
            uStack_33c = 1;
            uStack_33a = 0;
            local_338 = (byte)&uStack_248;
            bStack_337 = (byte)((ulonglong)&uStack_248 >> 8);
            uStack_336 = (undefined2)((ulonglong)&uStack_248 >> 0x10);
            fStack_334 = (float)((ulonglong)&uStack_248 >> 0x20);
            local_330 = core::fmt::num::impl$89::fmt;
            local_328 = (code *)&uStack_1c0;
            local_320 = core::fmt::float::impl$4::fmt;
            local_318 = (code *)&uStack_e6;
            local_310 = (undefined8 **)country_core::resources::walls::inner_wall_state::impl$7::fmt
            ;
            local_1a8 = (undefined8 **)&PTR_DAT_142a7f0f0;
            pcStack_1a0 = (code *)0x4;
            local_198 = (undefined8 **)&local_348;
            pcStack_190 = (code *)0x4;
            local_188 = 0.0;
            uStack_184 = 0;
            uStack_e2 = 1;
            alloc::fmt::format::format_inner();
            pcVar32 = local_118;
            ppuVar38 = ppuStack_120;
            uVar35 = local_128;
            local_150 = plStack_3c0;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_14 == (undefined8 *)0x0) {
              local_f0 = (longlong *)uVar35;
              ppuStack_1b8 = ppuVar38;
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_14 = 1;
            extraout_RAX_14[1] = 1;
            extraout_RAX_14[2] = uVar35;
            extraout_RAX_14[3] = ppuVar38;
            extraout_RAX_14[4] = pcVar32;
            country_core::systems::ui::signifiers::SignifierFactory::new();
            local_188 = SUB84(local_328,0);
            uStack_184 = (undefined4)((ulonglong)local_328 >> 0x20);
            local_1a8 = (undefined8 **)CONCAT26(uStack_342,CONCAT24(uStack_344,local_348));
            pcStack_1a0 = (code *)CONCAT26(uStack_33a,CONCAT24(uStack_33c,local_340));
            local_198 = (undefined8 **)
                        CONCAT44(fStack_334,CONCAT22(uStack_336,CONCAT11(bStack_337,local_338)));
            pcStack_190 = local_330;
            country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
            if (local_150 != (longlong *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            puVar34 = (undefined8 *)((longlong)local_f0 + 1);
            lVar47 = (longlong)ppuStack_1b8 + 0x68;
          }
        }
      }
    }
  }
  if (local_1d0 == 0) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_15 == 1) {
    plVar37 = extraout_RAX_15 + 1;
  }
  else if ((*extraout_RAX_15 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar37 = extraout_RAX_18, extraout_RAX_18 == (longlong *)0x0)) {
LAB_140772fde:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*plVar37 == 0) {
    *plVar37 = -1;
    local_f0 = plVar37;
    puffin::ThreadProfiler::end_scope();
LAB_140772f2e:
    *local_f0 = *local_f0 + 1;
    return;
  }
LAB_140772fea:
                    // WARNING: Subroutine does not return
  core::cell::panic_already_borrowed();
}

