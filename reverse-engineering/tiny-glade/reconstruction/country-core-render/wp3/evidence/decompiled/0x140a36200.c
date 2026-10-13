// address 0x140a36200
// name add
// signature void __cdecl add(void)

void __cdecl country_core::render::nani::mesh_atlas::MeshAtlas::add(void)

{
  char *pcVar1;
  int *piVar2;
  int *piVar3;
  longlong *plVar4;
  size_t sVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
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
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  char cVar29;
  char cVar30;
  undefined8 uVar31;
  int iVar32;
  uint uVar33;
  int extraout_EAX;
  ulonglong extraout_RAX;
  longlong lVar34;
  ulonglong extraout_RAX_00;
  ulonglong uVar35;
  void *_Dst;
  void *extraout_RAX_01;
  undefined8 extraout_RAX_02;
  ulonglong uVar36;
  ulonglong extraout_RAX_03;
  uint uVar37;
  undefined4 *in_RCX;
  ulonglong uVar38;
  undefined8 *puVar39;
  ulonglong *in_RDX;
  ulonglong uVar40;
  void *pvVar41;
  undefined1 (*pauVar42) [16];
  ushort uVar43;
  longlong lVar44;
  ulonglong uVar45;
  longlong in_R9;
  ulonglong uVar46;
  longlong lVar47;
  undefined4 uVar48;
  undefined1 (*pauVar49) [16];
  ulonglong uVar50;
  bool bVar51;
  byte bVar52;
  char cVar53;
  char cVar55;
  char cVar56;
  char cVar57;
  undefined1 auVar54 [16];
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  float fVar60;
  float fVar61;
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  float fVar64;
  undefined1 auVar65 [16];
  longlong local_178;
  longlong local_170;
  ulonglong local_168;
  longlong local_160;
  longlong local_158;
  ulonglong local_150;
  undefined1 (*local_148) [16];
  undefined1 (*pauStack_140) [16];
  longlong *local_138;
  code *pcStack_130;
  ulonglong local_128;
  ulonglong uStack_120;
  undefined1 local_118 [8];
  ulonglong local_110;
  ulonglong local_108;
  undefined4 *local_100;
  ulonglong *local_f8;
  ulonglong local_f0;
  void *local_e8;
  void *local_e0;
  undefined1 (*local_d8) [16];
  undefined1 (*pauStack_d0) [16];
  undefined1 (**local_c8) [16];
  undefined1 auStack_c0 [16];
  ulonglong uStack_b0;
  undefined1 (*local_a8) [16];
  ulonglong local_a0;
  uint local_94;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  uVar50 = *(ulonglong *)(in_R9 + 0x10);
  if (uVar50 == 0) {
    local_148 = (undefined1 (*) [16])local_118;
    pauStack_140 = (undefined1 (*) [16])resources::render::mesh_atlas_library::impl$10::fmt;
    local_d8 = (undefined1 (*) [16])&PTR_s_invalid_mesh__142acc948;
    pauStack_d0 = (undefined1 (*) [16])0x1;
    local_c8 = &local_148;
    auStack_c0 = ZEXT816(1);
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  local_a0 = *(ulonglong *)(in_R9 + 0x50);
  fVar64 = 0.0;
  local_100 = in_RCX;
  if (local_a0 != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar47 = *(longlong *)(in_R9 + 0x38);
    uVar36 = *(ulonglong *)(in_R9 + 0x40);
    bVar52 = (byte)(extraout_RAX >> 0x39);
    auVar54 = ZEXT216(CONCAT11(bVar52,bVar52));
    auVar54 = pshuflw(auVar54,auVar54,0);
    lVar34 = 0;
    uVar38 = extraout_RAX;
    while( true ) {
      uVar38 = uVar36 & uVar38;
      pcVar1 = (char *)(lVar47 + uVar38);
      cVar53 = auVar54[0];
      auVar62[0] = -(*pcVar1 == cVar53);
      cVar55 = auVar54[1];
      auVar62[1] = -(pcVar1[1] == cVar55);
      cVar56 = auVar54[2];
      auVar62[2] = -(pcVar1[2] == cVar56);
      cVar57 = auVar54[3];
      auVar62[3] = -(pcVar1[3] == cVar57);
      auVar62[4] = -(pcVar1[4] == cVar53);
      auVar62[5] = -(pcVar1[5] == cVar55);
      auVar62[6] = -(pcVar1[6] == cVar56);
      auVar62[7] = -(pcVar1[7] == cVar57);
      auVar62[8] = -(pcVar1[8] == cVar53);
      auVar62[9] = -(pcVar1[9] == cVar55);
      auVar62[10] = -(pcVar1[10] == cVar56);
      auVar62[0xb] = -(pcVar1[0xb] == cVar57);
      auVar62[0xc] = -(pcVar1[0xc] == cVar53);
      auVar62[0xd] = -(pcVar1[0xd] == cVar55);
      auVar62[0xe] = -(pcVar1[0xe] == cVar56);
      auVar62[0xf] = -(pcVar1[0xf] == cVar57);
      uVar43 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(auVar62[0xf] >> 7) << 0xf;
      uVar33 = (uint)uVar43;
      while (uVar43 != 0) {
        uVar37 = 0;
        for (uVar6 = uVar33; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar37 = uVar37 + 1;
        }
        lVar44 = (uVar37 + uVar38 & uVar36) * -0x38;
        if ((*(longlong *)(lVar47 + -0x28 + lVar44) == 0xf) &&
           (plVar4 = *(longlong **)(lVar44 + lVar47 + -0x38 + 8),
           *(longlong *)((longlong)plVar4 + 7) == 0x6e6f697469736f50 &&
           *plVar4 == 0x505f786574726556)) {
          core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
          bVar52 = (byte)(extraout_RAX_00 >> 0x39);
          auVar54 = ZEXT216(CONCAT11(bVar52,bVar52));
          auVar54 = pshuflw(auVar54,auVar54,0);
          lVar34 = 0;
          uVar38 = extraout_RAX_00;
          goto LAB_140a36397;
        }
        uVar43 = (ushort)(uVar33 - 1) & (ushort)uVar33;
        uVar33 = CONCAT22((short)(uVar33 - 1 >> 0x10),uVar43);
      }
      auVar58[0] = -(*pcVar1 == -1);
      auVar58[1] = -(pcVar1[1] == -1);
      auVar58[2] = -(pcVar1[2] == -1);
      auVar58[3] = -(pcVar1[3] == -1);
      auVar58[4] = -(pcVar1[4] == -1);
      auVar58[5] = -(pcVar1[5] == -1);
      auVar58[6] = -(pcVar1[6] == -1);
      auVar58[7] = -(pcVar1[7] == -1);
      auVar58[8] = -(pcVar1[8] == -1);
      auVar58[9] = -(pcVar1[9] == -1);
      auVar58[10] = -(pcVar1[10] == -1);
      auVar58[0xb] = -(pcVar1[0xb] == -1);
      auVar58[0xc] = -(pcVar1[0xc] == -1);
      auVar58[0xd] = -(pcVar1[0xd] == -1);
      auVar58[0xe] = -(pcVar1[0xe] == -1);
      auVar58[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar58 >> 7,0) & 1) != 0 || (SUB161(auVar58 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar58 >> 0x17,0) & 1) != 0) || (SUB161(auVar58 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar58 >> 0x27,0) & 1) != 0) || (SUB161(auVar58 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar58 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar58 >> 0x3f,0) & 1) != 0) || (SUB161(auVar58 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar58 >> 0x4f,0) & 1) != 0) || (SUB161(auVar58 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar58 >> 0x5f,0) & 1) != 0) || (SUB161(auVar58 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar58 >> 0x6f,0) & 1) != 0) || (SUB161(auVar58 >> 0x77,0) & 1) != 0) ||
          auVar58[0xf] < '\0') break;
      uVar38 = uVar38 + lVar34 + 0x10;
      lVar34 = lVar34 + 0x10;
    }
  }
