
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::terrain::edit_terrain_stroke::edit_terrain_stroke(void)

{
  byte *pbVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  byte bVar8;
  byte bVar9;
  int iVar10;
  longlong lVar11;
  longlong lVar12;
  float *_Src;
  undefined8 uVar13;
  uint uVar14;
  uint uVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  float fVar20;
  int *piVar21;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  char *pcVar22;
  longlong *extraout_RAX_01;
  longlong lVar23;
  ulonglong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  float *pfVar24;
  int *piVar25;
  ulonglong extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  uint uVar26;
  undefined8 *in_RCX;
  longlong lVar27;
  ulonglong uVar28;
  int *piVar29;
  longlong *plVar30;
  undefined8 *in_RDX;
  ulonglong uVar31;
  int *piVar32;
  longlong *in_R8;
  ulonglong uVar33;
  uint uVar34;
  longlong *in_R9;
  longlong lVar35;
  ulonglong uVar36;
  ushort uVar37;
  longlong lVar38;
  longlong lVar39;
  longlong lVar40;
  byte bVar41;
  char cVar42;
  char cVar47;
  float fVar43;
  char cVar45;
  char cVar46;
  undefined1 auVar44 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
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
  float fVar52;
  float fVar53;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  int *local_1e8;
  code *local_1e0;
  undefined4 *local_1d8;
  longlong local_1d0;
  undefined4 *local_1c8;
  longlong *local_1c0;
  longlong local_1b8;
  undefined8 local_1b0;
  char *local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  char *local_178;
  undefined8 local_170;
  undefined4 local_168;
  undefined4 local_164;
  undefined **local_160;
  undefined8 local_158;
  int **local_150;
  undefined8 local_148;
  undefined8 local_140;
  ulonglong *local_130;
  int *local_128;
  int *local_120;
  longlong local_118;
  undefined4 local_110;
  undefined4 local_10c;
  undefined4 *local_108;
  longlong local_100;
  longlong *local_f8;
  undefined4 local_e8;
  char local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_e1 = extraout_AL;
  if (extraout_AL != '\0') {
    uVar31 = 0x48;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar31 != 0) {
        if (uVar31 < 0x48) {
          if ((char)(&DAT_142ab338d)[uVar31] < -0x40) {
LAB_14094e0cc:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 != 0x48) goto LAB_14094e0cc;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar31 != 0xfffffffffffffffe)) {
        if (uVar31 + 2 < 0x48) {
          if ((char)(&UNK_142ab338f)[uVar31] < -0x40) {
LAB_14094e12a:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 + 2 != 0x48) goto LAB_14094e12a;
      }
    }
    pcVar22 = "Was requested to edit a terrain stroke  that doesnt exist";
    do {
      bVar41 = pcVar22[-1];
      uVar26 = (uint)(char)bVar41;
      if ((char)bVar41 < '\0') {
        bVar8 = pcVar22[-2];
        if ((char)bVar8 < -0x40) {
          bVar9 = pcVar22[-3];
          if ((char)bVar9 < -0x40) {
            pbVar1 = (byte *)(pcVar22 + -4);
            pcVar22 = pcVar22 + -4;
            uVar26 = bVar9 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar22 = pcVar22 + -3;
            uVar26 = bVar9 & 0xf;
          }
          uVar26 = bVar8 & 0x3f | uVar26 << 6;
        }
        else {
          pcVar22 = pcVar22 + -2;
          uVar26 = bVar8 & 0x1f;
        }
        uVar26 = (uint)(bVar41 & 0x3f) | uVar26 << 6;
        if (uVar26 != 0x5c) goto LAB_14094e1e9;
LAB_14094e1fe:
        if (pcVar22 == "") break;
        if (pcVar22 + -0x142ab38d7 < (char *)0x3e) {
          if (-0x41 < pcVar22[1]) break;
        }
        else if (pcVar22 + -0x142ab38d7 == (char *)0x3e) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      pcVar22 = pcVar22 + -1;
      if (uVar26 == 0x5c) goto LAB_14094e1fe;
LAB_14094e1e9:
      if (uVar26 == 0x2f) goto LAB_14094e1fe;
    } while (pcVar22 !=
             "crates/country-core/src/systems/terrain/edit_terrain_stroke.rsWas requested to edit a terrain stroke  that doesnt exist"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar30 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_14094f032;
    if (*plVar30 != 0) goto LAB_14094f03e;
    *plVar30 = -1;
    local_f8 = plVar30;
    puffin::ThreadProfiler::begin_scope();
    *local_f8 = *local_f8 + 1;
  }
  local_130 = (ulonglong *)*in_RDX;
  lVar40 = in_RDX[1];
  uVar31 = *local_130;
  uVar33 = 0;
  uVar28 = uVar31 - *(ulonglong *)(lVar40 + 0x18);
  if (uVar31 < *(ulonglong *)(lVar40 + 0x18)) {
    uVar28 = uVar33;
  }
  uVar36 = uVar31 - *(ulonglong *)(lVar40 + 0x38);
  if (uVar31 < *(ulonglong *)(lVar40 + 0x38)) {
    uVar36 = uVar33;
  }
  lVar23 = uVar28 * 0x30 + *(longlong *)(lVar40 + 8);
  uVar31 = *(ulonglong *)(lVar40 + 0x10) - uVar28;
  if (*(ulonglong *)(lVar40 + 0x10) < uVar28) {
    lVar23 = 8;
    uVar31 = uVar33;
  }
  lVar27 = uVar36 * 0x30 + *(longlong *)(lVar40 + 0x28);
  uVar28 = *(ulonglong *)(lVar40 + 0x30) - uVar36;
  if (*(ulonglong *)(lVar40 + 0x30) < uVar36) {
    lVar27 = 8;
    uVar28 = uVar33;
  }
  *local_130 = (*(longlong *)(lVar40 + 0x40) - uVar28) - uVar31;
  fVar20 = __real_3a83126f;
  local_128 = (int *)(uVar31 * 0x30 + lVar23);
  local_120 = (int *)(uVar28 * 0x30 + lVar27);
  local_110 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  local_f8 = (longlong *)*in_R8;
  local_1d8 = (undefined4 *)in_R8[2];
  lVar40 = *in_stack_00000038;
  local_1c0 = (longlong *)*in_stack_00000028;
  local_e8 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar11 = *in_R9;
  local_108 = (undefined4 *)in_R9[2];
  local_10c = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  local_118 = *in_stack_00000030;
  local_1c8 = (undefined4 *)in_stack_00000030[2];
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
  local_1d0 = (longlong)local_f8 + 0x28;
  local_100 = lVar11 + 0x20;
  local_1b8 = local_118 + 0x20;
LAB_14094e428:
  do {
    if ((lVar23 == 0) || ((int *)lVar23 == local_128)) {
      if ((lVar27 == 0) || ((int *)lVar27 == local_120)) break;
      lVar12 = 0;
      lVar39 = lVar27;
      lVar27 = lVar27 + 0x30;
    }
    else {
      lVar12 = lVar23 + 0x30;
      lVar39 = lVar23;
    }
    lVar23 = lVar12;
    *local_130 = *local_130 + 1;
    *local_1d8 = local_110;
    iVar10 = *(int *)(lVar39 + 0x24);
    local_1b0 = CONCAT44(local_1b0._4_4_,iVar10);
    if (*(longlong *)((longlong)local_f8 + 0x20) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar12 = *(longlong *)((longlong)local_f8 + 8);
      bVar41 = (byte)(extraout_RAX_02 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar41,bVar41));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar35 = 0;
      uVar31 = extraout_RAX_02;
      while( true ) {
        uVar31 = *(ulonglong *)((longlong)local_f8 + 0x10) & uVar31;
        pcVar22 = (char *)(lVar12 + uVar31);
        cVar42 = auVar44[0];
        auVar50[0] = -(*pcVar22 == cVar42);
        cVar45 = auVar44[1];
        auVar50[1] = -(pcVar22[1] == cVar45);
        cVar46 = auVar44[2];
        auVar50[2] = -(pcVar22[2] == cVar46);
        cVar47 = auVar44[3];
        auVar50[3] = -(pcVar22[3] == cVar47);
        auVar50[4] = -(pcVar22[4] == cVar42);
        auVar50[5] = -(pcVar22[5] == cVar45);
        auVar50[6] = -(pcVar22[6] == cVar46);
        auVar50[7] = -(pcVar22[7] == cVar47);
        auVar50[8] = -(pcVar22[8] == cVar42);
        auVar50[9] = -(pcVar22[9] == cVar45);
        auVar50[10] = -(pcVar22[10] == cVar46);
        auVar50[0xb] = -(pcVar22[0xb] == cVar47);
        auVar50[0xc] = -(pcVar22[0xc] == cVar42);
        auVar50[0xd] = -(pcVar22[0xd] == cVar45);
        auVar50[0xe] = -(pcVar22[0xe] == cVar46);
        auVar50[0xf] = -(pcVar22[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar50 >> 7,0) & 1) | (ushort)(SUB161(auVar50 >> 0xf,0) & 1) << 1
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
        uVar26 = (uint)uVar37;
        while (uVar37 != 0) {
          uVar34 = 0;
          for (uVar14 = uVar26; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
            uVar34 = uVar34 + 1;
          }
          lVar38 = (uVar34 + uVar31 & *(ulonglong *)((longlong)local_f8 + 0x10)) * -0x30;
          if (*(int *)(lVar12 + -0x30 + lVar38) == iVar10) {
            if (*(longlong *)(lVar12 + -0x18 + lVar38) == 0) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            uVar31 = *(ulonglong *)(lVar39 + 0x10);
            if (uVar31 == 0) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            lVar12 = lVar12 + lVar38;
            _Src = *(float **)(lVar39 + 8);
            pfVar24 = *(float **)(lVar12 + -0x20);
            fVar2 = pfVar24[2];
            fVar43 = *pfVar24;
            fVar53 = pfVar24[1];
            fVar52 = *_Src;
            fVar3 = _Src[1];
            fVar4 = _Src[2];
            fVar5 = *(float *)(lVar40 + 0x28);
            *(undefined8 *)(lVar12 + -0x18) = 0;
            if (*(ulonglong *)(lVar12 + -0x28) < uVar31) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              pfVar24 = *(float **)(lVar12 + -0x20);
              lVar35 = *(longlong *)(lVar12 + -0x18);
            }
            else {
              lVar35 = 0;
            }
            memcpy(pfVar24 + lVar35 * 3,_Src,uVar31 * 0xc);
            *(longlong *)(lVar12 + -0x18) = *(longlong *)(lVar12 + -0x18) + uVar31;
            uVar6 = *(undefined4 *)(lVar12 + -0x10);
            local_1a8 = (char *)CONCAT44(local_1a8._4_4_,*(undefined4 *)(lVar39 + 0x20));
            local_1b0 = *(undefined8 *)(lVar39 + 0x18);
            uVar7 = *(undefined4 *)(lVar12 + -8);
            *local_108 = local_e8;
            lVar12 = *(longlong *)(lVar11 + 0x30);
            uVar13 = *(undefined8 *)(lVar11 + 0x40);
            if (lVar12 == *(longlong *)(lVar11 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                        ();
            }
            lVar39 = *(longlong *)(lVar11 + 0x28);
            *(undefined8 *)(lVar39 + lVar12 * 0x28) = uVar13;
            *(undefined4 *)(lVar39 + 8 + lVar12 * 0x28) = 3;
            *(undefined8 *)(lVar39 + 0xc + lVar12 * 0x28) = local_1b0;
            *(undefined4 *)(lVar39 + 0x14 + lVar12 * 0x28) = local_1a8._0_4_;
            *(undefined4 *)(lVar39 + 0x18 + lVar12 * 0x28) = uVar6;
            *(undefined4 *)(lVar39 + 0x1c + lVar12 * 0x28) = uVar7;
            *(float *)(lVar39 + 0x20 + lVar12 * 0x28) =
                 (fVar5 * SQRT((fVar2 - fVar4) * (fVar2 - fVar4) +
                               (fVar53 - fVar3) * (fVar53 - fVar3) +
                               (fVar43 - fVar52) * (fVar43 - fVar52))) / fVar20;
            *(longlong *)(lVar11 + 0x30) = lVar12 + 1;
            *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
            *local_1c8 = local_10c;
            lVar12 = *(longlong *)(local_118 + 0x30);
            uVar13 = *(undefined8 *)(local_118 + 0x40);
            if (lVar12 == *(longlong *)(local_118 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                        ();
            }
            *(undefined8 *)(*(longlong *)(local_118 + 0x28) + lVar12 * 8) = uVar13;
            *(longlong *)(local_118 + 0x30) = lVar12 + 1;
            *(longlong *)(local_118 + 0x40) = *(longlong *)(local_118 + 0x40) + 1;
            LOCK();
            *local_1c0 = *local_1c0 + 1;
            UNLOCK();
            goto LAB_14094e428;
          }
          uVar37 = (ushort)(uVar26 - 1) & (ushort)uVar26;
          uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar37);
        }
        auVar48[0] = -(*pcVar22 == (char)iVar16);
        auVar48[1] = -(pcVar22[1] == (char)((uint)iVar16 >> 8));
        auVar48[2] = -(pcVar22[2] == (char)((uint)iVar16 >> 0x10));
        auVar48[3] = -(pcVar22[3] == (char)((uint)iVar16 >> 0x18));
        auVar48[4] = -(pcVar22[4] == (char)iVar17);
        auVar48[5] = -(pcVar22[5] == (char)((uint)iVar17 >> 8));
        auVar48[6] = -(pcVar22[6] == (char)((uint)iVar17 >> 0x10));
        auVar48[7] = -(pcVar22[7] == (char)((uint)iVar17 >> 0x18));
        auVar48[8] = -(pcVar22[8] == (char)iVar18);
        auVar48[9] = -(pcVar22[9] == (char)((uint)iVar18 >> 8));
        auVar48[10] = -(pcVar22[10] == (char)((uint)iVar18 >> 0x10));
        auVar48[0xb] = -(pcVar22[0xb] == (char)((uint)iVar18 >> 0x18));
        auVar48[0xc] = -(pcVar22[0xc] == (char)iVar19);
        auVar48[0xd] = -(pcVar22[0xd] == (char)((uint)iVar19 >> 8));
        auVar48[0xe] = -(pcVar22[0xe] == (char)((uint)iVar19 >> 0x10));
        auVar48[0xf] = -(pcVar22[0xf] == (char)((uint)iVar19 >> 0x18));
        if ((((((((((((((((SUB161(auVar48 >> 7,0) & 1) != 0 || (SUB161(auVar48 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar48 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar48 >> 0x1f,0) & 1) != 0) || (SUB161(auVar48 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar48 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar48 >> 0x37,0) & 1) != 0) || (SUB161(auVar48 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar48 >> 0x47,0) & 1) != 0) || (SUB161(auVar48 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar48 >> 0x57,0) & 1) != 0) || (SUB161(auVar48 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar48 >> 0x67,0) & 1) != 0) || (SUB161(auVar48 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar48 >> 0x77,0) & 1) != 0) || auVar48[0xf] < '\0') break;
        uVar31 = uVar31 + lVar35 + 0x10;
        lVar35 = lVar35 + 0x10;
      }
    }
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_1e8 = (int *)(lVar39 + 0x24);
      local_1e0 = resources::terrain_editing::impl$11::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_190 = *extraout_RAX_03;
      uStack_188 = extraout_RAX_03[1];
      local_164 = *(undefined4 *)(extraout_RAX_03 + 2);
      local_180 = 1;
      local_178 = "country_core::systems::terrain::edit_terrain_stroke";
      local_170 = 0x33;
      local_160 = &PTR_s_Was_requested_to_set_points_of_t_142ab3a20;
      local_158 = 2;
      local_150 = &local_1e8;
      local_148 = 1;
      local_140 = 0;
      local_1b0 = 0;
      local_1a8 = "country_core::systems::terrain::edit_terrain_stroke";
      local_1a0 = 0x33;
      local_198 = 0;
      local_168 = 1;
      ::log::__private_api::impl$2::log();
    }
  } while( true );
  local_130 = (ulonglong *)*in_RCX;
  lVar40 = in_RCX[1];
  uVar31 = *local_130;
  uVar33 = 0;
  uVar28 = uVar31 - *(ulonglong *)(lVar40 + 0x18);
  if (uVar31 < *(ulonglong *)(lVar40 + 0x18)) {
    uVar28 = uVar33;
  }
  uVar36 = uVar31 - *(ulonglong *)(lVar40 + 0x38);
  if (uVar31 < *(ulonglong *)(lVar40 + 0x38)) {
    uVar36 = uVar33;
  }
  piVar25 = (int *)(uVar28 * 0x28 + *(longlong *)(lVar40 + 8));
  uVar31 = *(ulonglong *)(lVar40 + 0x10) - uVar28;
  if (*(ulonglong *)(lVar40 + 0x10) < uVar28) {
    piVar25 = (int *)&DAT_00000008;
    uVar31 = uVar33;
  }
  piVar29 = (int *)(uVar36 * 0x28 + *(longlong *)(lVar40 + 0x28));
  uVar28 = *(ulonglong *)(lVar40 + 0x30) - uVar36;
  if (*(ulonglong *)(lVar40 + 0x30) < uVar36) {
    piVar29 = (int *)&DAT_00000008;
    uVar28 = uVar33;
  }
  *local_130 = (*(longlong *)(lVar40 + 0x40) - uVar28) - uVar31;
  fVar2 = __real_41a00000;
  uVar26 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
  fVar20 = __real_3f800000;
  local_128 = piVar25 + uVar31 * 10;
  local_120 = piVar29 + uVar28 * 10;
LAB_14094e8c8:
  do {
    if ((piVar25 == (int *)0x0) || (piVar25 == local_128)) {
      if ((piVar29 == (int *)0x0) || (piVar29 == local_120)) break;
      piVar21 = (int *)0x0;
      piVar32 = piVar29;
      piVar29 = piVar29 + 10;
    }
    else {
      piVar21 = piVar25 + 10;
      piVar32 = piVar25;
    }
    piVar25 = piVar21;
    *local_130 = *local_130 + 1;
    *local_1d8 = local_110;
    iVar16 = piVar32[7];
    local_1b0 = CONCAT44(local_1b0._4_4_,iVar16);
    if (*(longlong *)((longlong)local_f8 + 0x20) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar40 = *(longlong *)((longlong)local_f8 + 8);
      bVar41 = (byte)(extraout_RAX_04 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar41,bVar41));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar23 = 0;
      uVar31 = extraout_RAX_04;
      do {
        uVar31 = *(ulonglong *)((longlong)local_f8 + 0x10) & uVar31;
        pcVar22 = (char *)(lVar40 + uVar31);
        cVar42 = auVar44[0];
        auVar51[0] = -(*pcVar22 == cVar42);
        cVar45 = auVar44[1];
        auVar51[1] = -(pcVar22[1] == cVar45);
        cVar46 = auVar44[2];
        auVar51[2] = -(pcVar22[2] == cVar46);
        cVar47 = auVar44[3];
        auVar51[3] = -(pcVar22[3] == cVar47);
        auVar51[4] = -(pcVar22[4] == cVar42);
        auVar51[5] = -(pcVar22[5] == cVar45);
        auVar51[6] = -(pcVar22[6] == cVar46);
        auVar51[7] = -(pcVar22[7] == cVar47);
        auVar51[8] = -(pcVar22[8] == cVar42);
        auVar51[9] = -(pcVar22[9] == cVar45);
        auVar51[10] = -(pcVar22[10] == cVar46);
        auVar51[0xb] = -(pcVar22[0xb] == cVar47);
        auVar51[0xc] = -(pcVar22[0xc] == cVar42);
        auVar51[0xd] = -(pcVar22[0xd] == cVar45);
        auVar51[0xe] = -(pcVar22[0xe] == cVar46);
        auVar51[0xf] = -(pcVar22[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar51 >> 7,0) & 1) | (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar51 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar51 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar51 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar51 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar51 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar51 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar51 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar51 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar51 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar51 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar51 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe | (ushort)(auVar51[0xf] >> 7) << 0xf
        ;
        uVar34 = (uint)uVar37;
        while (uVar37 != 0) {
          uVar14 = 0;
          for (uVar15 = uVar34; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          lVar27 = (uVar14 + uVar31 & *(ulonglong *)((longlong)local_f8 + 0x10)) * -0x30;
          if (*(int *)(lVar40 + -0x30 + lVar27) == iVar16) {
            lVar40 = lVar40 + lVar27;
            if (*piVar32 == 2) {
              fVar43 = *(float *)(lVar40 + -0x10);
              fVar53 = 0.0;
              if (0.0 <= (float)piVar32[1] + fVar43) {
                fVar53 = (float)piVar32[1] + fVar43;
              }
              fVar52 = fVar20;
              if (fVar53 <= fVar20) {
                fVar52 = fVar53;
              }
              *(float *)(lVar40 + -0x10) = fVar52;
              if ((fVar43 != fVar52) || (NAN(fVar43) || NAN(fVar52))) {
                local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar32[6]);
                local_1b0 = *(undefined8 *)(piVar32 + 4);
                *local_108 = local_e8;
                lVar23 = *(longlong *)(lVar11 + 0x30);
                uVar13 = *(undefined8 *)(lVar11 + 0x40);
                if (lVar23 == *(longlong *)(lVar11 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                            ();
                }
                lVar27 = *(longlong *)(lVar11 + 0x28);
                *(undefined8 *)(lVar27 + lVar23 * 0x28) = uVar13;
                *(undefined4 *)(lVar27 + 8 + lVar23 * 0x28) = 1;
                *(undefined8 *)(lVar27 + 0xc + lVar23 * 0x28) = local_1b0;
                *(undefined4 *)(lVar27 + 0x14 + lVar23 * 0x28) = local_1a8._0_4_;
                *(float *)(lVar27 + 0x18 + lVar23 * 0x28) = fVar52;
                *(uint *)(lVar27 + 0x1c + lVar23 * 0x28) = (uint)(fVar43 - fVar52) & uVar26;
                goto LAB_14094ec22;
              }
            }
            else if (*piVar32 == 1) {
              fVar43 = (float)piVar32[1];
              fVar53 = *(float *)(lVar40 + -0x10);
              *(float *)(lVar40 + -0x10) = fVar43;
              local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar32[6]);
              local_1b0 = *(undefined8 *)(piVar32 + 4);
              *local_108 = local_e8;
              lVar23 = *(longlong *)(lVar11 + 0x30);
              uVar13 = *(undefined8 *)(lVar11 + 0x40);
              if (lVar23 == *(longlong *)(lVar11 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                          ();
              }
              lVar27 = *(longlong *)(lVar11 + 0x28);
              *(undefined8 *)(lVar27 + lVar23 * 0x28) = uVar13;
              *(undefined4 *)(lVar27 + 8 + lVar23 * 0x28) = 1;
              *(undefined8 *)(lVar27 + 0xc + lVar23 * 0x28) = local_1b0;
              *(undefined4 *)(lVar27 + 0x14 + lVar23 * 0x28) = local_1a8._0_4_;
              *(float *)(lVar27 + 0x18 + lVar23 * 0x28) = fVar43;
              *(uint *)(lVar27 + 0x1c + lVar23 * 0x28) = (uint)(fVar43 - fVar53) & uVar26;
LAB_14094ec22:
              *(longlong *)(lVar11 + 0x30) = lVar23 + 1;
              *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
            }
            if (piVar32[2] != 0) {
              if (piVar32[2] == 1) {
                fVar43 = (float)piVar32[3];
                fVar53 = fVar43 - *(float *)(lVar40 + -8);
                if (((fVar43 == 0.0) && (!NAN(fVar43))) && (0.0 < *(float *)(lVar40 + -8))) {
                  uVar6 = *(undefined4 *)(lVar40 + -0x10);
                  local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar32[6]);
                  local_1b0 = *(undefined8 *)(piVar32 + 4);
                  *local_108 = local_e8;
                  lVar23 = *(longlong *)(lVar11 + 0x30);
                  uVar13 = *(undefined8 *)(lVar11 + 0x40);
                  if (lVar23 == *(longlong *)(lVar11 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                              ();
                  }
                  lVar27 = *(longlong *)(lVar11 + 0x28);
                  *(undefined8 *)(lVar27 + lVar23 * 0x28) = uVar13;
                  *(undefined4 *)(lVar27 + 8 + lVar23 * 0x28) = 4;
                  *(undefined8 *)(lVar27 + 0xc + lVar23 * 0x28) = local_1b0;
                  *(undefined4 *)(lVar27 + 0x14 + lVar23 * 0x28) = local_1a8._0_4_;
                  *(undefined4 *)(lVar27 + 0x18 + lVar23 * 0x28) = uVar6;
                  *(uint *)(lVar27 + 0x1c + lVar23 * 0x28) = (uint)fVar53 & uVar26;
                  *(longlong *)(lVar11 + 0x30) = lVar23 + 1;
                  *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
                }
                *(float *)(lVar40 + -8) = fVar43;
                uVar6 = *(undefined4 *)(lVar40 + -0x10);
                local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar32[6]);
                local_1b0 = *(undefined8 *)(piVar32 + 4);
                *local_108 = local_e8;
                lVar40 = *(longlong *)(lVar11 + 0x30);
                uVar13 = *(undefined8 *)(lVar11 + 0x40);
                if (lVar40 == *(longlong *)(lVar11 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                            ();
                }
                lVar23 = *(longlong *)(lVar11 + 0x28);
                *(undefined8 *)(lVar23 + lVar40 * 0x28) = uVar13;
                *(undefined4 *)(lVar23 + 8 + lVar40 * 0x28) = 2;
                *(undefined8 *)(lVar23 + 0xc + lVar40 * 0x28) = local_1b0;
                *(undefined4 *)(lVar23 + 0x14 + lVar40 * 0x28) = local_1a8._0_4_;
                *(undefined4 *)(lVar23 + 0x18 + lVar40 * 0x28) = uVar6;
                *(float *)(lVar23 + 0x1c + lVar40 * 0x28) = fVar43;
                *(uint *)(lVar23 + 0x20 + lVar40 * 0x28) = (uint)fVar53 & uVar26;
LAB_14094ef10:
                *(longlong *)(lVar11 + 0x30) = lVar40 + 1;
                *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
              }
              else {
                fVar43 = (float)piVar32[3];
                fVar53 = *(float *)(lVar40 + -8);
                if (((0.0 < fVar43) && (fVar53 == 0.0)) && (!NAN(fVar53))) {
                  *(undefined4 *)(lVar40 + -8) = 0x3f800000;
                  fVar53 = fVar20;
                }
                fVar43 = fVar43 + fVar53;
                fVar52 = 0.0;
                if (0.0 <= fVar43) {
                  fVar52 = fVar43;
                }
                fVar43 = fVar2;
                if (fVar52 <= fVar2) {
                  fVar43 = fVar52;
                }
                fVar43 = (float)(~-(uint)(fVar43 < fVar20) & (uint)fVar43);
                *(float *)(lVar40 + -8) = fVar43;
                if ((fVar53 != fVar43) || (NAN(fVar53) || NAN(fVar43))) {
                  uVar6 = *(undefined4 *)(lVar40 + -0x10);
                  local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar32[6]);
                  local_1b0 = *(undefined8 *)(piVar32 + 4);
                  *local_108 = local_e8;
                  lVar23 = *(longlong *)(lVar11 + 0x30);
                  uVar13 = *(undefined8 *)(lVar11 + 0x40);
                  if (lVar23 == *(longlong *)(lVar11 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                              ();
                  }
                  lVar27 = *(longlong *)(lVar11 + 0x28);
                  *(undefined8 *)(lVar27 + lVar23 * 0x28) = uVar13;
                  *(undefined4 *)(lVar27 + 8 + lVar23 * 0x28) = 2;
                  *(undefined8 *)(lVar27 + 0xc + lVar23 * 0x28) = local_1b0;
                  *(undefined4 *)(lVar27 + 0x14 + lVar23 * 0x28) = local_1a8._0_4_;
                  *(undefined4 *)(lVar27 + 0x18 + lVar23 * 0x28) = uVar6;
                  *(float *)(lVar27 + 0x1c + lVar23 * 0x28) = fVar43;
                  *(uint *)(lVar27 + 0x20 + lVar23 * 0x28) = (uint)(fVar53 - fVar43) & uVar26;
                  *(longlong *)(lVar11 + 0x30) = lVar23 + 1;
                  *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
                }
                if (0.0 < fVar53) {
                  fVar43 = *(float *)(lVar40 + -8);
                  if ((fVar43 == 0.0) && (!NAN(fVar43))) {
                    uVar6 = *(undefined4 *)(lVar40 + -0x10);
                    local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar32[6]);
                    local_1b0 = *(undefined8 *)(piVar32 + 4);
                    *local_108 = local_e8;
                    lVar40 = *(longlong *)(lVar11 + 0x30);
                    uVar13 = *(undefined8 *)(lVar11 + 0x40);
                    if (lVar40 == *(longlong *)(lVar11 + 0x20)) {
                      alloc::raw_vec::RawVec::
                      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                                ();
                    }
                    lVar23 = *(longlong *)(lVar11 + 0x28);
                    *(undefined8 *)(lVar23 + lVar40 * 0x28) = uVar13;
                    *(undefined4 *)(lVar23 + 8 + lVar40 * 0x28) = 4;
                    *(undefined8 *)(lVar23 + 0xc + lVar40 * 0x28) = local_1b0;
                    *(undefined4 *)(lVar23 + 0x14 + lVar40 * 0x28) = local_1a8._0_4_;
                    *(undefined4 *)(lVar23 + 0x18 + lVar40 * 0x28) = uVar6;
                    *(uint *)(lVar23 + 0x1c + lVar40 * 0x28) = (uint)(fVar53 - fVar43) & uVar26;
                    goto LAB_14094ef10;
                  }
                }
              }
            }
            *local_1c8 = local_10c;
            lVar40 = *(longlong *)(local_118 + 0x30);
            uVar13 = *(undefined8 *)(local_118 + 0x40);
            if (lVar40 == *(longlong *)(local_118 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                        ();
            }
            *(undefined8 *)(*(longlong *)(local_118 + 0x28) + lVar40 * 8) = uVar13;
            *(longlong *)(local_118 + 0x30) = lVar40 + 1;
            *(longlong *)(local_118 + 0x40) = *(longlong *)(local_118 + 0x40) + 1;
            LOCK();
            *local_1c0 = *local_1c0 + 1;
            UNLOCK();
            goto LAB_14094e8c8;
          }
          uVar37 = (ushort)(uVar34 - 1) & (ushort)uVar34;
          uVar34 = CONCAT22((short)(uVar34 - 1 >> 0x10),uVar37);
        }
        auVar49[0] = -(*pcVar22 == -1);
        auVar49[1] = -(pcVar22[1] == -1);
        auVar49[2] = -(pcVar22[2] == -1);
        auVar49[3] = -(pcVar22[3] == -1);
        auVar49[4] = -(pcVar22[4] == -1);
        auVar49[5] = -(pcVar22[5] == -1);
        auVar49[6] = -(pcVar22[6] == -1);
        auVar49[7] = -(pcVar22[7] == -1);
        auVar49[8] = -(pcVar22[8] == -1);
        auVar49[9] = -(pcVar22[9] == -1);
        auVar49[10] = -(pcVar22[10] == -1);
        auVar49[0xb] = -(pcVar22[0xb] == -1);
        auVar49[0xc] = -(pcVar22[0xc] == -1);
        auVar49[0xd] = -(pcVar22[0xd] == -1);
        auVar49[0xe] = -(pcVar22[0xe] == -1);
        auVar49[0xf] = -(pcVar22[0xf] == -1);
        if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x1f,0) & 1) != 0) || (SUB161(auVar49 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x37,0) & 1) != 0) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
        uVar31 = uVar31 + lVar23 + 0x10;
        lVar23 = lVar23 + 0x10;
      } while( true );
    }
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_1e8 = piVar32 + 7;
      local_1e0 = resources::terrain_editing::impl$11::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_190 = *extraout_RAX_05;
      uStack_188 = extraout_RAX_05[1];
      local_164 = *(undefined4 *)(extraout_RAX_05 + 2);
      local_180 = 1;
      local_178 = "country_core::systems::terrain::edit_terrain_stroke";
      local_170 = 0x33;
      local_160 = &PTR_s_Was_requested_to_edit_a_terrain_s_142ab3950;
      local_158 = 2;
      local_150 = &local_1e8;
      local_148 = 1;
      local_140 = 0;
      local_1b0 = 0;
      local_1a8 = "country_core::systems::terrain::edit_terrain_stroke";
      local_1a0 = 0x33;
      local_198 = 0;
      local_168 = 1;
      ::log::__private_api::impl$2::log();
    }
  } while( true );
  if (local_e1 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 1) {
      plVar30 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_14094f032:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar30 != 0) {
LAB_14094f03e:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar30 = -1;
    local_f8 = plVar30;
    puffin::ThreadProfiler::end_scope();
    *local_f8 = *local_f8 + 1;
  }
  return;
}

