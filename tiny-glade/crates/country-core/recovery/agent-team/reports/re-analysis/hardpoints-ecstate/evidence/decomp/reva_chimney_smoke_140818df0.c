
/* WARNING: Removing unreachable block (ram,0x00014081985a) */

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
  bool bVar13;
  bool bVar14;
  uint uVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  char extraout_AL;
  uint uVar23;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong lVar24;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong lVar25;
  longlong *plVar26;
  longlong *in_RDX;
  ulonglong uVar27;
  undefined8 *puVar28;
  ulonglong uVar29;
  uint *puVar30;
  ushort uVar31;
  uint uVar32;
  longlong *in_R8;
  undefined *puVar33;
  ulonglong uVar34;
  longlong *in_R9;
  longlong lVar35;
  ulonglong uVar36;
  ulonglong uVar37;
  uint *puVar38;
  byte bVar39;
  longlong lVar40;
  longlong *plVar41;
  longlong *plVar42;
  byte bVar43;
  char cVar44;
  char cVar49;
  float fVar45;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  char cVar47;
  char cVar48;
  undefined1 auVar46 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 uVar54;
  undefined1 unaff_XMM6_Bb;
  undefined1 uVar55;
  undefined1 unaff_XMM6_Bc;
  undefined1 uVar56;
  undefined1 unaff_XMM6_Bd;
  undefined1 uVar57;
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
  float fVar58;
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
  float fVar59;
  float fVar60;
  longlong local_130;
  longlong local_110;
  ulonglong local_108;
  longlong local_f8;
  longlong local_e8;
  
  plVar26 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar26 != (longlong *)0x0) {
        if (&DAT_0000004b < plVar26) {
          if (plVar26 != (longlong *)&DAT_0000004c) goto LAB_140818eaa;
        }
        else if ((char)plVar26[0x28552006] < -0x40) {
LAB_140818eaa:
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar26 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar26 + 2) < (undefined1 *)0x4c) {
          if (*(char *)((longlong)plVar26 + 0x142a90032) < -0x40) {
LAB_140818ef7:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar26 + 2) != (undefined1 *)0x4c) goto LAB_140818ef7;
      }
    }
    puVar33 = &DAT_142a9017c;
    do {
      bVar43 = puVar33[-1];
      uVar23 = (uint)(char)bVar43;
      if ((char)bVar43 < '\0') {
        bVar39 = puVar33[-2];
        if ((char)bVar39 < -0x40) {
          bVar3 = puVar33[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar33 + -4;
            puVar33 = puVar33 + -4;
            uVar23 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar33 = puVar33 + -3;
            uVar23 = bVar3 & 0xf;
          }
          uVar23 = bVar39 & 0x3f | uVar23 << 6;
        }
        else {
          puVar33 = puVar33 + -2;
          uVar23 = bVar39 & 0x1f;
        }
        uVar23 = (uint)(bVar43 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_140818fb4;
LAB_140818fc5:
        if (puVar33 == &UNK_142a9013f) break;
        if (puVar33 + -0x142a9013f < (undefined *)0x3c) {
          if (-0x41 < (char)puVar33[1]) break;
        }
        else if (puVar33 + -0x142a9013f == (undefined *)0x3c) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar33 = puVar33 + -1;
      if (uVar23 == 0x5c) goto LAB_140818fc5;
LAB_140818fb4:
      if (uVar23 == 0x2f) goto LAB_140818fc5;
    } while (puVar33 != &DAT_142a90140);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar26 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar26 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_140819947;
    if (*plVar26 != 0) goto LAB_140819953;
    *plVar26 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar26 = *plVar26 + 1;
  }
  fVar20 = __real_3f800000;
  fVar45 = *(float *)(*in_R8 + 8);
  bVar14 = fVar45 < __real_3f3851ec;
  bVar13 = __real_3f666666 <= fVar45;
  local_130 = 0;
  lVar7 = *in_RDX;
  lVar8 = in_RDX[1];
  uVar5 = *(undefined4 *)((longlong)in_RDX + 0x14);
  puVar38 = *(uint **)(lVar7 + 0xf8);
  puVar30 = puVar38 + *(longlong *)(lVar7 + 0x100);
  cVar4 = *(char *)(lVar7 + 0x108);
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar26 = (longlong *)*in_RCX;
  puVar9 = (undefined4 *)in_RCX[2];
  local_f8 = 8;
  local_e8 = 0;
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar18 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar19 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  uVar29 = 0;
  local_108 = 0;
  do {
    if (cVar4 == '\0') {
      if (local_108 == uVar29) {
        do {
          if (puVar38 == puVar30) goto LAB_1408194ad;
          uVar23 = *puVar38;
          puVar38 = puVar38 + 1;
          uVar29 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0);
        } while (uVar29 == 0);
        lVar35 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar23 * 0xa0;
        uVar27 = (ulonglong)*(uint *)(lVar35 + 0x94);
        if (*(ulonglong *)(lVar7 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x40 + uVar27 * 0x48)) {
          lVar40 = *(longlong *)(lVar8 + 0x1a8) + uVar27 * 0x48;
          uVar27 = *(ulonglong *)(*(longlong *)(lVar40 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
          if (uVar27 == 0) goto LAB_14081924d;
          local_e8 = *(longlong *)(*(longlong *)(lVar40 + 0x18) + 0x10 + ~uVar27 * 0x30);
        }
        else {
LAB_14081924d:
          local_e8 = 0;
        }
        local_f8 = *(longlong *)(lVar35 + 0x50);
        local_108 = 0;
      }
      uVar27 = (ulonglong)*(uint *)(local_f8 + 8 + local_108 * 0x10);
    }
    else {
      if (local_108 == uVar29) {
        do {
          if (puVar38 == puVar30) goto LAB_1408194ad;
          uVar23 = *puVar38;
          puVar38 = puVar38 + 1;
          uVar29 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48);
        } while (uVar29 == 0);
        lVar35 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        if ((*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar35 + 0x40)) &&
           (uVar27 = *(ulonglong *)
                      (*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8),
           uVar27 != 0)) {
          local_e8 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar27 * 0x30);
        }
        else {
          local_e8 = 0;
        }
        local_108 = 0;
      }
      uVar27 = local_108 & 0xffffffff;
    }
    puVar28 = (undefined8 *)(uVar27 * 0x10 + local_e8);
    uVar10 = *puVar28;
    if (local_130 == 0) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    local_108 = local_108 + 1;
    *(undefined8 *)(local_130 * 8 + 8) = uVar10;
    local_130 = local_130 + 1;
    *puVar9 = uVar6;
    plVar42 = (longlong *)*puVar28;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar35 = *plVar26;
    bVar43 = (byte)(extraout_RAX_02 >> 0x39);
    auVar46 = ZEXT216(CONCAT11(bVar43,bVar43));
    auVar46 = pshuflw(auVar46,auVar46,0);
    lVar40 = 0;
    uVar27 = extraout_RAX_02;
    while( true ) {
      uVar27 = plVar26[1] & uVar27;
      pcVar2 = (char *)(lVar35 + uVar27);
      cVar44 = auVar46[0];
      auVar52[0] = -(*pcVar2 == cVar44);
      cVar47 = auVar46[1];
      auVar52[1] = -(pcVar2[1] == cVar47);
      cVar48 = auVar46[2];
      auVar52[2] = -(pcVar2[2] == cVar48);
      cVar49 = auVar46[3];
      auVar52[3] = -(pcVar2[3] == cVar49);
      auVar52[4] = -(pcVar2[4] == cVar44);
      auVar52[5] = -(pcVar2[5] == cVar47);
      auVar52[6] = -(pcVar2[6] == cVar48);
      auVar52[7] = -(pcVar2[7] == cVar49);
      auVar52[8] = -(pcVar2[8] == cVar44);
      auVar52[9] = -(pcVar2[9] == cVar47);
      auVar52[10] = -(pcVar2[10] == cVar48);
      auVar52[0xb] = -(pcVar2[0xb] == cVar49);
      auVar52[0xc] = -(pcVar2[0xc] == cVar44);
      auVar52[0xd] = -(pcVar2[0xd] == cVar47);
      auVar52[0xe] = -(pcVar2[0xe] == cVar48);
      auVar52[0xf] = -(pcVar2[0xf] == cVar49);
      uVar31 = (ushort)(SUB161(auVar52 >> 7,0) & 1) | (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe | (ushort)(auVar52[0xf] >> 7) << 0xf;
      uVar23 = (uint)uVar31;
      while (uVar31 != 0) {
        uVar32 = 0;
        for (uVar15 = uVar23; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar32 = uVar32 + 1;
        }
        uVar36 = uVar32 + uVar27 & plVar26[1];
        if (*(longlong **)(lVar35 + -0x10 + uVar36 * -0x10) == plVar42) {
          uVar27 = lVar35 + uVar36 * -0x10;
          plVar42 = plVar26;
          plVar41 = (longlong *)0x0;
          goto LAB_1408193b2;
        }
        uVar31 = (ushort)(uVar23 - 1) & (ushort)uVar23;
        uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar31);
      }
      auVar50[0] = -(*pcVar2 == (char)iVar16);
      auVar50[1] = -(pcVar2[1] == (char)((uint)iVar16 >> 8));
      auVar50[2] = -(pcVar2[2] == (char)((uint)iVar16 >> 0x10));
      auVar50[3] = -(pcVar2[3] == (char)((uint)iVar16 >> 0x18));
      auVar50[4] = -(pcVar2[4] == (char)iVar17);
      auVar50[5] = -(pcVar2[5] == (char)((uint)iVar17 >> 8));
      auVar50[6] = -(pcVar2[6] == (char)((uint)iVar17 >> 0x10));
      auVar50[7] = -(pcVar2[7] == (char)((uint)iVar17 >> 0x18));
      auVar50[8] = -(pcVar2[8] == (char)iVar18);
      auVar50[9] = -(pcVar2[9] == (char)((uint)iVar18 >> 8));
      auVar50[10] = -(pcVar2[10] == (char)((uint)iVar18 >> 0x10));
      auVar50[0xb] = -(pcVar2[0xb] == (char)((uint)iVar18 >> 0x18));
      auVar50[0xc] = -(pcVar2[0xc] == (char)iVar19);
      auVar50[0xd] = -(pcVar2[0xd] == (char)((uint)iVar19 >> 8));
      auVar50[0xe] = -(pcVar2[0xe] == (char)((uint)iVar19 >> 0x10));
      auVar50[0xf] = -(pcVar2[0xf] == (char)((uint)iVar19 >> 0x18));
      if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 || (SUB161(auVar50 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar50 >> 0x17,0) & 1) != 0) || (SUB161(auVar50 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar50 >> 0x27,0) & 1) != 0) || (SUB161(auVar50 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar50 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar50 >> 0x3f,0) & 1) != 0) || (SUB161(auVar50 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar50 >> 0x4f,0) & 1) != 0) || (SUB161(auVar50 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar50 >> 0x5f,0) & 1) != 0) || (SUB161(auVar50 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar50 >> 0x6f,0) & 1) != 0) || (SUB161(auVar50 >> 0x77,0) & 1) != 0) ||
          auVar50[0xf] < '\0') break;
      uVar27 = uVar27 + lVar40 + 0x10;
      lVar40 = lVar40 + 0x10;
    }
    uVar27 = extraout_RAX_02;
    plVar41 = plVar26;
    if (plVar26[2] == 0) {
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>_>
                ();
    }