LAB_140a364aa:
  local_110 = in_RDX[0x15];
  if (0x3f < local_110) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RDX[10] == 0) {
    iVar32 = 0;
  }
  else {
    pauVar42 = (undefined1 (*) [16])in_RDX[7];
    auVar54 = *pauVar42;
    uVar43 = (ushort)(SUB161(auVar54 >> 7,0) & 1) | (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar54[0xf] >> 7) << 0xf;
    pauVar49 = pauVar42;
    while (uVar43 == 0xffff) {
      pauVar49 = pauVar49 + 1;
      auVar54 = *pauVar49;
      pauVar42 = pauVar42 + -0x38;
      uVar43 = (ushort)(SUB161(auVar54 >> 7,0) & 1) | (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar54[0xf] >> 7) << 0xf;
    }
    uVar33 = 0;
    for (uVar37 = ~(uint)uVar43; (uVar37 & 1) == 0; uVar37 = uVar37 >> 1 | 0x80000000) {
      uVar33 = uVar33 + 1;
    }
    iVar32 = (int)*(undefined8 *)((longlong)pauVar42 + (ulonglong)uVar33 * -0x38 + -8);
  }
  uVar36 = in_RDX[2];
  local_f8 = in_RDX;
  local_108 = uVar50;
  if ((int)uVar36 == 0) {
    uVar31 = *(undefined8 *)(in_R9 + 0x58);
    uVar38 = *(ulonglong *)(in_R9 + 0x60);
    hashbrown::raw::impl$13::
    clone<tuple_<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>_>,alloc::alloc::Global>
              ();
    auStack_c0._8_8_ = uVar31;
    pvVar41 = *(void **)(in_R9 + 8);
    sVar5 = uVar50 * 4;
    uStack_b0 = uVar38;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    memcpy(_Dst,pvVar41,sVar5);
    uVar50 = local_110;
    local_e0 = _Dst;
    if (SBORROW8(0,*(longlong *)(in_R9 + 0x18))) {
      local_a0 = 0x8000000000000000;
    }
    else {
      local_a0 = *(size_t *)(in_R9 + 0x28);
      local_a8 = (undefined1 (*) [16])sVar5;
      if ((longlong)local_a0 < 0) {
LAB_140a36eb0:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      pvVar41 = *(void **)(in_R9 + 0x20);
      if (local_a0 == 0) {
        local_e8 = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_e8 = extraout_RAX_01;
        if (extraout_RAX_01 == (void *)0x0) goto LAB_140a36eb0;
      }
      memcpy(local_e8,pvVar41,local_a0);
      local_94 = CONCAT31(local_94._1_3_,*(undefined1 *)(in_R9 + 0x30));
    }
    uVar48 = (undefined4)uVar50;
    local_f0 = *(ulonglong *)(in_R9 + 0x68);
    local_a8 = *(undefined1 (**) [16])(in_R9 + 0x70);
    local_128 = auStack_c0._8_8_;
    uStack_120 = uStack_b0;
    local_138 = (longlong *)local_c8;
    pcStack_130 = (code *)auStack_c0._0_8_;
    local_148 = local_d8;
    pauStack_140 = pauStack_d0;
    core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
    *local_f8 = local_108;
    local_f8[1] = (ulonglong)local_e0;
    local_f8[2] = local_108;
    local_f8[3] = local_a0;
    local_f8[4] = (ulonglong)local_e8;
    local_f8[5] = local_a0;
    *(undefined1 *)(local_f8 + 6) = (undefined1)local_94;
    local_f8[7] = (ulonglong)local_148;
    local_f8[8] = (ulonglong)pauStack_140;
    local_f8[9] = (ulonglong)local_138;
    local_f8[10] = (ulonglong)pcStack_130;
    local_f8[0xb] = local_128;
    local_f8[0xc] = uStack_120;
    local_f8[0xd] = local_f0;
    local_f8[0xe] = (ulonglong)local_a8;
    uVar50 = local_108;
LAB_140a36c00:
    uVar38 = local_f8[0x11];
    if (uVar38 == local_f8[0xf]) {
      alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
    }
    uVar35 = local_f8[0x10];
    lVar47 = uVar38 * 0x10;
    *(int *)(uVar35 + lVar47) = (int)uVar36;
    *(int *)(uVar35 + 4 + lVar47) = (int)uVar50;
    *(float *)(uVar35 + 8 + lVar47) = fVar64;
    *(undefined4 *)(uVar35 + 0xc + lVar47) = 0;
    local_f8[0x11] = uVar38 + 1;
    hashbrown::map::HashMap::
    insert<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName>_>,alloc::alloc::Global>
              ();
    if (extraout_EAX == 0) {
      local_100[1] = uVar48;
      *local_100 = 0;
      return;
    }
    local_148 = (undefined1 (*) [16])local_118;
    pauStack_140 = (undefined1 (*) [16])resources::render::mesh_atlas_library::impl$10::fmt;
    local_d8 = (undefined1 (*) [16])&PTR_s_Duplicate_atlas_mesh_name__142acca60;
    pauStack_d0 = (undefined1 (*) [16])0x1;
    local_c8 = &local_148;
    auStack_c0 = ZEXT816(1);
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  lVar47 = *(longlong *)(in_R9 + 8);
  if (*in_RDX - uVar36 < uVar50) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    uVar35 = in_RDX[2];
    uVar38 = in_RDX[1];
    if (7 < uVar50) goto LAB_140a36549;
LAB_140a3655a:
    uVar40 = 0;
  }
  else {
    uVar38 = in_RDX[1];
    uVar35 = uVar36;
    if (uVar50 < 8) goto LAB_140a3655a;
LAB_140a36549:
    if ((uVar38 + uVar35 * 4) - lVar47 < 0x20) goto LAB_140a3655a;
    lVar34 = uVar35 * 4;
    uVar40 = uVar50 & 0x1ffffffffffffff8;
    uVar35 = uVar35 + uVar40;
    lVar34 = lVar34 + uVar38;
    uVar46 = 0;
    do {
      piVar2 = (int *)(lVar47 + uVar46);
      iVar8 = piVar2[1];
      iVar9 = piVar2[2];
      iVar10 = piVar2[3];
      piVar3 = (int *)(lVar47 + 0x10 + uVar46);
      iVar11 = *piVar3;
      iVar12 = piVar3[1];
      iVar13 = piVar3[2];
      iVar14 = piVar3[3];
      piVar3 = (int *)(lVar34 + uVar46);
      *piVar3 = *piVar2 + iVar32;
      piVar3[1] = iVar8 + iVar32;
      piVar3[2] = iVar9 + iVar32;
      piVar3[3] = iVar10 + iVar32;
      piVar2 = (int *)(lVar34 + 0x10 + uVar46);
      *piVar2 = iVar11 + iVar32;
      piVar2[1] = iVar12 + iVar32;
      piVar2[2] = iVar13 + iVar32;
      piVar2[3] = iVar14 + iVar32;
      uVar46 = uVar46 + 0x20;
    } while ((uVar50 * 4 & 0xffffffffffffffe0) != uVar46);
    if (uVar50 == uVar40) goto LAB_140a36744;
  }
  uVar45 = uVar40;
  for (uVar46 = uVar50 & 3; uVar46 != 0; uVar46 = uVar46 - 1) {
    *(int *)(uVar38 + uVar35 * 4) = *(int *)(lVar47 + uVar45 * 4) + iVar32;
    uVar35 = uVar35 + 1;
    uVar45 = uVar45 + 1;
  }
  if (uVar40 - uVar50 < 0xfffffffffffffffd) {
    lVar34 = uVar38 + uVar35 * 4;
    lVar47 = lVar47 + uVar45 * 4;
    lVar44 = 0;
    do {
      *(int *)(lVar34 + lVar44 * 4) = *(int *)(lVar47 + lVar44 * 4) + iVar32;
      *(int *)(lVar34 + 4 + lVar44 * 4) = *(int *)(lVar47 + 4 + lVar44 * 4) + iVar32;
      *(int *)(lVar34 + 8 + lVar44 * 4) = *(int *)(lVar47 + 8 + lVar44 * 4) + iVar32;
      *(int *)(lVar34 + 0xc + lVar44 * 4) = *(int *)(lVar47 + 0xc + lVar44 * 4) + iVar32;
      lVar44 = lVar44 + 4;
    } while (uVar50 - uVar45 != lVar44);
    uVar35 = uVar35 + lVar44;
  }
LAB_140a36744:
  in_RDX[2] = uVar35;
  local_d8 = (undefined1 (*) [16])in_RDX[7];
  local_c8 = (undefined1 (**) [16])(*local_d8 + in_RDX[8] + 1);
  auVar54 = *local_d8;
  auStack_c0._0_2_ =
       ~((ushort)(SUB161(auVar54 >> 7,0) & 1) | (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
         (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 | (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3
         | (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
         (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 | (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6
         | (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
         (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 | (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9
         | (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
         (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
         (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
         (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
         (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar54[0xf] >> 7) << 0xf);
  pauStack_d0 = local_d8 + 1;
  auStack_c0._8_8_ = in_RDX[10];
  local_f0 = uVar36;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<alloc::string::String>,std::collections::hash::map::Keys<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>_>_>
            ();
  local_d8 = *(undefined1 (**) [16])(in_R9 + 0x38);
  local_e0 = *(void **)(in_R9 + 0x40);
  local_c8 = (undefined1 (**) [16])((longlong)local_e0 + (longlong)local_d8 + 1);
  auVar54 = *local_d8;
  pauStack_d0 = local_d8 + 1;
  auStack_c0._0_2_ =
       ~((ushort)(SUB161(auVar54 >> 7,0) & 1) | (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
         (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 | (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3
         | (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
         (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 | (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6
         | (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
         (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 | (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9
         | (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
         (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
         (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
         (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
         (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar54[0xf] >> 7) << 0xf);
  auStack_c0._8_8_ = local_a0;
  local_a8 = local_d8;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<alloc::string::String>,std::collections::hash::map::Keys<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>_>_>
            ();
  if (1 < local_150) {
    if (local_150 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<ref_<alloc::string::String>,bool_(*)(ref_<ref_<alloc::string::String>_>,ref_<ref_<alloc::string::String>_>)>
                ();
    }
    else {
      core::slice::sort::stable::
      driftsort_main<ref_<alloc::string::String>,bool_(*)(ref_<ref_<alloc::string::String>_>,ref_<ref_<alloc::string::String>_>),alloc::vec::Vec<ref_<alloc::string::String>,alloc::alloc::Global>_>
                ();
    }
  }
  if (1 < local_168) {
    if (local_168 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<ref_<alloc::string::String>,bool_(*)(ref_<ref_<alloc::string::String>_>,ref_<ref_<alloc::string::String>_>)>
                ();
    }
    else {
      core::slice::sort::stable::
      driftsort_main<ref_<alloc::string::String>,bool_(*)(ref_<ref_<alloc::string::String>_>,ref_<ref_<alloc::string::String>_>),alloc::vec::Vec<ref_<alloc::string::String>,alloc::alloc::Global>_>
                ();
    }
  }
  if (local_150 == local_168) {
    uVar50 = 0;
    do {
      if (local_150 == uVar50) {
        pauVar49 = (undefined1 (*) [16])local_f8[7];
        uVar50 = local_f8[10];
        auVar54 = *pauVar49;
        pauVar42 = pauVar49 + 1;
        local_e8 = local_a8[-4] + 8;
        uVar33 = ~(uint)(ushort)((ushort)(SUB161(auVar54 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar54[0xf] >> 7) << 0xf);
LAB_140a36a5b:
        uVar36 = local_f0;
        bVar51 = uVar50 != 0;
        uVar50 = uVar50 - 1;
        if (bVar51) {
          if ((short)uVar33 == 0) {
            do {
              auVar54 = *pauVar42;
              uVar43 = (ushort)(SUB161(auVar54 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar54[0xf] >> 7) << 0xf;
              pauVar49 = pauVar49 + -0x38;
              pauVar42 = pauVar42 + 1;
            } while (uVar43 == 0xffff);
            uVar33 = ~(uint)uVar43;
            local_94 = uVar33 & -(uint)uVar43 - 2;
          }
          else {
            local_94 = uVar33 - 1 & uVar33;
          }
          if (local_a0 != 0) {
            uVar37 = 0;
            for (; (uVar33 & 1) == 0; uVar33 = uVar33 >> 1 | 0x80000000) {
              uVar37 = uVar37 + 1;
            }
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<alloc::string::String>_>();
            bVar52 = (byte)(extraout_RAX_03 >> 0x39);
            auVar54 = ZEXT216(CONCAT11(bVar52,bVar52));
            auVar54 = pshuflw(auVar54,auVar54,0);
            cVar53 = auVar54[0];
            cVar55 = auVar54[1];
            cVar56 = auVar54[2];
            cVar57 = auVar54[3];
            lVar47 = 0;
            uVar36 = extraout_RAX_03;
            pvVar41 = local_e8;
            while( true ) {
              uVar36 = uVar36 & (ulonglong)local_e0;
              pcVar1 = *local_a8 + uVar36;
              cVar15 = *pcVar1;
              cVar16 = pcVar1[1];
              cVar17 = pcVar1[2];
              cVar18 = pcVar1[3];
              cVar19 = pcVar1[4];
              cVar20 = pcVar1[5];
              cVar21 = pcVar1[6];
              cVar22 = pcVar1[7];
              cVar23 = pcVar1[8];
              cVar24 = pcVar1[9];
              cVar25 = pcVar1[10];
              cVar26 = pcVar1[0xb];
              cVar27 = pcVar1[0xc];
              cVar28 = pcVar1[0xd];
              cVar29 = pcVar1[0xe];
              cVar30 = pcVar1[0xf];
              auVar54[0] = -(cVar15 == cVar53);
              auVar54[1] = -(cVar16 == cVar55);
              auVar54[2] = -(cVar17 == cVar56);
              auVar54[3] = -(cVar18 == cVar57);
              auVar54[4] = -(cVar19 == cVar53);
              auVar54[5] = -(cVar20 == cVar55);
              auVar54[6] = -(cVar21 == cVar56);
              auVar54[7] = -(cVar22 == cVar57);
              auVar54[8] = -(cVar23 == cVar53);
              auVar54[9] = -(cVar24 == cVar55);
              auVar54[10] = -(cVar25 == cVar56);
              auVar54[0xb] = -(cVar26 == cVar57);
              auVar54[0xc] = -(cVar27 == cVar53);
              auVar54[0xd] = -(cVar28 == cVar55);
              auVar54[0xe] = -(cVar29 == cVar56);
              auVar54[0xf] = -(cVar30 == cVar57);
              uVar43 = (ushort)(SUB161(auVar54 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar54[0xf] >> 7) << 0xf;
              uVar33 = (uint)uVar43;
              if (uVar43 != 0) {
                sVar5 = *(size_t *)((longlong)pauVar49 + (ulonglong)uVar37 * -0x38 + -0x28);
                do {
                  uVar6 = 0;
                  for (uVar7 = uVar33; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                    uVar6 = uVar6 + 1;
                  }
                  lVar34 = (uVar6 + uVar36 & (ulonglong)local_e0) * -0x38;
                  if ((sVar5 == *(size_t *)((longlong)pvVar41 + lVar34 + 0x10)) &&
                     (iVar32 = memcmp(*(void **)((longlong)pauVar49 +
                                                (ulonglong)uVar37 * -0x38 + -0x30),
                                      *(void **)((longlong)pvVar41 + lVar34 + 8),sVar5),
                     pvVar41 = local_e8, iVar32 == 0)) {
                    mesh::VertexAttributeValues::append();
                    uVar33 = local_94;
                    goto LAB_140a36a5b;
                  }
                  uVar43 = (ushort)(uVar33 - 1) & (ushort)uVar33;
                  uVar33 = CONCAT22((short)(uVar33 - 1 >> 0x10),uVar43);
                } while (uVar43 != 0);
              }
              auVar65[0] = -(cVar15 == -1);
              auVar65[1] = -(cVar16 == -1);
              auVar65[2] = -(cVar17 == -1);
              auVar65[3] = -(cVar18 == -1);
              auVar65[4] = -(cVar19 == -1);
              auVar65[5] = -(cVar20 == -1);
              auVar65[6] = -(cVar21 == -1);
              auVar65[7] = -(cVar22 == -1);
              auVar65[8] = -(cVar23 == -1);
              auVar65[9] = -(cVar24 == -1);
              auVar65[10] = -(cVar25 == -1);
              auVar65[0xb] = -(cVar26 == -1);
              auVar65[0xc] = -(cVar27 == -1);
              auVar65[0xd] = -(cVar28 == -1);
              auVar65[0xe] = -(cVar29 == -1);
              auVar65[0xf] = -(cVar30 == -1);
              if ((((((((((((((((SUB161(auVar65 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar65 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar65 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar65 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar65 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar65 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar65 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar65 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar65 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar65 >> 0x4f,0) & 1) != 0) || (SUB161(auVar65 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar65 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar65 >> 0x67,0) & 1) != 0) || (SUB161(auVar65 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar65 >> 0x77,0) & 1) != 0) || auVar65[0xf] < '\0') break;
              uVar36 = uVar36 + lVar47 + 0x10;
              lVar47 = lVar47 + 0x10;
            }
          }
                    // WARNING: Subroutine does not return
          core::option::expect_failed();
        }
        if (local_178 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        uVar50 = local_108;
        uVar48 = (undefined4)local_110;
        if (local_160 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        goto LAB_140a36c00;
      }
      lVar47 = *(longlong *)(local_158 + uVar50 * 8);
      lVar34 = *(longlong *)(local_170 + uVar50 * 8);
      sVar5 = *(size_t *)(lVar47 + 0x10);
      if (sVar5 != *(size_t *)(lVar34 + 0x10)) break;
      uVar50 = uVar50 + 1;
      iVar32 = memcmp(*(void **)(lVar47 + 8),*(void **)(lVar34 + 8),sVar5);
    } while (iVar32 == 0);
  }
  pauStack_140 = (undefined1 (*) [16])
                 alloc::vec::impl$27::fmt<ref_<alloc::string::String>,alloc::alloc::Global>;
  pcStack_130 = alloc::vec::impl$27::fmt<ref_<alloc::string::String>,alloc::alloc::Global>;
  local_d8 = (undefined1 (*) [16])&PTR_s_Mesh_attribute_mismatch__Existin_142accad8;
  pauStack_d0 = (undefined1 (*) [16])0x2;
  local_c8 = &local_148;
  auStack_c0 = ZEXT816(2);
  local_148 = (undefined1 (*) [16])&local_160;
  local_138 = &local_178;
  alloc::fmt::format::format_inner();
  anyhow::Error::msg<alloc::string::String>();
  *(undefined8 *)(local_100 + 2) = extraout_RAX_02;
  *local_100 = 1;
  if (local_178 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_160 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_140a36397:
  uVar38 = uVar36 & uVar38;
  pcVar1 = (char *)(lVar47 + uVar38);
  cVar53 = auVar54[0];
  auVar63[0] = -(*pcVar1 == cVar53);
  cVar55 = auVar54[1];
  auVar63[1] = -(pcVar1[1] == cVar55);
  cVar56 = auVar54[2];
  auVar63[2] = -(pcVar1[2] == cVar56);
  cVar57 = auVar54[3];
  auVar63[3] = -(pcVar1[3] == cVar57);
  auVar63[4] = -(pcVar1[4] == cVar53);
  auVar63[5] = -(pcVar1[5] == cVar55);
  auVar63[6] = -(pcVar1[6] == cVar56);
  auVar63[7] = -(pcVar1[7] == cVar57);
  auVar63[8] = -(pcVar1[8] == cVar53);
  auVar63[9] = -(pcVar1[9] == cVar55);
  auVar63[10] = -(pcVar1[10] == cVar56);
  auVar63[0xb] = -(pcVar1[0xb] == cVar57);
  auVar63[0xc] = -(pcVar1[0xc] == cVar53);
  auVar63[0xd] = -(pcVar1[0xd] == cVar55);
  auVar63[0xe] = -(pcVar1[0xe] == cVar56);
  auVar63[0xf] = -(pcVar1[0xf] == cVar57);
  uVar43 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
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
           (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(auVar63[0xf] >> 7) << 0xf;
  uVar33 = (uint)uVar43;
  if (uVar43 != 0) {
LAB_140a363ac:
    uVar37 = 0;
    for (uVar6 = uVar33; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar37 = uVar37 + 1;
    }
    lVar44 = (uVar37 + uVar38 & uVar36) * -0x38;
    if ((*(longlong *)(lVar47 + -0x28 + lVar44) != 0xf) ||
       (plVar4 = *(longlong **)(lVar47 + -0x38 + lVar44 + 8),
       *(longlong *)((longlong)plVar4 + 7) != 0x6e6f697469736f50 || *plVar4 != 0x505f786574726556))
    goto LAB_140a363e0;
    if (*(int *)(lVar47 + -0x20 + lVar44) != 3) {
      local_d8 = (undefined1 (*) [16])
                 &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_163_llvm_9402988993337875171;
      pauStack_d0 = (undefined1 (*) [16])0x1;
      local_c8 = (undefined1 (**) [16])0x8;
      auStack_c0 = (undefined1  [16])0x0;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    lVar34 = *(longlong *)(lVar47 + lVar44 + -8);
    if (lVar34 == 0) {
      fVar64 = 0.0;
    }
    else {
      puVar39 = (undefined8 *)(*(longlong *)(lVar47 + lVar44 + -0x10) + 4);
      fVar64 = 0.0;
      do {
        fVar60 = (float)*puVar39;
        fVar61 = (float)((ulonglong)*puVar39 >> 0x20);
        fVar60 = fVar61 * fVar61 +
                 *(float *)((longlong)puVar39 + -4) * *(float *)((longlong)puVar39 + -4) +
                 fVar60 * fVar60;
        fVar61 = fVar60;
        if (fVar60 <= fVar64) {
          fVar61 = fVar64;
        }
        fVar64 = (float)(~-(uint)NAN(fVar64) & (uint)fVar61 | -(uint)NAN(fVar64) & (uint)fVar60);
        puVar39 = (undefined8 *)((longlong)puVar39 + 0xc);
        lVar34 = lVar34 + -1;
      } while (lVar34 != 0);
    }
    fVar64 = SQRT(fVar64);
    goto LAB_140a364aa;
  }
LAB_140a363ed:
  auVar59[0] = -(*pcVar1 == -1);
  auVar59[1] = -(pcVar1[1] == -1);
  auVar59[2] = -(pcVar1[2] == -1);
  auVar59[3] = -(pcVar1[3] == -1);
  auVar59[4] = -(pcVar1[4] == -1);
  auVar59[5] = -(pcVar1[5] == -1);
  auVar59[6] = -(pcVar1[6] == -1);
  auVar59[7] = -(pcVar1[7] == -1);
  auVar59[8] = -(pcVar1[8] == -1);
  auVar59[9] = -(pcVar1[9] == -1);
  auVar59[10] = -(pcVar1[10] == -1);
  auVar59[0xb] = -(pcVar1[0xb] == -1);
  auVar59[0xc] = -(pcVar1[0xc] == -1);
  auVar59[0xd] = -(pcVar1[0xd] == -1);
  auVar59[0xe] = -(pcVar1[0xe] == -1);
  auVar59[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar59 >> 7,0) & 1) != 0 || (SUB161(auVar59 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar59 >> 0x17,0) & 1) != 0) || (SUB161(auVar59 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar59 >> 0x27,0) & 1) != 0) || (SUB161(auVar59 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar59 >> 0x37,0) & 1) != 0) || (SUB161(auVar59 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar59 >> 0x47,0) & 1) != 0) || (SUB161(auVar59 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar59 >> 0x57,0) & 1) != 0) || (SUB161(auVar59 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar59 >> 0x67,0) & 1) != 0) || (SUB161(auVar59 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar59 >> 0x77,0) & 1) != 0) || auVar59[0xf] < '\0') {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  uVar38 = uVar38 + lVar34 + 0x10;
  lVar34 = lVar34 + 0x10;
  goto LAB_140a36397;
LAB_140a363e0:
  uVar43 = (ushort)(uVar33 - 1) & (ushort)uVar33;
  uVar33 = CONCAT22((short)(uVar33 - 1 >> 0x10),uVar43);
  if (uVar43 == 0) goto LAB_140a363ed;
  goto LAB_140a363ac;
}

