// dump_save @ 0x1403ad640
// sig: void __cdecl dump_save(void)


void __cdecl tiny_glade::panic_reporter::panic_dump::dump_save(void)

{
  undefined4 *puVar1;
  byte bVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined *puVar10;
  HANDLE extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined8 *puVar11;
  char *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  char cVar12;
  longlong *in_RCX;
  HANDLE pvVar13;
  longlong lVar14;
  longlong lVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined1 uVar18;
  byte *pbVar19;
  ulonglong uVar20;
  longlong lVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  undefined4 uVar24;
  undefined4 in_XMM1_Da;
  undefined4 in_XMM1_Db;
  undefined4 uStack_1ef;
  undefined4 uStack_1eb;
  undefined4 uStack_1e7;
  undefined4 uStack_1e3;
  undefined7 uStack_1df;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  longlong local_1b8;
  longlong local_1b0;
  longlong local_1a8;
  longlong local_1a0;
  HANDLE local_198 [5];
  longlong local_170;
  HANDLE local_168;
  longlong local_160;
  undefined8 local_150;
  undefined8 local_148;
  undefined1 local_140;
  ulonglong *local_138;
  code *local_130;
  HANDLE *local_128;
  code *local_120;
  ulonglong **local_118;
  undefined8 local_110;
  longlong *local_108;
  undefined8 local_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  undefined8 local_d8;
  ulonglong local_d0;
  undefined8 uStack_c8;
  undefined4 local_c0;
  undefined4 uStack_bc;
  undefined1 uStack_b8;
  undefined1 uStack_b7;
  undefined1 uStack_b6;
  undefined1 uStack_b5;
  undefined1 uStack_b4;
  undefined1 uStack_b3;
  undefined2 uStack_b2;
  longlong local_b0;
  undefined1 local_a8;
  longlong local_90;
  char local_88;
  undefined4 local_74;
  ulonglong local_70;
  longlong local_68;
  HANDLE local_60;
  undefined8 *local_58;
  ulonglong local_50;
  undefined8 local_48;
  
  puVar10 = 
  __imp__ZN12country_core7startup22panic_reporter_startup19PANIC_REPORTER_INFO17hbf0a1901d5a3098fE;
  local_48 = 0xfffffffffffffffe;
  if (*(int *)
       __imp__ZN12country_core7startup22panic_reporter_startup19PANIC_REPORTER_INFO17hbf0a1901d5a3098fE
      == 2) {
    in_RCX[1] = -0x8000000000000000;
    *in_RCX = -0x8000000000000000;
    return;
  }
  std::path::Path::_join();
  local_d8 = (ulonglong)local_d8._4_4_ << 0x20;
  local_d0 = 0;
  uStack_c8 = 0;
  local_c0 = 7;
  uStack_bc = 0;
  uStack_b8 = 0;
  uStack_b6 = 0;
  uStack_b3 = 0;
  uStack_b7 = 1;
  uStack_b4 = 1;
  uStack_b5 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  pvVar13 = extraout_RAX;
  std::fs::OpenOptions::_open();
  local_60 = pvVar13;
  if ((extraout_RAX_00 & 1) != 0) {
    local_128 = local_198;
    local_120 = std::io::error::impl$9::fmt;
    local_d8 = 2;
    uStack_c8 = 2;
    uStack_b8 = 0;
    uStack_b7 = 0;
    uStack_b6 = 0;
    uStack_b5 = 0;
    uStack_b4 = 0;
    uStack_b3 = 0;
    uStack_b2 = 0;
    local_b0 = 0x400000020;
    local_a8 = 3;
    local_198[0] = pvVar13;
    alloc::fmt::format::format_inner();
    lVar3 = local_160;
    lVar14 = local_170;
    if (((uint)local_198[0] & 3) == 1) {
      local_68 = (longlong)local_198[0] + -1;
      local_50 = *(ulonglong *)((longlong)local_198[0] + -1);
      local_58 = *(undefined8 **)((longlong)local_198[0] + 7);
      if ((code *)*local_58 != (code *)0x0) {
        (*(code *)*local_58)(local_50);
      }
      if (local_58[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_60 = local_168;
    if (lVar14 != -0x7fffffffffffffff) {
      in_RCX[1] = lVar14;
      in_RCX[2] = (longlong)local_168;
      in_RCX[3] = lVar3;
      *in_RCX = -0x8000000000000000;
      goto joined_r0x0001403adaee;
    }
  }
  lVar14 = *(longlong *)(puVar10 + 0x18);
  if (*(longlong *)(lVar14 + 0x10) == 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (char *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    builtin_strncpy(extraout_RAX_02,"History is empty",0x10);
    in_RCX[1] = 0x10;
    in_RCX[2] = (longlong)extraout_RAX_02;
    in_RCX[3] = 0x10;
    *in_RCX = -0x8000000000000000;
  }
  else {
    country_core::resources::history::HistoryIdentity::generate();
    local_58 = (undefined8 *)CONCAT44(local_58._4_4_,*(undefined4 *)(lVar14 + 0x70));
    local_50 = extraout_RAX_01;
    country_core::systems::camera_rig::MainCamera::store_parameters();
    local_68 = *(longlong *)(puVar10 + 0x28);
    local_74 = *(undefined4 *)(lVar14 + 0x68);
    country_core::resources::history::history_chain::HistoryEditChain::iter();
    country_core::resources::history::History::iter_resolved_snapshots();
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::resources::history::ResolvedHistoryChainSnapshot,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::history::HistoryChainSnapshot>,country_core::resources::history::impl_15::iter_resolved_snapshots::closure_env_0>_>
              ();
    country_core::resources::glade_color_customization::impl$0::default();
    cVar12 = (char)&local_d8;
    country_core::systems::save::save_game::
    save_game_to_file<std::fs::File,core::slice::iter::Iter<country_core::resources::history::MultiEdit>_>
              ();
    uVar16 = local_d0;
    lVar14 = local_d8;
    puVar11 = (undefined8 *)-local_d8;
    if (!SBORROW8(0,local_d8)) {
      lVar3 = CONCAT26(uStack_b2,
                       CONCAT15(uStack_b3,
                                CONCAT14(uStack_b4,
                                         CONCAT13(uStack_b5,
                                                  CONCAT12(uStack_b6,CONCAT11(uStack_b7,uStack_b8)))
                                        )));
      pbVar19 = (byte *)0x0;
      uVar24 = extraout_XMM0_Da;
      if (lVar3 != 0) {
        cVar12 = '\x18';
        puVar11 = (undefined8 *)((lVar3 + 1) * 0x18 + 0xfU & 0xfffffffffffffff0);
        pbVar19 = (byte *)((longlong)puVar11 + lVar3 + 0x11);
        if (pbVar19 != (byte *)0x0) {
          cVar12 = (char)local_c0 - (char)puVar11;
          std::alloc::__default_lib_allocator::__rust_dealloc();
          puVar11 = extraout_RAX_03;
          uVar24 = extraout_XMM0_Da_00;
        }
      }
      puVar10 = 
      __imp__ZN12country_core7startup22panic_reporter_startup19PANIC_REPORTER_INFO17hbf0a1901d5a3098fE
      ;
      if (lVar14 != 0) {
        uVar23 = uVar16;
        std::alloc::__default_lib_allocator::__rust_dealloc();
        cVar12 = (char)uVar23;
        puVar11 = extraout_RAX_04;
        uVar24 = extraout_XMM0_Da_01;
      }
      lVar14 = 0;
      if (local_90 != 0) {
        lVar14 = local_90 << 4;
        cVar12 = local_88;
        std::alloc::__default_lib_allocator::__rust_dealloc();
        puVar11 = extraout_RAX_05;
        uVar24 = extraout_XMM0_Da_02;
      }
      uVar18 = SUB81(pbVar19,0);
      if (*(int *)puVar10 == 1) {
        lVar3 = *(longlong *)(puVar10 + 8);
        if (lVar3 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        uVar23 = *(ulonglong *)(lVar3 + 0x10);
        if ((longlong)uVar23 < 0) {
LAB_1403ade09:
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        uVar16 = (ulonglong)*(byte *)(lVar3 + 0x58);
        if (uVar23 == 0) {
          puVar11 = (undefined8 *)0x1;
        }
        else {
          lVar14 = *(longlong *)(lVar3 + 8);
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_06 == (undefined8 *)0x0) goto LAB_1403ade09;
          uVar20 = uVar23 - 1;
          if (uVar23 < uVar23 - 1) {
            uVar20 = uVar23;
          }
          if ((ulonglong)((longlong)extraout_RAX_06 - lVar14) < 0x20 || uVar20 < 0x20) {
            lVar21 = 0;
            uVar20 = uVar23;
            lVar15 = lVar14;
            uVar24 = extraout_XMM0_Da_03;
          }
          else {
            uVar20 = uVar20 + 1;
            uVar22 = 0x20;
            if ((uVar20 & 0x1f) != 0) {
              uVar22 = (ulonglong)((uint)uVar20 & 0x1f);
            }
            lVar21 = uVar20 - uVar22;
            uVar20 = uVar23 - lVar21;
            lVar15 = lVar14 + lVar21;
            lVar17 = 0;
            do {
              puVar1 = (undefined4 *)(lVar14 + lVar17);
              uVar24 = *puVar1;
              uVar4 = puVar1[1];
              uVar5 = puVar1[2];
              uVar6 = puVar1[3];
              puVar1 = (undefined4 *)(lVar14 + 0x10 + lVar17);
              in_XMM1_Da = *puVar1;
              in_XMM1_Db = puVar1[1];
              uVar7 = puVar1[2];
              uVar8 = puVar1[3];
              puVar1 = (undefined4 *)((longlong)extraout_RAX_06 + lVar17);
              *puVar1 = uVar24;
              puVar1[1] = uVar4;
              puVar1[2] = uVar5;
              puVar1[3] = uVar6;
              puVar1 = (undefined4 *)((longlong)extraout_RAX_06 + lVar17 + 0x10);
              *puVar1 = in_XMM1_Da;
              puVar1[1] = in_XMM1_Db;
              puVar1[2] = uVar7;
              puVar1[3] = uVar8;
              lVar17 = lVar17 + 0x20;
            } while (lVar21 != lVar17);
          }
          uVar22 = 0;
          do {
            pbVar19 = (byte *)(lVar15 + uVar22);
            puVar11 = extraout_RAX_06;
            if (pbVar19 == (byte *)(lVar14 + uVar23)) break;
            bVar2 = *pbVar19;
            pbVar19 = (byte *)(ulonglong)bVar2;
            *(byte *)((longlong)extraout_RAX_06 + uVar22 + lVar21) = bVar2;
            uVar22 = uVar22 + 1;
          } while (uVar20 != uVar22);
        }
        uVar18 = SUB81(pbVar19,0);
        cVar12 = '\0';
        if (SBORROW8(0,*(longlong *)(lVar3 + 0x18))) {
          lVar14 = -0x8000000000000000;
        }
        else {
          uVar18 = *(undefined1 *)(lVar3 + 0x40);
          local_58 = puVar11;
          local_50 = uVar23;
          alloc::string::impl$6::clone();
          local_b0 = CONCAT71(local_b0._1_7_,uVar18);
          local_1c8 = *(undefined8 *)(lVar3 + 0x30);
          uStack_1c0 = *(undefined8 *)(lVar3 + 0x38);
          local_c0 = (undefined4)local_1c8;
          uStack_bc = (undefined4)((ulonglong)local_1c8 >> 0x20);
          uStack_b8 = (undefined1)uStack_1c0;
          uStack_b7 = (undefined1)((ulonglong)uStack_1c0 >> 8);
          uStack_b6 = (undefined1)((ulonglong)uStack_1c0 >> 0x10);
          uStack_b5 = (undefined1)((ulonglong)uStack_1c0 >> 0x18);
          uStack_b4 = (undefined1)((ulonglong)uStack_1c0 >> 0x20);
          uStack_b3 = (undefined1)((ulonglong)uStack_1c0 >> 0x28);
          uStack_b2 = (undefined2)((ulonglong)uStack_1c0 >> 0x30);
          in_XMM1_Da = (undefined4)*(undefined8 *)(lVar3 + 0x48);
          in_XMM1_Db = (undefined4)((ulonglong)*(undefined8 *)(lVar3 + 0x48) >> 0x20);
          uVar24 = *(undefined4 *)(lVar3 + 0x50);
          uVar18 = *(undefined1 *)(lVar3 + 0x54);
          cVar12 = *(char *)(lVar3 + 0x55);
          local_1d8 = local_d0;
          uStack_1d0 = uStack_c8;
          local_1b8 = local_b0;
          puVar11 = local_58;
          lVar14 = local_d8;
          uVar23 = local_50;
        }
        uVar9 = *(undefined8 *)(lVar3 + 0x60);
        uStack_1eb._3_1_ = (undefined1)uVar9;
        uStack_1e7 = (undefined4)((ulonglong)uVar9 >> 8);
        uStack_1e3._0_3_ = (undefined3)((ulonglong)uVar9 >> 0x28);
        uStack_1e3._3_1_ = (undefined1)*(undefined8 *)(lVar3 + 0x68);
        uStack_1df = (undefined7)((ulonglong)*(undefined8 *)(lVar3 + 0x68) >> 8);
      }
      else {
        uVar23 = 0x8000000000000000;
      }
      in_RCX[2] = local_e8;
      in_RCX[3] = lStack_e0;
      *in_RCX = local_f8;
      in_RCX[1] = lStack_f0;
      in_RCX[4] = uVar23;
      in_RCX[5] = (longlong)puVar11;
      in_RCX[6] = uVar23;
      in_RCX[7] = lVar14;
      *(undefined4 *)(in_RCX + 8) = (undefined4)local_1d8;
      *(undefined4 *)((longlong)in_RCX + 0x44) = local_1d8._4_4_;
      *(undefined4 *)(in_RCX + 9) = (undefined4)uStack_1d0;
      *(undefined4 *)((longlong)in_RCX + 0x4c) = uStack_1d0._4_4_;
      *(undefined4 *)(in_RCX + 10) = (undefined4)local_1c8;
      *(undefined4 *)((longlong)in_RCX + 0x54) = local_1c8._4_4_;
      *(undefined4 *)(in_RCX + 0xb) = (undefined4)uStack_1c0;
      *(undefined4 *)((longlong)in_RCX + 0x5c) = uStack_1c0._4_4_;
      in_RCX[0xc] = local_1b8;
      in_RCX[0xd] = CONCAT44(in_XMM1_Db,in_XMM1_Da);
      *(undefined4 *)(in_RCX + 0xe) = uVar24;
      *(undefined1 *)((longlong)in_RCX + 0x74) = uVar18;
      *(char *)((longlong)in_RCX + 0x75) = cVar12;
      *(char *)(in_RCX + 0xf) = (char)uVar16;
      *(undefined4 *)((longlong)in_RCX + 0x79) = uStack_1ef;
      *(undefined4 *)((longlong)in_RCX + 0x7d) = uStack_1eb;
      *(undefined4 *)((longlong)in_RCX + 0x81) = uStack_1e7;
      *(undefined4 *)((longlong)in_RCX + 0x85) = uStack_1e3;
      in_RCX[0x11] = CONCAT71(uStack_1df,uStack_1e3._3_1_);
      CloseHandle(local_60);
      return;
    }
    local_70 = local_d0;
    local_138 = &local_70;
    local_130 = anyhow::error::impl$4::fmt;
    local_170 = 2;
    local_160 = 2;
    local_150 = 0;
    local_148 = 0x400000020;
    local_140 = 3;
    local_128 = (HANDLE *)&DAT_1429cba70;
    local_120 = (code *)0x1;
    local_118 = &local_138;
    local_110 = 1;
    local_108 = &local_170;
    local_100 = 1;
    alloc::fmt::format::format_inner();
    anyhow::error::impl$6::drop();
    in_RCX[1] = local_1b0;
    in_RCX[2] = local_1a8;
    in_RCX[3] = local_1a0;
    *in_RCX = -0x8000000000000000;
    if (local_90 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  CloseHandle(local_60);
joined_r0x0001403adaee:
  if (local_f8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

