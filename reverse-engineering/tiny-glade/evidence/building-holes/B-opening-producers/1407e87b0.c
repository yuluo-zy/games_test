
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  undefined4 *puVar3;
  float *pfVar4;
  longlong lVar5;
  byte bVar6;
  byte bVar7;
  undefined4 uVar8;
  undefined4 *puVar9;
  longlong lVar10;
  code *pcVar11;
  int iVar12;
  undefined8 uVar13;
  longlong lVar14;
  undefined8 uVar15;
  float fVar16;
  char extraout_AL;
  byte extraout_AL_00;
  undefined1 extraout_AL_01;
  byte bVar17;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  uint uVar18;
  uint uVar19;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  float *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined4 *extraout_RAX_07;
  undefined8 extraout_RAX_08;
  ulonglong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  undefined4 *extraout_RAX_14;
  undefined4 *extraout_RAX_15;
  undefined4 *extraout_RAX_16;
  longlong *extraout_RAX_17;
  longlong *extraout_RAX_18;
  longlong *extraout_RAX_19;
  longlong in_RCX;
  longlong *plVar20;
  longlong lVar21;
  char *pcVar22;
  longlong lVar23;
  uint uVar24;
  longlong *in_RDX;
  undefined8 uVar25;
  undefined1 (*pauVar26) [16];
  longlong lVar27;
  longlong *plVar28;
  ulonglong uVar29;
  longlong *in_R8;
  undefined *puVar30;
  longlong lVar31;
  undefined8 uVar32;
  ulonglong uVar33;
  longlong in_R9;
  ushort uVar34;
  ulonglong uVar35;
  longlong lVar36;
  undefined1 (*pauVar37) [16];
  longlong lVar38;
  longlong lVar39;
  char cVar40;
  char cVar45;
  float fVar41;
  char cVar43;
  char cVar44;
  undefined1 auVar42 [16];
  float fVar46;
  float fVar47;
  float fVar48;
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  float fVar53;
  float fVar54;
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  float fVar55;
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  float fVar60;
  undefined4 uVar61;
  undefined4 uVar62;
  undefined4 uVar63;
  undefined4 uVar64;
  float fVar65;
  float fVar66;
  float fVar67;
  float local_248;
  float fStack_244;
  undefined2 local_22e;
  undefined1 local_22c;
  undefined2 local_22b;
  undefined1 local_229;
  undefined8 local_228;
  undefined4 local_220;
  undefined8 local_21c;
  undefined4 local_214;
  undefined7 local_210;
  undefined1 uStack_209;
  undefined4 local_208;
  undefined3 uStack_204;
  undefined2 local_1e4;
  undefined1 uStack_1e2;
  undefined5 uStack_1e1;
  undefined4 local_1dc;
  uint7 local_1d8;
  undefined1 uStack_1d1;
  float fStack_1d0;
  uint uStack_1cc;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  uint local_1b8;
  uint local_1b4;
  longlong *local_1b0;
  longlong local_1a8;
  longlong local_1a0;
  float local_198;
  undefined4 uStack_194;
  float local_190;
  undefined3 uStack_18c;
  float local_188;
  float fStack_184;
  float fStack_180;
  undefined4 uStack_17c;
  uint7 *local_178;
  ushort uStack_170;
  undefined2 uStack_16e;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  float fStack_160;
  float fStack_15c;
  float local_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  longlong *local_140;
  undefined4 local_134;
  char local_130;
  char cStack_12f;
  undefined2 uStack_12e;
  undefined4 uStack_12c;
  float fStack_128;
  uint uStack_124;
  undefined8 local_120;
  longlong local_118;
  longlong local_110;
  longlong *local_108;
  longlong *local_100;
  undefined1 local_f4;
  char local_f3;
  undefined1 local_f2;
  byte local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_f3 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    plVar20 = (longlong *)&DAT_142a89878;
    if (extraout_RAX == 1) {
      if (in_RDX != (longlong *)0x0) {
        if (in_RDX < (longlong *)0x54) {
          if ((char)in_RDX[0x2855130f] < -0x40) {
LAB_1407e8880:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != (longlong *)&DAT_00000054) goto LAB_1407e8880;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (in_RDX != (longlong *)0xfffffffffffffffe) {
          if ((undefined1 *)((longlong)in_RDX + 2) < (undefined1 *)0x54) {
            if (*(char *)((longlong)in_RDX + 0x142a8987a) < -0x40) {
LAB_1407e88cd:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if ((undefined1 *)((longlong)in_RDX + 2) != (undefined1 *)0x54) goto LAB_1407e88cd;
        }
        plVar20 = (longlong *)((longlong)in_RDX + 0x142a8987a);
      }
    }
    in_RDX = plVar20;
    puVar30 = &DAT_142a89a01;
    do {
      bVar17 = puVar30[-1];
      uVar18 = (uint)(char)bVar17;
      if ((char)bVar17 < '\0') {
        bVar6 = puVar30[-2];
        if ((char)bVar6 < -0x40) {
          bVar7 = puVar30[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = puVar30 + -4;
            puVar30 = puVar30 + -4;
            uVar18 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar30 = puVar30 + -3;
            uVar18 = bVar7 & 0xf;
          }
          uVar18 = bVar6 & 0x3f | uVar18 << 6;
        }
        else {
          puVar30 = puVar30 + -2;
          uVar18 = bVar6 & 0x1f;
        }
        uVar18 = (uint)(bVar17 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto LAB_1407e8994;
LAB_1407e89ac:
        if (puVar30 == &DAT_142a899bd) break;
        if (puVar30 + -0x142a899bd < (undefined *)0x43) {
          if (-0x41 < (char)puVar30[1]) break;
        }
        else if (puVar30 + -0x142a899bd == (undefined *)0x43) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar30 = puVar30 + -1;
      if (uVar18 == 0x5c) goto LAB_1407e89ac;
LAB_1407e8994:
      if (uVar18 == 0x2f) goto LAB_1407e89ac;
    } while (puVar30 != &DAT_142a899be);
    local_1b0 = (longlong *)in_RCX;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar20 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar20 = extraout_RAX_18, extraout_RAX_18 == (longlong *)0x0))
    goto LAB_1407ea95d;
    if (*plVar20 != 0) goto LAB_1407ea969;
    *plVar20 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar20 = *plVar20 + 1;
  }
  country_core::resources::walls::decorator_subtype::impl$8::from();
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  lVar38 = CONCAT44(fStack_184,local_188);
  pauVar26 = (undefined1 (*) [16])CONCAT44(uStack_17c,fStack_180);
  lVar5 = CONCAT44(uStack_164,uStack_168);
  plVar20 = (longlong *)*in_R8;
  uVar8 = *(undefined4 *)(in_R9 + 0x1c);
  puVar9 = *(undefined4 **)(in_R9 + 0x10);
  uVar18 = (uint)uStack_170;
joined_r0x0001407e8ada:
  if (lVar5 != 0) {
    if ((short)uVar18 == 0) {
      do {
        auVar42 = *pauVar26;
        uVar34 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
        lVar38 = lVar38 + -0x980;
        pauVar26 = pauVar26 + 1;
      } while (uVar34 == 0xffff);
      uVar18 = ~(uint)uVar34;
      local_1b4 = uVar18 & -(uint)uVar34 - 2;
    }
    else {
      if (lVar38 == 0) goto LAB_1407ea761;
      local_1b4 = uVar18 - 1 & uVar18;
    }
    uVar19 = 0;
    for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
      uVar19 = uVar19 + 1;
    }
    lVar10 = *(longlong *)(lVar38 + -0x98 + (ulonglong)uVar19 * -0x98);
    country_core::resources::walls::decorator_storage::Decorators::iter_modified();
    lVar5 = lVar5 + -1;
    lVar36 = CONCAT44(fStack_184,local_188);
    pauVar37 = (undefined1 (*) [16])CONCAT44(uStack_17c,fStack_180);
    local_1b0 = (longlong *)CONCAT44(uStack_164,uStack_168);
    lVar14 = CONCAT44(fStack_15c,fStack_160);
    uVar19 = (uint)uStack_170;
LAB_1407e8b9e:
    uVar18 = local_1b4;
    if (local_1b0 != (longlong *)0x0) {
      if ((short)uVar19 == 0) {
        do {
          auVar42 = *pauVar37;
          uVar34 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
          lVar36 = lVar36 + -0x40;
          pauVar37 = pauVar37 + 1;
        } while (uVar34 == 0xffff);
        uVar19 = ~(uint)uVar34;
        uVar24 = uVar19 & -(uint)uVar34 - 2;
      }
      else {
        uVar24 = uVar19 - 1 & uVar19;
        if (lVar36 == 0) goto joined_r0x0001407e8ada;
      }
      iVar12 = 0;
      for (; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
        iVar12 = iVar12 + 1;
      }
      local_188 = *(float *)((lVar36 - (ulonglong)(uint)(iVar12 << 2)) + -4);
      local_1b8 = uVar24;
      if (*(longlong *)(lVar14 + 0x48) != 0) {
        local_108 = (longlong *)(ulonglong)(uint)local_188;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        local_1b0 = (longlong *)((longlong)local_1b0 + -1);
        lVar27 = *(longlong *)(lVar14 + 0x30);
        bVar17 = (byte)(extraout_RAX_02 >> 0x39);
        auVar42 = ZEXT216(CONCAT11(bVar17,bVar17));
        auVar42 = pshuflw(auVar42,auVar42,0);
        lVar31 = 0;
        uVar35 = extraout_RAX_02;
        while( true ) {
          uVar35 = *(ulonglong *)(lVar14 + 0x38) & uVar35;
          pcVar22 = (char *)(lVar27 + uVar35);
          cVar40 = auVar42[0];
          auVar56[0] = -(*pcVar22 == cVar40);
          cVar43 = auVar42[1];
          auVar56[1] = -(pcVar22[1] == cVar43);
          cVar44 = auVar42[2];
          auVar56[2] = -(pcVar22[2] == cVar44);
          cVar45 = auVar42[3];
          auVar56[3] = -(pcVar22[3] == cVar45);
          auVar56[4] = -(pcVar22[4] == cVar40);
          auVar56[5] = -(pcVar22[5] == cVar43);
          auVar56[6] = -(pcVar22[6] == cVar44);
          auVar56[7] = -(pcVar22[7] == cVar45);
          auVar56[8] = -(pcVar22[8] == cVar40);
          auVar56[9] = -(pcVar22[9] == cVar43);
          auVar56[10] = -(pcVar22[10] == cVar44);
          auVar56[0xb] = -(pcVar22[0xb] == cVar45);
          auVar56[0xc] = -(pcVar22[0xc] == cVar40);
          auVar56[0xd] = -(pcVar22[0xd] == cVar43);
          auVar56[0xe] = -(pcVar22[0xe] == cVar44);
          auVar56[0xf] = -(pcVar22[0xf] == cVar45);
          uVar34 = (ushort)(SUB161(auVar56 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar56 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar56 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar56 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar56 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar56 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar56 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar56 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar56 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar56 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar56 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar56 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar56 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar56 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar56 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar56[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar34;
          while (uVar34 != 0) {
            uVar19 = 0;
            for (uVar24 = uVar18; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
              uVar19 = uVar19 + 1;
            }
            lVar39 = (uVar19 + uVar35 & *(ulonglong *)(lVar14 + 0x38)) * -0x50;
            if ((int)local_108 == *(int *)(lVar27 + -0x50 + lVar39)) {
              country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
              uVar19 = local_1b8;
              if (((extraout_AL_00 & 0x1e) != 6) ||
                 (local_110 = lVar27 + lVar39, 1 < *(byte *)(local_110 + -0x26)))
              goto LAB_1407e8b9e;
              if (plVar20[3] == 0) goto LAB_1407ea825;
              local_f1 = extraout_AL_00;
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
              lVar27 = *plVar20;
              bVar17 = (byte)(extraout_RAX_03 >> 0x39);
              auVar42 = ZEXT216(CONCAT11(bVar17,bVar17));
              auVar42 = pshuflw(auVar42,auVar42,0);
              lVar31 = 0;
              uVar35 = extraout_RAX_03;
              goto LAB_1407e8d83;
            }
            uVar34 = (ushort)(uVar18 - 1) & (ushort)uVar18;
            uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar34);
          }
          auVar49[0] = -(*pcVar22 == __xmm_ffffffffffffffffffffffffffffffff);
          auVar49[1] = -(pcVar22[1] == UNK_142925991);
          auVar49[2] = -(pcVar22[2] == UNK_142925992);
          auVar49[3] = -(pcVar22[3] == UNK_142925993);
          auVar49[4] = -(pcVar22[4] == UNK_142925994);
          auVar49[5] = -(pcVar22[5] == UNK_142925995);
          auVar49[6] = -(pcVar22[6] == UNK_142925996);
          auVar49[7] = -(pcVar22[7] == UNK_142925997);
          auVar49[8] = -(pcVar22[8] == UNK_142925998);
          auVar49[9] = -(pcVar22[9] == UNK_142925999);
          auVar49[10] = -(pcVar22[10] == UNK_14292599a);
          auVar49[0xb] = -(pcVar22[0xb] == UNK_14292599b);
          auVar49[0xc] = -(pcVar22[0xc] == UNK_14292599c);
          auVar49[0xd] = -(pcVar22[0xd] == UNK_14292599d);
          auVar49[0xe] = -(pcVar22[0xe] == UNK_14292599e);
          auVar49[0xf] = -(pcVar22[0xf] == UNK_14292599f);
          if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar49 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar49 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x2f,0) & 1) != 0) || (SUB161(auVar49 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
          uVar35 = uVar35 + lVar31 + 0x10;
          lVar31 = lVar31 + 0x10;
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    goto joined_r0x0001407e8ada;
  }
LAB_1407ea761:
  if (local_f3 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_17 == 1) {
      plVar20 = extraout_RAX_17 + 1;
    }
    else if ((*extraout_RAX_17 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar20 = extraout_RAX_19, extraout_RAX_19 == (longlong *)0x0)) {
LAB_1407ea95d:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar20 != 0) {
LAB_1407ea969:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar20 = -1;
    local_1b0 = plVar20;
    puffin::ThreadProfiler::end_scope();
    *local_1b0 = *local_1b0 + 1;
  }
  return;
LAB_1407e8d83:
  uVar35 = plVar20[1] & uVar35;
  pcVar22 = (char *)(lVar27 + uVar35);
  cVar40 = auVar42[0];
  auVar57[0] = -(*pcVar22 == cVar40);
  cVar43 = auVar42[1];
  auVar57[1] = -(pcVar22[1] == cVar43);
  cVar44 = auVar42[2];
  auVar57[2] = -(pcVar22[2] == cVar44);
  cVar45 = auVar42[3];
  auVar57[3] = -(pcVar22[3] == cVar45);
  auVar57[4] = -(pcVar22[4] == cVar40);
  auVar57[5] = -(pcVar22[5] == cVar43);
  auVar57[6] = -(pcVar22[6] == cVar44);
  auVar57[7] = -(pcVar22[7] == cVar45);
  auVar57[8] = -(pcVar22[8] == cVar40);
  auVar57[9] = -(pcVar22[9] == cVar43);
  auVar57[10] = -(pcVar22[10] == cVar44);
  auVar57[0xb] = -(pcVar22[0xb] == cVar45);
  auVar57[0xc] = -(pcVar22[0xc] == cVar40);
  auVar57[0xd] = -(pcVar22[0xd] == cVar43);
  auVar57[0xe] = -(pcVar22[0xe] == cVar44);
  auVar57[0xf] = -(pcVar22[0xf] == cVar45);
  uVar34 = (ushort)(SUB161(auVar57 >> 7,0) & 1) | (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe | (ushort)(auVar57[0xf] >> 7) << 0xf;
  uVar18 = (uint)uVar34;
  while (uVar34 != 0) {
    uVar19 = 0;
    for (uVar24 = uVar18; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
      uVar19 = uVar19 + 1;
    }
    local_118 = (uVar19 + uVar35 & plVar20[1]) * -0x2a8;
    if (lVar10 == *(longlong *)(lVar27 + -0x2a8 + local_118)) {
      country_core::resources::walls::decorator::DecoratorRank::new();
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      std::collections::hash::map::HashMap::
      get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                ();
      if (extraout_RAX_04 == 0) {
        local_1d8 = (uint7)&local_130;
        uStack_1d1 = (undefined1)((ulonglong)&local_130 >> 0x38);
        fStack_1d0 = 10.363022;
        uStack_1cc = 1;
        local_188 = 97.055664;
        fStack_184 = 1.4013e-45;
        fStack_180 = 1.4013e-45;
        uStack_17c = 0.0;
        uStack_168 = 0;
        uStack_164 = 0;
        local_178 = &local_1d8;
        uStack_170 = 1;
        uStack_16e = 0;
        uStack_16c = 0;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      fVar47 = __real_3ff64e8b;
      if (*(char *)(local_110 + -4) != '\x03') {
        fVar47 = *(float *)(&__real_3fbf3e033f738716 +
                           (ulonglong)(*(char *)(local_110 + -4) == '\x02') * 4);
      }
      country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
      fVar66 = *extraout_RAX_05;
      get_gothic_hat_bones();
      uVar64 = uStack_150;
      uVar63 = uStack_154;
      fVar54 = local_158;
      fVar53 = fStack_15c;
      fVar65 = fStack_160;
      uVar62 = uStack_164;
      uVar61 = uStack_168;
      fVar41 = (float)CONCAT22(uStack_16e,uStack_170);
      uVar13 = CONCAT44(uStack_168,uStack_16c);
      uVar25 = CONCAT44(fStack_184,local_188);
      uVar32 = CONCAT44(uStack_17c,fStack_180);
      fVar55 = SUB84(local_178,0);
      fVar48 = (float)((ulonglong)local_178 >> 0x20);
      uVar15 = CONCAT44(fStack_15c,fStack_160);
      uStack_12c = fStack_160 - fVar55;
      fVar46 = local_158 - fVar41;
      fVar60 = __real_3f800000 / SQRT(fVar46 * fVar46 + uStack_12c * uStack_12c);
      uStack_12c = uStack_12c * fVar60;
      fVar46 = (float)((uint)fVar60 ^ ___xmm_80000000800000008000000080000000) * fVar46;
      local_130 = SUB41(fVar46,0);
      cStack_12f = (char)((uint)fVar46 >> 8);
      uStack_12e = (undefined2)((uint)fVar46 >> 0x10);
      ::utils::swizzlers::impl$0::x0y();
      ::utils::xz_projective_rotation_bad();
      fVar67 = uStack_17c;
      fVar16 = fStack_180;
      fVar60 = fStack_184;
      fVar46 = local_188;
      *puVar9 = uVar8;
      country_core::resources::rt::impl$5::from();
      blueprint::DecoratorBlueprints::insert();
      lVar27 = lVar27 + local_118;
      local_1a8 = CONCAT71(local_1a8._1_7_,*(undefined1 *)(local_110 + -1));
      local_140 = (longlong *)CONCAT44(local_140._4_4_,*(undefined4 *)(local_110 + -0x20));
      local_100 = extraout_RAX_06;
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      local_1a0 = CONCAT71(local_1a0._1_7_,extraout_AL_01);
      local_134 = *(undefined4 *)(local_110 + -0x1c);
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      local_248 = fVar65 * (float)__xmm_00000000000000003f0000003f000000 +
                  fVar55 * (float)__xmm_00000000000000003f0000003f000000;
      fStack_244 = fVar53 * __xmm_00000000000000003f0000003f000000._4_4_ +
                   fVar48 * __xmm_00000000000000003f0000003f000000._4_4_;
      local_118 = CONCAT44(local_118._4_4_,fVar54 * __real_3f000000 + fVar41 * __real_3f000000);
      fVar65 = fVar66 - fVar47 * __real_3f000000;
      fVar66 = fVar47 * __real_3f000000 + fVar66;
      bVar17 = local_f1 == 7 | (byte)local_1a0;
      cVar40 = *(char *)(local_110 + -4);
      lVar31 = (ulonglong)bVar17 * 5 + 0x20;
      if (cVar40 == '\x01') {
        puVar30 = &DAT_142a89a38;
        if (bVar17 != 0) {
          puVar30 = &DAT_142a89a98;
        }
        uStack_17c = (float)((uint)uStack_17c._3_1_ << 0x18);
        local_188 = (float)((uint)local_188 & 0xffffff);
        fStack_184 = 0.0;
        fStack_180 = 0.0;
        fStack_128 = 0.0;
        uStack_124 = uStack_124 & 0xff000000;
        local_130 = SUB41(local_188,0);
        cStack_12f = (char)((uint)local_188 >> 8);
        uStack_12e = (undefined2)((uint)local_188 >> 0x10);
        uStack_12c = 0.0;
        lVar39 = local_100[0x18];
        if (lVar39 == local_100[0x16]) {
          alloc::raw_vec::RawVec::
          grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                    ();
        }
        lVar23 = local_100[0x17];
        lVar21 = lVar39 * 0x160;
        *(undefined8 *)(lVar23 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x10 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x20 + lVar21) = 0;
        *(undefined4 *)(lVar23 + 0x30 + lVar21) = 0;
        *(ulonglong *)(lVar23 + 0x34 + lVar21) = CONCAT17(uStack_209,local_210);
        *(undefined4 *)(lVar23 + 0x3c + lVar21) = local_208;
        *(undefined4 *)(lVar23 + 0x40 + lVar21) = 0;
        puVar2 = (undefined8 *)(lVar23 + 0x44 + lVar21);
        *puVar2 = CONCAT44(fStack_184,local_188);
        puVar2[1] = CONCAT44(uStack_17c,fStack_180);
        puVar2 = (undefined8 *)(lVar23 + 0x54 + lVar21);
        *puVar2 = local_178;
        puVar2[1] = CONCAT44(uStack_16c,CONCAT22(uStack_16e,uStack_170));
        puVar2 = (undefined8 *)(lVar23 + 100 + lVar21);
        *puVar2 = CONCAT44(uStack_164,uStack_168);
        puVar2[1] = CONCAT44(fStack_15c,fStack_160);
        puVar2 = (undefined8 *)(lVar23 + 0x70 + lVar21);
        *puVar2 = CONCAT44(local_158,fStack_15c);
        puVar2[1] = CONCAT44(uStack_150,uStack_154);
        *(undefined4 *)(lVar23 + 0x80 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x84 + lVar21) = local_228;
        *(undefined4 *)(lVar23 + 0x8c + lVar21) = local_220;
        *(undefined4 *)(lVar23 + 0x90 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x94 + lVar21) = local_21c;
        *(undefined4 *)(lVar23 + 0x9c + lVar21) = local_214;
        *(undefined **)(lVar23 + 0xa0 + lVar21) = puVar30;
        *(longlong *)(lVar23 + 0xa8 + lVar21) = lVar31;
        *(char **)(lVar23 + 0xb0 + lVar21) = "nani_solid_vertex_color_bricks";
        *(undefined8 *)(lVar23 + 0xb8 + lVar21) = 0x1e;
        pfVar4 = (float *)(lVar23 + 0xc0 + lVar21);
        *pfVar4 = fVar46;
        pfVar4[1] = fVar60;
        pfVar4[2] = fVar16;
        pfVar4[3] = fVar67;
        *(ulonglong *)(lVar23 + 0xd0 + lVar21) = CONCAT44(fStack_244,local_248);
        *(float *)(lVar23 + 0xd8 + lVar21) = (float)local_118;
        *(undefined8 *)(lVar23 + 0xdc + lVar21) = 0x3f8000003f800000;
        *(undefined4 *)(lVar23 + 0xe4 + lVar21) = 0x3f800000;
        *(undefined4 *)(lVar23 + 0xf0 + lVar21) = 1;
        *(int *)(lVar23 + 0xf4 + lVar21) = (int)local_140;
        *(undefined4 *)(lVar23 + 0xf8 + lVar21) = 0;
        *(undefined4 *)(lVar23 + 0x100 + lVar21) = local_134;
        *(undefined4 *)(lVar23 + 0x104 + lVar21) = extraout_EAX;
        *(float *)(lVar23 + 0x108 + lVar21) = fVar65;
        *(float *)(lVar23 + 0x10c + lVar21) = fVar66;
        *(float *)(lVar23 + 0x110 + lVar21) = fVar47;
        *(byte *)(lVar23 + 0x114 + lVar21) = (byte)local_1a0;
        *(undefined1 *)(lVar23 + 0x117 + lVar21) = uStack_1e2;
        *(undefined2 *)(lVar23 + 0x115 + lVar21) = local_1e4;
        *(undefined4 *)(lVar23 + 0x118 + lVar21) = 0;
        puVar2 = (undefined8 *)(lVar23 + 0x11c + lVar21);
        *puVar2 = CONCAT17(uStack_1d1,local_1d8);
        puVar2[1] = CONCAT44(uStack_1cc,fStack_1d0);
        puVar2 = (undefined8 *)(lVar23 + 300 + lVar21);
        *puVar2 = local_1c8;
        puVar2[1] = uStack_1c0;
        *(undefined1 *)(lVar23 + 0x13c + lVar21) = 2;
        *(ulonglong *)(lVar23 + 0x13d + lVar21) =
             CONCAT44(uStack_12c,CONCAT22(uStack_12e,CONCAT11(cStack_12f,local_130)));
        *(ulonglong *)(lVar23 + 0x144 + lVar21) =
             CONCAT35((undefined3)uStack_124,CONCAT41(fStack_128,uStack_12c._3_1_));
        *(undefined1 *)(lVar23 + 0x14c + lVar21) = 2;
        *(undefined1 *)(lVar23 + 0x14e + lVar21) = (undefined1)local_1a8;
        *(undefined2 *)(lVar23 + 0x14f + lVar21) = 0x900;
        uVar25 = CONCAT44(uStack_194,local_198);
        uVar32 = CONCAT35(uStack_18c,CONCAT41(local_190,uStack_194._3_1_));
      }
      else {
        if (cVar40 == '\x02') {
          puVar30 = &DAT_142a89a58;
          if (bVar17 != 0) {
            puVar30 = &DAT_142a89abd;
          }
        }
        else {
          uVar19 = local_1b8;
          if (cVar40 != '\x03') goto LAB_1407e8b9e;
          puVar30 = &DAT_142a89a78;
          if (bVar17 != 0) {
            puVar30 = &DAT_142a89ae2;
          }
        }
        uStack_17c = (float)((uint)uStack_17c._3_1_ << 0x18);
        local_188 = (float)((uint)local_188 & 0xffffff);
        fStack_184 = 0.0;
        fStack_180 = 0.0;
        fStack_1d0 = 0.0;
        uStack_1cc = uStack_1cc & 0xff000000;
        local_1d8 = (uint7)(uint)local_188;
        uStack_1d1 = 0;
        local_208 = uVar62;
        local_210 = (undefined7)uVar13;
        uStack_209 = (undefined1)((uint)uVar61 >> 0x18);
        lVar39 = local_100[0x18];
        if (lVar39 == local_100[0x16]) {
          alloc::raw_vec::RawVec::
          grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                    ();
        }
        lVar23 = local_100[0x17];
        lVar21 = lVar39 * 0x160;
        *(undefined8 *)(lVar23 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x10 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x20 + lVar21) = 0;
        *(undefined4 *)(lVar23 + 0x30 + lVar21) = 0;
        *(ulonglong *)(lVar23 + 0x34 + lVar21) = CONCAT44(uStack_194,local_198);
        *(float *)(lVar23 + 0x3c + lVar21) = local_190;
        *(undefined4 *)(lVar23 + 0x40 + lVar21) = 1;
        puVar2 = (undefined8 *)(lVar23 + 0x44 + lVar21);
        *puVar2 = uVar25;
        puVar2[1] = uVar32;
        puVar3 = (undefined4 *)(lVar23 + 0x54 + lVar21);
        *puVar3 = uVar63;
        puVar3[1] = uVar64;
        puVar3[2] = fVar55;
        puVar3[3] = fVar48;
        *(float *)(lVar23 + 100 + lVar21) = fVar41;
        *(ulonglong *)(lVar23 + 0x68 + lVar21) = CONCAT17(uStack_209,local_210);
        *(undefined4 *)(lVar23 + 0x70 + lVar21) = uVar62;
        *(undefined8 *)(lVar23 + 0x74 + lVar21) = uVar15;
        *(float *)(lVar23 + 0x7c + lVar21) = fVar54;
        *(undefined4 *)(lVar23 + 0x80 + lVar21) = 0;
        *(undefined8 *)(lVar23 + 0x84 + lVar21) = local_228;
        *(undefined4 *)(lVar23 + 0x8c + lVar21) = local_220;
        *(undefined4 *)(lVar23 + 0x90 + lVar21) = 0;
        *(undefined4 *)(lVar23 + 0x9c + lVar21) = local_214;
        *(undefined8 *)(lVar23 + 0x94 + lVar21) = local_21c;
        *(undefined **)(lVar23 + 0xa0 + lVar21) = puVar30;
        *(longlong *)(lVar23 + 0xa8 + lVar21) = lVar31;
        *(char **)(lVar23 + 0xb0 + lVar21) = "nani_gothic_window_bricks";
        *(undefined8 *)(lVar23 + 0xb8 + lVar21) = 0x19;
        pfVar4 = (float *)(lVar23 + 0xc0 + lVar21);
        *pfVar4 = fVar46;
        pfVar4[1] = fVar60;
        pfVar4[2] = fVar16;
        pfVar4[3] = fVar67;
        *(ulonglong *)(lVar23 + 0xd0 + lVar21) = CONCAT44(fStack_244,local_248);
        *(float *)(lVar23 + 0xd8 + lVar21) = (float)local_118;
        *(undefined8 *)(lVar23 + 0xdc + lVar21) = 0x3f8000003f800000;
        *(undefined4 *)(lVar23 + 0xe4 + lVar21) = 0x3f800000;
        *(undefined4 *)(lVar23 + 0xf0 + lVar21) = 1;
        *(int *)(lVar23 + 0xf4 + lVar21) = (int)local_140;
        *(undefined4 *)(lVar23 + 0xf8 + lVar21) = 0;
        *(undefined4 *)(lVar23 + 0x100 + lVar21) = local_134;
        *(undefined4 *)(lVar23 + 0x104 + lVar21) = extraout_EAX;
        *(float *)(lVar23 + 0x108 + lVar21) = fVar65;
        *(float *)(lVar23 + 0x10c + lVar21) = fVar66;
        *(float *)(lVar23 + 0x110 + lVar21) = fVar47;
        *(byte *)(lVar23 + 0x114 + lVar21) = (byte)local_1a0;
        *(undefined1 *)(lVar23 + 0x117 + lVar21) = uStack_1e2;
        *(undefined2 *)(lVar23 + 0x115 + lVar21) = local_1e4;
        *(undefined4 *)(lVar23 + 0x118 + lVar21) = 0;
        puVar2 = (undefined8 *)(lVar23 + 0x11c + lVar21);
        *puVar2 = CONCAT44(fStack_184,local_188);
        puVar2[1] = CONCAT44(uStack_17c,fStack_180);
        puVar2 = (undefined8 *)(lVar23 + 300 + lVar21);
        *puVar2 = local_178;
        puVar2[1] = CONCAT44(uStack_16c,CONCAT22(uStack_16e,uStack_170));
        *(undefined1 *)(lVar23 + 0x13c + lVar21) = 2;
        *(ulonglong *)(lVar23 + 0x13d + lVar21) = CONCAT17(uStack_1d1,local_1d8);
        *(ulonglong *)(lVar23 + 0x144 + lVar21) =
             CONCAT35((undefined3)uStack_1cc,CONCAT41(fStack_1d0,uStack_1d1));
        *(undefined1 *)(lVar23 + 0x14c + lVar21) = 2;
        *(undefined1 *)(lVar23 + 0x14e + lVar21) = (undefined1)local_1a8;
        *(undefined2 *)(lVar23 + 0x14f + lVar21) = 0x900;
        uVar25 = CONCAT44(uStack_12c,CONCAT22(uStack_12e,CONCAT11(cStack_12f,local_130)));
        uVar32 = CONCAT35((undefined3)uStack_124,CONCAT41(fStack_128,uStack_12c._3_1_));
      }
      *(undefined8 *)(lVar23 + 0x151 + lVar21) = uVar25;
      *(undefined8 *)(lVar23 + 0x158 + lVar21) = uVar32;
      local_100[0x18] = lVar39 + 1;
      bVar17 = *(byte *)(local_110 + -4);
      if (2 < (byte)(bVar17 - 1)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      local_1a0 = (longlong)*(int *)(&DAT_142a89e88 + (ulonglong)(byte)(bVar17 - 1) * 4);
      pcVar22 = &DAT_142a89bcc;
      if ((int)local_140 == 999) {
        pcVar22 = "nani_solid_vertex_color_bricks";
      }
      if (*(uint *)(lVar27 + -0x48) < 2) {
        iVar12 = *(int *)(&DAT_142a89e90 + (longlong)(char)bVar17 * 4);
        uStack_17c = (float)((uint)uStack_17c & 0xff000000);
        local_188 = (float)((uint)local_188 & 0xffffff);
        fStack_184 = 0.0;
        fStack_180 = 0.0;
        fStack_1d0 = 0.0;
        uStack_1cc = uStack_1cc & 0xff000000;
        local_1d8 = (uint7)(uint)local_188;
        uStack_1d1 = 0;
        country_core::resources::walls::decorator::DecoratorExtraInfo::as_window();
        uVar61 = *extraout_RAX_07;
        ::utils::name_hash::NameHash::compute();
        local_190 = fStack_1d0;
        uStack_18c = (undefined3)uStack_1cc;
        local_198 = (float)local_1d8;
        uStack_194 = (float)(CONCAT17(uStack_1d1,local_1d8) >> 0x20);
        lVar31 = local_100[0x18];
        if (lVar31 == local_100[0x16]) {
          alloc::raw_vec::RawVec::
          grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                    ();
        }
        fVar53 = fVar67 * fVar67;
        fVar55 = fVar16 * fVar16 + fVar60 * fVar60 + fVar46 * fVar46;
        fVar41 = fVar16 * _UNK_142a856b8 +
                 fVar60 * _UNK_142a856b4 + fVar46 * ___xmm_000000003e0f5c290000000000000000;
        fVar41 = fVar41 + fVar41;
        fVar54 = fVar67 + fVar67;
        local_248 = local_248 +
                    fVar54 * (fVar60 * __real_3e0f5c29 - fVar16 * 0.0) +
                    fVar41 * fVar46 + (fVar53 - fVar55) * ___xmm_000000003e0f5c290000000000000000;
        fStack_244 = fStack_244 +
                     fVar54 * (fVar16 * 0.0 - fVar46 * __real_3e0f5c29) +
                     fVar41 * fVar60 + (fVar53 - fVar55) * _UNK_142a856b4;
        fVar41 = (float)local_118 +
                 fVar54 * (fVar46 * 0.0 - fVar60 * 0.0) +
                 fVar41 * fVar16 + (fVar53 - fVar55) * _UNK_142a856b8;
        lVar39 = local_100[0x17];
        lVar23 = lVar31 * 0x160;
        *(undefined8 *)(lVar39 + lVar23) = 0;
        *(undefined8 *)(lVar39 + 0x10 + lVar23) = 1;
        *(longlong **)(lVar39 + 0x18 + lVar23) = local_108 + (ulonglong)bVar17 * 0x7d;
        *(undefined8 *)(lVar39 + 0x20 + lVar23) = 1;
        *(undefined8 *)(lVar39 + 0x28 + lVar23) = extraout_RAX_08;
        *(undefined4 *)(lVar39 + 0x30 + lVar23) = 0;
        *(undefined8 *)(lVar39 + 0x34 + lVar23) = local_228;
        *(undefined4 *)(lVar39 + 0x3c + lVar23) = local_220;
        *(undefined4 *)(lVar39 + 0x40 + lVar23) = 0;
        puVar2 = (undefined8 *)(lVar39 + 0x44 + lVar23);
        *puVar2 = CONCAT44(fStack_184,local_188);
        puVar2[1] = CONCAT44(uStack_17c,fStack_180);
        puVar2 = (undefined8 *)(lVar39 + 0x54 + lVar23);
        *puVar2 = local_178;
        puVar2[1] = CONCAT44(uStack_16c,CONCAT22(uStack_16e,uStack_170));
        puVar2 = (undefined8 *)(lVar39 + 100 + lVar23);
        *puVar2 = CONCAT44(uStack_164,uStack_168);
        puVar2[1] = CONCAT44(fStack_15c,fStack_160);
        puVar2 = (undefined8 *)(lVar39 + 0x70 + lVar23);
        *puVar2 = CONCAT44(local_158,fStack_15c);
        puVar2[1] = CONCAT44(uStack_150,uStack_154);
        *(undefined4 *)(lVar39 + 0x80 + lVar23) = 0;
        *(undefined8 *)(lVar39 + 0x84 + lVar23) = local_21c;
        *(undefined4 *)(lVar39 + 0x8c + lVar23) = local_214;
        *(undefined4 *)(lVar39 + 0x90 + lVar23) = 0;
        *(undefined4 *)(lVar39 + 0x9c + lVar23) = local_1dc;
        *(ulonglong *)(lVar39 + 0x94 + lVar23) = CONCAT53(uStack_1e1,CONCAT12(uStack_1e2,local_1e4))
        ;
        *(undefined **)(lVar39 + 0xa0 + lVar23) = &DAT_142a89e94 + iVar12;
        *(undefined8 *)(lVar39 + 0xa8 + lVar23) = 0x22;
        *(undefined **)(lVar39 + 0xb0 + lVar23) =
             &anon_b17bdb6b936b48c513e8fa54c354b9a3_5_llvm_3328826292648170336;
        *(undefined8 *)(lVar39 + 0xb8 + lVar23) = 0x24;
        pfVar4 = (float *)(lVar39 + 0xc0 + lVar23);
        *pfVar4 = fVar46;
        pfVar4[1] = fVar60;
        pfVar4[2] = fVar16;
        pfVar4[3] = fVar67;
        *(ulonglong *)(lVar39 + 0xd0 + lVar23) = CONCAT44(fStack_244,local_248);
        local_118 = CONCAT44(local_118._4_4_,fVar41);
        *(float *)(lVar39 + 0xd8 + lVar23) = fVar41;
        *(undefined8 *)(lVar39 + 0xdc + lVar23) = 0x3f8000003f800000;
        *(undefined4 *)(lVar39 + 0xe4 + lVar23) = 0x3f800000;
        *(undefined4 *)(lVar39 + 0xf0 + lVar23) = 1;
        *(undefined4 *)(lVar39 + 0xf4 + lVar23) = uVar61;
        *(undefined4 *)(lVar39 + 0xf8 + lVar23) = 0;
        puVar2 = (undefined8 *)(lVar39 + 0xfc + lVar23);
        *puVar2 = CONCAT44(uStack_12c,CONCAT22(uStack_12e,CONCAT11(cStack_12f,local_130)));
        puVar2[1] = CONCAT44(uStack_124,fStack_128);
        *(undefined8 *)(lVar39 + 0x10c + lVar23) = local_120;
        *(undefined1 *)(lVar39 + 0x114 + lVar23) = 2;
        *(undefined1 *)(lVar39 + 0x117 + lVar23) = local_22c;
        *(undefined2 *)(lVar39 + 0x115 + lVar23) = local_22e;
        *(undefined4 *)(lVar39 + 0x118 + lVar23) = 0;
        puVar2 = (undefined8 *)(lVar39 + 300 + lVar23);
        *puVar2 = local_1c8;
        puVar2[1] = uStack_1c0;
        puVar2 = (undefined8 *)(lVar39 + 0x11c + lVar23);
        *puVar2 = CONCAT17(uStack_1d1,local_1d8);
        puVar2[1] = CONCAT44(uStack_1cc,fStack_1d0);
        *(undefined1 *)(lVar39 + 0x13c + lVar23) = 2;
        *(ulonglong *)(lVar39 + 0x144 + lVar23) =
             CONCAT35(uStack_18c,CONCAT41(local_190,uStack_194._3_1_));
        *(ulonglong *)(lVar39 + 0x13d + lVar23) = CONCAT44(uStack_194,local_198);
        *(undefined1 *)(lVar39 + 0x14c + lVar23) = 2;
        *(undefined1 *)(lVar39 + 0x14e + lVar23) = (undefined1)local_1a8;
        *(undefined2 *)(lVar39 + 0x14f + lVar23) = 0x900;
        *(ulonglong *)(lVar39 + 0x151 + lVar23) = CONCAT17(uStack_209,local_210);
        *(ulonglong *)(lVar39 + 0x158 + lVar23) =
             CONCAT35(uStack_204,CONCAT41(local_208,uStack_209));
        local_100[0x18] = lVar31 + 1;
      }
      uStack_17c = (float)((uint)uStack_17c & 0xff000000);
      local_188 = (float)((uint)local_188 & 0xffffff);
      fStack_184 = 0.0;
      fStack_180 = 0.0;
      fStack_128 = 0.0;
      uStack_124 = uStack_124 & 0xff000000;
      local_130 = SUB41(local_188,0);
      cStack_12f = (char)((uint)local_188 >> 8);
      uStack_12e = (undefined2)((uint)local_188 >> 0x10);
      uStack_12c = 0.0;
      uStack_1e2 = local_229;
      local_1e4 = local_22b;
      lVar31 = local_100[0x18];
      if (lVar31 == local_100[0x16]) {
        alloc::raw_vec::RawVec::
        grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                  ();
      }
      lVar39 = local_100[0x17];
      lVar23 = lVar31 * 0x160;
      *(undefined8 *)(lVar39 + lVar23) = 0;
      *(undefined8 *)(lVar39 + 0x10 + lVar23) = 0;
      *(undefined8 *)(lVar39 + 0x20 + lVar23) = 0;
      *(undefined4 *)(lVar39 + 0x30 + lVar23) = 0;
      *(ulonglong *)(lVar39 + 0x34 + lVar23) = CONCAT17(uStack_209,local_210);
      *(undefined4 *)(lVar39 + 0x3c + lVar23) = local_208;
      *(undefined4 *)(lVar39 + 0x40 + lVar23) = 0;
      puVar2 = (undefined8 *)(lVar39 + 0x44 + lVar23);
      *puVar2 = CONCAT44(fStack_184,local_188);
      puVar2[1] = CONCAT44(uStack_17c,fStack_180);
      puVar2 = (undefined8 *)(lVar39 + 0x54 + lVar23);
      *puVar2 = local_178;
      puVar2[1] = CONCAT44(uStack_16c,CONCAT22(uStack_16e,uStack_170));
      puVar2 = (undefined8 *)(lVar39 + 100 + lVar23);
      *puVar2 = CONCAT44(uStack_164,uStack_168);
      puVar2[1] = CONCAT44(fStack_15c,fStack_160);
      puVar2 = (undefined8 *)(lVar39 + 0x70 + lVar23);
      *puVar2 = CONCAT44(local_158,fStack_15c);
      puVar2[1] = CONCAT44(uStack_150,uStack_154);
      *(undefined4 *)(lVar39 + 0x80 + lVar23) = 0;
      *(undefined8 *)(lVar39 + 0x84 + lVar23) = local_228;
      *(undefined4 *)(lVar39 + 0x8c + lVar23) = local_220;
      *(undefined4 *)(lVar39 + 0x90 + lVar23) = 0;
      *(undefined8 *)(lVar39 + 0x94 + lVar23) = local_21c;
      *(undefined4 *)(lVar39 + 0x9c + lVar23) = local_214;
      *(undefined **)(lVar39 + 0xa0 + lVar23) = &DAT_142a89e88 + local_1a0;
      *(undefined8 *)(lVar39 + 0xa8 + lVar23) = 0x1c;
      *(char **)(lVar39 + 0xb0 + lVar23) = pcVar22;
      *(undefined8 *)(lVar39 + 0xb8 + lVar23) = 0x1e;
      pfVar4 = (float *)(lVar39 + 0xc0 + lVar23);
      *pfVar4 = fVar46;
      pfVar4[1] = fVar60;
      pfVar4[2] = fVar16;
      pfVar4[3] = fVar67;
      *(ulonglong *)(lVar39 + 0xd0 + lVar23) = CONCAT44(fStack_244,local_248);
      *(float *)(lVar39 + 0xd8 + lVar23) = (float)local_118;
      *(undefined8 *)(lVar39 + 0xdc + lVar23) = 0x3f8000003f800000;
      *(undefined4 *)(lVar39 + 0xe4 + lVar23) = 0x3f800000;
      *(undefined4 *)(lVar39 + 0xf0 + lVar23) = 1;
      *(int *)(lVar39 + 0xf4 + lVar23) = (int)local_140;
      *(undefined4 *)(lVar39 + 0xf8 + lVar23) = 0;
      *(undefined4 *)(lVar39 + 0x100 + lVar23) = local_134;
      *(undefined4 *)(lVar39 + 0x104 + lVar23) = extraout_EAX;
      *(float *)(lVar39 + 0x108 + lVar23) = fVar65;
      *(float *)(lVar39 + 0x10c + lVar23) = fVar66;
      *(float *)(lVar39 + 0x110 + lVar23) = fVar47;
      *(undefined1 *)(lVar39 + 0x114 + lVar23) = 0;
      *(undefined1 *)(lVar39 + 0x117 + lVar23) = local_229;
      *(undefined2 *)(lVar39 + 0x115 + lVar23) = local_22b;
      *(undefined4 *)(lVar39 + 0x118 + lVar23) = 0;
      puVar2 = (undefined8 *)(lVar39 + 0x11c + lVar23);
      *puVar2 = CONCAT17(uStack_1d1,local_1d8);
      puVar2[1] = CONCAT44(uStack_1cc,fStack_1d0);
      puVar2 = (undefined8 *)(lVar39 + 300 + lVar23);
      *puVar2 = local_1c8;
      puVar2[1] = uStack_1c0;
      *(undefined1 *)(lVar39 + 0x13c + lVar23) = 2;
      *(ulonglong *)(lVar39 + 0x13d + lVar23) =
           CONCAT44(uStack_12c,CONCAT22(uStack_12e,CONCAT11(cStack_12f,local_130)));
      *(ulonglong *)(lVar39 + 0x144 + lVar23) =
           CONCAT35((undefined3)uStack_124,CONCAT41(fStack_128,uStack_12c._3_1_));
      *(undefined1 *)(lVar39 + 0x14c + lVar23) = 2;
      *(undefined1 *)(lVar39 + 0x14e + lVar23) = (undefined1)local_1a8;
      *(undefined2 *)(lVar39 + 0x14f + lVar23) = 0x900;
      *(ulonglong *)(lVar39 + 0x151 + lVar23) = CONCAT44(uStack_194,local_198);
      *(ulonglong *)(lVar39 + 0x158 + lVar23) =
           CONCAT35(uStack_18c,CONCAT41(local_190,uStack_194._3_1_));
      local_100[0x18] = lVar31 + 1;
      country_core::resources::walls::decorator::DecoratorRank::new();
      cVar40 = extraout_AL_02;
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      if (in_RDX[3] == 0) goto LAB_1407ea842;
      local_130 = extraout_AL_03;
      cStack_12f = cVar40;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>_>
                ();
      lVar39 = *in_RDX;
      uVar35 = in_RDX[1];
      bVar17 = (byte)(extraout_RAX_09 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar17,bVar17));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar31 = lVar39 + -0x48;
      plVar28 = (longlong *)0x0;
      uVar33 = extraout_RAX_09;
      goto LAB_1407e9e49;
    }
    uVar34 = (ushort)(uVar18 - 1) & (ushort)uVar18;
    uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar34);
  }
  auVar50[0] = -(*pcVar22 == __xmm_ffffffffffffffffffffffffffffffff);
  auVar50[1] = -(pcVar22[1] == UNK_142925991);
  auVar50[2] = -(pcVar22[2] == UNK_142925992);
  auVar50[3] = -(pcVar22[3] == UNK_142925993);
  auVar50[4] = -(pcVar22[4] == UNK_142925994);
  auVar50[5] = -(pcVar22[5] == UNK_142925995);
  auVar50[6] = -(pcVar22[6] == UNK_142925996);
  auVar50[7] = -(pcVar22[7] == UNK_142925997);
  auVar50[8] = -(pcVar22[8] == UNK_142925998);
  auVar50[9] = -(pcVar22[9] == UNK_142925999);
  auVar50[10] = -(pcVar22[10] == UNK_14292599a);
  auVar50[0xb] = -(pcVar22[0xb] == UNK_14292599b);
  auVar50[0xc] = -(pcVar22[0xc] == UNK_14292599c);
  auVar50[0xd] = -(pcVar22[0xd] == UNK_14292599d);
  auVar50[0xe] = -(pcVar22[0xe] == UNK_14292599e);
  auVar50[0xf] = -(pcVar22[0xf] == UNK_14292599f);
  if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 || (SUB161(auVar50 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar50 >> 0x17,0) & 1) != 0) || (SUB161(auVar50 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar50 >> 0x27,0) & 1) != 0) || (SUB161(auVar50 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar50 >> 0x37,0) & 1) != 0) || (SUB161(auVar50 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar50 >> 0x47,0) & 1) != 0) || (SUB161(auVar50 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar50 >> 0x57,0) & 1) != 0) || (SUB161(auVar50 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar50 >> 0x67,0) & 1) != 0) || (SUB161(auVar50 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar50 >> 0x77,0) & 1) != 0) || auVar50[0xf] < '\0') {
LAB_1407ea825:
                    /* WARNING: Subroutine does not return */
    core::option::expect_failed();
  }
  uVar35 = uVar35 + lVar31 + 0x10;
  lVar31 = lVar31 + 0x10;
  goto LAB_1407e8d83;
