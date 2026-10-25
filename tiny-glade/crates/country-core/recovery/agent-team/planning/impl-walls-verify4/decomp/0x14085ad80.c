
void __cdecl country_core::resources::walls::inner_walls::InnerWalls::load_state(void)

{
  ulonglong *puVar1;
  ulonglong *puVar2;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar3;
  undefined8 *in_RCX;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong *in_RDX;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  ulonglong *puVar10;
  ulonglong uVar11;
  longlong lVar12;
  
  if (in_RCX[3] != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RCX[0xf] != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RCX[0x15] != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  lVar8 = in_RDX[2];
  if (lVar8 == 0) {
    if (*in_RDX != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
  }
  else {
    puVar1 = (ulonglong *)in_RDX[1];
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX[0] = 1;
      extraout_RAX[1] = 0;
      extraout_RAX[2] = 0;
      extraout_RAX[3] = 0;
      extraout_RAX[4] = 0;
      extraout_RAX[5] = 0;
      extraout_RAX[6] = 0;
      extraout_RAX[7] = 0;
      *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
      *(longlong **)(extraout_RAX + 0x10) = in_RDX;
      lVar3 = extraout_RAX_00;
    }
    else {
      lVar3 = *(longlong *)(extraout_RAX + 8);
      in_RDX = *(longlong **)(extraout_RAX + 0x10);
    }
    *(longlong *)(extraout_RAX + 8) = lVar3 + 1;
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::history::HistorySnapshotId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::history::HistorySnapshotId,tuple_<>,std::hash::random::RandomState>_>
              ();
    lVar12 = 0;
    do {
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      lVar12 = lVar12 + 1;
    } while (lVar12 != lVar8);
    lVar12 = in_RCX[7];
    if ((lVar12 != 0) && (lVar12 + (lVar12 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    in_RCX[10] = lVar3;
    in_RCX[0xb] = in_RDX;
    in_RCX[8] = 0;
    in_RCX[9] = 0;
    in_RCX[6] = &anon_6a7339a970014eecaf19421d4b3f97a6_0_llvm_4089674097218605020;
    in_RCX[7] = 0;
    uVar4 = *puVar1;
    if (lVar8 != 1) {
      uVar7 = (lVar8 * 0x78 - 0x78U >> 3) * -0x1111111111111111;
      uVar6 = (ulonglong)((uint)uVar7 & 3);
      if (uVar7 - 1 < 3) {
        uVar9 = 0;
      }
      else {
        uVar9 = 0;
        puVar2 = puVar1;
        do {
          puVar10 = puVar2 + 0x3c;
          uVar11 = puVar2[0xf];
          if (uVar4 < uVar11) {
            uVar5 = puVar2[0x1e];
            uVar4 = uVar11;
            if (uVar5 <= uVar11) goto LAB_14085b05f;
LAB_14085b03c:
            uVar11 = puVar2[0x2d];
            if (uVar11 <= uVar5) goto LAB_14085b06b;
LAB_14085b045:
            uVar4 = *puVar10;
            uVar5 = uVar11;
            if (uVar4 <= uVar11) {
LAB_14085b076:
              uVar4 = uVar5;
            }
          }
          else {
            uVar5 = puVar2[0x1e];
            if (uVar4 < uVar5) goto LAB_14085b03c;
LAB_14085b05f:
            uVar5 = uVar4;
            uVar11 = puVar2[0x2d];
            if (uVar5 < uVar11) goto LAB_14085b045;
LAB_14085b06b:
            uVar4 = *puVar10;
            if (uVar4 <= uVar5) goto LAB_14085b076;
          }
          uVar9 = uVar9 + 4;
          puVar2 = puVar10;
        } while ((uVar7 & 0x3fffffffffffffc) != uVar9);
      }
      if (uVar6 != 0) {
        lVar8 = 0;
        do {
          uVar7 = *(ulonglong *)((longlong)puVar1 + lVar8 + uVar9 * 0x78 + 0x78);
          if (uVar4 < uVar7) {
            uVar4 = uVar7;
          }
          lVar8 = lVar8 + 0x78;
        } while (uVar6 * 0x78 - lVar8 != 0);
      }
    }
    in_RCX[0x18] = uVar4;
    std::collections::hash::map::impl$87::
    from_iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>_>
              ();
    hashbrown::raw::RawTableInner::
    drop_inner_table<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>
              ();
    in_RCX[4] = lVar3;
    in_RCX[5] = in_RDX;
    in_RCX[2] = 0;
    in_RCX[3] = 0;
    *in_RCX = &anon_6a7339a970014eecaf19421d4b3f97a6_0_llvm_4089674097218605020;
    in_RCX[1] = 0;
  }
  return;
}

