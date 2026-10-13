
void __cdecl
country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter(void)

{
  byte *pbVar1;
  ulonglong *puVar2;
  undefined4 uVar3;
  byte bVar4;
  undefined8 uVar5;
  undefined4 *puVar6;
  undefined1 (*_Dst) [16];
  ulonglong uVar7;
  float *pfVar8;
  int iVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  bool bVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  char extraout_AL;
  uint uVar18;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  uint *puVar19;
  ulonglong extraout_RAX_02;
  undefined8 **ppuVar20;
  undefined8 *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong extraout_RAX_07;
  undefined8 *in_RCX;
  longlong *plVar21;
  ulonglong uVar22;
  char *pcVar23;
  ulonglong uVar24;
  byte bVar25;
  undefined8 *puVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  ulonglong uVar30;
  longlong lVar31;
  uint uVar32;
  char *pcVar33;
  longlong lVar34;
  ushort uVar35;
  longlong lVar36;
  ulonglong uVar37;
  uint *puVar38;
  undefined1 (*pauVar39) [16];
  ulonglong uVar40;
  undefined1 (*pauVar41) [16];
  uint *puVar42;
  byte bVar43;
  char cVar44;
  char cVar46;
  char cVar47;
  char cVar48;
  undefined1 auVar45 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
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
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000058;
  ulonglong local_2c0;
  undefined8 local_2b8;
  undefined8 local_2b0;
  undefined4 *local_2a8;
  ulonglong local_2a0;
  longlong local_298;
  longlong local_290;
  longlong local_288;
  longlong local_280;
  undefined8 local_278;
  undefined8 local_270;
  uint *local_268;
  uint local_260;
  longlong local_250;
  undefined8 *local_248;
  ulonglong uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  ulonglong local_218;
  ulonglong uStack_210;
  undefined8 local_208;
  ulonglong uStack_200;
  ulonglong *local_1f0;
  code *local_1e8;
  longlong local_1b0;
  undefined **local_198;
  ulonglong uStack_190;
  ulonglong **local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  ulonglong uStack_160;
  undefined8 local_158;
  ulonglong uStack_150;
  undefined4 local_138;
  undefined1 local_120;
  undefined1 local_110;
  float local_10c;
  undefined2 local_f0;
  undefined4 local_d4;
  longlong *local_d0;
  longlong *local_c0;
  undefined8 **local_b8;
  undefined8 *local_b0;
  ulonglong *local_a8;
  longlong *local_a0;
  char local_91;
  longlong local_90;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  uVar5 = *in_stack_00000038;
  puVar6 = (undefined4 *)in_stack_00000038[2];
  local_d4 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  ppuVar20 = (undefined8 **)in_stack_00000038[4];
  local_90 = in_stack_00000038[8];
  lVar31 = in_stack_00000038[9];
  local_2b8 = in_stack_00000038[0xb];
  local_278 = *in_stack_00000058;
  local_270 = in_stack_00000058[4];
  local_268 = (uint *)in_stack_00000058[6];
  local_260 = *(uint *)((longlong)in_stack_00000058 + 0x3c);
  pcVar23 = (char *)(ulonglong)local_260;
  local_a8 = *(ulonglong **)(in_stack_00000030 + 0x20);
  local_1b0 = *(longlong *)(in_stack_00000030 + 0x28);
  *(undefined4 *)in_stack_00000038[6] = *(undefined4 *)((longlong)in_stack_00000038 + 0x3c);
  local_b0 = in_RCX;
  puffin::are_scopes_on();
  local_a0 = (longlong *)CONCAT71(local_a0._1_7_,extraout_AL);
  local_2b0 = uVar5;
  local_2a8 = puVar6;
  if (extraout_AL == '\0') {
    local_1f0 = (ulonglong *)0x0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    pcVar33 = 
    "country_core::systems::ui_clutter::ui_autoclutter::maintain_autoclutter_parent_hashmap::f";
    if (extraout_RAX == 1) {
      if (pcVar23 != (char *)0x0) {
        if (pcVar23 < (char *)0x56) {
          if ((char)(&DAT_142abb980)[(longlong)pcVar23] < -0x40) {
LAB_14099cbf4:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (pcVar23 != (char *)0x56) goto LAB_14099cbf4;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (pcVar23 != (char *)0xfffffffffffffffe) {
          if (pcVar23 + 2 < (char *)0x56) {
            if ("untry_core::systems::ui_clutter::ui_autoclutter::maintain_autoclutter_parent_hashmap::f"
                [(longlong)pcVar23] < -0x40) {
LAB_14099cc48:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (pcVar23 + 2 != (char *)0x56) goto LAB_14099cc48;
        }
        pcVar33 = "untry_core::systems::ui_clutter::ui_autoclutter::maintain_autoclutter_parent_hashmap::f"
                  + (longlong)pcVar23;
      }
    }
    pcVar23 = pcVar33;
    pcVar33 = "auto_clutter_";
    do {
      bVar43 = pcVar33[-1];
      uVar18 = (uint)(char)bVar43;
      if ((char)bVar43 < '\0') {
        bVar25 = pcVar33[-2];
        if ((char)bVar25 < -0x40) {
          bVar4 = pcVar33[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = (byte *)(pcVar33 + -4);
            pcVar33 = pcVar33 + -4;
            uVar18 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar33 = pcVar33 + -3;
            uVar18 = bVar4 & 0xf;
          }
          uVar18 = bVar25 & 0x3f | uVar18 << 6;
        }
        else {
          pcVar33 = pcVar33 + -2;
          uVar18 = bVar25 & 0x1f;
        }
        uVar18 = (uint)(bVar43 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto LAB_14099cd04;
LAB_14099cd15:
        if (pcVar33 == "pcrates/country-core/src/systems/ui_clutter/ui_autoclutter.rsauto_clutter_")
        break;
        if (pcVar33 + -0x142abbaec < (char *)0x3c) {
          if (-0x41 < pcVar33[1]) break;
        }
        else if (pcVar33 + -0x142abbaec == (char *)0x3c) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      pcVar33 = pcVar33 + -1;
      if (uVar18 == 0x5c) goto LAB_14099cd15;
LAB_14099cd04:
      if (uVar18 == 0x2f) goto LAB_14099cd15;
    } while (pcVar33 != "crates/country-core/src/systems/ui_clutter/ui_autoclutter.rsauto_clutter_")
    ;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar21 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14099da81;
    if (*plVar21 != 0) goto LAB_14099dd7e;
    *plVar21 = -1;
    local_b8 = (undefined8 **)plVar21;
    puffin::ThreadProfiler::begin_scope();
    *local_b8 = (undefined8 *)((longlong)*local_b8 + 1);
    local_1f0 = (ulonglong *)0x1;
  }
  lVar29 = (longlong)ppuVar20[3];
  local_b8 = ppuVar20;
  if (lVar29 != 0) {
    _Dst = (undefined1 (*) [16])*ppuVar20;
    auVar45 = *_Dst;
    uVar18 = ~(uint)(ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar45[0xf] >> 7) << 0xf);
    pauVar39 = _Dst + 1;
    pauVar41 = _Dst;
    do {
      if ((short)uVar18 == 0) {
        do {
          auVar45 = *pauVar39;
          pauVar41 = pauVar41 + -0x20;
          pauVar39 = pauVar39 + 1;
          uVar18 = (uint)(ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar45[0xf] >> 7) << 0xf);
        } while (uVar18 == 0xffff);
        uVar18 = ~uVar18;
      }
      iVar9 = 0;
      for (uVar32 = uVar18; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
        iVar9 = iVar9 + 1;
      }
      lVar36 = *(longlong *)((longlong)pauVar41 + (-0x18 - (ulonglong)(uint)(iVar9 << 5)));
      if (lVar36 != 0) {
        pcVar23 = (char *)(lVar36 * 0x60);
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      ppuVar20 = local_b8;
      uVar18 = uVar18 - 1 & uVar18;
      lVar29 = lVar29 + -1;
    } while (lVar29 != 0);
    uVar22 = (ulonglong)local_b8[1];
    if (uVar22 != 0) {
      memset(_Dst,(int)CONCAT71((int7)((ulonglong)pcVar23 >> 8),0xff),uVar22 + 0x11);
    }
    ppuVar20[3] = (undefined8 *)0x0;
    uVar40 = (uVar22 + 1 & 0xfffffffffffffff8) - (uVar22 + 1 >> 3);
    if (uVar22 < 8) {
      uVar40 = uVar22;
    }
    ppuVar20[2] = (undefined8 *)uVar40;
  }
  puVar38 = *(uint **)(local_90 + 0xf8);
  puVar42 = puVar38 + *(longlong *)(local_90 + 0x100);
  cVar44 = *(char *)(local_90 + 0x108);
  plVar21 = (longlong *)(ppuVar20 + 4);
  lVar34 = 8;
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  lVar29 = 0;
  lVar36 = 0;
  lVar28 = 0;
  uVar22 = 0;
  uVar40 = 0;
  if (cVar44 != '\0') goto LAB_14099cf4e;
LAB_14099d490:
  if (uVar40 != uVar22) goto LAB_14099d5ca;
  do {
    if (puVar38 == puVar42) goto LAB_14099d646;
    uVar18 = *puVar38;
    puVar38 = puVar38 + 1;
    uVar22 = *(ulonglong *)(*(longlong *)(lVar31 + 0x100) + 0x58 + (ulonglong)uVar18 * 0xa0);
  } while (uVar22 == 0);
  lVar34 = *(longlong *)(lVar31 + 0x100) + (ulonglong)uVar18 * 0xa0;
  uVar40 = (ulonglong)*(uint *)(lVar34 + 0x94);
  lVar28 = *(longlong *)(lVar31 + 0x1a8) + uVar40 * 0x48;
  uVar40 = *(ulonglong *)(*(longlong *)(lVar31 + 0x1a8) + 0x40 + uVar40 * 0x48);
  if ((*(ulonglong *)(local_90 + 0x110) < uVar40) &&
     (uVar37 = *(ulonglong *)(*(longlong *)(lVar28 + 0x38) + *(ulonglong *)(local_90 + 0x110) * 8),
     uVar37 != 0)) {
    lVar29 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar37 * 0x30);
    uVar37 = *(ulonglong *)(local_90 + 0x118);
    if (uVar40 <= uVar37) goto LAB_14099d52e;
LAB_14099d55c:
    uVar37 = *(ulonglong *)(*(longlong *)(lVar28 + 0x38) + uVar37 * 8);
    if (uVar37 == 0) {
      lVar36 = 0;
    }
    else {
      lVar36 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar37 * 0x30);
    }
    uVar37 = *(ulonglong *)(local_90 + 0x120);
    if (uVar37 < uVar40) goto LAB_14099d59b;
LAB_14099d5be:
    lVar28 = 0;
  }
  else {
    lVar29 = 0;
    uVar37 = *(ulonglong *)(local_90 + 0x118);
    if (uVar37 < uVar40) goto LAB_14099d55c;
LAB_14099d52e:
    lVar36 = 0;
    uVar37 = *(ulonglong *)(local_90 + 0x120);
    if (uVar40 <= uVar37) goto LAB_14099d5be;
LAB_14099d59b:
    uVar40 = *(ulonglong *)(*(longlong *)(lVar28 + 0x38) + uVar37 * 8);
    if (uVar40 == 0) goto LAB_14099d5be;
    lVar28 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar40 * 0x30);
  }
  lVar34 = *(longlong *)(lVar34 + 0x50);
  uVar40 = 0;
LAB_14099d5ca:
  uVar37 = (ulonglong)*(uint *)(lVar34 + 8 + uVar40 * 0x10);
  do {
    puVar19 = (uint *)(uVar37 * 0x60 + lVar36);
    puVar26 = (undefined8 *)(uVar37 * 0x30 + lVar28);
    uVar40 = uVar40 + 1;
    if (*puVar19 < 3) {
      local_c0 = (longlong *)CONCAT44(local_c0._4_4_,puVar19[4]);
      local_298 = lVar29;
      local_290 = lVar34;
      local_288 = lVar36;
      local_280 = lVar28;
      local_d0 = plVar21;
      local_91 = cVar44;
      resources::clutter_meshes::ClutterMeshTy::from();
      resources::clutter_meshes::clutter_mesh_ty_data();
      local_248 = (undefined8 *)*local_198;
      uStack_240 = (ulonglong)local_198[1];
      local_238 = local_198[2];
      uStack_230 = local_198[3];
      local_228 = *puVar26;
      uStack_220 = puVar26[1];
      local_218 = puVar26[2];
      uStack_210 = puVar26[3];
      local_208 = puVar26[4];
      uStack_200 = puVar26[5];
      uVar3 = *(undefined4 *)(lVar29 + uVar37 * 0x18 + 0xc);
      local_198 = (undefined **)CONCAT44(local_198._4_4_,(uint)local_c0);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar29 = (longlong)*local_b8;
      bVar43 = (byte)(extraout_RAX_02 >> 0x39);
      auVar45 = ZEXT216(CONCAT11(bVar43,bVar43));
      auVar45 = pshuflw(auVar45,auVar45,0);
      lVar36 = 0;
      uVar37 = extraout_RAX_02;
      while( true ) {
        uVar37 = (ulonglong)local_b8[1] & uVar37;
        pcVar23 = (char *)(lVar29 + uVar37);
        cVar44 = auVar45[0];
        auVar50[0] = -(*pcVar23 == cVar44);
        cVar46 = auVar45[1];
        auVar50[1] = -(pcVar23[1] == cVar46);
        cVar47 = auVar45[2];
        auVar50[2] = -(pcVar23[2] == cVar47);
        cVar48 = auVar45[3];
        auVar50[3] = -(pcVar23[3] == cVar48);
        auVar50[4] = -(pcVar23[4] == cVar44);
        auVar50[5] = -(pcVar23[5] == cVar46);
        auVar50[6] = -(pcVar23[6] == cVar47);
        auVar50[7] = -(pcVar23[7] == cVar48);
        auVar50[8] = -(pcVar23[8] == cVar44);
        auVar50[9] = -(pcVar23[9] == cVar46);
        auVar50[10] = -(pcVar23[10] == cVar47);
        auVar50[0xb] = -(pcVar23[0xb] == cVar48);
        auVar50[0xc] = -(pcVar23[0xc] == cVar44);
        auVar50[0xd] = -(pcVar23[0xd] == cVar46);
        auVar50[0xe] = -(pcVar23[0xe] == cVar47);
        auVar50[0xf] = -(pcVar23[0xf] == cVar48);
        uVar35 = (ushort)(SUB161(auVar50 >> 7,0) & 1) | (ushort)(SUB161(auVar50 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar50 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar50 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar50 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar50 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar50 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar50 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar50 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar50 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar50 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar50 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar50 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar50 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar50 >> 0x77,0) & 1) << 0xe | (ushort)(auVar50[0xf] >> 7) << 0xf
        ;
        uVar18 = (uint)uVar35;
        while (uVar35 != 0) {
          uVar32 = 0;
          for (uVar10 = uVar18; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
            uVar32 = uVar32 + 1;
          }
          uVar27 = uVar32 + uVar37 & (ulonglong)local_b8[1];
          local_250 = lVar29;
          if (*(uint *)(lVar29 + -0x20 + uVar27 * -0x20) == (uint)local_c0) {
            lVar28 = uVar27 * -0x20;
            local_158 = local_208;
            uStack_150 = uStack_200;
            local_168 = local_218;
            uStack_160 = uStack_210;
            local_178 = local_228;
            uStack_170 = uStack_220;
            local_188 = (ulonglong **)local_238;
            uStack_180 = uStack_230;
            local_198 = (undefined **)local_248;
            uStack_190 = uStack_240;
            lVar36 = *(longlong *)(lVar29 + -8 + lVar28);
            if (lVar36 == *(longlong *)(lVar29 + -0x18 + lVar28)) {
              alloc::raw_vec::RawVec::
              grow_one<country_core::resources::clutter_meshes::DisplayChildAutoClutterInfo,alloc::alloc::Global>
                        ();
            }
            local_2a0 = local_250 + lVar28;
            lVar29 = *(longlong *)(local_2a0 - 0x10);
            lVar28 = lVar36 * 0x60;
            puVar26 = (undefined8 *)(lVar29 + 0x40 + lVar28);
            *puVar26 = local_158;
            puVar26[1] = uStack_150;
            puVar2 = (ulonglong *)(lVar29 + 0x30 + lVar28);
            *puVar2 = local_168;
            puVar2[1] = uStack_160;
            puVar26 = (undefined8 *)(lVar29 + 0x20 + lVar28);
            *puVar26 = local_178;
            puVar26[1] = uStack_170;
            puVar26 = (undefined8 *)(lVar29 + 0x10 + lVar28);
            *puVar26 = local_188;
            puVar26[1] = uStack_180;
            *(undefined8 *)(lVar29 + lVar28) = local_198;
            ((undefined8 *)(lVar29 + lVar28))[1] = uStack_190;
            *(undefined4 *)(lVar29 + 0x50 + lVar28) = uVar3;
            *(longlong *)(local_2a0 - 8) = lVar36 + 1;
            bVar14 = true;
            local_c0 = (longlong *)0x0;
            ppuVar20 = local_b8;
            goto LAB_14099d329;
          }
          uVar35 = (ushort)(uVar18 - 1) & (ushort)uVar18;
          uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar35);
        }
        auVar49[0] = -(*pcVar23 == (char)iVar9);
        auVar49[1] = -(pcVar23[1] == (char)((uint)iVar9 >> 8));
        auVar49[2] = -(pcVar23[2] == (char)((uint)iVar9 >> 0x10));
        auVar49[3] = -(pcVar23[3] == (char)((uint)iVar9 >> 0x18));
        auVar49[4] = -(pcVar23[4] == (char)iVar11);
        auVar49[5] = -(pcVar23[5] == (char)((uint)iVar11 >> 8));
        auVar49[6] = -(pcVar23[6] == (char)((uint)iVar11 >> 0x10));
        auVar49[7] = -(pcVar23[7] == (char)((uint)iVar11 >> 0x18));
        auVar49[8] = -(pcVar23[8] == (char)iVar12);
        auVar49[9] = -(pcVar23[9] == (char)((uint)iVar12 >> 8));
        auVar49[10] = -(pcVar23[10] == (char)((uint)iVar12 >> 0x10));
        auVar49[0xb] = -(pcVar23[0xb] == (char)((uint)iVar12 >> 0x18));
        auVar49[0xc] = -(pcVar23[0xc] == (char)iVar13);
        auVar49[0xd] = -(pcVar23[0xd] == (char)((uint)iVar13 >> 8));
        auVar49[0xe] = -(pcVar23[0xe] == (char)((uint)iVar13 >> 0x10));
        auVar49[0xf] = -(pcVar23[0xf] == (char)((uint)iVar13 >> 0x18));
        if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x1f,0) & 1) != 0) || (SUB161(auVar49 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x37,0) & 1) != 0) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
        uVar37 = uVar37 + lVar36 + 0x10;
        lVar36 = lVar36 + 0x10;
      }
      local_2a0 = extraout_RAX_02;
      if (local_b8[2] == (undefined8 *)0x0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<country_core::resources::clutter_meshes::DisplayChildAutoClutterInfo,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<country_core::resources::clutter_meshes::DisplayChildAutoClutterInfo,alloc::alloc::Global>,std::hash::random::RandomState>_>
                  ();
        local_c0._0_4_ = (uint)local_198;
      }
      ppuVar20 = (undefined8 **)(ulonglong)(uint)local_c0;
      bVar14 = false;
      local_c0 = (longlong *)local_b8;