LAB_1407e9e49:
  uVar33 = uVar35 & uVar33;
  pcVar22 = (char *)(lVar39 + uVar33);
  cVar40 = auVar42[0];
  auVar58[0] = -(*pcVar22 == cVar40);
  cVar43 = auVar42[1];
  auVar58[1] = -(pcVar22[1] == cVar43);
  cVar44 = auVar42[2];
  auVar58[2] = -(pcVar22[2] == cVar44);
  cVar45 = auVar42[3];
  auVar58[3] = -(pcVar22[3] == cVar45);
  auVar58[4] = -(pcVar22[4] == cVar40);
  auVar58[5] = -(pcVar22[5] == cVar43);
  auVar58[6] = -(pcVar22[6] == cVar44);
  auVar58[7] = -(pcVar22[7] == cVar45);
  auVar58[8] = -(pcVar22[8] == cVar40);
  auVar58[9] = -(pcVar22[9] == cVar43);
  auVar58[10] = -(pcVar22[10] == cVar44);
  auVar58[0xb] = -(pcVar22[0xb] == cVar45);
  auVar58[0xc] = -(pcVar22[0xc] == cVar40);
  auVar58[0xd] = -(pcVar22[0xd] == cVar43);
  auVar58[0xe] = -(pcVar22[0xe] == cVar44);
  auVar58[0xf] = -(pcVar22[0xf] == cVar45);
  uVar34 = (ushort)(SUB161(auVar58 >> 7,0) & 1) | (ushort)(SUB161(auVar58 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar58 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar58 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar58 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar58 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar58 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar58 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar58 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar58 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar58 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar58 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar58 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar58 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar58 >> 0x77,0) & 1) << 0xe | (ushort)(auVar58[0xf] >> 7) << 0xf;
  uVar18 = (uint)uVar34;
  if (uVar34 != 0) {
    if (local_130 == '\x1a') {
      do {
        uVar19 = 0;
        for (uVar24 = uVar18; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
          uVar19 = uVar19 + 1;
        }
        uVar29 = uVar19 + uVar33 & uVar35;
        if (*(char *)(lVar31 + uVar29 * -0x48) == '\x1a') goto LAB_1407e9f05;
        uVar34 = (ushort)(uVar18 - 1) & (ushort)uVar18;
        uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar34);
      } while (uVar34 != 0);
    }
    else {
      do {
        uVar19 = 0;
        for (uVar24 = uVar18; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
          uVar19 = uVar19 + 1;
        }
        uVar29 = uVar19 + uVar33 & uVar35;
        cVar40 = *(char *)(lVar31 + uVar29 * -0x48);
        local_108 = plVar28;
        if (((cVar40 != '\x1a') && (local_130 == cVar40)) &&
           (cStack_12f == *(char *)(lVar31 + uVar29 * -0x48 + 1))) goto LAB_1407e9f05;
        uVar34 = (ushort)(uVar18 - 1) & (ushort)uVar18;
        uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar34);
      } while (uVar34 != 0);
    }
  }
  auVar51[0] = -(*pcVar22 == __xmm_ffffffffffffffffffffffffffffffff);
  auVar51[1] = -(pcVar22[1] == UNK_142925991);
  auVar51[2] = -(pcVar22[2] == UNK_142925992);
  auVar51[3] = -(pcVar22[3] == UNK_142925993);
  auVar51[4] = -(pcVar22[4] == UNK_142925994);
  auVar51[5] = -(pcVar22[5] == UNK_142925995);
  auVar51[6] = -(pcVar22[6] == UNK_142925996);
  auVar51[7] = -(pcVar22[7] == UNK_142925997);
  auVar51[8] = -(pcVar22[8] == UNK_142925998);
  auVar51[9] = -(pcVar22[9] == UNK_142925999);
  auVar51[10] = -(pcVar22[10] == UNK_14292599a);
  auVar51[0xb] = -(pcVar22[0xb] == UNK_14292599b);
  auVar51[0xc] = -(pcVar22[0xc] == UNK_14292599c);
  auVar51[0xd] = -(pcVar22[0xd] == UNK_14292599d);
  auVar51[0xe] = -(pcVar22[0xe] == UNK_14292599e);
  auVar51[0xf] = -(pcVar22[0xf] == UNK_14292599f);
  if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar51 >> 0x17,0) & 1) != 0) || (SUB161(auVar51 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar51 >> 0x27,0) & 1) != 0) || (SUB161(auVar51 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x37,0) & 1) != 0) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') {
