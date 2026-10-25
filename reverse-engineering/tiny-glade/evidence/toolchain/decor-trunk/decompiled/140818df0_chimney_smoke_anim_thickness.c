// ADDR 0x140818df0
// SIG void __cdecl chimney_smoke_anim_thickness(void)
// ORIG system::chimney_smoke_anim_thickness


// WARNING: Removing unreachable block (ram,0x00014081985a)

void __cdecl system_decorator::chimney_smoke_anim_thickness::chimney_smoke_anim_thickness(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  char cVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  undefined4 *puVar9;
  undefined8 uVar10;
  undefined1 (*pauVar11) [16];
  longlong lVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  char extraout_AL;
  uint uVar18;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong lVar19;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong lVar20;
  longlong *plVar21;
  longlong *in_RDX;
  ulonglong uVar22;
  undefined8 *puVar23;
  ulonglong uVar24;
  uint *puVar25;
  ushort uVar26;
  uint uVar27;
  longlong *in_R8;
  undefined *puVar28;
  ulonglong uVar29;
  longlong *in_R9;
  longlong lVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  uint *puVar33;
  byte bVar34;
  longlong lVar35;
  longlong *plVar36;
  longlong *plVar37;
  byte bVar38;
  char cVar39;
  char cVar44;
  undefined4 uVar40;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  char cVar42;
  char cVar43;
  undefined1 auVar41 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 uVar49;
  undefined1 unaff_XMM6_Bb;
  undefined1 uVar50;
  undefined1 unaff_XMM6_Bc;
  undefined1 uVar51;
  undefined1 unaff_XMM6_Bd;
  undefined1 uVar52;
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
  float fVar53;
  undefined1 unaff_XMM11_Ba;
  undefined1 unaff_XMM11_Bb;
  undefined1 unaff_XMM11_Bc;
  undefined1 unaff_XMM11_Bd;
  undefined1 unaff_XMM11_Be;
  undefined1 unaff_XMM11_Bf;
  undefined1 unaff_XMM11_Bg;
  undefined1 unaff_XMM11_Bh;
  undefined1 unaff_XMM11_Bi;
  undefined1 unaff_XMM11_Bj;
  undefined1 unaff_XMM11_Bk;
  undefined1 unaff_XMM11_Bl;
  undefined1 unaff_XMM11_Bm;
  undefined1 unaff_XMM11_Bn;
  undefined1 unaff_XMM11_Bo;
  undefined1 unaff_XMM11_Bp;
  float fVar54;
  float fVar55;
  longlong local_130;
  longlong local_110;
  ulonglong local_108;
  longlong local_f8;
  longlong local_e8;
  
  plVar21 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar21 != (longlong *)0x0) {
        if (&DAT_0000004b < plVar21) {
          if (plVar21 != (longlong *)&DAT_0000004c) goto LAB_140818eaa;
        }
        else if ((char)plVar21[0x28552006] < -0x40) {
LAB_140818eaa:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar21 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar21 + 2) < (undefined1 *)0x4c) {
          if (*(char *)((longlong)plVar21 + 0x142a90032) < -0x40) {
LAB_140818ef7:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar21 + 2) != (undefined1 *)0x4c) goto LAB_140818ef7;
      }
    }
    puVar28 = &DAT_142a9017c;
    do {
      bVar38 = puVar28[-1];
      uVar18 = (uint)(char)bVar38;
      if ((char)bVar38 < '\0') {
        bVar34 = puVar28[-2];
        if ((char)bVar34 < -0x40) {
          bVar3 = puVar28[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar18 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar18 = bVar3 & 0xf;
          }
          uVar18 = bVar34 & 0x3f | uVar18 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar18 = bVar34 & 0x1f;
        }
        uVar18 = (uint)(bVar38 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto LAB_140818fb4;
LAB_140818fc5:
        if (puVar28 == &UNK_142a9013f) break;
        if (puVar28 + -0x142a9013f < (undefined *)0x3c) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x142a9013f == (undefined *)0x3c) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar18 == 0x5c) goto LAB_140818fc5;
LAB_140818fb4:
      if (uVar18 == 0x2f) goto LAB_140818fc5;
    } while (puVar28 != &DAT_142a90140);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar21 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_140819947;
    if (*plVar21 != 0) goto LAB_140819953;
    *plVar21 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar21 = *plVar21 + 1;
  }
  fVar54 = *(float *)(*in_R8 + 8);
  local_130 = 0;
  lVar7 = *in_RDX;
  lVar8 = in_RDX[1];
  uVar5 = *(undefined4 *)((longlong)in_RDX + 0x14);
  puVar33 = *(uint **)(lVar7 + 0xf8);
  puVar25 = puVar33 + *(longlong *)(lVar7 + 0x100);
  cVar4 = *(char *)(lVar7 + 0x108);
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar21 = (longlong *)*in_RCX;
  puVar9 = (undefined4 *)in_RCX[2];
  local_f8 = 8;
  local_e8 = 0;
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  uVar24 = 0;
  local_108 = 0;
  do {
    if (cVar4 == '\0') {
      if (local_108 == uVar24) {
        do {
          if (puVar33 == puVar25) goto LAB_1408194ad;
          uVar18 = *puVar33;
          puVar33 = puVar33 + 1;
          uVar24 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar18 * 0xa0);
        } while (uVar24 == 0);
        lVar30 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar18 * 0xa0;
        uVar22 = (ulonglong)*(uint *)(lVar30 + 0x94);
        if (*(ulonglong *)(lVar7 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x40 + uVar22 * 0x48)) {
          lVar35 = *(longlong *)(lVar8 + 0x1a8) + uVar22 * 0x48;
          uVar22 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
          if (uVar22 == 0) goto LAB_14081924d;
          local_e8 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar22 * 0x30);
        }
        else {
LAB_14081924d:
          local_e8 = 0;
        }
        local_f8 = *(longlong *)(lVar30 + 0x50);
        local_108 = 0;
      }
      uVar22 = (ulonglong)*(uint *)(local_f8 + 8 + local_108 * 0x10);
    }
    else {
      if (local_108 == uVar24) {
        do {
          if (puVar33 == puVar25) goto LAB_1408194ad;
          uVar18 = *puVar33;
          puVar33 = puVar33 + 1;
          uVar24 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar18 * 0x48);
        } while (uVar24 == 0);
        lVar30 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar18 * 0x48;
        if ((*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar30 + 0x40)) &&
           (uVar22 = *(ulonglong *)
                      (*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8),
           uVar22 != 0)) {
          local_e8 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar22 * 0x30);
        }
        else {
          local_e8 = 0;
        }
        local_108 = 0;
      }
      uVar22 = local_108 & 0xffffffff;
    }
    puVar23 = (undefined8 *)(uVar22 * 0x10 + local_e8);
    uVar10 = *puVar23;
    if (local_130 == 0) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    local_108 = local_108 + 1;
    *(undefined8 *)(local_130 * 8 + 8) = uVar10;
    local_130 = local_130 + 1;
    *puVar9 = uVar6;
    plVar37 = (longlong *)*puVar23;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar30 = *plVar21;
    bVar38 = (byte)(extraout_RAX_02 >> 0x39);
    auVar41 = ZEXT216(CONCAT11(bVar38,bVar38));
    auVar41 = pshuflw(auVar41,auVar41,0);
    lVar35 = 0;
    uVar22 = extraout_RAX_02;
    while( true ) {
      uVar22 = plVar21[1] & uVar22;
      pcVar2 = (char *)(lVar30 + uVar22);
      cVar39 = auVar41[0];
      auVar47[0] = -(*pcVar2 == cVar39);
      cVar42 = auVar41[1];
      auVar47[1] = -(pcVar2[1] == cVar42);
      cVar43 = auVar41[2];
      auVar47[2] = -(pcVar2[2] == cVar43);
      cVar44 = auVar41[3];
      auVar47[3] = -(pcVar2[3] == cVar44);
      auVar47[4] = -(pcVar2[4] == cVar39);
      auVar47[5] = -(pcVar2[5] == cVar42);
      auVar47[6] = -(pcVar2[6] == cVar43);
      auVar47[7] = -(pcVar2[7] == cVar44);
      auVar47[8] = -(pcVar2[8] == cVar39);
      auVar47[9] = -(pcVar2[9] == cVar42);
      auVar47[10] = -(pcVar2[10] == cVar43);
      auVar47[0xb] = -(pcVar2[0xb] == cVar44);
      auVar47[0xc] = -(pcVar2[0xc] == cVar39);
      auVar47[0xd] = -(pcVar2[0xd] == cVar42);
      auVar47[0xe] = -(pcVar2[0xe] == cVar43);
      auVar47[0xf] = -(pcVar2[0xf] == cVar44);
      uVar26 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf;
      uVar18 = (uint)uVar26;
      while (uVar26 != 0) {
        uVar27 = 0;
        for (uVar13 = uVar18; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
          uVar27 = uVar27 + 1;
        }
        uVar31 = uVar27 + uVar22 & plVar21[1];
        if (*(longlong **)(lVar30 + -0x10 + uVar31 * -0x10) == plVar37) {
          uVar22 = lVar30 + uVar31 * -0x10;
          plVar37 = plVar21;
          plVar36 = (longlong *)0x0;
          goto LAB_1408193b2;
        }
        uVar26 = (ushort)(uVar18 - 1) & (ushort)uVar18;
        uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar26);
      }
      auVar45[0] = -(*pcVar2 == (char)iVar14);
      auVar45[1] = -(pcVar2[1] == (char)((uint)iVar14 >> 8));
      auVar45[2] = -(pcVar2[2] == (char)((uint)iVar14 >> 0x10));
      auVar45[3] = -(pcVar2[3] == (char)((uint)iVar14 >> 0x18));
      auVar45[4] = -(pcVar2[4] == (char)iVar15);
      auVar45[5] = -(pcVar2[5] == (char)((uint)iVar15 >> 8));
      auVar45[6] = -(pcVar2[6] == (char)((uint)iVar15 >> 0x10));
      auVar45[7] = -(pcVar2[7] == (char)((uint)iVar15 >> 0x18));
      auVar45[8] = -(pcVar2[8] == (char)iVar16);
      auVar45[9] = -(pcVar2[9] == (char)((uint)iVar16 >> 8));
      auVar45[10] = -(pcVar2[10] == (char)((uint)iVar16 >> 0x10));
      auVar45[0xb] = -(pcVar2[0xb] == (char)((uint)iVar16 >> 0x18));
      auVar45[0xc] = -(pcVar2[0xc] == (char)iVar17);
      auVar45[0xd] = -(pcVar2[0xd] == (char)((uint)iVar17 >> 8));
      auVar45[0xe] = -(pcVar2[0xe] == (char)((uint)iVar17 >> 0x10));
      auVar45[0xf] = -(pcVar2[0xf] == (char)((uint)iVar17 >> 0x18));
      if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 || (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar45 >> 0x17,0) & 1) != 0) || (SUB161(auVar45 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar45 >> 0x27,0) & 1) != 0) || (SUB161(auVar45 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar45 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar45 >> 0x3f,0) & 1) != 0) || (SUB161(auVar45 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar45 >> 0x4f,0) & 1) != 0) || (SUB161(auVar45 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar45 >> 0x5f,0) & 1) != 0) || (SUB161(auVar45 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar45 >> 0x6f,0) & 1) != 0) || (SUB161(auVar45 >> 0x77,0) & 1) != 0) ||
          auVar45[0xf] < '\0') break;
      uVar22 = uVar22 + lVar35 + 0x10;
      lVar35 = lVar35 + 0x10;
    }
    uVar22 = extraout_RAX_02;
    plVar36 = plVar21;
    if (plVar21[2] == 0) {
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>_>
                ();
    }
