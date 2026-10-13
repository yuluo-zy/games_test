// address 0x140992280
// name extend_no_data
// signature void __cdecl extend_no_data(void)

void __cdecl country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  int iVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined8 uVar11;
  char ***pppcVar12;
  char extraout_AL;
  char extraout_AL_00;
  ulonglong extraout_RAX;
  longlong lVar13;
  code *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  ulonglong uVar14;
  ulonglong *in_RCX;
  ulonglong uVar15;
  byte in_DL;
  undefined8 *puVar16;
  undefined8 *in_R8;
  ulonglong uVar17;
  uint uVar18;
  longlong in_R9;
  ushort uVar19;
  ulonglong uVar20;
  undefined8 *puVar21;
  bool bVar22;
  byte bVar23;
  char cVar24;
  char cVar26;
  char cVar27;
  char cVar28;
  undefined1 auVar25 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  undefined8 in_stack_00000028;
  longlong in_stack_00000030;
  undefined ***local_158;
  void *pvStack_150;
  ulonglong local_148;
  char ****local_128;
  char ***pppcStack_120;
  char ****local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined *local_f0;
  undefined8 local_e8;
  undefined4 local_e0;
  undefined4 local_dc;
  undefined *local_d8;
  undefined8 local_d0;
  char ****local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined ***local_a8;
  void *pvStack_a0;
  ulonglong local_98;
  char ***local_90;
  undefined ****local_88;
  code *pcStack_80;
  longlong local_78;
  char local_62;
  byte local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  if (in_R9 != 0) {
    puVar16 = in_R8 + in_R9 * 2;
    pcVar4 = *(code **)(in_stack_00000030 + 0x20);
    iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    local_61 = in_DL;
    while (puVar21 = in_R8, in_R8 != puVar16) {
      while (in_R8 = puVar21 + 2,
            in_RCX[0x14] < *(ulonglong *)(&DAT_142aba790 + (ulonglong)local_61 * 8)) {
LAB_14099233b:
        iVar3 = (int)in_RCX[0x16];
        *(int *)(in_RCX + 0x16) = iVar3 + 1;
        local_128 = (char ****)CONCAT44(local_128._4_4_,iVar3);
        if (in_RCX[6] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                    ();
          bVar23 = (byte)(extraout_RAX >> 0x39);
          auVar25 = ZEXT216(CONCAT11(bVar23,bVar23));
          auVar25 = pshuflw(auVar25,auVar25,0);
          lVar13 = 0;
          uVar17 = extraout_RAX;
          while( true ) {
            uVar17 = in_RCX[4] & uVar17;
            pcVar1 = (char *)(in_RCX[3] + uVar17);
            cVar24 = auVar25[0];
            auVar30[0] = -(*pcVar1 == cVar24);
            cVar26 = auVar25[1];
            auVar30[1] = -(pcVar1[1] == cVar26);
            cVar27 = auVar25[2];
            auVar30[2] = -(pcVar1[2] == cVar27);
            cVar28 = auVar25[3];
            auVar30[3] = -(pcVar1[3] == cVar28);
            auVar30[4] = -(pcVar1[4] == cVar24);
            auVar30[5] = -(pcVar1[5] == cVar26);
            auVar30[6] = -(pcVar1[6] == cVar27);
            auVar30[7] = -(pcVar1[7] == cVar28);
            auVar30[8] = -(pcVar1[8] == cVar24);
            auVar30[9] = -(pcVar1[9] == cVar26);
            auVar30[10] = -(pcVar1[10] == cVar27);
            auVar30[0xb] = -(pcVar1[0xb] == cVar28);
            auVar30[0xc] = -(pcVar1[0xc] == cVar24);
            auVar30[0xd] = -(pcVar1[0xd] == cVar26);
            auVar30[0xe] = -(pcVar1[0xe] == cVar27);
            auVar30[0xf] = -(pcVar1[0xf] == cVar28);
            uVar19 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar30[0xf] >> 7) << 0xf;
            uVar18 = (uint)uVar19;
            while (uVar19 != 0) {
              uVar5 = 0;
              for (uVar6 = uVar18; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                uVar5 = uVar5 + 1;
              }
              if (*(int *)(in_RCX[3] + (uVar5 + uVar17 & in_RCX[4]) * -8 + -8) == iVar3)
              goto LAB_14099233b;
              uVar19 = (ushort)(uVar18 - 1) & (ushort)uVar18;
              uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar19);
            }
            auVar29[0] = -(*pcVar1 == (char)iVar7);
            auVar29[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
            auVar29[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
            auVar29[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
            auVar29[4] = -(pcVar1[4] == (char)iVar8);
            auVar29[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
            auVar29[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
            auVar29[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
            auVar29[8] = -(pcVar1[8] == (char)iVar9);
            auVar29[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
            auVar29[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
            auVar29[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
            auVar29[0xc] = -(pcVar1[0xc] == (char)iVar10);
            auVar29[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
            auVar29[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
            auVar29[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
            if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar29 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar29 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar29 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar29 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x3f,0) & 1) != 0) || (SUB161(auVar29 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
            uVar17 = uVar17 + lVar13 + 0x10;
            lVar13 = lVar13 + 0x10;
          }
        }
        uVar17 = in_RCX[2];
        if (uVar17 == *in_RCX) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
        }
        *(int *)(in_RCX[1] + uVar17 * 4) = iVar3;
        in_RCX[2] = uVar17 + 1;
        hashbrown::map::HashMap::
        insert<country_core::render::nani::gpu_world::InstanceKey,u32,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        uVar20 = uVar17 & 0xffffffff;
        local_128 = (char ****)CONCAT44((int)uVar17,(int)local_128);
        pppcStack_120 = (char ***)CONCAT71(pppcStack_120._1_7_,local_61);
        (*pcVar4)(in_stack_00000028,&local_128);
        if (in_RCX[0xc] <= uVar20) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar11 = puVar21[1];
        puVar2 = (undefined8 *)(in_RCX[0xb] + uVar20 * 0x10);
        *puVar2 = *puVar21;
        puVar2[1] = uVar11;
        uVar17 = in_RCX[9];
        do {
          uVar15 = uVar20;
          if (uVar17 <= uVar20) {
            uVar15 = uVar17;
          }
          LOCK();
          uVar14 = in_RCX[9];
          bVar22 = uVar17 == uVar14;
          if (bVar22) {
            in_RCX[9] = uVar15;
            uVar14 = uVar17;
          }
          UNLOCK();
          uVar17 = uVar14;
        } while (!bVar22);
        uVar17 = in_RCX[10];
        do {
          uVar15 = uVar20;
          if (uVar20 < uVar17) {
            uVar15 = uVar17;
          }
          LOCK();
          uVar14 = in_RCX[10];
          bVar22 = uVar17 == uVar14;
          if (bVar22) {
            in_RCX[10] = uVar15;
            uVar14 = uVar17;
          }
          UNLOCK();
          uVar17 = uVar14;
        } while (!bVar22);
        in_RCX[0x14] = in_RCX[0x14] + 1;
        puVar21 = in_R8;
        if (in_R8 == puVar16) {
          return;
        }
      }
      local_88 = (undefined ****)&local_61;
      pcStack_80 = gpu_world::impl$17::fmt;
      local_128 = (char ****)&PTR_s_Max_instance_count_reached_for_142aba528;
      pppcStack_120 = (char ***)0x1;
      local_118 = (char ****)&local_88;
      local_110 = 1;
      local_108 = 0;
      alloc::fmt::format::format_inner();
      local_a8 = local_158;
      pvStack_a0 = pvStack_150;
      local_98 = local_148;
      if (DAT_143634ce8 != 3) {
        local_88 = (undefined ****)CONCAT71(local_88._1_7_,1);
        local_128 = (char ****)&local_88;
        std::sys::sync::once::futex::Once::call();
      }
      std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32>_>();
      if ((int)local_128 == 1) {
        local_88 = (undefined ****)pppcStack_120;
        pcStack_80 = (code *)CONCAT71(pcStack_80._1_7_,(char)local_118);
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_90 = pppcStack_120;
      local_62 = (char)local_118;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (code *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      *(undefined8 *)(extraout_RAX_00 + 7) = 0x2928202168746170;
      *(undefined8 *)extraout_RAX_00 = 0x705f656c75646f6d;
      local_128 = (char ****)&DAT_0000000f;
      local_118 = (char ****)0xf;
      pppcStack_120 = (char ***)extraout_RAX_00;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      *(undefined8 *)((longlong)pppcStack_120 + (longlong)local_118) = 0x3a20657461726324;
      *(undefined8 *)((code *)((longlong)pppcStack_120 + (longlong)local_118) + 8) =
           0x206c6576654c203a;
      *(undefined8 *)((longlong)pppcStack_120 + (longlong)local_118 + 0xf) = 0x6e726157203a3a20;
      local_78 = (longlong)local_118 + 0x17;
      local_88 = local_128;
      pcStack_80 = (code *)pppcStack_120;
      if ((ulonglong)((longlong)local_128 - local_78) < local_98) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar13 = local_78;
      memcpy(pcStack_80 + local_78,pvStack_a0,local_98);
      local_118 = (char ****)(lVar13 + local_98);
      local_128 = local_88;
      pppcStack_120 = (char ***)pcStack_80;
      local_78 = (longlong)local_118;
      alloc::collections::btree::map::BTreeMap::
      insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
                ();
      if ((extraout_AL == '\0') &&
         (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
        local_88 = &local_a8;
        pcStack_80 = alloc::string::impl$23::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_108 = *extraout_RAX_01;
        uStack_100 = extraout_RAX_01[1];
        local_dc = *(undefined4 *)(extraout_RAX_01 + 2);
        local_f8 = 2;
        local_f0 = &anon_3703a396df78e7356336212e87fdc18b_57_llvm_1710316161241481988;
        local_e8 = 0x25;
        local_d8 = &DAT_142aba568;
        local_d0 = 1;
        local_c8 = (char ****)&local_88;
        local_c0 = 1;
        local_b8 = 0;
        local_128 = (char ****)0x0;
        pppcStack_120 = (char ***)&anon_3703a396df78e7356336212e87fdc18b_57_llvm_1710316161241481988
        ;
        local_118 = (char ****)0x25;
        local_110 = 0;
        local_e0 = 1;
        ::log::__private_api::impl$2::log();
      }
      pppcVar12 = local_90;
      if (((local_62 == '\0') &&
          ((*(ulonglong *)
             __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
           0x7fffffffffffffff) != 0)) &&
         (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
        *(char *)((longlong)pppcVar12 + 1) = '\x01';
      }
      LOCK();
      cVar24 = *(char *)pppcVar12;
      *(char *)pppcVar12 = '\0';
      UNLOCK();
      if (cVar24 == '\x02') {
        std::sys::sync::mutex::futex::Mutex::wake();
      }
      if (local_a8 != (char ***)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  return;
}

