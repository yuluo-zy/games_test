##### 0x14233c8b0 ui_locked_animation

// WARNING: Removing unreachable block (ram,0x00014233cdcc)

void __cdecl toolbar::ui_interaction::ui_locked_animation(void)

{
  char cVar1;
  uint uVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined8 *in_RCX;
  longlong *plVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong *in_RDX;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  ulonglong uVar16;
  longlong *in_R8;
  longlong in_R9;
  uint *puVar17;
  uint *puVar18;
  longlong lVar19;
  float fVar20;
  float fVar21;
  float _X;
  longlong *in_stack_00000028;
  longlong local_f0;
  longlong local_e8;
  longlong local_c8;
  longlong local_c0;
  
  puVar4 = (ulonglong *)*in_RCX;
  lVar5 = in_RCX[1];
  uVar9 = *puVar4;
  uVar16 = 0;
  uVar7 = uVar9 - *(ulonglong *)(lVar5 + 0x18);
  if (uVar9 < *(ulonglong *)(lVar5 + 0x18)) {
    uVar7 = uVar16;
  }
  uVar14 = uVar9 - *(ulonglong *)(lVar5 + 0x38);
  if (uVar9 < *(ulonglong *)(lVar5 + 0x38)) {
    uVar14 = uVar16;
  }
  lVar11 = uVar7 * 0x10 + *(longlong *)(lVar5 + 8);
  uVar9 = *(ulonglong *)(lVar5 + 0x10) - uVar7;
  if (*(ulonglong *)(lVar5 + 0x10) < uVar7) {
    lVar11 = 8;
    uVar9 = uVar16;
  }
  lVar6 = uVar14 * 0x10 + *(longlong *)(lVar5 + 0x28);
  uVar7 = *(ulonglong *)(lVar5 + 0x30) - uVar14;
  if (*(ulonglong *)(lVar5 + 0x30) < uVar14) {
    lVar6 = 8;
    uVar7 = uVar16;
  }
  *puVar4 = (*(longlong *)(lVar5 + 0x40) - uVar7) - uVar9;
  plVar8 = *(longlong **)(in_R9 + 8);
  if (plVar8 == (longlong *)0x0) {
    plVar8 = *(longlong **)(in_R9 + 0x10);
  }
  lVar19 = uVar9 * 0x10 + lVar11;
  lVar15 = uVar7 * 0x10 + lVar6;
  lVar5 = *in_RDX;
  lVar13 = in_RDX[1];
  do {
    if ((lVar11 == 0) || (lVar11 == lVar19)) {
      if ((lVar6 == 0) || (lVar6 == lVar15)) {
        lVar5 = *in_R8;
        lVar11 = in_R8[1];
        if ((*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar11 + 0x198)) &&
           (uVar9 = *(ulonglong *)(*(longlong *)(lVar11 + 400) + *(ulonglong *)(lVar5 + 0x110) * 8),
           uVar9 != 0)) {
          local_f0 = ~uVar9 * 0x90 + *(longlong *)(lVar11 + 0x160);
          cVar1 = *(char *)(lVar5 + 0x108);
        }
        else {
          local_f0 = 0;
          cVar1 = *(char *)(lVar5 + 0x108);
        }
        if (cVar1 == '\0') {
          uVar3 = *(undefined4 *)((longlong)in_R8 + 0x14);
          puVar18 = *(uint **)(lVar5 + 0xf8);
          puVar17 = puVar18 + *(longlong *)(lVar5 + 0x100);
          lVar6 = *in_stack_00000028;
          local_e8 = 8;
          lVar15 = 0;
          lVar13 = 0;
          local_c8 = 0;
          do {
            if (lVar15 == lVar13) {
              if (puVar18 == puVar17) {
                return;
              }
              while( true ) {
                lVar13 = *(longlong *)
                          (*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
                if (lVar13 != 0) break;
                puVar18 = puVar18 + 1;
                if (puVar18 == puVar17) {
                  return;
                }
              }
              lVar19 = *(longlong *)(lVar11 + 0x100) + (ulonglong)*puVar18 * 0xa0;
              uVar9 = (ulonglong)*(uint *)(lVar19 + 0x94);
              lVar15 = *(longlong *)(*(longlong *)(lVar11 + 0x1a8) + 0x18 + uVar9 * 0x48);
              lVar10 = ~*(ulonglong *)
                         (*(longlong *)(*(longlong *)(lVar11 + 0x1a8) + 0x38 + uVar9 * 0x48) +
                         *(longlong *)(lVar5 + 0x118) * 8) * 0x30;
              local_c8 = *(longlong *)(lVar15 + 0x10 + lVar10);
              local_c0 = *(longlong *)(lVar15 + 0x28 + lVar10);
              local_e8 = *(longlong *)(lVar19 + 0x50);
              puVar18 = puVar18 + 1;
              lVar15 = 0;
            }
            uVar9 = *(ulonglong *)(local_e8 + lVar15 * 0x10);
            uVar16 = (ulonglong)*(uint *)(local_e8 + 8 + lVar15 * 0x10);
            uVar7 = (ulonglong)
                    *(uint *)(*(longlong *)(local_f0 + 0x80) + 4 + (uVar9 & 0xffffffff) * 8);
            lVar10 = *(longlong *)(local_f0 + 8) * uVar7;
            lVar19 = *(longlong *)(local_f0 + 0x10);
            fVar21 = *(float *)(lVar6 + 0x58);
            fVar20 = *(float *)(lVar19 + 4 + lVar10);
            *(undefined4 *)(*(longlong *)(local_f0 + 0x50) + uVar7 * 4) = uVar3;
            fVar20 = fVar21 / fVar20 + *(float *)(lVar19 + 8 + lVar10);
            fVar21 = 0.0;
            if (0.0 <= fVar20) {
              fVar21 = fVar20;
            }
            fVar20 = 1.0;
            if (fVar21 <= 1.0) {
              fVar20 = fVar21;
            }
            *(float *)(lVar19 + 8 + lVar10) = fVar20;
            _X = (fVar20 * 5.0 + -1.0) * 3.1415927;
            fVar20 = sinf(_X);
            fVar21 = *(float *)(lVar19 + lVar10);
            *(undefined4 *)(local_c0 + uVar16 * 4) = uVar3;
            *(float *)(local_c8 + 0x10 + uVar16 * 0x14) =
                 ((fVar20 / _X) * fVar21 + 1.0) * *(float *)(local_c8 + 0x10 + uVar16 * 0x14);
            fVar21 = *(float *)(lVar19 + 8 + lVar10);
            if ((fVar21 == 1.0) && (!NAN(fVar21))) {
              lVar19 = plVar8[2];
              if ((ulonglong)(*plVar8 - lVar19) < 0x18) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              lVar10 = plVar8[1];
              *(code **)(lVar10 + lVar19) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<toolbar::resouces_and_components::UiPlayLockedAnimation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar10 + 8 + lVar19) = bevy_ecs::error::handler::warn;
              *(ulonglong *)(lVar10 + 0x10 + lVar19) = uVar9;
              plVar8[2] = lVar19 + 0x18;
            }
            lVar15 = lVar15 + 1;
          } while( true );
        }
        return;
      }
      lVar11 = 0;
      lVar10 = lVar6 + 0x10;
      lVar12 = lVar6;
    }
    else {
      lVar10 = lVar6;
      lVar12 = lVar11;
      lVar11 = lVar11 + 0x10;
    }
    lVar6 = lVar10;
    *puVar4 = *puVar4 + 1;
    uVar9 = *(ulonglong *)(lVar12 + 8);
    uVar7 = uVar9 & 0xffffffff;
    if ((uVar7 < *(ulonglong *)(lVar13 + 0x10)) &&
       (lVar10 = *(longlong *)(lVar13 + 8), *(int *)(lVar10 + uVar7 * 0x14) == (int)(uVar9 >> 0x20))
       ) {
      uVar2 = *(uint *)(lVar10 + uVar7 * 0x14 + 4);
      uVar7 = (ulonglong)uVar2;
      if ((uVar7 != 0xffffffff) &&
         ((uVar7 < *(ulonglong *)(lVar5 + 0x38) &&
          ((*(ulonglong *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)(uVar2 >> 6) * 8) >>
            (uVar7 & 0x3f) & 1) != 0)))) {
        lVar10 = plVar8[2];
        if ((ulonglong)(*plVar8 - lVar10) < 0x28) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar12 = plVar8[1];
        *(code **)(lVar12 + lVar10) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<toolbar::resouces_and_components::UiPlayLockedAnimation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar12 + 8 + lVar10) = bevy_ecs::error::handler::panic;
        *(ulonglong *)(lVar12 + 0x10 + lVar10) = uVar9;
        *(undefined8 *)(lVar12 + 0x18 + lVar10) = 0x3e99999a3dcccccd;
        *(undefined4 *)(lVar12 + 0x20 + lVar10) = 0;
        *(undefined1 *)(lVar12 + 0x24 + lVar10) = 0;
        plVar8[2] = lVar10 + 0x28;
      }
    }
  } while( true );
}