LAB_1408193b2:
    uVar40 = 0;
    if ((fVar54 < 0.72 || 0.9 <= fVar54) && (*(char *)((longlong)puVar23 + 0xd) == '\x01')) {
      uVar40 = 0x3f800000;
    }
    if (plVar36 != (longlong *)0x0) {
      pauVar11 = (undefined1 (*) [16])*plVar36;
      uVar31 = plVar36[1];
      uVar29 = uVar31 & uVar22;
      auVar41 = *(undefined1 (*) [16])(*pauVar11 + uVar29);
      uVar26 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar41[0xf] >> 7) << 0xf;
      uVar18 = (uint)uVar26;
      if (uVar26 == 0) {
        lVar30 = 0x10;
        do {
          uVar29 = uVar29 + lVar30 & uVar31;
          auVar41 = *(undefined1 (*) [16])(*pauVar11 + uVar29);
          lVar30 = lVar30 + 0x10;
          uVar18 = (uint)(ushort)((ushort)(SUB161(auVar41 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar41[0xf] >> 7) << 0xf);
        } while (uVar18 == 0);
      }
      uVar27 = 0;
      for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
        uVar27 = uVar27 + 1;
      }
      uVar29 = uVar27 + uVar29 & uVar31;
      bVar38 = (*pauVar11)[uVar29];
      if (-1 < (char)bVar38) {
        auVar41 = *pauVar11;
        uVar18 = 0;
        for (uVar27 = (uint)(ushort)((ushort)(SUB161(auVar41 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar41[0xf] >> 7) << 0xf); (uVar27 & 1) == 0;
            uVar27 = uVar27 >> 1 | 0x80000000) {
          uVar18 = uVar18 + 1;
        }
        uVar29 = (ulonglong)uVar18;
        bVar38 = (*pauVar11)[uVar29];
      }
      bVar34 = (byte)(uVar22 >> 0x39);
      (*pauVar11)[uVar29] = bVar34;
      pauVar11[1][uVar29 - 0x10 & uVar31] = bVar34;
      plVar36[2] = plVar36[2] - ((ulonglong)bVar38 & 0xffffffffffffff01);
      *(longlong **)pauVar11[-1 - uVar29] = plVar37;
      *(undefined4 *)(pauVar11[-1 - uVar29] + 8) = uVar40;
      plVar36[3] = plVar36[3] + 1;
    }
  } while( true );