LAB_14099d329:
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_03[8] = local_208;
      extraout_RAX_03[9] = uStack_200;
      extraout_RAX_03[6] = local_218;
      extraout_RAX_03[7] = uStack_210;
      extraout_RAX_03[4] = local_228;
      extraout_RAX_03[5] = uStack_220;
      extraout_RAX_03[2] = local_238;
      extraout_RAX_03[3] = uStack_230;
      *extraout_RAX_03 = local_248;
      extraout_RAX_03[1] = uStack_240;
      *(undefined4 *)(extraout_RAX_03 + 10) = uVar3;
      if (bVar14) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        lVar29 = local_298;
        lVar34 = local_290;
        lVar36 = local_288;
        lVar28 = local_280;
        plVar21 = local_d0;
        cVar44 = local_91;
      }
      else {
        pauVar39 = (undefined1 (*) [16])*local_c0;
        uVar37 = local_c0[1];
        uVar27 = uVar37 & local_2a0;
        auVar45 = *(undefined1 (*) [16])(*pauVar39 + uVar27);
        uVar35 = (ushort)(SUB161(auVar45 >> 7,0) & 1) | (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar45[0xf] >> 7) << 0xf;
        uVar18 = (uint)uVar35;
        if (uVar35 == 0) {
          lVar29 = 0x10;
          do {
            uVar27 = uVar27 + lVar29 & uVar37;
            auVar45 = *(undefined1 (*) [16])(*pauVar39 + uVar27);
            lVar29 = lVar29 + 0x10;
            uVar18 = (uint)(ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                                    (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                                    (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                                    (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                                    (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                                    (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                                    (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                                    (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                                    (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                                    (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                                    (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                                    (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                                    (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                                    (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar45[0xf] >> 7) << 0xf);
          } while (uVar18 == 0);
        }
        uVar32 = 0;
        for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
          uVar32 = uVar32 + 1;
        }
        uVar27 = uVar32 + uVar27 & uVar37;
        bVar43 = (*pauVar39)[uVar27];
        if (-1 < (char)bVar43) {
          auVar45 = *pauVar39;
          uVar18 = 0;
          for (uVar32 = (uint)(ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                                       (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                                       (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                                       (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar45[0xf] >> 7) << 0xf); (uVar32 & 1) == 0;
              uVar32 = uVar32 >> 1 | 0x80000000) {
            uVar18 = uVar18 + 1;
          }
          uVar27 = (ulonglong)uVar18;
          bVar43 = (*pauVar39)[uVar27];
        }
        bVar25 = (byte)(local_2a0 >> 0x39);
        (*pauVar39)[uVar27] = bVar25;
        pauVar39[1][uVar27 - 0x10 & uVar37] = bVar25;
        local_c0[2] = local_c0[2] - (ulonglong)(bVar43 & 1);
        *(int *)pauVar39[uVar27 * -2 + -2] = (int)ppuVar20;
        *(undefined8 *)(pauVar39[uVar27 * -2 + -2] + 8) = 1;
        *(undefined8 **)pauVar39[uVar27 * -2 + -1] = extraout_RAX_03;
        *(undefined8 *)(pauVar39[uVar27 * -2 + -1] + 8) = 1;
        local_c0[3] = local_c0[3] + 1;
        lVar29 = local_298;
        lVar34 = local_290;
        lVar36 = local_288;
        lVar28 = local_280;
        plVar21 = local_d0;
        cVar44 = local_91;
      }
    }
    if (cVar44 == '\0') goto LAB_14099d490;
LAB_14099cf4e:
    if (uVar40 == uVar22) {
      do {
        if (puVar38 == puVar42) goto LAB_14099d646;
        uVar18 = *puVar38;
        puVar38 = puVar38 + 1;
        uVar22 = *(ulonglong *)(*(longlong *)(lVar31 + 0x1a8) + 0x10 + (ulonglong)uVar18 * 0x48);
      } while (uVar22 == 0);
      lVar28 = *(longlong *)(lVar31 + 0x1a8) + (ulonglong)uVar18 * 0x48;
      uVar40 = *(ulonglong *)(lVar28 + 0x40);
      if (*(ulonglong *)(local_90 + 0x110) < uVar40) {
        uVar37 = *(ulonglong *)(*(longlong *)(lVar28 + 0x38) + *(ulonglong *)(local_90 + 0x110) * 8)
        ;
        if (uVar37 == 0) {
          lVar29 = 0;
        }
        else {
          lVar29 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar37 * 0x30);
        }
        uVar37 = *(ulonglong *)(local_90 + 0x118);
        if (uVar37 < uVar40) goto LAB_14099d19d;
LAB_14099cfe8:
        lVar36 = 0;
        uVar37 = *(ulonglong *)(local_90 + 0x120);
        if (uVar37 < uVar40) goto LAB_14099d1e0;
LAB_14099cf15:
        lVar28 = 0;
      }
      else {
        lVar29 = 0;
        uVar37 = *(ulonglong *)(local_90 + 0x118);
        if (uVar40 <= uVar37) goto LAB_14099cfe8;
LAB_14099d19d:
        uVar37 = *(ulonglong *)(*(longlong *)(lVar28 + 0x38) + uVar37 * 8);
        if (uVar37 == 0) {
          lVar36 = 0;
        }
        else {
          lVar36 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar37 * 0x30);
        }
        uVar37 = *(ulonglong *)(local_90 + 0x120);
        if (uVar40 <= uVar37) goto LAB_14099cf15;
LAB_14099d1e0:
        uVar40 = *(ulonglong *)(*(longlong *)(lVar28 + 0x38) + uVar37 * 8);
        if (uVar40 == 0) goto LAB_14099cf15;
        lVar28 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar40 * 0x30);
      }
      uVar40 = 0;
    }
    uVar37 = uVar40 & 0xffffffff;
  } while( true );
LAB_14099d646:
  puVar2 = local_a8;
  if ((char)local_a0 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar21 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_14099da81:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
LAB_14099dd7e:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_a0 = plVar21;
    puffin::ThreadProfiler::end_scope();
    *local_a0 = *local_a0 + 1;
  }
  uVar22 = *puVar2;
  uVar37 = 0;
  uVar40 = uVar22 - *(ulonglong *)(local_1b0 + 0x18);
  if (uVar22 < *(ulonglong *)(local_1b0 + 0x18)) {
    uVar40 = uVar37;
  }
  uVar27 = uVar22 - *(ulonglong *)(local_1b0 + 0x38);
  if (uVar22 < *(ulonglong *)(local_1b0 + 0x38)) {
    uVar27 = uVar37;
  }
  lVar29 = *(longlong *)(local_1b0 + 8);
  uVar22 = *(ulonglong *)(local_1b0 + 0x30);
  uVar24 = uVar22 - uVar27;
  if (uVar22 < uVar27) {
    uVar24 = uVar37;
  }
  uVar7 = *(ulonglong *)(local_1b0 + 0x10);
  uVar30 = uVar7 - uVar40;
  if (uVar7 < uVar40) {
    uVar30 = uVar37;
  }
  lVar36 = *(longlong *)(local_1b0 + 0x28);
  uVar37 = *(ulonglong *)(local_1b0 + 0x40);
  *puVar2 = (uVar37 - uVar24) - uVar30;
  local_a0 = (longlong *)lVar31;
  if (uVar7 >= uVar40 && uVar7 - uVar40 != 0 || uVar22 >= uVar27 && uVar22 - uVar27 != 0) {
    lVar36 = lVar36 + uVar27 * 0x10;
    lVar29 = lVar29 + uVar40 * 0x10;
    if (uVar7 < uVar40) {
      lVar29 = 8;
    }
    lVar28 = 0;
    if (uVar40 < uVar7) {
      lVar28 = lVar29 + -0x10 + uVar30 * 0x10;
    }
    if (uVar22 < uVar27) {
      lVar36 = 8;
    }
    lVar29 = lVar36 + -0x10 + uVar24 * 0x10;
    if (uVar22 <= uVar27) {
      lVar29 = lVar28;
    }
    *puVar2 = uVar37;
    uVar22 = *(ulonglong *)(lVar29 + 8) & 0xffffffff;
    if ((uVar22 < *(ulonglong *)(lVar31 + 0x10)) &&
       (*(int *)(*(longlong *)(lVar31 + 8) + uVar22 * 0x14) ==
        (int)(*(ulonglong *)(lVar29 + 8) >> 0x20))) {
      uVar18 = *(uint *)(*(longlong *)(lVar31 + 8) + uVar22 * 0x14 + 4);
      uVar22 = (ulonglong)uVar18;
      if ((uVar22 != 0xffffffff) &&
         ((uVar22 < *(ulonglong *)(local_90 + 0x38) &&
          ((*(ulonglong *)(*(longlong *)(local_90 + 0x28) + (ulonglong)(uVar18 >> 6) * 8) >>
            (uVar22 & 0x3f) & 1) != 0)))) {
        *local_2a8 = local_d4;
        *local_268 = local_260;
        rmb_autoclutter::handle_rmb_autoclutter();
        return;
      }
    }
  }
  uVar18 = *(uint *)*local_b0;
  lVar29 = 0;
  if (uVar18 - 6 < 0xf) {
    lVar29 = (ulonglong)uVar18 - 5;
  }
  if (lVar29 == 0) {
    if (uVar18 != 0) {
      return;
    }
  }
  else {
    if (lVar29 != 4) {
      return;
    }
    if ((char)((uint *)*local_b0)[1] != '\x03') {
      return;
    }
  }
  pfVar8 = (float *)*in_stack_00000028;
  collision::world::Rapier::raycast();
  if ((local_168._4_4_ != 2) && ((char)local_198 == '\f')) {
    fVar15 = (float)uStack_180;
    fVar16 = uStack_180._4_4_;
    fVar17 = (float)local_178;
    local_2c0 = uStack_190;
    uVar22 = uStack_190 & 0xffffffff;
    if ((uVar22 < *(ulonglong *)(lVar31 + 0x10)) &&
       (*(int *)(*(longlong *)(lVar31 + 8) + uVar22 * 0x14) == (int)(uStack_190 >> 0x20))) {
      lVar29 = *(longlong *)(lVar31 + 8) + uVar22 * 0x14;
      uVar22 = (ulonglong)*(uint *)(lVar29 + 4);
      if (((uVar22 != 0xffffffff) && (uVar22 < *(ulonglong *)(local_90 + 0x38))) &&
         ((*(ulonglong *)
            (*(longlong *)(local_90 + 0x28) + (ulonglong)(*(uint *)(lVar29 + 4) >> 6) * 8) >>
           (uVar22 & 0x3f) & 1) != 0)) {
        lVar36 = *(longlong *)(lVar31 + 0x1a8) + (ulonglong)*(uint *)(lVar29 + 0xc) * 0x48;
        uVar22 = *(ulonglong *)
                  (*(longlong *)(lVar31 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar29 + 0xc) * 0x48)
        ;
        if ((*(ulonglong *)(local_90 + 0x110) < uVar22) &&
           (uVar40 = *(ulonglong *)
                      (*(longlong *)(lVar36 + 0x38) + *(ulonglong *)(local_90 + 0x110) * 8),
           uVar40 != 0)) {
          local_a0 = *(longlong **)(*(longlong *)(lVar36 + 0x18) + 0x10 + ~uVar40 * 0x30);
        }
        else {
          local_a0 = (longlong *)0x0;
        }
        if ((*(ulonglong *)(local_90 + 0x118) < uVar22) &&
           (uVar40 = *(ulonglong *)
                      (*(longlong *)(lVar36 + 0x38) + *(ulonglong *)(local_90 + 0x118) * 8),
           uVar40 != 0)) {
          lVar31 = *(longlong *)(*(longlong *)(lVar36 + 0x18) + 0x10 + ~uVar40 * 0x30);
        }
        else {
          lVar31 = 0;
        }
        if ((*(ulonglong *)(local_90 + 0x120) < uVar22) &&
           (uVar22 = *(ulonglong *)
                      (*(longlong *)(lVar36 + 0x38) + *(ulonglong *)(local_90 + 0x120) * 8),
           uVar22 != 0)) {
          local_90 = *(undefined8 *)(*(longlong *)(lVar36 + 0x18) + 0x10 + ~uVar22 * 0x30);
        }
        else {
          local_90 = 0;
        }
        uVar18 = *(uint *)(lVar29 + 0x10);
        local_1f0 = &local_2c0;
        local_1e8 = bevy_ecs::entity::impl$11::fmt;
        local_198 = &PTR_s_auto_clutter__142abbb38;
        uStack_190 = 1;
        local_188 = &local_1f0;
        uStack_180 = 1;
        local_178 = 0;
        ppuVar20 = &local_248;
        alloc::fmt::format::format_inner();
        local_10c = SQRT((pfVar8[2] - fVar17) * (pfVar8[2] - fVar17) +
                         (pfVar8[1] - fVar16) * (pfVar8[1] - fVar16) +
                         (*pfVar8 - fVar15) * (*pfVar8 - fVar15));
        local_198 = (undefined **)0x0;
        uStack_190 = 8;
        local_188 = (ulonglong **)0x0;
        uStack_180 = CONCAT44(uStack_180._4_4_,1);
        local_178 = 0;
        local_168 = local_168 & 0xffffffff00000000;
        uStack_150 = uStack_150 & 0xffffffff00000000;
        local_138 = 0;
        local_120 = 0;
        local_110 = 5;
        local_f0 = 0x301;
        systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        if (((*(char *)(extraout_RAX_07 + 0x18) != '\0') &&
            (*(undefined8 ***)(extraout_RAX_07 + 0x38) == ppuVar20)) &&
           (*(int *)(extraout_RAX_07 + 0x28) != 2)) {
          local_1b0 = (ulonglong)uVar18 * 3;
          local_a8 = (ulonglong *)(lVar31 + (ulonglong)uVar18 * 0x60);
          lVar31 = 0;
          if (*local_a8 - 3 < 3) {
            lVar31 = *local_a8 + -2;
          }
          local_b8 = ppuVar20;
                    /* WARNING: Could not emulate address calculation at 0x00014099dc9c */
                    /* WARNING: Treating indirect jump as call */
          (*(code *)((longlong)&DAT_142abbcdc + (longlong)(int)(&DAT_142abbcdc)[lVar31]))();
          return;
        }
      }
    }
  }
  return;
}

