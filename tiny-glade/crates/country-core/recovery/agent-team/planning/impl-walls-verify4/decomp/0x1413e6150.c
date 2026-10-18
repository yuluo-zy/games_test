
void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::
archive_storage_decorators<alloc::string::String>(void)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined4 uVar3;
  longlong *plVar4;
  longlong lVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  undefined *puVar9;
  undefined8 uVar10;
  uint uVar11;
  uint uVar12;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  ulonglong *extraout_RAX_09;
  ulonglong extraout_RAX_10;
  ulonglong *extraout_RAX_11;
  ulonglong extraout_RAX_12;
  longlong *in_RCX;
  longlong in_RDX;
  longlong *plVar13;
  longlong lVar14;
  undefined1 (*pauVar15) [16];
  byte bVar16;
  undefined *in_R8;
  ulonglong uVar17;
  longlong in_R9;
  longlong lVar18;
  ushort uVar19;
  ulonglong uVar20;
  undefined1 (*pauVar21) [16];
  byte bVar22;
  char cVar23;
  char cVar24;
  char cVar26;
  char cVar27;
  char cVar28;
  undefined1 auVar25 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined *local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  longlong local_1e8;
  longlong lStack_1e0;
  undefined *local_1d8;
  longlong *plStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  longlong local_1b8;
  longlong lStack_1b0;
  undefined *local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  longlong local_188;
  longlong lStack_180;
  undefined *local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  longlong local_158;
  longlong lStack_150;
  undefined *local_148;
  longlong *plStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  longlong local_128;
  longlong lStack_120;
  undefined *local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong *local_e0;
  undefined *local_d8;
  longlong *plStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  longlong local_b8;
  longlong lStack_b0;
  longlong local_a8;
  longlong local_a0;
  longlong *local_98;
  int local_90;
  int local_8c;
  longlong local_88;
  longlong *local_80;
  char local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_a0 = in_stack_00000028;
  if (in_RDX == 0) {
    local_98 = in_stack_00000038;
    local_71 = '\x01';
    local_178 = in_R8;
    local_a8 = in_R9;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar16 = (byte)(extraout_RAX_02 >> 0x38);
    bVar22 = bVar16 >> 1;
    auVar25 = ZEXT216(CONCAT11(bVar22,bVar22));
    auVar25 = pshuflw(auVar25,auVar25,0);
    lVar14 = 0;
    uVar17 = extraout_RAX_02;
    while( true ) {
      uVar17 = in_RCX[1] & uVar17;
      pcVar1 = (char *)(*in_RCX + uVar17);
      cVar23 = auVar25[0];
      auVar34[0] = -(*pcVar1 == cVar23);
      cVar24 = auVar25[1];
      auVar34[1] = -(pcVar1[1] == cVar24);
      cVar26 = auVar25[2];
      auVar34[2] = -(pcVar1[2] == cVar26);
      cVar27 = auVar25[3];
      auVar34[3] = -(pcVar1[3] == cVar27);
      auVar34[4] = -(pcVar1[4] == cVar23);
      auVar34[5] = -(pcVar1[5] == cVar24);
      auVar34[6] = -(pcVar1[6] == cVar26);
      auVar34[7] = -(pcVar1[7] == cVar27);
      auVar34[8] = -(pcVar1[8] == cVar23);
      auVar34[9] = -(pcVar1[9] == cVar24);
      auVar34[10] = -(pcVar1[10] == cVar26);
      auVar34[0xb] = -(pcVar1[0xb] == cVar27);
      auVar34[0xc] = -(pcVar1[0xc] == cVar23);
      auVar34[0xd] = -(pcVar1[0xd] == cVar24);
      auVar34[0xe] = -(pcVar1[0xe] == cVar26);
      auVar34[0xf] = -(pcVar1[0xf] == cVar27);
      uVar19 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar19;
      while (uVar19 != 0) {
        uVar12 = 0;
        for (uVar7 = uVar11; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar12 = uVar12 + 1;
        }
        if (*(undefined **)(*in_RCX + -0x98 + (uVar12 + uVar17 & in_RCX[1]) * -0x98) == in_R8)
        goto LAB_1413e6c3d;
        uVar19 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar19);
      }
      auVar31[0] = -(*pcVar1 == -1);
      auVar31[1] = -(pcVar1[1] == -1);
      auVar31[2] = -(pcVar1[2] == -1);
      auVar31[3] = -(pcVar1[3] == -1);
      auVar31[4] = -(pcVar1[4] == -1);
      auVar31[5] = -(pcVar1[5] == -1);
      auVar31[6] = -(pcVar1[6] == -1);
      auVar31[7] = -(pcVar1[7] == -1);
      auVar31[8] = -(pcVar1[8] == -1);
      auVar31[9] = -(pcVar1[9] == -1);
      auVar31[10] = -(pcVar1[10] == -1);
      auVar31[0xb] = -(pcVar1[0xb] == -1);
      auVar31[0xc] = -(pcVar1[0xc] == -1);
      auVar31[0xd] = -(pcVar1[0xd] == -1);
      auVar31[0xe] = -(pcVar1[0xe] == -1);
      auVar31[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar31 >> 0x17,0) & 1) != 0) || (SUB161(auVar31 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar31 >> 0x27,0) & 1) != 0) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar31 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar31 >> 0x3f,0) & 1) != 0) || (SUB161(auVar31 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar31 >> 0x4f,0) & 1) != 0) || (SUB161(auVar31 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar31 >> 0x5f,0) & 1) != 0) || (SUB161(auVar31 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar31 >> 0x6f,0) & 1) != 0) || (SUB161(auVar31 >> 0x77,0) & 1) != 0) ||
          auVar31[0xf] < '\0') break;
      uVar17 = uVar17 + lVar14 + 0x10;
      lVar14 = lVar14 + 0x10;
    }
    if (in_RCX[2] == 0) {
      local_71 = 1;
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,std::hash::random::RandomState>_>
                ();
      in_R8 = local_178;
    }
    local_71 = '\x01';
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_06 == (longlong *)0x0) {
      local_71 = 1;
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_b8 = *extraout_RAX_06;
    lStack_b0 = extraout_RAX_06[1];
    *extraout_RAX_06 = *extraout_RAX_06 + 1;
    local_c8 = 0;
    uStack_c0 = 0;
    local_d8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
    plStack_d0 = (longlong *)0x0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_07 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_158 = *extraout_RAX_07;
    lStack_150 = extraout_RAX_07[1];
    *extraout_RAX_07 = *extraout_RAX_07 + 1;
    local_178 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    uStack_170 = 0;
    local_168 = 0;
    uStack_160 = 0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_08 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_188 = *extraout_RAX_08;
    lStack_180 = extraout_RAX_08[1];
    *extraout_RAX_08 = *extraout_RAX_08 + 1;
    local_1b8 = local_b8;
    lStack_1b0 = lStack_b0;
    local_1c8 = local_c8;
    uStack_1c0 = uStack_c0;
    local_1d8 = local_d8;
    plStack_1d0 = plStack_d0;
    local_208 = local_178;
    uStack_200 = uStack_170;
    local_1f8 = local_168;
    uStack_1f0 = uStack_160;
    local_1e8 = local_158;
    lStack_1e0 = lStack_150;
    local_1a8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    uStack_1a0 = 0;
    local_198 = 0;
    uStack_190 = 0;
    decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
    memcpy(&local_178,&local_208,0x90);
    pauVar21 = (undefined1 (*) [16])*in_RCX;
    uVar17 = in_RCX[1];
    uVar20 = extraout_RAX_02 & uVar17;
    auVar25 = *(undefined1 (*) [16])(*pauVar21 + uVar20);
    uVar19 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar25[0xf] >> 7) << 0xf;
    uVar11 = (uint)uVar19;
    if (uVar19 == 0) {
      lVar14 = 0x10;
      do {
        uVar20 = uVar20 + lVar14 & uVar17;
        auVar25 = *(undefined1 (*) [16])(*pauVar21 + uVar20);
        lVar14 = lVar14 + 0x10;
        uVar11 = (uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar25[0xf] >> 7) << 0xf);
      } while (uVar11 == 0);
    }
    uVar12 = 0;
    for (; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
      uVar12 = uVar12 + 1;
    }
    uVar20 = uVar12 + uVar20 & uVar17;
    bVar22 = (*pauVar21)[uVar20];
    if (-1 < (char)bVar22) {
      auVar25 = *pauVar21;
      uVar11 = 0;
      for (uVar12 = (uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar25[0xf] >> 7) << 0xf); (uVar12 & 1) == 0;
          uVar12 = uVar12 >> 1 | 0x80000000) {
        uVar11 = uVar11 + 1;
      }
      uVar20 = (ulonglong)uVar11;
      bVar22 = (*pauVar21)[uVar20];
    }
    bVar16 = bVar16 >> 1;
    (*pauVar21)[uVar20] = bVar16;
    pauVar21[1][uVar20 - 0x10 & uVar17] = bVar16;
    in_RCX[2] = in_RCX[2] - (ulonglong)(bVar22 & 1);
    *(undefined **)((longlong)pauVar21 + uVar20 * -0x98 + -0x98) = in_R8;
    memcpy((void *)((longlong)pauVar21 + uVar20 * -0x98 + -0x90),&local_178,0x90);
    in_RCX[3] = in_RCX[3] + 1;
    cVar23 = '\x01';
    in_R9 = local_a8;
  }
  else {
    if (in_RDX == 1) {
      local_80 = (longlong *)*in_stack_00000038;
      local_88 = in_stack_00000038[1];
      if (in_RCX[0x18] != 0) {
LAB_1413e63bf:
        if (local_80 == (longlong *)0x0) {
          return;
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
        return;
      }
      local_98 = in_stack_00000038;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b8 = *extraout_RAX;
      lStack_b0 = extraout_RAX[1];
      *extraout_RAX = *extraout_RAX + 1;
      local_c8 = 0;
      uStack_c0 = 0;
      local_d8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
      plStack_d0 = (longlong *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_1e8 = *extraout_RAX_00;
      lStack_1e0 = extraout_RAX_00[1];
      *extraout_RAX_00 = *extraout_RAX_00 + 1;
      local_208 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_200 = 0;
      local_1f8 = 0;
      uStack_1f0 = 0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_f8 = *extraout_RAX_01;
      lStack_f0 = extraout_RAX_01[1];
      *extraout_RAX_01 = *extraout_RAX_01 + 1;
      local_128 = local_b8;
      lStack_120 = lStack_b0;
      local_138 = local_c8;
      uStack_130 = uStack_c0;
      local_148 = local_d8;
      plStack_140 = plStack_d0;
      local_178 = local_208;
      uStack_170 = uStack_200;
      local_168 = local_1f8;
      uStack_160 = uStack_1f0;
      local_158 = local_1e8;
      lStack_150 = lStack_1e0;
      local_118 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_110 = 0;
      local_108 = 0;
      uStack_100 = 0;
      decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
      plVar13 = in_RCX + 0x18;
    }
    else {
      local_80 = (longlong *)*in_stack_00000038;
      local_88 = in_stack_00000038[1];
      if (in_RCX[0x2a] != 0) goto LAB_1413e63bf;
      local_98 = in_stack_00000038;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b8 = *extraout_RAX_03;
      lStack_b0 = extraout_RAX_03[1];
      *extraout_RAX_03 = *extraout_RAX_03 + 1;
      local_c8 = 0;
      uStack_c0 = 0;
      local_d8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
      plStack_d0 = (longlong *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_1e8 = *extraout_RAX_04;
      lStack_1e0 = extraout_RAX_04[1];
      *extraout_RAX_04 = *extraout_RAX_04 + 1;
      local_208 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_200 = 0;
      local_1f8 = 0;
      uStack_1f0 = 0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_05 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_f8 = *extraout_RAX_05;
      lStack_f0 = extraout_RAX_05[1];
      *extraout_RAX_05 = *extraout_RAX_05 + 1;
      local_128 = local_b8;
      lStack_120 = lStack_b0;
      local_138 = local_c8;
      uStack_130 = uStack_c0;
      local_148 = local_d8;
      plStack_140 = plStack_d0;
      local_178 = local_208;
      uStack_170 = uStack_200;
      local_168 = local_1f8;
      uStack_160 = uStack_1f0;
      local_158 = local_1e8;
      lStack_150 = lStack_1e0;
      local_118 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_110 = 0;
      local_108 = 0;
      uStack_100 = 0;
      decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
      plVar13 = in_RCX + 0x2a;
    }
    memcpy(&local_208,&local_178,0x90);
    if (local_80 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    memcpy(plVar13,&local_208,0x90);
    cVar23 = '\0';
  }
  pauVar15 = *(undefined1 (**) [16])(in_R9 + 0x30);
  lVar14 = *(longlong *)(in_R9 + 0x48);
  auVar25 = *pauVar15;
  uVar11 = ~(uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar25[0xf] >> 7) << 0xf);
  pauVar21 = pauVar15 + 1;
  local_80 = in_stack_00000030 + 4;
  local_88 = local_a0 + 0x20;
  local_e0 = in_RCX;
  local_a8 = in_R9;
LAB_1413e68e3:
  do {
    if (lVar14 == 0) {
      pauVar15 = *(undefined1 (**) [16])(local_a8 + 0x30);
      lVar14 = *(longlong *)(local_a8 + 0x48);
      auVar25 = *pauVar15;
      uVar11 = ~(uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar25[0xf] >> 7) << 0xf);
      pauVar21 = pauVar15 + 1;
      local_e0 = local_e0 + 0x12;
      local_80 = in_stack_00000030 + 10;
LAB_1413e6b73:
      do {
        if (lVar14 == 0) {
          if (cVar23 != '\0') {
LAB_1413e6c3d:
            if (*local_98 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
          return;
        }
        if ((short)uVar11 == 0) {
          do {
            auVar25 = *pauVar21;
            uVar19 = (ushort)(SUB161(auVar25 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar25[0xf] >> 7) << 0xf;
            pauVar15 = pauVar15 + -0x50;
            pauVar21 = pauVar21 + 1;
          } while (uVar19 == 0xffff);
          uVar12 = ~(uint)uVar19;
          uVar11 = uVar12 & -(uint)uVar19 - 2;
        }
        else {
          uVar12 = uVar11;
          uVar11 = uVar11 - 1 & uVar11;
        }
        lVar14 = lVar14 + -1;
        uVar7 = 0;
        for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
          uVar7 = uVar7 + 1;
        }
        local_90 = *(int *)pauVar15[(ulonglong)uVar7 * -5 + -5];
      } while (in_stack_00000030[9] == 0);
      local_71 = cVar23;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar5 = in_stack_00000030[6];
      bVar16 = (byte)(extraout_RAX_12 >> 0x39);
      auVar25 = ZEXT216(CONCAT11(bVar16,bVar16));
      auVar25 = pshuflw(auVar25,auVar25,0);
      lVar18 = 0;
      uVar17 = extraout_RAX_12;
      do {
        uVar17 = in_stack_00000030[7] & uVar17;
        pcVar1 = (char *)(lVar5 + uVar17);
        cVar24 = auVar25[0];
        auVar33[0] = -(*pcVar1 == cVar24);
        cVar26 = auVar25[1];
        auVar33[1] = -(pcVar1[1] == cVar26);
        cVar27 = auVar25[2];
        auVar33[2] = -(pcVar1[2] == cVar27);
        cVar28 = auVar25[3];
        auVar33[3] = -(pcVar1[3] == cVar28);
        auVar33[4] = -(pcVar1[4] == cVar24);
        auVar33[5] = -(pcVar1[5] == cVar26);
        auVar33[6] = -(pcVar1[6] == cVar27);
        auVar33[7] = -(pcVar1[7] == cVar28);
        auVar33[8] = -(pcVar1[8] == cVar24);
        auVar33[9] = -(pcVar1[9] == cVar26);
        auVar33[10] = -(pcVar1[10] == cVar27);
        auVar33[0xb] = -(pcVar1[0xb] == cVar28);
        auVar33[0xc] = -(pcVar1[0xc] == cVar24);
        auVar33[0xd] = -(pcVar1[0xd] == cVar26);
        auVar33[0xe] = -(pcVar1[0xe] == cVar27);
        auVar33[0xf] = -(pcVar1[0xf] == cVar28);
        uVar19 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe | (ushort)(auVar33[0xf] >> 7) << 0xf
        ;
        uVar12 = (uint)uVar19;
        while (uVar19 != 0) {
          uVar7 = 0;
          for (uVar8 = uVar12; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar7 = uVar7 + 1;
          }
          uVar20 = uVar7 + uVar17 & in_stack_00000030[7];
          if (local_90 == *(int *)(lVar5 + -0x18 + uVar20 * -0x18)) {
            if (*(char *)(lVar5 + -4 + uVar20 * -0x18) != '\x11') {
              local_71 = cVar23;
              rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
              if (extraout_RAX_11 == (ulonglong *)0x0) {
                local_71 = cVar23;
                    // WARNING: Subroutine does not return
                core::option::expect_failed();
              }
              uVar17 = *extraout_RAX_11 & 0xffffffff;
              if (((*(ulonglong *)(local_a0 + 0x10) <= uVar17) ||
                  (lVar5 = *(longlong *)(local_a0 + 8), lVar5 == 0)) ||
                 (*(uint *)(lVar5 + 0x20 + uVar17 * 0x28) != ((uint)(*extraout_RAX_11 >> 0x20) | 1))
                 ) {
                local_71 = cVar23;
                    // WARNING: Subroutine does not return
                std::panicking::begin_panic<ref_<str_>_>();
              }
              uVar2 = *(undefined8 *)((longlong)extraout_RAX_11 + 0x44);
              uVar3 = *(undefined4 *)((longlong)extraout_RAX_11 + 0x4c);
              puVar9 = *(undefined **)((longlong)extraout_RAX_11 + 0x34);
              uVar10 = *(undefined8 *)((longlong)extraout_RAX_11 + 0x3c);
              plVar13 = (longlong *)(lVar5 + uVar17 * 0x28);
              plVar4 = (longlong *)plVar13[3];
              LOCK();
              lVar5 = *plVar4;
              *plVar4 = *plVar4 + 1;
              UNLOCK();
              if (*plVar4 == 0 || SCARRY8(lVar5,1) != *plVar4 < 0) {
LAB_1413e6de6:
                    // WARNING: Does not return
                pcVar6 = (code *)invalidInstructionException();
                (*pcVar6)();
              }
              local_148 = (undefined *)plVar13[2];
              local_158 = *plVar13;
              lStack_150 = plVar13[1];
              uStack_160 = CONCAT44(uStack_160._4_4_,uVar3);
              local_178 = puVar9;
              uStack_170 = uVar10;
              local_168 = uVar2;
              plStack_140 = plVar4;
              local_71 = cVar23;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              if ((char)local_1e8 != '\x11') {
                LOCK();
                *plStack_1d0 = *plStack_1d0 + -1;
                UNLOCK();
                if (*plStack_1d0 == 0) {
                  local_71 = cVar23;
                  alloc::sync::Arc::
                  drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                            ();
                }
              }
            }
            goto LAB_1413e6b73;
          }
          uVar19 = (ushort)(uVar12 - 1) & (ushort)uVar12;
          uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar19);
        }
        auVar30[0] = -(*pcVar1 == -1);
        auVar30[1] = -(pcVar1[1] == -1);
        auVar30[2] = -(pcVar1[2] == -1);
        auVar30[3] = -(pcVar1[3] == -1);
        auVar30[4] = -(pcVar1[4] == -1);
        auVar30[5] = -(pcVar1[5] == -1);
        auVar30[6] = -(pcVar1[6] == -1);
        auVar30[7] = -(pcVar1[7] == -1);
        auVar30[8] = -(pcVar1[8] == -1);
        auVar30[9] = -(pcVar1[9] == -1);
        auVar30[10] = -(pcVar1[10] == -1);
        auVar30[0xb] = -(pcVar1[0xb] == -1);
        auVar30[0xc] = -(pcVar1[0xc] == -1);
        auVar30[0xd] = -(pcVar1[0xd] == -1);
        auVar30[0xe] = -(pcVar1[0xe] == -1);
        auVar30[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 || (SUB161(auVar30 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar30 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar30 >> 0x1f,0) & 1) != 0) || (SUB161(auVar30 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar30 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar30 >> 0x37,0) & 1) != 0) || (SUB161(auVar30 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar30 >> 0x47,0) & 1) != 0) || (SUB161(auVar30 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar30 >> 0x57,0) & 1) != 0) || (SUB161(auVar30 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar30 >> 0x67,0) & 1) != 0) || (SUB161(auVar30 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar30 >> 0x77,0) & 1) != 0) || auVar30[0xf] < '\0') goto LAB_1413e6b73;
        uVar17 = uVar17 + lVar18 + 0x10;
        lVar18 = lVar18 + 0x10;
      } while( true );
    }
    if ((short)uVar11 == 0) {
      do {
        auVar25 = *pauVar21;
        uVar19 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar25[0xf] >> 7) << 0xf;
        pauVar15 = pauVar15 + -0x50;
        pauVar21 = pauVar21 + 1;
      } while (uVar19 == 0xffff);
      uVar12 = ~(uint)uVar19;
      uVar11 = uVar12 & -(uint)uVar19 - 2;
    }
    else {
      uVar12 = uVar11;
      uVar11 = uVar11 - 1 & uVar11;
    }
    lVar14 = lVar14 + -1;
    uVar7 = 0;
    for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      uVar7 = uVar7 + 1;
    }
    local_8c = *(int *)pauVar15[(ulonglong)uVar7 * -5 + -5];
    if (in_stack_00000030[3] != 0) {
      local_71 = cVar23;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar5 = *in_stack_00000030;
      bVar16 = (byte)(extraout_RAX_10 >> 0x39);
      auVar25 = ZEXT216(CONCAT11(bVar16,bVar16));
      auVar25 = pshuflw(auVar25,auVar25,0);
      lVar18 = 0;
      uVar17 = extraout_RAX_10;
      while( true ) {
        uVar17 = in_stack_00000030[1] & uVar17;
        pcVar1 = (char *)(lVar5 + uVar17);
        cVar24 = auVar25[0];
        auVar32[0] = -(*pcVar1 == cVar24);
        cVar26 = auVar25[1];
        auVar32[1] = -(pcVar1[1] == cVar26);
        cVar27 = auVar25[2];
        auVar32[2] = -(pcVar1[2] == cVar27);
        cVar28 = auVar25[3];
        auVar32[3] = -(pcVar1[3] == cVar28);
        auVar32[4] = -(pcVar1[4] == cVar24);
        auVar32[5] = -(pcVar1[5] == cVar26);
        auVar32[6] = -(pcVar1[6] == cVar27);
        auVar32[7] = -(pcVar1[7] == cVar28);
        auVar32[8] = -(pcVar1[8] == cVar24);
        auVar32[9] = -(pcVar1[9] == cVar26);
        auVar32[10] = -(pcVar1[10] == cVar27);
        auVar32[0xb] = -(pcVar1[0xb] == cVar28);
        auVar32[0xc] = -(pcVar1[0xc] == cVar24);
        auVar32[0xd] = -(pcVar1[0xd] == cVar26);
        auVar32[0xe] = -(pcVar1[0xe] == cVar27);
        auVar32[0xf] = -(pcVar1[0xf] == cVar28);
        uVar19 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe | (ushort)(auVar32[0xf] >> 7) << 0xf
        ;
        uVar12 = (uint)uVar19;
        while (uVar19 != 0) {
          uVar7 = 0;
          for (uVar8 = uVar12; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar7 = uVar7 + 1;
          }
          uVar20 = uVar7 + uVar17 & in_stack_00000030[1];
          if (local_8c == *(int *)(lVar5 + -0x18 + uVar20 * -0x18)) {
            if (*(char *)(lVar5 + -4 + uVar20 * -0x18) == '\x11') goto LAB_1413e68e3;
            local_71 = cVar23;
            rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
            if (extraout_RAX_09 == (ulonglong *)0x0) {
              local_71 = cVar23;
                    // WARNING: Subroutine does not return
              core::option::expect_failed();
            }
            uVar17 = *extraout_RAX_09 & 0xffffffff;
            if (((*(ulonglong *)(local_a0 + 0x10) <= uVar17) ||
                (lVar5 = *(longlong *)(local_a0 + 8), lVar5 == 0)) ||
               (*(uint *)(lVar5 + 0x20 + uVar17 * 0x28) != ((uint)(*extraout_RAX_09 >> 0x20) | 1)))
            {
              local_71 = cVar23;
                    // WARNING: Subroutine does not return
              std::panicking::begin_panic<ref_<str_>_>();
            }
            uVar2 = *(undefined8 *)((longlong)extraout_RAX_09 + 0x44);
            uVar3 = *(undefined4 *)((longlong)extraout_RAX_09 + 0x4c);
            puVar9 = *(undefined **)((longlong)extraout_RAX_09 + 0x34);
            uVar10 = *(undefined8 *)((longlong)extraout_RAX_09 + 0x3c);
            plVar13 = (longlong *)(lVar5 + uVar17 * 0x28);
            plVar4 = (longlong *)plVar13[3];
            LOCK();
            lVar5 = *plVar4;
            *plVar4 = *plVar4 + 1;
            UNLOCK();
            if (*plVar4 == 0 || SCARRY8(lVar5,1) != *plVar4 < 0) goto LAB_1413e6de6;
            local_148 = (undefined *)plVar13[2];
            local_158 = *plVar13;
            lStack_150 = plVar13[1];
            uStack_160 = CONCAT44(uStack_160._4_4_,uVar3);
            local_178 = puVar9;
            uStack_170 = uVar10;
            local_168 = uVar2;
            plStack_140 = plVar4;
            local_71 = cVar23;
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            if ((char)local_1e8 != '\x11') {
              LOCK();
              *plStack_1d0 = *plStack_1d0 + -1;
              UNLOCK();
              if (*plStack_1d0 == 0) {
                local_71 = cVar23;
                alloc::sync::Arc::
                drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                          ();
              }
            }
            goto LAB_1413e68e3;
          }
          uVar19 = (ushort)(uVar12 - 1) & (ushort)uVar12;
          uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar19);
        }
        auVar29[0] = -(*pcVar1 == -1);
        auVar29[1] = -(pcVar1[1] == -1);
        auVar29[2] = -(pcVar1[2] == -1);
        auVar29[3] = -(pcVar1[3] == -1);
        auVar29[4] = -(pcVar1[4] == -1);
        auVar29[5] = -(pcVar1[5] == -1);
        auVar29[6] = -(pcVar1[6] == -1);
        auVar29[7] = -(pcVar1[7] == -1);
        auVar29[8] = -(pcVar1[8] == -1);
        auVar29[9] = -(pcVar1[9] == -1);
        auVar29[10] = -(pcVar1[10] == -1);
        auVar29[0xb] = -(pcVar1[0xb] == -1);
        auVar29[0xc] = -(pcVar1[0xc] == -1);
        auVar29[0xd] = -(pcVar1[0xd] == -1);
        auVar29[0xe] = -(pcVar1[0xe] == -1);
        auVar29[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x1f,0) & 1) != 0) || (SUB161(auVar29 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x37,0) & 1) != 0) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
        uVar17 = uVar17 + lVar18 + 0x10;
        lVar18 = lVar18 + 0x10;
      }
    }
  } while( true );
}

