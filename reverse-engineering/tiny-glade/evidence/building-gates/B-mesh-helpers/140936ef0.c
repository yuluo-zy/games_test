
void __cdecl country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster(void)

{
  longlong *plVar1;
  char *pcVar2;
  void *_Buf1;
  size_t sVar3;
  longlong lVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  ushort uVar27;
  uint uVar28;
  int iVar29;
  int iVar30;
  ulonglong extraout_RAX;
  ulonglong uVar31;
  void *_Dst;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RCX;
  void *in_RDX;
  ulonglong uVar32;
  longlong lVar33;
  char in_R8B;
  ulonglong uVar34;
  longlong lVar35;
  longlong lVar36;
  bool bVar37;
  byte bVar38;
  undefined1 auVar39 [16];
  char cVar40;
  char cVar41;
  char cVar42;
  char cVar43;
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  undefined1 auVar44 [16];
  undefined **local_1a8;
  undefined8 local_1a0;
  longlong *local_198;
  undefined8 local_190;
  undefined8 local_188;
  longlong local_d0;
  code *local_c8;
  ulonglong local_c0;
  ulonglong local_b8;
  ulonglong local_b0;
  longlong local_a8;
  longlong local_a0;
  longlong *local_98;
  void *local_90;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  plVar1 = (longlong *)(in_RCX + 0x80);
  LOCK();
  bVar37 = *(longlong *)(in_RCX + 0x80) == 0;
  if (bVar37) {
    *(longlong *)(in_RCX + 0x80) = 8;
  }
  UNLOCK();
  local_90 = in_RDX;
  if (!bVar37) {
    local_81 = 1;
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  local_81 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_81 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  _Buf1 = *(void **)((longlong)local_90 + 0x48);
  sVar3 = *(size_t *)((longlong)local_90 + 0x50);
  uVar31 = local_b0;
  lVar4 = local_a8;
  local_98 = plVar1;
  if ((in_R8B == '\0') && (*(longlong *)(in_RCX + 0xa0) != 0)) {
    lVar33 = (longlong)local_90 + 0x48;
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar36 = *(longlong *)(in_RCX + 0x88);
    uVar32 = *(ulonglong *)(in_RCX + 0x90);
    uVar34 = uVar32 & extraout_RAX;
    bVar38 = (byte)(extraout_RAX >> 0x39);
    auVar39 = ZEXT216(CONCAT11(bVar38,bVar38));
    auVar39 = pshuflw(auVar39,auVar39,0);
    cVar40 = auVar39[0];
    cVar41 = auVar39[1];
    cVar42 = auVar39[2];
    cVar43 = auVar39[3];
    lVar35 = 0;
    iVar30 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                              CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                    CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                   CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    while( true ) {
      pcVar2 = (char *)(lVar36 + uVar34);
      cVar9 = *pcVar2;
      cVar10 = pcVar2[1];
      cVar11 = pcVar2[2];
      cVar12 = pcVar2[3];
      cVar13 = pcVar2[4];
      cVar14 = pcVar2[5];
      cVar15 = pcVar2[6];
      cVar16 = pcVar2[7];
      cVar17 = pcVar2[8];
      cVar18 = pcVar2[9];
      cVar19 = pcVar2[10];
      cVar20 = pcVar2[0xb];
      cVar21 = pcVar2[0xc];
      cVar22 = pcVar2[0xd];
      cVar23 = pcVar2[0xe];
      cVar24 = pcVar2[0xf];
      auVar39[0] = -(cVar9 == cVar40);
      auVar39[1] = -(cVar10 == cVar41);
      auVar39[2] = -(cVar11 == cVar42);
      auVar39[3] = -(cVar12 == cVar43);
      auVar39[4] = -(cVar13 == cVar40);
      auVar39[5] = -(cVar14 == cVar41);
      auVar39[6] = -(cVar15 == cVar42);
      auVar39[7] = -(cVar16 == cVar43);
      auVar39[8] = -(cVar17 == cVar40);
      auVar39[9] = -(cVar18 == cVar41);
      auVar39[10] = -(cVar19 == cVar42);
      auVar39[0xb] = -(cVar20 == cVar43);
      auVar39[0xc] = -(cVar21 == cVar40);
      auVar39[0xd] = -(cVar22 == cVar41);
      auVar39[0xe] = -(cVar23 == cVar42);
      auVar39[0xf] = -(cVar24 == cVar43);
      uVar27 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe | (ushort)(auVar39[0xf] >> 7) << 0xf;
      uVar26 = (ulonglong)uVar27;
      uVar25 = local_c0;
      uVar31 = local_b0;
      lVar4 = local_a8;
      while (local_a8 = lVar35, local_b0 = uVar34, local_c0 = uVar26, uVar27 != 0) {
        uVar5 = 0;
        for (uVar28 = (uint)local_c0; (uVar28 & 1) == 0; uVar28 = uVar28 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar31 = uVar5 + local_b0 & uVar32;
        local_b8 = uVar32;
        local_a0 = lVar33;
        if ((*(size_t *)(lVar36 + -0x10 + uVar31 * -0x18) == sVar3) &&
           (iVar29 = memcmp(_Buf1,*(void **)(uVar31 * -0x18 + lVar36 + -0x18),sVar3), iVar29 == 0))
        {
          local_82 = 1;
          frozen::FrozenMap::
          get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>_>
                    ();
          if (extraout_RAX_01 == 0) {
            local_82 = 1;
                    /* WARNING: Subroutine does not return */
            core::option::unwrap_failed();
          }
          if ((*(size_t *)(extraout_RAX_01 + 0x10) != *(size_t *)((longlong)local_90 + 0x10)) ||
             (iVar30 = memcmp(*(void **)(extraout_RAX_01 + 8),*(void **)((longlong)local_90 + 8),
                              *(size_t *)(extraout_RAX_01 + 0x10)), iVar30 != 0))
          goto LAB_1409372dd;
          if (SBORROW8(0,*(longlong *)((longlong)local_90 + 0x30)) ||
              SBORROW8(0,*(longlong *)(extraout_RAX_01 + 0x30))) {
            if ((!SBORROW8(0,*(longlong *)(extraout_RAX_01 + 0x30))) ||
               (!SBORROW8(0,*(longlong *)((longlong)local_90 + 0x30)))) goto LAB_1409372dd;
          }
          else if ((*(size_t *)(extraout_RAX_01 + 0x40) != *(size_t *)((longlong)local_90 + 0x40))
                  || (iVar30 = memcmp(*(void **)(extraout_RAX_01 + 0x38),
                                      *(void **)((longlong)local_90 + 0x38),
                                      *(size_t *)(extraout_RAX_01 + 0x40)), iVar30 != 0))
          goto LAB_1409372dd;
          if (*(longlong *)(extraout_RAX_01 + 0x28) != *(longlong *)((longlong)local_90 + 0x28))
          goto LAB_1409372dd;
          lVar4 = *(longlong *)(extraout_RAX_01 + 0x20);
          lVar33 = *(longlong *)((longlong)local_90 + 0x20);
          lVar35 = *(longlong *)(extraout_RAX_01 + 0x28) + 1;
          lVar36 = 0x28;
          goto LAB_140937290;
        }
        iVar29 = (int)local_c0 + -1;
        uVar27 = (ushort)iVar29 & (ushort)local_c0;
        uVar26 = (ulonglong)CONCAT22((short)((uint)iVar29 >> 0x10),uVar27);
        uVar32 = local_b8;
        lVar33 = local_a0;
        uVar34 = local_b0;
        lVar35 = local_a8;
        uVar25 = local_c0;
        uVar31 = local_b0;
        lVar4 = local_a8;
      }
      auVar44[0] = -(cVar9 == (char)iVar30);
      auVar44[1] = -(cVar10 == (char)((uint)iVar30 >> 8));
      auVar44[2] = -(cVar11 == (char)((uint)iVar30 >> 0x10));
      auVar44[3] = -(cVar12 == (char)((uint)iVar30 >> 0x18));
      auVar44[4] = -(cVar13 == (char)iVar6);
      auVar44[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
      auVar44[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
      auVar44[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
      auVar44[8] = -(cVar17 == (char)iVar7);
      auVar44[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
      auVar44[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
      auVar44[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
      auVar44[0xc] = -(cVar21 == (char)iVar8);
      auVar44[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
      auVar44[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
      auVar44[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
      local_c0 = uVar25;
      if ((((((((((((((((SUB161(auVar44 >> 7,0) & 1) != 0 || (SUB161(auVar44 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar44 >> 0x17,0) & 1) != 0) || (SUB161(auVar44 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar44 >> 0x27,0) & 1) != 0) || (SUB161(auVar44 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar44 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar44 >> 0x3f,0) & 1) != 0) || (SUB161(auVar44 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar44 >> 0x4f,0) & 1) != 0) || (SUB161(auVar44 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar44 >> 0x5f,0) & 1) != 0) || (SUB161(auVar44 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar44 >> 0x6f,0) & 1) != 0) || (SUB161(auVar44 >> 0x77,0) & 1) != 0) ||
          auVar44[0xf] < '\0') break;
      lVar35 = local_a8 + 0x10;
      uVar34 = local_b0 + local_a8 + 0x10 & uVar32;
      local_b0 = uVar31;
      local_a8 = lVar4;
    }
  }
  local_a8 = lVar4;
  local_b0 = uVar31;
  LOCK();
  HANDLE_COUNT = HANDLE_COUNT + 1;
  UNLOCK();
  local_82 = 1;
  hashbrown::map::HashMap::
  insert<ref_<str_>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  memcpy(&local_1a8,local_90,0xd8);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_1a8,0xd8);
  local_82 = 0;
  frozen::FrozenMap::
  insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
            ();
  plVar1 = local_98;
  if (extraout_RAX_00 == 0) {
    local_81 = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    local_81 = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar37 = *plVar1 == 8;
    if (bVar37) {
      *plVar1 = 0;
    }
    UNLOCK();
    if (!bVar37) {
      local_81 = 0;
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
    return;
  }
  local_82 = 0;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
  while( true ) {
    iVar30 = memcmp(*(void **)(lVar4 + -8 + lVar36),*(void **)(lVar33 + -8 + lVar36),
                    *(size_t *)(lVar4 + lVar36));
    lVar36 = lVar36 + 0x30;
    if (iVar30 != 0) break;
LAB_140937290:
    plVar1 = local_98;
    lVar35 = lVar35 + -1;
    if (lVar35 == 0) {
      local_81 = 1;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      local_81 = 1;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar37 = *plVar1 == 8;
      if (bVar37) {
        *plVar1 = 0;
      }
      UNLOCK();
      if (!bVar37) {
        local_81 = 1;
        parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
      }
      core::ptr::drop_in_place<country_core::render::shader::RasterShaderProgram>();
      return;
    }
    sVar3 = *(size_t *)(lVar4 + -0x18 + lVar36);
    if (((sVar3 != *(size_t *)(lVar33 + -0x18 + lVar36)) ||
        (iVar30 = memcmp(*(void **)(lVar4 + -0x20 + lVar36),*(void **)(lVar33 + -0x20 + lVar36),
                         sVar3), iVar30 != 0)) ||
       (*(size_t *)(lVar4 + lVar36) != *(size_t *)(lVar33 + lVar36))) break;
  }
LAB_1409372dd:
  local_d0 = local_a0;
  local_c8 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1a8 = &PTR_s_AssetShaderLibrary__the_shader_n_142ab10d0;
  local_1a0 = 2;
  local_188 = 0;
  local_198 = &local_d0;
  local_190 = 1;
  local_82 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