LAB_1408194ad:
  *puVar9 = uVar6;
  hashbrown::map::HashMap::
  retain<usize,f32,std::hash::random::RandomState,alloc::alloc::Global,system_decorator::chimney_smoke_anim_thickness::chimney_smoke_anim_thickness::closure_env_0>
            ();
  puVar25 = *(uint **)(lVar7 + 0xf8);
  fVar53 = -1.0;
  if (fVar54 < 0.72 || 0.9 <= fVar54) {
    fVar53 = 1.0;
  }
  puVar33 = puVar25 + *(longlong *)(lVar7 + 0x100);
  cVar4 = *(char *)(lVar7 + 0x108);
  lVar30 = *in_R9;
  local_110 = 8;
  lVar35 = 0;
  iVar14 = -(uint)(CONCAT13(unaff_XMM11_Bd,
                            CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))) ==
                  CONCAT13(unaff_XMM11_Bd,
                           CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM11_Bh,
                            CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))) ==
                  CONCAT13(unaff_XMM11_Bh,
                           CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM11_Bl,
                            CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))) ==
                  CONCAT13(unaff_XMM11_Bl,
                           CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM11_Bp,
                            CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))) ==
                  CONCAT13(unaff_XMM11_Bp,
                           CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))));
  uVar24 = 0;
  uVar22 = 0;
