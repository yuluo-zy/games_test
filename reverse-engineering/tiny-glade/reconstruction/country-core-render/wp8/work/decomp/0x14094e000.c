
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
  int *piVar20;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  char *pcVar21;
  longlong *extraout_RAX_01;
  longlong lVar22;
  ulonglong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  float *pfVar23;
  int *piVar24;
  ulonglong extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  uint uVar25;
  undefined8 *in_RCX;
  longlong lVar26;
  ulonglong uVar27;
  int *piVar28;
  longlong *plVar29;
  undefined8 *in_RDX;
  ulonglong uVar30;
  int *piVar31;
  longlong *in_R8;
  ulonglong uVar32;
  longlong *in_R9;
  longlong lVar33;
  ulonglong uVar34;
  ushort uVar35;
  longlong lVar36;
  longlong lVar37;
  longlong lVar38;
  byte bVar39;
  char cVar40;
  char cVar45;
  float fVar41;
  char cVar43;
  char cVar44;
  undefined1 auVar42 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
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
  float fVar50;
  float fVar51;
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
    uVar30 = 0x48;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar30 != 0) {
        if (uVar30 < 0x48) {
          if ((char)(&DAT_142ab338d)[uVar30] < -0x40) {
LAB_14094e0cc:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar30 != 0x48) goto LAB_14094e0cc;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar30 != 0xfffffffffffffffe)) {
        if (uVar30 + 2 < 0x48) {
          if ((char)(&UNK_142ab338f)[uVar30] < -0x40) {
LAB_14094e12a:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar30 + 2 != 0x48) goto LAB_14094e12a;
      }
    }
    pcVar21 = "Was requested to edit a terrain stroke  that doesnt exist";
    do {
      bVar39 = pcVar21[-1];
      uVar25 = (uint)(char)bVar39;
      if ((char)bVar39 < '\0') {
        bVar8 = pcVar21[-2];
        if ((char)bVar8 < -0x40) {
          bVar9 = pcVar21[-3];
          if ((char)bVar9 < -0x40) {
            pbVar1 = (byte *)(pcVar21 + -4);
            pcVar21 = pcVar21 + -4;
            uVar25 = bVar9 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar21 = pcVar21 + -3;
            uVar25 = bVar9 & 0xf;
          }
          uVar25 = bVar8 & 0x3f | uVar25 << 6;
        }
        else {
          pcVar21 = pcVar21 + -2;
          uVar25 = bVar8 & 0x1f;
        }
        uVar25 = (uint)(bVar39 & 0x3f) | uVar25 << 6;
        if (uVar25 != 0x5c) goto LAB_14094e1e9;
LAB_14094e1fe:
        if (pcVar21 == "") break;
        if (pcVar21 + -0x142ab38d7 < (char *)0x3e) {
          if (-0x41 < pcVar21[1]) break;
        }
        else if (pcVar21 + -0x142ab38d7 == (char *)0x3e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar21 = pcVar21 + -1;
      if (uVar25 == 0x5c) goto LAB_14094e1fe;
LAB_14094e1e9:
      if (uVar25 == 0x2f) goto LAB_14094e1fe;
    } while (pcVar21 !=
             "crates/country-core/src/systems/terrain/edit_terrain_stroke.rsWas requested to edit a terrain stroke  that doesnt exist"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar29 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar29 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_14094f032;
    if (*plVar29 != 0) goto LAB_14094f03e;
    *plVar29 = -1;
    local_f8 = plVar29;
    puffin::ThreadProfiler::begin_scope();
    *local_f8 = *local_f8 + 1;
  }
  local_130 = (ulonglong *)*in_RDX;
  lVar38 = in_RDX[1];
  uVar30 = *local_130;
  uVar32 = 0;
  uVar27 = uVar30 - *(ulonglong *)(lVar38 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar38 + 0x18)) {
    uVar27 = uVar32;
  }
  uVar34 = uVar30 - *(ulonglong *)(lVar38 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar38 + 0x38)) {
    uVar34 = uVar32;
  }
  lVar22 = uVar27 * 0x30 + *(longlong *)(lVar38 + 8);
  uVar30 = *(ulonglong *)(lVar38 + 0x10) - uVar27;
  if (*(ulonglong *)(lVar38 + 0x10) < uVar27) {
    lVar22 = 8;
    uVar30 = uVar32;
  }
  lVar26 = uVar34 * 0x30 + *(longlong *)(lVar38 + 0x28);
  uVar27 = *(ulonglong *)(lVar38 + 0x30) - uVar34;
  if (*(ulonglong *)(lVar38 + 0x30) < uVar34) {
    lVar26 = 8;
    uVar27 = uVar32;
  }
  *local_130 = (*(longlong *)(lVar38 + 0x40) - uVar27) - uVar30;
  local_128 = (int *)(uVar30 * 0x30 + lVar22);
  local_120 = (int *)(uVar27 * 0x30 + lVar26);
  local_110 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  local_f8 = (longlong *)*in_R8;
  local_1d8 = (undefined4 *)in_R8[2];
  lVar38 = *in_stack_00000038;
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
    if ((lVar22 == 0) || ((int *)lVar22 == local_128)) {
      if ((lVar26 == 0) || ((int *)lVar26 == local_120)) break;
      lVar12 = 0;
      lVar37 = lVar26;
      lVar26 = lVar26 + 0x30;
    }
    else {
      lVar12 = lVar22 + 0x30;
      lVar37 = lVar22;
    }
    lVar22 = lVar12;
    *local_130 = *local_130 + 1;
    *local_1d8 = local_110;
    iVar10 = *(int *)(lVar37 + 0x24);
    local_1b0 = CONCAT44(local_1b0._4_4_,iVar10);
    if (*(longlong *)((longlong)local_f8 + 0x20) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar12 = *(longlong *)((longlong)local_f8 + 8);
      bVar39 = (byte)(extraout_RAX_02 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar39,bVar39));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar33 = 0;
      uVar30 = extraout_RAX_02;
      while( true ) {
        uVar30 = *(ulonglong *)((longlong)local_f8 + 0x10) & uVar30;
        pcVar21 = (char *)(lVar12 + uVar30);
        cVar40 = auVar42[0];
        auVar48[0] = -(*pcVar21 == cVar40);
        cVar43 = auVar42[1];
        auVar48[1] = -(pcVar21[1] == cVar43);
        cVar44 = auVar42[2];
        auVar48[2] = -(pcVar21[2] == cVar44);
        cVar45 = auVar42[3];
        auVar48[3] = -(pcVar21[3] == cVar45);
        auVar48[4] = -(pcVar21[4] == cVar40);
        auVar48[5] = -(pcVar21[5] == cVar43);
        auVar48[6] = -(pcVar21[6] == cVar44);
        auVar48[7] = -(pcVar21[7] == cVar45);
        auVar48[8] = -(pcVar21[8] == cVar40);
        auVar48[9] = -(pcVar21[9] == cVar43);
        auVar48[10] = -(pcVar21[10] == cVar44);
        auVar48[0xb] = -(pcVar21[0xb] == cVar45);
        auVar48[0xc] = -(pcVar21[0xc] == cVar40);
        auVar48[0xd] = -(pcVar21[0xd] == cVar43);
        auVar48[0xe] = -(pcVar21[0xe] == cVar44);
        auVar48[0xf] = -(pcVar21[0xf] == cVar45);
        uVar35 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe | (ushort)(auVar48[0xf] >> 7) << 0xf
        ;
        uVar25 = (uint)uVar35;
        while (uVar35 != 0) {
          uVar14 = 0;
          for (uVar15 = uVar25; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          lVar36 = (uVar14 + uVar30 & *(ulonglong *)((longlong)local_f8 + 0x10)) * -0x30;
          if (*(int *)(lVar12 + -0x30 + lVar36) == iVar10) {
            if (*(longlong *)(lVar12 + -0x18 + lVar36) == 0) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar30 = *(ulonglong *)(lVar37 + 0x10);
            if (uVar30 == 0) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar12 = lVar12 + lVar36;
            _Src = *(float **)(lVar37 + 8);
            pfVar23 = *(float **)(lVar12 + -0x20);
            fVar41 = pfVar23[2];
            fVar51 = *pfVar23;
            fVar50 = pfVar23[1];
            fVar2 = *_Src;
            fVar3 = _Src[1];
            fVar4 = _Src[2];
            fVar5 = *(float *)(lVar38 + 0x28);
            *(undefined8 *)(lVar12 + -0x18) = 0;
            if (*(ulonglong *)(lVar12 + -0x28) < uVar30) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              pfVar23 = *(float **)(lVar12 + -0x20);
              lVar33 = *(longlong *)(lVar12 + -0x18);
            }
            else {
              lVar33 = 0;
            }
            memcpy(pfVar23 + lVar33 * 3,_Src,uVar30 * 0xc);
            *(longlong *)(lVar12 + -0x18) = *(longlong *)(lVar12 + -0x18) + uVar30;
            uVar6 = *(undefined4 *)(lVar12 + -0x10);
            local_1a8 = (char *)CONCAT44(local_1a8._4_4_,*(undefined4 *)(lVar37 + 0x20));
            local_1b0 = *(undefined8 *)(lVar37 + 0x18);
            uVar7 = *(undefined4 *)(lVar12 + -8);
            *local_108 = local_e8;
            lVar12 = *(longlong *)(lVar11 + 0x30);
            uVar13 = *(undefined8 *)(lVar11 + 0x40);
            if (lVar12 == *(longlong *)(lVar11 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                        ();
            }
            lVar37 = *(longlong *)(lVar11 + 0x28);
            *(undefined8 *)(lVar37 + lVar12 * 0x28) = uVar13;
            *(undefined4 *)(lVar37 + 8 + lVar12 * 0x28) = 3;
            *(undefined8 *)(lVar37 + 0xc + lVar12 * 0x28) = local_1b0;
            *(undefined4 *)(lVar37 + 0x14 + lVar12 * 0x28) = local_1a8._0_4_;
            *(undefined4 *)(lVar37 + 0x18 + lVar12 * 0x28) = uVar6;
            *(undefined4 *)(lVar37 + 0x1c + lVar12 * 0x28) = uVar7;
            *(float *)(lVar37 + 0x20 + lVar12 * 0x28) =
                 (fVar5 * SQRT((fVar41 - fVar4) * (fVar41 - fVar4) +
                               (fVar50 - fVar3) * (fVar50 - fVar3) +
                               (fVar51 - fVar2) * (fVar51 - fVar2))) / 0.001;
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
          uVar35 = (ushort)(uVar25 - 1) & (ushort)uVar25;
          uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar35);
        }
        auVar46[0] = -(*pcVar21 == (char)iVar16);
        auVar46[1] = -(pcVar21[1] == (char)((uint)iVar16 >> 8));
        auVar46[2] = -(pcVar21[2] == (char)((uint)iVar16 >> 0x10));
        auVar46[3] = -(pcVar21[3] == (char)((uint)iVar16 >> 0x18));
        auVar46[4] = -(pcVar21[4] == (char)iVar17);
        auVar46[5] = -(pcVar21[5] == (char)((uint)iVar17 >> 8));
        auVar46[6] = -(pcVar21[6] == (char)((uint)iVar17 >> 0x10));
        auVar46[7] = -(pcVar21[7] == (char)((uint)iVar17 >> 0x18));
        auVar46[8] = -(pcVar21[8] == (char)iVar18);
        auVar46[9] = -(pcVar21[9] == (char)((uint)iVar18 >> 8));
        auVar46[10] = -(pcVar21[10] == (char)((uint)iVar18 >> 0x10));
        auVar46[0xb] = -(pcVar21[0xb] == (char)((uint)iVar18 >> 0x18));
        auVar46[0xc] = -(pcVar21[0xc] == (char)iVar19);
        auVar46[0xd] = -(pcVar21[0xd] == (char)((uint)iVar19 >> 8));
        auVar46[0xe] = -(pcVar21[0xe] == (char)((uint)iVar19 >> 0x10));
        auVar46[0xf] = -(pcVar21[0xf] == (char)((uint)iVar19 >> 0x18));
        if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x1f,0) & 1) != 0) || (SUB161(auVar46 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x37,0) & 1) != 0) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
        uVar30 = uVar30 + lVar33 + 0x10;
        lVar33 = lVar33 + 0x10;
      }
    }
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_1e8 = (int *)(lVar37 + 0x24);
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
  lVar38 = in_RCX[1];
  uVar30 = *local_130;
  uVar32 = 0;
  uVar27 = uVar30 - *(ulonglong *)(lVar38 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar38 + 0x18)) {
    uVar27 = uVar32;
  }
  uVar34 = uVar30 - *(ulonglong *)(lVar38 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar38 + 0x38)) {
    uVar34 = uVar32;
  }
  piVar24 = (int *)(uVar27 * 0x28 + *(longlong *)(lVar38 + 8));
  uVar30 = *(ulonglong *)(lVar38 + 0x10) - uVar27;
  if (*(ulonglong *)(lVar38 + 0x10) < uVar27) {
    piVar24 = (int *)&DAT_00000008;
    uVar30 = uVar32;
  }
  piVar28 = (int *)(uVar34 * 0x28 + *(longlong *)(lVar38 + 0x28));
  uVar27 = *(ulonglong *)(lVar38 + 0x30) - uVar34;
  if (*(ulonglong *)(lVar38 + 0x30) < uVar34) {
    piVar28 = (int *)&DAT_00000008;
    uVar27 = uVar32;
  }
  *local_130 = (*(longlong *)(lVar38 + 0x40) - uVar27) - uVar30;
  local_128 = piVar24 + uVar30 * 10;
  local_120 = piVar28 + uVar27 * 10;
