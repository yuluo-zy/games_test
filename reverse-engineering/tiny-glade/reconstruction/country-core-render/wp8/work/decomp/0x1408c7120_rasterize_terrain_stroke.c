
void __cdecl
country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::
rasterize_terrain_stroke(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined1 auVar8 [16];
  byte bVar9;
  byte bVar10;
  byte bVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  longlong ****pppplVar14;
  ushort uVar15;
  uint uVar16;
  undefined1 *_Src;
  longlong **pplVar17;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar18;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong ***extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong ***extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong lVar19;
  ulonglong uVar20;
  longlong *plVar21;
  longlong in_RDX;
  ulonglong uVar22;
  undefined4 *puVar23;
  undefined1 (*pauVar24) [16];
  longlong ***_Size;
  undefined *puVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  undefined1 (*pauVar28) [16];
  uint uVar29;
  bool bVar30;
  longlong ***local_5c8;
  undefined8 uStack_5c0;
  undefined8 local_5b8;
  undefined8 uStack_5b0;
  undefined8 local_5a8;
  undefined8 uStack_5a0;
  undefined8 local_598;
  undefined8 uStack_590;
  undefined8 local_588;
  undefined8 uStack_580;
  undefined4 local_428;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  undefined4 local_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  undefined4 uStack_408;
  longlong *local_400;
  longlong ***local_3f8;
  longlong **pplStack_3f0;
  ulonglong local_3e8;
  undefined8 uStack_3e0;
  undefined *local_3d8;
  undefined8 uStack_3d0;
  undefined8 local_3c8;
  undefined *puStack_3c0;
  undefined8 local_3b8;
  undefined8 local_3b0;
  undefined **ppuStack_3a8;
  undefined8 local_3a0;
  undefined8 local_398;
  undefined8 uStack_390;
  undefined8 local_380;
  undefined8 uStack_378;
  undefined8 local_370;
  undefined8 uStack_368;
  undefined8 local_360;
  undefined8 uStack_358;
  undefined8 local_350;
  undefined8 uStack_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined8 local_320;
  undefined8 local_318;
  char local_2c2;
  longlong ***local_268;
  longlong **pplStack_260;
  ulonglong local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined *puStack_230;
  undefined8 local_228;
  undefined4 local_220;
  undefined4 uStack_21c;
  undefined **ppuStack_218;
  undefined8 local_210;
  longlong ***local_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  char local_132;
  undefined8 local_130;
  undefined8 local_e8;
  longlong local_d0;
  longlong local_c0;
  longlong local_b8;
  ulonglong local_a8;
  undefined1 *local_a0;
  ulonglong local_98;
  longlong **local_90;
  longlong lStack_88;
  longlong **local_80;
  longlong **local_78;
  longlong *local_70;
  undefined1 local_63;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  uVar22 = 0x47c94cb78283a72e;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar22 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar22 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar22 * 8), uVar22 != 0)) {
    lVar19 = ~uVar22 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar19) != 0) {
      plVar21 = *(longlong **)(*(longlong *)(in_RDX + 0x1e0) + lVar19 + 0x10);
      uVar22 = 0x1e38c16726d0fb8d;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_00 != 0) && (uVar22 < *(ulonglong *)(in_RDX + 0x218))) &&
         (uVar22 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar22 * 8), uVar22 != 0)) {
        lVar19 = ~uVar22 * 0x60;
        if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar19) != 0) {
          lVar19 = *(longlong *)(in_RDX + 0x1e0) + lVar19;
          local_c0 = *(longlong *)(lVar19 + 0x10);
          *(undefined4 *)(lVar19 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
          uVar22 = 0xcda6af2f6afb130e;
          bevy_ecs::component::Components::get_resource_id();
          if (((extraout_RAX_01 != 0) && (uVar22 < *(ulonglong *)(in_RDX + 0x218))) &&
             (uVar22 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar22 * 8), uVar22 != 0)) {
            lVar19 = ~uVar22 * 0x60;
            if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar19) != 0) {
              lVar19 = *(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar19 + 0x10);
              if ((*(longlong *)(lVar19 + 0x30) != 0 || *(longlong *)(lVar19 + 0x10) != 0) ||
                 (*(char *)(local_c0 + 0x40) != '\x01')) {
                local_90 = (longlong **)0x0;
                lStack_88 = 8;
                local_80 = (longlong **)0x0;
                local_a8 = 0;
                local_a0 = &DAT_00000008;
                local_98 = 0;
                local_61 = 1;
                puffin::are_scopes_on();
                if (extraout_AL == '\0') {
                  local_d0 = 0;
                }
                else {
                  puVar25 = &DAT_142aa2f07;
                  do {
                    bVar9 = puVar25[-1];
                    uVar18 = (uint)(char)bVar9;
                    if ((char)bVar9 < '\0') {
                      bVar10 = puVar25[-2];
                      if ((char)bVar10 < -0x40) {
                        bVar11 = puVar25[-3];
                        if ((char)bVar11 < -0x40) {
                          pbVar1 = puVar25 + -4;
                          puVar25 = puVar25 + -4;
                          uVar18 = bVar11 & 0x3f | (*pbVar1 & 7) << 6;
                        }
                        else {
                          puVar25 = puVar25 + -3;
                          uVar18 = bVar11 & 0xf;
                        }
                        uVar18 = bVar10 & 0x3f | uVar18 << 6;
                      }
                      else {
                        puVar25 = puVar25 + -2;
                        uVar18 = bVar10 & 0x1f;
                      }
                      uVar18 = (uint)(bVar9 & 0x3f) | uVar18 << 6;
                      if (uVar18 != 0x5c) goto LAB_1408c73a4;
LAB_1408c73bc:
                      if (puVar25 == &UNK_142aa2eb0) break;
                      if (puVar25 + -0x142aa2eb0 < (undefined *)0x56) {
                        if (-0x41 < (char)puVar25[1]) break;
                      }
                      else if (puVar25 + -0x142aa2eb0 == (undefined *)0x56) break;
                      local_61 = 1;
                    // WARNING: Subroutine does not return
                      core::str::slice_error_fail();
                    }
                    puVar25 = puVar25 + -1;
                    if (uVar18 == 0x5c) goto LAB_1408c73bc;
LAB_1408c73a4:
                    if (uVar18 == 0x2f) goto LAB_1408c73bc;
                  } while (puVar25 != &DAT_142aa2eb1);
                  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
                  if (*extraout_RAX_02 == 0) {
                    local_61 = 1;
                    std::sys::thread_local::native::lazy::Storage::
                    initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                              ();
                    local_78 = (longlong **)extraout_RAX_03;
                    if (extraout_RAX_03 == (longlong ***)0x0) goto LAB_1408c8b24;
                  }
                  else {
                    if ((int)*extraout_RAX_02 != 1) {
LAB_1408c8b24:
                      local_61 = 1;
                    // WARNING: Subroutine does not return
                      std::thread::local::panic_access_error();
                    }
                    local_78 = (longlong **)(extraout_RAX_02 + 1);
                  }
                  if ((longlong **)*local_78 != (longlong **)0x0) {
                    local_61 = 1;
                    // WARNING: Subroutine does not return
                    core::cell::panic_already_borrowed();
                  }
                  *local_78 = (longlong *)0xffffffffffffffff;
                  puffin::ThreadProfiler::begin_scope();
                  *local_78 = (longlong *)((longlong)*local_78 + 1);
                  local_d0 = 1;
                }
                pauVar28 = *(undefined1 (**) [16])(local_c0 + 8);
                auVar8 = *pauVar28;
                pauVar24 = pauVar28 + 1;
                local_400 = plVar21 + 1;
                lVar19 = *(longlong *)(local_c0 + 0x20);
                uVar18 = ~(uint)(ushort)((ushort)(SUB161(auVar8 >> 7,0) & 1) |
                                         (ushort)(SUB161(auVar8 >> 0xf,0) & 1) << 1 |
                                         (ushort)(SUB161(auVar8 >> 0x17,0) & 1) << 2 |
                                         (ushort)(SUB161(auVar8 >> 0x1f,0) & 1) << 3 |
                                         (ushort)(SUB161(auVar8 >> 0x27,0) & 1) << 4 |
                                         (ushort)(SUB161(auVar8 >> 0x2f,0) & 1) << 5 |
                                         (ushort)(SUB161(auVar8 >> 0x37,0) & 1) << 6 |
                                         (ushort)(SUB161(auVar8 >> 0x3f,0) & 1) << 7 |
                                         (ushort)(SUB161(auVar8 >> 0x47,0) & 1) << 8 |
                                         (ushort)(SUB161(auVar8 >> 0x4f,0) & 1) << 9 |
                                         (ushort)(SUB161(auVar8 >> 0x57,0) & 1) << 10 |
                                         (ushort)(SUB161(auVar8 >> 0x5f,0) & 1) << 0xb |
                                         (ushort)(SUB161(auVar8 >> 0x67,0) & 1) << 0xc |
                                         (ushort)(SUB161(auVar8 >> 0x6f,0) & 1) << 0xd |
                                         (ushort)(SUB161(auVar8 >> 0x77,0) & 1) << 0xe |
                                        (ushort)(byte)(auVar8[0xf] >> 7) << 0xf);
                local_70 = plVar21;
                while (lVar19 != 0) {
                  if ((short)uVar18 == 0) {
                    do {
                      auVar8 = *pauVar24;
                      uVar15 = (ushort)(SUB161(auVar8 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar8 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar8 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar8 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar8 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar8 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar8 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar8 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar8 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar8 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar8 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar8 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar8 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar8 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar8 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar8[0xf] >> 7) << 0xf;
                      pauVar28 = pauVar28 + -0x30;
                      pauVar24 = pauVar24 + 1;
                    } while (uVar15 == 0xffff);
                    uVar18 = ~(uint)uVar15;
                    uVar29 = uVar18 & -(uint)uVar15 - 2;
                  }
                  else {
                    uVar29 = uVar18 - 1 & uVar18;
                  }
                  lVar19 = lVar19 + -1;
                  uVar16 = 0;
                  for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
                    uVar16 = uVar16 + 1;
                  }
                  uVar22 = (ulonglong)uVar16;
                  uVar18 = uVar29;
                  if ((0.01 < *(float *)(pauVar28[uVar22 * -3 + -1] + 8)) &&
                     (*(longlong *)(pauVar28[uVar22 * -3 + -2] + 8) != 0)) {
                    if (0x3fff < local_98) {
                      if (DAT_143634cb8 != 3) {
                        local_3f8 = (longlong ***)CONCAT71(local_3f8._1_7_,1);
                        local_268 = (longlong ***)&local_3f8;
                        std::sys::sync::once::futex::Once::call();
                      }
                      std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32>_>();
                      if ((int)local_268 == 1) {
                        local_3f8 = (longlong ***)pplStack_260;
                        pplStack_3f0 = (longlong **)
                                       CONCAT71(pplStack_3f0._1_7_,(undefined1)local_258);
                    // WARNING: Subroutine does not return
                        core::result::unwrap_failed();
                      }
                      local_78 = pplStack_260;
                      local_b8 = CONCAT71(local_b8._1_7_,(undefined1)local_258);
                      std::alloc::__default_lib_allocator::__rust_alloc();
                      if (extraout_RAX_05 == (longlong ***)0x0) {
                    // WARNING: Subroutine does not return
                        alloc::raw_vec::handle_error();
                      }
                      builtin_strncpy((char *)((longlong)extraout_RAX_05 + 7),"path! ()",8);
                      *extraout_RAX_05 = (longlong **)0x705f656c75646f6d;
                      local_268 = (longlong ***)&DAT_0000000f;
                      local_258 = 0xf;
                      pplStack_260 = (longlong **)extraout_RAX_05;
                      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>()
                      ;
                      builtin_strncpy((char *)((longlong)pplStack_260 + local_258),
                                      "$crate :: Level ",0x10);
                      builtin_strncpy((char *)((longlong)pplStack_260 + local_258 + 0x10),":: Error"
                                      ,8);
                      local_3e8 = local_258 + 0x18;
                      local_3f8 = local_268;
                      pplStack_3f0 = pplStack_260;
                      if ((longlong)local_268 - local_3e8 < 0x1f) {
                        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
                                  ();
                      }
                      builtin_strncpy((char *)((longlong)pplStack_3f0 + local_3e8 + 0xf),
                                      " terrain strokes",0x10);
                      builtin_strncpy((char *)((longlong)pplStack_3f0 + local_3e8),
                                      "Ran out of live ",0x10);
                      local_3e8 = local_3e8 + 0x1f;
                      local_268 = local_3f8;
                      pplStack_260 = pplStack_3f0;
                      local_258 = local_3e8;
                      alloc::collections::btree::map::BTreeMap::
                      insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
                                ();
                      if ((extraout_AL_00 == '\0') &&
                         (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0)
                         ) {
                        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                        local_248 = *extraout_RAX_06;
                        uStack_240 = extraout_RAX_06[1];
                        uStack_21c = *(undefined4 *)(extraout_RAX_06 + 2);
                        local_238 = 1;
                        puStack_230 = &DAT_142aa3050;
                        local_228 = 0x4c;
                        ppuStack_218 = &PTR_DAT_142aa3040;
                        local_210 = 1;
                        local_208 = (longlong ***)&local_3f8;
                        local_268 = (longlong ***)0x0;
                        uStack_200 = 0;
                        uStack_1f8 = 0;
                        pplStack_260 = (longlong **)&DAT_142aa3050;
                        local_258 = 0x4c;
                        uStack_250 = 0;
                        local_220 = 1;
                        ::log::__private_api::impl$2::log();
                      }
                      pplVar17 = local_78;
                      if ((((char)local_b8 == '\0') &&
                          ((*(ulonglong *)
                             __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
                           & 0x7fffffffffffffff) != 0)) &&
                         (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0'))
                      {
                        *(char *)((longlong)pplVar17 + 1) = '\x01';
                      }
                      LOCK();
                      cVar3 = *(char *)pplVar17;
                      *(char *)pplVar17 = '\0';
                      UNLOCK();
                      if (cVar3 == '\x02') {
                        std::sys::sync::mutex::futex::Mutex::wake();
                      }
                      break;
                    }
                    local_78 = (longlong **)
                               CONCAT44(local_78._4_4_,*(undefined4 *)pauVar28[uVar22 * -3 + -3]);
                    LOCK();
                    bVar30 = (char)*plVar21 == '\0';
                    if (bVar30) {
                      *(char *)plVar21 = '\x01';
                    }
                    UNLOCK();
                    if (!bVar30) {
                      parking_lot::raw_mutex::RawMutex::lock_slow();
                    }
                    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                    local_b8 = lVar19;
                    rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
                    local_62 = 1;
                    parking_lot_core::parking_lot::deadlock_impl::release_resource();
                    LOCK();
                    bVar30 = (char)*local_70 == '\x01';
                    if (bVar30) {
                      *(char *)local_70 = '\0';
                    }
                    UNLOCK();
                    if (!bVar30) {
                      local_62 = 1;
                      parking_lot::raw_mutex::RawMutex::unlock_slow();
                    }
                    if ((((ulonglong)local_3f8 & 3) != 0) || (((ulonglong)pplStack_3f0 & 7) != 0)) {
                      local_62 = 1;
                    // WARNING: Subroutine does not return
                      bytemuck::internal::something_went_wrong<bytemuck::PodCastError>();
                    }
                    uVar26 = *(ulonglong *)(pauVar28[uVar22 * -3 + -2] + 8);
                    uVar20 = (ulonglong)pplStack_3f0 >> 3;
                    if (uVar26 < (ulonglong)pplStack_3f0 >> 3) {
                      uVar20 = uVar26;
                    }
                    if (uVar20 != 0) {
                      pppplVar14 = *(longlong *****)pauVar28[uVar22 * -3 + -2];
                      if ((uVar20 < 6) ||
                         ((local_3f8 < (longlong ****)((longlong)pppplVar14 + uVar20 * 0xc) &&
                          (pppplVar14 < local_3f8 + uVar20)))) {
                        uVar26 = 0;
LAB_1408c76cd:
                        uVar27 = uVar26;
                        if ((uVar20 & 1) != 0) {
                          uVar4 = *(undefined4 *)((longlong)pppplVar14 + uVar26 * 0xc + 8);
                          *(undefined4 *)(local_3f8 + uVar26) =
                               *(undefined4 *)((longlong)pppplVar14 + uVar26 * 0xc);
                          *(undefined4 *)((longlong)local_3f8 + uVar26 * 8 + 4) = uVar4;
                          uVar27 = uVar26 | 1;
                        }
                        if (uVar20 != (uVar26 | 1)) {
                          puVar23 = (undefined4 *)((longlong)pppplVar14 + uVar27 * 0xc + 0x14);
                          do {
                            uVar4 = puVar23[-3];
                            *(undefined4 *)(local_3f8 + uVar27) = puVar23[-5];
                            *(undefined4 *)((longlong)local_3f8 + uVar27 * 8 + 4) = uVar4;
                            uVar4 = *puVar23;
                            *(undefined4 *)(local_3f8 + uVar27 + 1) = puVar23[-2];
                            *(undefined4 *)((longlong)local_3f8 + uVar27 * 8 + 0xc) = uVar4;
                            uVar27 = uVar27 + 2;
                            puVar23 = puVar23 + 6;
                          } while (uVar20 != uVar27);
                        }
                      }
                      else {
                        uVar26 = uVar20 & 0x1ffffffffffffffc;
                        puVar23 = (undefined4 *)((longlong)pppplVar14 + 0x2c);
                        uVar27 = 0;
                        do {
                          uVar12 = puVar23[-8];
                          uVar13 = puVar23[-6];
                          uVar4 = puVar23[-3];
                          uVar5 = puVar23[-5];
                          uVar6 = *puVar23;
                          uVar7 = puVar23[-2];
                          local_3f8[uVar27] = (longlong **)CONCAT44(puVar23[-9],puVar23[-0xb]);
                          (local_3f8 + uVar27)[1] = (longlong **)CONCAT44(uVar13,uVar12);
                          local_3f8[uVar27 + 2] = (longlong **)CONCAT44(uVar4,uVar5);
                          (local_3f8 + uVar27 + 2)[1] = (longlong **)CONCAT44(uVar6,uVar7);
                          uVar27 = uVar27 + 4;
                          puVar23 = puVar23 + 0xc;
                        } while (uVar26 != uVar27);
                        if (uVar20 != uVar26) goto LAB_1408c76cd;
                      }
                      uVar26 = *(ulonglong *)(pauVar28[uVar22 * -3 + -2] + 8);
                    }
                    uVar22 = 0;
                    if (uVar26 < 0x400) {
                      uVar22 = uVar26;
                    }
                    if (uVar26 == 0) {
                      local_62 = 1;
                    // WARNING: Subroutine does not return
                      core::panicking::panic();
                    }
                    local_62 = 1;
                    rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address();
                    local_62 = 1;
                    ::utils::pack_2x16f_uint();
                    uVar26 = local_98;
                    if (local_98 == local_a8) {
                      local_62 = 1;
                      alloc::raw_vec::RawVec::
                      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                                ();
                    }
                    lVar19 = uVar26 * 0x10;
                    *(undefined8 *)(local_a0 + lVar19) = extraout_RAX_04;
                    *(uint *)(local_a0 + lVar19 + 8) = (int)local_78 << 10 | (uint)uVar22 & 0x3ff;
                    *(undefined4 *)(local_a0 + lVar19 + 0xc) = extraout_EAX;
                    local_98 = uVar26 + 1;
                    local_228 = local_3b8;
                    local_238 = local_3c8;
                    puStack_230 = puStack_3c0;
                    local_248 = local_3d8;
                    uStack_240 = uStack_3d0;
                    local_258 = local_3e8;
                    uStack_250 = uStack_3e0;
                    local_268 = local_3f8;
                    pplStack_260 = pplStack_3f0;
                    local_62 = 0;
                    rhapsody::raw::dynamic_constants::DynamicConstantsChunk::
                    into_gpu_lifetime_tracker();
                    pplVar17 = local_80;
                    if (local_80 == local_90) {
                      alloc::raw_vec::RawVec::
                      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                                ();
                    }
                    lVar19 = (longlong)pplVar17 * 0x20;
                    puVar2 = (undefined8 *)(lStack_88 + 0x10 + lVar19);
                    *puVar2 = CONCAT44(uStack_414,local_418);
                    puVar2[1] = CONCAT44(uStack_40c,uStack_410);
                    puVar2 = (undefined8 *)(lStack_88 + lVar19);
                    *puVar2 = CONCAT44(uStack_424,local_428);
                    puVar2[1] = CONCAT44(uStack_41c,uStack_420);
                    local_80 = (longlong **)((longlong)pplVar17 + 1);
                    lVar19 = local_b8;
                    plVar21 = local_70;
                  }
                }
                LOCK();
                bVar30 = (char)*plVar21 == '\0';
                if (bVar30) {
                  *(char *)plVar21 = '\x01';
                }
                UNLOCK();
                if (!bVar30) {
                  parking_lot::raw_mutex::RawMutex::lock_slow();
                }
                parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                _Src = local_a0;
                _Size = (longlong ***)(local_98 << 4);
                rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
                if ((longlong ***)pplStack_260 != _Size) {
                    // WARNING: Subroutine does not return
                  core::slice::impl$0::copy_from_slice::len_mismatch_fail();
                }
                memcpy(local_268,_Src,(size_t)_Size);
                plVar21 = local_70;
                local_3f8 = local_268;
                pplStack_3f0 = pplStack_260;
                local_3e8 = local_258;
                uStack_3e0 = uStack_250;
                local_3d8 = (undefined *)local_248;
                uStack_3d0 = uStack_240;
                local_3c8 = local_238;
                puStack_3c0 = puStack_230;
                local_3b8 = local_228;
                local_63 = 1;
                parking_lot_core::parking_lot::deadlock_impl::release_resource();
                LOCK();
                bVar30 = (char)*plVar21 == '\x01';
                if (bVar30) {
                  *(char *)plVar21 = '\0';
                }
                UNLOCK();
                if (!bVar30) {
                  local_63 = 1;
                  parking_lot::raw_mutex::RawMutex::unlock_slow();
                }
                local_63 = 1;
                rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address();
                local_228 = local_3b8;
                local_238 = local_3c8;
                puStack_230 = puStack_3c0;
                local_248 = local_3d8;
                uStack_240 = uStack_3d0;
                local_258 = local_3e8;
                uStack_250 = uStack_3e0;
                local_268 = local_3f8;
                pplStack_260 = pplStack_3f0;
                local_63 = 0;
                rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker()
                ;
                pplVar17 = local_80;
                if (local_80 == local_90) {
                  alloc::raw_vec::RawVec::
                  grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                            ();
                }
                lVar19 = (longlong)pplVar17 * 0x20;
                puVar2 = (undefined8 *)(lStack_88 + 0x10 + lVar19);
                *puVar2 = CONCAT44(uStack_414,local_418);
                puVar2[1] = CONCAT44(uStack_40c,uStack_410);
                puVar2 = (undefined8 *)(lStack_88 + lVar19);
                *puVar2 = CONCAT44(uStack_424,local_428);
                puVar2[1] = CONCAT44(uStack_41c,uStack_420);
                local_80 = (longlong **)((longlong)pplVar17 + 1);
                if (local_d0 != 0) {
                  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
                  if (*extraout_RAX_07 == 0) {
                    local_61 = 1;
                    std::sys::thread_local::native::lazy::Storage::
                    initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                              ();
                    plVar21 = extraout_RAX_08;
                    if (extraout_RAX_08 == (longlong *)0x0) goto LAB_1408c8b0f;
                  }
                  else {
                    if ((int)*extraout_RAX_07 != 1) {
LAB_1408c8b0f:
                      local_61 = 1;
                    // WARNING: Subroutine does not return
                      std::thread::local::panic_access_error();
                    }
                    plVar21 = extraout_RAX_07 + 1;
                  }
                  if (*plVar21 != 0) {
                    local_61 = 1;
                    // WARNING: Subroutine does not return
                    core::cell::panic_already_borrowed();
                  }
                  *plVar21 = -1;
                  local_70 = plVar21;
                  puffin::ThreadProfiler::end_scope();
                  *local_70 = *local_70 + 1;
                }
                local_61 = 1;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<ref_<str_>_>();
                if (local_132 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                            ();
                }
                memcpy(&local_3f8,&local_268,400);
                if (local_2c2 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                            ();
                }
                memcpy(&local_268,&local_3f8,400);
                if (local_132 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                            ();
                }
                memcpy(&local_3f8,&local_268,400);
                local_5c8._0_1_ = 9;
                local_268._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_268,&local_3f8,400);
                local_5c8._0_1_ = 0xb;
                local_3f8._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_3f8,&local_268,400);
                local_5c8._0_1_ = 0x13;
                local_268._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_268,&local_3f8,400);
                local_5c8._0_1_ = 0xe;
                local_3f8 = (longlong ***)CONCAT44(local_3f8._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_3f8,&local_268,400);
                local_5c8 = (longlong ***)CONCAT71(local_5c8._1_7_,0xd);
                local_268 = (longlong ***)CONCAT44(local_268._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_268,&local_3f8,400);
                local_61 = 1;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                write_storage_buffer<country_core::render::normals_debug::NormalDebugSsbo>();
                local_258 = (ulonglong)local_80;
                local_268 = (longlong ***)local_90;
                pplStack_260 = (longlong **)lStack_88;
                local_61 = 0;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                dispatch_with<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke::closure_env_1>
                          ();
                local_61 = 0;
                rhapsody::raw::texture::TextureDesc::new_2d();
                local_3e8 = local_5b8;
                uStack_3e0 = uStack_5b0;
                local_3f8 = local_5c8;
                pplStack_3f0 = (longlong **)uStack_5c0;
                local_61 = 0;
                rhapsody::rg::resource::impl$4::from();
                local_268 = (longlong ***)&DAT_142aa3101;
                pplStack_260 = (longlong **)0x9;
                local_61 = 0;
                rhapsody::rg::graph::RenderGraph::create_raw_resource();
                uStack_424 = SUB84(local_5c8,0);
                uStack_420 = (undefined4)((ulonglong)local_5c8 >> 0x20);
                uStack_41c = (undefined4)uStack_5c0;
                local_418 = (undefined4)((ulonglong)uStack_5c0 >> 0x20);
                uStack_414 = (undefined4)local_5b8;
                uStack_410 = (undefined4)((ulonglong)local_5b8 >> 0x20);
                uStack_40c = (undefined4)uStack_5b0;
                uStack_408 = (undefined4)((ulonglong)uStack_5b0 >> 0x20);
                local_3f8 = (longlong ***)&DAT_142aa310a;
                pplStack_3f0 = (longlong **)0x7;
                local_3e8 = CONCAT71(local_3e8._1_7_,1);
                local_61 = 0;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur>
                          ();
                if (local_132 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                            ();
                }
                memcpy(&local_3f8,&local_268,400);
                if (local_2c2 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                            ();
                }
                memcpy(&local_268,&local_3f8,400);
                local_5c8 = (longlong ***)CONCAT71(local_5c8._1_7_,9);
                local_3f8 = (longlong ***)CONCAT44(local_3f8._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_3f8,&local_268,400);
                local_268 = (longlong ***)CONCAT44(local_268._4_4_,2);
                local_5c8 = (longlong ***)&local_428;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_268,&local_3f8,400);
                local_588 = local_1c8;
                uStack_580 = uStack_1c0;
                local_598 = local_1d8;
                uStack_590 = uStack_1d0;
                local_5a8 = local_1e8;
                uStack_5a0 = uStack_1e0;
                local_5b8 = uStack_1f8;
                uStack_5b0 = uStack_1f0;
                local_5c8 = local_208;
                uStack_5c0 = uStack_200;
                local_3e8 = local_258;
                local_3f8 = local_268;
                pplStack_3f0 = pplStack_260;
                local_3c8 = uStack_250;
                puStack_3c0 = (undefined *)local_248;
                local_3b8 = uStack_240;
                uStack_3e0 = local_238;
                local_3d8 = puStack_230;
                uStack_3d0 = local_228;
                local_380 = local_1b8;
                uStack_378 = uStack_1b0;
                local_370 = local_1a8;
                uStack_368 = uStack_1a0;
                local_360 = local_198;
                uStack_358 = uStack_190;
                local_350 = local_188;
                uStack_348 = uStack_180;
                local_340 = local_178;
                uStack_338 = uStack_170;
                local_330 = local_168;
                uStack_328 = uStack_160;
                local_3a0 = local_210;
                local_3b0 = CONCAT44(uStack_21c,local_220);
                ppuStack_3a8 = ppuStack_218;
                local_318 = local_e8;
                local_320 = local_130;
                local_398 = 0x20000000001;
                uStack_390 = 0x100000200;
                rhapsody::rg::pass_builder::PassBuilder::
                render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                          ();
                if (local_132 != '\b') {
                  local_61 = 0;
                  rhapsody::raw::managed::handle::impl$6::drop();
                }
                local_3f8 = (longlong ***)&DAT_142aa3124;
                pplStack_3f0 = (longlong **)0x3;
                local_3e8 = local_3e8 & 0xffffffffffffff00;
                local_61 = 0;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur>
                          ();
                if (local_132 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                            ();
                }
                memcpy(&local_3f8,&local_268,400);
                if (local_2c2 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                            ();
                }
                memcpy(&local_268,&local_3f8,400);
                local_3f8 = (longlong ***)CONCAT44(local_3f8._4_4_,2);
                local_5c8 = (longlong ***)&local_428;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_3f8,&local_268,400);
                local_5c8 = (longlong ***)CONCAT71(local_5c8._1_7_,9);
                local_268 = (longlong ***)CONCAT44(local_268._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_268,&local_3f8,400);
                local_588 = local_1c8;
                uStack_580 = uStack_1c0;
                local_598 = local_1d8;
                uStack_590 = uStack_1d0;
                local_5a8 = local_1e8;
                uStack_5a0 = uStack_1e0;
                local_5b8 = uStack_1f8;
                uStack_5b0 = uStack_1f0;
                local_5c8 = local_208;
                uStack_5c0 = uStack_200;
                local_3e8 = local_258;
                local_3f8 = local_268;
                pplStack_3f0 = pplStack_260;
                local_3c8 = uStack_250;
                puStack_3c0 = (undefined *)local_248;
                local_3b8 = uStack_240;
                uStack_3e0 = local_238;
                local_3d8 = puStack_230;
                uStack_3d0 = local_228;
                local_380 = local_1b8;
                uStack_378 = uStack_1b0;
                local_370 = local_1a8;
                uStack_368 = uStack_1a0;
                local_360 = local_198;
                uStack_358 = uStack_190;
                local_350 = local_188;
                uStack_348 = uStack_180;
                local_340 = local_178;
                uStack_338 = uStack_170;
                local_330 = local_168;
                uStack_328 = uStack_160;
                local_3a0 = local_210;
                local_3b0 = CONCAT44(uStack_21c,local_220);
                ppuStack_3a8 = ppuStack_218;
                local_318 = local_e8;
                local_320 = local_130;
                local_398 = 0x20000000001;
                uStack_390 = 0x100000200;
                rhapsody::rg::pass_builder::PassBuilder::
                render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                          ();
                if (local_132 != '\b') {
                  local_61 = 0;
                  rhapsody::raw::managed::handle::impl$6::drop();
                }
                local_61 = 0;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<ref_<str_>_>();
                local_5c8 = (longlong ***)CONCAT71(local_5c8._1_7_,9);
                local_268._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_268,&local_3f8,400);
                local_d0 = CONCAT71(local_d0._1_7_,10);
                local_5c8 = (longlong ***)CONCAT44(local_5c8._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_5c8,&local_268,400);
                local_3f8 = (longlong ***)CONCAT71(local_3f8._1_7_,0xe);
                local_268 = (longlong ***)CONCAT44(local_268._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_268,&local_5c8,400);
                local_588 = local_1c8;
                uStack_580 = uStack_1c0;
                local_598 = local_1d8;
                uStack_590 = uStack_1d0;
                local_5a8 = local_1e8;
                uStack_5a0 = uStack_1e0;
                local_5b8 = uStack_1f8;
                uStack_5b0 = uStack_1f0;
                local_5c8 = local_208;
                uStack_5c0 = uStack_200;
                local_3e8 = local_258;
                local_3f8 = local_268;
                pplStack_3f0 = pplStack_260;
                local_3c8 = uStack_250;
                puStack_3c0 = (undefined *)local_248;
                local_3b8 = uStack_240;
                uStack_3e0 = local_238;
                local_3d8 = puStack_230;
                uStack_3d0 = local_228;
                local_380 = local_1b8;
                uStack_378 = uStack_1b0;
                local_370 = local_1a8;
                uStack_368 = uStack_1a0;
                local_360 = local_198;
                uStack_358 = uStack_190;
                local_350 = local_188;
                uStack_348 = uStack_180;
                local_340 = local_178;
                uStack_338 = uStack_170;
                local_330 = local_168;
                uStack_328 = uStack_160;
                local_3a0 = local_210;
                local_3b0 = CONCAT44(uStack_21c,local_220);
                ppuStack_3a8 = ppuStack_218;
                local_318 = local_e8;
                local_320 = local_130;
                local_398 = 0x20000000001;
                uStack_390 = 0x100000200;
                rhapsody::rg::pass_builder::PassBuilder::
                render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                          ();
                if (local_132 != '\b') {
                  local_61 = 0;
                  rhapsody::raw::managed::handle::impl$6::drop();
                }
                *(undefined1 *)(local_c0 + 0x40) = 1;
                if (local_a8 != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
              }
              return;
            }
          }
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


