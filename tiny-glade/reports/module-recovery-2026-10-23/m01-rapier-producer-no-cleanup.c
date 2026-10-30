
/* WARNING: Removing unreachable block (ram,0x0001408d2538) */
/* WARNING: Removing unreachable block (ram,0x0001408d5d8d) */
/* WARNING: Removing unreachable block (ram,0x0001408d263d) */
/* WARNING: Removing unreachable block (ram,0x0001408d26e3) */
/* WARNING: Removing unreachable block (ram,0x0001408d26fb) */
/* WARNING: Removing unreachable block (ram,0x0001408d38e5) */
/* WARNING: Removing unreachable block (ram,0x0001408d2723) */
/* WARNING: Removing unreachable block (ram,0x0001408d273e) */
/* WARNING: Removing unreachable block (ram,0x0001408d3d1f) */
/* WARNING: Removing unreachable block (ram,0x0001408d3e70) */
/* WARNING: Removing unreachable block (ram,0x0001408d3d3b) */
/* WARNING: Removing unreachable block (ram,0x0001408d3d57) */
/* WARNING: Removing unreachable block (ram,0x0001408d2751) */
/* WARNING: Removing unreachable block (ram,0x0001408d22ee) */
/* WARNING: Removing unreachable block (ram,0x0001408d5da1) */
/* WARNING: Removing unreachable block (ram,0x0001408d23f3) */
/* WARNING: Removing unreachable block (ram,0x0001408d2493) */
/* WARNING: Removing unreachable block (ram,0x0001408d24ab) */
/* WARNING: Removing unreachable block (ram,0x0001408d24b2) */
/* WARNING: Removing unreachable block (ram,0x0001408d38af) */
/* WARNING: Removing unreachable block (ram,0x0001408d24dd) */
/* WARNING: Removing unreachable block (ram,0x0001408d24fe) */
/* WARNING: Removing unreachable block (ram,0x0001408d3c78) */
/* WARNING: Removing unreachable block (ram,0x0001408d3e3c) */
/* WARNING: Removing unreachable block (ram,0x0001408d3c94) */
/* WARNING: Removing unreachable block (ram,0x0001408d3cb0) */
/* WARNING: Removing unreachable block (ram,0x0001408d2514) */
/* WARNING: Removing unreachable block (ram,0x0001408d27ab) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void country_core::systems::collision::world::update_rapier_colliders
               (void *world,void *wall_changed,void *wall_removed,void *all_roofs,void *roof_changed
               ,void *roof_removed,void *roof_deleted,void *arches,void *lintels,void *walls,
               void *auto_changed,void *auto_removed)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  ulonglong uVar5;
  uint *puVar6;
  void *_Src;
  bool bVar7;
  short sVar8;
  uint uVar9;
  uint uVar10;
  ulonglong uVar11;
  bool bVar12;
  float fVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined4 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  float fVar20;
  float fVar21;
  uint *puVar22;
  char extraout_AL;
  uint uVar23;
  uint uVar24;
  uint uVar25;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *puVar26;
  longlong extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong extraout_RAX_05;
  uint *_Dst;
  uint *extraout_RAX_06;
  ulonglong uVar27;
  longlong lVar28;
  ulonglong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong lVar29;
  ulonglong extraout_RAX_11;
  ulonglong extraout_RAX_12;
  undefined1 *extraout_RAX_13;
  undefined1 *extraout_RAX_14;
  ulonglong extraout_RAX_15;
  ulonglong extraout_RAX_16;
  undefined1 *extraout_RAX_17;
  longlong lVar30;
  undefined1 *extraout_RAX_18;
  uint *_Dst_00;
  uint *extraout_RAX_19;
  undefined1 *extraout_RAX_20;
  undefined8 *extraout_RAX_21;
  undefined8 *extraout_RAX_22;
  undefined8 *extraout_RAX_23;
  uint *_Dst_01;
  ulonglong extraout_RAX_24;
  undefined8 *extraout_RAX_25;
  uint *_Dst_02;
  uint *extraout_RAX_26;
  longlong lVar31;
  ulonglong extraout_RAX_27;
  ulonglong extraout_RAX_28;
  ulonglong extraout_RAX_29;
  longlong extraout_RAX_30;
  longlong extraout_RAX_31;
  ulonglong extraout_RAX_32;
  longlong extraout_RAX_33;
  longlong extraout_RAX_34;
  longlong extraout_RAX_35;
  longlong extraout_RAX_36;
  ulonglong extraout_RAX_37;
  undefined1 *extraout_RAX_38;
  uint *puVar32;
  undefined1 *extraout_RAX_39;
  void *pvVar33;
  undefined1 *extraout_RAX_40;
  ulonglong *_Dst_03;
  uint *extraout_RAX_41;
  uint *puVar34;
  ulonglong extraout_RAX_42;
  longlong *extraout_RAX_43;
  longlong *extraout_RAX_44;
  longlong *extraout_RAX_45;
  ulonglong uVar35;
  longlong *plVar36;
  ushort uVar37;
  ulonglong uVar38;
  longlong lVar39;
  uint *puVar40;
  uint *puVar41;
  longlong lVar42;
  ulonglong *puVar43;
  uint *unaff_RDI;
  longlong lVar44;
  int iVar45;
  undefined *puVar46;
  uint *puVar47;
  ulonglong uVar48;
  ulonglong uVar49;
  longlong lVar50;
  ulonglong uVar51;
  undefined1 *puVar52;
  uint *puVar53;
  ushort uVar54;
  longlong lVar55;
  undefined8 *puVar56;
  undefined1 uVar57;
  ulonglong *puVar58;
  ulonglong *puVar59;
  uint *puVar60;
  char cVar64;
  char cVar65;
  float fVar61;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined1 auVar62 [16];
  char cVar66;
  undefined1 auVar63 [16];
  undefined4 uVar67;
  char cVar69;
  char cVar70;
  undefined1 auVar68 [16];
  byte bVar71;
  float fVar72;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  longlong lStack_418;
  undefined4 *puStack_410;
  longlong lStack_408;
  uint *puStack_400;
  undefined8 *puStack_3f8;
  uint *puStack_3f0;
  longlong lStack_3b8;
  undefined8 uStack_3b0;
  uint *puStack_3a8;
  uint *puStack_3a0;
  uint *puStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  longlong lStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined1 auStack_368 [8];
  undefined8 uStack_360;
  longlong lStack_358;
  uint *puStack_350;
  undefined8 uStack_348;
  undefined8 uStack_338;
  uint *puStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  longlong lStack_310;
  undefined8 uStack_308;
  longlong lStack_2f0;
  undefined8 *puStack_2e8;
  undefined8 uStack_2e0;
  longlong lStack_2d8;
  undefined8 uStack_2b8;
  undefined8 uStack_2b0;
  float fStack_2a8;
  float fStack_2a4;
  uint *puStack_2a0;
  uint *puStack_298;
  undefined8 uStack_290;
  longlong lStack_288;
  uint *puStack_280;
  undefined8 uStack_278;
  undefined8 uStack_268;
  uint *puStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  longlong lStack_240;
  undefined8 uStack_238;
  undefined8 uStack_228;
  undefined8 *puStack_220;
  undefined8 uStack_218;
  longlong lStack_210;
  uint *puStack_208;
  ulonglong *puStack_200;
  undefined8 uStack_1f8;
  undefined8 *puStack_1f0;
  uint *puStack_1e8;
  uint *puStack_1d8;
  uint *puStack_1d0;
  void *pvStack_1c8;
  uint *puStack_1c0;
  uint *puStack_1b8;
  uint *puStack_1b0;
  uint *puStack_1a8;
  uint *puStack_1a0;
  char cStack_189;
  uint *puStack_188;
  ulonglong *puStack_180;
  uint *puStack_178;
  uint *puStack_170;
  ulonglong uStack_168;
  longlong lStack_160;
  uint *puStack_158;
  undefined1 uStack_14b;
  undefined1 uStack_14a;
  undefined1 uStack_149;
  uint *puStack_148;
  uint *puStack_140;
  uint *puStack_138;
  ulonglong *puStack_130;
  uint *puStack_128;
  undefined1 uStack_119;
  undefined1 uStack_118;
  undefined1 uStack_117;
  undefined1 uStack_116;
  undefined1 uStack_115;
  undefined1 uStack_112;
  undefined1 uStack_111;
  uint *puStack_110;
  uint *puStack_108;
  undefined1 uStack_f9;
  longlong *plStack_f8;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  puStack_180 = wall_changed;
  puffin::are_scopes_on();
  puStack_188 = all_roofs;
  if (extraout_AL == '\0') {
    bVar12 = false;
  }
  else {
    unaff_RDI = (uint *)&DAT_00000040;
    uVar38 = 0x40;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar38 != 0) {
        if (uVar38 < 0x40) {
          if ((char)(&DAT_142aa3652)[uVar38] < -0x40) {
LAB_1408d056c:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar38 != 0x40) goto LAB_1408d056c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar38 != 0xfffffffffffffffe) {
          if (uVar38 + 2 < 0x40) {
            if ((char)(&UNK_142aa3654)[uVar38] < -0x40) {
LAB_1408d05ca:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (uVar38 + 2 != 0x40) goto LAB_1408d05ca;
        }
        unaff_RDI = (uint *)(uVar38 * -1 + 0x3e);
      }
    }
    puVar46 = &DAT_142aa38ea;
    do {
      bVar71 = puVar46[-1];
      uVar23 = (uint)(char)bVar71;
      if ((char)bVar71 < '\0') {
        bVar3 = puVar46[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar46[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar46 + -4;
            puVar46 = puVar46 + -4;
            uVar23 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar46 = puVar46 + -3;
            uVar23 = bVar4 & 0xf;
          }
          uVar23 = bVar3 & 0x3f | uVar23 << 6;
        }
        else {
          puVar46 = puVar46 + -2;
          uVar23 = bVar3 & 0x1f;
        }
        uVar23 = (uint)(bVar71 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_1408d0684;
LAB_1408d0695:
        if (puVar46 == (undefined *)0x142aa38b7) break;
        if (puVar46 + -0x142aa38b7 < (undefined *)0x32) {
          if (-0x41 < (char)puVar46[1]) break;
        }
        else if (puVar46 + -0x142aa38b7 == (undefined *)0x32) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar46 = puVar46 + -1;
      if (uVar23 == 0x5c) goto LAB_1408d0695;
LAB_1408d0684:
      if (uVar23 == 0x2f) goto LAB_1408d0695;
    } while (puVar46 != &anon_b111d695fd3e0f72064e00b504fdf12a_34_llvm_7010674786476989468);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar36 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar36 = extraout_RAX_45, extraout_RAX_45 == (longlong *)0x0)) {
LAB_1408d5c7a:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar36 != 0) {
LAB_1408d5c97:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar36 = -1;
    plStack_f8 = plVar36;
    puffin::ThreadProfiler::begin_scope();
    *plStack_f8 = *plStack_f8 + 1;
    bVar12 = true;
  }
  **(undefined4 **)((longlong)world + 0x10) = *(undefined4 *)((longlong)world + 0x1c);
  fVar13 = __xmm_00000000000000003f0000003f000000._4_4_;
  fVar72 = (float)__xmm_00000000000000003f0000003f000000;
  plStack_f8 = *(longlong **)world;
  puVar47 = *(uint **)auto_changed;
  puVar34 = *(uint **)((longlong)auto_changed + 8);
  iVar45 = *(int *)((longlong)auto_changed + 0x14);
  puVar53 = *(uint **)(puVar47 + 0x3e);
  uVar24 = iVar45 + *(int *)((longlong)auto_changed + 0x10) * -1;
  uVar23 = 0xc233b000;
  if (uVar24 < 0xc233b000) {
    uVar23 = uVar24;
  }
  puVar40 = puVar53 + *(longlong *)(puVar47 + 0x40);
  cVar70 = *(char *)(puVar47 + 0x42);
  puStack_208 = (uint *)((longlong)plStack_f8 + 0x3e88);
  puStack_140 = (uint *)((longlong)plStack_f8 + 0x3ea8);
  lStack_2d8 = 8;
  puVar60 = (uint *)0x0;
  puVar41 = (uint *)0x0;
  lVar44 = 0;
  puStack_108 = (uint *)0x0;
  puStack_110 = (uint *)0x0;
  lStack_210 = 8;
  puStack_158 = (uint *)CONCAT44(puStack_158._4_4_,iVar45);
  lStack_160 = CONCAT44(lStack_160._4_4_,uVar23);
  lVar42 = lStack_210;
  while (lStack_210 = lVar42, puStack_148 = puVar47, cVar70 == '\0') {
    if (puVar60 != puVar41) goto LAB_1408d0989;
LAB_1408d0839:
    if (puVar53 == puVar40) goto LAB_1408d0fa2;
    uVar23 = *puVar53;
    puVar53 = puVar53 + 1;
    if (*(longlong *)(*(longlong *)(puVar34 + 0x40) + 0x58 + (ulonglong)uVar23 * 0xa0) == 0) {
      do {
        if (puVar53 == puVar40) goto LAB_1408d0fa2;
        uVar23 = *puVar53;
        puVar53 = puVar53 + 1;
      } while (*(longlong *)(*(longlong *)(puVar34 + 0x40) + 0x58 + (ulonglong)uVar23 * 0xa0) == 0);
      lVar42 = *(longlong *)(puVar34 + 0x40) + (ulonglong)uVar23 * 0xa0;
    }
    else {
      lVar42 = *(longlong *)(puVar34 + 0x40) + (ulonglong)uVar23 * 0xa0;
    }
    unaff_RDI = (uint *)(lVar42 + 0x58);
    lVar44 = *(longlong *)(puVar34 + 0x6a) + (ulonglong)*(uint *)(lVar42 + 0x94) * 0x48;
    uVar38 = *(ulonglong *)
              (*(longlong *)(puVar34 + 0x6a) + 0x40 + (ulonglong)*(uint *)(lVar42 + 0x94) * 0x48);
    if ((*(ulonglong *)(puVar47 + 0x44) < uVar38) &&
       (uVar48 = *(ulonglong *)(*(longlong *)(lVar44 + 0x38) + *(ulonglong *)(puVar47 + 0x44) * 8),
       uVar48 != 0)) {
      puStack_110 = *(uint **)(*(longlong *)(lVar44 + 0x18) + 0x10 + ~uVar48 * 0x30);
      uVar48 = *(ulonglong *)(puVar47 + 0x46);
      if (uVar38 <= uVar48) goto LAB_1408d0950;
LAB_1408d0917:
      uVar38 = *(ulonglong *)(*(longlong *)(lVar44 + 0x38) + uVar48 * 8);
      if (uVar38 == 0) goto LAB_1408d0950;
      puStack_108 = *(uint **)(*(longlong *)(lVar44 + 0x18) + 0x10 + ~uVar38 * 0x30);
    }
    else {
      puStack_110 = (uint *)0x0;
      uVar48 = *(ulonglong *)(puVar47 + 0x46);
      if (uVar48 < uVar38) goto LAB_1408d0917;
LAB_1408d0950:
      puStack_108 = (uint *)0x0;
    }
    bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
    lStack_2d8 = *(longlong *)(lVar42 + 0x50);
    puVar41 = *(uint **)unaff_RDI;
    puVar60 = (uint *)0x0;
    puVar47 = puStack_148;
    lVar44 = extraout_RAX_02;
    iVar45 = (int)puStack_158;
    uVar23 = (uint)lStack_160;
LAB_1408d0989:
    while( true ) {
      uVar38 = (ulonglong)*(uint *)(lStack_2d8 + 8 + (longlong)puVar60 * 0x10);
      if ((uint)(iVar45 + *(int *)(lVar44 + uVar38 * 4) * -1) < uVar23) break;
      puVar60 = (uint *)((longlong)puVar60 + 1);
      if (puVar60 == puVar41) goto LAB_1408d0839;
    }
    puVar26 = (undefined8 *)((longlong)puVar60 * 0x10 + lStack_2d8);
LAB_1408d0b65:
    puVar47 = puStack_110 + uVar38 * 0xc;
    puStack_1d8 = (uint *)*puVar26;
    uStack_2b0._0_4_ = puStack_108[uVar38 * 0x18 + 0x15];
    fVar61 = (float)puStack_108[uVar38 * 0x18 + 0x17] - (float)puStack_108[uVar38 * 0x18 + 0x16];
    uStack_2b0._4_4_ = fVar61 * 0.0 * fVar72;
    fStack_2a8 = fVar61 * fVar13;
    uStack_1f8 = (uint *)CONCAT44(fStack_2a8,uStack_2b0._4_4_);
    puStack_1f0 = (undefined8 *)CONCAT44(puStack_1f0._4_4_,uStack_2b0._4_4_);
    uStack_2b8 = (uint *)CONCAT44(fVar61,puStack_108[uVar38 * 0x18 + 0x14]);
    fStack_2a4 = uStack_2b0._4_4_;
    puStack_170 = puVar47;
    ::utils::calculate_hash<tuple_<array_<u32,3>,glam::u32::uvec3::UVec3>_>();
    puStack_1c0 = (uint *)extraout_RAX_04;
    parry3d::shape::shared_shape::SharedShape::cuboid();
    puStack_1b0 = puVar47;
    puStack_178 = (uint *)extraout_RAX_05;
    ::utils::IsoTransform::from_translation();
    lStack_358 = (longlong)puStack_2a0;
    uStack_258 = uStack_2b8;
    uStack_250 = CONCAT44(uStack_2b0._4_4_,(uint)uStack_2b0);
    uStack_248 = CONCAT44(fStack_2a4,fStack_2a8);
    uStack_2b8 = (uint *)0x1;
    uStack_2b0._0_4_ = 1;
    uStack_2b0._4_4_ = 0.0;
    fStack_2a8 = 5.60519e-45;
    fStack_2a4 = 0.0;
    puStack_2a0 = (uint *)0x0;
    puStack_298 = (uint *)0x4;
    uStack_290 = 0;
    lStack_288 = (longlong)puStack_178;
    uStack_278 = 0;
    uStack_268 = 0;
    lStack_240 = lStack_358;
    uStack_238 = puStack_1c0;
    puStack_280 = puVar47;
    puStack_260 = uStack_378;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (uint *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    memcpy(_Dst,&uStack_2b8,0x90);
    puStack_3a8 = puStack_1d8;
    uStack_3b0 = (uint *)CONCAT71(uStack_3b0._1_7_,0xc);
    uStack_2b8 = puStack_1d8;
    uStack_119 = 1;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    puVar60 = (uint *)((longlong)puVar60 + 1);
    lVar42 = *(longlong *)((longlong)plStack_f8 + 0x3e88);
    bVar71 = SUB81(extraout_RAX_06,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    puVar59 = (ulonglong *)0x0;
    unaff_RDI = _Dst;
    puVar47 = extraout_RAX_06;
    while( true ) {
      uVar38 = *(ulonglong *)((longlong)plStack_f8 + 0x3e90) & (ulonglong)puVar47;
      pcVar2 = (char *)(lVar42 + uVar38);
      cVar69 = SUB161(auVar62,0);
      cVar64 = SUB161(auVar62,1);
      cVar65 = SUB161(auVar62,2);
      cVar66 = SUB161(auVar62,3);
      bVar71 = (pcVar2[0xf] == cVar66) * -1;
      auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar66) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar65) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar23 = (uint)uVar54;
      while (uVar54 != 0) {
        uVar24 = 0;
        for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
          uVar24 = uVar24 + 1;
        }
        unaff_RDI = (uint *)(uVar24 + uVar38 & *(ulonglong *)((longlong)plStack_f8 + 0x3e90));
        puStack_200 = puVar59;
        if (*(uint **)(lVar42 + -0x20 + (longlong)unaff_RDI * -0x20) == puStack_1d8) {
          unaff_RDI = (uint *)((longlong)unaff_RDI * -0x20);
          puStack_1b0 = (uint *)(lVar42 + (longlong)unaff_RDI);
          puStack_1a8 = puStack_208;
          puStack_1b8 = (uint *)0x0;
          goto LAB_1408d0ecb;
        }
        uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
        uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
      }
      cVar69 = (pcVar2[0xf] == -1) * -1;
      auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar69 < '\0') break;
      puVar47 = (uint *)((longlong)puVar59 + uVar38 + 0x10);
      puVar59 = puVar59 + 2;
    }
    puVar47 = puStack_1d8;
    puStack_1c0 = extraout_RAX_06;
    if (*(longlong *)((longlong)plStack_f8 + 0x3e98) == 0) {
      uStack_119 = 1;
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
                ();
      puVar47 = uStack_2b8;
    }
    puStack_1b8 = puStack_208;
    puStack_1a8 = puStack_1c0;
    puStack_1b0 = puVar47;
LAB_1408d0ecb:
    _auStack_368 = CONCAT88(_Dst,puStack_3a0);
    uStack_2b0._0_4_ = puStack_170[6];
    uStack_2b8 = *(uint **)(puStack_170 + 4);
    puStack_2a0 = *(uint **)(puStack_170 + 2);
    fStack_2a8 = (float)*(longlong *)puStack_170;
    fStack_2a4 = SUB84(*(longlong *)puStack_170,4);
    uStack_119 = 0;
    upsert_collider_entry<bevy_ecs::entity::Entity,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
              ();
    puVar47 = puStack_148;
    iVar45 = (int)puStack_158;
    uVar23 = (uint)lStack_160;
    lVar42 = lStack_210;
    puStack_398 = _Dst;
    uStack_378 = uStack_3b0;
    uStack_370 = puStack_3a8;
  }
  if (puVar60 != puVar41) goto LAB_1408d0b3c;
  while (puVar53 != puVar40) {
    uVar38 = (ulonglong)*puVar53;
    puVar53 = puVar53 + 1;
    lVar42 = *(longlong *)(puVar34 + 0x6a);
    if (*(longlong *)(lVar42 + 0x10 + uVar38 * 0x48) == 0) {
      do {
        if (puVar53 == puVar40) goto LAB_1408d0fa2;
        uVar23 = *puVar53;
        puVar53 = puVar53 + 1;
      } while (*(longlong *)(*(longlong *)(puVar34 + 0x6a) + 0x10 + (ulonglong)uVar23 * 0x48) == 0);
      lVar44 = *(longlong *)(puVar34 + 0x6a) + (ulonglong)uVar23 * 0x48;
      puVar26 = (undefined8 *)(lVar44 + 0x10);
      uVar38 = *(ulonglong *)(puVar47 + 0x44);
      uVar48 = *(ulonglong *)(lVar44 + 0x40);
      if (uVar38 < uVar48) goto LAB_1408d0a6c;
LAB_1408d0ab0:
      puStack_110 = (uint *)0x0;
      uVar38 = *(ulonglong *)(puVar47 + 0x46);
      if (uVar48 <= uVar38) goto LAB_1408d0b00;
LAB_1408d0ac7:
      uVar38 = *(ulonglong *)(*(longlong *)(lVar44 + 0x38) + uVar38 * 8);
      if (uVar38 == 0) goto LAB_1408d0b00;
      puStack_108 = *(uint **)(*(longlong *)(lVar44 + 0x18) + 0x10 + ~uVar38 * 0x30);
    }
    else {
      lVar44 = lVar42 + uVar38 * 0x48;
      puVar26 = (undefined8 *)(lVar42 + uVar38 * 0x48 + 0x10);
      uVar38 = *(ulonglong *)(puVar47 + 0x44);
      uVar48 = *(ulonglong *)(lVar44 + 0x40);
      if (uVar48 <= uVar38) goto LAB_1408d0ab0;
LAB_1408d0a6c:
      uVar38 = *(ulonglong *)(*(longlong *)(lVar44 + 0x38) + uVar38 * 8);
      if (uVar38 == 0) goto LAB_1408d0ab0;
      puStack_110 = *(uint **)(*(longlong *)(lVar44 + 0x18) + 0x10 + ~uVar38 * 0x30);
      uVar38 = *(ulonglong *)(puVar47 + 0x46);
      if (uVar38 < uVar48) goto LAB_1408d0ac7;
LAB_1408d0b00:
      puStack_108 = (uint *)0x0;
    }
    bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
    lVar42 = *(longlong *)(lVar44 + 8);
    puVar41 = (uint *)*puVar26;
    puVar60 = (uint *)0x0;
    puVar47 = puStack_148;
    lVar44 = extraout_RAX_03;
    iVar45 = (int)puStack_158;
    uVar23 = (uint)lStack_160;
LAB_1408d0b3c:
    do {
      uVar38 = (ulonglong)puVar60 & 0xffffffff;
      if ((uint)(iVar45 + *(int *)(lVar44 + uVar38 * 4) * -1) < uVar23) {
        puVar26 = (undefined8 *)(lVar42 + (longlong)puVar60 * 8);
        lStack_210 = lVar42;
        goto LAB_1408d0b65;
      }
      puVar60 = (uint *)((longlong)puVar60 + 1);
    } while (puVar60 != puVar41);
  }
LAB_1408d0fa2:
  lVar42 = *(longlong *)((longlong)auto_removed + 0x10);
  if ((**(ulonglong **)auto_removed < *(ulonglong *)(lVar42 + 0x40)) &&
     (uVar38 = *(ulonglong *)(*(longlong *)(lVar42 + 0x38) + **(ulonglong **)auto_removed * 8),
     uVar38 != 0)) {
    lVar42 = *(longlong *)(lVar42 + 8);
    uVar38 = ~uVar38;
    puVar59 = *(ulonglong **)((longlong)auto_removed + 8);
    uVar48 = *puVar59;
    uVar51 = 0;
    puVar43 = (ulonglong *)(lVar42 + 0x18 + uVar38 * 0x48);
    uVar27 = uVar48 + *puVar43 * -1;
    if (uVar48 < *puVar43) {
      uVar27 = uVar51;
    }
    puVar43 = (ulonglong *)(lVar42 + 0x38 + uVar38 * 0x48);
    uVar35 = uVar48 + *puVar43 * -1;
    if (uVar48 < *puVar43) {
      uVar35 = uVar51;
    }
    puVar40 = (uint *)(uVar27 * 0x10 + *(longlong *)(lVar42 + 8 + uVar38 * 0x48));
    uVar48 = *(ulonglong *)(lVar42 + 0x10 + uVar38 * 0x48);
    uVar49 = *(ulonglong *)(lVar42 + 0x30 + uVar38 * 0x48);
    uVar5 = uVar48 + uVar27 * -1;
    if (uVar48 < uVar27) {
      puVar40 = (uint *)&DAT_00000008;
      uVar5 = uVar51;
    }
    puVar60 = (uint *)(uVar35 * 0x10 + *(longlong *)(lVar42 + 0x28 + uVar38 * 0x48));
    uVar48 = uVar49 + uVar35 * -1;
    if (uVar49 < uVar35) {
      puVar60 = (uint *)&DAT_00000008;
      uVar48 = uVar51;
    }
    *puVar59 = *(longlong *)(lVar42 + 0x40 + uVar38 * 0x48) + uVar48 * -1 + uVar5 * -1;
    puStack_148 = puVar40 + uVar5 * 4;
    puStack_158 = puVar60 + uVar48 * 4;
  }
  else {
    puVar59 = (ulonglong *)0x0;
  }
  puVar43 = (ulonglong *)0x0;
LAB_1408d1139:
  do {
    puVar47 = unaff_RDI;
    if (puVar43 != (ulonglong *)0x0) goto LAB_1408d1258;
LAB_1408d1240:
    do {
      puVar43 = puVar59;
      uVar18 = _UNK_142aa3298;
      uVar17 = ___xmm_8000000080000000bf80000080000000;
      uVar23 = __real_3ea147ae;
      uVar16 = __real_beeb851f;
      uVar15 = __xmm_0000000000000000bf80000080000000;
      lVar42 = DAT_14293e0c8;
      uVar14 = __xmm_3f800000000000000000000000000000;
      puVar59 = (ulonglong *)0x0;
      puVar34 = puVar60;
      puVar47 = puVar40;
      if (puVar43 == (ulonglong *)0x0) {
        puStack_110 = (uint *)*puStack_180;
        uVar38 = puStack_180[1];
        iVar45 = *(int *)((longlong)puStack_180 + 0x14);
        puVar34 = *(uint **)((longlong)puStack_110 + 0xf8);
        puVar47 = puVar34 + *(longlong *)((longlong)puStack_110 + 0x100);
        cVar70 = *(char *)((longlong)puStack_110 + 0x108);
        puStack_208 = *(uint **)walls;
        puStack_1d8 = *(uint **)lintels;
        puStack_1c0 = *(uint **)arches;
        uVar25 = iVar45 + *(int *)(puStack_180 + 2) * -1;
        uVar24 = 0xc233b000;
        if (uVar25 < 0xc233b000) {
          uVar24 = uVar25;
        }
        puStack_200 = (ulonglong *)(puStack_1c0 + 8);
        lVar44 = *(longlong *)puStack_188;
        lVar28 = *(longlong *)((longlong)puStack_188 + 8);
        pvStack_1c8 = (void *)((longlong)plStack_f8 + 0x3e58);
        puStack_1d0 = (uint *)((longlong)plStack_f8 + 0x3e28);
        puStack_178 = (uint *)((longlong)plStack_f8 + 0x3dc8);
        lStack_160 = (longlong)plStack_f8 + 0x3e78;
        puStack_170 = (uint *)((longlong)puStack_208 + 0x50);
        puStack_188 = (uint *)((longlong)plStack_f8 + 0x3de8);
        puStack_140 = (uint *)((longlong)plStack_f8 + 0x3e48);
        lStack_210 = 8;
        puStack_108 = (uint *)0x0;
        lStack_380 = 8;
        puStack_148 = (uint *)0x0;
        lVar31 = 0;
        uVar27 = 0;
        puStack_158 = (uint *)CONCAT44(puStack_158._4_4_,iVar45);
        puStack_180 = (ulonglong *)CONCAT44(puStack_180._4_4_,uVar24);
        uVar48 = 0;
        fVar72 = __real_3dcccccd;
        goto LAB_1408d14b2;
      }
LAB_1408d1258:
      if ((puVar47 == (uint *)0x0) || (unaff_RDI = puVar47 + 4, puVar47 == puStack_148)) {
        if ((puVar34 == (uint *)0x0) || (puVar34 == puStack_158)) goto LAB_1408d1240;
        unaff_RDI = (uint *)0x0;
        puVar47 = puVar34;
        puVar34 = puVar34 + 4;
      }
      *puVar43 = *puVar43 + 1;
      puVar47 = *(uint **)(puVar47 + 2);
    } while ((ulonglong)puVar47 >> 0x20 == 0);
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar42 = *(longlong *)((longlong)plStack_f8 + 0x3e88);
    uVar38 = *(ulonglong *)((longlong)plStack_f8 + 0x3e90);
    bVar71 = SUB81(extraout_RAX_07,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    lVar44 = 0;
    uVar48 = extraout_RAX_07;
    while( true ) {
      uVar48 = uVar38 & uVar48;
      pcVar2 = (char *)(lVar42 + uVar48);
      cVar70 = SUB161(auVar62,0);
      cVar69 = SUB161(auVar62,1);
      cVar64 = SUB161(auVar62,2);
      cVar65 = SUB161(auVar62,3);
      bVar71 = (pcVar2[0xf] == cVar65) * -1;
      auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar64) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar69) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar70) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar65) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar64) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar69) * -1,
                                                  CONCAT18((pcVar2[8] == cVar70) * -1,
                                                           CONCAT17((pcVar2[7] == cVar65) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar64) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar69)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar70) * -1,
                                                           CONCAT13((pcVar2[3] == cVar65) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar64) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar69)
                                                                                         * -1,(*
                                                  pcVar2 == cVar70) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar23 = (uint)uVar54;
      puStack_110 = unaff_RDI;
      puStack_108 = puVar34;
      if (uVar54 != 0) break;
LAB_1408d10c0:
      cVar70 = (pcVar2[0xf] == -1) * -1;
      auVar63 = CONCAT115(cVar70,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar70 < '\0') goto LAB_1408d1110;
      uVar48 = uVar48 + lVar44 + 0x10;
      lVar44 = lVar44 + 0x10;
    }
    while( true ) {
      uVar24 = 0;
      for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
        uVar24 = uVar24 + 1;
      }
      uVar27 = uVar24 + uVar48 & uVar38;
      if (*(uint **)(lVar42 + -0x20 + uVar27 * -0x20) == puVar47) break;
      uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
      uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
      if (uVar54 == 0) goto LAB_1408d10c0;
    }
    lVar44 = (longlong)(uVar27 * 0x20) >> 5;
    uVar38 = lVar44 + 0xfffffffffffffff0U & uVar38;
    pcVar2 = (char *)(lVar42 + uVar38);
    bVar71 = (pcVar2[0xf] == -1) * -1;
    auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    if (uVar54 == 0) {
      uVar37 = 0x10;
    }
    else {
      uVar37 = 0xf;
      if (uVar54 != 0) {
        for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
        }
      }
      uVar37 = uVar37 ^ 0xf;
    }
    pcVar2 = (char *)(lVar42 + lVar44);
    bVar71 = (pcVar2[0xf] == -1) * -1;
    auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    sVar8 = 0;
    for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                          (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
        uVar23 = uVar23 >> 1 | 0x80000000) {
      sVar8 = sVar8 + 1;
    }
    uVar57 = 0x80;
    if ((ushort)(uVar37 + sVar8) < 0x10) {
      *(longlong *)((longlong)plStack_f8 + 0x3e98) =
           *(longlong *)((longlong)plStack_f8 + 0x3e98) + 1;
      uVar57 = 0xff;
    }
    lVar28 = uVar27 * -0x20 + lVar42;
    *(undefined1 *)(lVar42 + lVar44) = uVar57;
    *(undefined1 *)(lVar42 + 0x10 + uVar38) = uVar57;
    *(longlong *)((longlong)plStack_f8 + 0x3ea0) = *(longlong *)((longlong)plStack_f8 + 0x3ea0) + -1
    ;
    _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar28 + -8));
    uStack_378 = *(uint **)(lVar28 + -0x18);
    uStack_370 = *(uint **)(lVar28 + -0x10);
    dispose_collider_mesh_instance();
    if ((char)puStack_298 != '\x11') {
      LOCK();
      *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
      UNLOCK();
      if (*(longlong *)puStack_280 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
      }
    }
  } while( true );
LAB_1408d14b2:
  fVar20 = fStack_2a4;
  fVar13 = fStack_2a8;
  fStack_2a8 = (float)uVar14;
  fVar61 = fStack_2a8;
  fStack_2a4 = SUB84(uVar14,4);
  fVar21 = fStack_2a4;
  fStack_2a8 = fVar13;
  fStack_2a4 = fVar20;
  if (cVar70 == '\0') {
    do {
      if (uVar27 == uVar48) {
        if (puVar34 == puVar47) goto LAB_1408d3eab;
        uVar24 = *puVar34;
        puVar34 = puVar34 + 1;
        if (*(longlong *)(*(longlong *)(uVar38 + 0x100) + 0x58 + (ulonglong)uVar24 * 0xa0) == 0) {
          do {
            if (puVar34 == puVar47) goto LAB_1408d3eab;
            uVar24 = *puVar34;
            puVar34 = puVar34 + 1;
          } while (*(longlong *)(*(longlong *)(uVar38 + 0x100) + 0x58 + (ulonglong)uVar24 * 0xa0) ==
                   0);
          lVar31 = *(longlong *)(uVar38 + 0x100) + (ulonglong)uVar24 * 0xa0;
        }
        else {
          lVar31 = *(longlong *)(uVar38 + 0x100) + (ulonglong)uVar24 * 0xa0;
        }
        lVar39 = *(longlong *)(uVar38 + 0x1a8) + (ulonglong)*(uint *)(lVar31 + 0x94) * 0x48;
        uVar48 = *(ulonglong *)
                  (*(longlong *)(uVar38 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar31 + 0x94) * 0x48
                  );
        if ((*(ulonglong *)((longlong)puStack_110 + 0x110) < uVar48) &&
           (uVar27 = *(ulonglong *)
                      (*(longlong *)(lVar39 + 0x38) +
                      *(ulonglong *)((longlong)puStack_110 + 0x110) * 8), uVar27 != 0)) {
          puStack_108 = *(uint **)(*(longlong *)(lVar39 + 0x18) + 0x10 + ~uVar27 * 0x30);
          uVar27 = *(ulonglong *)((longlong)puStack_110 + 0x118);
          if (uVar48 <= uVar27) goto LAB_1408d1780;
LAB_1408d1747:
          uVar48 = *(ulonglong *)(*(longlong *)(lVar39 + 0x38) + uVar27 * 8);
          if (uVar48 == 0) goto LAB_1408d1780;
          puStack_148 = *(uint **)(*(longlong *)(lVar39 + 0x18) + 0x10 + ~uVar48 * 0x30);
        }
        else {
          puStack_108 = (uint *)0x0;
          uVar27 = *(ulonglong *)((longlong)puStack_110 + 0x118);
          if (uVar27 < uVar48) goto LAB_1408d1747;
LAB_1408d1780:
          puStack_148 = (uint *)0x0;
        }
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        lStack_380 = *(longlong *)(lVar31 + 0x50);
        uVar48 = *(ulonglong *)(lVar31 + 0x58);
        uVar27 = 0;
        lVar31 = extraout_RAX_09;
        iVar45 = (int)puStack_158;
        uVar24 = (uint)puStack_180;
      }
      uVar51 = (ulonglong)*(uint *)(lStack_380 + 8 + uVar27 * 0x10);
      if ((uint)(iVar45 + *(int *)(lVar31 + uVar51 * 4) * -1) < uVar24) {
        puVar26 = (undefined8 *)(uVar27 * 0x10 + lStack_380);
        uStack_168 = uVar48;
LAB_1408d17f5:
        puVar53 = puStack_208;
        lVar39 = uVar51 * 0x68 + (longlong)puStack_108;
        puStack_130 = (ulonglong *)(uVar51 * 0x30 + (longlong)puStack_148);
        puStack_138 = (uint *)*puVar26;
        uVar48 = *puStack_130;
        uStack_2b8 = (uint *)uVar48;
        if (*(longlong *)((longlong)puStack_208 + 0x18) != 0) {
          uVar27 = uVar27 + 1;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar55 = *(longlong *)puVar53;
          uVar51 = *(ulonglong *)((longlong)puVar53 + 8);
          bVar71 = SUB81(extraout_RAX_10,7) >> 1;
          auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
          auVar62 = pshuflw(auVar62,auVar62,0);
          lVar29 = 0;
          uVar35 = extraout_RAX_10;
          while( true ) {
            uVar35 = uVar51 & uVar35;
            pcVar2 = (char *)(lVar55 + uVar35);
            cVar69 = SUB161(auVar62,0);
            cVar64 = SUB161(auVar62,1);
            cVar65 = SUB161(auVar62,2);
            cVar66 = SUB161(auVar62,3);
            bVar71 = (pcVar2[0xf] == cVar66) * -1;
            auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                                 CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                           CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                                     CONCAT111((pcVar2[0xb] ==
                                                                               cVar66) * -1,
                                                                               CONCAT110((pcVar2[10]
                                                                                         == cVar65)
                                                                                         * -1,
                                                  CONCAT19((pcVar2[9] == cVar64) * -1,
                                                           CONCAT18((pcVar2[8] == cVar69) * -1,
                                                                    CONCAT17((pcVar2[7] == cVar66) *
                                                                             -1,CONCAT16((pcVar2[6]
                                                                                         == cVar65)
                                                                                         * -1,
                                                  CONCAT15((pcVar2[5] == cVar64) * -1,
                                                           CONCAT14((pcVar2[4] == cVar69) * -1,
                                                                    CONCAT13((pcVar2[3] == cVar66) *
                                                                             -1,CONCAT12((pcVar2[2]
                                                                                         == cVar65)
                                                                                         * -1,
                                                  CONCAT11((pcVar2[1] == cVar64) * -1,
                                                           (*pcVar2 == cVar69) * -1)))))))))))))));
            uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
            uVar24 = (uint)uVar54;
            while (uVar54 != 0) {
              uVar25 = 0;
              for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar25 = uVar25 + 1;
              }
              uVar49 = uVar25 + uVar35 & uVar51;
              lVar50 = uVar49 * -0x2a8;
              if (*(ulonglong *)((uint *)(lVar55 + -0x2a8) + uVar49 * -0xaa) == uVar48) {
                if ((*(ulonglong *)(lVar55 + -0x290 + lVar50) < 2) ||
                   (*(float *)(lVar55 + -0x270 + lVar50) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic();
                }
                if (*(char *)(lVar50 + lVar55 + -9) == '\x01') {
                  core::hash::BuildHasher::
                  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                            ();
                  uVar35 = uStack_168;
                  lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3dc8);
                  uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3dd0);
                  bVar71 = SUB81(extraout_RAX_11,7) >> 1;
                  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
                  auVar62 = pshuflw(auVar62,auVar62,0);
                  puVar59 = (ulonglong *)0x0;
                  uVar51 = extraout_RAX_11;
                  goto LAB_1408d1974;
                }
                uVar48 = *puStack_130;
                uStack_2b8 = (uint *)uVar48;
                puStack_128 = (uint *)(lVar55 + -0x2a8);
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                          ();
                puVar53 = puStack_138;
                bVar71 = SUB81(extraout_RAX_12,7) >> 1;
                auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
                auVar62 = pshuflw(auVar62,auVar62,0);
                lVar29 = 0;
                uVar35 = extraout_RAX_12;
                goto LAB_1408d1a4a;
              }
              uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
              uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
            }
            cVar69 = (pcVar2[0xf] == -1) * -1;
            auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                                 CONCAT113((pcVar2[0xd] == -1) * -1,
                                                           CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                     CONCAT111((pcVar2[0xb] == -1) *
                                                                               -1,CONCAT110((pcVar2[
                                                  10] == -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
            if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar63 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar63 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') break;
            uVar35 = uVar35 + lVar29 + 0x10;
            lVar29 = lVar29 + 0x10;
          }
        }
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      uVar27 = uVar27 + 1;
    } while( true );
  }
  if (uVar27 != uVar48) goto LAB_1408d162b;
  while (puVar34 != puVar47) {
    uVar27 = (ulonglong)*puVar34;
    puVar34 = puVar34 + 1;
    lVar31 = *(longlong *)(uVar38 + 0x1a8);
    if (*(longlong *)(lVar31 + 0x10 + uVar27 * 0x48) == 0) {
      do {
        if (puVar34 == puVar47) goto LAB_1408d3eab;
        uVar24 = *puVar34;
        puVar34 = puVar34 + 1;
      } while (*(longlong *)(*(longlong *)(uVar38 + 0x1a8) + 0x10 + (ulonglong)uVar24 * 0x48) == 0);
      lVar39 = *(longlong *)(uVar38 + 0x1a8) + (ulonglong)uVar24 * 0x48;
      puVar59 = (ulonglong *)(lVar39 + 0x10);
      uVar48 = *(ulonglong *)((longlong)puStack_110 + 0x110);
      uVar27 = *(ulonglong *)(lVar39 + 0x40);
      if (uVar48 < uVar27) goto LAB_1408d155b;
LAB_1408d15a0:
      puStack_108 = (uint *)0x0;
      uVar48 = *(ulonglong *)((longlong)puStack_110 + 0x118);
      if (uVar27 <= uVar48) goto LAB_1408d15f0;
LAB_1408d15b7:
      uVar48 = *(ulonglong *)(*(longlong *)(lVar39 + 0x38) + uVar48 * 8);
      if (uVar48 == 0) goto LAB_1408d15f0;
      puStack_148 = *(uint **)(*(longlong *)(lVar39 + 0x18) + 0x10 + ~uVar48 * 0x30);
    }
    else {
      lVar39 = lVar31 + uVar27 * 0x48;
      puVar59 = (ulonglong *)(lVar31 + uVar27 * 0x48 + 0x10);
      uVar48 = *(ulonglong *)((longlong)puStack_110 + 0x110);
      uVar27 = *(ulonglong *)(lVar39 + 0x40);
      if (uVar27 <= uVar48) goto LAB_1408d15a0;
LAB_1408d155b:
      uVar48 = *(ulonglong *)(*(longlong *)(lVar39 + 0x38) + uVar48 * 8);
      if (uVar48 == 0) goto LAB_1408d15a0;
      puStack_108 = *(uint **)(*(longlong *)(lVar39 + 0x18) + 0x10 + ~uVar48 * 0x30);
      uVar48 = *(ulonglong *)((longlong)puStack_110 + 0x118);
      if (uVar48 < uVar27) goto LAB_1408d15b7;
LAB_1408d15f0:
      puStack_148 = (uint *)0x0;
    }
    bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
    lStack_210 = *(longlong *)(lVar39 + 8);
    uVar48 = *puVar59;
    uVar27 = 0;
    lVar31 = extraout_RAX_08;
    iVar45 = (int)puStack_158;
    uVar24 = (uint)puStack_180;
LAB_1408d162b:
    do {
      uVar51 = uVar27 & 0xffffffff;
      if ((uint)(iVar45 + *(int *)(lVar31 + uVar51 * 4) * -1) < uVar24) {
        puVar26 = (undefined8 *)(lStack_210 + uVar27 * 8);
        uStack_168 = uVar48;
        goto LAB_1408d17f5;
      }
      uVar27 = uVar27 + 1;
    } while (uVar27 != uVar48);
  }
LAB_1408d3eab:
  puVar47 = *(uint **)((longlong)wall_removed + 0x10);
  puVar34 = (uint *)**(ulonglong **)wall_removed;
  if (puVar34 < *(ulonglong *)((longlong)puVar47 + 0x40)) {
    uVar27 = *(ulonglong *)(*(longlong *)((longlong)puVar47 + 0x38) + (longlong)puVar34 * 8);
    puVar34 = (uint *)0x0;
    if (uVar27 == 0) goto LAB_1408d3f6c;
    lVar44 = *(longlong *)((longlong)puVar47 + 8);
    uVar27 = ~uVar27;
    puVar34 = (uint *)(uVar27 * 9);
    puVar59 = *(ulonglong **)((longlong)wall_removed + 8);
    uVar38 = *puVar59;
    uVar51 = 0;
    puVar43 = (ulonglong *)(lVar44 + 0x18 + uVar27 * 0x48);
    uVar48 = uVar38 + *puVar43 * -1;
    if (uVar38 < *puVar43) {
      uVar48 = uVar51;
    }
    puVar43 = (ulonglong *)(lVar44 + 0x38 + uVar27 * 0x48);
    uVar35 = uVar38 + *puVar43 * -1;
    if (uVar38 < *puVar43) {
      uVar35 = uVar51;
    }
    uVar38 = uVar48 * 0x10 + *(longlong *)(lVar44 + 8 + uVar27 * 0x48);
    uVar49 = *(ulonglong *)(lVar44 + 0x10 + uVar27 * 0x48);
    uVar5 = *(ulonglong *)(lVar44 + 0x30 + uVar27 * 0x48);
    uVar11 = uVar49 + uVar48 * -1;
    if (uVar49 < uVar48) {
      uVar38 = 8;
      uVar11 = uVar51;
    }
    uVar48 = uVar35 * 0x10 + *(longlong *)(lVar44 + 0x28 + uVar27 * 0x48);
    uVar49 = uVar5 + uVar35 * -1;
    if (uVar5 < uVar35) {
      uVar48 = 8;
      uVar49 = uVar51;
    }
    puVar47 = (uint *)(*(longlong *)(lVar44 + 0x40 + uVar27 * 0x48) + uVar49 * -1 + uVar11 * -1);
    *puVar59 = (ulonglong)puVar47;
    puStack_148 = (uint *)(uVar11 * 0x10 + uVar38);
    puStack_158 = (uint *)(uVar49 * 0x10 + uVar48);
  }
  else {
LAB_1408d3f6c:
    puVar59 = (ulonglong *)0x0;
  }
  puVar43 = (ulonglong *)0x0;
  goto LAB_1408d4036;
LAB_1408d1a4a:
  uVar35 = uVar51 & uVar35;
  pcVar2 = (char *)(lVar55 + uVar35);
  cVar69 = SUB161(auVar62,0);
  cVar64 = SUB161(auVar62,1);
  cVar65 = SUB161(auVar62,2);
  cVar66 = SUB161(auVar62,3);
  bVar71 = (pcVar2[0xf] == cVar66) * -1;
  auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                       CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                 CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                           CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                     CONCAT110((pcVar2[10] == cVar65
                                                                               ) * -1,CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
  uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
  uVar24 = (uint)uVar54;
  while (uVar54 != 0) {
    uVar25 = 0;
    for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
      uVar25 = uVar25 + 1;
    }
    uVar49 = uVar25 + uVar35 & uVar51;
    lVar50 = uVar49 * -0x2a8;
    if (*(ulonglong *)(puStack_128 + uVar49 * -0xaa) == uVar48) {
      if ((*(ulonglong *)(lVar55 + -0x290 + lVar50) < 2) ||
         (*(float *)(lVar55 + -0x270 + lVar50) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      lVar55 = lVar55 + lVar50;
      uVar24 = *(uint *)(lVar55 + -0x48);
      cStack_189 = *(char *)(lVar55 + -10);
      if (cStack_189 == '\x01') {
        if (*(longlong *)((longlong)puStack_1d8 + 0x18) == 0) goto LAB_1408d27c5;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar29 = *(longlong *)puStack_1d8;
        bVar71 = SUB81(extraout_RAX_13,7) >> 1;
        auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
        auVar62 = pshuflw(auVar62,auVar62,0);
        puVar40 = (uint *)0x0;
        puVar52 = extraout_RAX_13;
        goto LAB_1408d1b5d;
      }
      if (*(longlong *)(puStack_1c0 + 6) == 0) goto LAB_1408d27c5;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar29 = *(longlong *)puStack_1c0;
      bVar71 = SUB81(extraout_RAX_14,7) >> 1;
      auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
      auVar62 = pshuflw(auVar62,auVar62,0);
      puVar40 = (uint *)0x0;
      puVar52 = extraout_RAX_14;
      goto LAB_1408d1ca5;
    }
    uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
    uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
  }
  cVar69 = (pcVar2[0xf] == -1) * -1;
  auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') {
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar35 = uVar35 + lVar29 + 0x10;
  lVar29 = lVar29 + 0x10;
  goto LAB_1408d1a4a;
LAB_1408d1ca5:
  uVar48 = *(ulonglong *)(puStack_1c0 + 2) & (ulonglong)puVar52;
  pcVar2 = (char *)(lVar29 + uVar48);
  cVar69 = SUB161(auVar62,0);
  cVar64 = SUB161(auVar62,1);
  cVar65 = SUB161(auVar62,2);
  cVar66 = SUB161(auVar62,3);
  bVar71 = (pcVar2[0xf] == cVar66) * -1;
  auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                       CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                 CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                           CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                     CONCAT110((pcVar2[10] == cVar65
                                                                               ) * -1,CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
  uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
  uVar25 = (uint)uVar54;
  if (uVar54 != 0) {
LAB_1408d1cce:
    uVar9 = 0;
    for (uVar10 = uVar25; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
      uVar9 = uVar9 + 1;
    }
    uVar51 = uVar9 + uVar48 & *(ulonglong *)(puStack_1c0 + 2);
    if (*puStack_130 != *(ulonglong *)(lVar29 + -0x20 + uVar51 * -0x20)) goto code_r0x0001408d1cef;
    uStack_228 = 0;
    puStack_220 = (undefined8 *)&DAT_00000004;
    uStack_218 = 0;
    lVar50 = uVar51 * -0x20;
    puVar53 = *(uint **)(lVar29 + -0x10 + lVar50);
    puStack_128 = puVar53 + *(longlong *)(lVar29 + -8 + lVar50) * 0x1a;
    for (; puVar53 != puStack_128; puVar53 = puVar53 + 0x1a) {
      construct_arch_geo();
    }
    goto LAB_1408d2772;
  }
LAB_1408d1d0a:
  cVar69 = (pcVar2[0xf] == -1) * -1;
  auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d27c5;
  puVar52 = (undefined1 *)((longlong)puVar40 + uVar48 + 0x10);
  puVar40 = puVar40 + 4;
  goto LAB_1408d1ca5;
code_r0x0001408d1cef:
  uVar54 = (ushort)(uVar25 + 0xffffffff) & (ushort)uVar25;
  uVar25 = CONCAT22(SUB42(uVar25 + 0xffffffff,2),uVar54);
  puVar53 = puStack_138;
  puStack_128 = puVar40;
  if (uVar54 == 0) goto LAB_1408d1d0a;
  goto LAB_1408d1cce;
LAB_1408d1b5d:
  uVar48 = *(ulonglong *)((longlong)puStack_1d8 + 8) & (ulonglong)puVar52;
  pcVar2 = (char *)(lVar29 + uVar48);
  cVar69 = SUB161(auVar62,0);
  cVar64 = SUB161(auVar62,1);
  cVar65 = SUB161(auVar62,2);
  cVar66 = SUB161(auVar62,3);
  bVar71 = (pcVar2[0xf] == cVar66) * -1;
  auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                       CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                 CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                           CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                     CONCAT110((pcVar2[10] == cVar65
                                                                               ) * -1,CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
  uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
  uVar25 = (uint)uVar54;
  if (uVar54 != 0) {
LAB_1408d1b86:
    uVar9 = 0;
    for (uVar10 = uVar25; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
      uVar9 = uVar9 + 1;
    }
    uVar51 = uVar9 + uVar48 & *(ulonglong *)((longlong)puStack_1d8 + 8);
    if (*puStack_130 != *(ulonglong *)(lVar29 + -0x20 + uVar51 * -0x20)) goto code_r0x0001408d1ba7;
    uStack_228 = 0;
    puStack_220 = (undefined8 *)&DAT_00000004;
    uStack_218 = 0;
    lVar50 = uVar51 * -0x20;
    puVar53 = *(uint **)(lVar29 + -0x10 + lVar50);
    puStack_128 = puVar53 + *(longlong *)(lVar29 + -8 + lVar50) * 0x1a;
    for (; puVar53 != puStack_128; puVar53 = puVar53 + 0x1a) {
      construct_arch_geo();
    }
LAB_1408d2772:
    puVar53 = puStack_138;
    uStack_2e0 = 0;
    puStack_2e8 = (undefined8 *)&DAT_00000004;
    lStack_2f0 = 0;
    if (uStack_228 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    goto LAB_1408d27c5;
  }
LAB_1408d1bc2:
  cVar69 = (pcVar2[0xf] == -1) * -1;
  auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d27c5;
  puVar52 = (undefined1 *)((longlong)puVar40 + uVar48 + 0x10);
  puVar40 = puVar40 + 4;
  goto LAB_1408d1b5d;
code_r0x0001408d1ba7:
  uVar54 = (ushort)(uVar25 + 0xffffffff) & (ushort)uVar25;
  uVar25 = CONCAT22(SUB42(uVar25 + 0xffffffff,2),uVar54);
  puVar53 = puStack_138;
  puStack_128 = puVar40;
  if (uVar54 == 0) goto LAB_1408d1bc2;
  goto LAB_1408d1b86;
LAB_1408d27c5:
  uStack_378._4_4_ = SUB84(puVar53,4);
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar29 = *(longlong *)((longlong)plStack_f8 + 0x3e58);
  uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3e60);
  bVar71 = SUB81(extraout_RAX_17,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  puVar53 = (uint *)0x0;
  puVar52 = extraout_RAX_17;
  do {
    uVar51 = uVar48 & (ulonglong)puVar52;
    pcVar2 = (char *)(lVar29 + uVar51);
    cVar69 = SUB161(auVar62,0);
    cVar64 = SUB161(auVar62,1);
    cVar65 = SUB161(auVar62,2);
    cVar66 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar66) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar65) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar64)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar25 = (uint)uVar54;
    if (uVar54 != 0) {
LAB_1408d2833:
      uVar9 = 0;
      for (uVar10 = uVar25; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      uVar35 = uVar9 + uVar51 & uVar48;
      puStack_128 = puVar53;
      if (*(uint **)(lVar29 + -0x20 + uVar35 * -0x20) != puStack_138) break;
      lVar50 = (longlong)(uVar35 * 0x20) >> 5;
      uVar48 = lVar50 + 0xfffffffffffffff0U & uVar48;
      pcVar2 = (char *)(lVar29 + uVar48);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      if (uVar54 == 0) {
        uVar37 = 0x10;
      }
      else {
        uVar37 = 0xf;
        if (uVar54 != 0) {
          for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
          }
        }
        uVar37 = uVar37 ^ 0xf;
      }
      pcVar2 = (char *)(lVar29 + lVar50);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      sVar8 = 0;
      for (uVar25 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                            (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar25 & 1) == 0;
          uVar25 = uVar25 >> 1 | 0x80000000) {
        sVar8 = sVar8 + 1;
      }
      uVar57 = 0x80;
      if ((ushort)(uVar37 + sVar8) < 0x10) {
        *(longlong *)((longlong)plStack_f8 + 0x3e68) =
             *(longlong *)((longlong)plStack_f8 + 0x3e68) + 1;
        uVar57 = 0xff;
      }
      lVar30 = uVar35 * -0x20 + lVar29;
      *(undefined1 *)(lVar29 + lVar50) = uVar57;
      *(undefined1 *)(lVar29 + 0x10 + uVar48) = uVar57;
      *(longlong *)((longlong)plStack_f8 + 0x3e70) =
           *(longlong *)((longlong)plStack_f8 + 0x3e70) + -1;
      _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar30 + -8));
      uStack_378._4_4_ = SUB84(*(undefined8 *)(lVar30 + -0x18),4);
      uStack_370 = *(uint **)(lVar30 + -0x10);
      dispose_collider_mesh_instance();
      if ((char)puStack_298 != '\x11') {
        LOCK();
        *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
        UNLOCK();
        if (*(longlong *)puStack_280 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      goto LAB_1408d298e;
    }
LAB_1408d286a:
    cVar69 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d28b5;
    puVar52 = (undefined1 *)((longlong)puVar53 + uVar51 + 0x10);
    puVar53 = puVar53 + 4;
  } while( true );
  uVar54 = (ushort)(uVar25 + 0xffffffff) & (ushort)uVar25;
  uVar25 = CONCAT22(SUB42(uVar25 + 0xffffffff,2),uVar54);
  if (uVar54 == 0) goto LAB_1408d286a;
  goto LAB_1408d2833;
LAB_1408d28b5:
  if (*(longlong *)((longlong)plStack_f8 + 0x3e68) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d298e:
  auVar62 = _auStack_368;
  uVar48 = *puStack_130;
  uStack_2b8 = (uint *)uVar48;
  if (*(longlong *)((longlong)puStack_208 + 0x48) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar29 = *(longlong *)((longlong)puStack_208 + 0x30);
    bVar71 = SUB81(extraout_RAX_18,7) >> 1;
    auVar63 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar63 = pshuflw(auVar63,auVar63,0);
    puVar53 = (uint *)0x0;
    puVar52 = extraout_RAX_18;
    while( true ) {
      uVar51 = *(ulonglong *)((longlong)puStack_208 + 0x38) & (ulonglong)puVar52;
      pcVar2 = (char *)(lVar29 + uVar51);
      cVar69 = SUB161(auVar63,0);
      cVar64 = SUB161(auVar63,1);
      cVar65 = SUB161(auVar63,2);
      cVar66 = SUB161(auVar63,3);
      bVar71 = (pcVar2[0xf] == cVar66) * -1;
      auVar68 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar66) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar65) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar68 >> 7,0) & 1) | (ushort)(SUB161(auVar68 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar68 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar68 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar68 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar68 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar68 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar68 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar68 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar68 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar68 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar68 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar68 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar68 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar68 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar25 = (uint)uVar54;
      while (uVar54 != 0) {
        uVar9 = 0;
        for (uVar10 = uVar25; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
          uVar9 = uVar9 + 1;
        }
        uVar35 = uVar9 + uVar51 & *(ulonglong *)((longlong)puStack_208 + 0x38);
        puStack_128 = puVar53;
        if (*(ulonglong *)(lVar29 + -0x10 + uVar35 * -0x10) == uVar48) {
          uVar48 = *(ulonglong *)(lVar29 + -8 + uVar35 * -0x10);
          if (((uVar48 >> 0x20 == 0) ||
              (uVar51 = uVar48 & 0xffffffff, *(ulonglong *)(lVar28 + 0x10) <= uVar51)) ||
             (*(int *)(*(longlong *)(lVar28 + 8) + uVar51 * 0x14) != SUB84(uVar48,4)))
          goto LAB_1408d2b3a;
          lVar29 = *(longlong *)(lVar28 + 8) + uVar51 * 0x14;
          uVar48 = (ulonglong)*(uint *)(lVar29 + 4);
          if (((uVar48 == 0xffffffff) || (*(ulonglong *)(lVar44 + 0x38) <= uVar48)) ||
             ((*(ulonglong *)
                (*(longlong *)(lVar44 + 0x28) + (ulonglong)(*(uint *)(lVar29 + 4) >> 6) * 8) >>
               (uVar48 & 0x3f) & 1) == 0)) goto LAB_1408d2b3a;
          bVar7 = *(float *)(*(longlong *)
                              (*(longlong *)
                                (*(longlong *)(lVar28 + 0x1a8) + 0x18 +
                                (ulonglong)*(uint *)(lVar29 + 0xc) * 0x48) + 0x10 +
                              ~*(ulonglong *)
                                (*(longlong *)
                                  (*(longlong *)(lVar28 + 0x1a8) + 0x38 +
                                  (ulonglong)*(uint *)(lVar29 + 0xc) * 0x48) +
                                *(longlong *)(lVar44 + 0x110) * 8) * 0x30) + 0x30 +
                            (ulonglong)*(uint *)(lVar29 + 0x10) * 0x58) <= fVar72;
          goto LAB_1408d2b3c;
        }
        uVar54 = (ushort)(uVar25 + 0xffffffff) & (ushort)uVar25;
        uVar25 = CONCAT22(SUB42(uVar25 + 0xffffffff,2),uVar54);
      }
      cVar69 = (pcVar2[0xf] == -1) * -1;
      auVar68 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar68 >> 7,0) & 1) != 0 || (SUB161(auVar68 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar68 >> 0x17,0) & 1) != 0) || (SUB161(auVar68 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar68 >> 0x27,0) & 1) != 0) || (SUB161(auVar68 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar68 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar68 >> 0x3f,0) & 1) != 0) || (SUB161(auVar68 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar68 >> 0x4f,0) & 1) != 0) || (SUB161(auVar68 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar68 >> 0x5f,0) & 1) != 0) || (SUB161(auVar68 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar68 >> 0x6f,0) & 1) != 0) || (SUB161(auVar68 >> 0x77,0) & 1) != 0) ||
          cVar69 < '\0') break;
      puVar52 = (undefined1 *)((longlong)puVar53 + uVar51 + 0x10);
      puVar53 = puVar53 + 4;
    }
  }
LAB_1408d2b3a:
  bVar7 = false;
LAB_1408d2b3c:
  if (*(longlong *)(lVar39 + 0x10) == 0) {
    uStack_378 = puStack_138;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    uVar51 = uStack_168;
    lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3e28);
    uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3e30);
    bVar71 = SUB81(extraout_RAX_20,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    puVar53 = (uint *)0x0;
    puVar52 = extraout_RAX_20;
    do {
      uVar35 = uVar48 & (ulonglong)puVar52;
      auVar63 = *(undefined1 (*) [16])(lVar39 + uVar35);
      cVar69 = SUB161(auVar62,0);
      cVar64 = SUB161(auVar62,1);
      cVar65 = SUB161(auVar62,2);
      cVar66 = SUB161(auVar62,3);
      bVar71 = (SUB161(auVar63,0xf) == cVar66) * -1;
      auVar68 = CONCAT115(bVar71,CONCAT114((SUB161(auVar63,0xe) == cVar65) * -1,
                                           CONCAT113((SUB161(auVar63,0xd) == cVar64) * -1,
                                                     CONCAT112((SUB161(auVar63,0xc) == cVar69) * -1,
                                                               CONCAT111((SUB161(auVar63,0xb) ==
                                                                         cVar66) * -1,
                                                                         CONCAT110((SUB161(auVar63,
                                                  10) == cVar65) * -1,
                                                  CONCAT19((SUB161(auVar63,9) == cVar64) * -1,
                                                           CONCAT18((SUB161(auVar63,8) == cVar69) *
                                                                    -1,CONCAT17((SUB161(auVar63,7)
                                                                                == cVar66) * -1,
                                                                                CONCAT16((SUB161(
                                                  auVar63,6) == cVar65) * -1,
                                                  CONCAT15((SUB161(auVar63,5) == cVar64) * -1,
                                                           CONCAT14((SUB161(auVar63,4) == cVar69) *
                                                                    -1,CONCAT13((SUB161(auVar63,3)
                                                                                == cVar66) * -1,
                                                                                CONCAT12((SUB161(
                                                  auVar63,2) == cVar65) * -1,
                                                  CONCAT11((SUB161(auVar63,1) == cVar64) * -1,
                                                           (SUB161(auVar63,0) == cVar69) * -1)))))))
                                                  ))))))));
      uVar54 = (ushort)(SUB161(auVar68 >> 7,0) & 1) | (ushort)(SUB161(auVar68 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar68 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar68 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar68 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar68 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar68 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar68 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar68 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar68 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar68 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar68 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar68 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar68 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar68 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar24 = (uint)uVar54;
      if (uVar54 != 0) {
LAB_1408d2e90:
        uVar25 = 0;
        for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
          uVar25 = uVar25 + 1;
        }
        uVar49 = uVar25 + uVar35 & uVar48;
        puStack_128 = puVar53;
        if (*(uint **)(lVar39 + -0x20 + uVar49 * -0x20) != puStack_138) goto code_r0x0001408d2eb6;
        lVar29 = (longlong)(uVar49 * 0x20) >> 5;
        uVar48 = lVar29 + 0xfffffffffffffff0U & uVar48;
        pcVar2 = (char *)(lVar39 + uVar48);
        bVar71 = (pcVar2[0xf] == -1) * -1;
        auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                             CONCAT113((pcVar2[0xd] == -1) * -1,
                                                       CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                 CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                           CONCAT110((pcVar2[10] ==
                                                                                     -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
        uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
        if (uVar54 == 0) {
          uVar37 = 0x10;
        }
        else {
          uVar37 = 0xf;
          if (uVar54 != 0) {
            for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
            }
          }
          uVar37 = uVar37 ^ 0xf;
        }
        pcVar2 = (char *)(lVar39 + lVar29);
        bVar71 = (pcVar2[0xf] == -1) * -1;
        auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                             CONCAT113((pcVar2[0xd] == -1) * -1,
                                                       CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                 CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                           CONCAT110((pcVar2[10] ==
                                                                                     -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
        sVar8 = 0;
        for (uVar24 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                              (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar24 & 1) == 0;
            uVar24 = uVar24 >> 1 | 0x80000000) {
          sVar8 = sVar8 + 1;
        }
        uVar57 = 0x80;
        if ((ushort)(uVar37 + sVar8) < 0x10) {
          *(longlong *)((longlong)plStack_f8 + 0x3e38) =
               *(longlong *)((longlong)plStack_f8 + 0x3e38) + 1;
          uVar57 = 0xff;
        }
        lVar50 = uVar49 * -0x20 + lVar39;
        *(undefined1 *)(lVar39 + lVar29) = uVar57;
        *(undefined1 *)(lVar39 + 0x10 + uVar48) = uVar57;
        *(longlong *)((longlong)plStack_f8 + 0x3e40) =
             *(longlong *)((longlong)plStack_f8 + 0x3e40) + -1;
        _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar50 + -8));
        uStack_378 = *(uint **)(lVar50 + -0x18);
        uStack_370 = *(uint **)(lVar50 + -0x10);
        dispose_collider_mesh_instance();
        if ((char)puStack_298 != '\x11') {
          LOCK();
          *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
          UNLOCK();
          if (*(longlong *)puStack_280 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
        goto LAB_1408d2fd0;
      }
LAB_1408d2ed1:
      cVar69 = (SUB161(auVar63,0xf) == -1) * -1;
      auVar63 = CONCAT115(cVar69,CONCAT114((SUB161(auVar63,0xe) == -1) * -1,
                                           CONCAT113((SUB161(auVar63,0xd) == -1) * -1,
                                                     CONCAT112((SUB161(auVar63,0xc) == -1) * -1,
                                                               CONCAT111((SUB161(auVar63,0xb) == -1)
                                                                         * -1,CONCAT110((SUB161(
                                                  auVar63,10) == -1) * -1,
                                                  CONCAT19((SUB161(auVar63,9) == -1) * -1,
                                                           CONCAT18((SUB161(auVar63,8) == -1) * -1,
                                                                    CONCAT17((SUB161(auVar63,7) ==
                                                                             -1) * -1,CONCAT16((
                                                  SUB161(auVar63,6) == -1) * -1,
                                                  CONCAT15((SUB161(auVar63,5) == -1) * -1,
                                                           CONCAT14((SUB161(auVar63,4) == -1) * -1,
                                                                    CONCAT13((SUB161(auVar63,3) ==
                                                                             -1) * -1,CONCAT12((
                                                  SUB161(auVar63,2) == -1) * -1,
                                                  CONCAT11((SUB161(auVar63,1) == -1) * -1,
                                                           (SUB161(auVar63,0) == -1) * -1)))))))))))
                                                  ))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar69 < '\0') goto LAB_1408d3550;
      puVar52 = (undefined1 *)((longlong)puVar53 + uVar35 + 0x10);
      puVar53 = puVar53 + 4;
    } while( true );
  }
  uVar25 = uVar23;
  if (1 < uVar24) {
    uVar25 = 0;
  }
  uVar67 = uVar16;
  if (!bVar7) {
    uVar67 = 0;
  }
  uStack_370 = (uint *)CONCAT44(1,(undefined4)uStack_370);
  auStack_368 = (undefined1  [8])(ulonglong)uVar25;
  uStack_360._5_3_ = SUB163(auVar62,0xd);
  _auStack_368 = CONCAT112(1,CONCAT48(uVar67,auStack_368));
  resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo();
  puStack_1a8 = (uint *)uStack_278;
  puStack_1b8 = (uint *)lStack_288;
  puStack_1b0 = puStack_280;
  RaycastMeshData::new_unindexed();
  uStack_2b8 = (uint *)0x1;
  uStack_2b0._0_4_ = 1;
  uStack_2b0._4_4_ = 0.0;
  uStack_238 = uStack_308;
  uStack_248 = uStack_318;
  lStack_240 = lStack_310;
  uStack_258 = uStack_328;
  uStack_250 = uStack_320;
  uStack_268 = uStack_338;
  puStack_260 = puStack_330;
  uStack_278 = uStack_348;
  puStack_280 = puStack_350;
  puStack_298 = (uint *)auStack_368;
  uStack_290 = uStack_360;
  fStack_2a8 = 0.0;
  puStack_2a0 = uStack_370;
  fStack_2a4 = (float)uStack_378._4_4_;
  lStack_288 = lStack_358;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst_00 == (uint *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst_00,&uStack_2b8,0x90);
  puStack_3a8 = (uint *)*puStack_130;
  uStack_3b0 = (uint *)CONCAT71(uStack_3b0._1_7_,cStack_189 << 3);
  uStack_2b8 = puStack_138;
  uStack_115 = 1;
  puStack_128 = _Dst_00;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3dc8);
  bVar71 = SUB81(extraout_RAX_19,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  lVar29 = 0;
  puVar53 = extraout_RAX_19;
  while( true ) {
    uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3dd0) & (ulonglong)puVar53;
    pcVar2 = (char *)(lVar39 + uVar48);
    cVar69 = SUB161(auVar62,0);
    cVar64 = SUB161(auVar62,1);
    cVar65 = SUB161(auVar62,2);
    cVar66 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar66) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar65) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar64)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar24 = (uint)uVar54;
    while (uVar54 != 0) {
      uVar25 = 0;
      for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar25 = uVar25 + 1;
      }
      uVar51 = uVar25 + uVar48 & *(ulonglong *)((longlong)plStack_f8 + 0x3dd0);
      if (*(uint **)(lVar39 + -0x20 + uVar51 * -0x20) == puStack_138) {
        puStack_1b0 = (uint *)(lVar39 + uVar51 * -0x20);
        puStack_1a8 = puStack_178;
        puStack_1b8 = (uint *)0x0;
        goto LAB_1408d2f5f;
      }
      uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
      uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
    }
    cVar69 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') break;
    puVar53 = (uint *)(uVar48 + lVar29 + 0x10);
    lVar29 = lVar29 + 0x10;
  }
  puVar53 = puStack_138;
  if (*(longlong *)((longlong)plStack_f8 + 0x3dd8) == 0) {
    uStack_115 = 1;
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
    puVar53 = uStack_2b8;
  }
  puStack_1b8 = puStack_178;
  puStack_1b0 = puVar53;
  puStack_1a8 = extraout_RAX_19;
LAB_1408d2f5f:
  _auStack_368 = CONCAT88(_Dst_00,puStack_3a0);
  uStack_378 = uStack_3b0;
  uStack_2b0._0_4_ = 0;
  uStack_2b8 = (uint *)0x0;
  puStack_2a0 = (uint *)lVar42;
  uStack_115 = 0;
  auVar63 = _auStack_368;
  fStack_2a8 = fVar61;
  fStack_2a4 = fVar21;
  upsert_collider_entry<bevy_ecs::entity::Entity,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
            ();
  uVar51 = uStack_168;
  puStack_398 = _Dst_00;
  uStack_370 = puStack_3a8;
LAB_1408d2fd0:
  if ((*(float *)(lVar55 + -0x24) <= 0.0) || (1 < *(uint *)(lVar55 + -0x48))) {
    uStack_378 = puStack_138;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    uVar48 = uStack_168;
    lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3e28);
    uVar35 = *(ulonglong *)((longlong)plStack_f8 + 0x3e30);
    bVar71 = SUB81(extraout_RAX_24,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    puVar59 = (ulonglong *)0x0;
    uVar49 = extraout_RAX_24;
    do {
      uVar49 = uVar35 & uVar49;
      pcVar2 = (char *)(lVar39 + uVar49);
      cVar69 = SUB161(auVar62,0);
      cVar64 = SUB161(auVar62,1);
      cVar65 = SUB161(auVar62,2);
      cVar66 = SUB161(auVar62,3);
      bVar71 = (pcVar2[0xf] == cVar66) * -1;
      auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar66) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar65) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar24 = (uint)uVar54;
      if (uVar54 != 0) {
LAB_1408d33c2:
        uVar25 = 0;
        for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
          uVar25 = uVar25 + 1;
        }
        uVar51 = uVar25 + uVar49 & uVar35;
        puStack_130 = puVar59;
        if (*(uint **)(lVar39 + -0x20 + uVar51 * -0x20) != puStack_138) goto code_r0x0001408d33e8;
        lVar55 = (longlong)(uVar51 * 0x20) >> 5;
        uVar35 = lVar55 + 0xfffffffffffffff0U & uVar35;
        pcVar2 = (char *)(lVar39 + uVar35);
        bVar71 = (pcVar2[0xf] == -1) * -1;
        auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                             CONCAT113((pcVar2[0xd] == -1) * -1,
                                                       CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                 CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                           CONCAT110((pcVar2[10] ==
                                                                                     -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
        uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
        if (uVar54 == 0) {
          uVar37 = 0x10;
        }
        else {
          uVar37 = 0xf;
          if (uVar54 != 0) {
            for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
            }
          }
          uVar37 = uVar37 ^ 0xf;
        }
        pcVar2 = (char *)(lVar39 + lVar55);
        bVar71 = (pcVar2[0xf] == -1) * -1;
        auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                             CONCAT113((pcVar2[0xd] == -1) * -1,
                                                       CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                 CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                           CONCAT110((pcVar2[10] ==
                                                                                     -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
        sVar8 = 0;
        for (uVar24 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                              (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar24 & 1) == 0;
            uVar24 = uVar24 >> 1 | 0x80000000) {
          sVar8 = sVar8 + 1;
        }
        uVar57 = 0x80;
        if ((ushort)(uVar37 + sVar8) < 0x10) {
          *(longlong *)((longlong)plStack_f8 + 0x3e38) =
               *(longlong *)((longlong)plStack_f8 + 0x3e38) + 1;
          uVar57 = 0xff;
        }
        lVar29 = uVar51 * -0x20 + lVar39;
        *(undefined1 *)(lVar39 + lVar55) = uVar57;
        *(undefined1 *)(lVar39 + 0x10 + uVar35) = uVar57;
        *(longlong *)((longlong)plStack_f8 + 0x3e40) =
             *(longlong *)((longlong)plStack_f8 + 0x3e40) + -1;
        _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar29 + -8));
        uStack_378 = *(uint **)(lVar29 + -0x18);
        uStack_370 = *(uint **)(lVar29 + -0x10);
        dispose_collider_mesh_instance();
        if ((char)puStack_298 != '\x11') {
          LOCK();
          *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
          UNLOCK();
          if (*(longlong *)puStack_280 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
        goto LAB_1408d352e;
      }
LAB_1408d3403:
      cVar69 = (pcVar2[0xf] == -1) * -1;
      auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar69 < '\0') goto LAB_1408d344e;
      uVar49 = (longlong)puVar59 + uVar49 + 0x10;
      puVar59 = puVar59 + 2;
    } while( true );
  }
  puStack_130 = (ulonglong *)*puStack_130;
  if ((*(uint *)(lVar55 + -0x48) & 1) != 0) {
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    uStack_228 = CONCAT44(SUB164(auVar63,0),extraout_XMM0_Da);
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    uStack_1f8 = (uint *)CONCAT44(SUB164(auVar63,0),extraout_XMM0_Da_00);
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    uVar67 = SUB164(auVar63,0);
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    uStack_2b8 = (uint *)CONCAT44(SUB164(auVar63,0),extraout_XMM0_Da_02);
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    std::alloc::__default_lib_allocator::__rust_alloc();
    uVar19 = _UNK_142aa32a8;
    if (extraout_RAX_21 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_21 = ___xmm_00000000000000020000000100000000;
    extraout_RAX_21[1] = uVar19;
    extraout_RAX_21[2] = 0x300000002;
    lStack_2f0 = 2;
    uStack_2e0 = 2;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_22 == (undefined8 *)0x0) {
      uStack_117 = 1;
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    extraout_RAX_22[4] = uStack_390;
    extraout_RAX_22[5] = uStack_388;
    extraout_RAX_22[2] = puStack_3a0;
    extraout_RAX_22[3] = puStack_398;
    *extraout_RAX_22 = uStack_3b0;
    extraout_RAX_22[1] = puStack_3a8;
    uStack_228 = 4;
    uStack_218 = 4;
    puStack_220 = extraout_RAX_22;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_23 == (undefined8 *)0x0) {
      uStack_14b = 1;
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    *extraout_RAX_23 = uVar17;
    extraout_RAX_23[1] = uVar18;
    uVar19 = _UNK_142aa32b8;
    extraout_RAX_23[2] = ___xmm_bf8000008000000080000000bf800000;
    extraout_RAX_23[3] = uVar19;
    uVar19 = _UNK_142aa32c8;
    extraout_RAX_23[4] = ___xmm_80000000bf8000008000000080000000;
    extraout_RAX_23[5] = uVar19;
    uStack_1f8 = (uint *)&DAT_00000004;
    puStack_1e8 = (uint *)&DAT_00000004;
    uStack_2b8 = (uint *)0x8000000000000000;
    uStack_14b = 0;
    puStack_1f0 = extraout_RAX_23;
    RaycastMeshData::new_indexed();
    uStack_2b8 = (uint *)0x1;
    uStack_2b0._0_4_ = 1;
    uStack_2b0._4_4_ = 0.0;
    uStack_238 = uStack_308;
    uStack_248 = uStack_318;
    lStack_240 = lStack_310;
    uStack_258 = uStack_328;
    uStack_250 = uStack_320;
    uStack_268 = uStack_338;
    puStack_260 = puStack_330;
    uStack_278 = uStack_348;
    puStack_280 = puStack_350;
    puStack_298 = (uint *)auStack_368;
    uStack_290 = uStack_360;
    puStack_2a0 = uStack_370;
    fStack_2a8 = (float)extraout_XMM0_Da_01;
    fStack_2a4 = (float)uVar67;
    lStack_288 = lStack_358;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst_01 == (uint *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    puStack_128 = _Dst_01;
    memcpy(_Dst_01,&uStack_2b8,0x90);
    puStack_1a0 = puStack_128;
    puStack_1b8 = (uint *)CONCAT71(puStack_1b8._1_7_,1);
    puStack_1b0 = (uint *)puStack_130;
    puStack_2e8 = extraout_RAX_21;
    goto LAB_1408d3a8d;
  }
  ::utils::swizzlers::impl$1::xvy();
  ::utils::geometry::circle::Circle2d::as_points3();
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl_0::as_points2_ptcount::closure_env_0>,utils::geometry::circle::impl_0::as_points3::closure_env_0>_>
            ();
  uStack_116 = 1;
  ::utils::geometry::circle::Circle2d::as_points2();
  uStack_116 = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl_0::as_points2_ptcount::closure_env_0>_>
            ();
  if (puStack_3f0 == puStack_400) {
    uStack_f9 = 1;
    alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
  }
  if (puStack_3f0 != (uint *)0x0) {
    memmove((void *)((longlong)puStack_3f8 + 0xc),puStack_3f8,(longlong)puStack_3f0 * 0xc);
  }
  *(undefined4 *)(puStack_3f8 + 1) = uStack_41c;
  *puStack_3f8 = CONCAT44(uStack_420,uStack_424);
  puVar53 = (uint *)((longlong)puStack_3f0 + 1);
  if (lStack_408 == lStack_418) {
    uStack_f9 = 1;
    alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
  }
  if (lStack_408 != 0) {
    memmove(puStack_410 + 2,puStack_410,lStack_408 * 8);
  }
  *puStack_410 = uStack_424;
  puStack_410[1] = uStack_41c;
  lStack_408 = lStack_408 + 1;
  uStack_2b8 = (uint *)0x0;
  puStack_2a0 = (uint *)0x1;
  uStack_f9 = 1;
  puStack_298 = puVar53;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<array_<u32,3>,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::ops::range::Range<usize>,tuple_<usize,usize>_>,country_core::systems::collision::world::floor_mesh::closure_env_0>_>
            ();
  uStack_218 = uStack_2e0;
  uStack_228 = lStack_2f0;
  puStack_220 = puStack_2e8;
  uStack_1f8 = puStack_400;
  puStack_1f0 = puStack_3f8;
  fVar72 = __real_3dcccccd;
  puStack_1e8 = puVar53;
  if (puVar53 != (uint *)0x0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    fVar72 = __real_3dcccccd;
    if (extraout_RAX_25 == (undefined8 *)0x0) {
      uStack_14a = 1;
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    puVar26 = extraout_RAX_25;
    if (puVar53 != (uint *)0x1) {
      puVar40 = (uint *)0x1;
      if (&DAT_00000004 < puVar53) {
        puStack_128 = puStack_3f0;
        puVar60 = (uint *)((ulonglong)puStack_3f0 & 0xfffffffffffffffc);
        puVar26 = (undefined8 *)((longlong)extraout_RAX_25 + (longlong)puVar60 * 0xc);
        puVar40 = (uint *)((longlong)puVar60 + 1);
        puVar56 = extraout_RAX_25;
        puVar41 = puVar60;
        do {
          puVar56[2] = 0x80000000bf800000;
          *puVar56 = uVar17;
          puVar56[1] = uVar18;
          puVar56[5] = 0x80000000bf800000;
          puVar56[3] = uVar17;
          puVar56[4] = uVar18;
          puVar56 = puVar56 + 6;
          puVar41 = puVar41 + -1;
        } while (puVar41 != (uint *)0x0);
        if (puStack_3f0 == puVar60) goto LAB_1408d37e6;
      }
      do {
        puVar40 = (uint *)((longlong)puVar40 + 1);
        *puVar26 = uVar15;
        *(undefined4 *)(puVar26 + 1) = 0x80000000;
        puVar26 = (undefined8 *)((longlong)puVar26 + 0xc);
      } while (puVar53 != puVar40);
    }
LAB_1408d37e6:
    *puVar26 = uVar15;
    *(undefined4 *)(puVar26 + 1) = 0x80000000;
  }
  uStack_2b8 = (uint *)0x8000000000000000;
  uStack_14a = 0;
  RaycastMeshData::new_indexed();
  uStack_2b8 = (uint *)0x1;
  uStack_2b0._0_4_ = 1;
  uStack_2b0._4_4_ = 0.0;
  uStack_238 = uStack_308;
  uStack_248 = uStack_318;
  lStack_240 = lStack_310;
  uStack_258 = uStack_328;
  uStack_250 = uStack_320;
  uStack_268 = uStack_338;
  puStack_260 = puStack_330;
  uStack_278 = uStack_348;
  puStack_280 = puStack_350;
  puStack_298 = (uint *)auStack_368;
  uStack_290 = uStack_360;
  fStack_2a8 = SUB84(uStack_378,0);
  fStack_2a4 = SUB84(uStack_378,4);
  puStack_2a0 = uStack_370;
  lStack_288 = lStack_358;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst_02 == (uint *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst_02,&uStack_2b8,0x90);
  puStack_1b8 = (uint *)CONCAT71(puStack_1b8._1_7_,1);
  puStack_1b0 = (uint *)puStack_130;
  puStack_3f0 = puVar53;
  puStack_1a0 = _Dst_02;
  puStack_128 = _Dst_02;
  if (lStack_418 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_1408d3a8d:
  uStack_2b8 = puStack_138;
  uStack_118 = 1;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3e28);
  bVar71 = SUB81(extraout_RAX_26,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  puStack_130 = (ulonglong *)0x0;
  puVar53 = extraout_RAX_26;
  while( true ) {
    uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3e30) & (ulonglong)puVar53;
    pcVar2 = (char *)(lVar39 + uVar48);
    cVar69 = SUB161(auVar62,0);
    cVar64 = SUB161(auVar62,1);
    cVar65 = SUB161(auVar62,2);
    cVar66 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar66) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar65) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar64)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar24 = (uint)uVar54;
    while (uVar54 != 0) {
      uVar25 = 0;
      for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar25 = uVar25 + 1;
      }
      uVar51 = uVar25 + uVar48 & *(ulonglong *)((longlong)plStack_f8 + 0x3e30);
      if (*(uint **)(lVar39 + -0x20 + uVar51 * -0x20) == puStack_138) {
        puStack_3a8 = (uint *)(lVar39 + uVar51 * -0x20);
        puStack_3a0 = puStack_1d0;
        uStack_3b0 = (uint *)0x0;
        goto LAB_1408d3bee;
      }
      uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
      uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
    }
    cVar69 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') break;
    puVar53 = (uint *)((longlong)puStack_130 + uVar48 + 0x10);
    puStack_130 = puStack_130 + 2;
  }
  if (*(longlong *)((longlong)plStack_f8 + 0x3e38) == 0) {
    uStack_118 = 1;
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
    puStack_138 = uStack_2b8;
  }
  uStack_3b0 = puStack_1d0;
  puStack_3a8 = puStack_138;
  puStack_3a0 = extraout_RAX_26;
LAB_1408d3bee:
  _auStack_368 = CONCAT88(puStack_1a0,puStack_1a8);
  uStack_378 = puStack_1b8;
  uStack_370 = puStack_1b0;
  uStack_2b0._0_4_ = 0;
  uStack_2b8 = (uint *)0x0;
  puStack_2a0 = (uint *)lVar42;
  uStack_118 = 0;
  fStack_2a8 = fVar61;
  fStack_2a4 = fVar21;
  upsert_collider_entry<bevy_ecs::entity::Entity,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
            ();
  uVar48 = uStack_168;
  iVar45 = (int)puStack_158;
  uVar24 = (uint)puStack_180;
  goto LAB_1408d14b2;
code_r0x0001408d2eb6:
  uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
  uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
  if (uVar54 == 0) goto LAB_1408d2ed1;
  goto LAB_1408d2e90;
LAB_1408d3550:
  if (*(longlong *)((longlong)plStack_f8 + 0x3e38) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
  goto LAB_1408d2fd0;
code_r0x0001408d33e8:
  uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
  uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
  uVar51 = uStack_168;
  if (uVar54 == 0) goto LAB_1408d3403;
  goto LAB_1408d33c2;
LAB_1408d344e:
  uVar48 = uVar51;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e38) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
  goto LAB_1408d352e;
LAB_1408d1974:
  uVar51 = uVar48 & uVar51;
  pcVar2 = (char *)(lVar39 + uVar51);
  cVar69 = SUB161(auVar62,0);
  cVar64 = SUB161(auVar62,1);
  cVar65 = SUB161(auVar62,2);
  cVar66 = SUB161(auVar62,3);
  bVar71 = (pcVar2[0xf] == cVar66) * -1;
  auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                       CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                 CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                           CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                     CONCAT110((pcVar2[10] == cVar65
                                                                               ) * -1,CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
  uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
  uVar24 = (uint)uVar54;
  if (uVar54 != 0) {
LAB_1408d1991:
    uVar25 = 0;
    for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
      uVar25 = uVar25 + 1;
    }
    uVar49 = uVar25 + uVar51 & uVar48;
    puStack_130 = puVar59;
    if (*(uint **)(lVar39 + -0x20 + uVar49 * -0x20) != puStack_138) goto code_r0x0001408d19bb;
    lVar55 = (longlong)(uVar49 * 0x20) >> 5;
    uVar48 = lVar55 + 0xfffffffffffffff0U & uVar48;
    pcVar2 = (char *)(lVar39 + uVar48);
    bVar71 = (pcVar2[0xf] == -1) * -1;
    auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    if (uVar54 == 0) {
      uVar37 = 0x10;
    }
    else {
      uVar37 = 0xf;
      if (uVar54 != 0) {
        for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
        }
      }
      uVar37 = uVar37 ^ 0xf;
    }
    pcVar2 = (char *)(lVar39 + lVar55);
    bVar71 = (pcVar2[0xf] == -1) * -1;
    auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    sVar8 = 0;
    for (uVar24 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                          (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar24 & 1) == 0;
        uVar24 = uVar24 >> 1 | 0x80000000) {
      sVar8 = sVar8 + 1;
    }
    uVar57 = 0x80;
    if ((ushort)(uVar37 + sVar8) < 0x10) {
      *(longlong *)((longlong)plStack_f8 + 0x3dd8) =
           *(longlong *)((longlong)plStack_f8 + 0x3dd8) + 1;
      uVar57 = 0xff;
    }
    lVar29 = uVar49 * -0x20 + lVar39;
    *(undefined1 *)(lVar39 + lVar55) = uVar57;
    *(undefined1 *)(lVar39 + 0x10 + uVar48) = uVar57;
    *(longlong *)((longlong)plStack_f8 + 0x3de0) = *(longlong *)((longlong)plStack_f8 + 0x3de0) + -1
    ;
    _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar29 + -8));
    uStack_370 = *(uint **)(lVar29 + -0x10);
    dispose_collider_mesh_instance();
    if ((char)puStack_298 != '\x11') {
      LOCK();
      *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
      UNLOCK();
      if (*(longlong *)puStack_280 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
      }
    }
    goto LAB_1408d1dd9;
  }
LAB_1408d19d6:
  cVar69 = (pcVar2[0xf] == -1) * -1;
  auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d1c1b;
  uVar51 = (longlong)puVar59 + uVar51 + 0x10;
  puVar59 = puVar59 + 2;
  goto LAB_1408d1974;
code_r0x0001408d19bb:
  uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
  uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
  if (uVar54 == 0) goto LAB_1408d19d6;
  goto LAB_1408d1991;
LAB_1408d1c1b:
  if (*(longlong *)((longlong)plStack_f8 + 0x3dd8) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d1dd9:
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  uVar49 = uStack_168;
  lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3e28);
  uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3e30);
  bVar71 = SUB81(extraout_RAX_15,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  puVar59 = (ulonglong *)0x0;
  uVar51 = extraout_RAX_15;
  do {
    uVar51 = uVar48 & uVar51;
    pcVar2 = (char *)(lVar39 + uVar51);
    cVar69 = SUB161(auVar62,0);
    cVar64 = SUB161(auVar62,1);
    cVar65 = SUB161(auVar62,2);
    cVar66 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar66) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar65) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar64)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar24 = (uint)uVar54;
    if (uVar54 != 0) {
LAB_1408d1e4e:
      uVar25 = 0;
      for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar25 = uVar25 + 1;
      }
      uVar35 = uVar25 + uVar51 & uVar48;
      puStack_130 = puVar59;
      if (*(uint **)(lVar39 + -0x20 + uVar35 * -0x20) != puStack_138) break;
      lVar55 = (longlong)(uVar35 * 0x20) >> 5;
      uVar48 = lVar55 + 0xfffffffffffffff0U & uVar48;
      pcVar2 = (char *)(lVar39 + uVar48);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      if (uVar54 == 0) {
        uVar37 = 0x10;
      }
      else {
        uVar37 = 0xf;
        if (uVar54 != 0) {
          for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
          }
        }
        uVar37 = uVar37 ^ 0xf;
      }
      pcVar2 = (char *)(lVar39 + lVar55);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      sVar8 = 0;
      for (uVar24 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                            (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar24 & 1) == 0;
          uVar24 = uVar24 >> 1 | 0x80000000) {
        sVar8 = sVar8 + 1;
      }
      uVar57 = 0x80;
      if ((ushort)(uVar37 + sVar8) < 0x10) {
        *(longlong *)((longlong)plStack_f8 + 0x3e38) =
             *(longlong *)((longlong)plStack_f8 + 0x3e38) + 1;
        uVar57 = 0xff;
      }
      lVar29 = uVar35 * -0x20 + lVar39;
      *(undefined1 *)(lVar39 + lVar55) = uVar57;
      *(undefined1 *)(lVar39 + 0x10 + uVar48) = uVar57;
      *(longlong *)((longlong)plStack_f8 + 0x3e40) =
           *(longlong *)((longlong)plStack_f8 + 0x3e40) + -1;
      _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar29 + -8));
      uStack_370 = *(uint **)(lVar29 + -0x10);
      dispose_collider_mesh_instance();
      if ((char)puStack_298 != '\x11') {
        LOCK();
        *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
        UNLOCK();
        if (*(longlong *)puStack_280 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      goto LAB_1408d1fba;
    }
LAB_1408d1e8f:
    cVar69 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d1eda;
    uVar51 = (longlong)puVar59 + uVar51 + 0x10;
    puVar59 = puVar59 + 2;
  } while( true );
  uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
  uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
  uVar35 = uStack_168;
  if (uVar54 == 0) goto LAB_1408d1e8f;
  goto LAB_1408d1e4e;
LAB_1408d1eda:
  uVar49 = uVar35;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e38) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d1fba:
  uStack_378 = puStack_138;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  uVar48 = uStack_168;
  lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3e58);
  uVar51 = *(ulonglong *)((longlong)plStack_f8 + 0x3e60);
  bVar71 = SUB81(extraout_RAX_16,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  puVar59 = (ulonglong *)0x0;
  uVar35 = extraout_RAX_16;
  do {
    uVar35 = uVar51 & uVar35;
    pcVar2 = (char *)(lVar39 + uVar35);
    cVar69 = SUB161(auVar62,0);
    cVar64 = SUB161(auVar62,1);
    cVar65 = SUB161(auVar62,2);
    cVar66 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar66) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar65) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar64)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar24 = (uint)uVar54;
    if (uVar54 != 0) {
LAB_1408d202f:
      uVar25 = 0;
      for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar25 = uVar25 + 1;
      }
      uVar49 = uVar25 + uVar35 & uVar51;
      puStack_130 = puVar59;
      if (*(uint **)(lVar39 + -0x20 + uVar49 * -0x20) != puStack_138) break;
      lVar55 = (longlong)(uVar49 * 0x20) >> 5;
      uVar51 = lVar55 + 0xfffffffffffffff0U & uVar51;
      pcVar2 = (char *)(lVar39 + uVar51);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      if (uVar54 == 0) {
        uVar37 = 0x10;
      }
      else {
        uVar37 = 0xf;
        if (uVar54 != 0) {
          for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
          }
        }
        uVar37 = uVar37 ^ 0xf;
      }
      pcVar2 = (char *)(lVar39 + lVar55);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      sVar8 = 0;
      for (uVar24 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                            (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar24 & 1) == 0;
          uVar24 = uVar24 >> 1 | 0x80000000) {
        sVar8 = sVar8 + 1;
      }
      uVar57 = 0x80;
      if ((ushort)(uVar37 + sVar8) < 0x10) {
        *(longlong *)((longlong)plStack_f8 + 0x3e68) =
             *(longlong *)((longlong)plStack_f8 + 0x3e68) + 1;
        uVar57 = 0xff;
      }
      lVar29 = uVar49 * -0x20 + lVar39;
      *(undefined1 *)(lVar39 + lVar55) = uVar57;
      *(undefined1 *)(lVar39 + 0x10 + uVar51) = uVar57;
      *(longlong *)((longlong)plStack_f8 + 0x3e70) =
           *(longlong *)((longlong)plStack_f8 + 0x3e70) + -1;
      _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar29 + -8));
      uStack_378 = *(uint **)(lVar29 + -0x18);
      uStack_370 = *(uint **)(lVar29 + -0x10);
      dispose_collider_mesh_instance();
      if ((char)puStack_298 != '\x11') {
        LOCK();
        *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
        UNLOCK();
        if (*(longlong *)puStack_280 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      goto LAB_1408d352e;
    }
LAB_1408d2070:
    cVar69 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d20bb;
    uVar35 = (longlong)puVar59 + uVar35 + 0x10;
    puVar59 = puVar59 + 2;
  } while( true );
  uVar54 = (ushort)(uVar24 + 0xffffffff) & (ushort)uVar24;
  uVar24 = CONCAT22(SUB42(uVar24 + 0xffffffff,2),uVar54);
  uVar49 = uStack_168;
  if (uVar54 == 0) goto LAB_1408d2070;
  goto LAB_1408d202f;
LAB_1408d20bb:
  uVar48 = uVar49;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e68) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d352e:
  iVar45 = (int)puStack_158;
  uVar24 = (uint)puStack_180;
  goto LAB_1408d14b2;
LAB_1408d4036:
  puVar58 = puVar59;
  if (puVar43 != (ulonglong *)0x0) goto LAB_1408d4148;
LAB_1408d4130:
  do {
    puVar59 = (ulonglong *)0x0;
    puVar47 = (uint *)uVar48;
    puVar34 = (uint *)uVar38;
    puVar43 = puVar58;
    if (puVar58 == (ulonglong *)0x0) {
      puStack_148 = *(uint **)roof_deleted;
      lVar44 = *(longlong *)((longlong)roof_deleted + 8);
      puVar34 = *(uint **)((longlong)puStack_148 + 0xf8);
      puVar47 = puVar34 + *(longlong *)((longlong)puStack_148 + 0x100);
      puStack_1d8 = (uint *)((longlong)plStack_f8 + 0x3df8);
      puStack_110 = (uint *)CONCAT44(puStack_110._4_4_,*(int *)((longlong)roof_deleted + 0x14));
      uVar24 = *(int *)((longlong)roof_deleted + 0x14) +
               *(int *)((longlong)roof_deleted + 0x10) * -1;
      uVar23 = 0xc233b000;
      if (uVar24 < 0xc233b000) {
        uVar23 = uVar24;
      }
      puStack_108 = (uint *)CONCAT44(puStack_108._4_4_,uVar23);
      puStack_158 = (uint *)((longlong)plStack_f8 + 0x3e18);
      puStack_140 = (uint *)0x8;
      lVar31 = 0;
      lVar28 = 8;
      uVar38 = 0;
      uVar48 = 0;
      lStack_160 = CONCAT71(lStack_160._1_7_,*(char *)((longlong)puStack_148 + 0x108));
      cVar70 = *(char *)((longlong)puStack_148 + 0x108);
      goto joined_r0x0001408d4644;
    }
LAB_1408d4148:
    puVar58 = puVar59;
    if ((puVar34 == (uint *)0x0) || (uVar27 = (longlong)puVar34 + 0x10, puVar34 == puStack_148)) {
      if ((puVar47 == (uint *)0x0) || (puVar47 == puStack_158)) goto LAB_1408d4130;
      uVar27 = 0;
      puVar34 = puVar47;
      puVar47 = (uint *)((longlong)puVar47 + 0x10);
    }
    *puVar43 = *puVar43 + 1;
    puVar53 = *(uint **)((longlong)puVar34 + 8);
  } while ((ulonglong)puVar53 >> 0x20 == 0);
  puStack_110 = (uint *)uVar27;
  puStack_108 = puVar47;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar44 = *(longlong *)((longlong)plStack_f8 + 0x3dc8);
  uVar27 = *(ulonglong *)((longlong)plStack_f8 + 0x3dd0);
  bVar71 = SUB81(extraout_RAX_27,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  lVar28 = 0;
  uVar51 = extraout_RAX_27;
  do {
    uVar51 = uVar27 & uVar51;
    pcVar2 = (char *)(lVar44 + uVar51);
    cVar70 = SUB161(auVar62,0);
    cVar69 = SUB161(auVar62,1);
    cVar64 = SUB161(auVar62,2);
    cVar65 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar65) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar64) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar69) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar70) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar65) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar64) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar69)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar70) * -1,
                                                           CONCAT17((pcVar2[7] == cVar65) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar64) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar69)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar70) * -1,
                                                           CONCAT13((pcVar2[3] == cVar65) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar64) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar69)
                                                                                         * -1,(*
                                                  pcVar2 == cVar70) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar23 = (uint)uVar54;
    if (uVar54 != 0) {
LAB_1408d420d:
      uVar24 = 0;
      for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
        uVar24 = uVar24 + 1;
      }
      uVar35 = uVar24 + uVar51 & uVar27;
      if (*(uint **)(lVar44 + -0x20 + uVar35 * -0x20) != puVar53) goto code_r0x0001408d4229;
      lVar28 = (longlong)(uVar35 * 0x20) >> 5;
      uVar27 = lVar28 + 0xfffffffffffffff0U & uVar27;
      pcVar2 = (char *)(lVar44 + uVar27);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      if (uVar54 == 0) {
        uVar37 = 0x10;
      }
      else {
        uVar37 = 0xf;
        if (uVar54 != 0) {
          for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
          }
        }
        uVar37 = uVar37 ^ 0xf;
      }
      pcVar2 = (char *)(lVar44 + lVar28);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      sVar8 = 0;
      for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                            (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
          uVar23 = uVar23 >> 1 | 0x80000000) {
        sVar8 = sVar8 + 1;
      }
      uVar57 = 0x80;
      if ((ushort)(uVar37 + sVar8) < 0x10) {
        *(longlong *)((longlong)plStack_f8 + 0x3dd8) =
             *(longlong *)((longlong)plStack_f8 + 0x3dd8) + 1;
        uVar57 = 0xff;
      }
      lVar31 = uVar35 * -0x20 + lVar44;
      *(undefined1 *)(lVar44 + lVar28) = uVar57;
      *(undefined1 *)(lVar44 + 0x10 + uVar27) = uVar57;
      *(longlong *)((longlong)plStack_f8 + 0x3de0) =
           *(longlong *)((longlong)plStack_f8 + 0x3de0) + -1;
      _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar31 + -8));
      uStack_370 = *(uint **)(lVar31 + -0x10);
      dispose_collider_mesh_instance();
      if ((char)puStack_298 != '\x11') {
        LOCK();
        *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
        UNLOCK();
        if (*(longlong *)puStack_280 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      goto LAB_1408d4380;
    }
LAB_1408d4240:
    cVar70 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar70,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar70 < '\0') goto LAB_1408d4290;
    uVar51 = uVar51 + lVar28 + 0x10;
    lVar28 = lVar28 + 0x10;
  } while( true );
joined_r0x0001408d4644:
  if (cVar70 == '\0') {
    if (uVar48 != uVar38) goto LAB_1408d4703;
LAB_1408d4659:
    if (puVar34 == puVar47) goto LAB_1408d4a2b;
    uVar23 = *puVar34;
    puVar34 = puVar34 + 1;
    if (*(longlong *)(*(longlong *)(lVar44 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0) == 0) {
      do {
        if (puVar34 == puVar47) goto LAB_1408d4a2b;
        uVar23 = *puVar34;
        puVar34 = puVar34 + 1;
      } while (*(longlong *)(*(longlong *)(lVar44 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0) == 0);
      lVar31 = *(longlong *)(lVar44 + 0x100) + (ulonglong)uVar23 * 0xa0;
    }
    else {
      lVar31 = *(longlong *)(lVar44 + 0x100) + (ulonglong)uVar23 * 0xa0;
    }
    bevy_ecs::storage::table::Table::get_added_ticks_slice_for();
    lVar28 = *(longlong *)(lVar31 + 0x50);
    uVar38 = *(ulonglong *)(lVar31 + 0x58);
    uVar48 = 0;
    lVar31 = extraout_RAX_30;
LAB_1408d4703:
    if ((uint)puStack_108 <=
        (uint)((int)puStack_110 +
              *(int *)(lVar31 + (ulonglong)*(uint *)(lVar28 + 8 + uVar48 * 0x10) * 4) * -1))
    goto code_r0x0001408d4724;
    puVar26 = (undefined8 *)(uVar48 * 0x10 + lVar28);
LAB_1408d4804:
    puVar53 = (uint *)*puVar26;
    if ((ulonglong)puVar53 >> 0x20 == 0) goto LAB_1408d4a2b;
    puStack_188 = puVar53;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    uVar48 = uVar48 + 1;
    lVar39 = *(longlong *)((longlong)plStack_f8 + 0x3df8);
    uVar27 = *(ulonglong *)((longlong)plStack_f8 + 0x3e00);
    bVar71 = SUB81(extraout_RAX_32,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    lVar55 = 0;
    uVar51 = extraout_RAX_32;
    do {
      uVar51 = uVar27 & uVar51;
      pcVar2 = (char *)(lVar39 + uVar51);
      cVar70 = SUB161(auVar62,0);
      cVar69 = SUB161(auVar62,1);
      cVar64 = SUB161(auVar62,2);
      cVar65 = SUB161(auVar62,3);
      bVar71 = (pcVar2[0xf] == cVar65) * -1;
      auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar64) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar69) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar70) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar65) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar64) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar69) * -1,
                                                  CONCAT18((pcVar2[8] == cVar70) * -1,
                                                           CONCAT17((pcVar2[7] == cVar65) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar64) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar69)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar70) * -1,
                                                           CONCAT13((pcVar2[3] == cVar65) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar64) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar69)
                                                                                         * -1,(*
                                                  pcVar2 == cVar70) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar23 = (uint)uVar54;
      if (uVar54 != 0) {
        while( true ) {
          uVar24 = 0;
          for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
            uVar24 = uVar24 + 1;
          }
          uVar35 = uVar24 + uVar51 & uVar27;
          lStack_210 = lVar55;
          puStack_180 = (ulonglong *)lVar39;
          if (*(uint **)(lVar39 + -0x20 + uVar35 * -0x20) == puStack_188) break;
          uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
          uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
          if (uVar54 == 0) goto LAB_1408d48ec;
        }
        lVar55 = (longlong)(uVar35 * 0x20) >> 5;
        uVar27 = lVar55 + 0xfffffffffffffff0U & uVar27;
        pcVar2 = (char *)(lVar39 + uVar27);
        bVar71 = (pcVar2[0xf] == -1) * -1;
        auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                             CONCAT113((pcVar2[0xd] == -1) * -1,
                                                       CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                 CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                           CONCAT110((pcVar2[10] ==
                                                                                     -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
        uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
        if (uVar54 == 0) {
          uVar37 = 0x10;
        }
        else {
          uVar37 = 0xf;
          if (uVar54 != 0) {
            for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
            }
          }
          uVar37 = uVar37 ^ 0xf;
        }
        pcVar2 = (char *)(lVar39 + lVar55);
        bVar71 = (pcVar2[0xf] == -1) * -1;
        auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                             CONCAT113((pcVar2[0xd] == -1) * -1,
                                                       CONCAT112((pcVar2[0xc] == -1) * -1,
                                                                 CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                           CONCAT110((pcVar2[10] ==
                                                                                     -1) * -1,
                                                  CONCAT19((pcVar2[9] == -1) * -1,
                                                           CONCAT18((pcVar2[8] == -1) * -1,
                                                                    CONCAT17((pcVar2[7] == -1) * -1,
                                                                             CONCAT16((pcVar2[6] ==
                                                                                      -1) * -1,
                                                  CONCAT15((pcVar2[5] == -1) * -1,
                                                           CONCAT14((pcVar2[4] == -1) * -1,
                                                                    CONCAT13((pcVar2[3] == -1) * -1,
                                                                             CONCAT12((pcVar2[2] ==
                                                                                      -1) * -1,
                                                  CONCAT11((pcVar2[1] == -1) * -1,
                                                           (*pcVar2 == -1) * -1)))))))))))))));
        sVar8 = 0;
        for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                              (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
            uVar23 = uVar23 >> 1 | 0x80000000) {
          sVar8 = sVar8 + 1;
        }
        uVar57 = 0x80;
        if ((ushort)(uVar37 + sVar8) < 0x10) {
          *(longlong *)((longlong)plStack_f8 + 0x3e08) =
               *(longlong *)((longlong)plStack_f8 + 0x3e08) + 1;
          uVar57 = 0xff;
        }
        lVar29 = uVar35 * -0x20 + lVar39;
        *(undefined1 *)(lVar39 + lVar55) = uVar57;
        *(undefined1 *)(lVar39 + 0x10 + uVar27) = uVar57;
        *(longlong *)((longlong)plStack_f8 + 0x3e10) =
             *(longlong *)((longlong)plStack_f8 + 0x3e10) + -1;
        _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar29 + -8));
        uStack_378 = *(uint **)(lVar29 + -0x18);
        uStack_370 = *(uint **)(lVar29 + -0x10);
        dispose_collider_mesh_instance();
        if ((char)puStack_298 != '\x11') {
          LOCK();
          *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
          UNLOCK();
          if (*(longlong *)puStack_280 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
        goto LAB_1408d4a15;
      }
LAB_1408d48ec:
      cVar70 = (pcVar2[0xf] == -1) * -1;
      auVar63 = CONCAT115(cVar70,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar70 < '\0') goto LAB_1408d4938;
      uVar51 = uVar51 + lVar55 + 0x10;
      lVar55 = lVar55 + 0x10;
    } while( true );
  }
  if (uVar48 != uVar38) goto LAB_1408d47d5;
  while (puVar34 != puVar47) {
    uVar38 = (ulonglong)*puVar34;
    puVar34 = puVar34 + 1;
    lVar31 = *(longlong *)(lVar44 + 0x1a8);
    if (*(longlong *)(lVar31 + 0x10 + uVar38 * 0x48) == 0) {
      do {
        if (puVar34 == puVar47) goto LAB_1408d4a2b;
        uVar23 = *puVar34;
        puVar34 = puVar34 + 1;
      } while (*(longlong *)(*(longlong *)(lVar44 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48) == 0);
      lVar39 = *(longlong *)(lVar44 + 0x1a8) + (ulonglong)uVar23 * 0x48;
      lVar31 = lVar39;
    }
    else {
      lVar39 = lVar31 + uVar38 * 0x48;
      lVar31 = lVar31 + uVar38 * 0x48;
    }
    bevy_ecs::storage::table::Table::get_added_ticks_slice_for();
    puStack_140 = *(uint **)(lVar39 + 8);
    uVar38 = *(ulonglong *)(lVar31 + 0x10);
    uVar48 = 0;
    lVar31 = extraout_RAX_31;
LAB_1408d47d5:
    do {
      if ((uint)((int)puStack_110 + *(int *)(lVar31 + (uVar48 & 0xffffffff) * 4) * -1) <
          (uint)puStack_108) {
        puVar26 = (undefined8 *)((longlong)puStack_140 + uVar48 * 8);
        goto LAB_1408d4804;
      }
      uVar48 = uVar48 + 1;
    } while (uVar48 != uVar38);
  }
LAB_1408d4a2b:
  fVar13 = __real_3f800000;
  puVar53 = *(uint **)roof_changed;
  lVar44 = *(longlong *)((longlong)roof_changed + 8);
  puVar47 = *(uint **)(puVar53 + 0x3e);
  puVar34 = puVar47 + *(longlong *)(puVar53 + 0x40);
  cVar70 = *(char *)(puVar53 + 0x42);
  puStack_108 = (uint *)CONCAT44(puStack_108._4_4_,*(int *)((longlong)roof_changed + 0x14));
  uVar24 = *(int *)((longlong)roof_changed + 0x14) + *(int *)((longlong)roof_changed + 0x10) * -1;
  uVar23 = 0xc233b000;
  if (uVar24 < 0xc233b000) {
    uVar23 = uVar24;
  }
  puStack_148 = (uint *)CONCAT44(puStack_148._4_4_,uVar23);
  lStack_210 = 8;
  puStack_188 = (uint *)0x0;
  puStack_180 = (ulonglong *)0x0;
  lStack_3b8 = 8;
  puVar40 = (uint *)0x0;
  puVar41 = (uint *)0x0;
  puStack_140 = (uint *)0x0;
  puStack_110 = (uint *)((ulonglong)puStack_110 & 0xffffffff00000000);
  lStack_160 = 0;
  uVar38 = 0;
LAB_1408d4adb:
  puVar60 = uStack_3b0;
  lVar28 = lStack_210;
  if (cVar70 == '\0') {
    do {
      if (puVar41 == puVar40) {
        if (puVar47 == puVar34) goto LAB_1408d5812;
        uVar23 = *puVar47;
        puVar47 = puVar47 + 1;
        if (*(longlong *)(*(longlong *)(lVar44 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0) == 0) {
          do {
            if (puVar47 == puVar34) goto LAB_1408d5812;
            uVar23 = *puVar47;
            puVar47 = puVar47 + 1;
          } while (*(longlong *)(*(longlong *)(lVar44 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0) ==
                   0);
          lVar28 = *(longlong *)(lVar44 + 0x100) + (ulonglong)uVar23 * 0xa0;
        }
        else {
          lVar28 = *(longlong *)(lVar44 + 0x100) + (ulonglong)uVar23 * 0xa0;
        }
        lVar31 = *(longlong *)(lVar44 + 0x1a8) + (ulonglong)*(uint *)(lVar28 + 0x94) * 0x48;
        uVar38 = *(ulonglong *)
                  (*(longlong *)(lVar44 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar28 + 0x94) * 0x48
                  );
        if ((*(ulonglong *)(puVar53 + 0x44) < uVar38) &&
           (uVar48 = *(ulonglong *)
                      (*(longlong *)(lVar31 + 0x38) + *(ulonglong *)(puVar53 + 0x44) * 8),
           uVar48 != 0)) {
          puStack_188 = *(uint **)(*(longlong *)(lVar31 + 0x18) + 0x10 + ~uVar48 * 0x30);
          uVar48 = *(ulonglong *)(puVar53 + 0x46);
          if (uVar38 <= uVar48) goto LAB_1408d4e70;
LAB_1408d4e27:
          uVar38 = *(ulonglong *)(*(longlong *)(lVar31 + 0x38) + uVar48 * 8);
          if (uVar38 == 0) goto LAB_1408d4e70;
          puStack_180 = *(ulonglong **)(*(longlong *)(lVar31 + 0x18) + 0x10 + ~uVar38 * 0x30);
          uVar38 = *(ulonglong *)(puVar53 + 0x48);
          uVar48 = *(ulonglong *)(lVar28 + 0x88);
          if (uVar38 < uVar48) goto LAB_1408d4e8e;
LAB_1408d4ed0:
          puStack_110 = (uint *)((ulonglong)puStack_110 & 0xffffffff00000000);
          uVar38 = *(ulonglong *)(puVar53 + 0x4a);
          if (uVar48 <= uVar38) goto LAB_1408d4f10;
LAB_1408d4ee6:
          if (*(longlong *)(*(longlong *)(lVar28 + 0x80) + uVar38 * 8) == 0) goto LAB_1408d4f10;
          bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
          uVar23 = (uint)CONCAT71(SUB87(extraout_RAX_36,1),1);
          lStack_160 = extraout_RAX_36;
        }
        else {
          puStack_188 = (uint *)0x0;
          uVar48 = *(ulonglong *)(puVar53 + 0x46);
          if (uVar48 < uVar38) goto LAB_1408d4e27;
LAB_1408d4e70:
          puStack_180 = (ulonglong *)0x0;
          uVar38 = *(ulonglong *)(puVar53 + 0x48);
          uVar48 = *(ulonglong *)(lVar28 + 0x88);
          if (uVar48 <= uVar38) goto LAB_1408d4ed0;
LAB_1408d4e8e:
          if (*(longlong *)(*(longlong *)(lVar28 + 0x80) + uVar38 * 8) == 0) goto LAB_1408d4ed0;
          bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
          puStack_110 = (uint *)CONCAT44(puStack_110._4_4_,(int)CONCAT71(SUB87(lVar31,1),1));
          uVar38 = *(ulonglong *)(puVar53 + 0x4a);
          puStack_140 = (uint *)extraout_RAX_35;
          if (uVar38 < *(ulonglong *)(lVar28 + 0x88)) goto LAB_1408d4ee6;
LAB_1408d4f10:
          uVar23 = 0;
        }
        lStack_3b8 = *(longlong *)(lVar28 + 0x50);
        puVar40 = *(uint **)(lVar28 + 0x58);
        puVar41 = (uint *)0x0;
        uVar38 = (ulonglong)uVar23;
      }
      uVar23 = *(uint *)(lStack_3b8 + 8 + (longlong)puVar41 * 0x10);
      if (((((ulonglong)puStack_110 & 1) != 0) &&
          ((uint)puStack_108 + *(int *)((longlong)puStack_140 + (ulonglong)uVar23 * 4) * -1 <
           (uint)puStack_148)) ||
         (((uVar38 & 1) != 0 &&
          ((uint)puStack_108 + *(int *)(lStack_160 + (ulonglong)uVar23 * 4) * -1 < (uint)puStack_148
          )))) goto LAB_1408d4f80;
      puVar41 = (uint *)((longlong)puVar41 + 1);
    } while( true );
  }
  do {
    if (puVar41 == puVar40) {
      if (puVar47 == puVar34) goto LAB_1408d5812;
      uVar38 = (ulonglong)*puVar47;
      puVar47 = puVar47 + 1;
      lVar28 = *(longlong *)(lVar44 + 0x1a8);
      if (*(longlong *)(lVar28 + 0x10 + uVar38 * 0x48) == 0) {
        do {
          if (puVar47 == puVar34) goto LAB_1408d5812;
          uVar23 = *puVar47;
          puVar47 = puVar47 + 1;
        } while (*(longlong *)(*(longlong *)(lVar44 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48) == 0
                );
        lVar31 = *(longlong *)(lVar44 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        puVar26 = (undefined8 *)(lVar31 + 0x10);
        uVar38 = *(ulonglong *)(puVar53 + 0x44);
        uVar48 = *(ulonglong *)(lVar31 + 0x40);
        if (uVar38 < uVar48) goto LAB_1408d4b8c;
LAB_1408d4bd0:
        puStack_188 = (uint *)0x0;
        uVar38 = *(ulonglong *)(puVar53 + 0x46);
        if (uVar48 <= uVar38) goto LAB_1408d4c20;
LAB_1408d4be7:
        uVar38 = *(ulonglong *)(*(longlong *)(lVar31 + 0x38) + uVar38 * 8);
        if (uVar38 == 0) goto LAB_1408d4c20;
        puStack_180 = *(ulonglong **)(*(longlong *)(lVar31 + 0x18) + 0x10 + ~uVar38 * 0x30);
        uVar38 = *(ulonglong *)(puVar53 + 0x48);
        if (uVar38 < uVar48) goto LAB_1408d4c37;
LAB_1408d4c70:
        puStack_110 = (uint *)((ulonglong)puStack_110 & 0xffffffff00000000);
        uVar38 = *(ulonglong *)(puVar53 + 0x4a);
        if (uVar48 <= uVar38) goto LAB_1408d4cb0;
LAB_1408d4c86:
        if (*(longlong *)(*(longlong *)(lVar31 + 0x38) + uVar38 * 8) == 0) goto LAB_1408d4cb0;
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        uVar23 = (uint)CONCAT71(SUB87(extraout_RAX_34,1),1);
        lStack_160 = extraout_RAX_34;
      }
      else {
        lVar31 = lVar28 + uVar38 * 0x48;
        puVar26 = (undefined8 *)(lVar28 + uVar38 * 0x48 + 0x10);
        uVar38 = *(ulonglong *)(puVar53 + 0x44);
        uVar48 = *(ulonglong *)(lVar31 + 0x40);
        if (uVar48 <= uVar38) goto LAB_1408d4bd0;
LAB_1408d4b8c:
        uVar38 = *(ulonglong *)(*(longlong *)(lVar31 + 0x38) + uVar38 * 8);
        if (uVar38 == 0) goto LAB_1408d4bd0;
        puStack_188 = *(uint **)(*(longlong *)(lVar31 + 0x18) + 0x10 + ~uVar38 * 0x30);
        uVar38 = *(ulonglong *)(puVar53 + 0x46);
        if (uVar38 < uVar48) goto LAB_1408d4be7;
LAB_1408d4c20:
        puStack_180 = (ulonglong *)0x0;
        uVar38 = *(ulonglong *)(puVar53 + 0x48);
        if (uVar48 <= uVar38) goto LAB_1408d4c70;
LAB_1408d4c37:
        if (*(longlong *)(*(longlong *)(lVar31 + 0x38) + uVar38 * 8) == 0) goto LAB_1408d4c70;
        lVar28 = lVar31;
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        puStack_110 = (uint *)CONCAT44(puStack_110._4_4_,(int)CONCAT71(SUB87(lVar28,1),1));
        uVar38 = *(ulonglong *)(puVar53 + 0x4a);
        puStack_140 = (uint *)extraout_RAX_33;
        if (uVar38 < *(ulonglong *)(lVar31 + 0x40)) goto LAB_1408d4c86;
LAB_1408d4cb0:
        uVar23 = 0;
      }
      lVar28 = *(longlong *)(lVar31 + 8);
      puVar40 = (uint *)*puVar26;
      puVar41 = (uint *)0x0;
      uVar38 = (ulonglong)uVar23;
    }
    if (((((ulonglong)puStack_110 & 1) != 0) &&
        ((uint)puStack_108 +
         *(int *)((longlong)puStack_140 + ((ulonglong)puVar41 & 0xffffffff) * 4) * -1 <
         (uint)puStack_148)) ||
       (((uVar38 & 1) != 0 &&
        ((uint)puStack_108 + *(int *)(lStack_160 + ((ulonglong)puVar41 & 0xffffffff) * 4) * -1 <
         (uint)puStack_148)))) goto LAB_1408d4d08;
    puVar41 = (uint *)((longlong)puVar41 + 1);
  } while( true );
LAB_1408d4938:
  uStack_378 = puVar53;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e08) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d4a15:
  cVar70 = (char)lStack_160;
  goto joined_r0x0001408d4644;
code_r0x0001408d4724:
  uVar48 = uVar48 + 1;
  if (uVar48 == uVar38) goto LAB_1408d4659;
  goto LAB_1408d4703;
LAB_1408d4f80:
  uVar48 = (ulonglong)uVar23;
  puStack_178 = (uint *)CONCAT44(puStack_178._4_4_,(int)uVar38);
  puVar26 = (undefined8 *)((longlong)puVar41 * 0x10 + lStack_3b8);
  goto LAB_1408d4f90;
LAB_1408d5812:
  lVar42 = *(longlong *)((longlong)roof_removed + 0x10);
  if ((**(ulonglong **)roof_removed < *(ulonglong *)(lVar42 + 0x40)) &&
     (uVar38 = *(ulonglong *)(*(longlong *)(lVar42 + 0x38) + **(ulonglong **)roof_removed * 8),
     uVar38 != 0)) {
    lVar42 = *(longlong *)(lVar42 + 8);
    uVar38 = ~uVar38;
    puVar59 = *(ulonglong **)((longlong)roof_removed + 8);
    uVar48 = *puVar59;
    uVar51 = 0;
    puVar43 = (ulonglong *)(lVar42 + 0x18 + uVar38 * 0x48);
    uVar27 = uVar48 + *puVar43 * -1;
    if (uVar48 < *puVar43) {
      uVar27 = uVar51;
    }
    puVar43 = (ulonglong *)(lVar42 + 0x38 + uVar38 * 0x48);
    uVar35 = uVar48 + *puVar43 * -1;
    if (uVar48 < *puVar43) {
      uVar35 = uVar51;
    }
    puVar53 = (uint *)(uVar27 * 0x10 + *(longlong *)(lVar42 + 8 + uVar38 * 0x48));
    uVar48 = *(ulonglong *)(lVar42 + 0x10 + uVar38 * 0x48);
    uVar49 = *(ulonglong *)(lVar42 + 0x30 + uVar38 * 0x48);
    uVar5 = uVar48 + uVar27 * -1;
    if (uVar48 < uVar27) {
      puVar53 = (uint *)&DAT_00000008;
      uVar5 = uVar51;
    }
    puVar40 = (uint *)(uVar35 * 0x10 + *(longlong *)(lVar42 + 0x28 + uVar38 * 0x48));
    uVar48 = uVar49 + uVar35 * -1;
    if (uVar49 < uVar35) {
      puVar40 = (uint *)&DAT_00000008;
      uVar48 = uVar51;
    }
    *puVar59 = *(longlong *)(lVar42 + 0x40 + uVar38 * 0x48) + uVar48 * -1 + uVar5 * -1;
    puStack_108 = puVar53 + uVar5 * 4;
    puStack_148 = puVar40 + uVar48 * 4;
  }
  else {
    puVar59 = (ulonglong *)0x0;
  }
  puVar43 = (ulonglong *)0x0;
  puStack_140 = puVar40;
LAB_1408d59b6:
  do {
    puVar34 = puVar47;
    if (puVar43 != (ulonglong *)0x0) goto LAB_1408d5ad8;
LAB_1408d5ac0:
    do {
      puVar43 = puVar59;
      puVar59 = (ulonglong *)0x0;
      puVar41 = puVar40;
      puVar34 = puVar53;
      if (puVar43 == (ulonglong *)0x0) {
        if (bVar12) {
          puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
          if (*extraout_RAX_43 == 1) {
            plVar36 = extraout_RAX_43 + 1;
          }
          else if ((*extraout_RAX_43 != 0) ||
                  (std::sys::thread_local::native::lazy::Storage::
                   initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                             (), plVar36 = extraout_RAX_44, extraout_RAX_44 == (longlong *)0x0))
          goto LAB_1408d5c7a;
          if (*plVar36 != 0) goto LAB_1408d5c97;
          *plVar36 = -1;
          plStack_f8 = plVar36;
          puffin::ThreadProfiler::end_scope();
          *plStack_f8 = *plStack_f8 + 1;
        }
        return;
      }
LAB_1408d5ad8:
      if ((puVar34 == (uint *)0x0) || (puVar47 = puVar34 + 4, puVar34 == puStack_108)) {
        if ((puVar41 == (uint *)0x0) || (puVar41 == puStack_148)) goto LAB_1408d5ac0;
        puVar47 = (uint *)0x0;
        puVar34 = puVar41;
        puVar41 = puVar41 + 4;
      }
      *puVar43 = *puVar43 + 1;
      uVar38 = *(ulonglong *)(puVar34 + 2);
    } while (uVar38 >> 0x20 == 0);
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    puVar40 = puStack_140;
    lVar42 = *(longlong *)((longlong)plStack_f8 + 0x3df8);
    uVar48 = *(ulonglong *)((longlong)plStack_f8 + 0x3e00);
    bVar71 = SUB81(extraout_RAX_42,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    lVar44 = 0;
    uVar27 = extraout_RAX_42;
    while( true ) {
      uVar27 = uVar48 & uVar27;
      pcVar2 = (char *)(lVar42 + uVar27);
      cVar70 = SUB161(auVar62,0);
      cVar69 = SUB161(auVar62,1);
      cVar64 = SUB161(auVar62,2);
      cVar65 = SUB161(auVar62,3);
      bVar71 = (pcVar2[0xf] == cVar65) * -1;
      auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar64) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar69) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar70) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar65) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar64) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar69) * -1,
                                                  CONCAT18((pcVar2[8] == cVar70) * -1,
                                                           CONCAT17((pcVar2[7] == cVar65) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar64) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar69)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar70) * -1,
                                                           CONCAT13((pcVar2[3] == cVar65) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar64) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar69)
                                                                                         * -1,(*
                                                  pcVar2 == cVar70) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar23 = (uint)uVar54;
      puStack_110 = puVar41;
      if (uVar54 != 0) break;
LAB_1408d5940:
      cVar70 = (pcVar2[0xf] == -1) * -1;
      auVar63 = CONCAT115(cVar70,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar70 < '\0') goto LAB_1408d5990;
      uVar27 = uVar27 + lVar44 + 0x10;
      lVar44 = lVar44 + 0x10;
    }
    while( true ) {
      uVar24 = 0;
      for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
        uVar24 = uVar24 + 1;
      }
      uVar51 = uVar24 + uVar27 & uVar48;
      if (*(ulonglong *)(lVar42 + -0x20 + uVar51 * -0x20) == uVar38) break;
      uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
      uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
      if (uVar54 == 0) goto LAB_1408d5940;
    }
    lVar44 = (longlong)(uVar51 * 0x20) >> 5;
    uVar48 = lVar44 + 0xfffffffffffffff0U & uVar48;
    pcVar2 = (char *)(lVar42 + uVar48);
    bVar71 = (pcVar2[0xf] == -1) * -1;
    auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    if (uVar54 == 0) {
      uVar37 = 0x10;
    }
    else {
      uVar37 = 0xf;
      if (uVar54 != 0) {
        for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
        }
      }
      uVar37 = uVar37 ^ 0xf;
    }
    pcVar2 = (char *)(lVar42 + lVar44);
    bVar71 = (pcVar2[0xf] == -1) * -1;
    auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    sVar8 = 0;
    for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                          (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
        uVar23 = uVar23 >> 1 | 0x80000000) {
      sVar8 = sVar8 + 1;
    }
    uVar57 = 0x80;
    if ((ushort)(uVar37 + sVar8) < 0x10) {
      *(longlong *)((longlong)plStack_f8 + 0x3e08) =
           *(longlong *)((longlong)plStack_f8 + 0x3e08) + 1;
      uVar57 = 0xff;
    }
    *(undefined1 *)(lVar42 + lVar44) = uVar57;
    *(undefined1 *)(lVar42 + 0x10 + uVar48) = uVar57;
    *(longlong *)((longlong)plStack_f8 + 0x3e10) = *(longlong *)((longlong)plStack_f8 + 0x3e10) + -1
    ;
    dispose_collider_mesh_instance();
    if ((char)puStack_298 != '\x11') {
      LOCK();
      *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
      UNLOCK();
      if (*(longlong *)puStack_280 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
      }
    }
  } while( true );
LAB_1408d5990:
  if (*(longlong *)((longlong)plStack_f8 + 0x3e08) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
  goto LAB_1408d59b6;
LAB_1408d4d08:
  uVar48 = (ulonglong)puVar41 & 0xffffffff;
  puStack_178 = (uint *)CONCAT44(puStack_178._4_4_,(int)uVar38);
  puVar26 = (undefined8 *)(lVar28 + (longlong)puVar41 * 8);
  lStack_210 = lVar28;
LAB_1408d4f90:
  puStack_170 = (uint *)((longlong)puStack_188 + uVar48 * 9 * 8);
  puStack_1c0 = (uint *)*puVar26;
  puVar41 = (uint *)((longlong)puVar41 + 1);
  uVar38 = puStack_180[uVar48 * 0xb];
  uStack_2b8 = (uint *)uVar38;
  if (*(longlong *)((longlong)puStack_208 + 0x18) != 0) {
    puStack_200 = puStack_180 + uVar48 * 0xb;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    puVar22 = puStack_170;
    lVar28 = *(longlong *)puStack_208;
    bVar71 = SUB81(extraout_RAX_37,7) >> 1;
    auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
    auVar62 = pshuflw(auVar62,auVar62,0);
    lVar31 = 0;
    uVar48 = extraout_RAX_37;
    do {
      uVar48 = *(ulonglong *)((longlong)puStack_208 + 8) & uVar48;
      pcVar2 = (char *)(lVar28 + uVar48);
      cVar69 = SUB161(auVar62,0);
      cVar64 = SUB161(auVar62,1);
      cVar65 = SUB161(auVar62,2);
      cVar66 = SUB161(auVar62,3);
      bVar71 = (pcVar2[0xf] == cVar66) * -1;
      auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                           CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                     CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                               CONCAT111((pcVar2[0xb] == cVar66) *
                                                                         -1,CONCAT110((pcVar2[10] ==
                                                                                      cVar65) * -1,
                                                                                      CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      uVar23 = (uint)uVar54;
      while (uVar54 != 0) {
        uVar24 = 0;
        for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
          uVar24 = uVar24 + 1;
        }
        lVar39 = (uVar24 + uVar48 & *(ulonglong *)((longlong)puStack_208 + 8)) * -0x2a8;
        if (*(ulonglong *)(lVar28 + -0x2a8 + lVar39) == uVar38) {
          if ((*(ulonglong *)(lVar28 + -0x290 + lVar39) < 2) ||
             (*(float *)(lVar28 + -0x270 + lVar39) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          if (*(char *)(lVar28 + lVar39 + -9) != '\0') goto LAB_1408d50ca;
          if (*(longlong *)(puStack_170 + 4) == 0) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          fStack_2a8 = *(float *)((longlong)puStack_200 + 0x4c);
          uStack_2b8 = *(uint **)(puStack_170 + 2);
          uStack_2b0 = (longlong)uStack_2b8 + *(longlong *)(puStack_170 + 4) * 0x24;
          alloc::vec::spec_from_iter::impl$0::
          from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl_1::triangles::closure_env_0>_>
                    ();
          puVar6 = *(uint **)(puVar22 + 10);
          puVar32 = SUB168(ZEXT816(puVar6) * ZEXT816(0x24),0);
          if ((SUB168(ZEXT816(puVar6) * ZEXT816(0x24),8) != 0) ||
             ((uint *)0x7ffffffffffffffc < puVar32)) {
LAB_1408d5d72:
            uStack_111 = 1;
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error();
          }
          puStack_1d0 = puVar32;
          puStack_170 = puVar22;
          if (puVar32 == (uint *)0x0) {
            puVar52 = &DAT_00000004;
            puVar32 = (uint *)0x0;
            pvVar33 = *(void **)(puVar22 + 8);
          }
          else {
            pvStack_1c8 = *(void **)(puVar22 + 8);
            std::alloc::__default_lib_allocator::__rust_alloc();
            puVar52 = extraout_RAX_39;
            puVar32 = puVar6;
            pvVar33 = pvStack_1c8;
            if (extraout_RAX_39 == (undefined1 *)0x0) goto LAB_1408d5d72;
          }
          puVar22 = puStack_1d0;
          puStack_1d0 = puVar32;
          memcpy(puVar52,pvVar33,(size_t)puVar22);
          puStack_1b8 = puStack_1d0;
          uVar38 = *(ulonglong *)(puStack_170 + 0x10);
          pvVar33 = SUB168(ZEXT816(uVar38) * ZEXT816(0x18),0);
          puStack_1b0 = (uint *)puVar52;
          puStack_1a8 = puVar6;
          if ((SUB168(ZEXT816(uVar38) * ZEXT816(0x18),8) != 0) ||
             ((void *)0x7ffffffffffffffc < pvVar33)) {
LAB_1408d5d55:
            uStack_149 = 1;
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error();
          }
          _Src = *(void **)(puStack_170 + 0xe);
          if (pvVar33 == (void *)0x0) {
            puVar52 = &DAT_00000004;
            puStack_170 = (uint *)0x0;
            puVar22 = puStack_170;
          }
          else {
            pvStack_1c8 = pvVar33;
            std::alloc::__default_lib_allocator::__rust_alloc();
            puVar52 = extraout_RAX_40;
            pvVar33 = pvStack_1c8;
            puVar22 = (uint *)uVar38;
            if (extraout_RAX_40 == (undefined1 *)0x0) goto LAB_1408d5d55;
          }
          puStack_170 = puVar22;
          memcpy(puVar52,_Src,(size_t)pvVar33);
          uStack_2b8 = puStack_170;
          uStack_2b0._0_4_ = (uint)puVar52;
          uStack_2b0._4_4_ = SUB84(puVar52,4);
          fStack_2a8 = (float)uVar38;
          fStack_2a4 = SUB84(uVar38,4);
          uStack_149 = 0;
          RaycastMeshData::new_unindexed();
          uStack_2b8 = (uint *)0x1;
          uStack_2b0._0_4_ = 1;
          uStack_2b0._4_4_ = 0.0;
          uStack_238 = uStack_308;
          uStack_248 = uStack_318;
          lStack_240 = lStack_310;
          uStack_258 = uStack_328;
          uStack_250 = uStack_320;
          uStack_268 = uStack_338;
          puStack_260 = puStack_330;
          uStack_278 = uStack_348;
          puStack_280 = puStack_350;
          puStack_298 = (uint *)auStack_368;
          uStack_290 = uStack_360;
          fStack_2a8 = SUB84(uStack_378,0);
          fStack_2a4 = SUB84(uStack_378,4);
          puStack_2a0 = uStack_370;
          lStack_288 = lStack_358;
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (_Dst_03 == (ulonglong *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::alloc::handle_alloc_error();
          }
          memcpy(_Dst_03,&uStack_2b8,0x90);
          uStack_370 = (uint *)*puStack_200;
          uStack_3b0._3_5_ = SUB85(puVar60,3);
          uStack_3b0._0_3_ =
               CONCAT12(fVar72 < *(float *)(puStack_200 + 6) &&
                        *(float *)((longlong)puStack_200 + 0x34) == fVar13,
                        CONCAT11(fVar72 >= *(float *)(puStack_200 + 6),2));
          uStack_2b8 = puStack_1c0;
          uStack_112 = 1;
          puStack_200 = _Dst_03;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar28 = *(longlong *)((longlong)plStack_f8 + 0x3df8);
          bVar71 = SUB81(extraout_RAX_41,7) >> 1;
          auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
          auVar62 = pshuflw(auVar62,auVar62,0);
          lVar31 = 0;
          puVar60 = extraout_RAX_41;
          goto LAB_1408d5667;
        }
        uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
        uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
      }
      cVar69 = (pcVar2[0xf] == -1) * -1;
      auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          cVar69 < '\0') break;
      uVar48 = uVar48 + lVar31 + 0x10;
      lVar31 = lVar31 + 0x10;
    } while( true );
  }
LAB_1408d50ca:
  uStack_378 = puStack_1c0;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  puVar22 = puStack_178;
  puVar60 = *(uint **)((longlong)plStack_f8 + 0x3df8);
  uVar38 = *(ulonglong *)((longlong)plStack_f8 + 0x3e00);
  bVar71 = SUB81(extraout_RAX_38,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  puVar59 = (ulonglong *)0x0;
  puVar52 = extraout_RAX_38;
  do {
    uVar48 = uVar38 & (ulonglong)puVar52;
    pcVar2 = (char *)((longlong)puVar60 + uVar48);
    cVar69 = SUB161(auVar62,0);
    cVar64 = SUB161(auVar62,1);
    cVar65 = SUB161(auVar62,2);
    cVar66 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar66) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar65) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar64)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar23 = (uint)uVar54;
    if (uVar54 != 0) {
LAB_1408d5140:
      uVar24 = 0;
      for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
        uVar24 = uVar24 + 1;
      }
      uVar27 = uVar24 + uVar48 & uVar38;
      puStack_200 = puVar59;
      puStack_170 = puVar60;
      if (*(uint **)(puVar60 + uVar27 * -8 + -8) != puStack_1c0) goto code_r0x0001408d5166;
      lVar28 = (longlong)(uVar27 * 0x20) >> 5;
      uVar38 = lVar28 + 0xfffffffffffffff0U & uVar38;
      pcVar2 = (char *)((longlong)puVar60 + uVar38);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      if (uVar54 == 0) {
        uVar37 = 0x10;
      }
      else {
        uVar37 = 0xf;
        if (uVar54 != 0) {
          for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
          }
        }
        uVar37 = uVar37 ^ 0xf;
      }
      pcVar2 = (char *)((longlong)puVar60 + lVar28);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      sVar8 = 0;
      for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                            (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
          uVar23 = uVar23 >> 1 | 0x80000000) {
        sVar8 = sVar8 + 1;
      }
      uVar57 = 0x80;
      if ((ushort)(uVar37 + sVar8) < 0x10) {
        *(longlong *)((longlong)plStack_f8 + 0x3e08) =
             *(longlong *)((longlong)plStack_f8 + 0x3e08) + 1;
        uVar57 = 0xff;
      }
      *(undefined1 *)((longlong)puVar60 + lVar28) = uVar57;
      *(undefined1 *)((longlong)puVar60 + uVar38 + 0x10) = uVar57;
      *(longlong *)((longlong)plStack_f8 + 0x3e10) =
           *(longlong *)((longlong)plStack_f8 + 0x3e10) + -1;
      _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,puVar60[uVar27 * -8 + -2]);
      uStack_378 = *(uint **)(puVar60 + uVar27 * -8 + -6);
      uStack_370 = *(uint **)(puVar60 + uVar27 * -8 + -4);
      dispose_collider_mesh_instance();
      if ((char)puStack_298 != '\x11') {
        LOCK();
        *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
        UNLOCK();
        if (*(longlong *)puStack_280 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      goto LAB_1408d53ad;
    }
LAB_1408d5184:
    cVar69 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d51dd;
    puVar52 = (undefined1 *)((longlong)puVar59 + uVar48 + 0x10);
    puVar59 = puVar59 + 2;
  } while( true );
LAB_1408d5667:
  uVar38 = *(ulonglong *)((longlong)plStack_f8 + 0x3e00) & (ulonglong)puVar60;
  pcVar2 = (char *)(lVar28 + uVar38);
  cVar69 = SUB161(auVar62,0);
  cVar64 = SUB161(auVar62,1);
  cVar65 = SUB161(auVar62,2);
  cVar66 = SUB161(auVar62,3);
  bVar71 = (pcVar2[0xf] == cVar66) * -1;
  auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar65) * -1,
                                       CONCAT113((pcVar2[0xd] == cVar64) * -1,
                                                 CONCAT112((pcVar2[0xc] == cVar69) * -1,
                                                           CONCAT111((pcVar2[0xb] == cVar66) * -1,
                                                                     CONCAT110((pcVar2[10] == cVar65
                                                                               ) * -1,CONCAT19((
                                                  pcVar2[9] == cVar64) * -1,
                                                  CONCAT18((pcVar2[8] == cVar69) * -1,
                                                           CONCAT17((pcVar2[7] == cVar66) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar65) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar64)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar69) * -1,
                                                           CONCAT13((pcVar2[3] == cVar66) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar65) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar64)
                                                                                         * -1,(*
                                                  pcVar2 == cVar69) * -1)))))))))))))));
  uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
  uVar23 = (uint)uVar54;
  while (puStack_170 = extraout_RAX_41, uVar54 != 0) {
    uVar24 = 0;
    for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
      uVar24 = uVar24 + 1;
    }
    uVar48 = uVar24 + uVar38 & *(ulonglong *)((longlong)plStack_f8 + 0x3e00);
    if (*(uint **)(lVar28 + -0x20 + uVar48 * -0x20) == puStack_1c0) {
      puStack_1b0 = (uint *)(lVar28 + uVar48 * -0x20);
      puStack_1a8 = puStack_1d8;
      puStack_1b8 = (uint *)0x0;
      goto LAB_1408d5762;
    }
    uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
    uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
  }
  cVar69 = (pcVar2[0xf] == -1) * -1;
  auVar63 = CONCAT115(cVar69,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar69 < '\0') goto LAB_1408d5723;
  puVar60 = (uint *)(uVar38 + lVar31 + 0x10);
  lVar31 = lVar31 + 0x10;
  goto LAB_1408d5667;
LAB_1408d5723:
  puVar60 = puStack_1c0;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e08) == 0) {
    uStack_112 = 1;
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
    puVar60 = uStack_2b8;
  }
  puStack_1b8 = puStack_1d8;
  puStack_1a8 = puStack_170;
  puStack_1b0 = puVar60;
LAB_1408d5762:
  uVar38 = (ulonglong)puStack_178 & 0xffffffff;
  _auStack_368 = CONCAT88(_Dst_03,puStack_3a0);
  uStack_378 = uStack_3b0;
  uStack_2b0._0_4_ = 0;
  uStack_2b8 = (uint *)0x0;
  puStack_2a0 = (uint *)lVar42;
  uStack_112 = 0;
  fStack_2a8 = fVar61;
  fStack_2a4 = fVar21;
  upsert_collider_entry<bevy_ecs::entity::Entity,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
            ();
  goto LAB_1408d4adb;
code_r0x0001408d5166:
  uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
  uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
  if (uVar54 == 0) goto LAB_1408d5184;
  goto LAB_1408d5140;
LAB_1408d51dd:
  if (*(longlong *)((longlong)plStack_f8 + 0x3e08) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d53ad:
  uVar38 = (ulonglong)puVar22 & 0xffffffff;
  goto LAB_1408d4adb;
code_r0x0001408d4229:
  uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
  uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
  if (uVar54 == 0) goto LAB_1408d4240;
  goto LAB_1408d420d;
LAB_1408d4290:
  if (*(longlong *)((longlong)plStack_f8 + 0x3dd8) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d4380:
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar44 = *(longlong *)((longlong)plStack_f8 + 0x3e28);
  uVar27 = *(ulonglong *)((longlong)plStack_f8 + 0x3e30);
  bVar71 = SUB81(extraout_RAX_28,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  lVar28 = 0;
  uVar51 = extraout_RAX_28;
  do {
    uVar51 = uVar27 & uVar51;
    pcVar2 = (char *)(lVar44 + uVar51);
    cVar70 = SUB161(auVar62,0);
    cVar69 = SUB161(auVar62,1);
    cVar64 = SUB161(auVar62,2);
    cVar65 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar65) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar64) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar69) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar70) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar65) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar64) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar69)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar70) * -1,
                                                           CONCAT17((pcVar2[7] == cVar65) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar64) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar69)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar70) * -1,
                                                           CONCAT13((pcVar2[3] == cVar65) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar64) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar69)
                                                                                         * -1,(*
                                                  pcVar2 == cVar70) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar23 = (uint)uVar54;
    if (uVar54 != 0) {
LAB_1408d43e8:
      uVar24 = 0;
      for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
        uVar24 = uVar24 + 1;
      }
      uVar35 = uVar24 + uVar51 & uVar27;
      if (*(uint **)(lVar44 + -0x20 + uVar35 * -0x20) != puVar53) break;
      lVar28 = (longlong)(uVar35 * 0x20) >> 5;
      uVar27 = lVar28 + 0xfffffffffffffff0U & uVar27;
      pcVar2 = (char *)(lVar44 + uVar27);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
      if (uVar54 == 0) {
        uVar37 = 0x10;
      }
      else {
        uVar37 = 0xf;
        if (uVar54 != 0) {
          for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
          }
        }
        uVar37 = uVar37 ^ 0xf;
      }
      pcVar2 = (char *)(lVar44 + lVar28);
      bVar71 = (pcVar2[0xf] == -1) * -1;
      auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                           CONCAT113((pcVar2[0xd] == -1) * -1,
                                                     CONCAT112((pcVar2[0xc] == -1) * -1,
                                                               CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                         CONCAT110((pcVar2[10] == -1
                                                                                   ) * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
      sVar8 = 0;
      for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                            (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
          uVar23 = uVar23 >> 1 | 0x80000000) {
        sVar8 = sVar8 + 1;
      }
      uVar57 = 0x80;
      if ((ushort)(uVar37 + sVar8) < 0x10) {
        *(longlong *)((longlong)plStack_f8 + 0x3e38) =
             *(longlong *)((longlong)plStack_f8 + 0x3e38) + 1;
        uVar57 = 0xff;
      }
      lVar31 = uVar35 * -0x20 + lVar44;
      *(undefined1 *)(lVar44 + lVar28) = uVar57;
      *(undefined1 *)(lVar44 + 0x10 + uVar27) = uVar57;
      *(longlong *)((longlong)plStack_f8 + 0x3e40) =
           *(longlong *)((longlong)plStack_f8 + 0x3e40) + -1;
      _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar31 + -8));
      uStack_370 = *(uint **)(lVar31 + -0x10);
      dispose_collider_mesh_instance();
      if ((char)puStack_298 != '\x11') {
        LOCK();
        *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
        UNLOCK();
        if (*(longlong *)puStack_280 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      goto LAB_1408d4550;
    }
LAB_1408d4410:
    cVar70 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar70,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar70 < '\0') goto LAB_1408d4460;
    uVar51 = uVar51 + lVar28 + 0x10;
    lVar28 = lVar28 + 0x10;
  } while( true );
  uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
  uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
  if (uVar54 == 0) goto LAB_1408d4410;
  goto LAB_1408d43e8;
LAB_1408d4460:
  if (*(longlong *)((longlong)plStack_f8 + 0x3e38) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
LAB_1408d4550:
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar44 = *(longlong *)((longlong)plStack_f8 + 0x3e58);
  uVar27 = *(ulonglong *)((longlong)plStack_f8 + 0x3e60);
  bVar71 = SUB81(extraout_RAX_29,7) >> 1;
  auVar62 = ZEXT216(CONCAT11(bVar71,bVar71));
  auVar62 = pshuflw(auVar62,auVar62,0);
  lVar28 = 0;
  uVar51 = extraout_RAX_29;
  while( true ) {
    uVar51 = uVar27 & uVar51;
    pcVar2 = (char *)(lVar44 + uVar51);
    cVar70 = SUB161(auVar62,0);
    cVar69 = SUB161(auVar62,1);
    cVar64 = SUB161(auVar62,2);
    cVar65 = SUB161(auVar62,3);
    bVar71 = (pcVar2[0xf] == cVar65) * -1;
    auVar63 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == cVar64) * -1,
                                         CONCAT113((pcVar2[0xd] == cVar69) * -1,
                                                   CONCAT112((pcVar2[0xc] == cVar70) * -1,
                                                             CONCAT111((pcVar2[0xb] == cVar65) * -1,
                                                                       CONCAT110((pcVar2[10] ==
                                                                                 cVar64) * -1,
                                                                                 CONCAT19((pcVar2[9]
                                                                                          == cVar69)
                                                                                          * -1,
                                                  CONCAT18((pcVar2[8] == cVar70) * -1,
                                                           CONCAT17((pcVar2[7] == cVar65) * -1,
                                                                    CONCAT16((pcVar2[6] == cVar64) *
                                                                             -1,CONCAT15((pcVar2[5]
                                                                                         == cVar69)
                                                                                         * -1,
                                                  CONCAT14((pcVar2[4] == cVar70) * -1,
                                                           CONCAT13((pcVar2[3] == cVar65) * -1,
                                                                    CONCAT12((pcVar2[2] == cVar64) *
                                                                             -1,CONCAT11((pcVar2[1]
                                                                                         == cVar69)
                                                                                         * -1,(*
                                                  pcVar2 == cVar70) * -1)))))))))))))));
    uVar54 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
    uVar23 = (uint)uVar54;
    if (uVar54 != 0) break;
LAB_1408d3fc0:
    cVar70 = (pcVar2[0xf] == -1) * -1;
    auVar63 = CONCAT115(cVar70,CONCAT114((pcVar2[0xe] == -1) * -1,
                                         CONCAT113((pcVar2[0xd] == -1) * -1,
                                                   CONCAT112((pcVar2[0xc] == -1) * -1,
                                                             CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                       CONCAT110((pcVar2[10] == -1)
                                                                                 * -1,CONCAT19((
                                                  pcVar2[9] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
    if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x37,0) & 1) != 0) || (SUB161(auVar63 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar63 >> 0x47,0) & 1) != 0) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar63 >> 0x77,0) & 1) != 0) || cVar70 < '\0') goto LAB_1408d4010;
    uVar51 = uVar51 + lVar28 + 0x10;
    lVar28 = lVar28 + 0x10;
  }
  while( true ) {
    uVar24 = 0;
    for (uVar25 = uVar23; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
      uVar24 = uVar24 + 1;
    }
    uVar35 = uVar24 + uVar51 & uVar27;
    if (*(uint **)(lVar44 + -0x20 + uVar35 * -0x20) == puVar53) break;
    uVar54 = (ushort)(uVar23 + 0xffffffff) & (ushort)uVar23;
    uVar23 = CONCAT22(SUB42(uVar23 + 0xffffffff,2),uVar54);
    if (uVar54 == 0) goto LAB_1408d3fc0;
  }
  lVar28 = (longlong)(uVar35 * 0x20) >> 5;
  uVar27 = lVar28 + 0xfffffffffffffff0U & uVar27;
  pcVar2 = (char *)(lVar44 + uVar27);
  bVar71 = (pcVar2[0xf] == -1) * -1;
  auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  uVar54 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(bVar71 >> 7) << 0xf;
  if (uVar54 == 0) {
    uVar37 = 0x10;
  }
  else {
    uVar37 = 0xf;
    if (uVar54 != 0) {
      for (; uVar54 >> uVar37 == 0; uVar37 = uVar37 + 0xffff) {
      }
    }
    uVar37 = uVar37 ^ 0xf;
  }
  pcVar2 = (char *)(lVar44 + lVar28);
  bVar71 = (pcVar2[0xf] == -1) * -1;
  auVar62 = CONCAT115(bVar71,CONCAT114((pcVar2[0xe] == -1) * -1,
                                       CONCAT113((pcVar2[0xd] == -1) * -1,
                                                 CONCAT112((pcVar2[0xc] == -1) * -1,
                                                           CONCAT111((pcVar2[0xb] == -1) * -1,
                                                                     CONCAT110((pcVar2[10] == -1) *
                                                                               -1,CONCAT19((pcVar2[9
                                                  ] == -1) * -1,
                                                  CONCAT18((pcVar2[8] == -1) * -1,
                                                           CONCAT17((pcVar2[7] == -1) * -1,
                                                                    CONCAT16((pcVar2[6] == -1) * -1,
                                                                             CONCAT15((pcVar2[5] ==
                                                                                      -1) * -1,
                                                  CONCAT14((pcVar2[4] == -1) * -1,
                                                           CONCAT13((pcVar2[3] == -1) * -1,
                                                                    CONCAT12((pcVar2[2] == -1) * -1,
                                                                             CONCAT11((pcVar2[1] ==
                                                                                      -1) * -1,(*
                                                  pcVar2 == -1) * -1)))))))))))))));
  sVar8 = 0;
  for (uVar23 = (ushort)((ushort)(SUB161(auVar62 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                        (ushort)(bVar71 >> 7) << 0xf) | 0x10000; (uVar23 & 1) == 0;
      uVar23 = uVar23 >> 1 | 0x80000000) {
    sVar8 = sVar8 + 1;
  }
  uVar57 = 0x80;
  if ((ushort)(uVar37 + sVar8) < 0x10) {
    *(longlong *)((longlong)plStack_f8 + 0x3e68) = *(longlong *)((longlong)plStack_f8 + 0x3e68) + 1;
    uVar57 = 0xff;
  }
  lVar31 = uVar35 * -0x20 + lVar44;
  *(undefined1 *)(lVar44 + lVar28) = uVar57;
  *(undefined1 *)(lVar44 + 0x10 + uVar27) = uVar57;
  *(longlong *)((longlong)plStack_f8 + 0x3e70) = *(longlong *)((longlong)plStack_f8 + 0x3e70) + -1;
  _auStack_368 = CONCAT124(stack0xfffffffffffffc9c,*(undefined4 *)(lVar31 + -8));
  uStack_378 = *(uint **)(lVar31 + -0x18);
  uStack_370 = *(uint **)(lVar31 + -0x10);
  dispose_collider_mesh_instance();
  puVar47 = puStack_108;
  puVar34 = puStack_110;
  if ((char)puStack_298 != '\x11') {
    LOCK();
    *(longlong *)puStack_280 = *(longlong *)puStack_280 + -1;
    UNLOCK();
    if (*(longlong *)puStack_280 == 0) {
      alloc::sync::Arc::
      drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
      puVar47 = puStack_108;
      puVar34 = puStack_110;
    }
  }
  goto LAB_1408d4036;
LAB_1408d4010:
  puVar47 = puStack_108;
  puVar34 = puStack_110;
  uStack_378 = puVar53;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e68) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
    puVar47 = puStack_108;
    puVar34 = puStack_110;
  }
  goto LAB_1408d4036;
LAB_1408d1110:
  uStack_378 = puVar47;
  if (*(longlong *)((longlong)plStack_f8 + 0x3e98) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
  goto LAB_1408d1139;
}

