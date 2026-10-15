// name=place_roof_spires
// sig=void __cdecl place_roof_spires(void)
// size=1553

void __cdecl system_roof::visual::place_spires::place_roof_spires(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  longlong lVar5;
  bool bVar6;
  char extraout_AL;
  uint uVar7;
  int iVar8;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong *plVar9;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  ulonglong in_RDX;
  ulonglong uVar10;
  undefined *puVar11;
  longlong local_58;
  
  uVar10 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar6 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar10 != 0) {
        if (uVar10 < 0x34) {
          if ((char)(&DAT_142ff407b)[uVar10] < -0x40) {
LAB_142196eeb:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar10 != 0x34) goto LAB_142196eeb;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar10 != 0xfffffffffffffffe)) {
        if (uVar10 + 2 < 0x34) {
          if ("stem_roof::visual::place_spires::place_roof_spires::f"[uVar10] < -0x40) {
LAB_142196f40:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar10 + 2 != 0x34) goto LAB_142196f40;
      }
    }
    puVar11 = &DAT_142ff41ee;
    do {
      bVar2 = puVar11[-1];
      uVar7 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar7 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar7 = bVar4 & 0xf;
          }
          uVar7 = bVar3 & 0x3f | uVar7 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar7 = bVar3 & 0x1f;
        }
        uVar7 = (uint)(bVar2 & 0x3f) | uVar7 << 6;
        if (uVar7 != 0x5c) goto LAB_142197004;
LAB_142197016:
        if (puVar11 == &UNK_142ff41bf) break;
        if (puVar11 + -0x142ff41bf < (undefined *)0x2e) {
          if (-0x41 < (char)puVar11[1]) break;
        }
        else if (puVar11 + -0x142ff41bf == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar11 = puVar11 + -1;
      if (uVar7 == 0x5c) goto LAB_142197016;
LAB_142197004:
      if (uVar7 == 0x2f) goto LAB_142197016;
    } while (puVar11 != &DAT_142ff41c0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar9 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_1421973c4;
    if (*plVar9 != 0) goto LAB_1421973d0;
    *plVar9 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar9 = *plVar9 + 1;
    bVar6 = true;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
LAB_14219728e:
    if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
  }
  else {
    if ((extraout_RAX_02[3] != 0x36) ||
       (iVar8 = memcmp("country_core::resources::roof_tile_ssbo::RoofSpireSsbo",
                       (void *)extraout_RAX_02[2],0x36), iVar8 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    plVar9 = (longlong *)(**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (plVar9 == (longlong *)0x0) goto LAB_14219728e;
    lVar5 = *(longlong *)(in_RDX + 8);
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>_>
              ();
    country_core::render::impl$0::deref();
    if ((local_58 != 0) &&
       (((ulonglong)plVar9[4] < (ulonglong)(local_58 << 5) ||
        (rhapsody::raw::device::GraphicsDevice::update_buffer_bytes(), lVar5 != 0xb)))) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    *plVar9 = local_58;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (bVar6) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar9 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_1421973c4:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar9 != 0) {
LAB_1421973d0:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar9 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar9 = *plVar9 + 1;
  }
  return;
}