LAB_1408193b2:
    fVar45 = 0.0;
    if ((bVar14 || bVar13) && (*(char *)((longlong)puVar28 + 0xd) == '\x01')) {
      fVar45 = fVar20;
    }
    if (plVar41 != (longlong *)0x0) {
      pauVar11 = (undefined1 (*) [16])*plVar41;
      uVar36 = plVar41[1];
      uVar34 = uVar36 & uVar27;
      auVar46 = *(undefined1 (*) [16])(*pauVar11 + uVar34);
      uVar31 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar46[0xf] >> 7) << 0xf;
      uVar23 = (uint)uVar31;
      if (uVar31 == 0) {
        lVar35 = 0x10;
        do {
          uVar34 = uVar34 + lVar35 & uVar36;
          auVar46 = *(undefined1 (*) [16])(*pauVar11 + uVar34);
          lVar35 = lVar35 + 0x10;
          uVar23 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
        } while (uVar23 == 0);
      }
      uVar32 = 0;
      for (; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
        uVar32 = uVar32 + 1;
      }
      uVar34 = uVar32 + uVar34 & uVar36;
      bVar43 = (*pauVar11)[uVar34];
      if (-1 < (char)bVar43) {
        auVar46 = *pauVar11;
        uVar23 = 0;
        for (uVar32 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar46[0xf] >> 7) << 0xf); (uVar32 & 1) == 0;
            uVar32 = uVar32 >> 1 | 0x80000000) {
          uVar23 = uVar23 + 1;
        }
        uVar34 = (ulonglong)uVar23;
        bVar43 = (*pauVar11)[uVar34];
      }
      bVar39 = (byte)(uVar27 >> 0x39);
      (*pauVar11)[uVar34] = bVar39;
      pauVar11[1][uVar34 - 0x10 & uVar36] = bVar39;
      plVar41[2] = plVar41[2] - ((ulonglong)bVar43 & 0xffffffffffffff01);
      *(longlong **)pauVar11[-1 - uVar34] = plVar42;
      *(float *)(pauVar11[-1 - uVar34] + 8) = fVar45;
      plVar41[3] = plVar41[3] + 1;
    }
  } while( true );