LAB_1407ea842:
    local_1d8 = (uint7)&local_130;
    uStack_1d1 = (undefined1)((ulonglong)&local_130 >> 0x38);
    fStack_1d0 = 3.9763908;
    uStack_1cc = 1;
    local_188 = 84.29596;
    fStack_184 = 1.4013e-45;
    fStack_180 = 1.4013e-45;
    uStack_17c = 0.0;
    uStack_168 = 0;
    uStack_164 = 0;
    local_178 = &local_1d8;
    uStack_170 = 1;
    uStack_16e = 0;
    uStack_16c = 0;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  uVar33 = (longlong)plVar28 + uVar33 + 0x10;
  plVar28 = plVar28 + 2;
  goto LAB_1407e9e49;
LAB_1407e9f05:
  plVar28 = *(longlong **)(lVar39 + -0x28 + uVar29 * -0x48);
  LOCK();
  lVar31 = *plVar28;
  *plVar28 = *plVar28 + 1;
  UNLOCK();
  if (*plVar28 != 0 && SCARRY8(lVar31,1) == *plVar28 < 0) {
    local_198 = SUB84(plVar28,0);
    uStack_194 = (float)((ulonglong)plVar28 >> 0x20);
    local_f2 = 1;
    country_core::resources::walls::decorator::DecoratorRank::new();
    local_f2 = 1;
    cVar40 = extraout_AL_04;
    country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
    if (in_RDX[9] != 0) {
      local_f2 = 1;
      local_130 = extraout_AL_05;
      cStack_12f = cVar40;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>_>
                ();
      lVar39 = in_RDX[6];
      uVar35 = in_RDX[7];
      bVar17 = (byte)(extraout_RAX_10 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar17,bVar17));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar31 = lVar39 + -0x48;
      lVar23 = 0;
      uVar33 = extraout_RAX_10;
      while( true ) {
        uVar33 = uVar35 & uVar33;
        pcVar22 = (char *)(lVar39 + uVar33);
        cVar40 = auVar42[0];
        auVar59[0] = -(*pcVar22 == cVar40);
        cVar43 = auVar42[1];
        auVar59[1] = -(pcVar22[1] == cVar43);
        cVar44 = auVar42[2];
        auVar59[2] = -(pcVar22[2] == cVar44);
        cVar45 = auVar42[3];
        auVar59[3] = -(pcVar22[3] == cVar45);
        auVar59[4] = -(pcVar22[4] == cVar40);
        auVar59[5] = -(pcVar22[5] == cVar43);
        auVar59[6] = -(pcVar22[6] == cVar44);
        auVar59[7] = -(pcVar22[7] == cVar45);
        auVar59[8] = -(pcVar22[8] == cVar40);
        auVar59[9] = -(pcVar22[9] == cVar43);
        auVar59[10] = -(pcVar22[10] == cVar44);
        auVar59[0xb] = -(pcVar22[0xb] == cVar45);
        auVar59[0xc] = -(pcVar22[0xc] == cVar40);
        auVar59[0xd] = -(pcVar22[0xd] == cVar43);
        auVar59[0xe] = -(pcVar22[0xe] == cVar44);
        auVar59[0xf] = -(pcVar22[0xf] == cVar45);
        uVar34 = (ushort)(SUB161(auVar59 >> 7,0) & 1) | (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe | (ushort)(auVar59[0xf] >> 7) << 0xf
        ;
        uVar18 = (uint)uVar34;
        if (uVar34 != 0) {
          local_108 = plVar28;
          if (local_130 == '\x1a') {
            local_134 = CONCAT31(local_134._1_3_,0x1a);
            do {
              uVar19 = 0;
              for (uVar24 = uVar18; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
                uVar19 = uVar19 + 1;
              }
              uVar29 = uVar19 + uVar33 & uVar35;
              if (*(char *)(lVar31 + uVar29 * -0x48) == '\x1a') goto LAB_1407ea0df;
              uVar34 = (ushort)(uVar18 - 1) & (ushort)uVar18;
              uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar34);
            } while (uVar34 != 0);
          }
          else {
            local_140 = (longlong *)CONCAT71(local_140._1_7_,cStack_12f);
            do {
              uVar19 = 0;
              for (uVar24 = uVar18; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
                uVar19 = uVar19 + 1;
              }
              uVar29 = uVar19 + uVar33 & uVar35;
              cVar40 = *(char *)(lVar31 + uVar29 * -0x48);
              local_1a8 = lVar31;
              local_1a0 = lVar23;
              if (((cVar40 != '\x1a') && (local_130 == cVar40)) &&
                 (cStack_12f == *(char *)(lVar31 + uVar29 * -0x48 + 1))) goto LAB_1407ea0df;
              uVar34 = (ushort)(uVar18 - 1) & (ushort)uVar18;
              uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar34);
            } while (uVar34 != 0);
          }
        }
        auVar52[0] = -(*pcVar22 == __xmm_ffffffffffffffffffffffffffffffff);
        auVar52[1] = -(pcVar22[1] == UNK_142925991);
        auVar52[2] = -(pcVar22[2] == UNK_142925992);
        auVar52[3] = -(pcVar22[3] == UNK_142925993);
        auVar52[4] = -(pcVar22[4] == UNK_142925994);
        auVar52[5] = -(pcVar22[5] == UNK_142925995);
        auVar52[6] = -(pcVar22[6] == UNK_142925996);
        auVar52[7] = -(pcVar22[7] == UNK_142925997);
        auVar52[8] = -(pcVar22[8] == UNK_142925998);
        auVar52[9] = -(pcVar22[9] == UNK_142925999);
        auVar52[10] = -(pcVar22[10] == UNK_14292599a);
        auVar52[0xb] = -(pcVar22[0xb] == UNK_14292599b);
        auVar52[0xc] = -(pcVar22[0xc] == UNK_14292599c);
        auVar52[0xd] = -(pcVar22[0xd] == UNK_14292599d);
        auVar52[0xe] = -(pcVar22[0xe] == UNK_14292599e);
        auVar52[0xf] = -(pcVar22[0xf] == UNK_14292599f);
        if ((((((((((((((((SUB161(auVar52 >> 7,0) & 1) != 0 || (SUB161(auVar52 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar52 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar52 >> 0x1f,0) & 1) != 0) || (SUB161(auVar52 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar52 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar52 >> 0x37,0) & 1) != 0) || (SUB161(auVar52 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar52 >> 0x47,0) & 1) != 0) || (SUB161(auVar52 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar52 >> 0x57,0) & 1) != 0) || (SUB161(auVar52 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar52 >> 0x67,0) & 1) != 0) || (SUB161(auVar52 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar52 >> 0x77,0) & 1) != 0) || auVar52[0xf] < '\0') break;
        uVar33 = uVar33 + lVar23 + 0x10;
        lVar23 = lVar23 + 0x10;
      }
    }
    local_1d8 = (uint7)&local_130;
    uStack_1d1 = (undefined1)((ulonglong)&local_130 >> 0x38);
    fStack_1d0 = 3.9763908;
    uStack_1cc = 1;
    local_188 = 84.29596;
    fStack_184 = 1.4013e-45;
    fStack_180 = 1.4013e-45;
    uStack_17c = 0.0;
    uStack_168 = 0;
    uStack_164 = 0;
    local_178 = &local_1d8;
    uStack_170 = 1;
    uStack_16e = 0;
    uStack_16c = 0;
    local_f2 = 1;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  goto LAB_1407eaaa8;
LAB_1407ea0df:
  plVar28 = *(longlong **)(lVar39 + -0x28 + uVar29 * -0x48);
  LOCK();
  lVar31 = *plVar28;
  *plVar28 = *plVar28 + 1;
  UNLOCK();
  if (*plVar28 != 0 && SCARRY8(lVar31,1) == *plVar28 < 0) {
    local_130 = (char)plVar28;
    cStack_12f = (char)((ulonglong)plVar28 >> 8);
    uStack_12e = (undefined2)((ulonglong)plVar28 >> 0x10);
    uStack_12c = (float)((ulonglong)plVar28 >> 0x20);
    local_f4 = 1;
    local_140 = plVar28;
    ::utils::geometry::rectangle::Rectangle3d::extent();
    plVar28 = local_100;
    fVar47 = fStack_180 * __real_00000000 * (float)__xmm_00000000000000003f0000003f000000;
    fVar66 = fStack_180 * __xmm_00000000000000003f0000003f000000._4_4_;
    fVar55 = fVar67 * fVar67;
    fVar53 = fVar16 * fVar16 + fVar60 * fVar60 + fVar46 * fVar46;
    fVar65 = fVar16 * fVar66 + fVar60 * fVar47 + fVar46 * fVar47;
    fVar65 = fVar65 + fVar65;
    fVar54 = fVar67 + fVar67;
    fVar41 = fStack_244 +
             (fVar47 * fVar16 - fVar66 * fVar46) * fVar54 +
             fVar65 * fVar60 + fVar47 * (fVar55 - fVar53);
    fStack_1d0 = (fVar47 * fVar46 - fVar47 * fVar60) * fVar54 +
                 fVar65 * fVar16 + fVar66 * (fVar55 - fVar53) + (float)local_118;
    local_1d8 = (uint7)CONCAT44(fVar41,local_248 +
                                       (fVar66 * fVar60 - fVar47 * fVar16) * fVar54 +
                                       fVar65 * fVar46 + fVar47 * (fVar55 - fVar53));
    uStack_1d1 = (undefined1)((uint)fVar41 >> 0x18);
    local_f4 = 0;
    blueprint::DecoratorBlueprint::set_colliders();
    if (*plVar28 != 0 || plVar28[1] != 0) {
      local_188 = 84.54523;
      fStack_184 = 1.4013e-45;
      fStack_180 = 1.4013e-45;
      uStack_17c = 0.0;
      local_178 = (uint7 *)0x8;
      uStack_170 = 0;
      uStack_16e = 0;
      uStack_16c = 0;
      uStack_168 = 0;
      uStack_164 = 0;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    plVar28[1] = 0;
    *plVar28 = 1;
    *(float *)(plVar28 + 2) = fVar46;
    *(float *)((longlong)plVar28 + 0x14) = fVar60;
    *(float *)(plVar28 + 3) = fVar16;
    *(float *)((longlong)plVar28 + 0x1c) = fVar67;
    plVar28[4] = CONCAT44(fStack_244,local_248);
    *(float *)(plVar28 + 5) = (float)local_118;
    *(undefined8 *)((longlong)plVar28 + 0x2c) = 0x3f8000003f800000;
    *(undefined4 *)((longlong)plVar28 + 0x34) = 0x3f800000;
    cVar40 = *(char *)(local_110 + -4);
    if (cVar40 == '\x01') {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_13 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      *(undefined4 *)extraout_RAX_13 = 0xbe933333;
      local_108 = extraout_RAX_13;
      fVar47 = __real_be933333;
      uVar61 = ___xmm_3f16bb993f0a5e353faac0833f32fec5;
      uVar62 = _UNK_142a89654;
      uVar63 = _UNK_142a89658;
      uVar64 = _UNK_142a8965c;
      uVar25 = ___xmm_3eb0a3d73eb84b5d3f6893753efd2f1b;
      uVar32 = _UNK_142a89648;
    }
    else if (cVar40 == '\x02') {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_12 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      *(undefined4 *)extraout_RAX_12 = 0xbdc8b43a;
      local_108 = extraout_RAX_12;
      fVar47 = __real_bdc8b43a;
      uVar61 = ___xmm_3f8be76d3f8041893fd9999a3f958106;
      uVar62 = _UNK_142a89624;
      uVar63 = _UNK_142a89628;
      uVar64 = _UNK_142a8962c;
      uVar25 = ___xmm_3e99999a3f1ced913fad0e563f07ae14;
      uVar32 = _UNK_142a89618;
    }
    else {
      if (cVar40 != '\x03') {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_11 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      *(undefined4 *)extraout_RAX_11 = 0x3e6b851f;
      local_108 = extraout_RAX_11;
      fVar47 = __real_3e6b851f;
      uVar61 = ___xmm_3f9f1aa03fca3d71401666663fc10625;
      uVar62 = _UNK_142a895f4;
      uVar63 = _UNK_142a895f8;
      uVar64 = _UNK_142a895fc;
      uVar25 = ___xmm_3eae147b3f5020c53fea3d713ec28f5c;
      uVar32 = _UNK_142a895e8;
    }
    *(undefined4 *)((longlong)local_108 + 4) = uVar61;
    *(undefined4 *)(local_108 + 1) = uVar62;
    *(undefined4 *)((longlong)local_108 + 0xc) = uVar63;
    *(undefined4 *)(local_108 + 2) = uVar64;
    *(undefined8 *)((longlong)local_108 + 0x14) = uVar25;
    *(undefined8 *)((longlong)local_108 + 0x1c) = uVar32;
    country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
    fVar66 = (float)extraout_RAX_14[1];
    uVar63 = *extraout_RAX_14;
    lVar31 = plVar28[0x1b];
    if (lVar31 == plVar28[0x19]) {
      alloc::raw_vec::RawVec::
      grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
    }
    lVar39 = plVar28[0x1a];
    *(undefined4 *)(lVar39 + lVar31 * 0x14) = uVar63;
    *(float *)(lVar39 + 4 + lVar31 * 0x14) = fVar47 + fVar66;
    *(undefined4 *)(lVar39 + 8 + lVar31 * 0x14) = uVar61;
    *(undefined4 *)(lVar39 + 0xc + lVar31 * 0x14) = uVar62;
    *(undefined1 *)(lVar39 + 0x10 + lVar31 * 0x14) = 0;
    plVar28[0x1b] = lVar31 + 1;
    fVar47 = *(float *)((longlong)local_108 + 0xc);
    lVar31 = local_108[2];
    country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
    fVar66 = (float)extraout_RAX_15[1];
    uVar61 = *extraout_RAX_15;
    lVar39 = plVar28[0x1b];
    if (lVar39 == plVar28[0x19]) {
      alloc::raw_vec::RawVec::
      grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
    }
    lVar23 = plVar28[0x1a];
    *(undefined4 *)(lVar23 + lVar39 * 0x14) = uVar61;
    *(float *)(lVar23 + 4 + lVar39 * 0x14) = fVar47 + fVar66;
    *(longlong *)(lVar23 + 8 + lVar39 * 0x14) = lVar31;
    *(undefined1 *)(lVar23 + 0x10 + lVar39 * 0x14) = 0;
    plVar28[0x1b] = lVar39 + 1;
    fVar47 = *(float *)(local_108 + 3);
    uVar25 = *(undefined8 *)((longlong)local_108 + 0x1c);
    country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
    fVar66 = (float)extraout_RAX_16[1];
    uVar61 = *extraout_RAX_16;
    lVar31 = plVar28[0x1b];
    if (lVar31 == plVar28[0x19]) {
      alloc::raw_vec::RawVec::
      grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
    }
    lVar39 = plVar28[0x1a];
    *(undefined4 *)(lVar39 + lVar31 * 0x14) = uVar61;
    *(float *)(lVar39 + 4 + lVar31 * 0x14) = fVar47 + fVar66;
    *(undefined8 *)(lVar39 + 8 + lVar31 * 0x14) = uVar25;
    *(undefined1 *)(lVar39 + 0x10 + lVar31 * 0x14) = 0;
    plVar28[0x1b] = lVar31 + 1;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    uVar19 = local_1b8;
    if (((local_f1 == 7) && (*(uint *)(lVar27 + -0x48) < 2)) &&
       (::utils::geometry::rectangle::Rectangle3d::extent(), uVar19 = local_1b8,
       __real_00000000 < local_198)) {
      uStack_194 = uStack_194 + __real_00000000;
      local_190 = 1.13;
      ::utils::swizzlers::impl$1::xvy();
      fVar47 = fStack_184;
      fVar66 = fVar46 * ___xmm_000000008000000080000000bf800000;
      fVar41 = fVar60 * _UNK_1429a90a4;
      fVar67 = fVar16 * _UNK_1429a90a8;
      if (2 < (byte)(*(char *)(local_110 + -4) - 1U)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      fVar65 = fVar47 * __real_00000000;
      fVar48 = fVar16 * _UNK_1429cd0c8 +
               fVar60 * _UNK_1429cd0c4 + fVar46 * ___xmm_000000003f8000000000000000000000;
      fVar48 = fVar48 + fVar48;
      fVar67 = (fVar54 * (fVar60 * _UNK_1429cd0c8 + fVar67) +
               fVar48 * fVar46 + (fVar55 - fVar53) * ___xmm_000000003f8000000000000000000000) *
               (float)__xmm_00000000000000003f0000003f000000 *
               (float)__xmm_00000000000000003f0000003f000000 + local_248 + fVar65;
      uStack_12c = (fVar54 * (fVar16 * ___xmm_000000003f8000000000000000000000 + fVar66) +
                   fVar48 * fVar60 + (fVar55 - fVar53) * _UNK_1429cd0c4) *
                   __xmm_00000000000000003f0000003f000000._4_4_ *
                   __xmm_00000000000000003f0000003f000000._4_4_ + fStack_244 + fVar47;
      fStack_128 = (fVar54 * (fVar46 * _UNK_1429cd0c4 + fVar41) +
                   fVar48 * fVar16 + (fVar55 - fVar53) * _UNK_1429cd0c8) * __real_3f000000 *
                   __real_3f000000 + (float)local_118 + fVar65;
      local_130 = SUB41(fVar67,0);
      cStack_12f = (char)((uint)fVar67 >> 8);
      uStack_12e = (undefined2)((uint)fVar67 >> 0x10);
      fStack_1d0 = local_190;
      local_1d8 = (uint7)CONCAT44(uStack_194,local_198);
      uStack_1d1 = (undefined1)((uint)uStack_194 >> 0x18);
      country_core::resources::walls::dormers::DormerHoleSsbo::gothic();
      lVar27 = local_100[0x1e];
      if (lVar27 == local_100[0x1c]) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::walls::dormers::DormerHoleSsbo,alloc::alloc::Global>();
      }
      lVar31 = local_100[0x1d];
      lVar39 = lVar27 * 0x30;
      puVar2 = (undefined8 *)(lVar31 + 0x20 + lVar39);
      *puVar2 = CONCAT44(uStack_164,uStack_168);
      puVar2[1] = CONCAT44(fStack_15c,fStack_160);
      puVar2 = (undefined8 *)(lVar31 + 0x10 + lVar39);
      *puVar2 = local_178;
      puVar2[1] = CONCAT44(uStack_16c,CONCAT22(uStack_16e,uStack_170));
      *(undefined8 *)(lVar31 + lVar39) = CONCAT44(fStack_184,local_188);
      ((undefined8 *)(lVar31 + lVar39))[1] = CONCAT44(uStack_17c,fStack_180);
      local_100[0x1e] = lVar27 + 1;
      uVar19 = local_1b8;
    }
    goto LAB_1407e8b9e;
  }
LAB_1407eaaa8:
                    /* WARNING: Does not return */
  pcVar11 = (code *)invalidInstructionException();
  (*pcVar11)();
}

