// addr=0x140929160 name=signfier_mark_for_delete
// signature=void __cdecl signfier_mark_for_delete(void)


// WARNING: Removing unreachable block (ram,0x0001409297bb)
// WARNING: Removing unreachable block (ram,0x0001409294e0)
// WARNING: Removing unreachable block (ram,0x0001409294f8)
// WARNING: Removing unreachable block (ram,0x0001409294fb)
// WARNING: Removing unreachable block (ram,0x0001409294fe)
// WARNING: Removing unreachable block (ram,0x00014092951a)
// WARNING: Removing unreachable block (ram,0x00014092952a)
// WARNING: Removing unreachable block (ram,0x000140929530)
// WARNING: Removing unreachable block (ram,0x000140929540)
// WARNING: Removing unreachable block (ram,0x000140929889)
// WARNING: Removing unreachable block (ram,0x00014092989e)

void __cdecl
country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete(void)

{
  byte *pbVar1;
  uint *puVar2;
  longlong lVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  char cVar7;
  longlong lVar8;
  undefined8 uVar9;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar10;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar11;
  undefined8 *puVar12;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  uint uVar13;
  longlong *in_RCX;
  longlong *plVar14;
  longlong *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  float *pfVar17;
  longlong in_R8;
  bool bVar18;
  longlong lVar19;
  uint *puVar20;
  ulonglong uVar21;
  longlong local_98;
  longlong local_90;
  longlong local_80;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar15 = 0x5a;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar15 != 0) {
        if (uVar15 < 0x5a) {
          if ((char)(&DAT_142aae608)[uVar15] < -0x40) {
LAB_1409291df:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 != 0x5a) goto LAB_1409291df;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar15 != 0xfffffffffffffffe)) {
        if (uVar15 + 2 < 0x5a) {
          if ((char)(&UNK_142aae60a)[uVar15] < -0x40) {
LAB_14092923d:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 + 2 != 0x5a) goto LAB_14092923d;
      }
    }
    puVar10 = &DAT_142aaf586;
    do {
      bVar4 = puVar10[-1];
      uVar13 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar10[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar10[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar13 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar13 = bVar6 & 0xf;
          }
          uVar13 = bVar5 & 0x3f | uVar13 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar13 = bVar5 & 0x1f;
        }
        uVar13 = (uint)(bVar4 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140929309;
LAB_14092931f:
        if (puVar10 == &UNK_142aaf53b) break;
        if (puVar10 + -0x142aaf53b < (undefined *)0x4a) {
          if (-0x41 < (char)puVar10[1]) break;
        }
        else if (puVar10 + -0x142aaf53b == (undefined *)0x4a) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar10 = puVar10 + -1;
      if (uVar13 == 0x5c) goto LAB_14092931f;
LAB_140929309:
      if (uVar13 == 0x2f) goto LAB_14092931f;
    } while (puVar10 != &DAT_142aaf53c);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar14 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar14 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_140929939;
    if (*plVar14 != 0) goto LAB_140929945;
    *plVar14 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar14 = *plVar14 + 1;
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  if (*(char *)(*in_RCX + 0x18) != '\0') {
    lVar8 = *(longlong *)(*in_RCX + 0x20);
    lVar19 = *(longlong *)(lVar8 + 0x18);
    lVar8 = lVar19 + *(longlong *)(lVar8 + 0x20) * 0x28;
    for (; lVar19 != lVar8; lVar19 = lVar19 + 0x28) {
      ::utils::
      calculate_hash<tuple_<country_core::systems::ui::resources::ui_state::WidgetId,enum2_<core::option::Option<ref_<str_>_>_>_>_>
                ();
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
    }
  }
  lVar8 = *in_RDX;
  puVar20 = *(uint **)(lVar8 + 0xf8);
  plVar14 = *(longlong **)(in_R8 + 8);
  if (plVar14 == (longlong *)0x0) {
    plVar14 = *(longlong **)(in_R8 + 0x10);
  }
  lVar19 = in_RDX[1];
  puVar2 = puVar20 + *(longlong *)(lVar8 + 0x100);
  cVar7 = *(char *)(lVar8 + 0x108);
  local_98 = 8;
  local_90 = 8;
  uVar15 = 0;
  uVar21 = 0;
  local_80 = 0;
  bVar18 = false;
  do {
    if (cVar7 == '\0') {
      if (uVar21 == uVar15) {
        do {
          if (puVar20 == puVar2) goto LAB_1409298b0;
          uVar13 = *puVar20;
          puVar20 = puVar20 + 1;
          uVar15 = *(ulonglong *)(*(longlong *)(lVar19 + 0x100) + 0x58 + (ulonglong)uVar13 * 0xa0);
        } while (uVar15 == 0);
        lVar11 = *(longlong *)(lVar19 + 0x100) + (ulonglong)uVar13 * 0xa0;
        lVar3 = *(longlong *)(lVar19 + 0x1a8) + (ulonglong)*(uint *)(lVar11 + 0x94) * 0x48;
        uVar21 = *(ulonglong *)
                  (*(longlong *)(lVar19 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar11 + 0x94) * 0x48
                  );
        if ((*(ulonglong *)(lVar8 + 0x110) < uVar21) &&
           (*(longlong *)(*(longlong *)(lVar3 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
          uVar16 = *(ulonglong *)(lVar8 + 0x118);
          if (*(ulonglong *)(lVar11 + 0x88) <= uVar16) goto LAB_14092975d;
LAB_1409296fd:
          if (*(longlong *)(*(longlong *)(lVar11 + 0x80) + uVar16 * 8) == 0) goto LAB_14092975d;
          bVar18 = true;
          if ((uVar16 < uVar21) &&
             (uVar21 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + uVar16 * 8), uVar21 != 0)) {
            local_80 = *(longlong *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar21 * 0x30);
          }
          else {
            local_80 = 0;
          }
        }
        else {
          uVar16 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar16 < *(ulonglong *)(lVar11 + 0x88)) goto LAB_1409296fd;
LAB_14092975d:
          bVar18 = false;
        }
        local_90 = *(longlong *)(lVar11 + 0x50);
        uVar21 = 0;
      }
      puVar12 = (undefined8 *)(local_90 + uVar21 * 0x10);
      uVar16 = (ulonglong)*(uint *)(local_90 + 8 + uVar21 * 0x10);
    }
    else {
      if (uVar21 == uVar15) {
        do {
          if (puVar20 == puVar2) goto LAB_1409298b0;
          uVar13 = *puVar20;
          puVar20 = puVar20 + 1;
          uVar15 = *(ulonglong *)(*(longlong *)(lVar19 + 0x1a8) + 0x10 + (ulonglong)uVar13 * 0x48);
        } while (uVar15 == 0);
        lVar3 = *(longlong *)(lVar19 + 0x1a8) + (ulonglong)uVar13 * 0x48;
        uVar21 = *(ulonglong *)(lVar3 + 0x40);
        if ((*(ulonglong *)(lVar8 + 0x110) < uVar21) &&
           (*(longlong *)(*(longlong *)(lVar3 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
          uVar16 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar21 <= uVar16) goto LAB_14092973e;
LAB_1409296b9:
          uVar21 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + uVar16 * 8);
          if (uVar21 == 0) goto LAB_14092973e;
          local_80 = *(longlong *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar21 * 0x30);
          bVar18 = true;
        }
        else {
          uVar16 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar16 < uVar21) goto LAB_1409296b9;
LAB_14092973e:
          bVar18 = false;
        }
        local_98 = *(longlong *)(lVar3 + 8);
        uVar21 = 0;
      }
      puVar12 = (undefined8 *)(local_98 + uVar21 * 8);
      uVar16 = uVar21 & 0xffffffff;
    }
    pfVar17 = (float *)(local_80 + uVar16 * 8);
    if (!bVar18) {
      pfVar17 = (float *)0x0;
    }
    uVar9 = *puVar12;
    uVar21 = uVar21 + 1;
    if ((pfVar17 == (float *)0x0) || (pfVar17[1] < *pfVar17 || pfVar17[1] == *pfVar17)) {
      lVar3 = plVar14[2];
      if ((ulonglong)(*plVar14 - lVar3) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar11 = plVar14[1];
      *(code **)(lVar11 + lVar3) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar11 + 8 + lVar3) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar11 + 0x10 + lVar3) = uVar9;
      *(undefined1 *)(lVar11 + 0x18 + lVar3) = 0;
      plVar14[2] = lVar3 + 0x20;
    }
  } while( true );
LAB_1409298b0:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar14 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar14 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_140929939:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar14 != 0) {
LAB_140929945:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar14 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar14 = *plVar14 + 1;
  }
  return;
}


