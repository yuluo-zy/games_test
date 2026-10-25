// addr=0x1408383a0 name=hover_intersection
// signature=void __cdecl hover_intersection(void)


void __cdecl country_core::systems::ui::hover_intersection::hover_intersection(void)

{
  byte *pbVar1;
  undefined1 auVar2 [16];
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined8 uVar6;
  undefined1 *puVar7;
  ulonglong uVar8;
  ushort uVar9;
  int iVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined1 **ppuVar15;
  float *pfVar16;
  char extraout_AL;
  uint uVar17;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar18;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar19;
  longlong *extraout_RAX_04;
  longlong lVar20;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  uint uVar21;
  undefined8 *in_RCX;
  undefined8 *puVar22;
  longlong *plVar23;
  longlong *in_RDX;
  ulonglong uVar24;
  undefined1 (*pauVar25) [16];
  longlong lVar26;
  ulonglong *puVar27;
  longlong in_R8;
  float *pfVar28;
  longlong lVar29;
  undefined8 *puVar30;
  longlong *in_R9;
  longlong lVar31;
  longlong *plVar32;
  undefined1 (*pauVar33) [16];
  longlong *plVar34;
  bool bVar35;
  longlong *in_stack_00000038;
  undefined1 *local_158;
  undefined1 *local_150;
  undefined1 local_148 [12];
  undefined1 local_13c [12];
  longlong local_130;
  longlong *local_128;
  longlong local_120;
  undefined1 **local_118;
  longlong local_110 [3];
  longlong *local_f8;
  longlong local_f0;
  longlong *local_e8;
  longlong local_e0;
  longlong local_d0;
  longlong local_c8;
  longlong *local_c0;
  undefined1 *local_b8;
  undefined1 **local_b0;
  longlong *local_a8;
  longlong local_a0;
  void *local_98;
  longlong *local_90;
  longlong *local_88;
  longlong *local_80;
  undefined1 ***local_78;
  longlong *local_70;
  ulonglong *local_68;
  longlong local_60;
  longlong *local_58;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_70 = in_R9;
  local_60 = in_R8;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_e0 = 0;
  }
  else {
    uVar24 = 0x41;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar24 != 0) {
        if (uVar24 < 0x41) {
          if ((char)(&DAT_142a924b0)[uVar24] < -0x40) {
LAB_140838422:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar24 != 0x41) goto LAB_140838422;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar24 != 0xfffffffffffffffe)) {
        if (uVar24 + 2 < 0x41) {
          if ((char)(&UNK_142a924b2)[uVar24] < -0x40) {
LAB_140838480:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar24 + 2 != 0x41) goto LAB_140838480;
      }
    }
    puVar18 = &anon_03891170661cff6f447a90196a76d89b_80_llvm_10311377136338234151;
    do {
      bVar3 = puVar18[-1];
      uVar21 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar18[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar18[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar18 + -4;
            puVar18 = puVar18 + -4;
            uVar21 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar18 = puVar18 + -3;
            uVar21 = bVar5 & 0xf;
          }
          uVar21 = bVar4 & 0x3f | uVar21 << 6;
        }
        else {
          puVar18 = puVar18 + -2;
          uVar21 = bVar4 & 0x1f;
        }
        uVar21 = (uint)(bVar3 & 0x3f) | uVar21 << 6;
        if (uVar21 != 0x5c) goto LAB_140838549;
LAB_140838561:
        if (puVar18 == &UNK_142a92be1) break;
        if (puVar18 + -0x142a92be1 < (undefined *)0x38) {
          if (-0x41 < (char)puVar18[1]) break;
        }
        else if (puVar18 + -0x142a92be1 == (undefined *)0x38) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar18 = puVar18 + -1;
      if (uVar21 == 0x5c) goto LAB_140838561;
LAB_140838549:
      if (uVar21 == 0x2f) goto LAB_140838561;
    } while (puVar18 != &DAT_142a92be2);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_140838f7a;
    if (*plVar23 != 0) goto LAB_140838f86;
    *plVar23 = -1;
    local_58 = plVar23;
    puffin::ThreadProfiler::begin_scope();
    *local_58 = *local_58 + 1;
    local_e0 = 1;
  }
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar23 = (longlong *)*in_RCX;
  local_f8 = in_RDX;
  ::utils::geometry::ray::Ray::from_mouse_ss();
  local_150 = local_13c;
  in_stack_00000038[2] = 0;
  puVar27 = (ulonglong *)(plVar23 + 0xc);
  uVar24 = plVar23[0xc];
  local_158 = local_148;
  if (uVar24 < 0xfffffffffffffff0 && (char)(((uint)uVar24 & 8) >> 3) == '\0') {
    LOCK();
    uVar8 = *puVar27;
    if (uVar24 == uVar8) {
      *puVar27 = uVar24 + 0x10;
    }
    UNLOCK();
    if (uVar24 != uVar8) goto LAB_1408386b4;
  }
  else {
LAB_1408386b4:
    parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_98 = (void *)((ulonglong)puVar27 | 1);
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar26 = plVar23[0x10];
  local_58 = plVar23;
  if (lVar26 != 0) {
    pauVar25 = (undefined1 (*) [16])plVar23[0xd];
    auVar2 = *pauVar25;
    pauVar33 = pauVar25 + 1;
    uVar21 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
    local_68 = puVar27;
    do {
      if ((short)uVar21 == 0) {
        do {
          auVar2 = *pauVar33;
          uVar9 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
          pauVar25 = pauVar25 + -0x10;
          pauVar33 = pauVar33 + 1;
        } while (uVar9 == 0xffff);
        uVar17 = ~(uint)uVar9;
        uVar21 = uVar17 & -(uint)uVar9 - 2;
      }
      else {
        uVar17 = uVar21;
        uVar21 = uVar21 - 1 & uVar21;
      }
      lVar26 = lVar26 + -1;
      iVar10 = 0;
      for (; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
        iVar10 = iVar10 + 1;
      }
      uVar6 = *(undefined8 *)((longlong)pauVar25 + (-0x10 - (ulonglong)(uint)(iVar10 << 4)));
      lVar20 = *(longlong *)((longlong)pauVar25 + (-8 - (ulonglong)(uint)(iVar10 << 4)));
      lVar31 = in_stack_00000038[2];
      if (lVar31 == *in_stack_00000038) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar19 = in_stack_00000038[1];
      *(undefined8 *)(lVar19 + lVar31 * 0x10) = uVar6;
      *(longlong *)(lVar19 + 8 + lVar31 * 0x10) = lVar20 + 0x10;
      in_stack_00000038[2] = lVar31 + 1;
      puVar27 = local_68;
    } while (lVar26 != 0);
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  plVar23 = local_58;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar24 = *puVar27;
  *puVar27 = *puVar27 - 0x10;
  UNLOCK();
  if ((uVar24 & 0xfffffffffffffff2) == 0x12) {
    parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  }
  bevy_tasks::usages::ComputeTaskPool::get();
  uVar24 = *(ulonglong *)(extraout_RAX_02 + 0x10);
  if (uVar24 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_div_by_zero();
  }
  plVar32 = (longlong *)in_stack_00000038[1];
  puVar7 = (undefined1 *)in_stack_00000038[2];
  if (((ulonglong)puVar7 | uVar24) >> 0x20 == 0) {
    uVar24 = ((ulonglong)puVar7 & 0xffffffff) / (uVar24 & 0xffffffff);
  }
  else {
    uVar24 = (ulonglong)puVar7 / uVar24;
  }
  if (uVar24 < (puVar7 == (undefined1 *)0xffffffffffffffff)) {
    uVar24 = (ulonglong)(puVar7 == (undefined1 *)0xffffffffffffffff);
  }
  local_110[0] = uVar24 + (uVar24 == 0);
  local_b0 = &local_158;
  local_a8 = local_70;
  local_a0 = local_60;
  local_118 = &local_b8;
  local_b8 = local_148;
  core::ops::function::FnOnce::
  call_once<bevy_tasks::task_pool::impl_2::THREAD_EXECUTOR::constant_0::closure_env_0,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<alloc::sync::Arc<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_03 == 0) {
LAB_140838f92:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_80 = local_110;
  local_90 = plVar32;
  local_88 = (longlong *)puVar7;
  local_78 = &local_118;
  bevy_tasks::task_pool::TaskPool::
  scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env_0<ref_<slice2_<tuple_<country_core::systems::ui::resources::ui_state::WidgetId,ref_<country_core::systems::ui::resources::widget_params::WidgetParams>_>_>_>,tuple_<country_core::systems::ui::resources::ui_state::WidgetId,ref_<country_core::systems::ui::resources::widget_params::WidgetParams>_>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env_2,tuple_<alloc::vec::Vec<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global>,alloc::vec::Vec<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::alloc::Global>_>_>,tuple_<alloc::vec::Vec<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global>,alloc::vec::Vec<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::alloc::Global>_>_>
            ();
  if (SBORROW8(0,local_130)) goto LAB_140838f92;
  in_stack_00000038[2] = 0;
  plVar23[2] = 0;
  local_b8 = (undefined1 *)0x0;
  local_b0 = (undefined1 **)0x8;
  local_a8 = (longlong *)0x0;
  local_c0 = local_128 + local_120 * 6;
  local_90 = local_128;
  local_88 = local_128;
  local_f0 = local_130;
  local_80 = (longlong *)local_130;
  local_e8 = local_128;
  local_78 = (undefined1 ***)local_c0;
  if (local_120 == 0) {
    plVar34 = (longlong *)0x0;
    plVar32 = local_128;
  }
  else {
    local_d0 = 8;
    plVar34 = (longlong *)0x0;
    do {
      plVar23 = local_58;
      plVar32 = local_128 + 6;
      lVar26 = *local_128;
      if (SBORROW8(0,lVar26)) break;
      local_98 = (void *)local_128[1];
      uVar24 = local_128[2];
      local_60 = local_128[3];
      local_68 = (ulonglong *)local_128[4];
      uVar8 = local_128[5];
      lVar20 = local_58[2];
      local_c8 = lVar26;
      local_70 = plVar32;
      if ((ulonglong)(*local_58 - lVar20) < uVar24) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar20 = plVar23[2];
      }
      plVar23 = local_58;
      memcpy((void *)(lVar20 * 0x38 + local_58[1]),local_98,uVar24 * 0x38);
      plVar23[2] = lVar20 + uVar24;
      if (local_c8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((ulonglong)((longlong)local_b8 - (longlong)plVar34) < uVar8) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        local_d0 = (longlong)local_b0;
        plVar34 = local_a8;
      }
      memcpy((void *)((longlong)plVar34 * 0x40 + local_d0),local_68,uVar8 << 6);
      plVar34 = (longlong *)((longlong)plVar34 + uVar8);
      local_a8 = plVar34;
      if (local_60 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      bVar35 = plVar32 != local_c0;
      plVar23 = local_58;
      plVar32 = local_c0;
      local_128 = local_70;
    } while (bVar35);
  }
  local_88 = plVar32;
  if (local_c0 != plVar32) {
    uVar24 = (ulonglong)((longlong)local_c0 - (longlong)plVar32) / 0x30;
    plVar32 = plVar32 + 4;
    do {
      if (plVar32[-4] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (plVar32[-1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar32 = plVar32 + 6;
      uVar24 = uVar24 - 1;
    } while (uVar24 != 0);
  }
  if (local_f0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_90 = (longlong *)local_148;
  local_88 = (longlong *)local_13c;
  local_80 = local_f8;
  alloc::vec::Vec::
  retain<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global,country_core::systems::ui::hover_intersection::hover_intersection::closure_env_3>
            ();
  ppuVar15 = local_b0;
  local_90 = &local_130;
  if (1 < plVar34) {
    if (plVar34 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl_0::sort_by::closure_env_0<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env_4>_>
                ();
    }
    else {
      core::slice::sort::stable::
      driftsort_main<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl_0::sort_by::closure_env_0<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env_4>,alloc::vec::Vec<tuple_<ref_<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::alloc::Global>_>
                ();
    }
  }
  lVar26 = plVar23[2];
  if (lVar26 != 0) {
    lVar20 = plVar23[1];
    if ((0.0 <= *(float *)(lVar20 + 0x20)) || (lVar31 = 1, (*(byte *)(lVar20 + 0x31) & 4) != 0)) {
      lVar31 = 1;
      lVar19 = 0;
      pfVar16 = (float *)(lVar20 + 0x58);
      do {
        pfVar28 = pfVar16;
        if (lVar26 == lVar31) goto LAB_140838d37;
        lVar31 = lVar31 + 1;
        pfVar16 = pfVar28 + 0xe;
      } while ((0.0 <= *pfVar28) || ((*(byte *)((longlong)pfVar28 + 0x11) & 4) != 0));
    }
    lVar19 = 1;
    lVar29 = lVar26 - lVar31;
    if (lVar29 != 0) {
      puVar22 = (undefined8 *)(lVar20 + lVar31 * 0x38);
      lVar19 = 1;
      do {
        while ((*(float *)(puVar22 + 4) < 0.0 && ((*(byte *)((longlong)puVar22 + 0x31) & 4) == 0)))
        {
          lVar19 = lVar19 + 1;
          puVar22 = puVar22 + 7;
          lVar29 = lVar29 + -1;
          if (lVar29 == 0) goto LAB_140838d37;
        }
        puVar22[lVar19 * -7 + 6] = puVar22[6];
        uVar6 = *puVar22;
        uVar11 = puVar22[1];
        uVar12 = puVar22[2];
        uVar13 = puVar22[3];
        uVar14 = puVar22[5];
        puVar22[lVar19 * -7 + 4] = puVar22[4];
        (puVar22 + lVar19 * -7 + 4)[1] = uVar14;
        puVar22[lVar19 * -7 + 2] = uVar12;
        (puVar22 + lVar19 * -7 + 2)[1] = uVar13;
        puVar22[lVar19 * -7] = uVar6;
        (puVar22 + lVar19 * -7)[1] = uVar11;
        puVar22 = puVar22 + 7;
        lVar29 = lVar29 + -1;
      } while (lVar29 != 0);
    }
LAB_140838d37:
    uVar24 = lVar26 - lVar19;
    plVar23[2] = uVar24;
    local_90 = &local_130;
    if (1 < uVar24) {
      if (uVar24 < 0x15) {
        core::slice::sort::shared::smallsort::
        insertion_sort_shift_left<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl_0::sort_by::closure_env_0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env_6>_>
                  ();
      }
      else {
        core::slice::sort::stable::
        driftsort_main<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl_0::sort_by::closure_env_0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env_6>,alloc::vec::Vec<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global>_>
                  ();
      }
    }
  }
  lVar26 = plVar23[2];
  if ((ulonglong)(*plVar23 - lVar26) < plVar34) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar26 = plVar23[2];
    lVar20 = plVar23[1];
    if (plVar34 == (longlong *)0x1) goto LAB_140838d79;
LAB_140838e2c:
    puVar30 = (undefined8 *)(lVar26 * 0x38 + lVar20);
    puVar22 = (undefined8 *)((longlong)ppuVar15 + 0x48);
    lVar31 = 0;
    do {
      puVar30[6] = puVar22[-2];
      uVar6 = puVar22[-8];
      uVar11 = puVar22[-7];
      uVar12 = puVar22[-6];
      uVar13 = puVar22[-5];
      uVar14 = puVar22[-3];
      puVar30[4] = puVar22[-4];
      puVar30[5] = uVar14;
      puVar30[2] = uVar12;
      puVar30[3] = uVar13;
      *puVar30 = uVar6;
      puVar30[1] = uVar11;
      uVar6 = puVar22[1];
      uVar11 = puVar22[2];
      uVar12 = puVar22[3];
      uVar13 = puVar22[4];
      uVar14 = puVar22[5];
      puVar30[7] = *puVar22;
      puVar30[8] = uVar6;
      puVar30[9] = uVar11;
      puVar30[10] = uVar12;
      puVar30[0xb] = uVar13;
      puVar30[0xc] = uVar14;
      puVar30[0xd] = puVar22[6];
      puVar30 = puVar30 + 0xe;
      lVar31 = lVar31 + -2;
      puVar22 = puVar22 + 0x10;
    } while (-lVar31 != ((ulonglong)plVar34 & 0xfffffffffffffffe));
    lVar26 = lVar26 - lVar31;
    lVar31 = -lVar31;
    if (((ulonglong)plVar34 & 1) == 0) goto LAB_140838eec;
LAB_140838eb7:
    lVar31 = lVar31 * 0x40;
    lVar19 = lVar26 * 0x38;
    *(undefined8 *)(lVar20 + 0x30 + lVar19) = *(undefined8 *)((longlong)ppuVar15 + 0x38 + lVar31);
    puVar22 = (undefined8 *)((longlong)ppuVar15 + 8 + lVar31);
    uVar6 = *puVar22;
    uVar11 = puVar22[1];
    puVar22 = (undefined8 *)((longlong)ppuVar15 + 0x18 + lVar31);
    uVar12 = *puVar22;
    uVar13 = puVar22[1];
    puVar22 = (undefined8 *)((longlong)ppuVar15 + 0x28 + lVar31);
    uVar14 = puVar22[1];
    puVar30 = (undefined8 *)(lVar20 + 0x20 + lVar19);
    *puVar30 = *puVar22;
    puVar30[1] = uVar14;
    puVar22 = (undefined8 *)(lVar20 + 0x10 + lVar19);
    *puVar22 = uVar12;
    puVar22[1] = uVar13;
    *(undefined8 *)(lVar20 + lVar19) = uVar6;
    ((undefined8 *)(lVar20 + lVar19))[1] = uVar11;
    lVar26 = lVar26 + 1;
  }
  else if (plVar34 != (longlong *)0x0) {
    lVar20 = plVar23[1];
    if (plVar34 != (longlong *)0x1) goto LAB_140838e2c;
LAB_140838d79:
    lVar31 = 0;
    if (((ulonglong)plVar34 & 1) != 0) goto LAB_140838eb7;
  }
LAB_140838eec:
  plVar23[2] = lVar26;
  if (local_b8 != (undefined1 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_e0 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      plVar23 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_140838f7a:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar23 != 0) {
LAB_140838f86:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    local_58 = plVar23;
    puffin::ThreadProfiler::end_scope();
    *local_58 = *local_58 + 1;
  }
  return;
}


