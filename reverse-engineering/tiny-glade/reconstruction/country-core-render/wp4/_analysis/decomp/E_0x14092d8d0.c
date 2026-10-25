// ADDRESS: 0x14092d8d0
// SIZE: 1183
// SIGNATURE: void __cdecl reload_mesh_src(void)

void __cdecl country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src(void)

{
  ulonglong *puVar1;
  char *pcVar2;
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
  ushort uVar26;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong in_RCX;
  longlong lVar27;
  ulonglong uVar28;
  uint uVar29;
  byte bVar30;
  undefined1 auVar31 [16];
  char cVar32;
  char cVar33;
  char cVar34;
  char cVar35;
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
  undefined1 auVar36 [16];
  longlong local_1d0;
  longlong lStack_1c8;
  longlong local_1c0;
  longlong lStack_1b8;
  longlong local_1b0;
  longlong lStack_1a8;
  longlong local_1a0;
  longlong lStack_198;
  longlong local_190;
  longlong lStack_188;
  longlong local_180;
  longlong lStack_178;
  longlong local_170;
  longlong lStack_168;
  longlong local_160;
  int local_158;
  int *local_150;
  code *local_148;
  undefined1 local_b8 [16];
  int local_a8 [2];
  int *local_a0;
  code *local_98;
  ulonglong *local_90;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  puVar1 = (ulonglong *)(in_RCX + 0x98);
  uVar3 = *(ulonglong *)(in_RCX + 0x98);
  if (uVar3 < 0xfffffffffffffff0 && (char)(((uint)uVar3 & 8) >> 3) == '\0') {
    LOCK();
    uVar28 = *puVar1;
    if (uVar3 == uVar28) {
      *puVar1 = uVar3 + 0x10;
    }
    UNLOCK();
    if (uVar3 == uVar28) goto LAB_14092d95e;
  }
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_14092d95e:
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_90 = puVar1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (*(longlong *)(in_RCX + 0xb8) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<std::path::Path>_>();
    lVar4 = *(longlong *)(in_RCX + 0xa0);
    uVar3 = *(ulonglong *)(in_RCX + 0xa8);
    bVar30 = (byte)(extraout_RAX >> 0x39);
    auVar31 = ZEXT216(CONCAT11(bVar30,bVar30));
    auVar31 = pshuflw(auVar31,auVar31,0);
    cVar32 = auVar31[0];
    cVar33 = auVar31[1];
    cVar34 = auVar31[2];
    cVar35 = auVar31[3];
    lVar27 = 0;
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
    uVar28 = extraout_RAX;
    while( true ) {
      uVar28 = uVar3 & uVar28;
      pcVar2 = (char *)(lVar4 + uVar28);
      cVar10 = *pcVar2;
      cVar11 = pcVar2[1];
      cVar12 = pcVar2[2];
      cVar13 = pcVar2[3];
      cVar14 = pcVar2[4];
      cVar15 = pcVar2[5];
      cVar16 = pcVar2[6];
      cVar17 = pcVar2[7];
      cVar18 = pcVar2[8];
      cVar19 = pcVar2[9];
      cVar20 = pcVar2[10];
      cVar21 = pcVar2[0xb];
      cVar22 = pcVar2[0xc];
      cVar23 = pcVar2[0xd];
      cVar24 = pcVar2[0xe];
      cVar25 = pcVar2[0xf];
      auVar31[0] = -(cVar10 == cVar32);
      auVar31[1] = -(cVar11 == cVar33);
      auVar31[2] = -(cVar12 == cVar34);
      auVar31[3] = -(cVar13 == cVar35);
      auVar31[4] = -(cVar14 == cVar32);
      auVar31[5] = -(cVar15 == cVar33);
      auVar31[6] = -(cVar16 == cVar34);
      auVar31[7] = -(cVar17 == cVar35);
      auVar31[8] = -(cVar18 == cVar32);
      auVar31[9] = -(cVar19 == cVar33);
      auVar31[10] = -(cVar20 == cVar34);
      auVar31[0xb] = -(cVar21 == cVar35);
      auVar31[0xc] = -(cVar22 == cVar32);
      auVar31[0xd] = -(cVar23 == cVar33);
      auVar31[0xe] = -(cVar24 == cVar34);
      auVar31[0xf] = -(cVar25 == cVar35);
      uVar26 = (ushort)(SUB161(auVar31 >> 7,0) & 1) | (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe | (ushort)(auVar31[0xf] >> 7) << 0xf;
      uVar29 = (uint)uVar26;
      while (uVar5 = uVar29, uVar26 != 0) {
        for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        }
        hashbrown::impl$0::equivalent<std::path::Path,std::path::PathBuf>();
        puVar1 = local_90;
        if (extraout_AL != '\0') {
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          uVar3 = *puVar1;
          *puVar1 = *puVar1 - 0x10;
          UNLOCK();
          if ((uVar3 & 0xfffffffffffffff2) == 0x12) {
            parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
          }
          std::path::Path::extension();
          if ((extraout_RAX_00 == 0) || (std::ffi::os_str::impl$41::try_from(), local_158 == 1)) {
            local_a0 = (int *)0x0;
          }
          else {
            local_a0 = local_150;
            local_98 = local_148;
            if ((local_148 == (code *)0x4) && (*local_150 == 0x6e6f736a)) {
              get_mut();
              if (extraout_RAX_01 != (longlong *)0x0) {
                country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
                if (SBORROW8(0,local_1d0)) {
                  if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
                    local_a0 = local_a8;
                    local_98 = anyhow::error::impl$4::fmt;
                    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                    ::log::__private_api::impl$2::log();
                  }
                  anyhow::error::impl$6::drop();
                }
                else {
                  local_90 = (ulonglong *)extraout_RAX_01;
                  core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
                  local_90[0xe] = local_160;
                  local_90[0xc] = local_170;
                  local_90[0xd] = lStack_168;
                  local_90[10] = local_180;
                  local_90[0xb] = lStack_178;
                  local_90[8] = local_190;
                  local_90[9] = lStack_188;
                  local_90[6] = local_1a0;
                  local_90[7] = lStack_198;
                  local_90[4] = local_1b0;
                  local_90[5] = lStack_1a8;
                  local_90[2] = local_1c0;
                  local_90[3] = lStack_1b8;
                  *local_90 = local_1d0;
                  local_90[1] = lStack_1c8;
                }
                return;
              }
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
          }
                    // WARNING: Subroutine does not return
          core::panicking::
          assert_failed<enum2_<core::option::Option<ref_<str_>_>_>,enum2_<core::option::Option<ref_<str_>_>_>_>
                    ();
        }
        uVar26 = (ushort)(uVar29 - 1) & (ushort)uVar29;
        uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar26);
      }
      auVar36[0] = -(cVar10 == (char)iVar6);
      auVar36[1] = -(cVar11 == (char)((uint)iVar6 >> 8));
      auVar36[2] = -(cVar12 == (char)((uint)iVar6 >> 0x10));
      auVar36[3] = -(cVar13 == (char)((uint)iVar6 >> 0x18));
      auVar36[4] = -(cVar14 == (char)iVar7);
      auVar36[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
      auVar36[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
      auVar36[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
      auVar36[8] = -(cVar18 == (char)iVar8);
      auVar36[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
      auVar36[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
      auVar36[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
      auVar36[0xc] = -(cVar22 == (char)iVar9);
      auVar36[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
      auVar36[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
      auVar36[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar36 >> 7,0) & 1) != 0 || (SUB161(auVar36 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar36 >> 0x17,0) & 1) != 0) || (SUB161(auVar36 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar36 >> 0x27,0) & 1) != 0) || (SUB161(auVar36 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar36 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar36 >> 0x3f,0) & 1) != 0) || (SUB161(auVar36 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar36 >> 0x4f,0) & 1) != 0) || (SUB161(auVar36 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar36 >> 0x5f,0) & 1) != 0) || (SUB161(auVar36 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar36 >> 0x6f,0) & 1) != 0) || (SUB161(auVar36 >> 0x77,0) & 1) != 0) ||
          auVar36[0xf] < '\0') break;
      uVar28 = uVar28 + lVar27 + 0x10;
      lVar27 = lVar27 + 0x10;
    }
  }
  local_a0 = (int *)local_b8;
  local_98 = core::fmt::impl$74::fmt<std::ffi::os_str::OsStr>;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