LAB_14094e8c8:
  do {
    if ((piVar24 == (int *)0x0) || (piVar24 == local_128)) {
      if ((piVar28 == (int *)0x0) || (piVar28 == local_120)) break;
      piVar20 = (int *)0x0;
      piVar31 = piVar28;
      piVar28 = piVar28 + 10;
    }
    else {
      piVar20 = piVar24 + 10;
      piVar31 = piVar24;
    }
    piVar24 = piVar20;
    *local_130 = *local_130 + 1;
    *local_1d8 = local_110;
    iVar16 = piVar31[7];
    local_1b0 = CONCAT44(local_1b0._4_4_,iVar16);
    if (*(longlong *)((longlong)local_f8 + 0x20) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar38 = *(longlong *)((longlong)local_f8 + 8);
      bVar39 = (byte)(extraout_RAX_04 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar39,bVar39));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar22 = 0;
      uVar30 = extraout_RAX_04;
      do {
        uVar30 = *(ulonglong *)((longlong)local_f8 + 0x10) & uVar30;
        pcVar21 = (char *)(lVar38 + uVar30);
        cVar40 = auVar42[0];
        auVar49[0] = -(*pcVar21 == cVar40);
        cVar43 = auVar42[1];
        auVar49[1] = -(pcVar21[1] == cVar43);
        cVar44 = auVar42[2];
        auVar49[2] = -(pcVar21[2] == cVar44);
        cVar45 = auVar42[3];
        auVar49[3] = -(pcVar21[3] == cVar45);
        auVar49[4] = -(pcVar21[4] == cVar40);
        auVar49[5] = -(pcVar21[5] == cVar43);
        auVar49[6] = -(pcVar21[6] == cVar44);
        auVar49[7] = -(pcVar21[7] == cVar45);
        auVar49[8] = -(pcVar21[8] == cVar40);
        auVar49[9] = -(pcVar21[9] == cVar43);
        auVar49[10] = -(pcVar21[10] == cVar44);
        auVar49[0xb] = -(pcVar21[0xb] == cVar45);
        auVar49[0xc] = -(pcVar21[0xc] == cVar40);
        auVar49[0xd] = -(pcVar21[0xd] == cVar43);
        auVar49[0xe] = -(pcVar21[0xe] == cVar44);
        auVar49[0xf] = -(pcVar21[0xf] == cVar45);
        uVar35 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe | (ushort)(auVar49[0xf] >> 7) << 0xf
        ;
        uVar25 = (uint)uVar35;
        while (uVar35 != 0) {
          uVar14 = 0;
          for (uVar15 = uVar25; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          lVar26 = (uVar14 + uVar30 & *(ulonglong *)((longlong)local_f8 + 0x10)) * -0x30;
          if (*(int *)(lVar38 + -0x30 + lVar26) == iVar16) {
            lVar38 = lVar38 + lVar26;
            if (*piVar31 == 2) {
              fVar41 = *(float *)(lVar38 + -0x10);
              fVar51 = 0.0;
              if (0.0 <= (float)piVar31[1] + fVar41) {
                fVar51 = (float)piVar31[1] + fVar41;
              }
              fVar50 = 1.0;
              if (fVar51 <= 1.0) {
                fVar50 = fVar51;
              }
              *(float *)(lVar38 + -0x10) = fVar50;
              if ((fVar41 != fVar50) || (NAN(fVar41) || NAN(fVar50))) {
                local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar31[6]);
                local_1b0 = *(undefined8 *)(piVar31 + 4);
                *local_108 = local_e8;
                lVar22 = *(longlong *)(lVar11 + 0x30);
                uVar13 = *(undefined8 *)(lVar11 + 0x40);
                if (lVar22 == *(longlong *)(lVar11 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                            ();
                }
                lVar26 = *(longlong *)(lVar11 + 0x28);
                *(undefined8 *)(lVar26 + lVar22 * 0x28) = uVar13;
                *(undefined4 *)(lVar26 + 8 + lVar22 * 0x28) = 1;
                *(undefined8 *)(lVar26 + 0xc + lVar22 * 0x28) = local_1b0;
                *(undefined4 *)(lVar26 + 0x14 + lVar22 * 0x28) = local_1a8._0_4_;
                *(float *)(lVar26 + 0x18 + lVar22 * 0x28) = fVar50;
                *(float *)(lVar26 + 0x1c + lVar22 * 0x28) = ABS(fVar41 - fVar50);
                goto LAB_14094ec22;
              }
            }
            else if (*piVar31 == 1) {
              fVar41 = (float)piVar31[1];
              fVar51 = *(float *)(lVar38 + -0x10);
              *(float *)(lVar38 + -0x10) = fVar41;
              local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar31[6]);
              local_1b0 = *(undefined8 *)(piVar31 + 4);
              *local_108 = local_e8;
              lVar22 = *(longlong *)(lVar11 + 0x30);
              uVar13 = *(undefined8 *)(lVar11 + 0x40);
              if (lVar22 == *(longlong *)(lVar11 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                          ();
              }
              lVar26 = *(longlong *)(lVar11 + 0x28);
              *(undefined8 *)(lVar26 + lVar22 * 0x28) = uVar13;
              *(undefined4 *)(lVar26 + 8 + lVar22 * 0x28) = 1;
              *(undefined8 *)(lVar26 + 0xc + lVar22 * 0x28) = local_1b0;
              *(undefined4 *)(lVar26 + 0x14 + lVar22 * 0x28) = local_1a8._0_4_;
              *(float *)(lVar26 + 0x18 + lVar22 * 0x28) = fVar41;
              *(float *)(lVar26 + 0x1c + lVar22 * 0x28) = ABS(fVar41 - fVar51);
LAB_14094ec22:
              *(longlong *)(lVar11 + 0x30) = lVar22 + 1;
              *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
            }
            if (piVar31[2] != 0) {
              if (piVar31[2] == 1) {
                fVar41 = (float)piVar31[3];
                fVar51 = fVar41 - *(float *)(lVar38 + -8);
                if (((fVar41 == 0.0) && (!NAN(fVar41))) && (0.0 < *(float *)(lVar38 + -8))) {
                  uVar6 = *(undefined4 *)(lVar38 + -0x10);
                  local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar31[6]);
                  local_1b0 = *(undefined8 *)(piVar31 + 4);
                  *local_108 = local_e8;
                  lVar22 = *(longlong *)(lVar11 + 0x30);
                  uVar13 = *(undefined8 *)(lVar11 + 0x40);
                  if (lVar22 == *(longlong *)(lVar11 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                              ();
                  }
                  lVar26 = *(longlong *)(lVar11 + 0x28);
                  *(undefined8 *)(lVar26 + lVar22 * 0x28) = uVar13;
                  *(undefined4 *)(lVar26 + 8 + lVar22 * 0x28) = 4;
                  *(undefined8 *)(lVar26 + 0xc + lVar22 * 0x28) = local_1b0;
                  *(undefined4 *)(lVar26 + 0x14 + lVar22 * 0x28) = local_1a8._0_4_;
                  *(undefined4 *)(lVar26 + 0x18 + lVar22 * 0x28) = uVar6;
                  *(float *)(lVar26 + 0x1c + lVar22 * 0x28) = ABS(fVar51);
                  *(longlong *)(lVar11 + 0x30) = lVar22 + 1;
                  *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
                }
                *(float *)(lVar38 + -8) = fVar41;
                uVar6 = *(undefined4 *)(lVar38 + -0x10);
                local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar31[6]);
                local_1b0 = *(undefined8 *)(piVar31 + 4);
                *local_108 = local_e8;
                lVar38 = *(longlong *)(lVar11 + 0x30);
                uVar13 = *(undefined8 *)(lVar11 + 0x40);
                if (lVar38 == *(longlong *)(lVar11 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                            ();
                }
                lVar22 = *(longlong *)(lVar11 + 0x28);
                *(undefined8 *)(lVar22 + lVar38 * 0x28) = uVar13;
                *(undefined4 *)(lVar22 + 8 + lVar38 * 0x28) = 2;
                *(undefined8 *)(lVar22 + 0xc + lVar38 * 0x28) = local_1b0;
                *(undefined4 *)(lVar22 + 0x14 + lVar38 * 0x28) = local_1a8._0_4_;
                *(undefined4 *)(lVar22 + 0x18 + lVar38 * 0x28) = uVar6;
                *(float *)(lVar22 + 0x1c + lVar38 * 0x28) = fVar41;
                *(float *)(lVar22 + 0x20 + lVar38 * 0x28) = ABS(fVar51);
LAB_14094ef10:
                *(longlong *)(lVar11 + 0x30) = lVar38 + 1;
                *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
              }
              else {
                fVar41 = (float)piVar31[3];
                fVar51 = *(float *)(lVar38 + -8);
                if (((0.0 < fVar41) && (fVar51 == 0.0)) && (!NAN(fVar51))) {
                  *(undefined4 *)(lVar38 + -8) = 0x3f800000;
                  fVar51 = 1.0;
                }
                fVar41 = fVar41 + fVar51;
                fVar50 = 0.0;
                if (0.0 <= fVar41) {
                  fVar50 = fVar41;
                }
                fVar41 = 20.0;
                if (fVar50 <= 20.0) {
                  fVar41 = fVar50;
                }
                fVar41 = (float)(~-(uint)(fVar41 < 1.0) & (uint)fVar41);
                *(float *)(lVar38 + -8) = fVar41;
                if ((fVar51 != fVar41) || (NAN(fVar51) || NAN(fVar41))) {
                  uVar6 = *(undefined4 *)(lVar38 + -0x10);
                  local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar31[6]);
                  local_1b0 = *(undefined8 *)(piVar31 + 4);
                  *local_108 = local_e8;
                  lVar22 = *(longlong *)(lVar11 + 0x30);
                  uVar13 = *(undefined8 *)(lVar11 + 0x40);
                  if (lVar22 == *(longlong *)(lVar11 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                              ();
                  }
                  lVar26 = *(longlong *)(lVar11 + 0x28);
                  *(undefined8 *)(lVar26 + lVar22 * 0x28) = uVar13;
                  *(undefined4 *)(lVar26 + 8 + lVar22 * 0x28) = 2;
                  *(undefined8 *)(lVar26 + 0xc + lVar22 * 0x28) = local_1b0;
                  *(undefined4 *)(lVar26 + 0x14 + lVar22 * 0x28) = local_1a8._0_4_;
                  *(undefined4 *)(lVar26 + 0x18 + lVar22 * 0x28) = uVar6;
                  *(float *)(lVar26 + 0x1c + lVar22 * 0x28) = fVar41;
                  *(float *)(lVar26 + 0x20 + lVar22 * 0x28) = ABS(fVar51 - fVar41);
                  *(longlong *)(lVar11 + 0x30) = lVar22 + 1;
                  *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
                }
                if (0.0 < fVar51) {
                  fVar41 = *(float *)(lVar38 + -8);
                  if ((fVar41 == 0.0) && (!NAN(fVar41))) {
                    uVar6 = *(undefined4 *)(lVar38 + -0x10);
                    local_1a8 = (char *)CONCAT44(local_1a8._4_4_,piVar31[6]);
                    local_1b0 = *(undefined8 *)(piVar31 + 4);
                    *local_108 = local_e8;
                    lVar38 = *(longlong *)(lVar11 + 0x30);
                    uVar13 = *(undefined8 *)(lVar11 + 0x40);
                    if (lVar38 == *(longlong *)(lVar11 + 0x20)) {
                      alloc::raw_vec::RawVec::
                      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                                ();
                    }
                    lVar22 = *(longlong *)(lVar11 + 0x28);
                    *(undefined8 *)(lVar22 + lVar38 * 0x28) = uVar13;
                    *(undefined4 *)(lVar22 + 8 + lVar38 * 0x28) = 4;
                    *(undefined8 *)(lVar22 + 0xc + lVar38 * 0x28) = local_1b0;
                    *(undefined4 *)(lVar22 + 0x14 + lVar38 * 0x28) = local_1a8._0_4_;
                    *(undefined4 *)(lVar22 + 0x18 + lVar38 * 0x28) = uVar6;
                    *(float *)(lVar22 + 0x1c + lVar38 * 0x28) = ABS(fVar51 - fVar41);
                    goto LAB_14094ef10;
                  }
                }
              }
            }
            *local_1c8 = local_10c;
            lVar38 = *(longlong *)(local_118 + 0x30);
            uVar13 = *(undefined8 *)(local_118 + 0x40);
            if (lVar38 == *(longlong *)(local_118 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                        ();
            }
            *(undefined8 *)(*(longlong *)(local_118 + 0x28) + lVar38 * 8) = uVar13;
            *(longlong *)(local_118 + 0x30) = lVar38 + 1;
            *(longlong *)(local_118 + 0x40) = *(longlong *)(local_118 + 0x40) + 1;
            LOCK();
            *local_1c0 = *local_1c0 + 1;
            UNLOCK();
            goto LAB_14094e8c8;
          }
          uVar35 = (ushort)(uVar25 - 1) & (ushort)uVar25;
          uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar35);
        }
        auVar47[0] = -(*pcVar21 == -1);
        auVar47[1] = -(pcVar21[1] == -1);
        auVar47[2] = -(pcVar21[2] == -1);
        auVar47[3] = -(pcVar21[3] == -1);
        auVar47[4] = -(pcVar21[4] == -1);
        auVar47[5] = -(pcVar21[5] == -1);
        auVar47[6] = -(pcVar21[6] == -1);
        auVar47[7] = -(pcVar21[7] == -1);
        auVar47[8] = -(pcVar21[8] == -1);
        auVar47[9] = -(pcVar21[9] == -1);
        auVar47[10] = -(pcVar21[10] == -1);
        auVar47[0xb] = -(pcVar21[0xb] == -1);
        auVar47[0xc] = -(pcVar21[0xc] == -1);
        auVar47[0xd] = -(pcVar21[0xd] == -1);
        auVar47[0xe] = -(pcVar21[0xe] == -1);
        auVar47[0xf] = -(pcVar21[0xf] == -1);
        if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 || (SUB161(auVar47 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar47 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar47 >> 0x1f,0) & 1) != 0) || (SUB161(auVar47 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar47 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar47 >> 0x37,0) & 1) != 0) || (SUB161(auVar47 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar47 >> 0x47,0) & 1) != 0) || (SUB161(auVar47 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar47 >> 0x57,0) & 1) != 0) || (SUB161(auVar47 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar47 >> 0x67,0) & 1) != 0) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') break;
        uVar30 = uVar30 + lVar22 + 0x10;
        lVar22 = lVar22 + 0x10;
      } while( true );
    }
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_1e8 = piVar31 + 7;
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
      plVar29 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar29 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_14094f032:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar29 != 0) {
LAB_14094f03e:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar29 = -1;
    local_f8 = plVar29;
    puffin::ThreadProfiler::end_scope();
    *local_f8 = *local_f8 + 1;
  }
  return;
}


