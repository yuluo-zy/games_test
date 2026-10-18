
// WARNING: Removing unreachable block (ram,0x0001407f9663)
// WARNING: Removing unreachable block (ram,0x0001407f9670)
// WARNING: Removing unreachable block (ram,0x0001407f96b5)
// WARNING: Removing unreachable block (ram,0x0001407f96be)
// WARNING: Removing unreachable block (ram,0x0001407f96cd)

void __cdecl
system_decorator::decorator_visual::maintain_visual_entities::clear_out_visual_entities(void)

{
  byte *pbVar1;
  undefined1 auVar2 [16];
  byte bVar3;
  byte bVar4;
  byte bVar5;
  longlong lVar6;
  ulonglong *puVar7;
  longlong lVar8;
  ushort uVar9;
  int iVar10;
  ulonglong *puVar11;
  char extraout_AL;
  uint uVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *in_RCX;
  longlong *plVar13;
  ulonglong in_RDX;
  undefined *puVar14;
  undefined1 (*local_108) [16];
  undefined1 (*local_100) [16];
  ushort local_f0;
  longlong local_e8;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x57) {
          if ((char)(&DAT_142a8c410)[in_RDX] < -0x40) {
LAB_1407f92d7:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x57) goto LAB_1407f92d7;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x57) {
          if ("stem_decorator::decorator_visual::maintain_visual_entities::clear_out_visual_entities::f"
              [in_RDX] < -0x40) {
LAB_1407f932b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x57) goto LAB_1407f932b;
      }
    }
    puVar14 = &DAT_142a8c581;
    do {
      bVar3 = puVar14[-1];
      uVar12 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar14[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar14[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar14 + -4;
            puVar14 = puVar14 + -4;
            uVar12 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar14 = puVar14 + -3;
            uVar12 = bVar5 & 0xf;
          }
          uVar12 = bVar4 & 0x3f | uVar12 << 6;
        }
        else {
          puVar14 = puVar14 + -2;
          uVar12 = bVar4 & 0x1f;
        }
        uVar12 = (uint)(bVar3 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_1407f93e4;
LAB_1407f93f6:
        if (puVar14 == &UNK_142a8c537) break;
        if (puVar14 + -0x142a8c537 < (undefined *)0x49) {
          if (-0x41 < (char)puVar14[1]) break;
        }
        else if (puVar14 + -0x142a8c537 == (undefined *)0x49) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar14 = puVar14 + -1;
      if (uVar12 == 0x5c) goto LAB_1407f93f6;
LAB_1407f93e4:
      if (uVar12 == 0x2f) goto LAB_1407f93f6;
    } while (puVar14 != &DAT_142a8c538);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar13 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_1407f9768;
    if (*plVar13 != 0) goto LAB_1407f9774;
    *plVar13 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar13 = *plVar13 + 1;
  }
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  lVar6 = *in_RCX;
  local_108 = *(undefined1 (**) [16])(lVar6 + 0x48);
  auVar2 = *local_108;
  local_f0 = ~((ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
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
  local_e8 = *(longlong *)(lVar6 + 0x60);
  *(undefined **)(lVar6 + 0x48) = &anon_4e7f1218742912fb20d1cab74f57dfe2_0_llvm_16591663493409836454
  ;
  *(undefined8 *)(lVar6 + 0x50) = 0;
  *(undefined8 *)(lVar6 + 0x58) = 0;
  *(undefined8 *)(lVar6 + 0x60) = 0;
  local_100 = local_108 + 1;
  do {
    if ((local_108 == (undefined1 (*) [16])0x0) || (local_e8 == 0)) break;
    uVar12 = (uint)local_f0;
    if (local_f0 == 0) {
      do {
        auVar2 = *local_100;
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
        local_108 = local_108 + -0x20;
        local_100 = local_100 + 1;
      } while (uVar9 == 0xffff);
      uVar12 = ~(uint)uVar9;
      local_f0 = (ushort)uVar12 & -uVar9 - 2;
    }
    else {
      local_f0 = local_f0 - 1 & local_f0;
    }
    local_e8 = local_e8 + -1;
    iVar10 = 0;
    for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      iVar10 = iVar10 + 1;
    }
    uVar12 = iVar10 << 5;
    lVar6 = *(longlong *)((longlong)local_108 + (-0x18 - (ulonglong)uVar12));
    if (SBORROW8(0,lVar6)) break;
    puVar7 = *(ulonglong **)((longlong)local_108 + (-0x10 - (ulonglong)uVar12));
    lVar8 = *(longlong *)((longlong)local_108 + (-8 - (ulonglong)uVar12));
    puVar11 = puVar7;
    while (puVar7 != (ulonglong *)0x0) {
      if ((puVar11 == puVar7 + lVar8) || (*puVar11 >> 0x20 == 0)) {
        if (lVar6 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        break;
      }
      bevy_ecs::system::commands::EntityCommands::despawn();
      puVar11 = puVar11 + 1;
    }
  } while( true );
  core::ptr::
  drop_in_place<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Drain<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<bevy_ecs::entity::Entity,alloc::alloc::Global>,system_decorator::decorator_visual_state::impl_3::drain::closure_0::closure_env_0>,system_decorator::decorator_visual_state::impl_3::drain::closure_env_0>_>
            ();
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar13 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_1407f9768:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar13 != 0) {
LAB_1407f9774:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar13 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar13 = *plVar13 + 1;
  }
  return;
}

