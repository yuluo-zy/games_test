
void __cdecl
system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers(void)

{
  byte *pbVar1;
  char *pcVar2;
  undefined8 *puVar3;
  ulonglong *puVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  byte bVar10;
  byte bVar11;
  undefined1 uVar12;
  undefined4 uVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  code *pcVar23;
  int iVar24;
  uint uVar25;
  longlong lVar26;
  bool bVar27;
  float *pfVar28;
  undefined **ppuVar29;
  float *pfVar30;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte bVar31;
  uint uVar32;
  uint uVar33;
  uint extraout_EAX;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  uint extraout_EAX_02;
  uint extraout_EAX_03;
  uint extraout_EAX_04;
  uint extraout_EAX_05;
  uint extraout_EAX_06;
  uint extraout_EAX_07;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  ulonglong uVar34;
  float *extraout_RAX_06;
  float *extraout_RAX_07;
  float *extraout_RAX_08;
  ulonglong extraout_RAX_09;
  undefined4 *extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *plVar35;
  longlong lVar36;
  ulonglong uVar37;
  ulonglong in_RDX;
  undefined8 uVar38;
  longlong lVar39;
  undefined1 (*pauVar40) [16];
  undefined8 in_R8;
  undefined *puVar41;
  longlong lVar42;
  ushort uVar43;
  ulonglong uVar44;
  longlong lVar45;
  longlong lVar46;
  bool bVar47;
  char cVar48;
  char cVar55;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar49;
  float extraout_XMM0_Da_01;
  float fVar50;
  float fVar51;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  longlong extraout_XMM0_Qa;
  longlong extraout_XMM0_Qa_00;
  char cVar53;
  char cVar54;
  undefined1 auVar52 [16];
  float fVar56;
  float fVar57;
  float fVar59;
  undefined1 auVar58 [16];
  undefined1 auVar60 [16];
  float fVar61;
  float fVar62;
  float fVar63;
  longlong *in_stack_00000028;
  longlong in_stack_00000030;
  float local_348;
  float fStack_344;
  longlong local_338;
  longlong lStack_330;
  longlong local_328;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  longlong local_318;
  longlong lStack_310;
  longlong local_2f0;
  float local_2d8;
  float fStack_2d4;
  float local_2c8;
  float fStack_2c4;
  float local_2b8;
  float fStack_2b4;
  float local_2a8;
  float fStack_2a4;
  float local_28c;
  undefined4 uStack_288;
  undefined4 local_284;
  undefined4 local_280;
  float local_27c;
  undefined8 local_278;
  undefined4 local_270;
  undefined8 local_268;
  undefined4 local_260;
  undefined8 local_258;
  longlong local_250;
  longlong local_248;
  longlong *local_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined7 local_218;
  undefined1 uStack_211;
  float local_210;
  undefined3 uStack_20c;
  longlong local_208;
  undefined1 (*local_200) [16];
  undefined3 local_1f8;
  undefined4 uStack_1f5;
  undefined4 uStack_1f1;
  undefined4 uStack_1ed;
  undefined1 uStack_1e9;
  ulonglong local_1e8;
  ulonglong uStack_1e0;
  longlong local_1d8;
  ulonglong local_1c8;
  uint local_1bc;
  float local_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  uint uStack_1ac;
  ulonglong local_1a8;
  undefined1 auStack_1a0 [16];
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined8 uStack_184;
  float local_178;
  undefined4 uStack_174;
  float fStack_170;
  undefined4 uStack_16c;
  float local_168;
  undefined2 local_15b;
  undefined1 local_159;
  float *local_158;
  code *pcStack_150;
  float local_148;
  undefined4 uStack_144;
  longlong *local_140;
  longlong local_138;
  float *local_130;
  undefined **local_128;
  longlong lStack_120;
  float **local_118;
  ulonglong uStack_110;
  longlong local_108;
  longlong lStack_100;
  longlong *local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar27 = false;
  }
  else {
    local_140 = (longlong *)in_R8;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x5d) {
          if ((char)(&DAT_142a86f50)[in_RDX] < -0x40) {
LAB_1407c7b8d:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x5d) goto LAB_1407c7b8d;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x5d) {
          if ((char)(&DAT_142a86f52)[in_RDX] < -0x40) {
LAB_1407c7be1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x5d) goto LAB_1407c7be1;
      }
    }
    puVar41 = &DAT_142a870ef;
    do {
      bVar31 = puVar41[-1];
      uVar32 = (uint)(char)bVar31;
      if ((char)bVar31 < '\0') {
        bVar10 = puVar41[-2];
        if ((char)bVar10 < -0x40) {
          bVar11 = puVar41[-3];
          if ((char)bVar11 < -0x40) {
            pbVar1 = puVar41 + -4;
            puVar41 = puVar41 + -4;
            uVar32 = bVar11 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar41 = puVar41 + -3;
            uVar32 = bVar11 & 0xf;
          }
          uVar32 = bVar10 & 0x3f | uVar32 << 6;
        }
        else {
          puVar41 = puVar41 + -2;
          uVar32 = bVar10 & 0x1f;
        }
        uVar32 = (uint)(bVar31 & 0x3f) | uVar32 << 6;
        if (uVar32 != 0x5c) goto LAB_1407c7ca4;
LAB_1407c7cb5:
        if (puVar41 == &UNK_142a8709f) break;
        if (puVar41 + -0x142a8709f < (undefined *)0x4f) {
          if (-0x41 < (char)puVar41[1]) break;
        }
        else if (puVar41 + -0x142a8709f == (undefined *)0x4f) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar41 = puVar41 + -1;
      if (uVar32 == 0x5c) goto LAB_1407c7cb5;
LAB_1407c7ca4:
      if (uVar32 == 0x2f) goto LAB_1407c7cb5;
    } while (puVar41 != &anon_83cf05c32f3448427fe13385232527e5_14_llvm_3765002362097927207);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar35 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar35 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0))
    goto LAB_1407c9b0b;
    if (*plVar35 != 0) goto LAB_1407c9b17;
    *plVar35 = -1;
    local_f8 = plVar35;
    puffin::ThreadProfiler::begin_scope();
    *local_f8 = *local_f8 + 1;
    bVar27 = true;
  }
  country_core::resources::walls::decorator_subtype::impl$8::from();
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  lVar45 = CONCAT44(uStack_1b4,local_1b8);
  pauVar40 = (undefined1 (*) [16])CONCAT44(uStack_1ac,uStack_1b0);
  local_248 = *in_stack_00000028;
  local_250 = in_stack_00000028[1];
  local_280 = *(undefined4 *)(in_stack_00000030 + 0x1c);
  puVar14 = *(undefined4 **)(in_stack_00000030 + 0x10);
  lVar46 = auStack_1a0._8_8_;
  uVar32 = (uint)(ushort)auStack_1a0._0_2_;
  while (lVar46 != 0) {
    if ((short)uVar32 == 0) {
      do {
        auVar52 = *pauVar40;
        uVar43 = (ushort)(SUB161(auVar52 >> 7,0) & 1) | (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar52 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar52 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar52 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar52 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar52 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar52 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar52 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar52 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar52 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar52 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar52 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar52 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar52[0xf] >> 7) << 0xf;
        lVar45 = lVar45 + -0x980;
        pauVar40 = pauVar40 + 1;
      } while (uVar43 == 0xffff);
      uVar32 = ~(uint)uVar43;
      uVar33 = uVar32 & -(uint)uVar43 - 2;
    }
    else {
      if (lVar45 == 0) break;
      uVar33 = uVar32 - 1 & uVar32;
    }
    local_140 = (longlong *)CONCAT44(local_140._4_4_,uVar33);
    uVar33 = 0;
    for (; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
      uVar33 = uVar33 + 1;
    }
    local_258 = *(undefined8 *)(lVar45 + -0x98 + (ulonglong)uVar33 * -0x98);
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    lVar46 = lVar46 + -1;
    uVar32 = (uint)local_140;
    if (((extraout_RAX_02 >> 0x20 != 0) &&
        (uVar44 = extraout_RAX_02 & 0xffffffff, uVar44 < *(ulonglong *)(local_250 + 0x10))) &&
       (*(int *)(*(longlong *)(local_250 + 8) + uVar44 * 0x14) == (int)(extraout_RAX_02 >> 0x20))) {
      uVar33 = *(uint *)(*(longlong *)(local_250 + 8) + uVar44 * 0x14 + 4);
      uVar44 = (ulonglong)uVar33;
      if (((uVar44 != 0xffffffff) && (uVar44 < *(ulonglong *)(local_248 + 0x38))) &&
         ((*(ulonglong *)(*(longlong *)(local_248 + 0x28) + (ulonglong)(uVar33 >> 6) * 8) >>
           (uVar44 & 0x3f) & 1) != 0)) {
        country_core::resources::roofs::roof_shape::Roof::is_flat();
        uVar32 = (uint)local_140;
        if (extraout_AL_00 == '\0') {
          country_core::resources::walls::decorator_storage::Decorators::iter_modified();
          local_208 = CONCAT44(uStack_1b4,local_1b8);
          local_200 = (undefined1 (*) [16])CONCAT44(uStack_1ac,uStack_1b0);
          local_2f0 = auStack_1a0._8_8_;
          lVar26 = CONCAT44(uStack_18c,uStack_190);
          uVar32 = (uint)(ushort)auStack_1a0._0_2_;
LAB_1407c8024:
          if (local_2f0 != 0) {
            if ((short)uVar32 == 0) {
              do {
                auVar52 = *local_200;
                uVar43 = (ushort)(SUB161(auVar52 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar52 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar52 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar52 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar52 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar52 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar52 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar52 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar52 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar52 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar52 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar52 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar52 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar52[0xf] >> 7) << 0xf;
                local_208 = local_208 + -0x40;
                local_200 = local_200 + 1;
              } while (uVar43 == 0xffff);
              uVar32 = ~(uint)uVar43;
              local_1bc = uVar32 & -(uint)uVar43 - 2;
            }
            else {
              if (local_208 == 0) goto LAB_1407c9a0a;
              local_1bc = uVar32 - 1 & uVar32;
            }
            iVar24 = 0;
            for (; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
              iVar24 = iVar24 + 1;
            }
            local_1b8 = *(float *)((local_208 - (ulonglong)(uint)(iVar24 << 2)) + -4);
            local_1c8 = (ulonglong)(uint)local_1b8;
            if (*(longlong *)(lVar26 + 0x48) != 0) {
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                        ();
              local_2f0 = local_2f0 + -1;
              lVar15 = *(longlong *)(lVar26 + 0x30);
              bVar31 = (byte)(extraout_RAX_03 >> 0x39);
              auVar52 = ZEXT216(CONCAT11(bVar31,bVar31));
              auVar52 = pshuflw(auVar52,auVar52,0);
              lVar42 = 0;
              uVar44 = extraout_RAX_03;
              while( true ) {
                uVar44 = *(ulonglong *)(lVar26 + 0x38) & uVar44;
                pcVar2 = (char *)(lVar15 + uVar44);
                cVar48 = auVar52[0];
                auVar60[0] = -(*pcVar2 == cVar48);
                cVar53 = auVar52[1];
                auVar60[1] = -(pcVar2[1] == cVar53);
                cVar54 = auVar52[2];
                auVar60[2] = -(pcVar2[2] == cVar54);
                cVar55 = auVar52[3];
                auVar60[3] = -(pcVar2[3] == cVar55);
                auVar60[4] = -(pcVar2[4] == cVar48);
                auVar60[5] = -(pcVar2[5] == cVar53);
                auVar60[6] = -(pcVar2[6] == cVar54);
                auVar60[7] = -(pcVar2[7] == cVar55);
                auVar60[8] = -(pcVar2[8] == cVar48);
                auVar60[9] = -(pcVar2[9] == cVar53);
                auVar60[10] = -(pcVar2[10] == cVar54);
                auVar60[0xb] = -(pcVar2[0xb] == cVar55);
                auVar60[0xc] = -(pcVar2[0xc] == cVar48);
                auVar60[0xd] = -(pcVar2[0xd] == cVar53);
                auVar60[0xe] = -(pcVar2[0xe] == cVar54);
                auVar60[0xf] = -(pcVar2[0xf] == cVar55);
                uVar43 = (ushort)(SUB161(auVar60 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar60 >> 0xf,0) & 1) << 1 |
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
                         (ushort)(SUB161(auVar60 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar60[0xf] >> 7) << 0xf;
                uVar32 = (uint)uVar43;
                while (uVar43 != 0) {
                  uVar33 = 0;
                  for (uVar25 = uVar32; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
                    uVar33 = uVar33 + 1;
                  }
                  lVar39 = (uVar33 + uVar44 & *(ulonglong *)(lVar26 + 0x38)) * -0x50;
                  if ((int)local_1c8 == *(int *)(lVar15 + -0x50 + lVar39)) {
                    country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
                              ();
                    uVar32 = local_1bc;
                    if (extraout_AL_01 == '\x02') {
                      country_core::resources::walls::cached_decorator_transforms::
                      CachedDecoratorTransforms::transform();
                      local_138 = lVar15 + lVar39;
                      bVar31 = *(char *)(local_138 + -4) - 1;
                      if (2 < bVar31) {
                    // WARNING: Subroutine does not return
                        core::panicking::panic();
                      }
                      fVar49 = *(float *)(&DAT_142a87238 + (ulonglong)bVar31 * 4);
                      *puVar14 = local_280;
                      country_core::resources::rt::impl$5::from();
                      blueprint::DecoratorBlueprints::insert();
                      collision_geo::get_collision_mesh();
                      LOCK();
                      lVar15 = *extraout_RAX_05;
                      *extraout_RAX_05 = *extraout_RAX_05 + 1;
                      UNLOCK();
                      if (*extraout_RAX_05 == 0 || SCARRY8(lVar15,1) != *extraout_RAX_05 < 0) {
                    // WARNING: Does not return
                        pcVar23 = (code *)invalidInstructionException();
                        (*pcVar23)();
                      }
                      local_1b8 = (float)local_328;
                      fVar62 = local_1b8;
                      uStack_1b4 = (undefined4)((ulonglong)local_328 >> 0x20);
                      uVar13 = uStack_1b4;
                      blueprint::DecoratorBlueprint::set_colliders();
                      if (*extraout_RAX_04 != 0 || extraout_RAX_04[1] != 0) {
                        local_1b8 = 84.54523;
                        uStack_1b4 = 1;
                        uStack_1b0 = 1;
                        uStack_1ac = 0;
                        local_1a8 = 8;
                        auStack_1a0 = (undefined1  [16])0x0;
                    // WARNING: Subroutine does not return
                        core::panicking::panic_fmt();
                      }
                      extraout_RAX_04[1] = 0;
                      *extraout_RAX_04 = 1;
                      extraout_RAX_04[2] = local_338;
                      extraout_RAX_04[3] = lStack_330;
                      extraout_RAX_04[4] = local_328;
                      extraout_RAX_04[5] = CONCAT44(uStack_31c,uStack_320);
                      extraout_RAX_04[6] = local_318;
                      extraout_RAX_04[7] = lStack_310;
                      cVar48 = *(char *)(local_138 + -4);
                      uStack_1b0 = uStack_320;
                      local_1b8 = fVar62;
                      uStack_1b4 = uVar13;
                      country_core::resources::roofs::roof_shape::Roof::
                      get_roof_shape_from_pos_ws_clamped();
                      country_core::systems::wall::valid_enclosure::ShapeParameters::
                      get_closest_u_from_pos();
                      local_f8 = extraout_RAX_04;
                      if (cVar48 == '\x01') {
                        std::alloc::__default_lib_allocator::__rust_alloc();
                        if (extraout_RAX_08 == (float *)0x0) {
                    // WARNING: Subroutine does not return
                          alloc::alloc::handle_alloc_error();
                        }
                        *(longlong *)(extraout_RAX_08 + 8) = local_318;
                        *(longlong *)(extraout_RAX_08 + 10) = lStack_310;
                        *(longlong *)(extraout_RAX_08 + 4) = local_328;
                        *(longlong *)(extraout_RAX_08 + 6) = CONCAT44(uStack_31c,uStack_320);
                        *(longlong *)extraout_RAX_08 = local_338;
                        *(longlong *)(extraout_RAX_08 + 2) = lStack_330;
                        local_228 = 1;
                        local_130 = extraout_RAX_08;
                      }
                      else if (cVar48 == '\x02') {
                        country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
                        fVar62 = (fVar49 / extraout_XMM0_Da_03) * 0.25;
                        fVar49 = fmodf(extraout_XMM0_Da - fVar62,1.0);
                        fVar62 = fmodf(fVar62 + extraout_XMM0_Da,1.0);
                        country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3();
                        local_178 = (float)(~-(uint)(fVar49 < 0.0) & (uint)fVar49 |
                                           (uint)(fVar49 + 1.0) & -(uint)(fVar49 < 0.0));
                        if ((local_178 < 0.0) || (1.0 < local_178)) {
                          local_158 = &local_178;
                          pcStack_150 = core::fmt::float::impl$5::fmt;
                          local_128 = &
                                      anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125
                          ;
                          lStack_120 = 1;
                          local_108 = 0;
                          uStack_110 = 1;
                          local_118 = &local_158;
                    // WARNING: Subroutine does not return
                          core::panicking::panic_fmt();
                        }
                        if (NAN(local_178)) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic();
                        }
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar44 = local_1a8;
                        uVar37 = (ulonglong)extraout_EAX_04;
                        if (local_1a8 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        uVar34 = uVar37 + 1;
                        if (local_1a8 <= uVar34) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        local_178 = (float)(~-(uint)(fVar62 < 0.0) & (uint)fVar62 |
                                           (uint)(fVar62 + 1.0) & -(uint)(fVar62 < 0.0));
                        lVar15 = CONCAT44(uStack_1ac,uStack_1b0);
                        local_1d8._0_4_ = (float)extraout_XMM0_Qa_00;
                        fVar50 = 1.0 - (float)local_1d8;
                        fVar49 = *(float *)(lVar15 + 8 + uVar37 * 0xc);
                        uVar5 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar38 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar62 = *(float *)(lVar15 + 8 + uVar34 * 0xc) * (float)local_1d8;
                        if ((local_178 < 0.0) || (1.0 < local_178)) {
                          local_158 = &local_178;
                          pcStack_150 = core::fmt::float::impl$5::fmt;
                          local_128 = &
                                      anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125
                          ;
                          lStack_120 = 1;
                          local_108 = 0;
                          local_118 = &local_158;
                          uStack_110 = 1;
                    // WARNING: Subroutine does not return
                          core::panicking::panic_fmt();
                        }
                        if (NAN(local_178)) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic();
                        }
                        local_1d8 = extraout_XMM0_Qa_00;
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar37 = (ulonglong)extraout_EAX_05;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        uVar34 = uVar37 + 1;
                        if (uVar44 <= uVar34) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        fVar63 = 1.0 - extraout_XMM0_Da_04;
                        fVar61 = *(float *)(lVar15 + 8 + uVar37 * 0xc);
                        uVar6 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar7 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar56 = *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar34 = (ulonglong)extraout_EAX_06;
                        uVar37 = uVar34 + 1;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        fVar51 = *(float *)(lVar15 + 8 + uVar37 * 0xc) -
                                 *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        uVar8 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar9 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar57 = (float)uVar8 - (float)uVar9;
                        fVar59 = (float)((ulonglong)uVar8 >> 0x20) -
                                 (float)((ulonglong)uVar9 >> 0x20);
                        fVar59 = 1.0 / SQRT(fVar51 * fVar51 + fVar59 * fVar59 + fVar57 * fVar57);
                        local_158 = (float *)CONCAT44(fVar57 * fVar59,-fVar51 * fVar59);
                        ::utils::swizzlers::impl$0::x0y();
                        local_178 = SUB84(local_128,0);
                        uStack_174 = (float)((ulonglong)local_128 >> 0x20);
                        fStack_170 = 1.0 / SQRT(uStack_174 * uStack_174 + local_178 * local_178 +
                                                (float)lStack_120 * (float)lStack_120);
                        local_178 = fStack_170 * local_178;
                        uStack_174 = fStack_170 * uStack_174;
                        fStack_170 = fStack_170 * (float)lStack_120;
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar34 = (ulonglong)extraout_EAX_07;
                        uVar37 = uVar34 + 1;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        fVar51 = *(float *)(lVar15 + 8 + uVar37 * 0xc) -
                                 *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        uVar8 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar9 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar57 = (float)uVar8 - (float)uVar9;
                        fVar59 = (float)((ulonglong)uVar8 >> 0x20) -
                                 (float)((ulonglong)uVar9 >> 0x20);
                        fVar59 = 1.0 / SQRT(fVar51 * fVar51 + fVar59 * fVar59 + fVar57 * fVar57);
                        fVar57 = fVar57 * fVar59;
                        local_218 = (undefined7)CONCAT44(fVar57,-fVar51 * fVar59);
                        uStack_211 = (undefined1)((uint)fVar57 >> 0x18);
                        ::utils::swizzlers::impl$0::x0y();
                        fVar51 = SUB84(local_128,0);
                        fVar57 = (float)((ulonglong)local_128 >> 0x20);
                        fVar59 = 1.0 / SQRT(fVar57 * fVar57 + fVar51 * fVar51 +
                                            (float)lStack_120 * (float)lStack_120);
                        local_158 = (float *)CONCAT44(fVar59 * fVar57,fVar59 * fVar51);
                        pcStack_150 = (code *)CONCAT44(pcStack_150._4_4_,fVar59 * (float)lStack_120)
                        ;
                        std::alloc::__default_lib_allocator::__rust_alloc();
                        if (extraout_RAX_07 == (float *)0x0) {
                    // WARNING: Subroutine does not return
                          alloc::alloc::handle_alloc_error();
                        }
                        local_130 = extraout_RAX_07;
                        ::utils::xz_projective_rotation_bad();
                        lVar15 = lStack_120;
                        ppuVar29 = local_128;
                        ::utils::xz_projective_rotation_bad();
                        local_2c8 = (float)uVar5;
                        fStack_2c4 = (float)((ulonglong)uVar5 >> 0x20);
                        local_2d8 = (float)uVar38;
                        fStack_2d4 = (float)((ulonglong)uVar38 >> 0x20);
                        local_2a8 = (float)uVar6;
                        fStack_2a4 = (float)((ulonglong)uVar6 >> 0x20);
                        local_2b8 = (float)uVar7;
                        fStack_2b4 = (float)((ulonglong)uVar7 >> 0x20);
                        *(longlong *)(local_130 + 0x10) =
                             CONCAT44(extraout_XMM0_Da_04 * fStack_2b4 + fVar63 * fStack_2a4,
                                      extraout_XMM0_Da_04 * local_2b8 + fVar63 * local_2a8);
                        local_130[0x12] = fVar61 * fVar63 + fVar56 * extraout_XMM0_Da_04;
                        *(undefined ***)local_130 = ppuVar29;
                        *(longlong *)(local_130 + 2) = lVar15;
                        *(longlong *)(local_130 + 4) =
                             CONCAT44((float)local_1d8 * fStack_2d4 + fVar50 * fStack_2c4,
                                      (float)local_1d8 * local_2d8 + fVar50 * local_2c8);
                        local_130[6] = fVar49 * fVar50 + fVar62;
                        local_130[7] = 1.0;
                        local_130[8] = 1.0;
                        local_130[9] = 1.0;
                        *(undefined ***)(local_130 + 0xc) = local_128;
                        *(longlong *)(local_130 + 0xe) = lStack_120;
                        local_130[0x13] = 1.0;
                        local_130[0x14] = 1.0;
                        local_130[0x15] = 1.0;
                        if (CONCAT44(uStack_1b4,local_1b8) != 0) {
                          std::alloc::__default_lib_allocator::__rust_dealloc();
                        }
                        if (auStack_1a0._0_8_ != 0) {
                          std::alloc::__default_lib_allocator::__rust_dealloc();
                        }
                        local_228 = 2;
                      }
                      else {
                        if (cVar48 != '\x03') {
                    // WARNING: Subroutine does not return
                          core::panicking::panic();
                        }
                        country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
                        fVar62 = (fVar49 / extraout_XMM0_Da_00) / 3.0;
                        fVar49 = fmodf(extraout_XMM0_Da - fVar62,1.0);
                        fVar62 = fmodf(fVar62 + extraout_XMM0_Da,1.0);
                        country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3();
                        local_178 = (float)(~-(uint)(fVar49 < 0.0) & (uint)fVar49 |
                                           (uint)(fVar49 + 1.0) & -(uint)(fVar49 < 0.0));
                        if ((local_178 < 0.0) || (1.0 < local_178)) {
                          local_158 = &local_178;
                          pcStack_150 = core::fmt::float::impl$5::fmt;
                          local_128 = &
                                      anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125
                          ;
                          lStack_120 = 1;
                          local_108 = 0;
                          uStack_110 = 1;
                          local_118 = &local_158;
                    // WARNING: Subroutine does not return
                          core::panicking::panic_fmt();
                        }
                        if (NAN(local_178)) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic();
                        }
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar44 = local_1a8;
                        uVar37 = (ulonglong)extraout_EAX;
                        if (local_1a8 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        uVar34 = uVar37 + 1;
                        if (local_1a8 <= uVar34) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        local_178 = (float)(~-(uint)(fVar62 < 0.0) & (uint)fVar62 |
                                           (uint)(fVar62 + 1.0) & -(uint)(fVar62 < 0.0));
                        lVar15 = CONCAT44(uStack_1ac,uStack_1b0);
                        fVar50 = 1.0 - extraout_XMM0_Da_01;
                        fVar49 = *(float *)(lVar15 + 8 + uVar37 * 0xc);
                        uVar5 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar38 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar62 = *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        if ((local_178 < 0.0) || (1.0 < local_178)) {
                          local_158 = &local_178;
                          pcStack_150 = core::fmt::float::impl$5::fmt;
                          local_128 = &
                                      anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125
                          ;
                          lStack_120 = 1;
                          local_108 = 0;
                          local_118 = &local_158;
                          uStack_110 = 1;
                    // WARNING: Subroutine does not return
                          core::panicking::panic_fmt();
                        }
                        if (NAN(local_178)) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic();
                        }
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar37 = (ulonglong)extraout_EAX_00;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        uVar34 = uVar37 + 1;
                        if (uVar44 <= uVar34) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        local_1d8._0_4_ = (float)extraout_XMM0_Qa;
                        fVar63 = 1.0 - (float)local_1d8;
                        fVar61 = *(float *)(lVar15 + 8 + uVar37 * 0xc);
                        local_238 = *(ulonglong *)(lVar15 + uVar37 * 0xc);
                        uStack_230 = 0;
                        uVar6 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar56 = *(float *)(lVar15 + 8 + uVar34 * 0xc) * (float)local_1d8;
                        local_1d8 = extraout_XMM0_Qa;
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar34 = (ulonglong)extraout_EAX_01;
                        uVar37 = uVar34 + 1;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        fVar51 = *(float *)(lVar15 + 8 + uVar37 * 0xc) -
                                 *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        uVar7 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar8 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar57 = (float)uVar7 - (float)uVar8;
                        fVar59 = (float)((ulonglong)uVar7 >> 0x20) -
                                 (float)((ulonglong)uVar8 >> 0x20);
                        fVar59 = 1.0 / SQRT(fVar51 * fVar51 + fVar59 * fVar59 + fVar57 * fVar57);
                        local_158 = (float *)CONCAT44(fVar57 * fVar59,-fVar51 * fVar59);
                        ::utils::swizzlers::impl$0::x0y();
                        fVar51 = SUB84(local_128,0);
                        fVar57 = (float)((ulonglong)local_128 >> 0x20);
                        fVar59 = 1.0 / SQRT(fVar57 * fVar57 + fVar51 * fVar51 +
                                            (float)lStack_120 * (float)lStack_120);
                        local_210 = fVar59 * (float)lStack_120;
                        local_218 = (undefined7)CONCAT44(fVar59 * fVar57,fVar59 * fVar51);
                        uStack_211 = (undefined1)((uint)(fVar59 * fVar57) >> 0x18);
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar34 = (ulonglong)extraout_EAX_02;
                        uVar37 = uVar34 + 1;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        fVar51 = *(float *)(lVar15 + 8 + uVar37 * 0xc) -
                                 *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        uVar7 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar8 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar57 = (float)uVar7 - (float)uVar8;
                        fVar59 = (float)((ulonglong)uVar7 >> 0x20) -
                                 (float)((ulonglong)uVar8 >> 0x20);
                        fVar59 = 1.0 / SQRT(fVar51 * fVar51 + fVar59 * fVar59 + fVar57 * fVar57);
                        local_158 = (float *)CONCAT44(fVar57 * fVar59,-fVar51 * fVar59);
                        ::utils::swizzlers::impl$0::x0y();
                        local_240 = (longlong *)CONCAT44(local_240._4_4_,fVar49 * fVar50);
                        local_178 = SUB84(local_128,0);
                        uStack_174 = (float)((ulonglong)local_128 >> 0x20);
                        fStack_170 = 1.0 / SQRT(uStack_174 * uStack_174 + local_178 * local_178 +
                                                (float)lStack_120 * (float)lStack_120);
                        local_178 = fStack_170 * local_178;
                        uStack_174 = fStack_170 * uStack_174;
                        fStack_170 = fStack_170 * (float)lStack_120;
                        local_27c = fVar62 * extraout_XMM0_Da_01;
                        local_228._0_4_ = extraout_XMM0_Da_01;
                        std::alloc::__default_lib_allocator::__rust_alloc();
                        if (extraout_RAX_06 == (float *)0x0) {
                    // WARNING: Subroutine does not return
                          alloc::alloc::handle_alloc_error();
                        }
                        local_130 = extraout_RAX_06;
                        ::utils::xz_projective_rotation_bad();
                        lVar42 = lStack_120;
                        ppuVar29 = local_128;
                        if ((extraout_XMM0_Da < 0.0) || (1.0 < extraout_XMM0_Da)) {
                          local_158 = &local_28c;
                          pcStack_150 = core::fmt::float::impl$5::fmt;
                          local_128 = &
                                      anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125
                          ;
                          lStack_120 = 1;
                          local_108 = 0;
                          local_118 = &local_158;
                          uStack_110 = 1;
                    // WARNING: Subroutine does not return
                          core::panicking::panic_fmt();
                        }
                        if (NAN(extraout_XMM0_Da)) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic();
                        }
                        local_28c = extraout_XMM0_Da;
                        ::utils::curve::Curve::
                        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                                  ();
                        uVar37 = (ulonglong)extraout_EAX_03;
                        if (uVar44 <= uVar37) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        uVar34 = uVar37 + 1;
                        if (uVar44 <= uVar34) {
                    // WARNING: Subroutine does not return
                          core::panicking::panic_bounds_check();
                        }
                        fVar49 = *(float *)(lVar15 + 8 + uVar37 * 0xc);
                        uVar7 = *(undefined8 *)(lVar15 + uVar37 * 0xc);
                        uVar8 = *(undefined8 *)(lVar15 + uVar34 * 0xc);
                        fVar62 = *(float *)(lVar15 + 8 + uVar34 * 0xc);
                        ::utils::xz_projective_rotation_bad();
                        local_2b8 = (float)uVar5;
                        fStack_2b4 = (float)((ulonglong)uVar5 >> 0x20);
                        local_2c8 = (float)uVar38;
                        fStack_2c4 = (float)((ulonglong)uVar38 >> 0x20);
                        local_348 = (float)uVar6;
                        fStack_344 = (float)((ulonglong)uVar6 >> 0x20);
                        fVar51 = 1.0 - extraout_XMM0_Da_02;
                        *(longlong *)(local_130 + 0x1c) =
                             CONCAT44((float)local_1d8 * fStack_344 + fVar63 * local_238._4_4_,
                                      (float)local_1d8 * local_348 + fVar63 * (float)local_238);
                        local_130[0x1e] = fVar61 * fVar63 + fVar56;
                        *(undefined ***)local_130 = ppuVar29;
                        *(longlong *)(local_130 + 2) = lVar42;
                        *(longlong *)(local_130 + 4) =
                             CONCAT44((float)local_228 * fStack_2c4 + fVar50 * fStack_2b4,
                                      (float)local_228 * local_2c8 + fVar50 * local_2b8);
                        local_130[6] = local_240._0_4_ + local_27c;
                        local_130[7] = 1.0;
                        local_130[8] = 1.0;
                        local_130[9] = 1.0;
                        *(longlong *)(local_130 + 0xc) = local_338;
                        *(longlong *)(local_130 + 0xe) = lStack_330;
                        *(longlong *)(local_130 + 0x10) =
                             CONCAT44(extraout_XMM0_Da_02 * (float)((ulonglong)uVar8 >> 0x20) +
                                      fVar51 * (float)((ulonglong)uVar7 >> 0x20),
                                      extraout_XMM0_Da_02 * (float)uVar8 + fVar51 * (float)uVar7);
                        local_130[0x12] = fVar62 * extraout_XMM0_Da_02 + fVar49 * fVar51;
                        local_130[0x13] = 1.0;
                        local_130[0x14] = 1.0;
                        local_130[0x15] = 1.0;
                        *(undefined ***)(local_130 + 0x18) = local_128;
                        *(longlong *)(local_130 + 0x1a) = lStack_120;
                        local_130[0x1f] = 1.0;
                        local_130[0x20] = 1.0;
                        local_130[0x21] = 1.0;
                        if (CONCAT44(uStack_1b4,local_1b8) != 0) {
                          std::alloc::__default_lib_allocator::__rust_dealloc();
                        }
                        if (auStack_1a0._0_8_ != 0) {
                          std::alloc::__default_lib_allocator::__rust_dealloc();
                        }
                        local_228 = 3;
                      }
                      ::utils::rng::init_rng();
                      if ((*(int *)(local_138 + -0x1c) * -0x5555554a + 0x5556884bU < 0x55555556) &&
                         (*(byte *)(local_138 + -4) < 3)) {
                        bVar47 = false;
                        local_238 = extraout_RAX_09;
                      }
                      else {
                        local_238 = extraout_RAX_09 + 0xa0761d6478bd642f;
                        auVar52._8_8_ = 0;
                        auVar52._0_8_ = local_238 ^ 0xe7037ed1a0b428db;
                        auVar20._8_8_ = 0;
                        auVar20._0_8_ = local_238;
                        bVar47 = (SUB164(auVar52 * auVar20,0) ^ SUB164(auVar52 * auVar20,8)) >> 0x1e
                                 == 1;
                      }
                      iVar24 = (int)local_228;
                      local_240 = local_f8 + 0x1f;
                      lVar15 = 0;
                      lVar42 = local_1d8;
                      pfVar30 = local_130;
                      while (pfVar28 = pfVar30, local_1d8 = lVar15,
                            pfVar28 !=
                            (float *)((ulonglong)(uint)(iVar24 * 0x30) + (longlong)local_130)) {
                        local_128 = *(undefined ***)pfVar28;
                        lStack_120 = *(longlong *)(pfVar28 + 2);
                        local_118 = *(float ***)(pfVar28 + 4);
                        uStack_110 = *(ulonglong *)(pfVar28 + 6);
                        local_108 = *(longlong *)(pfVar28 + 8);
                        lStack_100 = *(longlong *)(pfVar28 + 10);
                        uStack_1ac = uStack_1ac & 0xff000000;
                        local_1b8 = (float)((uint)local_1b8 & 0xffffff);
                        uStack_1b4 = 0;
                        uStack_1b0 = 0;
                        uVar12 = *(undefined1 *)(local_138 + -1);
                        fStack_170 = 0.0;
                        uStack_16c._3_1_ = (undefined1)((uint)uStack_16c >> 0x18);
                        uStack_16c._0_3_ = 0;
                        local_178 = local_1b8;
                        uStack_174 = 0.0;
                        uVar13 = *(undefined4 *)(local_138 + -0x20);
                        lVar15 = local_f8[0x18];
                        if (lVar15 == local_f8[0x16]) {
                          alloc::raw_vec::RawVec::
                          grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                                    ();
                        }
                        lVar42 = local_f8[0x17];
                        lVar39 = lVar15 * 0x160;
                        *(undefined8 *)(lVar42 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x10 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x20 + lVar39) = 0;
                        *(undefined4 *)(lVar42 + 0x30 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x34 + lVar39) = local_278;
                        *(undefined4 *)(lVar42 + 0x3c + lVar39) = local_270;
                        *(undefined4 *)(lVar42 + 0x40 + lVar39) = 0;
                        puVar3 = (undefined8 *)(lVar42 + 0x44 + lVar39);
                        *puVar3 = CONCAT44(uStack_1b4,local_1b8);
                        puVar3[1] = CONCAT44(uStack_1ac,uStack_1b0);
                        puVar4 = (ulonglong *)(lVar42 + 0x54 + lVar39);
                        *puVar4 = local_1a8;
                        puVar4[1] = auStack_1a0._0_8_;
                        puVar3 = (undefined8 *)(lVar42 + 100 + lVar39);
                        *puVar3 = auStack_1a0._8_8_;
                        puVar3[1] = CONCAT44(uStack_18c,uStack_190);
                        puVar3 = (undefined8 *)(lVar42 + 0x70 + lVar39);
                        *puVar3 = CONCAT44(uStack_188,uStack_18c);
                        puVar3[1] = uStack_184;
                        *(undefined4 *)(lVar42 + 0x80 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x84 + lVar39) = local_268;
                        *(undefined4 *)(lVar42 + 0x8c + lVar39) = local_260;
                        *(undefined4 *)(lVar42 + 0x90 + lVar39) = 0;
                        *(ulonglong *)(lVar42 + 0x94 + lVar39) = CONCAT44(uStack_288,local_28c);
                        *(undefined4 *)(lVar42 + 0x9c + lVar39) = local_284;
                        *(undefined **)(lVar42 + 0xa0 + lVar39) = &DAT_142a87120;
                        *(undefined8 *)(lVar42 + 0xa8 + lVar39) = 0x2a;
                        *(undefined **)(lVar42 + 0xb0 + lVar39) = &DAT_142a8714a;
                        *(undefined8 *)(lVar42 + 0xb8 + lVar39) = 0x1e;
                        plVar35 = (longlong *)(lVar42 + 0xe0 + lVar39);
                        *plVar35 = local_108;
                        plVar35[1] = lStack_100;
                        plVar35 = (longlong *)(lVar42 + 0xd0 + lVar39);
                        *plVar35 = (longlong)local_118;
                        plVar35[1] = uStack_110;
                        plVar35 = (longlong *)(lVar42 + 0xc0 + lVar39);
                        *plVar35 = (longlong)local_128;
                        plVar35[1] = lStack_120;
                        *(undefined4 *)(lVar42 + 0xf0 + lVar39) = 1;
                        *(undefined4 *)(lVar42 + 0xf4 + lVar39) = uVar13;
                        *(undefined4 *)(lVar42 + 0xf8 + lVar39) = 0;
                        *(ulonglong *)(lVar42 + 0x10c + lVar39) = CONCAT44(uStack_144,local_148);
                        plVar35 = (longlong *)(lVar42 + 0xfc + lVar39);
                        *plVar35 = (longlong)local_158;
                        plVar35[1] = (longlong)pcStack_150;
                        *(undefined1 *)(lVar42 + 0x114 + lVar39) = 2;
                        *(undefined1 *)(lVar42 + 0x117 + lVar39) = local_159;
                        *(undefined2 *)(lVar42 + 0x115 + lVar39) = local_15b;
                        *(undefined4 *)(lVar42 + 0x118 + lVar39) = 0;
                        puVar3 = (undefined8 *)(lVar42 + 0x11c + lVar39);
                        *puVar3 = CONCAT17((undefined1)uStack_1f1,CONCAT43(uStack_1f5,local_1f8));
                        puVar3[1] = CONCAT17(uStack_1e9,CONCAT43(uStack_1ed,uStack_1f1._1_3_));
                        puVar4 = (ulonglong *)(lVar42 + 300 + lVar39);
                        *puVar4 = local_1e8;
                        puVar4[1] = uStack_1e0;
                        *(undefined1 *)(lVar42 + 0x13c + lVar39) = 2;
                        *(ulonglong *)(lVar42 + 0x144 + lVar39) =
                             CONCAT35((undefined3)uStack_16c,CONCAT41(fStack_170,uStack_174._3_1_));
                        *(ulonglong *)(lVar42 + 0x13d + lVar39) = CONCAT44(uStack_174,local_178);
                        *(undefined1 *)(lVar42 + 0x14c + lVar39) = 2;
                        *(undefined1 *)(lVar42 + 0x14e + lVar39) = uVar12;
                        *(undefined2 *)(lVar42 + 0x14f + lVar39) = 0x900;
                        *(ulonglong *)(lVar42 + 0x151 + lVar39) = CONCAT17(uStack_211,local_218);
                        *(ulonglong *)(lVar42 + 0x158 + lVar39) =
                             CONCAT35(uStack_20c,CONCAT41(local_210,uStack_211));
                        local_f8[0x18] = lVar15 + 1;
                        local_1a8 = *(ulonglong *)(pfVar28 + 4);
                        auStack_1a0 = *(undefined1 (*) [16])(pfVar28 + 6);
                        local_1b8 = (float)*(longlong *)pfVar28;
                        uStack_1b4 = (undefined4)((ulonglong)*(longlong *)pfVar28 >> 0x20);
                        uStack_1b0 = (undefined4)*(longlong *)(pfVar28 + 2);
                        uStack_1ac = (uint)((ulonglong)*(longlong *)(pfVar28 + 2) >> 0x20);
                        uStack_190 = (undefined4)*(longlong *)(pfVar28 + 10);
                        uStack_18c = (undefined4)((ulonglong)*(longlong *)(pfVar28 + 10) >> 0x20);
                        uStack_1ed = 0;
                        uStack_1f5 = 0;
                        uStack_1f1 = 0;
                        bVar31 = *(byte *)(local_138 + -4);
                        country_core::resources::walls::decorator::DecoratorExtraInfo::as_window();
                        uVar13 = *extraout_RAX_10;
                        local_128 = (undefined **)CONCAT44(uStack_1b4,local_1b8);
                        lStack_120 = CONCAT44(uStack_1ac,uStack_1b0);
                        lStack_100 = CONCAT44(uStack_18c,uStack_190);
                        local_108 = auStack_1a0._8_8_;
                        local_118 = (float **)local_1a8;
                        uStack_110 = auStack_1a0._0_8_;
                        local_178 = (float)CONCAT43(uStack_1f5,local_1f8);
                        uStack_174._3_1_ = (undefined1)uStack_1f1;
                        uStack_174 = (float)CONCAT13(uStack_174._3_1_,(int3)((uint)uStack_1f5 >> 8))
                        ;
                        fStack_170 = (float)(CONCAT44(uStack_1ed,uStack_1f1) >> 8);
                        uStack_16c._0_3_ = (undefined3)((uint)uStack_1ed >> 8);
                        uVar12 = *(undefined1 *)(local_138 + -1);
                        lVar15 = local_f8[0x18];
                        if (lVar15 == local_f8[0x16]) {
                          alloc::raw_vec::RawVec::
                          grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                                    ();
                        }
                        lVar42 = local_f8[0x17];
                        lVar39 = lVar15 * 0x160;
                        *(undefined8 *)(lVar42 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x10 + lVar39) = 1;
                        *(ulonglong *)(lVar42 + 0x18 + lVar39) =
                             (ulonglong)bVar31 * 1000 + local_1c8;
                        *(undefined8 *)(lVar42 + 0x20 + lVar39) = 0;
                        *(undefined4 *)(lVar42 + 0x30 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x34 + lVar39) = local_278;
                        *(undefined4 *)(lVar42 + 0x3c + lVar39) = local_270;
                        *(undefined4 *)(lVar42 + 0x40 + lVar39) = 0;
                        puVar3 = (undefined8 *)(lVar42 + 0x44 + lVar39);
                        *puVar3 = CONCAT44(uStack_1b4,local_1b8);
                        puVar3[1] = CONCAT44(uStack_1ac,uStack_1b0);
                        puVar4 = (ulonglong *)(lVar42 + 0x54 + lVar39);
                        *puVar4 = local_1a8;
                        puVar4[1] = auStack_1a0._0_8_;
                        puVar3 = (undefined8 *)(lVar42 + 100 + lVar39);
                        *puVar3 = auStack_1a0._8_8_;
                        puVar3[1] = CONCAT44(uStack_18c,uStack_190);
                        puVar3 = (undefined8 *)(lVar42 + 0x70 + lVar39);
                        *puVar3 = CONCAT44(uStack_188,uStack_18c);
                        puVar3[1] = uStack_184;
                        *(undefined4 *)(lVar42 + 0x80 + lVar39) = 0;
                        *(undefined8 *)(lVar42 + 0x84 + lVar39) = local_268;
                        *(undefined4 *)(lVar42 + 0x8c + lVar39) = local_260;
                        *(undefined4 *)(lVar42 + 0x90 + lVar39) = 0;
                        *(ulonglong *)(lVar42 + 0x94 + lVar39) = CONCAT44(uStack_288,local_28c);
                        *(undefined4 *)(lVar42 + 0x9c + lVar39) = local_284;
                        *(char **)(lVar42 + 0xa0 + lVar39) =
                             "decorators/window_cottage_1x1_dormer_glass";
                        *(undefined8 *)(lVar42 + 0xa8 + lVar39) = 0x2a;
                        *(undefined **)(lVar42 + 0xb0 + lVar39) =
                             &anon_b17bdb6b936b48c513e8fa54c354b9a3_5_llvm_3328826292648170336;
                        *(undefined8 *)(lVar42 + 0xb8 + lVar39) = 0x24;
                        puVar3 = (undefined8 *)(lVar42 + 0xe0 + lVar39);
                        *puVar3 = local_108;
                        puVar3[1] = lStack_100;
                        puVar4 = (ulonglong *)(lVar42 + 0xd0 + lVar39);
                        *puVar4 = (ulonglong)local_118;
                        puVar4[1] = uStack_110;
                        puVar3 = (undefined8 *)(lVar42 + 0xc0 + lVar39);
                        *puVar3 = local_128;
                        puVar3[1] = lStack_120;
                        *(undefined4 *)(lVar42 + 0xf0 + lVar39) = 1;
                        *(undefined4 *)(lVar42 + 0xf4 + lVar39) = uVar13;
                        *(undefined4 *)(lVar42 + 0xf8 + lVar39) = 0;
                        *(ulonglong *)(lVar42 + 0x10c + lVar39) = CONCAT44(uStack_144,local_148);
                        plVar35 = (longlong *)(lVar42 + 0xfc + lVar39);
                        *plVar35 = (longlong)local_158;
                        plVar35[1] = (longlong)pcStack_150;
                        *(undefined1 *)(lVar42 + 0x114 + lVar39) = 2;
                        *(undefined1 *)(lVar42 + 0x117 + lVar39) = local_159;
                        *(undefined2 *)(lVar42 + 0x115 + lVar39) = local_15b;
                        *(undefined4 *)(lVar42 + 0x118 + lVar39) = 0;
                        puVar3 = (undefined8 *)(lVar42 + 0x11c + lVar39);
                        *puVar3 = CONCAT17((undefined1)uStack_1f1,CONCAT43(uStack_1f5,local_1f8));
                        puVar3[1] = CONCAT17(uStack_1e9,CONCAT43(uStack_1ed,uStack_1f1._1_3_));
                        puVar4 = (ulonglong *)(lVar42 + 300 + lVar39);
                        *puVar4 = local_1e8;
                        puVar4[1] = uStack_1e0;
                        *(undefined1 *)(lVar42 + 0x13c + lVar39) = 2;
                        *(ulonglong *)(lVar42 + 0x144 + lVar39) =
                             CONCAT35((undefined3)uStack_16c,CONCAT41(fStack_170,uStack_174._3_1_));
                        *(ulonglong *)(lVar42 + 0x13d + lVar39) = CONCAT44(uStack_174,local_178);
                        *(undefined1 *)(lVar42 + 0x14c + lVar39) = 2;
                        *(undefined1 *)(lVar42 + 0x14e + lVar39) = uVar12;
                        *(undefined2 *)(lVar42 + 0x14f + lVar39) = 0x900;
                        *(ulonglong *)(lVar42 + 0x151 + lVar39) = CONCAT17(uStack_211,local_218);
                        *(ulonglong *)(lVar42 + 0x158 + lVar39) =
                             CONCAT35(uStack_20c,CONCAT41(local_210,uStack_211));
                        local_f8[0x18] = lVar15 + 1;
                        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::
                        get_instruction();
                        fVar49 = *pfVar28;
                        fVar62 = pfVar28[1];
                        fVar50 = pfVar28[2];
                        fVar61 = pfVar28[3];
                        fVar57 = fVar61 * fVar61;
                        fVar59 = fVar50 * fVar50 + fVar62 * fVar62 + fVar49 * fVar49;
                        fVar63 = fVar50 * -0.05 + fVar62 * 0.05 + fVar49 * 0.05;
                        fVar63 = fVar63 + fVar63;
                        fVar61 = fVar61 + fVar61;
                        fVar56 = fVar61 * (fVar49 * 0.05 - fVar62 * 0.05) +
                                 fVar63 * fVar50 + (fVar57 - fVar59) * -0.05 + pfVar28[6] + 0.0;
                        fVar51 = (float)*(longlong *)(pfVar28 + 4) +
                                 fVar61 * (fVar62 * -0.05 - fVar50 * 0.05) +
                                 fVar63 * fVar49 + (fVar57 - fVar59) * 0.05 + 0.0;
                        fVar49 = (float)((ulonglong)*(longlong *)(pfVar28 + 4) >> 0x20) +
                                 fVar61 * (fVar50 * 0.05 - fVar49 * -0.05) +
                                 fVar63 * fVar62 + (fVar57 - fVar59) * 0.05 + 0.5;
                        local_1f8 = SUB43(fVar51,0);
                        uStack_1f5 = (undefined4)(CONCAT44(fVar49,fVar51) >> 0x18);
                        uStack_1f1._1_3_ = SUB43(fVar56,0);
                        uStack_1f1 = CONCAT31(uStack_1f1._1_3_,(char)((uint)fVar49 >> 0x18));
                        uStack_1ed._0_1_ = (undefined1)((uint)fVar56 >> 0x18);
                        local_128 = (undefined **)0x3f8000003f4ccccd;
                        lStack_120 = CONCAT44(lStack_120._4_4_,0x3e4ccccd);
                        country_core::resources::walls::dormers::DormerHoleSsbo::cottage();
                        lVar42 = local_f8[0x1e];
                        if (lVar42 == local_f8[0x1c]) {
                          alloc::raw_vec::RawVec::
                          grow_one<country_core::resources::walls::dormers::DormerHoleSsbo,alloc::alloc::Global>
                                    ();
                        }
                        lVar15 = local_1d8 + 1;
                        lVar39 = local_f8[0x1d];
                        lVar36 = lVar42 * 0x30;
                        puVar3 = (undefined8 *)(lVar39 + 0x20 + lVar36);
                        *puVar3 = auStack_1a0._8_8_;
                        puVar3[1] = CONCAT44(uStack_18c,uStack_190);
                        puVar4 = (ulonglong *)(lVar39 + 0x10 + lVar36);
                        *puVar4 = local_1a8;
                        puVar4[1] = auStack_1a0._0_8_;
                        *(undefined8 *)(lVar39 + lVar36) = CONCAT44(uStack_1b4,local_1b8);
                        ((undefined8 *)(lVar39 + lVar36))[1] = CONCAT44(uStack_1ac,uStack_1b0);
                        local_f8[0x1e] = lVar42 + 1;
                        lVar42 = local_1d8;
                        pfVar30 = pfVar28 + 0xc;
                        if (bVar47) {
                          local_158 = *(float **)pfVar28;
                          pcStack_150 = *(code **)(pfVar28 + 2);
                          local_148 = pfVar28[4];
                          fVar49 = pfVar28[5];
                          local_118 = *(float ***)(pfVar28 + 10);
                          local_128 = *(undefined ***)(pfVar28 + 6);
                          lStack_120 = *(longlong *)(pfVar28 + 8);
                          local_1a8 = CONCAT44(local_1a8._4_4_,(int)local_1c8);
                          local_1b8 = 84.222595;
                          uStack_1b4 = 1;
                          uStack_1b0 = 10;
                          uStack_1ac = 0;
                          auStack_1a0._0_8_ = local_1d8;
                          ::utils::
                          calculate_hash<tuple_<country_core::resources::walls::decorator::DecoratorId,ref_<str_>,usize>_>
                                    ();
                          uVar5 = local_258;
                          uVar44 = local_238 + 0xa0761d6478bd642f;
                          auVar16._8_8_ = 0;
                          auVar16._0_8_ = uVar44 ^ 0xe7037ed1a0b428db;
                          auVar21._8_8_ = 0;
                          auVar21._0_8_ = uVar44;
                          uVar37 = SUB168(auVar16 * auVar21,8) ^ SUB168(auVar16 * auVar21,0);
                          auVar17._8_8_ = 0;
                          auVar17._0_8_ = uVar37;
                          uVar38 = SUB168(auVar17 * ZEXT816(6),8);
                          if (uVar37 * 6 < 4) {
                            local_238 = local_238 + 0x40ec3ac8f17ac85e;
                            do {
                              auVar18._8_8_ = 0;
                              auVar18._0_8_ = local_238 ^ 0xe7037ed1a0b428db;
                              auVar22._8_8_ = 0;
                              auVar22._0_8_ = local_238;
                              uVar44 = uVar44 + 0xa0761d6478bd642f;
                              uVar37 = SUB168(auVar18 * auVar22,8) ^ SUB168(auVar18 * auVar22,0);
                              local_238 = local_238 + 0xa0761d6478bd642f;
                            } while (uVar37 * 6 < 4);
                            auVar19._8_8_ = 0;
                            auVar19._0_8_ = uVar37;
                            uVar38 = SUB168(auVar19 * ZEXT816(6),8);
                          }
                          local_168 = local_148;
                          local_178 = SUB84(local_158,0);
                          uStack_174 = (float)((ulonglong)local_158 >> 0x20);
                          fStack_170 = SUB84(pcStack_150,0);
                          uStack_16c = (undefined4)((ulonglong)pcStack_150 >> 0x20);
                          local_1f8 = SUB83(local_128,0);
                          uStack_1f5 = (undefined4)((ulonglong)local_128 >> 0x18);
                          uStack_1f1._0_1_ = (undefined1)((ulonglong)local_128 >> 0x38);
                          uStack_1f1._1_3_ = (undefined3)lStack_120;
                          uStack_1ed = (undefined4)((ulonglong)lStack_120 >> 0x18);
                          uStack_1e9 = (undefined1)((ulonglong)lStack_120 >> 0x38);
                          local_1e8 = (ulonglong)local_118;
                          lVar42 = local_f8[0x21];
                          local_238 = uVar44;
                          local_1d8 = extraout_RAX_11;
                          if (lVar42 == local_f8[0x1f]) {
                            alloc::raw_vec::RawVec::
                            grow_one<tuple_<glam::f32::affine3a::Affine3A,u32,array_<f32,2>_>,alloc::alloc::Global>
                                      ();
                          }
                          lVar39 = local_f8[0x20];
                          lVar36 = lVar42 * 0x50;
                          *(undefined1 *)(lVar39 + lVar36) = 1;
                          *(undefined8 *)(lVar39 + 8 + lVar36) = uVar38;
                          puVar3 = (undefined8 *)(lVar39 + 0x10 + lVar36);
                          *puVar3 = CONCAT44(uStack_174,local_178);
                          puVar3[1] = CONCAT44(uStack_16c,fStack_170);
                          *(float *)(lVar39 + 0x20 + lVar36) = local_168;
                          *(float *)(lVar39 + 0x24 + lVar36) = fVar49 + -0.1;
                          puVar3 = (undefined8 *)(lVar39 + 0x28 + lVar36);
                          *puVar3 = CONCAT17((undefined1)uStack_1f1,CONCAT43(uStack_1f5,local_1f8));
                          puVar3[1] = CONCAT17(uStack_1e9,CONCAT43(uStack_1ed,uStack_1f1._1_3_));
                          *(ulonglong *)(lVar39 + 0x38 + lVar36) = local_1e8;
                          *(longlong *)(lVar39 + 0x40 + lVar36) = local_1d8;
                          *(undefined8 *)(lVar39 + 0x48 + lVar36) = uVar5;
                          local_f8[0x21] = lVar42 + 1;
                          lVar42 = local_1d8;
                        }
                      }
                      local_1d8 = lVar42;
                      std::alloc::__default_lib_allocator::__rust_dealloc();
                      uVar32 = local_1bc;
                    }
                    goto LAB_1407c8024;
                  }
                  uVar43 = (ushort)(uVar32 - 1) & (ushort)uVar32;
                  uVar32 = CONCAT22((short)(uVar32 - 1 >> 0x10),uVar43);
                }
                auVar58[0] = -(*pcVar2 == -1);
                auVar58[1] = -(pcVar2[1] == -1);
                auVar58[2] = -(pcVar2[2] == -1);
                auVar58[3] = -(pcVar2[3] == -1);
                auVar58[4] = -(pcVar2[4] == -1);
                auVar58[5] = -(pcVar2[5] == -1);
                auVar58[6] = -(pcVar2[6] == -1);
                auVar58[7] = -(pcVar2[7] == -1);
                auVar58[8] = -(pcVar2[8] == -1);
                auVar58[9] = -(pcVar2[9] == -1);
                auVar58[10] = -(pcVar2[10] == -1);
                auVar58[0xb] = -(pcVar2[0xb] == -1);
                auVar58[0xc] = -(pcVar2[0xc] == -1);
                auVar58[0xd] = -(pcVar2[0xd] == -1);
                auVar58[0xe] = -(pcVar2[0xe] == -1);
                auVar58[0xf] = -(pcVar2[0xf] == -1);
                if ((((((((((((((((SUB161(auVar58 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar58 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar58 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar58 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar58 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar58 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar58 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar58 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar58 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar58 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar58 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar58 >> 0x5f,0) & 1) != 0) || (SUB161(auVar58 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar58 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar58 >> 0x77,0) & 1) != 0) || auVar58[0xf] < '\0') break;
                uVar44 = uVar44 + lVar42 + 0x10;
                lVar42 = lVar42 + 0x10;
              }
            }
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
LAB_1407c9a0a:
          uVar32 = (uint)local_140;
        }
      }
    }
  }
  if (bVar27) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_12 == 1) {
      plVar35 = extraout_RAX_12 + 1;
    }
    else if ((*extraout_RAX_12 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar35 = extraout_RAX_13, extraout_RAX_13 == (longlong *)0x0)) {
LAB_1407c9b0b:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar35 != 0) {
LAB_1407c9b17:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar35 = -1;
    local_140 = plVar35;
    puffin::ThreadProfiler::end_scope();
    *local_140 = *local_140 + 1;
  }
  return;
}

