// 0x1421a6d40
// void __cdecl generate_roof_stone_floor_and_roof_bottom(void)
// generate_roof_stone_floor_and_roof_bottom


void __cdecl
system_roof::visual::generate_roof_stone_floor_and_roof_bottom::
generate_roof_stone_floor_and_roof_bottom(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  ulonglong *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined8 *in_RCX;
  longlong lVar10;
  ulonglong uVar11;
  longlong *in_RDX;
  longlong lVar12;
  longlong *plVar13;
  longlong in_R8;
  longlong lVar14;
  ulonglong uVar15;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined7 uStack_a7;
  
  puVar3 = (ulonglong *)*in_RCX;
  lVar4 = in_RCX[1];
  uVar5 = *puVar3;
  uVar9 = 0;
  uVar11 = uVar5 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar11 = uVar9;
  }
  uVar15 = uVar5 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar15 = uVar9;
  }
  lVar10 = uVar11 * 0x20 + *(longlong *)(lVar4 + 8);
  uVar5 = *(ulonglong *)(lVar4 + 0x10) - uVar11;
  if (*(ulonglong *)(lVar4 + 0x10) < uVar11) {
    lVar10 = 8;
    uVar5 = uVar9;
  }
  lVar14 = uVar15 * 0x20 + *(longlong *)(lVar4 + 0x28);
  uVar11 = *(ulonglong *)(lVar4 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar4 + 0x30) < uVar15) {
    lVar14 = 8;
    uVar11 = uVar9;
  }
  *puVar3 = (*(longlong *)(lVar4 + 0x40) - uVar11) - uVar5;
  plVar13 = *(longlong **)(in_R8 + 8);
  if (plVar13 == (longlong *)0x0) {
    plVar13 = *(longlong **)(in_R8 + 0x10);
  }
  lVar4 = uVar5 * 0x20 + lVar10;
  lVar2 = uVar11 * 0x20 + lVar14;
  lVar6 = *in_RDX;
  lVar7 = in_RDX[1];
  lVar12 = lVar14;
  if (lVar10 == 0) goto LAB_1421a6e50;
LAB_1421a6e40:
  lVar12 = lVar14;
  if (lVar10 == lVar4) goto LAB_1421a6e50;
  lVar12 = lVar10;
  lVar10 = lVar10 + 0x20;
  do {
    *puVar3 = *puVar3 + 1;
    uVar5 = *(ulonglong *)(lVar12 + 8);
    uVar11 = uVar5 & 0xffffffff;
    if ((*(ulonglong *)(lVar7 + 0x10) <= uVar11) ||
       (*(int *)(*(longlong *)(lVar7 + 8) + uVar11 * 0x14) != (int)(uVar5 >> 0x20))) {
LAB_1421a704d:
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    lVar12 = *(longlong *)(lVar7 + 8) + uVar11 * 0x14;
    uVar11 = (ulonglong)*(uint *)(lVar12 + 4);
    if ((uVar11 == 0xffffffff) ||
       ((*(ulonglong *)(lVar6 + 0x38) <= uVar11 ||
        ((*(ulonglong *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar12 + 4) >> 6) * 8)
          >> (uVar11 & 0x3f) & 1) == 0)))) goto LAB_1421a704d;
    lVar8 = *(longlong *)
             (*(longlong *)
               (*(longlong *)(lVar7 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48) +
              0x10 + ~*(ulonglong *)
                       (*(longlong *)
                         (*(longlong *)(lVar7 + 0x1a8) + 0x38 +
                         (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48) +
                       *(longlong *)(lVar6 + 0x110) * 8) * 0x30);
    lVar12 = (ulonglong)*(uint *)(lVar12 + 0x10) * 0x58;
    if ((*(byte *)(lVar8 + 8 + lVar12) & 1) == 0) {
      log2f(*(float *)(lVar8 + lVar12 + 0x14) * 3.0);
    }
    country_core::resources::walls::decorator::DecoratorRank::new();
    country_core::components::drawable::impl$1::from();
    lVar12 = plVar13[2];
    if ((ulonglong)(*plVar13 - lVar12) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar8 = plVar13[1];
    *(code **)(lVar8 + lVar12) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniInstanceDesc>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar8 + 8 + lVar12);
    *puVar1 = local_b8;
    puVar1[1] = uStack_b0;
    *(ulonglong *)(lVar8 + 0x18 + lVar12) = CONCAT71(uStack_a7,9);
    *(undefined1 *)(lVar8 + 0x20 + lVar12) = 0;
    *(ulonglong *)(lVar8 + 0x28 + lVar12) = uVar5;
    *(code **)(lVar8 + 0x30 + lVar12) = bevy_ecs::error::handler::panic;
    plVar13[2] = lVar12 + 0x38;
    lVar12 = lVar14;
    if (lVar10 != 0) goto LAB_1421a6e40;
LAB_1421a6e50:
    if ((lVar12 == 0) || (lVar12 == lVar2)) {
      return;
    }
    lVar14 = lVar12 + 0x20;
    lVar10 = 0;
  } while( true );
}


