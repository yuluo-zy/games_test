
void __cdecl
bevy_ecs::system::commands::entity_command::impl$0::
apply<tuple_<>,bevy_ecs::system::commands::entity_command::insert::closure_env_0<bevy_ecs::entity_disabling::Disabled>_>
          (void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  int iVar3;
  longlong *plVar4;
  code *pcVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  undefined4 uVar8;
  uint uVar9;
  uint uVar10;
  longlong *plVar11;
  longlong *plVar12;
  char in_CL;
  longlong lVar13;
  longlong *plVar14;
  longlong *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  undefined4 uVar19;
  longlong lVar20;
  longlong *plVar21;
  ulonglong uVar22;
  longlong lVar23;
  undefined8 in_stack_fffffffffffffea8;
  undefined4 local_130;
  undefined8 local_110;
  longlong *local_108;
  undefined1 local_100;
  undefined7 uStack_ff;
  longlong *local_f8;
  longlong local_f0;
  longlong local_e8;
  longlong local_e0;
  longlong *local_d8;
  longlong local_d0;
  undefined1 local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong *plStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  longlong local_98;
  longlong *local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  undefined4 local_48;
  
  uVar19 = (undefined4)((ulonglong)in_stack_fffffffffffffea8 >> 0x20);
  lVar23 = *in_RDX;
  plStack_b0 = (longlong *)in_RDX[1];
  local_a8 = in_RDX[2];
  uStack_a0 = in_RDX[3];
  iVar3 = (int)local_a8;
  local_b8 = lVar23;
  if ((int)local_a8 == -1) {
                    // WARNING: Subroutine does not return
    world::entity_ref::EntityWorldMut::panic_despawned();
  }
  uVar2 = *(undefined4 *)(lVar23 + 0x5b0);
  local_110 = (longlong *)(lVar23 + 0x38);
  local_108 = (longlong *)(lVar23 + 0x5a8);
  bundle::Bundles::register_info<bevy_ecs::entity_disabling::Disabled>();
  lVar20 = CONCAT44(uVar19,uVar2);
  bundle::BundleInserter::new_with_id();
  plVar21 = plStack_b0;
  uVar10 = local_a8._4_4_;
  uVar22 = (ulonglong)local_a8._4_4_;
  uVar19 = (undefined4)uStack_a0;
  uVar16 = uStack_a0 >> 0x20;
  local_e8 = local_70;
  if (in_CL == '\0') {
    uVar9 = *(uint *)(local_58 + 0x98);
    if ((char)uVar9 < '\0') {
      lVar20 = *(longlong *)(local_68 + 0x50) + *(longlong *)(local_68 + 0x58) * 8;
      local_110 = (longlong *)((ulonglong)local_110 & 0xffffffffffffff00);
      observer::Observers::
      invoke<tuple_<>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<bevy_ecs::component::ComponentId>_>_>
                ();
      uVar9 = *(uint *)(local_58 + 0x98);
    }
    if (((uVar9 & 4) != 0) && (lVar13 = *(longlong *)(local_68 + 0x58), lVar13 != 0)) {
      lVar18 = *(longlong *)(local_68 + 0x50);
      lVar17 = 0;
      do {
        plVar4 = *(longlong **)(lVar18 + lVar17);
        pcVar5 = *(code **)(*(longlong *)(local_50 + 0x40) + 0xb8 + (longlong)plVar4 * 0xd0);
        if (pcVar5 != (code *)0x0) {
          local_110 = plVar21;
          local_100 = 0;
          local_108 = plVar4;
          (*pcVar5)(local_50,&local_110);
        }
        lVar17 = lVar17 + 8;
      } while (lVar13 << 3 != lVar17);
    }
  }
  local_e0 = lVar23;
  lVar23 = local_50;
  if (local_88 == 0) {
    uVar15 = **(ulonglong **)(local_e8 + 8);
    if ((uVar15 < *(ulonglong *)(local_60 + 0x40)) &&
       (uVar6 = *(ulonglong *)(*(longlong *)(local_60 + 0x38) + uVar15 * 8), uVar6 != 0)) {
      lVar13 = ~uVar6 * 0x30 + *(longlong *)(local_60 + 0x18);
    }
    else {
      lVar13 = 0;
    }
    local_e8 = *(longlong *)(local_68 + 0x20);
    lVar18 = *(longlong *)(local_68 + 0x28);
    if (**(char **)(local_68 + 8) == '\0') {
      memmove((void *)(*(size_t *)(lVar13 + 8) * uVar16 + *(longlong *)(lVar13 + 0x10)),&local_d8,
              *(size_t *)(lVar13 + 8));
      *(undefined4 *)(*(longlong *)(lVar13 + 0x20) + uVar16 * 4) = local_48;
LAB_140039081:
      *(undefined4 *)(*(longlong *)(lVar13 + 0x28) + uVar16 * 4) = local_48;
    }
    else {
      if (in_CL == '\0') {
        storage::blob_array::BlobArray::replace_unchecked();
        goto LAB_140039081;
      }
      if (((uVar15 < *(ulonglong *)(local_60 + 0x40)) &&
          (uVar15 = *(ulonglong *)(*(longlong *)(local_60 + 0x38) + uVar15 * 8), uVar15 != 0)) &&
         (pcVar5 = *(code **)(*(longlong *)(local_60 + 0x18) + 0x18 + ~uVar15 * 0x30),
         pcVar5 != (code *)0x0)) {
        (*pcVar5)(&local_d8,**(char **)(local_68 + 8),local_e8,local_68,lVar20);
      }
    }
    if (lVar18 != 0) {
      lVar18 = lVar18 << 4;
      do {
        component::RequiredComponentConstructor::initialize();
        lVar18 = lVar18 + -0x10;
      } while (lVar18 != 0);
    }
    uVar10 = *(uint *)(local_58 + 0x98);
    lVar20 = local_58;
  }
  else {
    uVar19 = (undefined4)((ulonglong)plVar21 >> 0x20);
    if ((int)local_88 == 1) {
      if (*(ulonglong *)(local_58 + 0x58) <= uVar22) goto LAB_1400396b2;
      uVar15 = *(ulonglong *)(local_58 + 0x58) - 1;
      lVar20 = *(longlong *)(local_58 + 0x50);
      lVar13 = uVar22 * 0x10;
      uVar9 = *(uint *)(lVar20 + 8 + lVar13);
      uVar16 = (ulonglong)uVar9;
      puVar1 = (undefined8 *)(lVar20 + uVar15 * 0x10);
      uVar7 = puVar1[1];
      *(undefined8 *)(lVar20 + lVar13) = *puVar1;
      ((undefined8 *)(lVar20 + lVar13))[1] = uVar7;
      *(ulonglong *)(local_58 + 0x58) = uVar15;
      if (uVar15 != uVar22) {
        if (uVar15 <= uVar22) goto LAB_1400396c1;
        if (((uint *)(lVar20 + lVar13))[1] != 0) {
          *(uint *)(*(longlong *)(local_50 + 8) + 8 + (ulonglong)*(uint *)(lVar20 + lVar13) * 0x14)
               = uVar10;
        }
      }
      uVar22 = *(ulonglong *)(local_80 + 0x58);
      if (uVar22 == *(ulonglong *)(local_80 + 0x48)) {
        alloc::raw_vec::RawVec::
        grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                  ();
      }
      lVar20 = *(longlong *)(local_80 + 0x50);
      lVar13 = uVar22 * 0x10;
      *(int *)(lVar20 + lVar13) = (int)plVar21;
      *(undefined4 *)(lVar20 + 4 + lVar13) = uVar19;
      *(uint *)(lVar20 + 8 + lVar13) = uVar9;
      *(ulonglong *)(local_80 + 0x58) = uVar22 + 1;
      iVar3 = *(int *)(local_80 + 0x90);
      uVar19 = *(undefined4 *)(local_80 + 0x94);
      lVar20 = *(longlong *)(local_50 + 8);
      uVar15 = (ulonglong)plVar21 & 0xffffffff;
      *(int *)(lVar20 + 4 + uVar15 * 0x14) = iVar3;
      *(int *)(lVar20 + 8 + uVar15 * 0x14) = (int)uVar22;
      *(undefined4 *)(lVar20 + 0xc + uVar15 * 0x14) = uVar19;
      *(uint *)(lVar20 + 0x10 + uVar15 * 0x14) = uVar9;
      uVar15 = **(ulonglong **)(local_e8 + 8);
      local_f0 = local_80;
      if ((uVar15 < *(ulonglong *)(local_60 + 0x40)) &&
         (uVar6 = *(ulonglong *)(*(longlong *)(local_60 + 0x38) + uVar15 * 8), uVar6 != 0)) {
        lVar20 = ~uVar6 * 0x30 + *(longlong *)(local_60 + 0x18);
      }
      else {
        lVar20 = 0;
      }
      lVar13 = *(longlong *)(local_68 + 0x28);
      if (**(char **)(local_68 + 8) == '\0') {
        memmove((void *)(*(size_t *)(lVar20 + 8) * uVar16 + *(longlong *)(lVar20 + 0x10)),&local_d8,
                *(size_t *)(lVar20 + 8));
        *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + uVar16 * 4) = local_48;
LAB_14003911e:
        *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + uVar16 * 4) = local_48;
      }
      else {
        if (in_CL == '\0') {
          storage::blob_array::BlobArray::replace_unchecked();
          goto LAB_14003911e;
        }
        if (((uVar15 < *(ulonglong *)(local_60 + 0x40)) &&
            (uVar15 = *(ulonglong *)(*(longlong *)(local_60 + 0x38) + uVar15 * 8), uVar15 != 0)) &&
           (pcVar5 = *(code **)(*(longlong *)(local_60 + 0x18) + 0x18 + ~uVar15 * 0x30),
           pcVar5 != (code *)0x0)) {
          (*pcVar5)(&local_d8);
        }
      }
      if (lVar13 != 0) {
        lVar13 = lVar13 << 4;
        do {
          component::RequiredComponentConstructor::initialize();
          lVar13 = lVar13 + -0x10;
        } while (lVar13 != 0);
      }
      uVar10 = *(uint *)(local_f0 + 0x98);
      lVar20 = local_f0;
    }
    else {
      if (*(ulonglong *)(local_58 + 0x58) <= uVar22) {
LAB_1400396b2:
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::swap_remove::assert_failed();
      }
      local_90 = plVar21;
      local_c0 = local_78;
      lVar23 = *(longlong *)(local_50 + 0x100);
      uVar16 = *(ulonglong *)(local_58 + 0x58) - 1;
      lVar20 = *(longlong *)(local_58 + 0x50);
      lVar13 = uVar22 * 0x10;
      uVar2 = *(undefined4 *)(lVar20 + 8 + lVar13);
      puVar1 = (undefined8 *)(lVar20 + uVar16 * 0x10);
      uVar7 = puVar1[1];
      *(undefined8 *)(lVar20 + lVar13) = *puVar1;
      ((undefined8 *)(lVar20 + lVar13))[1] = uVar7;
      *(ulonglong *)(local_58 + 0x58) = uVar16;
      if (uVar16 != uVar22) {
        if (uVar16 <= uVar22) {
LAB_1400396c1:
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (((uint *)(lVar20 + lVar13))[1] != 0) {
          *(uint *)(*(longlong *)(local_50 + 8) + 8 + (ulonglong)*(uint *)(lVar20 + lVar13) * 0x14)
               = uVar10;
        }
      }
      storage::table::Table::move_to_superset_unchecked();
      uVar8 = local_108._0_4_;
      uVar16 = (ulonglong)local_108 & 0xffffffff;
      uVar22 = *(ulonglong *)(local_80 + 0x58);
      if (uVar22 == *(ulonglong *)(local_80 + 0x48)) {
        alloc::raw_vec::RawVec::
        grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                  ();
      }
      lVar20 = *(longlong *)(local_80 + 0x50);
      lVar13 = uVar22 * 0x10;
      *(int *)(lVar20 + lVar13) = (int)local_90;
      *(undefined4 *)(lVar20 + 4 + lVar13) = uVar19;
      *(undefined4 *)(lVar20 + 8 + lVar13) = uVar8;
      *(ulonglong *)(local_80 + 0x58) = uVar22 + 1;
      iVar3 = *(int *)(local_80 + 0x90);
      uVar19 = *(undefined4 *)(local_80 + 0x94);
      lVar20 = *(longlong *)(local_50 + 8);
      uVar15 = (ulonglong)local_90 & 0xffffffff;
      *(int *)(lVar20 + 4 + uVar15 * 0x14) = iVar3;
      *(int *)(lVar20 + 8 + uVar15 * 0x14) = (int)uVar22;
      *(undefined4 *)(lVar20 + 0xc + uVar15 * 0x14) = uVar19;
      *(undefined4 *)(lVar20 + 0x10 + uVar15 * 0x14) = uVar8;
      if (local_110._4_4_ != 0) {
        uVar15 = (ulonglong)local_110 & 0xffffffff;
        lVar20 = *(longlong *)(local_50 + 8);
        uVar10 = *(uint *)(lVar20 + 4 + uVar15 * 0x14);
        *(undefined4 *)(lVar20 + 0x10 + uVar15 * 0x14) = uVar2;
        uVar15 = *(ulonglong *)(lVar20 + 8 + uVar15 * 0x14) & 0xffffffff;
        if (*(uint *)(local_58 + 0x90) == uVar10) {
          if (*(ulonglong *)(local_58 + 0x58) <= uVar15) goto LAB_1400396d0;
          plVar21 = (longlong *)(local_58 + 0x50);
        }
        else if (*(uint *)(local_80 + 0x90) == uVar10) {
          if (*(ulonglong *)(local_80 + 0x58) <= uVar15) {
LAB_1400396d0:
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          plVar21 = (longlong *)(local_80 + 0x50);
        }
        else {
          if (*(ulonglong *)(lVar23 + 0x58 + (ulonglong)uVar10 * 0xa0) <= uVar15)
          goto LAB_1400396d0;
          plVar21 = (longlong *)(lVar23 + (ulonglong)uVar10 * 0xa0 + 0x50);
        }
        *(undefined4 *)(*plVar21 + 8 + uVar15 * 0x10) = uVar2;
      }
      uVar15 = **(ulonglong **)(local_e8 + 8);
      if ((uVar15 < *(ulonglong *)(local_78 + 0x40)) &&
         (uVar6 = *(ulonglong *)(*(longlong *)(local_78 + 0x38) + uVar15 * 8), uVar6 != 0)) {
        lVar23 = ~uVar6 * 0x30 + *(longlong *)(local_78 + 0x18);
      }
      else {
        lVar23 = 0;
      }
      lVar20 = *(longlong *)(local_68 + 0x28);
      local_f0 = local_80;
      local_98 = local_50;
      if (**(char **)(local_68 + 8) == '\0') {
        memmove((void *)(*(size_t *)(lVar23 + 8) * uVar16 + *(longlong *)(lVar23 + 0x10)),&local_d8,
                *(size_t *)(lVar23 + 8));
        *(undefined4 *)(*(longlong *)(lVar23 + 0x20) + uVar16 * 4) = local_48;
LAB_1400392d0:
        *(undefined4 *)(*(longlong *)(lVar23 + 0x28) + uVar16 * 4) = local_48;
      }
      else {
        if (in_CL == '\0') {
          storage::blob_array::BlobArray::replace_unchecked();
          goto LAB_1400392d0;
        }
        if (((uVar15 < *(ulonglong *)(local_78 + 0x40)) &&
            (uVar15 = *(ulonglong *)(*(longlong *)(local_c0 + 0x38) + uVar15 * 8), uVar15 != 0)) &&
           (pcVar5 = *(code **)(*(longlong *)(local_c0 + 0x18) + 0x18 + ~uVar15 * 0x30),
           pcVar5 != (code *)0x0)) {
          (*pcVar5)(&local_d8);
        }
      }
      plVar21 = local_90;
      if (lVar20 != 0) {
        lVar20 = lVar20 << 4;
        do {
          component::RequiredComponentConstructor::initialize();
          lVar20 = lVar20 + -0x10;
        } while (lVar20 != 0);
      }
      uVar10 = *(uint *)(local_f0 + 0x98);
      lVar20 = local_f0;
      lVar23 = local_98;
    }
  }
  if (((uVar10 & 1) != 0) && (lVar13 = *(longlong *)(local_68 + 0x40), lVar13 != 0)) {
    lVar18 = *(longlong *)(local_68 + 0x38);
    lVar17 = 0;
    do {
      plVar4 = *(longlong **)(lVar18 + lVar17);
      pcVar5 = *(code **)(*(longlong *)(lVar23 + 0x40) + 0xa8 + (longlong)plVar4 * 0xd0);
      if (pcVar5 != (code *)0x0) {
        local_100 = 0;
        local_110 = plVar21;
        local_108 = plVar4;
        (*pcVar5)(lVar23,&local_110);
      }
      lVar17 = lVar17 + 8;
    } while (lVar13 << 3 != lVar17);
    uVar10 = *(uint *)(lVar20 + 0x98);
  }
  local_130 = (undefined4)uVar16;
  if ((uVar10 & 0x20) != 0) {
    local_110 = (longlong *)((ulonglong)local_110 & 0xffffffffffffff00);
    observer::Observers::
    invoke<tuple_<>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<bevy_ecs::component::ComponentId>_>_>
              ();
  }
  if (in_CL != '\0') {
    uVar10 = *(uint *)(lVar20 + 0x98);
    if (((uVar10 & 2) != 0) && (lVar13 = *(longlong *)(local_68 + 0x40), lVar13 != 0)) {
      lVar18 = *(longlong *)(local_68 + 0x38);
      lVar17 = 0;
      do {
        plVar4 = *(longlong **)(lVar18 + lVar17);
        pcVar5 = *(code **)(*(longlong *)(lVar23 + 0x40) + 0xb0 + (longlong)plVar4 * 0xd0);
        if (pcVar5 != (code *)0x0) {
          local_100 = 0;
          local_110 = plVar21;
          local_108 = plVar4;
          (*pcVar5)(lVar23,&local_110);
        }
        lVar17 = lVar17 + 8;
      } while (lVar13 << 3 != lVar17);
      uVar10 = *(uint *)(lVar20 + 0x98);
    }
    if ((uVar10 & 0x40) != 0) {
      local_110 = (longlong *)((ulonglong)local_110 & 0xffffffffffffff00);
      observer::Observers::
      invoke<tuple_<>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<bevy_ecs::component::ComponentId>_>_>
                ();
    }
    goto LAB_14003963a;
  }
  archetype::ArchetypeAfterBundleInsert::iter_inserted();
  plVar4 = local_108;
  uVar10 = *(uint *)(lVar20 + 0x98);
  if ((uVar10 & 2) != 0) {
    plVar11 = (longlong *)CONCAT71(uStack_ff,local_100);
    plVar14 = local_110;
    local_f0 = lVar20;
    do {
      if ((plVar14 == (longlong *)0x0) || (plVar14 == plVar4)) {
        if ((plVar11 == (longlong *)0x0) || (plVar11 == local_f8)) goto LAB_14003958d;
        plVar12 = plVar11 + 1;
        plVar14 = (longlong *)0x0;
      }
      else {
        plVar12 = plVar11;
        plVar11 = plVar14;
        plVar14 = plVar14 + 1;
      }
      lVar20 = *plVar11;
      pcVar5 = *(code **)(*(longlong *)(lVar23 + 0x40) + 0xb0 + lVar20 * 0xd0);
      plVar11 = plVar12;
      if (pcVar5 != (code *)0x0) {
        local_c8 = 0;
        local_d8 = plVar21;
        local_d0 = lVar20;
        (*pcVar5)(lVar23,&local_d8);
      }
    } while( true );
  }
LAB_1400395a0:
  if ((uVar10 & 0x40) != 0) {
    archetype::ArchetypeAfterBundleInsert::iter_inserted();
    local_d8 = (longlong *)((ulonglong)local_d8 & 0xffffffffffffff00);
    observer::Observers::
    invoke<tuple_<>,core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::slice::iter::Iter<bevy_ecs::component::ComponentId>,core::slice::iter::Iter<bevy_ecs::component::ComponentId>_>_>_>
              ();
  }
LAB_14003963a:
  local_a8 = CONCAT44((int)uVar22,iVar3);
  uStack_a0 = CONCAT44(local_130,uVar19);
  world::World::flush_entities();
  world::World::flush_components();
  world::World::flush_commands();
  world::entity_ref::EntityWorldMut::update_location();
  return;
LAB_14003958d:
  uVar10 = *(uint *)(local_f0 + 0x98);
  goto LAB_1400395a0;
}

