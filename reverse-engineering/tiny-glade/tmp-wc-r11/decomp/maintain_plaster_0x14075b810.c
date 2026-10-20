// name: maintain_plaster
// addr: 0x14075b810
// size: 5273
// signature: void __cdecl maintain_plaster(void)


// WARNING: Type propagation algorithm not settling

void __cdecl system_wall_constructor::plaster_systems::maintain_plaster(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  float *pfVar3;
  char *pcVar4;
  int *piVar5;
  byte bVar6;
  byte bVar7;
  ulonglong ******ppppppuVar8;
  undefined4 *puVar9;
  undefined8 uVar10;
  longlong *plVar11;
  short sVar12;
  uint uVar13;
  uint uVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  ushort uVar19;
  uint uVar20;
  undefined4 uVar21;
  uint uVar22;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar23;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong *******pppppppuVar24;
  ulonglong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  ulonglong extraout_RAX_09;
  longlong in_RCX;
  ulonglong uVar25;
  longlong lVar26;
  ulonglong uVar27;
  longlong lVar28;
  ulonglong *******pppppppuVar29;
  ulonglong *******pppppppuVar30;
  undefined1 uVar31;
  ulonglong *******pppppppuVar32;
  ulonglong *******pppppppuVar33;
  longlong *in_R8;
  undefined *puVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  undefined1 (*pauVar37) [16];
  undefined8 *in_R9;
  longlong lVar38;
  longlong lVar39;
  ushort uVar40;
  undefined1 (*pauVar41) [16];
  longlong *plVar42;
  longlong *plVar43;
  undefined1 (*pauVar44) [16];
  bool bVar45;
  char cVar46;
  byte bVar47;
  char cVar50;
  char cVar51;
  char cVar52;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 uVar48;
  undefined1 auVar49 [16];
  undefined4 uVar53;
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  undefined1 auVar60 [16];
  float fVar61;
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  undefined1 unaff_XMM9_Bc;
  undefined1 unaff_XMM9_Bd;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  undefined1 unaff_XMM9_Bg;
  undefined1 unaff_XMM9_Bh;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  undefined1 unaff_XMM9_Bk;
  undefined1 unaff_XMM9_Bl;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  undefined1 unaff_XMM9_Bo;
  undefined1 unaff_XMM9_Bp;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  ulonglong *******local_1c8;
  ulonglong *******pppppppuStack_1c0;
  ulonglong *******local_1b8;
  ulonglong *******pppppppuStack_1b0;
  ulonglong *******local_1a8;
  undefined8 local_1a0;
  longlong local_198;
  longlong local_190;
  ulonglong local_188;
  longlong *local_180;
  ulonglong local_178;
  longlong local_170;
  longlong *local_168;
  undefined8 *local_160;
  longlong *local_150;
  undefined8 local_148;
  ulonglong *******pppppppuStack_140;
  ulonglong *******local_138;
  ulonglong *******pppppppuStack_130;
  ulonglong *******local_128;
  ulonglong *******local_120;
  ulonglong *******local_118;
  longlong local_110;
  undefined8 *local_108;
  longlong local_100;
  undefined8 *local_f8;
  ulonglong *local_e8;
  longlong *local_e0;
  longlong *local_d8;
  undefined8 local_d0;
  char local_c2;
  undefined1 local_c1;
  undefined8 local_c0;
  
  local_c0 = 0xfffffffffffffffe;
  local_168 = in_R8;
  local_160 = in_R9;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar25 = 0x3a;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar25 != 0) {
        if (uVar25 < 0x3a) {
          if ((char)(&DAT_142a7ced3)[uVar25] < -0x40) {
LAB_14075b8ce:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 != 0x3a) goto LAB_14075b8ce;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar25 != 0xfffffffffffffffe)) {
        if (uVar25 + 2 < 0x3a) {
          if ((char)(&UNK_142a7ced5)[uVar25] < -0x40) {
LAB_14075b925:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 + 2 != 0x3a) goto LAB_14075b925;
      }
    }
    puVar34 = &DAT_142a7d0ee;
    do {
      bVar47 = puVar34[-1];
      uVar20 = (uint)(char)bVar47;
      if ((char)bVar47 < '\0') {
        bVar6 = puVar34[-2];
        if ((char)bVar6 < -0x40) {
          bVar7 = puVar34[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = puVar34 + -4;
            puVar34 = puVar34 + -4;
            uVar20 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar34 = puVar34 + -3;
            uVar20 = bVar7 & 0xf;
          }
          uVar20 = bVar6 & 0x3f | uVar20 << 6;
        }
        else {
          puVar34 = puVar34 + -2;
          uVar20 = bVar6 & 0x1f;
        }
        uVar20 = (uint)(bVar47 & 0x3f) | uVar20 << 6;
        if (uVar20 != 0x5c) goto LAB_14075b9e4;
LAB_14075b9f5:
        if (puVar34 == &UNK_142a7d0b7) break;
        if (puVar34 + -0x142a7d0b7 < (undefined *)0x36) {
          if (-0x41 < (char)puVar34[1]) break;
        }
        else if (puVar34 + -0x142a7d0b7 == (undefined *)0x36) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar34 = puVar34 + -1;
      if (uVar20 == 0x5c) goto LAB_14075b9f5;
LAB_14075b9e4:
      if (uVar20 == 0x2f) goto LAB_14075b9f5;
    } while (puVar34 != &DAT_142a7d0b8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar42 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar42 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_14075cbf7:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar42 != 0) {
LAB_14075ccd1:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar42 = -1;
    local_d0 = plVar42;
    puffin::ThreadProfiler::begin_scope();
    *local_d0 = *local_d0 + 1;
  }
  LOCK();
  bVar45 = BUFFER_DEFERRED_DISPOSAL == '\0';
  if (bVar45) {
    BUFFER_DEFERRED_DISPOSAL = '\x01';
  }
  UNLOCK();
  local_c2 = extraout_AL;
  if (!bVar45) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar34 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    uVar25 = 0;
    uVar21 = 0;
    lVar38 = DAT_14362bfd0;
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    uVar25 = extraout_RAX_09 ^ 1;
    uVar21 = (undefined4)uVar25;
    lVar38 = DAT_14362bfd0;
  }
  DAT_14362bfd0 = lVar38;
  if (DAT_14362bfc1 != '\0') {
    local_148 = (ulonglong *******)&BUFFER_DEFERRED_DISPOSAL;
    uVar36 = (ulonglong)pppppppuStack_140 >> 8;
    pppppppuStack_140 = (ulonglong *******)CONCAT71((int7)uVar36,(char)uVar25);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  uVar25 = (ulonglong)local_d0 >> 0x20;
  local_d0 = (longlong *)CONCAT44((int)uVar25,uVar21);
  DAT_14362bfc1 = '\0';
  if (DAT_14362bfd8 != 0) {
    lVar23 = DAT_14362bfd8 * 0x30;
    lVar26 = 0;
    do {
      piVar5 = (int *)(lVar38 + 0x28 + lVar26);
      *piVar5 = *piVar5 + -1;
      lVar26 = lVar26 + 0x30;
    } while (lVar23 != lVar26);
  }
  alloc::vec::Vec::
  retain<tuple_<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global,system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env_0>
            ();
  if ((((char)local_d0 == '\0') && ((*(ulonglong *)puVar34 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0')) {
    DAT_14362bfc1 = '\x01';
  }
  cVar46 = BUFFER_DEFERRED_DISPOSAL;
  LOCK();
  BUFFER_DEFERRED_DISPOSAL = '\0';
  UNLOCK();
  if (cVar46 == '\x02') {
    std::sys::sync::mutex::futex::Mutex::wake();
  }
  local_e8 = (ulonglong *)*in_stack_00000040;
  lVar38 = in_stack_00000040[1];
  uVar25 = *local_e8;
  uVar27 = 0;
  uVar36 = uVar25 - *(ulonglong *)(lVar38 + 0x18);
  if (uVar25 < *(ulonglong *)(lVar38 + 0x18)) {
    uVar36 = uVar27;
  }
  uVar35 = uVar25 - *(ulonglong *)(lVar38 + 0x38);
  if (uVar25 < *(ulonglong *)(lVar38 + 0x38)) {
    uVar35 = uVar27;
  }
  plVar42 = (longlong *)(uVar36 * 0x10 + *(longlong *)(lVar38 + 8));
  uVar25 = *(ulonglong *)(lVar38 + 0x10) - uVar36;
  if (*(ulonglong *)(lVar38 + 0x10) < uVar36) {
    plVar42 = (longlong *)&DAT_00000008;
    uVar25 = uVar27;
  }
  lVar23 = uVar35 * 0x10 + *(longlong *)(lVar38 + 0x28);
  uVar36 = *(ulonglong *)(lVar38 + 0x30) - uVar35;
  if (*(ulonglong *)(lVar38 + 0x30) < uVar35) {
    lVar23 = 8;
    uVar36 = uVar27;
  }
  *local_e8 = (*(longlong *)(lVar38 + 0x40) - uVar36) - uVar25;
  local_e0 = plVar42 + uVar25 * 2;
  local_150 = (longlong *)(uVar36 * 0x10 + lVar23);
LAB_14075bc0e:
  if ((plVar42 == (longlong *)0x0) || (plVar42 == local_e0)) {
    if ((lVar23 == 0) || ((longlong *)lVar23 == local_150)) goto LAB_14075bf17;
    local_d0 = (longlong *)(lVar23 + 0x10);
    local_d8 = (longlong *)0x0;
  }
  else {
    local_d8 = plVar42 + 2;
    local_d0 = (longlong *)lVar23;
  }
  *local_e8 = *local_e8 + 1;
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>_>
            ();
  pppppppuVar32 = pppppppuStack_140;
  if (!SBORROW8(0,(longlong)pppppppuStack_140)) {
    local_1c8 = local_138;
    pppppppuStack_1c0 = pppppppuStack_130;
  }
  local_148 = pppppppuStack_140;
  pppppppuStack_140 = local_1c8;
  local_138 = pppppppuStack_1c0;
  pppppppuStack_130 = (ulonglong *******)0x0;
  local_110 = 0;
  pppppppuVar29 = (ulonglong *******)0x0;
  do {
    pppppppuVar33 = (ulonglong *******)0x8000000000000001;
    pppppppuVar24 = pppppppuVar32;
    if (pppppppuVar32 != (ulonglong *******)0x8000000000000001) goto LAB_14075bda5;
LAB_14075bdb7:
    if (local_110 == 0) {
LAB_14075beb6:
      if ((-0x7fffffffffffffff < (longlong)local_148) && (local_148 != (ulonglong *******)0x0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar23 = (longlong)local_d0;
      plVar42 = local_d8;
      if ((pppppppuVar29 != (ulonglong *******)0x0) && (local_120 != (ulonglong *******)0x0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        lVar23 = (longlong)local_d0;
        plVar42 = local_d8;
      }
      goto LAB_14075bc0e;
    }
    if (local_108 == local_f8) {
LAB_14075be90:
      if (local_100 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_110 = 0;
      goto LAB_14075beb6;
    }
    puVar2 = local_108 + 1;
    pppppppuVar24 = (ulonglong *******)*local_108;
    local_108 = puVar2;
    if ((ulonglong)pppppppuVar24 >> 0x20 == 0) goto LAB_14075be90;
    pppppppuVar30 = (ulonglong *******)0x0;
    pppppppuVar32 = pppppppuVar33;
LAB_14075be43:
    local_1c8 = (ulonglong *******)*in_stack_00000030;
    pppppppuStack_1c0 = (ulonglong *******)in_stack_00000030[1];
    local_1b8 = (ulonglong *******)in_stack_00000030[2];
    pppppppuStack_1b0 = (ulonglong *******)in_stack_00000030[3];
    local_1a8 = pppppppuVar24;
    bevy_ecs::system::commands::EntityCommands::despawn();
    if (pppppppuVar30 != (ulonglong *******)0x0) {
      do {
        if (local_128 != local_118) {
          pppppppuVar33 = local_128 + 1;
          pppppppuVar24 = (ulonglong *******)*local_128;
          local_128 = pppppppuVar33;
          if ((ulonglong)pppppppuVar24 >> 0x20 != 0) goto LAB_14075be43;
        }
        if (local_120 != (ulonglong *******)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        pppppppuStack_130 = (ulonglong *******)0x0;
        pppppppuVar29 = (ulonglong *******)0x0;
        pppppppuVar33 = (ulonglong *******)0x8000000000000001;
        pppppppuVar24 = local_148;
        if (local_148 == (ulonglong *******)0x8000000000000001) goto LAB_14075bdb7;
LAB_14075bda5:
        pppppppuVar32 = local_138;
        pppppppuVar30 = pppppppuStack_140;
        local_148 = (ulonglong *******)&DAT_8000000000000000;
        pppppppuVar33 = (ulonglong *******)&DAT_8000000000000000;
        if (SBORROW8(0,(longlong)pppppppuVar24)) goto LAB_14075bdb7;
        if ((pppppppuVar29 != (ulonglong *******)0x0) && (local_120 != (ulonglong *******)0x0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_118 = pppppppuVar30 + (longlong)pppppppuVar32;
        pppppppuStack_130 = pppppppuVar30;
        local_128 = pppppppuVar30;
        pppppppuVar32 = (ulonglong *******)&DAT_8000000000000000;
        pppppppuVar29 = pppppppuVar30;
        local_120 = pppppppuVar24;
        if (pppppppuVar30 == (ulonglong *******)0x0) break;
      } while( true );
    }
  } while( true );
LAB_14075bf17:
  local_148 = (ulonglong *******)*in_stack_00000038;
  lVar38 = in_stack_00000038[1];
  ppppppuVar8 = *local_148;
  uVar36 = 0;
  uVar25 = (longlong)ppppppuVar8 - *(ulonglong *)(lVar38 + 0x18);
  if (ppppppuVar8 < *(ulonglong *******)(lVar38 + 0x18)) {
    uVar25 = uVar36;
  }
  uVar27 = (longlong)ppppppuVar8 - *(ulonglong *)(lVar38 + 0x38);
  if (ppppppuVar8 < *(ulonglong *******)(lVar38 + 0x38)) {
    uVar27 = uVar36;
  }
  pppppppuStack_140 = (ulonglong *******)(uVar25 * 0x10 + *(longlong *)(lVar38 + 8));
  uVar35 = *(ulonglong *)(lVar38 + 0x10) - uVar25;
  if (*(ulonglong *)(lVar38 + 0x10) < uVar25) {
    pppppppuStack_140 = (ulonglong *******)0x8;
    uVar35 = uVar36;
  }
  pppppppuStack_130 = (ulonglong *******)(uVar27 * 0x10 + *(longlong *)(lVar38 + 0x28));
  uVar25 = *(ulonglong *)(lVar38 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar38 + 0x30) < uVar27) {
    pppppppuStack_130 = (ulonglong *******)0x8;
    uVar25 = uVar36;
  }
  local_120 = (ulonglong *******)(uVar25 + uVar35);
  *local_148 = (ulonglong ******)(*(longlong *)(lVar38 + 0x40) - (longlong)local_120);
  local_138 = (ulonglong *******)(uVar35 * 0x10 + (longlong)pppppppuStack_140);
  local_128 = (ulonglong *******)(uVar25 * 0x10 + (longlong)pppppppuStack_130);
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::walls::WallId,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_lintels::construct_lintels::closure_env_0>_>
            ();
  pauVar41 = *(undefined1 (**) [16])*local_168;
  local_d0 = (longlong *)((undefined8 *)*local_168)[3];
  auVar49 = *pauVar41;
  uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
  pauVar37 = pauVar41 + 1;
  local_d8 = (longlong *)*in_stack_00000050;
  local_e8 = (ulonglong *)in_stack_00000050[2];
  local_e0 = (longlong *)
             CONCAT44(local_e0._4_4_,*(undefined4 *)((longlong)in_stack_00000050 + 0x1c));
  uVar21 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  lVar38 = *in_stack_00000048;
  puVar9 = (undefined4 *)in_stack_00000048[2];
  iVar15 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                            CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                  CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                            CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                  CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                            CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                  CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  iVar18 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                            CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                  CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
LAB_14075c305:
  plVar42 = local_d8;
  if (local_d0 != (longlong *)0x0) {
    if ((short)uVar20 == 0) {
      do {
        auVar49 = *pauVar37;
        uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
        pauVar41 = pauVar41 + -0x2a8;
        pauVar37 = pauVar37 + 1;
      } while (uVar19 == 0xffff);
      uVar22 = ~(uint)uVar19;
      uVar20 = uVar22 & -(uint)uVar19 - 2;
    }
    else {
      uVar22 = uVar20;
      uVar20 = uVar20 - 1 & uVar20;
    }
    local_d0 = (longlong *)((longlong)local_d0 + -1);
    uVar13 = 0;
    for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
      uVar13 = uVar13 + 1;
    }
    lVar23 = (ulonglong)uVar13 * -0x2a8;
    if (local_d8[3] != 0) {
      plVar43 = (longlong *)((longlong)pauVar41 + lVar23 + -0x2a8);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar26 = *plVar42;
      uVar25 = plVar42[1];
      bVar47 = (byte)(extraout_RAX_03 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
      auVar49 = pshuflw(auVar49,auVar49,0);
      local_170 = 0;
      uVar36 = extraout_RAX_03;
      while( true ) {
        uVar36 = uVar25 & uVar36;
        pcVar4 = (char *)(lVar26 + uVar36);
        cVar46 = auVar49[0];
        auVar58[0] = -(*pcVar4 == cVar46);
        cVar50 = auVar49[1];
        auVar58[1] = -(pcVar4[1] == cVar50);
        cVar51 = auVar49[2];
        auVar58[2] = -(pcVar4[2] == cVar51);
        cVar52 = auVar49[3];
        auVar58[3] = -(pcVar4[3] == cVar52);
        auVar58[4] = -(pcVar4[4] == cVar46);
        auVar58[5] = -(pcVar4[5] == cVar50);
        auVar58[6] = -(pcVar4[6] == cVar51);
        auVar58[7] = -(pcVar4[7] == cVar52);
        auVar58[8] = -(pcVar4[8] == cVar46);
        auVar58[9] = -(pcVar4[9] == cVar50);
        auVar58[10] = -(pcVar4[10] == cVar51);
        auVar58[0xb] = -(pcVar4[0xb] == cVar52);
        auVar58[0xc] = -(pcVar4[0xc] == cVar46);
        auVar58[0xd] = -(pcVar4[0xd] == cVar50);
        auVar58[0xe] = -(pcVar4[0xe] == cVar51);
        auVar58[0xf] = -(pcVar4[0xf] == cVar52);
        uVar19 = (ushort)(SUB161(auVar58 >> 7,0) & 1) | (ushort)(SUB161(auVar58 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar58 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar58 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar58 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar58 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar58 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar58 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar58 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar58 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar58 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar58 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar58 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar58 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar58 >> 0x77,0) & 1) << 0xe | (ushort)(auVar58[0xf] >> 7) << 0xf
        ;
        uVar22 = (uint)uVar19;
        if (uVar19 != 0) {
          local_150 = (longlong *)CONCAT44(local_150._4_4_,uVar20);
          do {
            uVar13 = 0;
            for (uVar14 = uVar22; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
              uVar13 = uVar13 + 1;
            }
            if (*plVar43 == *(longlong *)(lVar26 + -0x10 + (uVar13 + uVar36 & uVar25) * -0x10)) {
              uVar53 = *(undefined4 *)((longlong)pauVar41 + lVar23 + -0x10);
              local_c1 = 1;
              ::utils::time_continuous_lerp();
              uVar20 = (uint)local_150;
              fVar61 = *(float *)((longlong)pauVar41 + lVar23 + -0x10);
              if (0.01 <= ABS(extraout_XMM0_Da - fVar61)) {
                local_c1 = 1;
                country_core::resources::walls::public_wall_state::PublicWallState::
                try_get_wall_shape();
                if (extraout_RAX_02 == 0) {
                  uVar48 = 0;
                  uVar53 = 0;
                }
                else {
                  local_c1 = 1;
                  country_core::systems::wall::valid_enclosure::ShapeParameters::center();
                  uVar48 = extraout_XMM0_Da_00;
                }
                local_1a0 = CONCAT44(uVar53,uVar48);
                local_c1 = 1;
                country_core::resources::walls::public_wall_state::PublicWallState::max_y();
                local_c1 = 1;
                ::utils::swizzlers::impl$1::xvy();
                lVar26 = *plVar43;
                pppppppuStack_1c0 =
                     (ulonglong *******)CONCAT44(pppppppuStack_1c0._4_4_,pppppppuStack_140._0_4_);
                local_1c8 = local_148;
                *puVar9 = uVar21;
                lVar28 = *(longlong *)(lVar38 + 0x30);
                uVar10 = *(undefined8 *)(lVar38 + 0x40);
                if (lVar28 == *(longlong *)(lVar38 + 0x20)) {
                  local_c1 = 1;
                  local_170 = lVar26;
                  alloc::raw_vec::RawVec::
                  grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                            ();
                  lVar26 = local_170;
                }
                lVar39 = *(longlong *)(lVar38 + 0x28);
                *(undefined8 *)(lVar39 + lVar28 * 0x28) = uVar10;
                *(longlong *)(lVar39 + 8 + lVar28 * 0x28) = lVar26;
                *(ulonglong ********)(lVar39 + 0x10 + lVar28 * 0x28) = local_1c8;
                *(undefined4 *)(lVar39 + 0x18 + lVar28 * 0x28) = pppppppuStack_1c0._0_4_;
                *(undefined4 *)(lVar39 + 0x1c + lVar28 * 0x28) = 0x3f800000;
                *(undefined1 *)(lVar39 + 0x20 + lVar28 * 0x28) = 9;
                uVar53 = (undefined4)local_148;
                *(undefined4 *)(lVar39 + 0x24 + lVar28 * 0x28) = local_148._3_4_;
                *(undefined4 *)(lVar39 + 0x21 + lVar28 * 0x28) = uVar53;
                *(longlong *)(lVar38 + 0x30) = lVar28 + 1;
                *(longlong *)(lVar38 + 0x40) = *(longlong *)(lVar38 + 0x40) + 1;
                fVar61 = extraout_XMM0_Da;
              }
              *(undefined4 *)local_e8 = local_e0._0_4_;
              local_c1 = 1;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              uVar25 = local_178;
              pfVar3 = (float *)((longlong)pauVar41 + lVar23 + -0x10);
              if ((fVar61 != *pfVar3) || (NAN(fVar61) || NAN(*pfVar3))) {
                lVar23 = *plVar43;
                if (local_178 == local_188) {
                  local_c1 = 1;
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                            ();
                }
                local_180[uVar25] = lVar23;
                local_178 = uVar25 + 1;
              }
              goto LAB_14075c305;
            }
            uVar19 = (ushort)(uVar22 - 1) & (ushort)uVar22;
            uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar19);
          } while (uVar19 != 0);
        }
        auVar54[0] = -(*pcVar4 == (char)iVar15);
        auVar54[1] = -(pcVar4[1] == (char)((uint)iVar15 >> 8));
        auVar54[2] = -(pcVar4[2] == (char)((uint)iVar15 >> 0x10));
        auVar54[3] = -(pcVar4[3] == (char)((uint)iVar15 >> 0x18));
        auVar54[4] = -(pcVar4[4] == (char)iVar16);
        auVar54[5] = -(pcVar4[5] == (char)((uint)iVar16 >> 8));
        auVar54[6] = -(pcVar4[6] == (char)((uint)iVar16 >> 0x10));
        auVar54[7] = -(pcVar4[7] == (char)((uint)iVar16 >> 0x18));
        auVar54[8] = -(pcVar4[8] == (char)iVar17);
        auVar54[9] = -(pcVar4[9] == (char)((uint)iVar17 >> 8));
        auVar54[10] = -(pcVar4[10] == (char)((uint)iVar17 >> 0x10));
        auVar54[0xb] = -(pcVar4[0xb] == (char)((uint)iVar17 >> 0x18));
        auVar54[0xc] = -(pcVar4[0xc] == (char)iVar18);
        auVar54[0xd] = -(pcVar4[0xd] == (char)((uint)iVar18 >> 8));
        auVar54[0xe] = -(pcVar4[0xe] == (char)((uint)iVar18 >> 0x10));
        auVar54[0xf] = -(pcVar4[0xf] == (char)((uint)iVar18 >> 0x18));
        if ((((((((((((((((SUB161(auVar54 >> 7,0) & 1) != 0 || (SUB161(auVar54 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar54 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar54 >> 0x1f,0) & 1) != 0) || (SUB161(auVar54 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar54 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar54 >> 0x37,0) & 1) != 0) || (SUB161(auVar54 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar54 >> 0x47,0) & 1) != 0) || (SUB161(auVar54 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar54 >> 0x57,0) & 1) != 0) || (SUB161(auVar54 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar54 >> 0x67,0) & 1) != 0) || (SUB161(auVar54 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar54 >> 0x77,0) & 1) != 0) || auVar54[0xf] < '\0') break;
        uVar36 = uVar36 + local_170 + 0x10;
        local_170 = local_170 + 0x10;
      }
    }
    *(undefined4 *)local_e8 = local_e0._0_4_;
    local_c1 = 1;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    goto LAB_14075c305;
  }
  local_c1 = 1;
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  local_128 = local_1a8;
  local_138 = local_1b8;
  pppppppuStack_130 = pppppppuStack_1b0;
  local_148 = local_1c8;
  pppppppuStack_140 = pppppppuStack_1c0;
  local_c1 = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::walls::WallId,core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>_>_>
            ();
  *(undefined4 *)local_e8 = local_e0._0_4_;
  lVar38 = local_d8[3];
  if (lVar38 != 0) {
    pauVar41 = (undefined1 (*) [16])*local_d8;
    uVar25 = local_d8[1];
    pauVar37 = pauVar41 + 1;
    auVar49 = *pauVar41;
    uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
    lVar23 = local_d8[2];
    lVar26 = lVar38;
    pauVar44 = pauVar41;
    do {
      if ((short)uVar20 == 0) {
        do {
          auVar49 = *pauVar37;
          pauVar44 = pauVar44 + -0x10;
          pauVar37 = pauVar37 + 1;
          uVar20 = (uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
        } while (uVar20 == 0xffff);
        uVar20 = ~uVar20;
      }
      iVar15 = 0;
      for (uVar22 = uVar20; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
        iVar15 = iVar15 + 1;
      }
      uVar20 = uVar20 - 1 & uVar20;
      lVar39 = (longlong)pauVar44 - (ulonglong)(uint)(iVar15 << 4);
      lVar26 = lVar26 + -1;
      lVar28 = 0;
LAB_14075c560:
      if (local_190 << 3 != lVar28) goto code_r0x00014075c565;
      lVar28 = (longlong)pauVar41 - lVar39 >> 4;
      uVar36 = lVar28 - 0x10U & uVar25;
      pcVar4 = *pauVar41 + uVar36;
      auVar49[0] = -(*pcVar4 == -1);
      auVar49[1] = -(pcVar4[1] == -1);
      auVar49[2] = -(pcVar4[2] == -1);
      auVar49[3] = -(pcVar4[3] == -1);
      auVar49[4] = -(pcVar4[4] == -1);
      auVar49[5] = -(pcVar4[5] == -1);
      auVar49[6] = -(pcVar4[6] == -1);
      auVar49[7] = -(pcVar4[7] == -1);
      auVar49[8] = -(pcVar4[8] == -1);
      auVar49[9] = -(pcVar4[9] == -1);
      auVar49[10] = -(pcVar4[10] == -1);
      auVar49[0xb] = -(pcVar4[0xb] == -1);
      auVar49[0xc] = -(pcVar4[0xc] == -1);
      auVar49[0xd] = -(pcVar4[0xd] == -1);
      auVar49[0xe] = -(pcVar4[0xe] == -1);
      auVar49[0xf] = -(pcVar4[0xf] == -1);
      uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe | (ushort)(auVar49[0xf] >> 7) << 0xf;
      if (uVar19 == 0) {
        uVar40 = 0x10;
      }
      else {
        uVar40 = 0xf;
        if (uVar19 != 0) {
          for (; uVar19 >> uVar40 == 0; uVar40 = uVar40 - 1) {
          }
        }
        uVar40 = uVar40 ^ 0xf;
      }
      pcVar4 = *pauVar41 + lVar28;
      auVar55[0] = -(*pcVar4 == -1);
      auVar55[1] = -(pcVar4[1] == -1);
      auVar55[2] = -(pcVar4[2] == -1);
      auVar55[3] = -(pcVar4[3] == -1);
      auVar55[4] = -(pcVar4[4] == -1);
      auVar55[5] = -(pcVar4[5] == -1);
      auVar55[6] = -(pcVar4[6] == -1);
      auVar55[7] = -(pcVar4[7] == -1);
      auVar55[8] = -(pcVar4[8] == -1);
      auVar55[9] = -(pcVar4[9] == -1);
      auVar55[10] = -(pcVar4[10] == -1);
      auVar55[0xb] = -(pcVar4[0xb] == -1);
      auVar55[0xc] = -(pcVar4[0xc] == -1);
      auVar55[0xd] = -(pcVar4[0xd] == -1);
      auVar55[0xe] = -(pcVar4[0xe] == -1);
      auVar55[0xf] = -(pcVar4[0xf] == -1);
      sVar12 = 0;
      for (uVar22 = (ushort)((ushort)(SUB161(auVar55 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar55 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar55 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar55 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar55 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar55 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar55 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar55 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar55 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar55 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar55 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar55 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar55 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar55 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar55 >> 0x77,0) & 1) << 0xe |
                            (ushort)(auVar55[0xf] >> 7) << 0xf) | 0x10000; (uVar22 & 1) == 0;
          uVar22 = uVar22 >> 1 | 0x80000000) {
        sVar12 = sVar12 + 1;
      }
      uVar31 = 0x80;
      if ((ushort)(uVar40 + sVar12) < 0x10) {
        lVar23 = lVar23 + 1;
        local_d8[2] = lVar23;
        uVar31 = 0xff;
      }
      (*pauVar41)[lVar28] = uVar31;
      pauVar41[1][uVar36] = uVar31;
      lVar38 = lVar38 + -1;
      local_d8[3] = lVar38;
LAB_14075c501:
      if (lVar26 == 0) break;
    } while( true );
  }
  if (local_1a0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_148 = (ulonglong *******)&local_1c8;
  uVar25 = local_178;
  if (1 < local_178) {
    if (local_178 < 0x15) {
      local_c1 = 1;
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<country_core::resources::walls::WallId,alloc::slice::impl_0::sort_by_key::closure_env_0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env_4>_>
                ();
    }
    else {
      local_c1 = 1;
      core::slice::sort::stable::
      driftsort_main<country_core::resources::walls::WallId,alloc::slice::impl_0::sort_by_key::closure_env_0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env_4>,alloc::vec::Vec<country_core::resources::walls::WallId,alloc::alloc::Global>_>
                ();
    }
    uVar25 = local_178;
    if (1 < local_178) {
      uVar36 = 1;
      do {
        uVar27 = uVar36;
        if (local_180[uVar36] == local_180[uVar36 - 1]) goto joined_r0x00014075ccfd;
        uVar36 = uVar36 + 1;
      } while (local_178 != uVar36);
    }
  }
LAB_14075c620:
  local_178 = uVar25;
  local_e8 = (ulonglong *)local_188;
  local_d0 = local_180 + local_178;
  local_d8 = (longlong *)*local_160;
  local_150 = local_d8 + 4;
  plVar42 = local_180;
LAB_14075c66e:
  while (plVar42 != local_d0) {
    local_1a0 = *plVar42;
    **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>_>
              ();
    plVar42 = plVar42 + 1;
    if (!SBORROW8(0,(longlong)pppppppuStack_1c0)) {
      pppppppuStack_140 = local_1b8;
      local_138 = pppppppuStack_1b0;
      local_148 = pppppppuStack_1c0;
      pppppppuStack_130 = (ulonglong *******)0x0;
      local_110 = 0;
      pppppppuVar32 = (ulonglong *******)0x0;
      pppppppuVar29 = pppppppuStack_1c0;
      do {
        pppppppuVar30 = (ulonglong *******)0x8000000000000001;
        pppppppuVar24 = pppppppuVar29;
        pppppppuVar33 = pppppppuVar32;
        if (pppppppuVar29 != (ulonglong *******)0x8000000000000001) goto LAB_14075c7a6;
LAB_14075c7b8:
        pppppppuVar32 = pppppppuVar33;
        if (local_110 == 0) {
LAB_14075c8b6:
          if ((-0x7fffffffffffffff < (longlong)local_148) && (local_148 != (ulonglong *******)0x0))
          {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          plVar43 = local_d8;
          if ((pppppppuVar32 != (ulonglong *******)0x0) && (local_120 != (ulonglong *******)0x0)) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          plVar11 = (longlong *)*local_168;
          if (plVar11[3] == 0) goto LAB_14075caed;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          bVar47 = (byte)(extraout_RAX_04 >> 0x39);
          auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
          auVar49 = pshuflw(auVar49,auVar49,0);
          lVar38 = 0;
          uVar25 = extraout_RAX_04;
          goto LAB_14075c961;
        }
        if (local_108 == local_f8) {
LAB_14075c890:
          if (local_100 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          local_110 = 0;
          goto LAB_14075c8b6;
        }
        puVar2 = local_108 + 1;
        pppppppuVar24 = (ulonglong *******)*local_108;
        local_108 = puVar2;
        if ((ulonglong)pppppppuVar24 >> 0x20 == 0) goto LAB_14075c890;
        pppppppuVar33 = (ulonglong *******)0x0;
        pppppppuVar29 = pppppppuVar30;
LAB_14075c843:
        local_1c8 = (ulonglong *******)*in_stack_00000030;
        pppppppuStack_1c0 = (ulonglong *******)in_stack_00000030[1];
        local_1b8 = (ulonglong *******)in_stack_00000030[2];
        pppppppuStack_1b0 = (ulonglong *******)in_stack_00000030[3];
        local_1a8 = pppppppuVar24;
        bevy_ecs::system::commands::EntityCommands::despawn();
        if (pppppppuVar33 != (ulonglong *******)0x0) {
          do {
            if (local_128 != local_118) {
              pppppppuVar30 = local_128 + 1;
              pppppppuVar24 = (ulonglong *******)*local_128;
              local_128 = pppppppuVar30;
              if ((ulonglong)pppppppuVar24 >> 0x20 != 0) goto LAB_14075c843;
            }
            if (local_120 != (ulonglong *******)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            pppppppuStack_130 = (ulonglong *******)0x0;
            pppppppuVar33 = (ulonglong *******)0x0;
            pppppppuVar30 = (ulonglong *******)0x8000000000000001;
            pppppppuVar24 = local_148;
            if (local_148 == (ulonglong *******)0x8000000000000001) goto LAB_14075c7b8;
LAB_14075c7a6:
            pppppppuVar29 = local_138;
            pppppppuVar32 = pppppppuStack_140;
            local_148 = (ulonglong *******)&DAT_8000000000000000;
            pppppppuVar30 = (ulonglong *******)&DAT_8000000000000000;
            if (SBORROW8(0,(longlong)pppppppuVar24)) goto LAB_14075c7b8;
            if ((pppppppuVar33 != (ulonglong *******)0x0) && (local_120 != (ulonglong *******)0x0))
            {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            local_118 = pppppppuVar32 + (longlong)pppppppuVar29;
            pppppppuStack_130 = pppppppuVar32;
            local_128 = pppppppuVar32;
            pppppppuVar29 = (ulonglong *******)&DAT_8000000000000000;
            pppppppuVar33 = pppppppuVar32;
            local_120 = pppppppuVar24;
            if (pppppppuVar32 == (ulonglong *******)0x0) break;
          } while( true );
        }
      } while( true );
    }
  }
  if (local_e8 != (ulonglong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_c2 == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_06 == 1) {
    plVar42 = extraout_RAX_06 + 1;
  }
  else if ((*extraout_RAX_06 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar42 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
  goto LAB_14075cbf7;
  if (*plVar42 == 0) {
    *plVar42 = -1;
    local_d0 = plVar42;
    puffin::ThreadProfiler::end_scope();
    *local_d0 = *local_d0 + 1;
    return;
  }
  goto LAB_14075ccd1;
code_r0x00014075c565:
  plVar42 = (longlong *)(local_198 + lVar28);
  lVar28 = lVar28 + 8;
  if (*(longlong *)(lVar39 + -0x10) == *plVar42) goto LAB_14075c501;
  goto LAB_14075c560;
joined_r0x00014075ccfd:
  while (uVar27 = uVar27 + 1, uVar25 = uVar36, uVar27 < local_178) {
    if (local_180[uVar27] != local_180[uVar36 - 1]) {
      local_180[uVar36] = local_180[uVar27];
      uVar36 = uVar36 + 1;
    }
  }
  goto LAB_14075c620;
LAB_14075c961:
  uVar25 = plVar11[1] & uVar25;
  pcVar4 = (char *)(*plVar11 + uVar25);
  cVar46 = auVar49[0];
  auVar59[0] = -(*pcVar4 == cVar46);
  cVar50 = auVar49[1];
  auVar59[1] = -(pcVar4[1] == cVar50);
  cVar51 = auVar49[2];
  auVar59[2] = -(pcVar4[2] == cVar51);
  cVar52 = auVar49[3];
  auVar59[3] = -(pcVar4[3] == cVar52);
  auVar59[4] = -(pcVar4[4] == cVar46);
  auVar59[5] = -(pcVar4[5] == cVar50);
  auVar59[6] = -(pcVar4[6] == cVar51);
  auVar59[7] = -(pcVar4[7] == cVar52);
  auVar59[8] = -(pcVar4[8] == cVar46);
  auVar59[9] = -(pcVar4[9] == cVar50);
  auVar59[10] = -(pcVar4[10] == cVar51);
  auVar59[0xb] = -(pcVar4[0xb] == cVar52);
  auVar59[0xc] = -(pcVar4[0xc] == cVar46);
  auVar59[0xd] = -(pcVar4[0xd] == cVar50);
  auVar59[0xe] = -(pcVar4[0xe] == cVar51);
  auVar59[0xf] = -(pcVar4[0xf] == cVar52);
  uVar19 = (ushort)(SUB161(auVar59 >> 7,0) & 1) | (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe | (ushort)(auVar59[0xf] >> 7) << 0xf;
  uVar20 = (uint)uVar19;
  while (uVar19 != 0) {
    uVar22 = 0;
    for (uVar13 = uVar20; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    if (local_1a0 == *(longlong *)(*plVar11 + -0x2a8 + (uVar22 + uVar25 & plVar11[1]) * -0x2a8)) {
      country_core::resources::walls::inner_wall_state::StructureType::is_fence();
      if (extraout_AL_00 != '\0') goto LAB_14075c66e;
      if (plVar43[3] == 0) goto LAB_14075caa1;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar47 = (byte)(extraout_RAX_05 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
      auVar49 = pshuflw(auVar49,auVar49,0);
      lVar38 = 0;
      uVar25 = extraout_RAX_05;
      goto LAB_14075ca2e;
    }
    uVar19 = (ushort)(uVar20 - 1) & (ushort)uVar20;
    uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar19);
  }
  auVar56[0] = -(*pcVar4 == -1);
  auVar56[1] = -(pcVar4[1] == -1);
  auVar56[2] = -(pcVar4[2] == -1);
  auVar56[3] = -(pcVar4[3] == -1);
  auVar56[4] = -(pcVar4[4] == -1);
  auVar56[5] = -(pcVar4[5] == -1);
  auVar56[6] = -(pcVar4[6] == -1);
  auVar56[7] = -(pcVar4[7] == -1);
  auVar56[8] = -(pcVar4[8] == -1);
  auVar56[9] = -(pcVar4[9] == -1);
  auVar56[10] = -(pcVar4[10] == -1);
  auVar56[0xb] = -(pcVar4[0xb] == -1);
  auVar56[0xc] = -(pcVar4[0xc] == -1);
  auVar56[0xd] = -(pcVar4[0xd] == -1);
  auVar56[0xe] = -(pcVar4[0xe] == -1);
  auVar56[0xf] = -(pcVar4[0xf] == -1);
  if ((((((((((((((((SUB161(auVar56 >> 7,0) & 1) != 0 || (SUB161(auVar56 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar56 >> 0x17,0) & 1) != 0) || (SUB161(auVar56 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar56 >> 0x27,0) & 1) != 0) || (SUB161(auVar56 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar56 >> 0x37,0) & 1) != 0) || (SUB161(auVar56 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar56 >> 0x47,0) & 1) != 0) || (SUB161(auVar56 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar56 >> 0x57,0) & 1) != 0) || (SUB161(auVar56 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar56 >> 0x67,0) & 1) != 0) || (SUB161(auVar56 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar56 >> 0x77,0) & 1) != 0) || auVar56[0xf] < '\0') {
LAB_14075caed:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar25 = uVar25 + lVar38 + 0x10;
  lVar38 = lVar38 + 0x10;
  goto LAB_14075c961;
LAB_14075ca2e:
  uVar25 = plVar43[1] & uVar25;
  pcVar4 = (char *)(*plVar43 + uVar25);
  cVar46 = auVar49[0];
  auVar60[0] = -(*pcVar4 == cVar46);
  cVar50 = auVar49[1];
  auVar60[1] = -(pcVar4[1] == cVar50);
  cVar51 = auVar49[2];
  auVar60[2] = -(pcVar4[2] == cVar51);
  cVar52 = auVar49[3];
  auVar60[3] = -(pcVar4[3] == cVar52);
  auVar60[4] = -(pcVar4[4] == cVar46);
  auVar60[5] = -(pcVar4[5] == cVar50);
  auVar60[6] = -(pcVar4[6] == cVar51);
  auVar60[7] = -(pcVar4[7] == cVar52);
  auVar60[8] = -(pcVar4[8] == cVar46);
  auVar60[9] = -(pcVar4[9] == cVar50);
  auVar60[10] = -(pcVar4[10] == cVar51);
  auVar60[0xb] = -(pcVar4[0xb] == cVar52);
  auVar60[0xc] = -(pcVar4[0xc] == cVar46);
  auVar60[0xd] = -(pcVar4[0xd] == cVar50);
  auVar60[0xe] = -(pcVar4[0xe] == cVar51);
  auVar60[0xf] = -(pcVar4[0xf] == cVar52);
  uVar19 = (ushort)(SUB161(auVar60 >> 7,0) & 1) | (ushort)(SUB161(auVar60 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar60 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar60 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar60 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar60 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar60 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar60 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar60 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar60 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar60 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar60 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar60 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar60 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar60 >> 0x77,0) & 1) << 0xe | (ushort)(auVar60[0xf] >> 7) << 0xf;
  uVar20 = (uint)uVar19;
  while (uVar19 != 0) {
    uVar22 = 0;
    for (uVar13 = uVar20; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    if (local_1a0 == *(longlong *)(*plVar43 + -0x68 + (uVar22 + uVar25 & plVar43[1]) * -0x68)) {
      country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized();
      country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates();
      goto LAB_14075caa1;
    }
    uVar19 = (ushort)(uVar20 - 1) & (ushort)uVar20;
    uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar19);
  }
  auVar57[0] = -(*pcVar4 == -1);
  auVar57[1] = -(pcVar4[1] == -1);
  auVar57[2] = -(pcVar4[2] == -1);
  auVar57[3] = -(pcVar4[3] == -1);
  auVar57[4] = -(pcVar4[4] == -1);
  auVar57[5] = -(pcVar4[5] == -1);
  auVar57[6] = -(pcVar4[6] == -1);
  auVar57[7] = -(pcVar4[7] == -1);
  auVar57[8] = -(pcVar4[8] == -1);
  auVar57[9] = -(pcVar4[9] == -1);
  auVar57[10] = -(pcVar4[10] == -1);
  auVar57[0xb] = -(pcVar4[0xb] == -1);
  auVar57[0xc] = -(pcVar4[0xc] == -1);
  auVar57[0xd] = -(pcVar4[0xd] == -1);
  auVar57[0xe] = -(pcVar4[0xe] == -1);
  auVar57[0xf] = -(pcVar4[0xf] == -1);
  if ((((((((((((((((SUB161(auVar57 >> 7,0) & 1) != 0 || (SUB161(auVar57 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar57 >> 0x17,0) & 1) != 0) || (SUB161(auVar57 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar57 >> 0x27,0) & 1) != 0) || (SUB161(auVar57 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar57 >> 0x37,0) & 1) != 0) || (SUB161(auVar57 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar57 >> 0x47,0) & 1) != 0) || (SUB161(auVar57 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar57 >> 0x57,0) & 1) != 0) || (SUB161(auVar57 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar57 >> 0x67,0) & 1) != 0) || (SUB161(auVar57 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar57 >> 0x77,0) & 1) != 0) || auVar57[0xf] < '\0') goto LAB_14075caa1;
  uVar25 = uVar25 + lVar38 + 0x10;
  lVar38 = lVar38 + 0x10;
  goto LAB_14075ca2e;
LAB_14075caa1:
  generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>_>,country_core::resources::walls::wall_holes::impl_1::holes_no_duplicates::closure_env_0>,country_core::resources::walls::wall_holes::impl_1::holes_no_duplicates::closure_env_1>_>
            ();
  goto LAB_14075c66e;
}

