// archive_current_decorators_in_storage<ref$<str$>_>  size=1098
// void __cdecl archive_current_decorators_in_storage<ref$<str$>_>(void)

// WARNING: Removing unreachable block (ram,0x0001413e75b6)
// WARNING: Removing unreachable block (ram,0x0001413e75cb)

void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::
archive_current_decorators_in_storage<ref_<str_>_>(void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  code *pcVar4;
  ulonglong extraout_RAX;
  void *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong in_RDX;
  longlong in_R8;
  longlong *in_R9;
  ulonglong uVar5;
  ushort uVar6;
  longlong lVar7;
  uint uVar8;
  byte bVar9;
  char cVar10;
  char cVar12;
  char cVar13;
  char cVar14;
  undefined1 auVar11 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  void *in_stack_00000030;
  size_t in_stack_00000038;
  size_t *local_f8;
  code *pcStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  undefined *local_c8;
  longlong lStack_c0;
  size_t **local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  size_t local_90;
  void *local_88;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  if (in_RDX == 0) {
    if (in_R9[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar9 = (byte)(extraout_RAX >> 0x39);
      auVar11 = ZEXT216(CONCAT11(bVar9,bVar9));
      auVar11 = pshuflw(auVar11,auVar11,0);
      lVar7 = 0;
      uVar5 = extraout_RAX;
      while( true ) {
        uVar5 = in_R9[1] & uVar5;
        pcVar1 = (char *)(*in_R9 + uVar5);
        cVar10 = auVar11[0];
        auVar16[0] = -(*pcVar1 == cVar10);
        cVar12 = auVar11[1];
        auVar16[1] = -(pcVar1[1] == cVar12);
        cVar13 = auVar11[2];
        auVar16[2] = -(pcVar1[2] == cVar13);
        cVar14 = auVar11[3];
        auVar16[3] = -(pcVar1[3] == cVar14);
        auVar16[4] = -(pcVar1[4] == cVar10);
        auVar16[5] = -(pcVar1[5] == cVar12);
        auVar16[6] = -(pcVar1[6] == cVar13);
        auVar16[7] = -(pcVar1[7] == cVar14);
        auVar16[8] = -(pcVar1[8] == cVar10);
        auVar16[9] = -(pcVar1[9] == cVar12);
        auVar16[10] = -(pcVar1[10] == cVar13);
        auVar16[0xb] = -(pcVar1[0xb] == cVar14);
        auVar16[0xc] = -(pcVar1[0xc] == cVar10);
        auVar16[0xd] = -(pcVar1[0xd] == cVar12);
        auVar16[0xe] = -(pcVar1[0xe] == cVar13);
        auVar16[0xf] = -(pcVar1[0xf] == cVar14);
        uVar6 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe | (ushort)(auVar16[0xf] >> 7) << 0xf;
        uVar8 = (uint)uVar6;
        while (uVar6 != 0) {
          uVar2 = 0;
          for (uVar3 = uVar8; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
            uVar2 = uVar2 + 1;
          }
          if (*(longlong *)(*in_R9 + -0x98 + (uVar2 + uVar5 & in_R9[1]) * -0x98) == in_R8)
          goto LAB_1413e7607;
          uVar6 = (ushort)(uVar8 - 1) & (ushort)uVar8;
          uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar6);
        }
        auVar15[0] = -(*pcVar1 == -1);
        auVar15[1] = -(pcVar1[1] == -1);
        auVar15[2] = -(pcVar1[2] == -1);
        auVar15[3] = -(pcVar1[3] == -1);
        auVar15[4] = -(pcVar1[4] == -1);
        auVar15[5] = -(pcVar1[5] == -1);
        auVar15[6] = -(pcVar1[6] == -1);
        auVar15[7] = -(pcVar1[7] == -1);
        auVar15[8] = -(pcVar1[8] == -1);
        auVar15[9] = -(pcVar1[9] == -1);
        auVar15[10] = -(pcVar1[10] == -1);
        auVar15[0xb] = -(pcVar1[0xb] == -1);
        auVar15[0xc] = -(pcVar1[0xc] == -1);
        auVar15[0xd] = -(pcVar1[0xd] == -1);
        auVar15[0xe] = -(pcVar1[0xe] == -1);
        auVar15[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar15 >> 7,0) & 1) != 0 || (SUB161(auVar15 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar15 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar15 >> 0x1f,0) & 1) != 0) || (SUB161(auVar15 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar15 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar15 >> 0x37,0) & 1) != 0) || (SUB161(auVar15 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar15 >> 0x47,0) & 1) != 0) || (SUB161(auVar15 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar15 >> 0x57,0) & 1) != 0) || (SUB161(auVar15 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar15 >> 0x67,0) & 1) != 0) || (SUB161(auVar15 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar15 >> 0x77,0) & 1) != 0) || auVar15[0xf] < '\0') break;
        uVar5 = uVar5 + lVar7 + 0x10;
        lVar7 = lVar7 + 0x10;
      }
    }
  }
  else {
    if (in_RDX == 1) {
      lVar7 = in_R9[0x12];
    }
    else {
      lVar7 = in_R9[0x24];
    }
    if (lVar7 != 0) {
LAB_1413e7607:
      archive_storage_decorators<ref_<str_>_>();
      return;
    }
  }
  if ((longlong)in_stack_00000038 < 0) {
LAB_1413e7636:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_stack_00000038 == 0) {
    local_88 = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_88 = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_1413e7636;
  }
  memcpy(local_88,in_stack_00000030,in_stack_00000038);
  local_90 = in_stack_00000038;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_d8 = *extraout_RAX_01;
  lStack_d0 = extraout_RAX_01[1];
  *extraout_RAX_01 = *extraout_RAX_01 + 1;
  local_e8 = 0;
  uStack_e0 = 0;
  local_f8 = (size_t *)&anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
  pcStack_f0 = (code *)0x0;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 != (longlong *)0x0) {
    local_a8 = *extraout_RAX_02;
    lStack_a0 = extraout_RAX_02[1];
    *extraout_RAX_02 = *extraout_RAX_02 + 1;
    local_c8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    lStack_c0 = 0;
    local_b8 = (size_t **)0x0;
    uStack_b0 = 0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    lVar7 = lStack_c0;
    pcVar4 = pcStack_f0;
    if (extraout_RAX_03 != (longlong *)0x0) {
      *extraout_RAX_03 = *extraout_RAX_03 + 1;
      local_f8 = &local_90;
      pcStack_f0 = alloc::string::impl$23::fmt;
      local_c8 = &anon_75abc2eb47494bf1a10abec2d08a795b_20_llvm_2298456351553783597;
      lStack_c0 = 2;
      local_b8 = &local_f8;
      uStack_b0 = 1;
      local_a8 = 0;
      alloc::fmt::format::format_inner();
      archive_storage_decorators<alloc::string::String>();
      if ((pcVar4 != (code *)0x0) && ((longlong)pcVar4 * 0x51 != -0x61)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((lVar7 != 0) && (lVar7 + (lVar7 * 4 + 0x13U & 0xfffffffffffffff0) != -0x11)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_90 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'archive_storage_decorators<alloc::string::String>', 'address': '1413e6150'}, {'name': 'archive_storage_decorators<ref$<str$>_>', 'address': '1413e5360'}, {'name': 'call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>', 'address': '1400d9b20'}, {'name': 'format_inner', 'address': '140d79f90'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}, {'name': 'memcpy', 'address': '142923c00'}, {'name': 'panic_access_error', 'address': '1428d80d0'}]