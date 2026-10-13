// ADDRESS: 0x14093c200
// SIZE: 900
// SIGNATURE: void __cdecl add(void)

void __cdecl country_core::asset_libraries::texture_library::AssetTextureLibrary::add(void)

{
  char *pcVar1;
  longlong *plVar2;
  ulonglong uVar3;
  longlong lVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
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
  char cVar25;
  char extraout_AL;
  char extraout_AL_00;
  ushort uVar26;
  ulonglong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  longlong *in_RDX;
  uint uVar27;
  longlong lVar28;
  longlong lVar29;
  ulonglong uVar30;
  byte bVar31;
  undefined1 auVar32 [16];
  char cVar33;
  char cVar34;
  char cVar35;
  char cVar36;
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
  undefined1 auVar37 [16];
  longlong *local_158;
  code *local_150;
  longlong *local_148;
  code *local_140;
  undefined **local_138;
  undefined *puStack_130;
  longlong **local_128;
  longlong lStack_120;
  longlong local_118;
  longlong lStack_110;
  longlong local_108;
  undefined *local_100;
  undefined8 local_f8;
  undefined4 local_f0;
  undefined4 local_ec;
  undefined **local_e8;
  undefined8 local_e0;
  longlong **local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  longlong local_b0;
  longlong local_a8;
  longlong *local_a0;
  longlong local_98;
  longlong local_90;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  local_a0 = in_RDX;
  lVar4 = local_90;
  if (!SBORROW8(0,*in_RDX)) {
    local_81 = 0;
    local_82 = 1;
    ::utils::name_hash::NameHash::compute();
    lVar4 = local_90;
    if (in_RCX[9] != 0) {
      lVar28 = in_RCX[6];
      uVar3 = in_RCX[7];
      bVar31 = (byte)(extraout_RAX >> 0x39);
      auVar32 = ZEXT216(CONCAT11(bVar31,bVar31));
      auVar32 = pshuflw(auVar32,auVar32,0);
      cVar33 = auVar32[0];
      cVar34 = auVar32[1];
      cVar35 = auVar32[2];
      cVar36 = auVar32[3];
      lVar29 = 0;
      iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                               CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                     CONCAT13(unaff_XMM7_Bd,
                              CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
      iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                               CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                     CONCAT13(unaff_XMM7_Bh,
                              CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
      iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                               CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                     CONCAT13(unaff_XMM7_Bl,
                              CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
      iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                               CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                     CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
      uVar30 = extraout_RAX;
      while( true ) {
        pcVar1 = (char *)(lVar28 + (uVar3 & uVar30));
        cVar10 = *pcVar1;
        cVar11 = pcVar1[1];
        cVar12 = pcVar1[2];
        cVar13 = pcVar1[3];
        cVar14 = pcVar1[4];
        cVar15 = pcVar1[5];
        cVar16 = pcVar1[6];
        cVar17 = pcVar1[7];
        cVar18 = pcVar1[8];
        cVar19 = pcVar1[9];
        cVar20 = pcVar1[10];
        cVar21 = pcVar1[0xb];
        cVar22 = pcVar1[0xc];
        cVar23 = pcVar1[0xd];
        cVar24 = pcVar1[0xe];
        cVar25 = pcVar1[0xf];
        auVar32[0] = -(cVar10 == cVar33);
        auVar32[1] = -(cVar11 == cVar34);
        auVar32[2] = -(cVar12 == cVar35);
        auVar32[3] = -(cVar13 == cVar36);
        auVar32[4] = -(cVar14 == cVar33);
        auVar32[5] = -(cVar15 == cVar34);
        auVar32[6] = -(cVar16 == cVar35);
        auVar32[7] = -(cVar17 == cVar36);
        auVar32[8] = -(cVar18 == cVar33);
        auVar32[9] = -(cVar19 == cVar34);
        auVar32[10] = -(cVar20 == cVar35);
        auVar32[0xb] = -(cVar21 == cVar36);
        auVar32[0xc] = -(cVar22 == cVar33);
        auVar32[0xd] = -(cVar23 == cVar34);
        auVar32[0xe] = -(cVar24 == cVar35);
        auVar32[0xf] = -(cVar25 == cVar36);
        uVar26 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe | (ushort)(auVar32[0xf] >> 7) << 0xf
        ;
        uVar27 = (uint)uVar26;
        lVar4 = local_90;
        while (local_90 = lVar29, uVar5 = uVar27, uVar26 != 0) {
          for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          }
          local_81 = 0;
          local_82 = 1;
          local_98 = lVar28;
          ::utils::name_hash::impl$2::eq();
          if (extraout_AL != '\0') {
            local_158 = &local_b0;
            local_150 = core::fmt::impl$74::fmt<alloc::string::String>;
            local_138 = &PTR_s_AssetTextureLibrary__the_name_is_142ab1b60;
            puStack_130 = (undefined *)0x2;
            local_118 = 0;
            local_128 = &local_158;
            lStack_120 = 1;
            local_81 = 0;
            local_82 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          uVar26 = (ushort)(uVar27 - 1) & (ushort)uVar27;
          uVar27 = CONCAT22((short)(uVar27 - 1 >> 0x10),uVar26);
          lVar28 = local_98;
          lVar29 = local_90;
          lVar4 = local_90;
        }
        auVar37[0] = -(cVar10 == (char)iVar6);
        auVar37[1] = -(cVar11 == (char)((uint)iVar6 >> 8));
        auVar37[2] = -(cVar12 == (char)((uint)iVar6 >> 0x10));
        auVar37[3] = -(cVar13 == (char)((uint)iVar6 >> 0x18));
        auVar37[4] = -(cVar14 == (char)iVar7);
        auVar37[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
        auVar37[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
        auVar37[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
        auVar37[8] = -(cVar18 == (char)iVar8);
        auVar37[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
        auVar37[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
        auVar37[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
        auVar37[0xc] = -(cVar22 == (char)iVar9);
        auVar37[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
        auVar37[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
        auVar37[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
        if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar37 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar37 >> 0x1f,0) & 1) != 0) || (SUB161(auVar37 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar37 >> 0x37,0) & 1) != 0) || (SUB161(auVar37 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar37 >> 0x47,0) & 1) != 0) || (SUB161(auVar37 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar37 >> 0x57,0) & 1) != 0) || (SUB161(auVar37 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar37 >> 0x67,0) & 1) != 0) || (SUB161(auVar37 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar37 >> 0x77,0) & 1) != 0) || auVar37[0xf] < '\0') break;
        uVar30 = (uVar3 & uVar30) + local_90 + 0x10;
        lVar29 = local_90 + 0x10;
        local_90 = lVar4;
      }
    }
  }
  local_90 = lVar4;
  local_108 = local_a0[9];
  local_138 = (undefined **)local_a0[3];
  puStack_130 = (undefined *)local_a0[4];
  local_128 = (longlong **)local_a0[5];
  lStack_120 = local_a0[6];
  local_118 = local_a0[7];
  lStack_110 = local_a0[8];
  lVar4 = in_RCX[2];
  if (lVar4 == *in_RCX) {
    alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
  }
  lVar28 = in_RCX[1];
  lVar29 = lVar4 * 0x38;
  *(longlong *)(lVar28 + 0x30 + lVar29) = local_108;
  plVar2 = (longlong *)(lVar28 + 0x20 + lVar29);
  *plVar2 = local_118;
  plVar2[1] = lStack_110;
  plVar2 = (longlong *)(lVar28 + 0x10 + lVar29);
  *plVar2 = (longlong)local_128;
  plVar2[1] = lStack_120;
  *(longlong *)(lVar28 + lVar29) = (longlong)local_138;
  ((longlong *)(lVar28 + lVar29))[1] = (longlong)puStack_130;
  in_RCX[2] = lVar4 + 1;
  local_a8 = lVar4;
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_158 = local_a0;
    local_150 = core::option::impl$46::fmt<alloc::string::String>;
    local_148 = &local_a8;
    local_140 = asset_libraries::impl$11::fmt<rhapsody::raw::texture::Texture>;
    local_81 = 0;
    local_82 = 0;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_118 = *extraout_RAX_00;
    lStack_110 = extraout_RAX_00[1];
    local_ec = (undefined4)extraout_RAX_00[2];
    local_108 = 5;
    local_100 = &DAT_142ab1be8;
    local_f8 = 0x2e;
    local_e8 = &PTR_s_Loaded_texture_as_142ab1bb0;
    local_e0 = 2;
    local_d8 = &local_158;
    local_d0 = 2;
    local_c8 = 0;
    local_138 = (undefined **)0x0;
    puStack_130 = &DAT_142ab1be8;
    local_128 = (longlong **)0x2e;
    lStack_120 = 0;
    local_f0 = 1;
    local_81 = 0;
    local_82 = 0;
    ::log::__private_api::impl$2::log();
  }
  local_98 = *local_a0;
  if (!SBORROW8(0,local_98)) {
    local_90 = local_a0[1];
    ::utils::name_hash::NameHash::compute();
    cVar33 = extraout_AL_00;
    hashbrown::map::HashMap::
    insert<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash>_>,alloc::alloc::Global>
              ();
    if (cVar33 != '\x02') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (local_98 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

