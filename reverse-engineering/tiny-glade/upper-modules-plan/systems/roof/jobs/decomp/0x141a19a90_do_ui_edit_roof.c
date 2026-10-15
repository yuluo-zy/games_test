// name=do_ui_edit_roof
// sig=void __cdecl do_ui_edit_roof(void)
// size=1579

void __cdecl system_roof::ui_edit_roof::do_ui_edit_roof(void)

{
  byte *pbVar1;
  byte bVar2;
  undefined1 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  float *pfVar13;
  float *pfVar14;
  ulonglong *puVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined8 uVar18;
  uint *puVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  longlong lVar23;
  undefined4 *puVar24;
  longlong lVar25;
  undefined8 uVar26;
  undefined8 uVar27;
  undefined8 uVar28;
  undefined8 uVar29;
  undefined8 uVar30;
  undefined8 uVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  undefined8 uVar34;
  char extraout_AL;
  byte bVar35;
  byte extraout_AL_00;
  uint uVar36;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  byte bVar37;
  longlong *in_RCX;
  longlong *plVar38;
  longlong lVar39;
  ulonglong uVar40;
  longlong lVar41;
  undefined8 *in_RDX;
  ulonglong uVar42;
  ulonglong uVar43;
  ulonglong uVar44;
  longlong lVar45;
  undefined8 *puVar46;
  ulonglong uVar47;
  longlong in_R8;
  char *pcVar48;
  longlong lVar49;
  longlong lVar50;
  longlong lVar51;
  ulonglong uVar52;
  longlong lVar53;
  bool bVar54;
  float fVar55;
  float fVar56;
  float unaff_XMM7_Da;
  float fVar57;
  float fVar58;
  float fVar59;
  longlong in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_18c;
  undefined8 uStack_148;
  uint uStack_140;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  longlong *local_a8;
  
  lVar51 = in_stack_00000040[4];
  puVar9 = (undefined4 *)in_stack_00000040[6];
  uVar4 = *(undefined4 *)((longlong)in_stack_00000040 + 0x3c);
  lVar50 = in_stack_00000040[8];
  puVar10 = (undefined4 *)in_stack_00000040[10];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000040 + 0x5c);
  lVar53 = in_stack_00000040[0xc];
  puVar11 = (undefined4 *)in_stack_00000040[0xe];
  lVar23 = in_stack_00000040[0x14];
  puVar24 = (undefined4 *)in_stack_00000040[0x16];
  lVar25 = in_stack_00000040[0x17];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000040 + 0x7c);
  lVar45 = in_stack_00000040[0x10];
  puVar12 = (undefined4 *)in_stack_00000040[0x12];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000040 + 0x9c);
  pfVar13 = (float *)in_stack_00000040[0x18];
  pfVar14 = (float *)in_stack_00000040[0x19];
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar42 = 0x2a;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar42 != 0) {
        if (uVar42 < 0x2a) {
          if ((char)(&DAT_142eaf313)[uVar42] < -0x40) {
LAB_141a19bfd:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar42 != 0x2a) goto LAB_141a19bfd;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar42 != 0xfffffffffffffffe)) {
        if (uVar42 + 2 < 0x2a) {
          if ((char)(&UNK_142eaf315)[uVar42] < -0x40) {
LAB_141a19c5b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar42 + 2 != 0x2a) goto LAB_141a19c5b;
      }
    }
    pcVar48 = "roof_edit_";
    do {
      bVar37 = pcVar48[-1];
      uVar36 = (uint)(char)bVar37;
      if ((char)bVar37 < '\0') {
        bVar35 = pcVar48[-2];
        if ((char)bVar35 < -0x40) {
          bVar2 = pcVar48[-3];
          if ((char)bVar2 < -0x40) {
            pbVar1 = (byte *)(pcVar48 + -4);
            pcVar48 = pcVar48 + -4;
            uVar36 = bVar2 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar48 = pcVar48 + -3;
            uVar36 = bVar2 & 0xf;
          }
          uVar36 = bVar35 & 0x3f | uVar36 << 6;
        }
        else {
          pcVar48 = pcVar48 + -2;
          uVar36 = bVar35 & 0x1f;
        }
        uVar36 = (uint)(bVar37 & 0x3f) | uVar36 << 6;
        if (uVar36 != 0x5c) goto LAB_141a19d14;
LAB_141a19d26:
        if (pcVar48 == "") break;
        if (pcVar48 + -0x142eaf527 < (char *)0x27) {
          if (-0x41 < pcVar48[1]) break;
        }
        else if (pcVar48 + -0x142eaf527 == (char *)0x27) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar48 = pcVar48 + -1;
      if (uVar36 == 0x5c) goto LAB_141a19d26;
LAB_141a19d14:
      if (uVar36 == 0x2f) goto LAB_141a19d26;
    } while (pcVar48 != "crates/systems/roof/src/ui_edit_roof.rsroof_edit_");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar38 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar38 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_141a1a761;
    if (*plVar38 != 0) goto LAB_141a1a871;
    *plVar38 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar38 = *plVar38 + 1;
  }
  puVar15 = (ulonglong *)*in_stack_00000050;
  lVar41 = in_stack_00000050[1];
  uVar42 = *puVar15;
  uVar47 = 0;
  uVar44 = uVar42 - *(ulonglong *)(lVar41 + 0x18);
  if (uVar42 < *(ulonglong *)(lVar41 + 0x18)) {
    uVar44 = uVar47;
  }
  uVar40 = uVar42 - *(ulonglong *)(lVar41 + 0x38);
  if (uVar42 < *(ulonglong *)(lVar41 + 0x38)) {
    uVar40 = uVar47;
  }
  lVar49 = *(longlong *)(lVar41 + 8);
  uVar42 = *(ulonglong *)(lVar41 + 0x30);
  uVar43 = uVar42 - uVar40;
  if (uVar42 < uVar40) {
    uVar43 = uVar47;
  }
  uVar16 = *(ulonglong *)(lVar41 + 0x10);
  uVar52 = uVar16 - uVar44;
  if (uVar16 < uVar44) {
    uVar52 = uVar47;
  }
  lVar39 = *(longlong *)(lVar41 + 0x28);
  uVar47 = *(ulonglong *)(lVar41 + 0x40);
  *puVar15 = (uVar47 - uVar43) - uVar52;
  if ((uVar16 < uVar44 || uVar16 - uVar44 == 0) && (uVar42 < uVar40 || uVar42 - uVar40 == 0)) {
    if (extraout_AL == '\0') {
      return;
    }
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      local_a8 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_a8 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_141a1a761;
    if (*local_a8 != 0) goto LAB_141a1a871;
    *local_a8 = -1;
    puffin::ThreadProfiler::end_scope();
    goto LAB_141a1a642;
  }
  lVar49 = lVar49 + uVar44 * 0x18;
  if (uVar16 < uVar44) {
    lVar49 = 8;
  }
  lVar41 = 0;
  if (uVar44 < uVar16) {
    lVar41 = lVar49 + -0x18 + uVar52 * 0x18;
  }
  lVar39 = lVar39 + uVar40 * 0x18;
  if (uVar42 < uVar40) {
    lVar39 = 8;
  }
  lVar49 = lVar39 + -0x18 + uVar43 * 0x18;
  if (uVar42 <= uVar40) {
    lVar49 = lVar41;
  }
  *puVar15 = uVar47;
  lVar41 = in_RCX[1];
  uVar42 = *(ulonglong *)(lVar49 + 8) & 0xffffffff;
  if ((*(ulonglong *)(lVar41 + 0x10) <= uVar42) ||
     (*(int *)(*(longlong *)(lVar41 + 8) + uVar42 * 0x14) !=
      (int)(*(ulonglong *)(lVar49 + 8) >> 0x20))) {
LAB_141a1a894:
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  lVar39 = *(longlong *)(lVar41 + 8) + uVar42 * 0x14;
  uVar42 = (ulonglong)*(uint *)(lVar39 + 4);
  if ((uVar42 == 0xffffffff) ||
     ((lVar17 = *in_RCX, *(ulonglong *)(lVar17 + 0x38) <= uVar42 ||
      ((*(ulonglong *)(*(longlong *)(lVar17 + 0x28) + (ulonglong)(*(uint *)(lVar39 + 4) >> 6) * 8)
        >> (uVar42 & 0x3f) & 1) == 0)))) goto LAB_141a1a894;
  if (*(ulonglong *)(lVar17 + 0x110) <
      *(ulonglong *)
       (*(longlong *)(lVar41 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar39 + 0xc) * 0x48)) {
    lVar41 = *(longlong *)(lVar41 + 0x1a8) + (ulonglong)*(uint *)(lVar39 + 0xc) * 0x48;
    uVar42 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + *(ulonglong *)(lVar17 + 0x110) * 8);
    if (uVar42 == 0) goto LAB_141a19fa8;
    lVar41 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar42 * 0x30);
  }
  else {
LAB_141a19fa8:
    lVar41 = 0;
  }
  puVar46 = (undefined8 *)((ulonglong)*(uint *)(lVar39 + 0x10) * 0x58 + lVar41);
  bVar35 = *(char *)(lVar49 + 0x10) - 2;
  bVar37 = 3;
  if (bVar35 < 3) {
    bVar37 = bVar35;
  }
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(bVar37) {
  case 1:
    *puVar10 = uVar5;
    lVar51 = *(longlong *)(lVar50 + 0x30);
    uVar18 = *(undefined8 *)(lVar50 + 0x40);
    if (lVar51 == *(longlong *)(lVar50 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar53 = *(longlong *)(lVar50 + 0x28);
    *(undefined8 *)(lVar53 + lVar51 * 0x10) = uVar18;
    *(undefined1 *)(lVar53 + 8 + lVar51 * 0x10) = 0;
    *(longlong *)(lVar50 + 0x30) = lVar51 + 1;
    *(longlong *)(lVar50 + 0x40) = *(longlong *)(lVar50 + 0x40) + 1;
    uVar18 = *puVar46;
    uVar20 = puVar46[7];
    uStack_108 = (undefined4)((ulonglong)puVar46[6] >> 0x20);
    uVar21 = *(undefined8 *)((longlong)puVar46 + 0x24);
    uVar22 = *(undefined8 *)((longlong)puVar46 + 0x2c);
    uStack_10c = (undefined4)((ulonglong)uVar22 >> 0x20);
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    puVar9 = (undefined4 *)*in_RDX;
    *puVar9 = 0;
    *(undefined8 *)(puVar9 + 4) = uVar18;
    *(undefined8 *)(puVar9 + 6) = uVar21;
    *(undefined8 *)(puVar9 + 8) = uVar22;
    *(ulonglong *)(puVar9 + 9) = CONCAT44(uStack_108,uStack_10c);
    *(undefined8 *)(puVar9 + 0xb) = uVar20;
    country_core::resources::walls::inner_walls::InnerWalls::get();
    if (extraout_RAX_03 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    *pfVar14 = *(float *)(extraout_RAX_03 + 0x54);
    uVar4 = *(undefined4 *)(in_R8 + 0x1c);
    puVar9 = *(undefined4 **)(in_R8 + 0x10);
    *puVar9 = uVar4;
    country_core::resources::rt::impl$5::from();
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    archive_current_decorators_in_storage<ref_<str_>_>();
    *puVar9 = uVar4;
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    decorator_cull_allow_wall();
    *pfVar13 = 0.0;
    goto joined_r0x000141a1a11f;
  case 2:
    *puVar11 = uVar6;
    lVar51 = *(longlong *)(lVar53 + 0x30);
    uVar18 = *(undefined8 *)(lVar53 + 0x40);
    if (lVar51 == *(longlong *)(lVar53 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar53 + 0x28) + lVar51 * 8) = uVar18;
    *(longlong *)(lVar53 + 0x30) = lVar51 + 1;
    *(longlong *)(lVar53 + 0x40) = *(longlong *)(lVar53 + 0x40) + 1;
    puVar15 = (ulonglong *)*in_stack_00000038;
    lVar51 = in_stack_00000038[1];
    uVar42 = *puVar15;
    uVar47 = 0;
    uVar44 = uVar42 - *(ulonglong *)(lVar51 + 0x18);
    if (uVar42 < *(ulonglong *)(lVar51 + 0x18)) {
      uVar44 = uVar47;
    }
    uVar40 = uVar42 - *(ulonglong *)(lVar51 + 0x38);
    if (uVar42 < *(ulonglong *)(lVar51 + 0x38)) {
      uVar40 = uVar47;
    }
    lVar50 = *(longlong *)(lVar51 + 8);
    uVar42 = *(ulonglong *)(lVar51 + 0x30);
    uVar43 = uVar42 - uVar40;
    if (uVar42 < uVar40) {
      uVar43 = uVar47;
    }
    uVar16 = *(ulonglong *)(lVar51 + 0x10);
    uVar52 = uVar16 - uVar44;
    if (uVar16 < uVar44) {
      uVar52 = uVar47;
    }
    lVar53 = *(longlong *)(lVar51 + 0x28);
    uVar47 = *(ulonglong *)(lVar51 + 0x40);
    *puVar15 = (uVar47 - uVar43) - uVar52;
    if (uVar16 >= uVar44 && uVar16 - uVar44 != 0 || uVar42 >= uVar40 && uVar42 - uVar40 != 0) {
      lVar53 = lVar53 + uVar40 * 0x10;
      lVar50 = lVar50 + uVar44 * 0x10;
      if (uVar16 < uVar44) {
        lVar50 = 8;
      }
      lVar51 = 0;
      if (uVar44 < uVar16) {
        lVar51 = lVar50 + -0x10 + uVar52 * 0x10;
      }
      if (uVar42 < uVar40) {
        lVar53 = 8;
      }
      lVar50 = lVar53 + -0x10 + uVar43 * 0x10;
      if (uVar42 <= uVar40) {
        lVar50 = lVar51;
      }
      *puVar15 = uVar47;
      fVar56 = (float)*(undefined8 *)(lVar50 + 8);
      fVar55 = (float)((ulonglong)*(undefined8 *)(lVar50 + 8) >> 0x20);
      if (0.0 < SQRT(fVar55 * fVar55 + fVar56 * fVar56)) {
        fVar57 = ABS(fVar56);
        fVar58 = ABS(fVar55);
        if (fVar57 < fVar58) {
          unaff_XMM7_Da = fVar55 * -0.0025;
          fVar55 = *pfVar13;
          fVar59 = unaff_XMM7_Da;
        }
        else {
          fVar56 = fVar56 * -0.0025 + fVar56 * -0.0025;
          fVar55 = ABS(fVar56) + *pfVar13;
          *pfVar13 = fVar55;
          fVar59 = 0.0;
        }
        if (0.2 < fVar55) {
          bevy_ecs::event::writer::EventWriter::
          write<country_core::resources::onboarding::NotifyHintSystemRoofShape>();
        }
        **(undefined4 **)(in_stack_00000030 + 0x10) = *(undefined4 *)(in_stack_00000030 + 0x1c);
        if (*pfVar14 < 1.6) {
          fVar55 = *(float *)(puVar46 + 6);
          if (((0.1 < fVar55) && (fVar59 + fVar55 <= 0.1)) ||
             ((fVar55 <= 0.1 && (0.1 < fVar59 + fVar55)))) {
            country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height();
          }
        }
        uVar18 = *puVar46;
        country_core::resources::roofs::roof_shape::Roof::ty();
        *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
        lVar51 = *in_stack_00000040;
        lVar50 = *(longlong *)(lVar51 + 0x30);
        uVar20 = *(undefined8 *)(lVar51 + 0x40);
        if (lVar50 == *(longlong *)(lVar51 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
        }
        lVar53 = *(longlong *)(lVar51 + 0x28);
        lVar45 = lVar50 * 0x40;
        *(uint *)(lVar53 + lVar45) = (uint)(fVar58 <= fVar57 & extraout_AL_00) * 2;
        *(float *)(lVar53 + 4 + lVar45) = fVar56;
        *(uint *)(lVar53 + 8 + lVar45) = (uint)(fVar57 < fVar58) * 2;
        *(float *)(lVar53 + 0xc + lVar45) = unaff_XMM7_Da;
        *(undefined4 *)(lVar53 + 0x10 + lVar45) = 0;
        *(undefined4 *)(lVar53 + 0x18 + lVar45) = 0;
        *(undefined8 *)(lVar53 + 0x20 + lVar45) = uVar18;
        *(undefined4 *)(lVar53 + 0x28 + lVar45) = 0;
        *(undefined1 *)(lVar53 + 0x34 + lVar45) = 0;
        *(undefined8 *)(lVar53 + 0x38 + lVar45) = uVar20;
        *(longlong *)(lVar51 + 0x30) = lVar50 + 1;
        *(longlong *)(lVar51 + 0x40) = *(longlong *)(lVar51 + 0x40) + 1;
        goto joined_r0x000141a1a11f;
      }
    }
    break;
  case 3:
    *puVar10 = uVar5;
    lVar53 = *(longlong *)(lVar50 + 0x30);
    uVar18 = *(undefined8 *)(lVar50 + 0x40);
    if (lVar53 == *(longlong *)(lVar50 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar41 = *(longlong *)(lVar50 + 0x28);
    *(undefined8 *)(lVar41 + lVar53 * 0x10) = uVar18;
    *(undefined1 *)(lVar41 + 8 + lVar53 * 0x10) = 1;
    *(longlong *)(lVar50 + 0x30) = lVar53 + 1;
    *(longlong *)(lVar50 + 0x40) = *(longlong *)(lVar50 + 0x40) + 1;
    puVar19 = (uint *)*in_RDX;
    if (*puVar19 != 2) {
      if ((((((float)puVar19[8] != *(float *)((longlong)puVar46 + 0x2c)) ||
            (NAN((float)puVar19[8]) || NAN(*(float *)((longlong)puVar46 + 0x2c)))) ||
           ((float)puVar19[9] != *(float *)(puVar46 + 6))) ||
          ((NAN((float)puVar19[9]) || NAN(*(float *)(puVar46 + 6)) ||
           ((float)puVar19[6] != *(float *)((longlong)puVar46 + 0x24))))) ||
         ((((NAN((float)puVar19[6]) || NAN(*(float *)((longlong)puVar46 + 0x24)) ||
            (((float)puVar19[7] != *(float *)(puVar46 + 5) ||
             (NAN((float)puVar19[7]) || NAN(*(float *)(puVar46 + 5)))))) ||
           ((float)puVar19[10] != *(float *)((longlong)puVar46 + 0x34))) ||
          ((((NAN((float)puVar19[10]) || NAN(*(float *)((longlong)puVar46 + 0x34)) ||
             ((char)puVar19[0xc] != *(char *)((longlong)puVar46 + 0x3c))) ||
            ((float)puVar19[0xb] != *(float *)(puVar46 + 7))) ||
           ((NAN((float)puVar19[0xb]) || NAN(*(float *)(puVar46 + 7)) ||
            (*(char *)((longlong)puVar19 + 0x31) != *(char *)((longlong)puVar46 + 0x3d))))))))) {
        bVar54 = (*puVar19 & 1) != 0;
        if (bVar54) {
          uStack_140 = puVar19[3];
          uStack_148 = *(undefined8 *)(puVar19 + 1);
        }
        uVar18 = *puVar46;
        uVar5 = *(undefined4 *)((longlong)puVar46 + 0x4c);
        uVar6 = *(undefined4 *)(puVar46 + 10);
        uVar3 = *(undefined1 *)((longlong)puVar46 + 0x54);
        uVar20 = *(undefined8 *)(puVar19 + 4);
        uVar26 = *(undefined8 *)(puVar19 + 6);
        uVar27 = *(undefined8 *)(puVar19 + 8);
        uVar28 = *(undefined8 *)(puVar19 + 0xb);
        uStack_108 = (undefined4)((ulonglong)*(undefined8 *)(puVar19 + 9) >> 0x20);
        uStack_10c = (undefined4)((ulonglong)uVar27 >> 0x20);
        uVar29 = puVar46[1];
        uVar30 = puVar46[2];
        uVar31 = *(undefined8 *)((longlong)puVar46 + 0x1c);
        uStack_1e8 = (undefined4)((ulonglong)*(undefined8 *)((longlong)puVar46 + 0x14) >> 0x20);
        uStack_1ec = (undefined4)((ulonglong)uVar30 >> 0x20);
        uVar32 = *(undefined8 *)((longlong)puVar46 + 0x24);
        uVar33 = *(undefined8 *)((longlong)puVar46 + 0x2c);
        uVar34 = puVar46[7];
        uStack_1c8 = (undefined4)((ulonglong)puVar46[6] >> 0x20);
        uStack_1cc = (undefined4)((ulonglong)uVar33 >> 0x20);
        uVar21 = puVar46[8];
        uVar8 = *(undefined4 *)(puVar46 + 9);
        *puVar9 = uVar4;
        lVar50 = *(longlong *)(lVar51 + 0x30);
        uVar22 = *(undefined8 *)(lVar51 + 0x40);
        if (lVar50 == *(longlong *)(lVar51 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>();
        }
        lVar53 = *(longlong *)(lVar51 + 0x28);
        lVar41 = lVar50 * 0x98;
        *(uint *)(lVar53 + lVar41) = (uint)bVar54;
        *(undefined8 *)(lVar53 + 4 + lVar41) = uStack_148;
        *(uint *)(lVar53 + 0xc + lVar41) = uStack_140;
        *(undefined8 *)(lVar53 + 0x10 + lVar41) = uVar20;
        puVar46 = (undefined8 *)(lVar53 + 0x18 + lVar41);
        *puVar46 = uVar26;
        puVar46[1] = uVar27;
        puVar46 = (undefined8 *)(lVar53 + 0x24 + lVar41);
        *puVar46 = CONCAT44(uStack_108,uStack_10c);
        puVar46[1] = uVar28;
        *(undefined8 *)(lVar53 + 0x38 + lVar41) = uVar18;
        puVar46 = (undefined8 *)(lVar53 + 0x4c + lVar41);
        *puVar46 = CONCAT44(uStack_1e8,uStack_1ec);
        puVar46[1] = uVar31;
        puVar46 = (undefined8 *)(lVar53 + 0x40 + lVar41);
        *puVar46 = uVar29;
        puVar46[1] = uVar30;
        puVar46 = (undefined8 *)(lVar53 + 0x5c + lVar41);
        *puVar46 = uVar32;
        puVar46[1] = uVar33;
        puVar46 = (undefined8 *)(lVar53 + 0x68 + lVar41);
        *puVar46 = CONCAT44(uStack_1c8,uStack_1cc);
        puVar46[1] = uVar34;
        *(undefined4 *)(lVar53 + 0x80 + lVar41) = uVar8;
        *(undefined8 *)(lVar53 + 0x78 + lVar41) = uVar21;
        *(undefined4 *)(lVar53 + 0x84 + lVar41) = uVar5;
        *(undefined4 *)(lVar53 + 0x88 + lVar41) = uVar6;
        *(undefined1 *)(lVar53 + 0x8c + lVar41) = uVar3;
        *(undefined8 *)(lVar53 + 0x90 + lVar41) = uVar22;
        *(longlong *)(lVar51 + 0x30) = lVar50 + 1;
        *(longlong *)(lVar51 + 0x40) = *(longlong *)(lVar51 + 0x40) + 1;
        country_core::resources::walls::inner_walls::InnerWalls::get();
        if (extraout_RAX_04 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        if ((*pfVar14 != *(float *)(extraout_RAX_04 + 0x54)) ||
           (NAN(*pfVar14) || NAN(*(float *)(extraout_RAX_04 + 0x54)))) {
          **(undefined4 **)(in_stack_00000048 + 0x10) = *(undefined4 *)(in_stack_00000048 + 0x1c);
          country_core::resources::history::wall_history::impl$0::ev_height_changed();
        }
      }
      *puVar12 = uVar7;
      lVar51 = *(longlong *)(lVar45 + 0x30);
      uVar18 = *(undefined8 *)(lVar45 + 0x40);
      if (lVar51 == *(longlong *)(lVar45 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar45 + 0x28) + lVar51 * 8) = uVar18;
      *(longlong *)(lVar45 + 0x30) = lVar51 + 1;
      *(longlong *)(lVar45 + 0x40) = *(longlong *)(lVar45 + 0x40) + 1;
      uStack_18c = (undefined4)((ulonglong)lVar25 >> 0x20);
      *puVar24 = uStack_18c;
      lVar51 = *(longlong *)(lVar23 + 0x30);
      uVar18 = *(undefined8 *)(lVar23 + 0x40);
      if (lVar51 == *(longlong *)(lVar23 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar50 = *(longlong *)(lVar23 + 0x28);
      *(undefined8 *)(lVar50 + lVar51 * 0x10) = uVar18;
      *(undefined1 *)(lVar50 + 8 + lVar51 * 0x10) = 0;
      *(longlong *)(lVar23 + 0x30) = lVar51 + 1;
      *(longlong *)(lVar23 + 0x40) = *(longlong *)(lVar23 + 0x40) + 1;
    }
  }
joined_r0x000141a1a11f:
  if (extraout_AL == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_05 == 1) {
    local_a8 = extraout_RAX_05 + 1;
  }
  else if ((*extraout_RAX_05 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), local_a8 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_141a1a761:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*local_a8 != 0) {
LAB_141a1a871:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *local_a8 = -1;
  puffin::ThreadProfiler::end_scope();
LAB_141a1a642:
  *local_a8 = *local_a8 + 1;
  return;
}

