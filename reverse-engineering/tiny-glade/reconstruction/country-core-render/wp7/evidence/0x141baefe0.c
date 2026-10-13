
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_water::water_audio_grid::update_water_audio_grid(void)

{
  byte *pbVar1;
  longlong lVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  longlong lVar6;
  longlong lVar7;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar8;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *in_RCX;
  longlong *plVar9;
  ulonglong uVar10;
  ulonglong in_RDX;
  undefined *puVar11;
  ulonglong uVar12;
  longlong lVar13;
  float extraout_XMM0_Da;
  short sVar14;
  short sVar16;
  undefined1 auVar15 [16];
  float fVar17;
  float fVar18;
  longlong *in_stack_00000028;
  undefined8 local_140;
  float local_138;
  undefined8 local_120;
  float local_118;
  longlong *local_100;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') {
      return;
    }
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x37) {
          if ((char)(&DAT_142eed9b8)[in_RDX] < -0x40) {
LAB_141baf09e:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x37) goto LAB_141baf09e;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x37) {
          if ("stem_water::water_audio_grid::update_water_audio_grid::f"[in_RDX] < -0x40) {
LAB_141baf108:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x37) goto LAB_141baf108;
      }
    }
    puVar11 = &DAT_142eedaec;
    do {
      bVar3 = puVar11[-1];
      uVar8 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar11[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar11[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar8 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar8 = bVar5 & 0xf;
          }
          uVar8 = bVar4 & 0x3f | uVar8 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar8 = bVar4 & 0x1f;
        }
        uVar8 = (uint)(bVar3 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto LAB_141baf1c4;
LAB_141baf1d5:
        if (puVar11 == &UNK_142eedabf) break;
        if (puVar11 + -0x142eedabf < (undefined *)0x2c) {
          if (-0x41 < (char)puVar11[1]) break;
        }
        else if (puVar11 + -0x142eedabf == (undefined *)0x2c) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar11 = puVar11 + -1;
      if (uVar8 == 0x5c) goto LAB_141baf1d5;
LAB_141baf1c4:
      if (uVar8 == 0x2f) goto LAB_141baf1d5;
    } while (puVar11 != &DAT_142eedac0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar9 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_141baf608;
    if (*plVar9 != 0) goto LAB_141baf614;
    *plVar9 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar9 = *plVar9 + 1;
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\x01') {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_02 == 1) {
        local_100 = extraout_RAX_02 + 1;
      }
      else if ((*extraout_RAX_02 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), local_100 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
      goto LAB_141baf608;
      if (*local_100 != 0) goto LAB_141baf614;
      *local_100 = -1;
      puffin::ThreadProfiler::end_scope();
      goto LAB_141baf552;
    }
  }
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  lVar6 = *in_RCX;
  lVar2 = lVar6 + 1;
  *in_RCX = lVar2;
  country_core::systems::camera_rig::MainCamera::eye_position();
  uVar10 = lVar6 + lVar2 * 10 + 1;
  auVar15._8_8_ = 0;
  auVar15._0_8_ =
       (ulonglong)(uint)((int)uVar10 * 2) & 0xffffffff0000ffff | ((uVar10 & 0xffff) >> 2) << 0x10;
  sVar14 = SUB162(auVar15 & ___xmm_000000000000000000000000000e000e,0) + -7;
  sVar16 = SUB162(auVar15 & ___xmm_000000000000000000000000000e000e,2) + -7;
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar6 = *(longlong *)(*in_stack_00000028 + 8);
  lVar7 = *(longlong *)(*in_stack_00000028 + 0x10);
  uVar12 = 1L << ((byte)uVar10 & 0x3f);
  bVar3 = (byte)uVar10 & 0x3f;
  lVar2 = lVar6;
LAB_141baf3f0:
  lVar13 = lVar2;
  if (lVar13 != lVar6 + lVar7 * 0x18) {
    ::utils::swizzlers::impl$0::x0y();
    lVar2 = lVar13 + 0x18;
    fVar17 = (float)local_140 - (float)local_120;
    fVar18 = (float)((ulonglong)local_140 >> 0x20) - (float)((ulonglong)local_120 >> 0x20);
    if ((6400.0 <= (local_138 - local_118) * (local_138 - local_118) +
                   fVar18 * fVar18 + fVar17 * fVar17) ||
       (::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>(),
       extraout_XMM0_Da <= 0.5)) {
      uVar10 = *(ulonglong *)(lVar13 + 8);
    }
    else {
      country_core::systems::collision::world::Raycasting::intersects_any();
      uVar10 = *(ulonglong *)(lVar13 + 8);
      if (extraout_AL_00 == '\0') {
        *(ulonglong *)(lVar13 + 8) = uVar10 | uVar12;
        if ((uVar10 & uVar12) == 0) {
          *(uint *)(lVar13 + 0x10) =
               CONCAT22((short)((ulonglong)*(undefined8 *)(lVar13 + 0x10) >> 0x10) + sVar16,
                        (short)*(undefined8 *)(lVar13 + 0x10) + sVar14);
        }
        goto LAB_141baf3f0;
      }
    }
    *(ulonglong *)(lVar13 + 8) = uVar10 & (-2L << bVar3 | 0xfffffffffffffffeU >> 0x40 - bVar3);
    if ((uVar10 & uVar12) != 0) {
      *(uint *)(lVar13 + 0x10) =
           CONCAT22((short)((ulonglong)*(undefined8 *)(lVar13 + 0x10) >> 0x10) - sVar16,
                    (short)*(undefined8 *)(lVar13 + 0x10) - sVar14);
    }
    goto LAB_141baf3f0;
  }
  if (extraout_AL == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_03 == 1) {
    local_100 = extraout_RAX_03 + 1;
  }
  else if ((*extraout_RAX_03 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), local_100 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_141baf608:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*local_100 == 0) {
    *local_100 = -1;
    puffin::ThreadProfiler::end_scope();
LAB_141baf552:
    *local_100 = *local_100 + 1;
                    // WARNING: Read-only address (ram,0x000142eed8c0) is written
    return;
  }
LAB_141baf614:
                    // WARNING: Subroutine does not return
  core::cell::panic_already_borrowed();
}

