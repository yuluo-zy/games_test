
void __cdecl
core::iter::adapters::map::impl$2::
fold<utils::geometry::intersect::Triangle,core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<utils::geometry::intersect::Triangle,alloc::vec::impl_20::extend_trusted::closure_env_0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env_0>_>_>_>
          (void)

{
  undefined8 *puVar1;
  ulonglong *puVar2;
  ulonglong uVar3;
  longlong *plVar4;
  longlong lVar5;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  undefined8 local_cc;
  undefined8 uStack_c4;
  undefined8 local_bc;
  undefined8 uStack_b4;
  undefined4 local_ac;
  
  puVar2 = (ulonglong *)*in_RCX;
  uVar8 = in_RCX[1];
  uVar3 = in_RCX[2];
  plVar4 = (longlong *)*in_RDX;
  lVar6 = in_RDX[1];
  lVar5 = in_RDX[2];
  while( true ) {
    if (uVar8 == 0) {
      *plVar4 = lVar6;
      return;
    }
    uVar7 = uVar8;
    if (uVar3 < uVar8) {
      uVar7 = uVar3;
    }
    if (uVar3 == 0) {
                    // WARNING: Subroutine does not return
      panicking::panic_bounds_check();
    }
    if (0x17 < *puVar2) {
                    // WARNING: Subroutine does not return
      panicking::panic_bounds_check();
    }
    if (uVar7 == 1) break;
    if (0x17 < puVar2[1]) {
                    // WARNING: Subroutine does not return
      panicking::panic_bounds_check();
    }
    if (uVar7 < 3) {
                    // WARNING: Subroutine does not return
      panicking::panic_bounds_check();
    }
    if (0x17 < puVar2[2]) {
                    // WARNING: Subroutine does not return
      panicking::panic_bounds_check();
    }
    ::utils::geometry::intersect::Triangle::new();
    uVar8 = uVar8 - uVar7;
    puVar2 = puVar2 + uVar7;
    *(undefined4 *)(lVar5 + 0x20 + lVar6 * 0x24) = local_ac;
    puVar1 = (undefined8 *)(lVar5 + 0x10 + lVar6 * 0x24);
    *puVar1 = local_bc;
    puVar1[1] = uStack_b4;
    puVar1 = (undefined8 *)(lVar5 + lVar6 * 0x24);
    *puVar1 = local_cc;
    puVar1[1] = uStack_c4;
    lVar6 = lVar6 + 1;
  }
                    // WARNING: Subroutine does not return
  panicking::panic_bounds_check();
}