joined_r0x00014081953f:
  if (cVar4 == '\0') {
    if (uVar22 != uVar24) {
LAB_1408195f8:
      uVar31 = (ulonglong)*(uint *)(local_110 + 8 + uVar22 * 0x10);
      goto LAB_14081968d;
    }
    if (puVar25 != puVar33) {
      while( true ) {
        uVar24 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar25 * 0xa0);
        if (uVar24 != 0) break;
        puVar25 = puVar25 + 1;
        if (puVar25 == puVar33) goto LAB_14081986d;
      }
      lVar19 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar25 * 0xa0;
      uVar22 = (ulonglong)*(uint *)(lVar19 + 0x94);
      lVar12 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x18 + uVar22 * 0x48);
      lVar20 = ~*(ulonglong *)
                 (*(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x38 + uVar22 * 0x48) +
                 *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
      lVar35 = *(longlong *)(lVar12 + 0x10 + lVar20);
      local_f8 = *(longlong *)(lVar12 + 0x28 + lVar20);
      local_110 = *(longlong *)(lVar19 + 0x50);
      puVar25 = puVar25 + 1;
      uVar22 = 0;
      goto LAB_1408195f8;
    }
  }
  else {
    if (uVar22 == uVar24) {
      if (puVar25 == puVar33) goto LAB_14081986d;
      while( true ) {
        uVar24 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar25 * 0x48);
        if (uVar24 != 0) break;
        puVar25 = puVar25 + 1;
        if (puVar25 == puVar33) goto LAB_14081986d;
      }
      lVar35 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar25 * 0x48;
      lVar12 = *(longlong *)(lVar35 + 0x18);
      lVar19 = ~*(ulonglong *)(*(longlong *)(lVar35 + 0x38) + *(longlong *)(lVar7 + 0x110) * 8) *
               0x30;
      lVar35 = *(longlong *)(lVar12 + 0x10 + lVar19);
      local_f8 = *(longlong *)(lVar12 + 0x28 + lVar19);
      puVar25 = puVar25 + 1;
      uVar22 = 0;
    }
    uVar31 = uVar22 & 0xffffffff;
