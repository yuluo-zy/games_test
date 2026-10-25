// archive_current_decorators_in_storages<ref$<str$>_>  size=4060
// void __cdecl archive_current_decorators_in_storages<ref$<str$>_>(void)

void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::
archive_current_decorators_in_storages<ref_<str_>_>(void)

{
  longlong *plVar1;
  char *pcVar2;
  longlong *plVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  undefined1 auVar8 [12];
  uint uVar9;
  uint uVar10;
  void *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  ulonglong extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  ulonglong *extraout_RAX_14;
  ulonglong extraout_RAX_15;
  ulonglong *extraout_RAX_16;
  ulonglong extraout_RAX_17;
  longlong *in_RCX;
  longlong *plVar11;
  longlong *in_RDX;
  longlong lVar12;
  undefined1 (*pauVar13) [16];
  longlong in_R8;
  ulonglong uVar14;
  longlong *in_R9;
  longlong lVar15;
  longlong lVar16;
  byte bVar17;
  longlong lVar18;
  undefined1 (*pauVar19) [16];
  longlong *plVar20;
  ushort uVar21;
  ulonglong uVar22;
  byte bVar23;
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
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 unaff_XMM6_Ba;
  char cVar37;
  undefined1 unaff_XMM6_Bb;
  char cVar38;
  undefined1 unaff_XMM6_Bc;
  char cVar39;
  undefined1 unaff_XMM6_Bd;
  char cVar40;
  undefined1 unaff_XMM6_Be;
  char cVar41;
  undefined1 unaff_XMM6_Bf;
  char cVar42;
  undefined1 unaff_XMM6_Bg;
  char cVar43;
  undefined1 unaff_XMM6_Bh;
  char cVar44;
  undefined1 unaff_XMM6_Bi;
  char cVar45;
  undefined1 unaff_XMM6_Bj;
  char cVar46;
  undefined1 unaff_XMM6_Bk;
  char cVar47;
  undefined1 unaff_XMM6_Bl;
  char cVar48;
  undefined1 unaff_XMM6_Bm;
  char cVar49;
  undefined1 unaff_XMM6_Bn;
  char cVar50;
  undefined1 unaff_XMM6_Bo;
  char cVar51;
  undefined1 unaff_XMM6_Bp;
  char cVar52;
  longlong *in_stack_00000028;
  void *in_stack_00000030;
  size_t in_stack_00000038;
  undefined1 local_258 [16];
  undefined1 local_248 [16];
  longlong local_238;
  longlong lStack_230;
  int *local_228;
  code *pcStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  longlong local_208;
  longlong lStack_200;
  undefined *local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  longlong local_1d8;
  longlong lStack_1d0;
  size_t local_1c0;
  void *local_1b8;
  undefined1 local_1a8 [16];
  undefined1 local_198 [16];
  longlong local_188;
  longlong lStack_180;
  int *local_178;
  code *pcStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  longlong local_158;
  longlong lStack_150;
  undefined *local_148;
  longlong lStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  longlong local_128;
  longlong lStack_120;
  longlong local_118;
  longlong *local_110;
  longlong *local_108;
  longlong *local_100;
  longlong *local_f8;
  longlong *local_f0;
  int local_e8;
  undefined4 uStack_e4;
  longlong *local_d0;
  int *local_c8;
  code *pcStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  int local_8c;
  longlong *local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  if ((longlong)in_stack_00000038 < 0) {
LAB_1413e8855:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_d0 = in_R9;
  local_88 = in_RCX;
  if (in_stack_00000038 == 0) {
    local_1b8 = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_1b8 = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_1413e8855;
  }
  memcpy(local_1b8,in_stack_00000030,in_stack_00000038);
  local_1c0 = in_stack_00000038;
  local_98 = *in_stack_00000028;
  plVar3 = (longlong *)in_stack_00000028[4];
  local_100 = local_88 + 0x2a;
  local_f8 = local_88 + 0x18;
  plVar1 = local_d0 + 0x24;
  local_108 = local_d0 + 0x12;
  local_f0 = local_88 + 4;
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar37 = (char)iVar7;
  cVar38 = (char)((uint)iVar7 >> 8);
  cVar39 = (char)((uint)iVar7 >> 0x10);
  cVar40 = (char)((uint)iVar7 >> 0x18);
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar41 = (char)iVar7;
  cVar42 = (char)((uint)iVar7 >> 8);
  cVar43 = (char)((uint)iVar7 >> 0x10);
  cVar44 = (char)((uint)iVar7 >> 0x18);
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar45 = (char)iVar7;
  cVar46 = (char)((uint)iVar7 >> 8);
  cVar47 = (char)((uint)iVar7 >> 0x10);
  cVar48 = (char)((uint)iVar7 >> 0x18);
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                 CONCAT13(unaff_XMM6_Bp,
                          CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar49 = (char)iVar7;
  cVar50 = (char)((uint)iVar7 >> 8);
  cVar51 = (char)((uint)iVar7 >> 0x10);
  cVar52 = (char)((uint)iVar7 >> 0x18);
  local_118 = local_98 + 0x20;
  plVar11 = in_RDX;
LAB_1413e7be0:
  if (plVar11 == in_RDX + in_R8 * 2) {
    if (local_1c0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
  local_110 = plVar11 + 2;
  lVar12 = plVar11[1];
  if (*plVar11 == 0) {
    local_1a8._0_8_ = lVar12;
    if (local_d0[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar15 = *local_d0;
      bVar17 = (byte)(extraout_RAX_06 >> 0x39);
      auVar25 = ZEXT216(CONCAT11(bVar17,bVar17));
      auVar25 = pshuflw(auVar25,auVar25,0);
      lVar16 = 0;
      uVar14 = extraout_RAX_06;
      while( true ) {
        uVar14 = local_d0[1] & uVar14;
        pcVar2 = (char *)(lVar15 + uVar14);
        cVar24 = auVar25[0];
        auVar34[0] = -(*pcVar2 == cVar24);
        cVar26 = auVar25[1];
        auVar34[1] = -(pcVar2[1] == cVar26);
        cVar27 = auVar25[2];
        auVar34[2] = -(pcVar2[2] == cVar27);
        cVar28 = auVar25[3];
        auVar34[3] = -(pcVar2[3] == cVar28);
        auVar34[4] = -(pcVar2[4] == cVar24);
        auVar34[5] = -(pcVar2[5] == cVar26);
        auVar34[6] = -(pcVar2[6] == cVar27);
        auVar34[7] = -(pcVar2[7] == cVar28);
        auVar34[8] = -(pcVar2[8] == cVar24);
        auVar34[9] = -(pcVar2[9] == cVar26);
        auVar34[10] = -(pcVar2[10] == cVar27);
        auVar34[0xb] = -(pcVar2[0xb] == cVar28);
        auVar34[0xc] = -(pcVar2[0xc] == cVar24);
        auVar34[0xd] = -(pcVar2[0xd] == cVar26);
        auVar34[0xe] = -(pcVar2[0xe] == cVar27);
        auVar34[0xf] = -(pcVar2[0xf] == cVar28);
        uVar21 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf
        ;
        uVar9 = (uint)uVar21;
        while (uVar21 != 0) {
          uVar10 = 0;
          for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar10 = uVar10 + 1;
          }
          lVar18 = (uVar10 + uVar14 & local_d0[1]) * -0x98;
          if (*(longlong *)(lVar15 + -0x98 + lVar18) == lVar12) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            plVar20 = (longlong *)(lVar15 + lVar18 + -0x90);
            bVar17 = (byte)(extraout_RAX_10 >> 0x38);
            bVar23 = bVar17 >> 1;
            auVar25 = ZEXT216(CONCAT11(bVar23,bVar23));
            auVar25 = pshuflw(auVar25,auVar25,0);
            lVar15 = 0;
            uVar14 = extraout_RAX_10;
            goto LAB_1413e7960;
          }
          uVar21 = (ushort)(uVar9 - 1) & (ushort)uVar9;
          uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar21);
        }
        auVar30[0] = -(*pcVar2 == cVar37);
        auVar30[1] = -(pcVar2[1] == cVar38);
        auVar30[2] = -(pcVar2[2] == cVar39);
        auVar30[3] = -(pcVar2[3] == cVar40);
        auVar30[4] = -(pcVar2[4] == cVar41);
        auVar30[5] = -(pcVar2[5] == cVar42);
        auVar30[6] = -(pcVar2[6] == cVar43);
        auVar30[7] = -(pcVar2[7] == cVar44);
        auVar30[8] = -(pcVar2[8] == cVar45);
        auVar30[9] = -(pcVar2[9] == cVar46);
        auVar30[10] = -(pcVar2[10] == cVar47);
        auVar30[0xb] = -(pcVar2[0xb] == cVar48);
        auVar30[0xc] = -(pcVar2[0xc] == cVar49);
        auVar30[0xd] = -(pcVar2[0xd] == cVar50);
        auVar30[0xe] = -(pcVar2[0xe] == cVar51);
        auVar30[0xf] = -(pcVar2[0xf] == cVar52);
        if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 || (SUB161(auVar30 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar30 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar30 >> 0x1f,0) & 1) != 0) || (SUB161(auVar30 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar30 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar30 >> 0x37,0) & 1) != 0) || (SUB161(auVar30 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar30 >> 0x47,0) & 1) != 0) || (SUB161(auVar30 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar30 >> 0x57,0) & 1) != 0) || (SUB161(auVar30 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar30 >> 0x67,0) & 1) != 0) || (SUB161(auVar30 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar30 >> 0x77,0) & 1) != 0) || auVar30[0xf] < '\0') break;
        uVar14 = uVar14 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
    }
