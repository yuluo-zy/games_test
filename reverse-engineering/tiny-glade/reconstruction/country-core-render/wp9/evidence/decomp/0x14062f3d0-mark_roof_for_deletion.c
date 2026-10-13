// signature: void __cdecl mark_roof_for_deletion(void)

// WARNING: Removing unreachable block (ram,0x00014062f844)

void __cdecl system_roof::delete_roof::mark_roof_for_deletion(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined4 uVar4;
  uint uVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined4 *puVar8;
  longlong lVar9;
  undefined4 uVar10;
  longlong lVar11;
  longlong lVar12;
  ulonglong *puVar13;
  undefined8 *in_RCX;
  longlong lVar14;
  ulonglong uVar15;
  longlong *in_RDX;
  uint *puVar16;
  ulonglong uVar17;
  longlong in_R8;
  longlong *in_R9;
  ulonglong uVar18;
  ulonglong uVar19;
  uint *puVar20;
  undefined8 local_df;
  undefined3 uStack_d7;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 local_c4;
  undefined4 local_bc;
  longlong local_a0;
  longlong local_98;
  longlong local_68;
  longlong *local_60;
  longlong local_50;
  
  local_68 = 0;
  puVar13 = (ulonglong *)*in_RCX;
  lVar6 = in_RCX[1];
  uVar17 = *puVar13;
  uVar15 = 0;
  uVar19 = uVar17 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar19 = uVar15;
  }
  uVar18 = uVar17 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar18 = uVar15;
  }
  lVar11 = uVar19 * 0x18 + *(longlong *)(lVar6 + 8);
  uVar17 = *(ulonglong *)(lVar6 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar19) {
    lVar11 = 8;
    uVar17 = uVar15;
  }
  lVar14 = uVar18 * 0x18 + *(longlong *)(lVar6 + 0x28);
  uVar19 = *(ulonglong *)(lVar6 + 0x30) - uVar18;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar18) {
    lVar14 = 8;
    uVar19 = uVar15;
  }
  *puVar13 = (*(longlong *)(lVar6 + 0x40) - uVar19) - uVar17;
  lVar6 = lVar11 + uVar17 * 0x18;
  lVar12 = lVar14 + uVar19 * 0x18;
  do {
    if ((lVar11 == 0) || (lVar11 == lVar6)) {
      if ((lVar14 == 0) || (lVar14 == lVar12)) break;
      *puVar13 = *puVar13 + 1;
      uVar7 = *(undefined8 *)(lVar14 + 8);
      lVar14 = lVar14 + 0x18;
      lVar11 = 0;
    }
    else {
      *puVar13 = *puVar13 + 1;
      uVar7 = *(undefined8 *)(lVar11 + 8);
      lVar11 = lVar11 + 0x18;
    }
    if (local_68 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(local_68 * 8 + 8) = uVar7;
    local_68 = local_68 + 1;
  } while( true );
  lVar6 = *in_RDX;
  puVar20 = *(uint **)(lVar6 + 0xf8);
  local_60 = *(longlong **)(in_R8 + 8);
  if (local_60 == (longlong *)0x0) {
    local_60 = *(longlong **)(in_R8 + 0x10);
  }
  lVar11 = in_RDX[1];
  puVar16 = puVar20 + *(longlong *)(lVar6 + 0x100);
  cVar3 = *(char *)(lVar6 + 0x108);
  uVar4 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar14 = *in_R9;
  puVar8 = (undefined4 *)in_R9[2];
  local_a0 = 8;
  local_50 = 0;
  local_98 = 8;
  uVar17 = 0;
  uVar19 = 0;
LAB_14062f5d6:
  do {
    if (cVar3 == '\0') {
      if (uVar19 == uVar17) {
        do {
          if (puVar20 == puVar16) {
            return;
          }
          uVar5 = *puVar20;
          puVar20 = puVar20 + 1;
          uVar17 = *(ulonglong *)(*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)uVar5 * 0xa0);
        } while (uVar17 == 0);
        lVar12 = *(longlong *)(lVar11 + 0x100) + (ulonglong)uVar5 * 0xa0;
        uVar19 = (ulonglong)*(uint *)(lVar12 + 0x94);
        if (*(ulonglong *)(lVar6 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar11 + 0x1a8) + 0x40 + uVar19 * 0x48)) {
          lVar9 = *(longlong *)(lVar11 + 0x1a8) + uVar19 * 0x48;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8);
          if (uVar19 == 0) goto LAB_14062f70b;
          local_50 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar19 * 0x30);
        }
        else {
LAB_14062f70b:
          local_50 = 0;
        }
        local_98 = *(longlong *)(lVar12 + 0x50);
        uVar19 = 0;
      }
      puVar13 = (ulonglong *)(local_98 + uVar19 * 0x10);
      uVar15 = (ulonglong)*(uint *)(local_98 + 8 + uVar19 * 0x10);
    }
    else {
      if (uVar19 == uVar17) {
        do {
          if (puVar20 == puVar16) {
            return;
          }
          uVar5 = *puVar20;
          puVar20 = puVar20 + 1;
          uVar17 = *(ulonglong *)(*(longlong *)(lVar11 + 0x1a8) + 0x10 + (ulonglong)uVar5 * 0x48);
        } while (uVar17 == 0);
        lVar12 = *(longlong *)(lVar11 + 0x1a8) + (ulonglong)uVar5 * 0x48;
        if ((*(ulonglong *)(lVar6 + 0x110) < *(ulonglong *)(lVar12 + 0x40)) &&
           (uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar19 != 0)) {
          local_50 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar19 * 0x30);
        }
        else {
          local_50 = 0;
        }
        local_a0 = *(longlong *)(lVar12 + 8);
        uVar19 = 0;
      }
      puVar13 = (ulonglong *)(local_a0 + uVar19 * 8);
      uVar15 = uVar19 & 0xffffffff;
    }
    uVar18 = *puVar13;
    if (uVar18 >> 0x20 == 0) {
      return;
    }
    uVar19 = uVar19 + 1;
    lVar12 = 0;
    do {
      if (local_68 << 3 == lVar12) goto LAB_14062f5d6;
      plVar1 = (longlong *)(lVar12 + 8);
      lVar12 = lVar12 + 8;
    } while (*(longlong *)(uVar15 * 0x58 + local_50) != *plVar1);
    lVar12 = local_60[2];
    if ((ulonglong)(*local_60 - lVar12) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar9 = local_60[1];
    *(code **)(lVar9 + lVar12) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar9 + 8 + lVar12) = bevy_ecs::error::handler::panic;
    *(ulonglong *)(lVar9 + 0x10 + lVar12) = uVar18;
    *(undefined1 *)(lVar9 + 0x18 + lVar12) = 0;
    local_60[2] = lVar12 + 0x20;
    country_core::resources::roofs::roof_shape::Roof::pivot_3d();
    uStack_d0 = (undefined4)local_c4;
    uVar10 = uStack_d0;
    uStack_cc = (undefined4)((ulonglong)local_c4 >> 0x20);
    *puVar8 = uVar4;
    lVar12 = *(longlong *)(lVar14 + 0x30);
    uVar7 = *(undefined8 *)(lVar14 + 0x40);
    if (lVar12 == *(longlong *)(lVar14 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar9 = *(longlong *)(lVar14 + 0x28);
    *(undefined8 *)(lVar9 + lVar12 * 0x28) = uVar7;
    *(undefined1 *)(lVar9 + 8 + lVar12 * 0x28) = 1;
    uStack_d0._0_1_ = (undefined1)local_c4;
    puVar2 = (undefined8 *)(lVar9 + 0x14 + lVar12 * 0x28);
    *puVar2 = CONCAT44(uVar10,uStack_d4);
    puVar2[1] = CONCAT44(local_bc,uStack_cc);
    puVar2 = (undefined8 *)(lVar9 + 9 + lVar12 * 0x28);
    *puVar2 = local_df;
    puVar2[1] = CONCAT17((undefined1)uStack_d0,CONCAT43(uStack_d4,uStack_d7));
    *(longlong *)(lVar14 + 0x30) = lVar12 + 1;
    *(longlong *)(lVar14 + 0x40) = *(longlong *)(lVar14 + 0x40) + 1;
  } while( true );
}