LAB_14081968d:
    plVar37 = (longlong *)(uVar31 * 0x10 + lVar35);
    if (plVar37 != (longlong *)0x0) {
      ::utils::rng::init_rng();
      fastrand::Rng::f32();
      if (*(char *)((longlong)plVar37 + 0xd) == '\0') {
        fastrand::Rng::f32();
        fVar54 = extraout_XMM0_Da_00 * 3.0 + (1.0 - extraout_XMM0_Da_00);
        uVar49 = SUB41(fVar54,0);
        uVar50 = (undefined1)((uint)fVar54 >> 8);
        uVar51 = (undefined1)((uint)fVar54 >> 0x10);
        uVar52 = (undefined1)((uint)fVar54 >> 0x18);
        fVar54 = -1.0;
      }
      else {
        fVar54 = extraout_XMM0_Da * 4.0 + (1.0 - extraout_XMM0_Da) + (1.0 - extraout_XMM0_Da);
        uVar49 = SUB41(fVar54,0);
        uVar50 = (undefined1)((uint)fVar54 >> 8);
        uVar51 = (undefined1)((uint)fVar54 >> 0x10);
        uVar52 = (undefined1)((uint)fVar54 >> 0x18);
        fVar54 = fVar53;
      }
      *puVar9 = uVar6;
      if (plVar21[3] != 0) {
        uVar22 = uVar22 + 1;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar12 = *plVar21;
        bVar38 = (byte)(extraout_RAX_03 >> 0x39);
        auVar41 = ZEXT216(CONCAT11(bVar38,bVar38));
        auVar41 = pshuflw(auVar41,auVar41,0);
        lVar19 = 0;
        uVar29 = extraout_RAX_03;
        while( true ) {
          uVar29 = plVar21[1] & uVar29;
          pcVar2 = (char *)(lVar12 + uVar29);
          cVar39 = auVar41[0];
          auVar48[0] = -(*pcVar2 == cVar39);
          cVar42 = auVar41[1];
          auVar48[1] = -(pcVar2[1] == cVar42);
          cVar43 = auVar41[2];
          auVar48[2] = -(pcVar2[2] == cVar43);
          cVar44 = auVar41[3];
          auVar48[3] = -(pcVar2[3] == cVar44);
          auVar48[4] = -(pcVar2[4] == cVar39);
          auVar48[5] = -(pcVar2[5] == cVar42);
          auVar48[6] = -(pcVar2[6] == cVar43);
          auVar48[7] = -(pcVar2[7] == cVar44);
          auVar48[8] = -(pcVar2[8] == cVar39);
          auVar48[9] = -(pcVar2[9] == cVar42);
          auVar48[10] = -(pcVar2[10] == cVar43);
          auVar48[0xb] = -(pcVar2[0xb] == cVar44);
          auVar48[0xc] = -(pcVar2[0xc] == cVar39);
          auVar48[0xd] = -(pcVar2[0xd] == cVar42);
          auVar48[0xe] = -(pcVar2[0xe] == cVar43);
          auVar48[0xf] = -(pcVar2[0xf] == cVar44);
          uVar26 = (ushort)(SUB161(auVar48 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar48 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar48 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar48 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar48 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar48 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar48 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar48 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar48 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar48 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar48 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar48 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar48[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar26;
          if (uVar26 != 0) {
            do {
              uVar27 = 0;
              for (uVar13 = uVar18; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
                uVar27 = uVar27 + 1;
              }
              uVar32 = uVar27 + uVar29 & plVar21[1];
              if (*plVar37 == *(longlong *)(lVar12 + -0x10 + uVar32 * -0x10)) {
                lVar19 = uVar32 * -0x10;
                fVar55 = (fVar54 * *(float *)(lVar30 + 0x28)) /
                         (float)CONCAT13(uVar52,CONCAT12(uVar51,CONCAT11(uVar50,uVar49))) +
                         *(float *)(lVar12 + -8 + lVar19);
                fVar54 = 0.0;
                if (0.0 <= fVar55) {
                  fVar54 = fVar55;
                }
                fVar55 = 1.0;
                if (fVar54 <= 1.0) {
                  fVar55 = fVar54;
                }
                *(float *)(lVar12 + -8 + lVar19) = fVar55;
                *(undefined4 *)(local_f8 + uVar31 * 4) = uVar5;
                *(float *)(plVar37 + 1) = fVar55;
                goto joined_r0x00014081953f;
              }
              uVar26 = (ushort)(uVar18 - 1) & (ushort)uVar18;
              uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar26);
            } while (uVar26 != 0);
          }
          auVar46[0] = -(*pcVar2 == (char)iVar14);
          auVar46[1] = -(pcVar2[1] == (char)((uint)iVar14 >> 8));
          auVar46[2] = -(pcVar2[2] == (char)((uint)iVar14 >> 0x10));
          auVar46[3] = -(pcVar2[3] == (char)((uint)iVar14 >> 0x18));
          auVar46[4] = -(pcVar2[4] == (char)iVar15);
          auVar46[5] = -(pcVar2[5] == (char)((uint)iVar15 >> 8));
          auVar46[6] = -(pcVar2[6] == (char)((uint)iVar15 >> 0x10));
          auVar46[7] = -(pcVar2[7] == (char)((uint)iVar15 >> 0x18));
          auVar46[8] = -(pcVar2[8] == (char)iVar16);
          auVar46[9] = -(pcVar2[9] == (char)((uint)iVar16 >> 8));
          auVar46[10] = -(pcVar2[10] == (char)((uint)iVar16 >> 0x10));
          auVar46[0xb] = -(pcVar2[0xb] == (char)((uint)iVar16 >> 0x18));
          auVar46[0xc] = -(pcVar2[0xc] == (char)iVar17);
          auVar46[0xd] = -(pcVar2[0xd] == (char)((uint)iVar17 >> 8));
          auVar46[0xe] = -(pcVar2[0xe] == (char)((uint)iVar17 >> 0x10));
          auVar46[0xf] = -(pcVar2[0xf] == (char)((uint)iVar17 >> 0x18));
          if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar46 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar46 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x2f,0) & 1) != 0) || (SUB161(auVar46 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
          uVar29 = uVar29 + lVar19 + 0x10;
          lVar19 = lVar19 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
LAB_14081986d:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar21 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_140819947:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
LAB_140819953:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar21 = *plVar21 + 1;
  }
  return;
}