LAB_1413e7e60:
    alloc::string::impl$6::clone();
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_07 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_a8 = *extraout_RAX_07;
    lStack_a0 = extraout_RAX_07[1];
    *extraout_RAX_07 = *extraout_RAX_07 + 1;
    local_b8 = 0;
    uStack_b0 = 0;
    local_c8 = (int *)&anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
    pcStack_c0 = (code *)0x0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_08 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_238 = *extraout_RAX_08;
    lStack_230 = extraout_RAX_08[1];
    *extraout_RAX_08 = *extraout_RAX_08 + 1;
    local_258._8_8_ = 0;
    local_258._0_8_ = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    local_248 = ZEXT816(0);
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_09 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_128 = *extraout_RAX_09;
    lStack_120 = extraout_RAX_09[1];
    *extraout_RAX_09 = *extraout_RAX_09 + 1;
    local_158 = local_a8;
    lStack_150 = lStack_a0;
    local_168 = local_b8;
    uStack_160 = uStack_b0;
    local_178 = local_c8;
    pcStack_170 = pcStack_c0;
    local_1a8 = local_258;
    local_198 = local_248;
    local_188 = local_238;
    lStack_180 = lStack_230;
    local_138 = 0;
    uStack_130 = 0;
    local_148 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    lStack_140 = 0;
    local_c8 = &local_e8;
    pcStack_c0 = alloc::string::impl$23::fmt;
    local_258._8_8_ = 2;
    local_258._0_8_ = &anon_75abc2eb47494bf1a10abec2d08a795b_20_llvm_2298456351553783597;
    local_248._8_8_ = 1;
    local_248._0_8_ = &local_c8;
    local_238 = 0;
    alloc::fmt::format::format_inner();
    archive_storage_decorators<alloc::string::String>();
    if ((pcStack_170 != (code *)0x0) && ((longlong)pcStack_170 * 0x51 != -0x61)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((local_1a8._8_8_ != 0) &&
       (local_1a8._8_8_ + (local_1a8._8_8_ * 4 + 0x13U & 0xfffffffffffffff0) != -0x11)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((lStack_140 != 0) && (lStack_140 + (lStack_140 * 4 + 0x13U & 0xfffffffffffffff0) != -0x11))
    {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar11 = local_110;
    if (CONCAT44(uStack_e4,local_e8) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      plVar11 = local_110;
    }
    goto LAB_1413e7be0;
  }
  if ((int)*plVar11 == 1) {
    if (*local_108 == 0) goto LAB_1413e7e60;
    plVar11 = local_110;
    if (*local_f8 != 0) goto LAB_1413e7be0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_a8 = *extraout_RAX_03;
    lStack_a0 = extraout_RAX_03[1];
    *extraout_RAX_03 = *extraout_RAX_03 + 1;
    local_b8 = 0;
    uStack_b0 = 0;
    local_c8 = (int *)&anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
    pcStack_c0 = (code *)0x0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_238 = *extraout_RAX_04;
    lStack_230 = extraout_RAX_04[1];
    *extraout_RAX_04 = *extraout_RAX_04 + 1;
    local_258._8_8_ = 0;
    local_258._0_8_ = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    local_248 = ZEXT816(0);
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    plVar20 = local_108;
    if (extraout_RAX_05 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_128 = *extraout_RAX_05;
    lStack_120 = extraout_RAX_05[1];
    *extraout_RAX_05 = *extraout_RAX_05 + 1;
    local_158 = local_a8;
    lStack_150 = lStack_a0;
    local_168 = local_b8;
    uStack_160 = uStack_b0;
    local_178 = local_c8;
    pcStack_170 = pcStack_c0;
    local_1a8 = local_258;
    local_198 = local_248;
    local_188 = local_238;
    lStack_180 = lStack_230;
    local_138 = 0;
    uStack_130 = 0;
    local_148 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    lStack_140 = 0;
    decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
    memcpy(local_258,local_1a8,0x90);
    plVar11 = local_f8;
  }
  else {
    if (*plVar1 == 0) goto LAB_1413e7e60;
    plVar11 = local_110;
    if (*local_100 != 0) goto LAB_1413e7be0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_11 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_a8 = *extraout_RAX_11;
    lStack_a0 = extraout_RAX_11[1];
    *extraout_RAX_11 = *extraout_RAX_11 + 1;
    local_b8 = 0;
    uStack_b0 = 0;
    local_c8 = (int *)&anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
    pcStack_c0 = (code *)0x0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_12 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_238 = *extraout_RAX_12;
    lStack_230 = extraout_RAX_12[1];
    *extraout_RAX_12 = *extraout_RAX_12 + 1;
    local_258._8_8_ = 0;
    local_258._0_8_ = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    local_248 = ZEXT816(0);
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_13 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_128 = *extraout_RAX_13;
    lStack_120 = extraout_RAX_13[1];
    *extraout_RAX_13 = *extraout_RAX_13 + 1;
    local_158 = local_a8;
    lStack_150 = lStack_a0;
    local_168 = local_b8;
    uStack_160 = uStack_b0;
    local_178 = local_c8;
    pcStack_170 = pcStack_c0;
    local_1a8 = local_258;
    local_198 = local_248;
    local_188 = local_238;
    lStack_180 = lStack_230;
    local_138 = 0;
    uStack_130 = 0;
    local_148 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    lStack_140 = 0;
    decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
    memcpy(local_258,local_1a8,0x90);
    plVar11 = local_100;
    plVar20 = plVar1;
  }
  memcpy(plVar11,local_258,0x90);
  goto LAB_1413e8229;
LAB_1413e7960:
  uVar14 = local_88[1] & uVar14;
  auVar34 = *(undefined1 (*) [16])(*local_88 + uVar14);
  cVar24 = auVar25[0];
  auVar33[0] = -(auVar34[0] == cVar24);
  cVar26 = auVar25[1];
  auVar33[1] = -(auVar34[1] == cVar26);
  cVar27 = auVar25[2];
  auVar33[2] = -(auVar34[2] == cVar27);
  cVar28 = auVar25[3];
  auVar33[3] = -(auVar34[3] == cVar28);
  auVar33[4] = -(auVar34[4] == cVar24);
  auVar33[5] = -(auVar34[5] == cVar26);
  auVar33[6] = -(auVar34[6] == cVar27);
  auVar33[7] = -(auVar34[7] == cVar28);
  auVar33[8] = -(auVar34[8] == cVar24);
  auVar33[9] = -(auVar34[9] == cVar26);
  auVar33[10] = -(auVar34[10] == cVar27);
  auVar33[0xb] = -(auVar34[0xb] == cVar28);
  auVar33[0xc] = -(auVar34[0xc] == cVar24);
  auVar33[0xd] = -(auVar34[0xd] == cVar26);
  auVar33[0xe] = -(auVar34[0xe] == cVar27);
  auVar33[0xf] = -(auVar34[0xf] == cVar28);
  uVar21 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
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
           (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe | (ushort)(auVar33[0xf] >> 7) << 0xf;
  uVar9 = (uint)uVar21;
  while (uVar21 != 0) {
    uVar10 = 0;
    for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
      uVar10 = uVar10 + 1;
    }
    plVar11 = local_110;
    if (*(longlong *)(*local_88 + -0x98 + (uVar10 + uVar14 & local_88[1]) * -0x98) == lVar12)
    goto LAB_1413e7be0;
    uVar21 = (ushort)(uVar9 - 1) & (ushort)uVar9;
    uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar21);
  }
  auVar29[0] = -(auVar34[0] == cVar37);
  auVar29[1] = -(auVar34[1] == cVar38);
  auVar29[2] = -(auVar34[2] == cVar39);
  auVar29[3] = -(auVar34[3] == cVar40);
  auVar29[4] = -(auVar34[4] == cVar41);
  auVar29[5] = -(auVar34[5] == cVar42);
  auVar29[6] = -(auVar34[6] == cVar43);
  auVar29[7] = -(auVar34[7] == cVar44);
  auVar29[8] = -(auVar34[8] == cVar45);
  auVar29[9] = -(auVar34[9] == cVar46);
  auVar29[10] = -(auVar34[10] == cVar47);
  auVar29[0xb] = -(auVar34[0xb] == cVar48);
  auVar29[0xc] = -(auVar34[0xc] == cVar49);
  auVar29[0xd] = -(auVar34[0xd] == cVar50);
  auVar29[0xe] = -(auVar34[0xe] == cVar51);
  auVar29[0xf] = -(auVar34[0xf] == cVar52);
  if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar29 >> 0x17,0) & 1) != 0) || (SUB161(auVar29 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar29 >> 0x27,0) & 1) != 0) || (SUB161(auVar29 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x37,0) & 1) != 0) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') goto LAB_1413e79ce;
  uVar14 = uVar14 + lVar15 + 0x10;
  lVar15 = lVar15 + 0x10;
  goto LAB_1413e7960;
