
void __cdecl
system_decorator::move_decorators_following_anchors::move_decorators_following_anchors(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined1 (*pauVar8) [16];
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  char extraout_AL;
  uint uVar17;
  uint extraout_EAX;
  uint extraout_EAX_00;
  undefined4 extraout_EAX_01;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar18;
  ulonglong uVar19;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  undefined4 *extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  uint uVar20;
  undefined8 *in_RCX;
  longlong *plVar21;
  undefined8 *in_RDX;
  undefined8 *puVar22;
  ulonglong uVar23;
  undefined1 *puVar24;
  undefined1 *puVar25;
  longlong lVar26;
  ushort uVar27;
  longlong *in_R8;
  char *pcVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined8 *in_R9;
  longlong lVar31;
  longlong lVar32;
  undefined1 (*pauVar33) [16];
  undefined1 (*pauVar34) [16];
  byte bVar35;
  char cVar36;
  char cVar41;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  float fVar37;
  float extraout_XMM0_Da_03;
  char cVar39;
  char cVar40;
  undefined1 auVar38 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  float fVar44;
  float fVar45;
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  undefined1 unaff_XMM8_Bc;
  undefined1 unaff_XMM8_Bd;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  undefined1 unaff_XMM8_Bg;
  undefined1 unaff_XMM8_Bh;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  undefined1 unaff_XMM8_Bk;
  undefined1 unaff_XMM8_Bl;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  undefined1 unaff_XMM8_Bo;
  undefined1 unaff_XMM8_Bp;
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  longlong *in_stack_00000040;
  longlong local_1e8;
  longlong *local_188;
  code *pcStack_180;
  longlong local_168 [2];
  undefined8 local_158;
  undefined1 (*local_150) [16];
  longlong **local_148;
  undefined8 local_140;
  undefined8 uStack_138;
  longlong local_130;
  longlong *local_128;
  ulonglong local_120;
  undefined8 local_118;
  ulonglong *local_110;
  undefined1 local_102 [17];
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar22 = in_RDX;
  local_110 = (ulonglong *)in_R8;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar22 != (undefined8 *)0x0) {
        if (puVar22 < (undefined8 *)0x56) {
          if (*(char *)(puVar22 + 0x285520bf) < -0x40) {
LAB_14081c864:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (puVar22 != (undefined8 *)0x56) goto LAB_14081c864;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar22 != (undefined8 *)0xfffffffffffffffe)) {
        if ((longlong)puVar22 + 2U < 0x56) {
          if (*(char *)((longlong)puVar22 + 0x142a905fa) < -0x40) {
LAB_14081c8b1:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((longlong)puVar22 + 2U != 0x56) goto LAB_14081c8b1;
      }
    }
    pcVar28 = "internal error: entered unreachable code";
    do {
      bVar35 = pcVar28[-1];
      uVar17 = (uint)(char)bVar35;
      if ((char)bVar35 < '\0') {
        bVar2 = pcVar28[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = pcVar28[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = (byte *)(pcVar28 + -4);
            pcVar28 = pcVar28 + -4;
            uVar17 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar28 = pcVar28 + -3;
            uVar17 = bVar3 & 0xf;
          }
          uVar17 = bVar2 & 0x3f | uVar17 << 6;
        }
        else {
          pcVar28 = pcVar28 + -2;
          uVar17 = bVar2 & 0x1f;
        }
        uVar17 = (uint)(bVar35 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_14081c974;
LAB_14081c985:
        if (pcVar28 == "") break;
        if (pcVar28 + -0x142a90737 < (char *)0x41) {
          if (-0x41 < pcVar28[1]) break;
        }
        else if (pcVar28 + -0x142a90737 == (char *)0x41) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      pcVar28 = pcVar28 + -1;
      if (uVar17 == 0x5c) goto LAB_14081c985;
LAB_14081c974:
      if (uVar17 == 0x2f) goto LAB_14081c985;
    } while (pcVar28 !=
             "crates/systems/decorator/src/move_decorators_following_anchors.rsinternal error: entered unreachable code"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar21 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_13, extraout_RAX_13 == (longlong *)0x0))
    goto LAB_14081d5bc;
    if (*plVar21 != 0) goto LAB_14081d602;
    *plVar21 = -1;
    local_102._2_8_ = plVar21;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_102._2_8_ = *(longlong *)local_102._2_8_ + 1;
  }
  puVar5 = (ulonglong *)*in_RDX;
  lVar26 = in_RDX[1];
  uVar30 = *puVar5;
  uVar23 = 0;
  uVar19 = uVar30 - *(ulonglong *)(lVar26 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x18)) {
    uVar19 = uVar23;
  }
  uVar29 = uVar30 - *(ulonglong *)(lVar26 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x38)) {
    uVar29 = uVar23;
  }
  lVar18 = uVar19 * 0x10 + *(longlong *)(lVar26 + 8);
  uVar30 = *(ulonglong *)(lVar26 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar26 + 0x10) < uVar19) {
    lVar18 = 8;
    uVar30 = uVar23;
  }
  lVar32 = uVar29 * 0x10 + *(longlong *)(lVar26 + 0x28);
  uVar19 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
    lVar32 = 8;
    uVar19 = uVar23;
  }
  *puVar5 = (*(longlong *)(lVar26 + 0x40) - uVar19) - uVar30;
  lVar26 = uVar30 * 0x10 + lVar18;
  lVar31 = uVar19 * 0x10 + lVar32;
  local_120 = CONCAT44(local_120._4_4_,*(undefined4 *)((longlong)in_stack_00000040 + 0x1c));
  local_102._2_8_ = *in_stack_00000040;
  local_118 = (undefined4 *)in_stack_00000040[2];
  local_f1 = extraout_AL;
  do {
    if ((lVar18 == 0) || (lVar18 == lVar26)) {
      if ((lVar32 == 0) || (lVar32 == lVar31)) break;
      lVar32 = lVar32 + 0x10;
      lVar18 = 0;
    }
    else {
      lVar18 = lVar18 + 0x10;
    }
    *puVar5 = *puVar5 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_removed();
  } while( true );
  country_core::resources::walls::decorator_subtype::impl$8::from();
  puVar5 = (ulonglong *)*local_110;
  lVar26 = local_110[1];
  uVar30 = *puVar5;
  uVar23 = 0;
  uVar19 = uVar30 - *(ulonglong *)(lVar26 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x18)) {
    uVar19 = uVar23;
  }
  uVar29 = uVar30 - *(ulonglong *)(lVar26 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x38)) {
    uVar29 = uVar23;
  }
  lVar18 = uVar19 * 0x10 + *(longlong *)(lVar26 + 8);
  uVar30 = *(ulonglong *)(lVar26 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar26 + 0x10) < uVar19) {
    lVar18 = 8;
    uVar30 = uVar23;
  }
  lVar32 = uVar29 * 0x10 + *(longlong *)(lVar26 + 0x28);
  uVar19 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
    lVar32 = 8;
    uVar19 = uVar23;
  }
  *puVar5 = (*(longlong *)(lVar26 + 0x40) - uVar19) - uVar30;
  lVar31 = uVar30 * 0x10 + lVar18;
  lVar26 = uVar19 * 0x10 + lVar32;
  local_128 = (longlong *)CONCAT44(local_128._4_4_,*(undefined4 *)(in_stack_00000030 + 0x1c));
  puVar6 = *(undefined4 **)(in_stack_00000030 + 0x10);
  uVar4 = *(undefined4 *)(in_stack_00000038 + 0x1c);
  puVar7 = *(undefined4 **)(in_stack_00000038 + 0x10);
  local_110 = puVar5;
  do {
    if ((lVar18 == 0) || (lVar18 == lVar31)) {
      if ((lVar32 == 0) || (lVar32 == lVar26)) break;
      lVar32 = lVar32 + 0x10;
      lVar18 = 0;
    }
    else {
      lVar18 = lVar18 + 0x10;
    }
    *local_110 = *local_110 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX_02 != 0) {
      country_core::resources::walls::decorator_storage::Decorators::mark_all_modified();
    }
    *puVar6 = local_128._0_4_;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    *puVar7 = uVar4;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
  } while( true );
  puVar5 = (ulonglong *)*in_R9;
  lVar26 = in_R9[1];
  uVar30 = *puVar5;
  uVar23 = 0;
  uVar19 = uVar30 - *(ulonglong *)(lVar26 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x18)) {
    uVar19 = uVar23;
  }
  uVar29 = uVar30 - *(ulonglong *)(lVar26 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x38)) {
    uVar29 = uVar23;
  }
  lVar18 = uVar19 * 0x20 + *(longlong *)(lVar26 + 8);
  uVar30 = *(ulonglong *)(lVar26 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar26 + 0x10) < uVar19) {
    lVar18 = 8;
    uVar30 = uVar23;
  }
  lVar32 = uVar29 * 0x20 + *(longlong *)(lVar26 + 0x28);
  uVar19 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
    lVar32 = 8;
    uVar19 = uVar23;
  }
  *puVar5 = (*(longlong *)(lVar26 + 0x40) - uVar19) - uVar30;
  local_110 = (ulonglong *)(uVar30 * 0x20 + lVar18);
  local_128 = (longlong *)(uVar19 * 0x20 + lVar32);
  do {
    if ((lVar18 == 0) || ((ulonglong *)lVar18 == local_110)) {
      if ((lVar32 == 0) || ((longlong *)lVar32 == local_128)) break;
      lVar32 = lVar32 + 0x20;
      lVar18 = 0;
    }
    else {
      lVar18 = lVar18 + 0x20;
    }
    *puVar5 = *puVar5 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX_03 != 0) {
      local_158 = *(undefined ***)(extraout_RAX_03 + 0x30);
      local_148 = (longlong **)
                  (*(undefined1 (*) [16])local_158 + *(longlong *)(extraout_RAX_03 + 0x38) + 1);
      auVar38 = *(undefined1 (*) [16])local_158;
      local_140 = CONCAT62(local_140._2_6_,
                           ~((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar38[0xf] >> 7) << 0xf));
      local_150 = (undefined1 (*) [16])((longlong)local_158 + 0x10);
      uStack_138 = *(ulonglong *)(extraout_RAX_03 + 0x48);
      local_130 = extraout_RAX_03 + 0x60;
      while (country_core::resources::walls::decorator_storage::impl$3::next(), local_1e8 != 0) {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (*(char *)(extraout_RAX_04 + 0x22) == '\x02') {
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
        }
      }
    }
  } while( true );
  puVar5 = (ulonglong *)*in_stack_00000028;
  lVar26 = in_stack_00000028[1];
  uVar30 = *puVar5;
  uVar23 = 0;
  uVar19 = uVar30 - *(ulonglong *)(lVar26 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x18)) {
    uVar19 = uVar23;
  }
  uVar29 = uVar30 - *(ulonglong *)(lVar26 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar26 + 0x38)) {
    uVar29 = uVar23;
  }
  lVar18 = uVar19 * 0x30 + *(longlong *)(lVar26 + 8);
  uVar30 = *(ulonglong *)(lVar26 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar26 + 0x10) < uVar19) {
    lVar18 = 8;
    uVar30 = uVar23;
  }
  lVar32 = uVar29 * 0x30 + *(longlong *)(lVar26 + 0x28);
  uVar19 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
    lVar32 = 8;
    uVar19 = uVar23;
  }
  *puVar5 = (*(longlong *)(lVar26 + 0x40) - uVar19) - uVar30;
  local_110 = (ulonglong *)(uVar30 * 0x30 + lVar18);
  local_128 = (longlong *)(uVar19 * 0x30 + lVar32);
  do {
    if ((lVar18 == 0) || ((ulonglong *)lVar18 == local_110)) {
      if ((lVar32 == 0) || ((longlong *)lVar32 == local_128)) goto LAB_14081cfdf;
      lVar32 = lVar32 + 0x30;
      lVar18 = 0;
    }
    else {
      lVar18 = lVar18 + 0x30;
    }
    *puVar5 = *puVar5 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX_05 != 0) {
      local_158 = *(undefined ***)(extraout_RAX_05 + 0x30);
      local_148 = (longlong **)
                  (*(undefined1 (*) [16])local_158 + *(longlong *)(extraout_RAX_05 + 0x38) + 1);
      auVar38 = *(undefined1 (*) [16])local_158;
      local_140 = CONCAT62(local_140._2_6_,
                           ~((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar38[0xf] >> 7) << 0xf));
      local_150 = (undefined1 (*) [16])((longlong)local_158 + 0x10);
      uStack_138 = *(ulonglong *)(extraout_RAX_05 + 0x48);
      local_130 = extraout_RAX_05 + 0x60;
      while (country_core::resources::walls::decorator_storage::impl$3::next(), local_1e8 != 0) {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (*(char *)(extraout_RAX_06 + 0x22) == '\x02') {
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
        }
      }
    }
  } while( true );