LAB_1408194ad:
  *puVar9 = uVar6;
  hashbrown::map::HashMap::
  retain<usize,f32,std::hash::random::RandomState,alloc::alloc::Global,system_decorator::chimney_smoke_anim_thickness::chimney_smoke_anim_thickness::closure_env_0>
            ();
  fVar22 = __real_40400000;
  fVar21 = __real_bf800000;
  fVar20 = __real_40800000;
  fVar45 = __real_3f800000;
  puVar30 = *(uint **)(lVar7 + 0xf8);
  fVar58 = __real_bf800000;
  if (bVar14 || bVar13) {
    fVar58 = __real_3f800000;
  }
  puVar38 = puVar30 + *(longlong *)(lVar7 + 0x100);
  cVar4 = *(char *)(lVar7 + 0x108);
  lVar35 = *in_R9;
  local_110 = 8;
  lVar40 = 0;
  iVar16 = -(uint)(CONCAT13(unaff_XMM11_Bd,
                            CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))) ==
                  CONCAT13(unaff_XMM11_Bd,
                           CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM11_Bh,
                            CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))) ==
                  CONCAT13(unaff_XMM11_Bh,
                           CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))));
  iVar18 = -(uint)(CONCAT13(unaff_XMM11_Bl,
                            CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))) ==
                  CONCAT13(unaff_XMM11_Bl,
                           CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))));
  iVar19 = -(uint)(CONCAT13(unaff_XMM11_Bp,
                            CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))) ==
                  CONCAT13(unaff_XMM11_Bp,
                           CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))));
  uVar29 = 0;
  uVar27 = 0;