LAB_1413e79ce:
  if (local_88[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,std::hash::random::RandomState>_>
              ();
    lVar12 = local_1a8._0_8_;
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_a8 = *extraout_RAX_00;
  lStack_a0 = extraout_RAX_00[1];
  *extraout_RAX_00 = *extraout_RAX_00 + 1;
  local_b8 = 0;
  uStack_b0 = 0;
  local_c8 = (int *)&anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
  pcStack_c0 = (code *)0x0;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_188 = *extraout_RAX_01;
  lStack_180 = extraout_RAX_01[1];
  *extraout_RAX_01 = *extraout_RAX_01 + 1;
  local_1a8._8_8_ = 0;
  local_1a8._0_8_ = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  local_198 = ZEXT816(0);
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_1d8 = *extraout_RAX_02;
  lStack_1d0 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  local_208 = local_a8;
  lStack_200 = lStack_a0;
  local_218 = local_b8;
  uStack_210 = uStack_b0;
  local_228 = local_c8;
  pcStack_220 = pcStack_c0;
  local_258 = local_1a8;
  local_248 = local_198;
  local_238 = local_188;
  lStack_230 = lStack_180;
  local_1e8 = 0;
  uStack_1e0 = 0;
  local_1f8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  uStack_1f0 = 0;
  decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
  memcpy(local_1a8,local_258,0x90);
  plVar11 = local_88;
  pauVar19 = (undefined1 (*) [16])*local_88;
  uVar14 = local_88[1];
  uVar22 = extraout_RAX_10 & uVar14;
  auVar25 = *(undefined1 (*) [16])(*pauVar19 + uVar22);
  uVar21 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
           (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar25[0xf] >> 7) << 0xf
  ;
  uVar9 = (uint)uVar21;
  if (uVar21 == 0) {
    lVar15 = 0x10;
    do {
      uVar22 = uVar22 + lVar15 & uVar14;
      auVar25 = *(undefined1 (*) [16])(*pauVar19 + uVar22);
      lVar15 = lVar15 + 0x10;
      uVar9 = (uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
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
    } while (uVar9 == 0);
  }
  uVar10 = 0;
  for (; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
    uVar10 = uVar10 + 1;
  }
  uVar22 = uVar10 + uVar22 & uVar14;
  bVar23 = (*pauVar19)[uVar22];
  if (-1 < (char)bVar23) {
    auVar25 = *pauVar19;
    uVar9 = 0;
    for (uVar10 = (uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
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
                                (ushort)(byte)(auVar25[0xf] >> 7) << 0xf); (uVar10 & 1) == 0;
        uVar10 = uVar10 >> 1 | 0x80000000) {
      uVar9 = uVar9 + 1;
    }
    uVar22 = (ulonglong)uVar9;
    bVar23 = (*pauVar19)[uVar22];
  }
  bVar17 = bVar17 >> 1;
  (*pauVar19)[uVar22] = bVar17;
  pauVar19[1][uVar22 - 0x10 & uVar14] = bVar17;
  local_88[2] = local_88[2] - (ulonglong)(bVar23 & 1);
  *(longlong *)((longlong)pauVar19 + uVar22 * -0x98 + -0x98) = lVar12;
  memcpy((void *)((longlong)pauVar19 + uVar22 * -0x98 + -0x90),local_1a8,0x90);
  plVar11[3] = plVar11[3] + 1;
LAB_1413e8229:
  pauVar13 = (undefined1 (*) [16])plVar20[6];
  lVar12 = plVar20[9];
  auVar25 = *pauVar13;
  uVar9 = ~(uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
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
  pauVar19 = pauVar13 + 1;
LAB_1413e83d3:
  if (lVar12 != 0) {
    if ((short)uVar9 == 0) {
      do {
        auVar25 = *pauVar19;
        uVar21 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
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
        pauVar13 = pauVar13 + -0x50;
        pauVar19 = pauVar19 + 1;
      } while (uVar21 == 0xffff);
      uVar10 = ~(uint)uVar21;
      uVar9 = uVar10 & -(uint)uVar21 - 2;
    }
    else {
      uVar10 = uVar9;
      uVar9 = uVar9 - 1 & uVar9;
    }
    lVar12 = lVar12 + -1;
    uVar5 = 0;
    for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
      uVar5 = uVar5 + 1;
    }
    local_8c = *(int *)pauVar13[(ulonglong)uVar5 * -5 + -5];
    if (plVar3[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar15 = *plVar3;
      bVar17 = (byte)(extraout_RAX_15 >> 0x39);
      auVar25 = ZEXT216(CONCAT11(bVar17,bVar17));
      auVar25 = pshuflw(auVar25,auVar25,0);
      lVar16 = 0;
      uVar14 = extraout_RAX_15;
      while( true ) {
        uVar14 = plVar3[1] & uVar14;
        auVar34 = *(undefined1 (*) [16])(lVar15 + uVar14);
        cVar24 = auVar25[0];
        auVar35[0] = -(auVar34[0] == cVar24);
        cVar26 = auVar25[1];
        auVar35[1] = -(auVar34[1] == cVar26);
        cVar27 = auVar25[2];
        auVar35[2] = -(auVar34[2] == cVar27);
        cVar28 = auVar25[3];
        auVar35[3] = -(auVar34[3] == cVar28);
        auVar35[4] = -(auVar34[4] == cVar24);
        auVar35[5] = -(auVar34[5] == cVar26);
        auVar35[6] = -(auVar34[6] == cVar27);
        auVar35[7] = -(auVar34[7] == cVar28);
        auVar35[8] = -(auVar34[8] == cVar24);
        auVar35[9] = -(auVar34[9] == cVar26);
        auVar35[10] = -(auVar34[10] == cVar27);
        auVar35[0xb] = -(auVar34[0xb] == cVar28);
        auVar35[0xc] = -(auVar34[0xc] == cVar24);
        auVar35[0xd] = -(auVar34[0xd] == cVar26);
        auVar35[0xe] = -(auVar34[0xe] == cVar27);
        auVar35[0xf] = -(auVar34[0xf] == cVar28);
        uVar21 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf
        ;
        uVar10 = (uint)uVar21;
        while (uVar21 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          uVar22 = uVar5 + uVar14 & plVar3[1];
          if (local_8c == *(int *)(lVar15 + -0x18 + uVar22 * -0x18)) {
            if (*(char *)(lVar15 + -4 + uVar22 * -0x18) == '\x11') goto LAB_1413e83d3;
            rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
            if (extraout_RAX_14 == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
              core::option::expect_failed();
            }
            uVar14 = *extraout_RAX_14 & 0xffffffff;
            if (((*(ulonglong *)(local_98 + 0x10) <= uVar14) ||
                (lVar15 = *(longlong *)(local_98 + 8), lVar15 == 0)) ||
               (*(uint *)(lVar15 + 0x20 + uVar14 * 0x28) != ((uint)(*extraout_RAX_14 >> 0x20) | 1)))
            {
                    // WARNING: Subroutine does not return
              std::panicking::begin_panic<ref_<str_>_>();
            }
            auVar8 = *(undefined1 (*) [12])((longlong)extraout_RAX_14 + 0x44);
            auVar25 = *(undefined1 (*) [16])((longlong)extraout_RAX_14 + 0x34);
            plVar11 = (longlong *)(lVar15 + uVar14 * 0x28);
            pcVar4 = (code *)plVar11[3];
            LOCK();
            lVar15 = *(longlong *)pcVar4;
            *(longlong *)pcVar4 = *(longlong *)pcVar4 + 1;
            UNLOCK();
            if (*(longlong *)pcVar4 == 0 || SCARRY8(lVar15,1) != *(longlong *)pcVar4 < 0)
            goto LAB_1413e8850;
            local_178 = (int *)plVar11[2];
            local_188 = *plVar11;
            lStack_180 = plVar11[1];
            local_198._0_12_ = auVar8;
            local_1a8 = auVar25;
            pcStack_170 = pcVar4;
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            if ((char)local_238 != '\x11') {
              LOCK();
              *(longlong *)pcStack_220 = *(longlong *)pcStack_220 + -1;
              UNLOCK();
              if (*(longlong *)pcStack_220 == 0) {
                alloc::sync::Arc::
                drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                          ();
              }
            }
            goto LAB_1413e83d3;
          }
          uVar21 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar21);
        }
        auVar31[0] = -(auVar34[0] == cVar37);
        auVar31[1] = -(auVar34[1] == cVar38);
        auVar31[2] = -(auVar34[2] == cVar39);
        auVar31[3] = -(auVar34[3] == cVar40);
        auVar31[4] = -(auVar34[4] == cVar41);
        auVar31[5] = -(auVar34[5] == cVar42);
        auVar31[6] = -(auVar34[6] == cVar43);
        auVar31[7] = -(auVar34[7] == cVar44);
        auVar31[8] = -(auVar34[8] == cVar45);
        auVar31[9] = -(auVar34[9] == cVar46);
        auVar31[10] = -(auVar34[10] == cVar47);
        auVar31[0xb] = -(auVar34[0xb] == cVar48);
        auVar31[0xc] = -(auVar34[0xc] == cVar49);
        auVar31[0xd] = -(auVar34[0xd] == cVar50);
        auVar31[0xe] = -(auVar34[0xe] == cVar51);
        auVar31[0xf] = -(auVar34[0xf] == cVar52);
        if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar31 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar31 >> 0x1f,0) & 1) != 0) || (SUB161(auVar31 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar31 >> 0x37,0) & 1) != 0) || (SUB161(auVar31 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar31 >> 0x47,0) & 1) != 0) || (SUB161(auVar31 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar31 >> 0x57,0) & 1) != 0) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') break;
        uVar14 = uVar14 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
    }
    goto LAB_1413e83d3;
  }
  pauVar13 = (undefined1 (*) [16])plVar20[6];
  lVar12 = plVar20[9];
  auVar25 = *pauVar13;
  uVar9 = ~(uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
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
  pauVar19 = pauVar13 + 1;
LAB_1413e8633:
  plVar11 = local_110;
  if (lVar12 != 0) {
    if ((short)uVar9 == 0) {
      do {
        auVar25 = *pauVar19;
        uVar21 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
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
        pauVar13 = pauVar13 + -0x50;
        pauVar19 = pauVar19 + 1;
      } while (uVar21 == 0xffff);
      uVar10 = ~(uint)uVar21;
      uVar9 = uVar10 & -(uint)uVar21 - 2;
    }
    else {
      uVar10 = uVar9;
      uVar9 = uVar9 - 1 & uVar9;
    }
    lVar12 = lVar12 + -1;
    uVar5 = 0;
    for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
      uVar5 = uVar5 + 1;
    }
    local_e8 = *(int *)pauVar13[(ulonglong)uVar5 * -5 + -5];
    if (plVar3[9] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar15 = plVar3[6];
      bVar17 = (byte)(extraout_RAX_17 >> 0x39);
      auVar25 = ZEXT216(CONCAT11(bVar17,bVar17));
      auVar25 = pshuflw(auVar25,auVar25,0);
      lVar16 = 0;
      uVar14 = extraout_RAX_17;
      do {
        uVar14 = plVar3[7] & uVar14;
        auVar34 = *(undefined1 (*) [16])(lVar15 + uVar14);
        cVar24 = auVar25[0];
        auVar36[0] = -(auVar34[0] == cVar24);
        cVar26 = auVar25[1];
        auVar36[1] = -(auVar34[1] == cVar26);
        cVar27 = auVar25[2];
        auVar36[2] = -(auVar34[2] == cVar27);
        cVar28 = auVar25[3];
        auVar36[3] = -(auVar34[3] == cVar28);
        auVar36[4] = -(auVar34[4] == cVar24);
        auVar36[5] = -(auVar34[5] == cVar26);
        auVar36[6] = -(auVar34[6] == cVar27);
        auVar36[7] = -(auVar34[7] == cVar28);
        auVar36[8] = -(auVar34[8] == cVar24);
        auVar36[9] = -(auVar34[9] == cVar26);
        auVar36[10] = -(auVar34[10] == cVar27);
        auVar36[0xb] = -(auVar34[0xb] == cVar28);
        auVar36[0xc] = -(auVar34[0xc] == cVar24);
        auVar36[0xd] = -(auVar34[0xd] == cVar26);
        auVar36[0xe] = -(auVar34[0xe] == cVar27);
        auVar36[0xf] = -(auVar34[0xf] == cVar28);
        uVar21 = (ushort)(SUB161(auVar36 >> 7,0) & 1) | (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe | (ushort)(auVar36[0xf] >> 7) << 0xf
        ;
        uVar10 = (uint)uVar21;
        while (uVar21 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          uVar22 = uVar5 + uVar14 & plVar3[7];
          if (local_e8 == *(int *)(lVar15 + -0x18 + uVar22 * -0x18)) {
            if (*(char *)(lVar15 + -4 + uVar22 * -0x18) != '\x11') {
              rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
              if (extraout_RAX_16 == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
                core::option::expect_failed();
              }
              uVar14 = *extraout_RAX_16 & 0xffffffff;
              if (((*(ulonglong *)(local_98 + 0x10) <= uVar14) ||
                  (lVar15 = *(longlong *)(local_98 + 8), lVar15 == 0)) ||
                 (*(uint *)(lVar15 + 0x20 + uVar14 * 0x28) != ((uint)(*extraout_RAX_16 >> 0x20) | 1)
                 )) {
                    // WARNING: Subroutine does not return
                std::panicking::begin_panic<ref_<str_>_>();
              }
              auVar8 = *(undefined1 (*) [12])((longlong)extraout_RAX_16 + 0x44);
              auVar25 = *(undefined1 (*) [16])((longlong)extraout_RAX_16 + 0x34);
              plVar11 = (longlong *)(lVar15 + uVar14 * 0x28);
              pcVar4 = (code *)plVar11[3];
              LOCK();
              lVar15 = *(longlong *)pcVar4;
              *(longlong *)pcVar4 = *(longlong *)pcVar4 + 1;
              UNLOCK();
              if (*(longlong *)pcVar4 == 0 || SCARRY8(lVar15,1) != *(longlong *)pcVar4 < 0) {
LAB_1413e8850:
                    // WARNING: Does not return
                pcVar4 = (code *)invalidInstructionException();
                (*pcVar4)();
              }
              local_178 = (int *)plVar11[2];
              local_188 = *plVar11;
              lStack_180 = plVar11[1];
              local_198._0_12_ = auVar8;
              local_1a8 = auVar25;
              pcStack_170 = pcVar4;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              if ((char)local_238 != '\x11') {
                LOCK();
                *(longlong *)pcStack_220 = *(longlong *)pcStack_220 + -1;
                UNLOCK();
                if (*(longlong *)pcStack_220 == 0) {
                  alloc::sync::Arc::
                  drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                            ();
                }
              }
            }
            goto LAB_1413e8633;
          }
          uVar21 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar21);
        }
        auVar32[0] = -(auVar34[0] == cVar37);
        auVar32[1] = -(auVar34[1] == cVar38);
        auVar32[2] = -(auVar34[2] == cVar39);
        auVar32[3] = -(auVar34[3] == cVar40);
        auVar32[4] = -(auVar34[4] == cVar41);
        auVar32[5] = -(auVar34[5] == cVar42);
        auVar32[6] = -(auVar34[6] == cVar43);
        auVar32[7] = -(auVar34[7] == cVar44);
        auVar32[8] = -(auVar34[8] == cVar45);
        auVar32[9] = -(auVar34[9] == cVar46);
        auVar32[10] = -(auVar34[10] == cVar47);
        auVar32[0xb] = -(auVar34[0xb] == cVar48);
        auVar32[0xc] = -(auVar34[0xc] == cVar49);
        auVar32[0xd] = -(auVar34[0xd] == cVar50);
        auVar32[0xe] = -(auVar34[0xe] == cVar51);
        auVar32[0xf] = -(auVar34[0xf] == cVar52);
        if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 || (SUB161(auVar32 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar32 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar32 >> 0x1f,0) & 1) != 0) || (SUB161(auVar32 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar32 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar32 >> 0x37,0) & 1) != 0) || (SUB161(auVar32 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar32 >> 0x47,0) & 1) != 0) || (SUB161(auVar32 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar32 >> 0x57,0) & 1) != 0) || (SUB161(auVar32 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar32 >> 0x67,0) & 1) != 0) || (SUB161(auVar32 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar32 >> 0x77,0) & 1) != 0) || auVar32[0xf] < '\0') break;
        uVar14 = uVar14 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      } while( true );
    }
    goto LAB_1413e8633;
  }
  goto LAB_1413e7be0;
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'archive_storage_decorators<alloc::string::String>', 'address': '1413e6150'}, {'name': 'begin_panic<ref$<str$>_>', 'address': '1428d8725'}, {'name': 'call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>', 'address': '1400d9b20'}, {'name': 'clone', 'address': '140d7bd90'}, {'name': 'drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>', 'address': '140b5c620'}, {'name': 'expect_failed', 'address': '1428d9330'}, {'name': 'format_inner', 'address': '140d79f90'}, {'name': 'get_mut_internal', 'address': '140b93940'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}, {'name': 'hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex>_>', 'address': '1405b0ed0'}, {'name': 'insert<country_core::resources::walls::decorator::DecoratorId,tuple$<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>', 'address': '1411fcc70'}, {'name': 'memcpy', 'address': '142923c00'}, {'name': 'panic_access_error', 'address': '1428d80d0'}, {'name': 'reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,std::hash::random::RandomState>_>', 'address': '1428b36e0'}, {'name': 'tracked_clone_from<tuple$<>_>', 'address': '14155a4c0'}]