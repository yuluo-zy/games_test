
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::
rasterize_terrain_stroke(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  char *pcVar3;
  char cVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined1 auVar9 [16];
  byte bVar10;
  byte bVar11;
  byte bVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  longlong ****pppplVar15;
  ushort uVar16;
  uint uVar17;
  undefined8 uVar18;
  float fVar19;
  undefined8 uVar20;
  undefined1 *_Src;
  longlong **pplVar21;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar22;
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
  longlong lVar23;
  ulonglong uVar24;
  longlong *plVar25;
  longlong in_RDX;
  ulonglong uVar26;
  undefined4 *puVar27;
  undefined1 (*pauVar28) [16];
  longlong ***_Size;
  undefined *puVar29;
  ulonglong uVar30;
  ulonglong uVar31;
  undefined1 (*pauVar32) [16];
  uint uVar33;
  bool bVar34;
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
  uVar26 = 0x47c94cb78283a72e;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar26 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar26 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar26 * 8), uVar26 != 0)) {
    lVar23 = ~uVar26 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar23) != 0) {
      plVar25 = *(longlong **)(*(longlong *)(in_RDX + 0x1e0) + lVar23 + 0x10);
      uVar26 = 0x1e38c16726d0fb8d;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_00 != 0) && (uVar26 < *(ulonglong *)(in_RDX + 0x218))) &&
         (uVar26 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar26 * 8), uVar26 != 0)) {
        lVar23 = ~uVar26 * 0x60;
        if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar23) != 0) {
          lVar23 = *(longlong *)(in_RDX + 0x1e0) + lVar23;
          local_c0 = *(longlong *)(lVar23 + 0x10);
          *(undefined4 *)(lVar23 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
          uVar26 = 0xcda6af2f6afb130e;
          bevy_ecs::component::Components::get_resource_id();
          if (((extraout_RAX_01 != 0) && (uVar26 < *(ulonglong *)(in_RDX + 0x218))) &&
             (uVar26 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar26 * 8), uVar26 != 0)) {
            lVar23 = ~uVar26 * 0x60;
            if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar23) != 0) {
              lVar23 = *(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar23 + 0x10);
              if ((*(longlong *)(lVar23 + 0x30) != 0 || *(longlong *)(lVar23 + 0x10) != 0) ||
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
                  puVar29 = &DAT_142aa2f07;
                  do {
                    bVar10 = puVar29[-1];
                    uVar22 = (uint)(char)bVar10;
                    if ((char)bVar10 < '\0') {
                      bVar11 = puVar29[-2];
                      if ((char)bVar11 < -0x40) {
                        bVar12 = puVar29[-3];
                        if ((char)bVar12 < -0x40) {
                          pbVar1 = puVar29 + -4;
                          puVar29 = puVar29 + -4;
                          uVar22 = bVar12 & 0x3f | (*pbVar1 & 7) << 6;
                        }
                        else {
                          puVar29 = puVar29 + -3;
                          uVar22 = bVar12 & 0xf;
                        }
                        uVar22 = bVar11 & 0x3f | uVar22 << 6;
                      }
                      else {
                        puVar29 = puVar29 + -2;
                        uVar22 = bVar11 & 0x1f;
                      }
                      uVar22 = (uint)(bVar10 & 0x3f) | uVar22 << 6;
                      if (uVar22 != 0x5c) goto LAB_1408c73a4;
LAB_1408c73bc:
                      if (puVar29 == &UNK_142aa2eb0) break;
                      if (puVar29 + -0x142aa2eb0 < (undefined *)0x56) {
                        if (-0x41 < (char)puVar29[1]) break;
                      }
                      else if (puVar29 + -0x142aa2eb0 == (undefined *)0x56) break;
                      local_61 = 1;
                    /* WARNING: Subroutine does not return */
                      core::str::slice_error_fail();
                    }
                    puVar29 = puVar29 + -1;
                    if (uVar22 == 0x5c) goto LAB_1408c73bc;
LAB_1408c73a4:
                    if (uVar22 == 0x2f) goto LAB_1408c73bc;
                  } while (puVar29 != &DAT_142aa2eb1);
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
                    /* WARNING: Subroutine does not return */
                      std::thread::local::panic_access_error();
                    }
                    local_78 = (longlong **)(extraout_RAX_02 + 1);
                  }
                  if ((longlong **)*local_78 != (longlong **)0x0) {
                    local_61 = 1;
                    /* WARNING: Subroutine does not return */
                    core::cell::panic_already_borrowed();
                  }
                  *local_78 = (longlong *)0xffffffffffffffff;
                  puffin::ThreadProfiler::begin_scope();
                  *local_78 = (longlong *)((longlong)*local_78 + 1);
                  local_d0 = 1;
                }
                fVar19 = __real_3c23d70a;
                pauVar32 = *(undefined1 (**) [16])(local_c0 + 8);
                auVar9 = *pauVar32;
                pauVar28 = pauVar32 + 1;
                local_400 = plVar25 + 1;
                lVar23 = *(longlong *)(local_c0 + 0x20);
                uVar22 = ~(uint)(ushort)((ushort)(SUB161(auVar9 >> 7,0) & 1) |
                                         (ushort)(SUB161(auVar9 >> 0xf,0) & 1) << 1 |
                                         (ushort)(SUB161(auVar9 >> 0x17,0) & 1) << 2 |
                                         (ushort)(SUB161(auVar9 >> 0x1f,0) & 1) << 3 |
                                         (ushort)(SUB161(auVar9 >> 0x27,0) & 1) << 4 |
                                         (ushort)(SUB161(auVar9 >> 0x2f,0) & 1) << 5 |
                                         (ushort)(SUB161(auVar9 >> 0x37,0) & 1) << 6 |
                                         (ushort)(SUB161(auVar9 >> 0x3f,0) & 1) << 7 |
                                         (ushort)(SUB161(auVar9 >> 0x47,0) & 1) << 8 |
                                         (ushort)(SUB161(auVar9 >> 0x4f,0) & 1) << 9 |
                                         (ushort)(SUB161(auVar9 >> 0x57,0) & 1) << 10 |
                                         (ushort)(SUB161(auVar9 >> 0x5f,0) & 1) << 0xb |
                                         (ushort)(SUB161(auVar9 >> 0x67,0) & 1) << 0xc |
                                         (ushort)(SUB161(auVar9 >> 0x6f,0) & 1) << 0xd |
                                         (ushort)(SUB161(auVar9 >> 0x77,0) & 1) << 0xe |
                                        (ushort)(byte)(auVar9[0xf] >> 7) << 0xf);
                local_70 = plVar25;
                while (lVar23 != 0) {
                  if ((short)uVar22 == 0) {
                    do {
                      auVar9 = *pauVar28;
                      uVar16 = (ushort)(SUB161(auVar9 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar9 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar9 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar9 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar9 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar9 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar9 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar9 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar9 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar9 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar9 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar9 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar9 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar9 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar9 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar9[0xf] >> 7) << 0xf;
                      pauVar32 = pauVar32 + -0x30;
                      pauVar28 = pauVar28 + 1;
                    } while (uVar16 == 0xffff);
                    uVar22 = ~(uint)uVar16;
                    uVar33 = uVar22 & -(uint)uVar16 - 2;
                  }
                  else {
                    uVar33 = uVar22 - 1 & uVar22;
                  }
                  lVar23 = lVar23 + -1;
                  uVar17 = 0;
                  for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
                    uVar17 = uVar17 + 1;
                  }
                  uVar26 = (ulonglong)uVar17;
                  uVar22 = uVar33;
                  if ((fVar19 < *(float *)(pauVar32[uVar26 * -3 + -1] + 8)) &&
                     (*(longlong *)(pauVar32[uVar26 * -3 + -2] + 8) != 0)) {
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
                    /* WARNING: Subroutine does not return */
                        core::result::unwrap_failed();
                      }
                      local_78 = pplStack_260;
                      local_b8 = CONCAT71(local_b8._1_7_,(undefined1)local_258);
                      std::alloc::__default_lib_allocator::__rust_alloc();
                      if (extraout_RAX_05 == (longlong ***)0x0) {
                    /* WARNING: Subroutine does not return */
                        alloc::raw_vec::handle_error();
                      }
                      builtin_strncpy((char *)((longlong)extraout_RAX_05 + 7),"path! ()",8);
                      *extraout_RAX_05 = (longlong **)0x705f656c75646f6d;
                      local_268 = (longlong ***)&DAT_0000000f;
                      local_258 = 0xf;
                      pplStack_260 = (longlong **)extraout_RAX_05;
                      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>()
                      ;
                      uVar18 = _UNK_142aa300b;
                      *(undefined8 *)((longlong)pplStack_260 + local_258) = _DAT_142aa3003;
                      *(undefined8 *)((char *)((longlong)pplStack_260 + local_258) + 8) = uVar18;
                      builtin_strncpy((char *)((longlong)pplStack_260 + local_258 + 0x10),":: Error"
                                      ,8);
                      local_3e8 = local_258 + 0x18;
                      local_3f8 = local_268;
                      pplStack_3f0 = pplStack_260;
                      if ((longlong)local_268 - local_3e8 < 0x1f) {
                        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
                                  ();
                      }
                      uVar18 = s_terrain_strokes_142aa302a._8_8_;
                      pcVar3 = (char *)((longlong)pplStack_3f0 + local_3e8 + 0xf);
                      *(ulonglong *)pcVar3 =
                           CONCAT71(s_terrain_strokes_142aa302a._1_7_,s_terrain_strokes_142aa302a[0]
                                   );
                      *(undefined8 *)(pcVar3 + 8) = uVar18;
                      uVar18 = CONCAT17(s_terrain_strokes_142aa302a[0],_UNK_142aa3023);
                      *(undefined8 *)((longlong)pplStack_3f0 + local_3e8) = _DAT_142aa301b;
                      *(undefined8 *)((char *)((longlong)pplStack_3f0 + local_3e8) + 8) = uVar18;
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
                      pplVar21 = local_78;
                      if ((((char)local_b8 == '\0') &&
                          ((*(ulonglong *)
                             __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
                           & 0x7fffffffffffffff) != 0)) &&
                         (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0'))
                      {
                        *(char *)((longlong)pplVar21 + 1) = '\x01';
                      }
                      LOCK();
                      cVar4 = *(char *)pplVar21;
                      *(char *)pplVar21 = '\0';
                      UNLOCK();
                      if (cVar4 == '\x02') {
                        std::sys::sync::mutex::futex::Mutex::wake();
                      }
                      break;
                    }
                    local_78 = (longlong **)
                               CONCAT44(local_78._4_4_,*(undefined4 *)pauVar32[uVar26 * -3 + -3]);
                    LOCK();
                    bVar34 = (char)*plVar25 == '\0';
                    if (bVar34) {
                      *(char *)plVar25 = '\x01';
                    }
                    UNLOCK();
                    if (!bVar34) {
                      parking_lot::raw_mutex::RawMutex::lock_slow();
                    }
                    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                    local_b8 = lVar23;
                    rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
                    local_62 = 1;
                    parking_lot_core::parking_lot::deadlock_impl::release_resource();
                    LOCK();
                    bVar34 = (char)*local_70 == '\x01';
                    if (bVar34) {
                      *(char *)local_70 = '\0';
                    }
                    UNLOCK();
                    if (!bVar34) {
                      local_62 = 1;
                      parking_lot::raw_mutex::RawMutex::unlock_slow();
                    }
                    if ((((ulonglong)local_3f8 & 3) != 0) || (((ulonglong)pplStack_3f0 & 7) != 0)) {
                      local_62 = 1;
                    /* WARNING: Subroutine does not return */
                      bytemuck::internal::something_went_wrong<bytemuck::PodCastError>();
                    }
                    uVar30 = *(ulonglong *)(pauVar32[uVar26 * -3 + -2] + 8);
                    uVar24 = (ulonglong)pplStack_3f0 >> 3;
                    if (uVar30 < (ulonglong)pplStack_3f0 >> 3) {
                      uVar24 = uVar30;
                    }
                    if (uVar24 != 0) {
                      pppplVar15 = *(longlong *****)pauVar32[uVar26 * -3 + -2];
                      if ((uVar24 < 6) ||
                         ((local_3f8 < (longlong ****)((longlong)pppplVar15 + uVar24 * 0xc) &&
                          (pppplVar15 < local_3f8 + uVar24)))) {
                        uVar30 = 0;
LAB_1408c76cd:
                        uVar31 = uVar30;
                        if ((uVar24 & 1) != 0) {
                          uVar5 = *(undefined4 *)((longlong)pppplVar15 + uVar30 * 0xc + 8);
                          *(undefined4 *)(local_3f8 + uVar30) =
                               *(undefined4 *)((longlong)pppplVar15 + uVar30 * 0xc);
                          *(undefined4 *)((longlong)local_3f8 + uVar30 * 8 + 4) = uVar5;
                          uVar31 = uVar30 | 1;
                        }
                        if (uVar24 != (uVar30 | 1)) {
                          puVar27 = (undefined4 *)((longlong)pppplVar15 + uVar31 * 0xc + 0x14);
                          do {
                            uVar5 = puVar27[-3];
                            *(undefined4 *)(local_3f8 + uVar31) = puVar27[-5];
                            *(undefined4 *)((longlong)local_3f8 + uVar31 * 8 + 4) = uVar5;
                            uVar5 = *puVar27;
                            *(undefined4 *)(local_3f8 + uVar31 + 1) = puVar27[-2];
                            *(undefined4 *)((longlong)local_3f8 + uVar31 * 8 + 0xc) = uVar5;
                            uVar31 = uVar31 + 2;
                            puVar27 = puVar27 + 6;
                          } while (uVar24 != uVar31);
                        }
                      }
                      else {
                        uVar30 = uVar24 & 0x1ffffffffffffffc;
                        puVar27 = (undefined4 *)((longlong)pppplVar15 + 0x2c);
                        uVar31 = 0;
                        do {
                          uVar13 = puVar27[-8];
                          uVar14 = puVar27[-6];
                          uVar5 = puVar27[-3];
                          uVar6 = puVar27[-5];
                          uVar7 = *puVar27;
                          uVar8 = puVar27[-2];
                          local_3f8[uVar31] = (longlong **)CONCAT44(puVar27[-9],puVar27[-0xb]);
                          (local_3f8 + uVar31)[1] = (longlong **)CONCAT44(uVar14,uVar13);
                          local_3f8[uVar31 + 2] = (longlong **)CONCAT44(uVar5,uVar6);
                          (local_3f8 + uVar31 + 2)[1] = (longlong **)CONCAT44(uVar7,uVar8);
                          uVar31 = uVar31 + 4;
                          puVar27 = puVar27 + 0xc;
                        } while (uVar30 != uVar31);
                        if (uVar24 != uVar30) goto LAB_1408c76cd;
                      }
                      uVar30 = *(ulonglong *)(pauVar32[uVar26 * -3 + -2] + 8);
                    }
                    uVar26 = 0;
                    if (uVar30 < 0x400) {
                      uVar26 = uVar30;
                    }
                    if (uVar30 == 0) {
                      local_62 = 1;
                    /* WARNING: Subroutine does not return */
                      core::panicking::panic();
                    }
                    local_62 = 1;
                    rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address();
                    local_62 = 1;
                    ::utils::pack_2x16f_uint();
                    uVar30 = local_98;
                    if (local_98 == local_a8) {
                      local_62 = 1;
                      alloc::raw_vec::RawVec::
                      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                                ();
                    }
                    lVar23 = uVar30 * 0x10;
                    *(undefined8 *)(local_a0 + lVar23) = extraout_RAX_04;
                    *(uint *)(local_a0 + lVar23 + 8) = (int)local_78 << 10 | (uint)uVar26 & 0x3ff;
                    *(undefined4 *)(local_a0 + lVar23 + 0xc) = extraout_EAX;
                    local_98 = uVar30 + 1;
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
                    pplVar21 = local_80;
                    if (local_80 == local_90) {
                      alloc::raw_vec::RawVec::
                      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                                ();
                    }
                    lVar23 = (longlong)pplVar21 * 0x20;
                    puVar2 = (undefined8 *)(lStack_88 + 0x10 + lVar23);
                    *puVar2 = CONCAT44(uStack_414,local_418);
                    puVar2[1] = CONCAT44(uStack_40c,uStack_410);
                    puVar2 = (undefined8 *)(lStack_88 + lVar23);
                    *puVar2 = CONCAT44(uStack_424,local_428);
                    puVar2[1] = CONCAT44(uStack_41c,uStack_420);
                    local_80 = (longlong **)((longlong)pplVar21 + 1);
                    lVar23 = local_b8;
                    plVar25 = local_70;
                  }
                }
                LOCK();
                bVar34 = (char)*plVar25 == '\0';
                if (bVar34) {
                  *(char *)plVar25 = '\x01';
                }
                UNLOCK();
                if (!bVar34) {
                  parking_lot::raw_mutex::RawMutex::lock_slow();
                }
                parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                _Src = local_a0;
                _Size = (longlong ***)(local_98 << 4);
                rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
                if ((longlong ***)pplStack_260 != _Size) {
                    /* WARNING: Subroutine does not return */
                  core::slice::impl$0::copy_from_slice::len_mismatch_fail();
                }
                memcpy(local_268,_Src,(size_t)_Size);
                plVar25 = local_70;
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
                bVar34 = (char)*plVar25 == '\x01';
                if (bVar34) {
                  *(char *)plVar25 = '\0';
                }
                UNLOCK();
                if (!bVar34) {
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
                pplVar21 = local_80;
                if (local_80 == local_90) {
                  alloc::raw_vec::RawVec::
                  grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                            ();
                }
                lVar23 = (longlong)pplVar21 * 0x20;
                puVar2 = (undefined8 *)(lStack_88 + 0x10 + lVar23);
                *puVar2 = CONCAT44(uStack_414,local_418);
                puVar2[1] = CONCAT44(uStack_40c,uStack_410);
                puVar2 = (undefined8 *)(lStack_88 + lVar23);
                *puVar2 = CONCAT44(uStack_424,local_428);
                puVar2[1] = CONCAT44(uStack_41c,uStack_420);
                local_80 = (longlong **)((longlong)pplVar21 + 1);
                if (local_d0 != 0) {
                  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
                  if (*extraout_RAX_07 == 0) {
                    local_61 = 1;
                    std::sys::thread_local::native::lazy::Storage::
                    initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                              ();
                    plVar25 = extraout_RAX_08;
                    if (extraout_RAX_08 == (longlong *)0x0) goto LAB_1408c8b0f;
                  }
                  else {
                    if ((int)*extraout_RAX_07 != 1) {
LAB_1408c8b0f:
                      local_61 = 1;
                    /* WARNING: Subroutine does not return */
                      std::thread::local::panic_access_error();
                    }
                    plVar25 = extraout_RAX_07 + 1;
                  }
                  if (*plVar25 != 0) {
                    local_61 = 1;
                    /* WARNING: Subroutine does not return */
                    core::cell::panic_already_borrowed();
                  }
                  *plVar25 = -1;
                  local_70 = plVar25;
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
                uVar20 = _UNK_142aa2508;
                uVar18 = ___xmm_00000001000002000000020000000001;
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
                local_398 = ___xmm_00000001000002000000020000000001;
                uStack_390 = _UNK_142aa2508;
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
                local_398 = uVar18;
                uStack_390 = uVar20;
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
                local_398 = uVar18;
                uStack_390 = uVar20;
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
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