joined_r0x00014081953f:
  if (cVar4 == '\0') {
    if (uVar27 != uVar29) {
LAB_1408195f8:
      uVar36 = (ulonglong)*(uint *)(local_110 + 8 + uVar27 * 0x10);
      goto LAB_14081968d;
    }
    if (puVar30 != puVar38) {
      while( true ) {
        uVar29 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar30 * 0xa0);
        if (uVar29 != 0) break;
        puVar30 = puVar30 + 1;
        if (puVar30 == puVar38) goto LAB_14081986d;
      }
      lVar24 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar30 * 0xa0;
      uVar27 = (ulonglong)*(uint *)(lVar24 + 0x94);
      lVar12 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x18 + uVar27 * 0x48);
      lVar25 = ~*(ulonglong *)
                 (*(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x38 + uVar27 * 0x48) +
                 *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
      lVar40 = *(longlong *)(lVar12 + 0x10 + lVar25);
      local_f8 = *(longlong *)(lVar12 + 0x28 + lVar25);
      local_110 = *(longlong *)(lVar24 + 0x50);
      puVar30 = puVar30 + 1;
      uVar27 = 0;
      goto LAB_1408195f8;
    }
  }
  else {
    if (uVar27 == uVar29) {
      if (puVar30 == puVar38) goto LAB_14081986d;
      while( true ) {
        uVar29 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar30 * 0x48);
        if (uVar29 != 0) break;
        puVar30 = puVar30 + 1;
        if (puVar30 == puVar38) goto LAB_14081986d;
      }
      lVar40 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar30 * 0x48;
      lVar12 = *(longlong *)(lVar40 + 0x18);
      lVar24 = ~*(ulonglong *)(*(longlong *)(lVar40 + 0x38) + *(longlong *)(lVar7 + 0x110) * 8) *
               0x30;
      lVar40 = *(longlong *)(lVar12 + 0x10 + lVar24);
      local_f8 = *(longlong *)(lVar12 + 0x28 + lVar24);
      puVar30 = puVar30 + 1;
      uVar27 = 0;
    }
    uVar36 = uVar27 & 0xffffffff;