code_r0x00014081d17b:
  uVar27 = (ushort)(uVar17 - 1) & (ushort)uVar17;
  uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar27);
  if (uVar27 == 0) goto LAB_14081d190;
  goto LAB_14081d160;
LAB_14081cfdf:
  *local_118 = (undefined4)local_120;
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls_mut();
  fVar16 = __real_3d4ccccd;
  fVar15 = __real_3f800000;
  fVar14 = __real_3f000000;
  local_110 = (ulonglong *)*in_RCX;
  local_128 = (longlong *)(local_110 + 4);
  iVar10 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                            CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                  CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  uVar17 = (uint)(ushort)local_140;
  pauVar34 = (undefined1 (*) [16])local_158;
  pauVar33 = local_150;
  puVar6 = local_118;
  local_120 = uStack_138;
  while (local_120 != 0) {
    if ((short)uVar17 == 0) {
      do {
        auVar38 = *pauVar33;
        uVar27 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
        pauVar34 = pauVar34 + -0x98;
        pauVar33 = pauVar33 + 1;
      } while (uVar27 == 0xffff);
      uVar17 = ~(uint)uVar27;
      uVar20 = uVar17 & -(uint)uVar27 - 2;
    }
    else {
      if (pauVar34 == (undefined1 (*) [16])0x0) break;
      uVar20 = uVar17 - 1 & uVar17;
    }
    local_118 = (undefined4 *)CONCAT44((int)((ulonglong)puVar6 >> 0x20),uVar20);
    uVar20 = 0;
    for (; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
      uVar20 = uVar20 + 1;
    }
    local_168[0] = *(longlong *)((longlong)pauVar34 + (ulonglong)uVar20 * -0x98 + -0x98);
    country_core::resources::walls::decorator_storage::Decorators::iter_modified_mut();
    country_core::resources::walls::decorator_storage::impl$4::len();
    puVar5 = local_110;
    local_120 = local_120 - 1;
    if (extraout_RAX_07 != 0) {
      if (local_110[3] == 0) {
LAB_14081d531:
        local_188 = local_168;
        pcStack_180 = country_core::resources::walls::impl$7::fmt;
        local_158 = &PTR_s_The_list_of_decorators_contains_a_142a90808;
        local_150 = (undefined1 (*) [16])0x1;
        uStack_138 = 0;
        local_140 = 1;
        local_148 = &local_188;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar26 = *puVar5;
      puVar24 = (undefined1 *)puVar5[1];
      bVar35 = (byte)(extraout_RAX_08 >> 0x39);
      auVar38 = ZEXT216(CONCAT11(bVar35,bVar35));
      auVar38 = pshuflw(auVar38,auVar38,0);
      lVar18 = 0;
      uVar30 = extraout_RAX_08;
      do {
        uVar30 = (ulonglong)puVar24 & uVar30;
        pcVar28 = (char *)(lVar26 + uVar30);
        cVar36 = auVar38[0];
        auVar43[0] = -(*pcVar28 == cVar36);
        cVar39 = auVar38[1];
        auVar43[1] = -(pcVar28[1] == cVar39);
        cVar40 = auVar38[2];
        auVar43[2] = -(pcVar28[2] == cVar40);
        cVar41 = auVar38[3];
        auVar43[3] = -(pcVar28[3] == cVar41);
        auVar43[4] = -(pcVar28[4] == cVar36);
        auVar43[5] = -(pcVar28[5] == cVar39);
        auVar43[6] = -(pcVar28[6] == cVar40);
        auVar43[7] = -(pcVar28[7] == cVar41);
        auVar43[8] = -(pcVar28[8] == cVar36);
        auVar43[9] = -(pcVar28[9] == cVar39);
        auVar43[10] = -(pcVar28[10] == cVar40);
        auVar43[0xb] = -(pcVar28[0xb] == cVar41);
        auVar43[0xc] = -(pcVar28[0xc] == cVar36);
        auVar43[0xd] = -(pcVar28[0xd] == cVar39);
        auVar43[0xe] = -(pcVar28[0xe] == cVar40);
        auVar43[0xf] = -(pcVar28[0xf] == cVar41);
        uVar27 = (ushort)(SUB161(auVar43 >> 7,0) & 1) | (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar43 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar43 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar43 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar43 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar43 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar43 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar43 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar43 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar43 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar43 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar43 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe | (ushort)(auVar43[0xf] >> 7) << 0xf
        ;
        uVar17 = (uint)uVar27;
        if (uVar27 != 0) {
LAB_14081d160:
          uVar20 = 0;
          for (uVar9 = uVar17; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar20 = uVar20 + 1;
          }
          lVar32 = (uVar20 + uVar30 & (ulonglong)puVar24) * -0x2a8;
          if (local_168[0] != *(longlong *)(lVar26 + -0x2a8 + lVar32)) goto code_r0x00014081d17b;
          lVar18 = lVar26 + lVar32;
          local_102._2_8_ = lVar26 + lVar32 + -0x2a0;
          while (country_core::resources::walls::decorator_storage::impl$5::next(),
                puVar24 != (undefined1 *)0x0) {
            puVar25 = (undefined1 *)(ulonglong)extraout_EAX;
            lVar26 = (ulonglong)(byte)puVar24[0x22] - 1;
            if (2 < (byte)(puVar24[0x22] - 2)) {
              lVar26 = 0;
            }
            if (lVar26 == 0) {
              country_core::resources::walls::decorator::DecoratorDst::as_wall_mut();
              country_core::resources::walls::public_wall_state::PublicWallState::
              get_wall_attachment_height();
              ::utils::curve::Curve::
              get_approx_coord_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
              puVar24 = *(undefined1 **)(lVar18 + -0x278);
              if (puVar24 <= (undefined1 *)(ulonglong)extraout_EAX_00) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              if (puVar24 <= (undefined1 *)(ulonglong)extraout_EAX_00 + 1) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              if ((extraout_XMM0_Da_00 < 0.0) || (fVar15 < extraout_XMM0_Da_00)) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic();
              }
              ::utils::lerp();
              fVar44 = (float)extraout_RAX_09[4];
              country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
              local_158 = (undefined **)CONCAT44(extraout_XMM0_Da_01,extraout_EAX_01);
              ::utils::wall_space::impl$1::project_clamp();
              *extraout_RAX_09 = extraout_XMM0_Da_02;
              extraout_RAX_09[1] = extraout_XMM0_Da + fVar44;
              if ((*(char *)((longlong)extraout_RAX_09 + 0x21) == '\x01') &&
                 (country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::
                  try_get_subtype(), puVar24 = puVar25, extraout_RAX_10 != 0)) {
                pauVar8 = *(undefined1 (**) [16])(lVar18 + -600);
                if (pauVar8 == (undefined1 (*) [16])0x1) {
                    /* WARNING: Subroutine does not return */
                  core::option::unwrap_failed();
                }
                if (pauVar8 == (undefined1 (*) [16])0x0) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic();
                }
                local_158 = *(undefined ***)(lVar18 + -0x260);
                local_148 = (longlong **)
                            core::ops::function::FnOnce::
                            call_once<utils::curve::curve_slice::impl_5::remap_2d::closure_env_0<slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>,tuple_<usize,ref_<glam::f32::vec3::Vec3>_>_>
                ;
                local_140 = 0;
                uStack_138 = 0;
                local_150 = pauVar8;
                ::utils::curve::curve_slice::CurveSlice::
                extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
                          ();
                uVar30 = uStack_138 & 0xffffffff;
                if (*(ulonglong *)(lVar18 + -600) <= uVar30) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(lVar18 + -600) <= uVar30 + 1) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                fVar45 = uStack_138._4_4_;
                fVar44 = *(float *)(*(longlong *)(lVar18 + -0x260) + 4 + uVar30 * 0xc);
                fVar37 = *(float *)(*(longlong *)(lVar18 + -0x260) + 4 + (uVar30 + 1) * 0xc);
                country_core::resources::walls::decorator::DecoratorRank::new();
                country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
                puVar24 = local_102;
                std::collections::hash::map::HashMap::
                get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                          ();
                if (extraout_RAX_11 == 0) {
                  local_188 = (longlong *)local_102;
                  pcStack_180 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
                  local_158 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
                  local_150 = (undefined1 (*) [16])0x1;
                  uStack_138 = 0;
                  local_140 = 1;
                  local_148 = &local_188;
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_fmt();
                }
                fVar44 = fVar45 * (fVar37 - fVar44) + fVar44;
                if (fVar44 <= 0.0) {
                  fVar44 = 0.0;
                }
                fVar37 = (*(float *)(extraout_RAX_11 + 0x3c) - *(float *)(extraout_RAX_11 + 0x38)) *
                         fVar14;
                fVar45 = fVar44 + fVar37 + fVar16;
                fVar44 = fVar37;
                if (fVar37 <= fVar45) {
                  fVar44 = fVar45;
                }
                fVar44 = (float)(-(uint)NAN(fVar45) & (uint)fVar37 |
                                ~-(uint)NAN(fVar45) & (uint)fVar44);
                extraout_RAX_09[6] = fVar44;
                country_core::resources::walls::public_wall_state::PublicWallState::max_y();
                extraout_RAX_09[4] = fVar44 - extraout_XMM0_Da_03;
                extraout_RAX_09[1] = fVar44;
              }
            }
            else if (lVar26 != 1) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic();
            }
          }
          break;
        }
LAB_14081d190:
        auVar42[0] = -(*pcVar28 == (char)iVar10);
        auVar42[1] = -(pcVar28[1] == (char)((uint)iVar10 >> 8));
        auVar42[2] = -(pcVar28[2] == (char)((uint)iVar10 >> 0x10));
        auVar42[3] = -(pcVar28[3] == (char)((uint)iVar10 >> 0x18));
        auVar42[4] = -(pcVar28[4] == (char)iVar11);
        auVar42[5] = -(pcVar28[5] == (char)((uint)iVar11 >> 8));
        auVar42[6] = -(pcVar28[6] == (char)((uint)iVar11 >> 0x10));
        auVar42[7] = -(pcVar28[7] == (char)((uint)iVar11 >> 0x18));
        auVar42[8] = -(pcVar28[8] == (char)iVar12);
        auVar42[9] = -(pcVar28[9] == (char)((uint)iVar12 >> 8));
        auVar42[10] = -(pcVar28[10] == (char)((uint)iVar12 >> 0x10));
        auVar42[0xb] = -(pcVar28[0xb] == (char)((uint)iVar12 >> 0x18));
        auVar42[0xc] = -(pcVar28[0xc] == (char)iVar13);
        auVar42[0xd] = -(pcVar28[0xd] == (char)((uint)iVar13 >> 8));
        auVar42[0xe] = -(pcVar28[0xe] == (char)((uint)iVar13 >> 0x10));
        auVar42[0xf] = -(pcVar28[0xf] == (char)((uint)iVar13 >> 0x18));
        if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar42 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar42 >> 0x1f,0) & 1) != 0) || (SUB161(auVar42 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar42 >> 0x37,0) & 1) != 0) || (SUB161(auVar42 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar42 >> 0x47,0) & 1) != 0) || (SUB161(auVar42 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar42 >> 0x57,0) & 1) != 0) || (SUB161(auVar42 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar42 >> 0x67,0) & 1) != 0) || (SUB161(auVar42 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar42 >> 0x77,0) & 1) != 0) || auVar42[0xf] < '\0') goto LAB_14081d531;
        uVar30 = uVar30 + lVar18 + 0x10;
        lVar18 = lVar18 + 0x10;
      } while( true );
    }
    uVar17 = (uint)local_118;
    puVar6 = local_118;
  }
  if (local_f1 != '\0') {
    local_118 = puVar6;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_12 == 1) {
      plVar21 = extraout_RAX_12 + 1;
    }
    else if ((*extraout_RAX_12 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0)) {
LAB_14081d5bc:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
LAB_14081d602:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_102._2_8_ = plVar21;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_102._2_8_ = *(longlong *)local_102._2_8_ + 1;
  }
  return;
}