LAB_14081968d:
    plVar42 = (longlong *)(uVar36 * 0x10 + lVar40);
    if (plVar42 != (longlong *)0x0) {
      ::utils::rng::init_rng();
      fastrand::Rng::f32();
      if (*(char *)((longlong)plVar42 + 0xd) == '\0') {
        fastrand::Rng::f32();
        fVar59 = extraout_XMM0_Da_00 * fVar22 + (fVar45 - extraout_XMM0_Da_00);
        uVar54 = SUB41(fVar59,0);
        uVar55 = (undefined1)((uint)fVar59 >> 8);
        uVar56 = (undefined1)((uint)fVar59 >> 0x10);
        uVar57 = (undefined1)((uint)fVar59 >> 0x18);
        fVar59 = fVar21;
      }
      else {
        fVar59 = extraout_XMM0_Da * fVar20 +
                 (fVar45 - extraout_XMM0_Da) + (fVar45 - extraout_XMM0_Da);
        uVar54 = SUB41(fVar59,0);
        uVar55 = (undefined1)((uint)fVar59 >> 8);
        uVar56 = (undefined1)((uint)fVar59 >> 0x10);
        uVar57 = (undefined1)((uint)fVar59 >> 0x18);
        fVar59 = fVar58;
      }
      *puVar9 = uVar6;
      if (plVar26[3] != 0) {
        uVar27 = uVar27 + 1;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar12 = *plVar26;
        bVar43 = (byte)(extraout_RAX_03 >> 0x39);
        auVar46 = ZEXT216(CONCAT11(bVar43,bVar43));
        auVar46 = pshuflw(auVar46,auVar46,0);
        lVar24 = 0;
        uVar34 = extraout_RAX_03;
        while( true ) {
          uVar34 = plVar26[1] & uVar34;
          pcVar2 = (char *)(lVar12 + uVar34);
          cVar44 = auVar46[0];
          auVar53[0] = -(*pcVar2 == cVar44);
          cVar47 = auVar46[1];
          auVar53[1] = -(pcVar2[1] == cVar47);
          cVar48 = auVar46[2];
          auVar53[2] = -(pcVar2[2] == cVar48);
          cVar49 = auVar46[3];
          auVar53[3] = -(pcVar2[3] == cVar49);
          auVar53[4] = -(pcVar2[4] == cVar44);
          auVar53[5] = -(pcVar2[5] == cVar47);
          auVar53[6] = -(pcVar2[6] == cVar48);
          auVar53[7] = -(pcVar2[7] == cVar49);
          auVar53[8] = -(pcVar2[8] == cVar44);
          auVar53[9] = -(pcVar2[9] == cVar47);
          auVar53[10] = -(pcVar2[10] == cVar48);
          auVar53[0xb] = -(pcVar2[0xb] == cVar49);
          auVar53[0xc] = -(pcVar2[0xc] == cVar44);
          auVar53[0xd] = -(pcVar2[0xd] == cVar47);
          auVar53[0xe] = -(pcVar2[0xe] == cVar48);
          auVar53[0xf] = -(pcVar2[0xf] == cVar49);
          uVar31 = (ushort)(SUB161(auVar53 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar53[0xf] >> 7) << 0xf;
          uVar23 = (uint)uVar31;
          if (uVar31 != 0) {
            do {
              uVar32 = 0;
              for (uVar15 = uVar23; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar32 = uVar32 + 1;
              }
              uVar37 = uVar32 + uVar34 & plVar26[1];
              if (*plVar42 == *(longlong *)(lVar12 + -0x10 + uVar37 * -0x10)) {
                lVar24 = uVar37 * -0x10;
                fVar60 = (fVar59 * *(float *)(lVar35 + 0x28)) /
                         (float)CONCAT13(uVar57,CONCAT12(uVar56,CONCAT11(uVar55,uVar54))) +
                         *(float *)(lVar12 + -8 + lVar24);
                fVar59 = 0.0;
                if (0.0 <= fVar60) {
                  fVar59 = fVar60;
                }
                fVar60 = fVar45;
                if (fVar59 <= fVar45) {
                  fVar60 = fVar59;
                }
                *(float *)(lVar12 + -8 + lVar24) = fVar60;
                *(undefined4 *)(local_f8 + uVar36 * 4) = uVar5;
                *(float *)(plVar42 + 1) = fVar60;
                goto joined_r0x00014081953f;
              }
              uVar31 = (ushort)(uVar23 - 1) & (ushort)uVar23;
              uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar31);
            } while (uVar31 != 0);
          }
          auVar51[0] = -(*pcVar2 == (char)iVar16);
          auVar51[1] = -(pcVar2[1] == (char)((uint)iVar16 >> 8));
          auVar51[2] = -(pcVar2[2] == (char)((uint)iVar16 >> 0x10));
          auVar51[3] = -(pcVar2[3] == (char)((uint)iVar16 >> 0x18));
          auVar51[4] = -(pcVar2[4] == (char)iVar17);
          auVar51[5] = -(pcVar2[5] == (char)((uint)iVar17 >> 8));
          auVar51[6] = -(pcVar2[6] == (char)((uint)iVar17 >> 0x10));
          auVar51[7] = -(pcVar2[7] == (char)((uint)iVar17 >> 0x18));
          auVar51[8] = -(pcVar2[8] == (char)iVar18);
          auVar51[9] = -(pcVar2[9] == (char)((uint)iVar18 >> 8));
          auVar51[10] = -(pcVar2[10] == (char)((uint)iVar18 >> 0x10));
          auVar51[0xb] = -(pcVar2[0xb] == (char)((uint)iVar18 >> 0x18));
          auVar51[0xc] = -(pcVar2[0xc] == (char)iVar19);
          auVar51[0xd] = -(pcVar2[0xd] == (char)((uint)iVar19 >> 8));
          auVar51[0xe] = -(pcVar2[0xe] == (char)((uint)iVar19 >> 0x10));
          auVar51[0xf] = -(pcVar2[0xf] == (char)((uint)iVar19 >> 0x18));
          if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar51 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar51 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x2f,0) & 1) != 0) || (SUB161(auVar51 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
          uVar34 = uVar34 + lVar24 + 0x10;
          lVar24 = lVar24 + 0x10;
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  }
LAB_14081986d:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar26 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar26 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_140819947:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar26 != 0) {
LAB_140819953:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar26 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar26 = *plVar26 + 1;
  }
  return;
}

