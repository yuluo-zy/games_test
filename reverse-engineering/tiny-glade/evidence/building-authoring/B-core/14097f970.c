
/* WARNING: Instruction at (ram,0x00014098176b) overlaps instruction at (ram,0x00014098176a)
    */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::wall::shapes::shape_placement::shape_placement_params(void)

{
  int iVar1;
  int *piVar2;
  ulonglong *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  byte *pbVar6;
  longlong lVar7;
  longlong lVar8;
  undefined8 *puVar9;
  longlong lVar10;
  undefined8 uVar11;
  uint uVar12;
  uint uVar13;
  longlong lVar14;
  undefined1 uVar15;
  char *pcVar16;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  float *extraout_RAX_03;
  float *extraout_RAX_04;
  float *extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined8 *in_RCX;
  longlong lVar17;
  undefined8 *in_RDX;
  undefined1 *puVar18;
  ulonglong uVar19;
  int **ppiVar20;
  undefined4 *puVar21;
  longlong *plVar22;
  longlong *in_R8;
  ulonglong uVar23;
  longlong lVar24;
  uint uVar25;
  ulonglong uVar26;
  ushort uVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  bool bVar30;
  ulonglong uVar31;
  undefined4 *puVar32;
  undefined2 in_FS;
  byte bVar33;
  char cVar34;
  char cVar38;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  undefined4 extraout_XMM0_Da_04;
  undefined4 extraout_XMM0_Da_05;
  undefined4 extraout_XMM0_Da_06;
  char cVar36;
  char cVar37;
  undefined4 extraout_XMM0_Db;
  undefined1 auVar35 [16];
  float fVar39;
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  undefined4 uVar47;
  float fVar48;
  undefined4 uVar49;
  undefined4 uVar50;
  float fVar51;
  float fVar52;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong *in_stack_00000060;
  undefined8 *in_stack_00000068;
  int *piStack_240;
  undefined1 *puStack_238;
  undefined1 auStack_230 [16];
  undefined8 uStack_220;
  undefined *puStack_218;
  undefined8 *local_210;
  undefined1 *local_208;
  undefined *puStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e4;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  float local_1d4;
  float fStack_1d0;
  float local_1cc;
  undefined8 local_1c8;
  undefined4 local_1c0;
  undefined8 local_1b8;
  undefined8 *local_1b0;
  undefined8 local_1a8;
  longlong lStack_1a0;
  longlong local_198;
  undefined8 local_190;
  float local_188;
  undefined8 local_180;
  float local_178;
  undefined8 local_170;
  undefined8 local_168;
  float local_160;
  float fStack_15c;
  float fStack_158;
  float fStack_154;
  float *pfStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined4 uStack_118;
  int local_114;
  undefined4 uStack_108;
  undefined2 uStack_104;
  undefined1 uStack_f9;
  longlong *local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  piVar2 = (int *)*in_RCX;
  if (*piVar2 != 6) {
    return;
  }
  if (1 < (byte)((char)piVar2[1] - 1U)) {
    return;
  }
  puVar3 = (ulonglong *)*in_stack_00000068;
  lVar4 = in_stack_00000068[1];
  uVar23 = *puVar3;
  uVar28 = 0;
  uVar29 = uVar23 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar23 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar29 = uVar28;
  }
  uVar26 = uVar23 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar23 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar26 = uVar28;
  }
  uVar23 = *(ulonglong *)(lVar4 + 0x30);
  uVar19 = uVar23 - uVar26;
  if (uVar26 > uVar23) {
    uVar19 = uVar28;
  }
  uVar5 = *(ulonglong *)(lVar4 + 0x10);
  uVar31 = uVar5 - uVar29;
  if (uVar29 > uVar5) {
    uVar31 = uVar28;
  }
  uVar28 = *(ulonglong *)(lVar4 + 0x40);
  *puVar3 = uVar28 - (uVar31 + uVar19);
  if (uVar29 <= uVar5 && uVar5 - uVar29 != 0 || uVar26 <= uVar23 && uVar23 - uVar26 != 0) {
    *puVar3 = uVar28;
    return;
  }
  pbVar6 = (byte *)*in_RDX;
  if ((*pbVar6 & 1) == 0) {
    local_188 = 0.0;
    local_190 = 0;
  }
  else {
    local_188 = *(float *)(pbVar6 + 0xc);
    local_190 = *(undefined8 *)(pbVar6 + 4);
  }
  puVar18 = (undefined1 *)*in_stack_00000050;
  if (puVar18 == (undefined1 *)0x0) {
LAB_14097fb53:
    lVar4 = *in_R8;
    local_178 = local_188;
    local_180 = local_190;
    uVar11 = local_180;
    lVar7 = *in_stack_00000028;
    lVar8 = in_stack_00000028[1];
    local_1a8 = 0;
    lStack_1a0 = 8;
    local_198 = 0;
    local_180._0_4_ = (float)local_190;
    local_148._0_4_ = (float)local_180 + ___xmm_4010000040100000c0100000c0100000;
    local_148._4_4_ = local_188 + _UNK_142ab6634;
    uStack_140._0_4_ = (float)local_180 + _UNK_142ab6638;
    uStack_140._4_4_ = local_188 + _UNK_142ab663c;
    piStack_240 = (int *)0x14097fbaf;
    local_180 = uVar11;
    wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped();
    uStack_140 = lStack_1a0 + local_198 * 8;
    local_148._0_4_ = (float)lStack_1a0;
    local_148._4_4_ = (float)((ulonglong)lStack_1a0 >> 0x20);
    uStack_138._0_4_ = (float)lVar4;
    uStack_138._4_4_ = (float)((ulonglong)lVar4 >> 0x20);
    piStack_240 = (int *)0x14097fbe9;
    local_130 = &local_180;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref_<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>_>,country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::closure_env_0>_>
              ();
    uVar23 = CONCAT44(fStack_15c,local_160);
    if (uVar23 == 0) {
      if (local_170 != (undefined8 *)0x0) {
        piStack_240 = (int *)0x14097fd1c;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_1a8 != 0) {
        piStack_240 = (int *)0x14097fd38;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (*(char *)*in_stack_00000048 != '\x01') {
        local_148._3_1_ = (undefined1)local_190;
        local_148._4_4_ = (float)((ulonglong)local_190 >> 8);
        uStack_140._0_3_ = (undefined3)((ulonglong)local_190 >> 0x28);
        *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
        puVar21 = (undefined4 *)*in_stack_00000038;
        *puVar21 = 0;
        puVar21[2] = 0;
        *(undefined1 *)(puVar21 + 0xb) = 2;
        uVar11 = CONCAT44(local_148._4_4_,(float)local_148);
        local_148._7_1_ = (undefined1)((ulonglong)local_190 >> 0x20);
        *(undefined8 *)((longlong)puVar21 + 0x2d) = uVar11;
        *(ulonglong *)(puVar21 + 0xd) =
             CONCAT35((int3)((uint)local_188 >> 8),
                      CONCAT41(CONCAT13(SUB41(local_188,0),(undefined3)uStack_140),local_148._7_1_))
        ;
        *(undefined8 *)(puVar21 + 0xf) = 0;
        *(undefined8 *)(puVar21 + 0x11) = 0;
        puVar21[0x13] = 0;
        pcVar16 = (char *)*in_stack_00000040;
        cVar34 = '\0';
        goto LAB_140980785;
      }
      uVar49 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      puVar21 = (undefined4 *)*in_stack_00000038;
      puVar32 = (undefined4 *)in_stack_00000038[2];
      goto LAB_14097ff79;
    }
    local_f8 = (longlong *)local_168;
    local_148 = &local_1b0;
    if (uVar23 != 1) {
      local_1b0 = &local_180;
      if (uVar23 < 0x15) {
        plVar22 = (longlong *)((longlong)local_168 + 0x20);
        do {
          piStack_240 = (int *)0x14098090c;
          core::slice::sort::shared::smallsort::
          insert_tail<country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl_0::sort_by::closure_env_0<country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::closure_env_1>_>
                    ();
          plVar22 = plVar22 + 4;
        } while (plVar22 != (longlong *)((longlong)local_168 + uVar23 * 4 * 8));
      }
      else {
        piStack_240 = (int *)0x140980945;
        core::slice::sort::stable::
        driftsort_main<country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl_0::sort_by::closure_env_0<country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::closure_env_1>,alloc::vec::Vec<country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global>_>
                  ();
      }
    }
    uStack_140._0_4_ = *(float *)((longlong)local_f8 + 0x1c);
    local_148._0_4_ = (float)*(undefined8 *)((longlong)local_f8 + 0x14);
    local_148._4_4_ = (float)((ulonglong)*(undefined8 *)((longlong)local_f8 + 0x14) >> 0x20);
    lVar17 = local_f8[1];
    puVar9 = (undefined8 *)*local_f8;
    lVar14 = local_f8[2];
    local_1b0 = puVar9;
    if (*(longlong *)(lVar4 + 0x48) != 0) {
      piStack_240 = (int *)0x14097fc5a;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar10 = *(longlong *)(lVar4 + 0x30);
      bVar33 = (byte)(extraout_RAX >> 0x39);
      auVar35 = ZEXT216(CONCAT11(bVar33,bVar33));
      auVar35 = pshuflw(auVar35,auVar35,0);
      lVar24 = 0;
      uVar23 = extraout_RAX;
      while( true ) {
        uVar23 = *(ulonglong *)(lVar4 + 0x38) & uVar23;
        pcVar16 = (char *)(lVar10 + uVar23);
        cVar34 = auVar35[0];
        auVar41[0] = -(*pcVar16 == cVar34);
        cVar36 = auVar35[1];
        auVar41[1] = -(pcVar16[1] == cVar36);
        cVar37 = auVar35[2];
        auVar41[2] = -(pcVar16[2] == cVar37);
        cVar38 = auVar35[3];
        auVar41[3] = -(pcVar16[3] == cVar38);
        auVar41[4] = -(pcVar16[4] == cVar34);
        auVar41[5] = -(pcVar16[5] == cVar36);
        auVar41[6] = -(pcVar16[6] == cVar37);
        auVar41[7] = -(pcVar16[7] == cVar38);
        auVar41[8] = -(pcVar16[8] == cVar34);
        auVar41[9] = -(pcVar16[9] == cVar36);
        auVar41[10] = -(pcVar16[10] == cVar37);
        auVar41[0xb] = -(pcVar16[0xb] == cVar38);
        auVar41[0xc] = -(pcVar16[0xc] == cVar34);
        auVar41[0xd] = -(pcVar16[0xd] == cVar36);
        auVar41[0xe] = -(pcVar16[0xe] == cVar37);
        auVar41[0xf] = -(pcVar16[0xf] == cVar38);
        uVar27 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe | (ushort)(auVar41[0xf] >> 7) << 0xf
        ;
        uVar25 = (uint)uVar27;
        while (uVar27 != 0) {
          uVar12 = 0;
          for (uVar13 = uVar25; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar12 = uVar12 + 1;
          }
          uVar29 = uVar12 + uVar23 & *(ulonglong *)(lVar4 + 0x38);
          if (*(undefined8 **)(lVar10 + -0x10 + uVar29 * -0x10) == puVar9) {
            uVar23 = *(ulonglong *)(lVar10 + -8 + uVar29 * -0x10);
            if (uVar23 >> 0x20 == 0) {
              bVar30 = false;
              goto LAB_14097fe70;
            }
            uVar29 = uVar23 & 0xffffffff;
            if ((*(ulonglong *)(lVar8 + 0x10) <= uVar29) ||
               (*(int *)(*(longlong *)(lVar8 + 8) + uVar29 * 0x14) != (int)(uVar23 >> 0x20)))
            goto LAB_14097fe6d;
            lVar4 = *(longlong *)(lVar8 + 8) + uVar29 * 0x14;
            uVar23 = (ulonglong)*(uint *)(lVar4 + 4);
            if ((uVar23 == 0xffffffff) ||
               ((*(ulonglong *)(lVar7 + 0x38) <= uVar23 ||
                ((*(ulonglong *)
                   (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar4 + 4) >> 6) * 8) >>
                  (uVar23 & 0x3f) & 1) == 0)))) goto LAB_14097fe6d;
            bVar30 = *(float *)(*(longlong *)
                                 (*(longlong *)
                                   (*(longlong *)(lVar8 + 0x1a8) + 0x18 +
                                   (ulonglong)*(uint *)(lVar4 + 0xc) * 0x48) + 0x10 +
                                 ~*(ulonglong *)
                                   (*(longlong *)
                                     (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
                                     (ulonglong)*(uint *)(lVar4 + 0xc) * 0x48) +
                                   *(longlong *)(lVar7 + 0x110) * 8) * 0x30) + 0x30 +
                               (ulonglong)*(uint *)(lVar4 + 0x10) * 0x58) <= __real_3dcccccd;
            goto LAB_14097fe70;
          }
          uVar27 = (ushort)(uVar25 - 1) & (ushort)uVar25;
          uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar27);
        }
        auVar40[0] = -(*pcVar16 == -1);
        auVar40[1] = -(pcVar16[1] == -1);
        auVar40[2] = -(pcVar16[2] == -1);
        auVar40[3] = -(pcVar16[3] == -1);
        auVar40[4] = -(pcVar16[4] == -1);
        auVar40[5] = -(pcVar16[5] == -1);
        auVar40[6] = -(pcVar16[6] == -1);
        auVar40[7] = -(pcVar16[7] == -1);
        auVar40[8] = -(pcVar16[8] == -1);
        auVar40[9] = -(pcVar16[9] == -1);
        auVar40[10] = -(pcVar16[10] == -1);
        auVar40[0xb] = -(pcVar16[0xb] == -1);
        auVar40[0xc] = -(pcVar16[0xc] == -1);
        auVar40[0xd] = -(pcVar16[0xd] == -1);
        auVar40[0xe] = -(pcVar16[0xe] == -1);
        auVar40[0xf] = -(pcVar16[0xf] == -1);
        if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar40 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar40 >> 0x1f,0) & 1) != 0) || (SUB161(auVar40 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar40 >> 0x37,0) & 1) != 0) || (SUB161(auVar40 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar40 >> 0x47,0) & 1) != 0) || (SUB161(auVar40 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar40 >> 0x57,0) & 1) != 0) || (SUB161(auVar40 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar40 >> 0x67,0) & 1) != 0) || (SUB161(auVar40 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar40 >> 0x77,0) & 1) != 0) || auVar40[0xf] < '\0') break;
        uVar23 = uVar23 + lVar24 + 0x10;
        lVar24 = lVar24 + 0x10;
      }
    }
LAB_14097fe6d:
    bVar30 = false;
LAB_14097fe70:
    local_1c0 = *(undefined4 *)((longlong)local_f8 + 0x1c);
    local_1c8 = *(undefined8 *)((longlong)local_f8 + 0x14);
    local_1d4 = (float)local_148;
    fStack_1d0 = local_148._4_4_;
    local_1cc = (float)uStack_140;
    if (local_170 != (undefined8 *)0x0) {
      piStack_240 = (int *)0x14097feaa;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_1a8 != 0) {
      piStack_240 = (int *)0x14097fec6;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar49 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    puVar32 = (undefined4 *)in_stack_00000038[2];
    *puVar32 = uVar49;
    puVar21 = (undefined4 *)*in_stack_00000038;
    *puVar21 = 0;
    puVar21[2] = 1;
    *(longlong *)(puVar21 + 6) = lVar17;
    *(undefined8 **)(puVar21 + 8) = puVar9;
    puVar21[10] = (int)lVar14;
    *(bool *)(puVar21 + 0xb) = bVar30;
    *(ulonglong *)((longlong)puVar21 + 0x2d) =
         CONCAT17(fStack_1d0._0_1_,CONCAT43(local_1d4,uStack_1d8._1_3_));
    *(ulonglong *)(puVar21 + 0xd) = CONCAT44(local_1cc,fStack_1d0);
    *(undefined8 *)(puVar21 + 0xf) = 0;
    *(undefined8 *)(puVar21 + 0x11) = local_1c8;
    uStack_1dc = local_1c0;
code_r0x00014097ff17:
    puVar21[0x13] = uStack_1dc;
    cVar34 = '\x02';
    cVar36 = *(char *)*in_stack_00000048;
joined_r0x0001409806ab:
    if (cVar36 == '\0') {
      *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
      lVar4 = *in_stack_00000060;
      lVar7 = *(longlong *)(lVar4 + 0x30);
      uVar11 = *(undefined8 *)(lVar4 + 0x40);
      if (lVar7 == *(longlong *)(lVar4 + 0x20)) {
        piStack_240 = (int *)0x1409806db;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar8 = *(longlong *)(lVar4 + 0x28);
      *(undefined8 *)(lVar8 + lVar7 * 0x10) = uVar11;
      *(undefined1 *)(lVar8 + 8 + lVar7 * 0x10) = 0;
      *(longlong *)(lVar4 + 0x30) = lVar7 + 1;
      *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
      pcVar16 = (char *)*in_stack_00000040;
      if (((byte)(cVar34 - 1U) < 2) && (*pcVar16 == '\0')) {
        uStack_138._0_3_ = (undefined3)((uint)puVar21[0xe] >> 8);
        uVar11 = *(undefined8 *)(puVar21 + 0xc);
        local_148._7_1_ = (undefined1)uVar11;
        uStack_140._0_4_ = (float)((ulonglong)uVar11 >> 8);
        uStack_140._4_3_ = (undefined3)((ulonglong)uVar11 >> 0x28);
        uStack_140._4_4_ = (float)CONCAT13((char)puVar21[0xe],uStack_140._4_3_);
        *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
        lVar4 = *in_stack_00000030;
        lVar7 = *(longlong *)(lVar4 + 0x30);
        uVar11 = *(undefined8 *)(lVar4 + 0x40);
        if (lVar7 == *(longlong *)(lVar4 + 0x20)) {
          piStack_240 = (int *)0x14098074f;
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar8 = *(longlong *)(lVar4 + 0x28);
        lVar17 = lVar7 * 0x20;
        *(undefined8 *)(lVar8 + lVar17) = uVar11;
        *(undefined1 *)(lVar8 + 8 + lVar17) = 3;
        *(uint *)(lVar8 + 0x18 + lVar17) = CONCAT31((undefined3)uStack_138,uStack_140._7_1_);
        puVar9 = (undefined8 *)(lVar8 + 9 + lVar17);
        *puVar9 = CONCAT44(local_148._4_4_,(float)local_148);
        puVar9[1] = CONCAT44(uStack_140._4_4_,(float)uStack_140);
        *(longlong *)(lVar4 + 0x30) = lVar7 + 1;
        *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
      }
LAB_140980785:
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      *pcVar16 = cVar34;
      return;
    }
    *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
    lVar4 = *in_stack_00000060;
    lVar7 = *(longlong *)(lVar4 + 0x30);
    uVar11 = *(undefined8 *)(lVar4 + 0x40);
    if (lVar7 == *(longlong *)(lVar4 + 0x20)) {
      piStack_240 = (int *)0x14097ff5a;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar8 = *(longlong *)(lVar4 + 0x28);
    *(undefined8 *)(lVar8 + lVar7 * 0x10) = uVar11;
    *(undefined1 *)(lVar8 + 8 + lVar7 * 0x10) = 1;
    *(longlong *)(lVar4 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
LAB_14097ff79:
    local_148._3_1_ = (undefined1)local_190;
    local_148._4_4_ = (float)((ulonglong)local_190 >> 8);
    uStack_140._0_3_ = (undefined3)((ulonglong)local_190 >> 0x28);
    *puVar32 = uVar49;
    *puVar21 = 0;
    puVar21[2] = 0;
    *(undefined1 *)(puVar21 + 0xb) = 2;
    uVar11 = CONCAT44(local_148._4_4_,(float)local_148);
    local_148._7_1_ = (undefined1)((ulonglong)local_190 >> 0x20);
    *(undefined8 *)((longlong)puVar21 + 0x2d) = uVar11;
    *(ulonglong *)(puVar21 + 0xd) =
         CONCAT35((int3)((uint)local_188 >> 8),
                  CONCAT41(CONCAT13(SUB41(local_188,0),(undefined3)uStack_140),local_148._7_1_));
    *(undefined1 (*) [16])(puVar21 + 0xf) = (undefined1  [16])0x0;
    puVar21[0x13] = 0;
    return;
  }
  local_208 = (undefined1 *)CONCAT44(local_208._4_4_,0xfb);
  local_210 = (undefined8 *)CONCAT44(local_210._4_4_,0x7f7fffff);
  piStack_240 = (int *)0x14097fae3;
  collision::world::Rapier::raycast();
  if (local_114 == 2) goto LAB_14097fb53;
  fVar43 = (float)local_130;
  fVar52 = local_130._4_4_;
  fVar42 = (float)local_128;
  local_198 = CONCAT44(uStack_138._4_4_,(float)uStack_138);
  lVar4 = CONCAT44(uStack_140._4_4_,(float)uStack_140);
  iVar1 = *(int *)(&DAT_142ab7524 + ((ulonglong)(uint)(float)local_148 & 0xff) * 4);
  pcVar16 = &DAT_142ab7524 + iVar1;
  local_1a8 = CONCAT44(local_148._4_4_,(float)local_148);
  lStack_1a0 = lVar4;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch((ulonglong)(uint)(float)local_148 & 0xff) {
  case 0:
  case 9:
    local_f8 = (longlong *)*in_R8;
    piStack_240 = (int *)&UNK_14097ffdd;
    resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_00 == 0) {
                    /* WARNING: Subroutine does not return */
      piStack_240 = (int *)&UNK_1409809ad;
      core::option::unwrap_failed();
    }
    if (__real_42700000 <= fVar52) {
      fVar52 = __real_42700000;
    }
    piStack_240 = (int *)&UNK_140980010;
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
    fVar45 = __real_3f800000;
    fVar46 = 0.0;
    fVar39 = 0.0;
    if (0.0 <= extraout_XMM0_Da) {
      fVar39 = extraout_XMM0_Da;
    }
    fVar48 = __real_3f800000;
    if (fVar39 <= __real_3f800000) {
      fVar48 = fVar39;
    }
    piStack_240 = (int *)&UNK_140980045;
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    if (0.0 <= extraout_XMM0_Da_00) {
      fVar46 = extraout_XMM0_Da_00;
    }
    if (fVar46 <= fVar45) {
      fVar45 = fVar46;
    }
    if (*(uint *)(extraout_RAX_00 + 600) < 2) {
      if ((*(uint *)(extraout_RAX_00 + 600) & 1) == 0) {
        piStack_240 = (int *)&UNK_140980156;
        fVar39 = fVar45;
        ::utils::geometry::circle::Circle2d::get_normal_at_u();
        fVar46 = extraout_XMM0_Da_03;
      }
      else {
        fVar46 = *(float *)(extraout_RAX_00 + 0x25c);
        fVar39 = *(float *)(extraout_RAX_00 + 0x260);
      }
    }
    else {
      piStack_240 = (int *)&UNK_140980138;
      ::utils::curve::Curve::
      get_tangent_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      fVar46 = (float)local_148;
      fVar39 = (float)uStack_140;
    }
    fVar44 = fVar52;
    if (fVar52 < __real_3fc00000) {
      piStack_240 = (int *)&UNK_140980187;
      ::utils::curve::Curve::get_pos_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
                ();
      fVar52 = 0.0;
      fVar44 = local_148._4_4_;
      fVar42 = (float)uStack_140;
      fVar43 = (float)local_148;
    }
    piStack_240 = (int *)&UNK_1409801b2;
    ::utils::curve::Curve::get_pos_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
    local_170 = (undefined8 *)CONCAT44((float)uStack_140,(float)local_148);
    piStack_240 = (int *)&UNK_1409801d7;
    ::utils::swizzlers::impl$1::xvy();
    if ((((0.0 < fVar52) && (*(uint *)(extraout_RAX_00 + 600) < 2)) && (*piVar2 == 6)) &&
       ((char)piVar2[1] == '\x02')) {
      if ((*(uint *)(extraout_RAX_00 + 600) & 1) == 0) {
        uVar11 = *(undefined8 *)(extraout_RAX_00 + 0x25c);
        local_148._0_4_ = (float)uVar11;
        local_148._4_4_ = (float)((ulonglong)uVar11 >> 0x20);
        uStack_140._0_4_ = (float)*(undefined8 *)(extraout_RAX_00 + 0x264);
        uStack_140._4_4_ = (float)((ulonglong)*(undefined8 *)(extraout_RAX_00 + 0x264) >> 0x20);
        uStack_140._0_4_ = (float)uStack_140 + ___real_bf6ccccd;
        piStack_240 = (int *)&UNK_140980819;
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        uVar47 = extraout_XMM0_Da_06;
      }
      else {
        uVar11 = *(undefined8 *)(extraout_RAX_00 + 0x25c);
        local_148._0_4_ = (float)uVar11;
        local_148._4_4_ = (float)((ulonglong)uVar11 >> 0x20);
        uStack_140._0_4_ = (float)*(undefined8 *)(extraout_RAX_00 + 0x264);
        uStack_140._4_4_ = (float)((ulonglong)*(undefined8 *)(extraout_RAX_00 + 0x264) >> 0x20);
        uStack_138._0_4_ =
             (float)*(undefined8 *)(extraout_RAX_00 + 0x26c) +
             ___xmm_0000000000000000bfa00000bfa00000;
        uStack_138._4_4_ =
             (float)((ulonglong)*(undefined8 *)(extraout_RAX_00 + 0x26c) >> 0x20) + _UNK_142ab6624;
        piStack_240 = (int *)&UNK_14098024a;
        ::utils::geometry::rectangle::Rectangle2d::get_pos_at_u();
        uVar47 = extraout_XMM0_Da_04;
      }
      local_170 = (undefined8 *)CONCAT44(fVar45,uVar47);
      piStack_240 = (int *)&UNK_140980834;
      ::utils::swizzlers::impl$1::xvy();
      fVar44 = local_148._4_4_;
      fVar43 = (float)local_148;
      fVar42 = (float)uStack_140;
    }
    piStack_240 = (int *)&UNK_14098085a;
    resources::walls::public_walls::PublicWalls::roof_entity();
    piStack_240 = (int *)&UNK_140980869;
    core::option::Option::
    and_then<bevy_ecs::entity::Entity,ref_<country_core::resources::roofs::roof_shape::Roof>,country_core::systems::ui_move_shapes::ui_move_shapes::closure_env_2>
              ();
    if (extraout_RAX_06 == 0) {
      bVar30 = false;
    }
    else {
      bVar30 = *(float *)(extraout_RAX_06 + 0x30) <= __real_3dcccccd;
    }
    uVar49 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    puVar32 = (undefined4 *)in_stack_00000038[2];
    *puVar32 = uVar49;
    puVar21 = (undefined4 *)*in_stack_00000038;
    *puVar21 = 0;
    puVar21[2] = 1;
    puVar21[6] = fVar46;
    puVar21[7] = fVar39;
    *(longlong *)(puVar21 + 8) = lVar4;
    puVar21[10] = fVar48;
    *(bool *)(puVar21 + 0xb) = bVar30;
    puVar21[0xc] = fVar43;
    puVar21[0xd] = fVar44;
    puVar21[0xe] = fVar42;
    puVar21[0xf] = fVar52;
    puVar21[0x10] = 0;
    *(undefined8 *)(puVar21 + 0x11) = uStack_1e4;
    goto code_r0x00014097ff17;
  default:
    goto LAB_14097fb53;
  case 2:
    local_1a8._1_1_ = (undefined1)((uint)(float)local_148 >> 8);
    uVar15 = local_1a8._1_1_;
    pfStack_150 = (float *)*in_R8;
    piStack_240 = (int *)&UNK_14098009c;
    resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_01 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      piStack_240 = (int *)&UNK_1409809b9;
      core::option::unwrap_failed();
    }
    uStack_f9 = uVar15;
    if (1 < *(uint *)(extraout_RAX_01 + 0x4b)) {
                    /* WARNING: Subroutine does not return */
      piStack_240 = (int *)&UNK_1409809c5;
      core::option::unwrap_failed();
    }
    piStack_240 = (int *)&UNK_1409800d7;
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
    fVar45 = __real_3f800000;
    piStack_240 = (int *)&UNK_1409800ff;
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    local_f8 = extraout_RAX_01;
    if ((*(byte *)(extraout_RAX_01 + 0x4b) & 1) == 0) {
      fVar46 = 0.0;
      if (0.0 <= extraout_XMM0_Da_02) {
        fVar46 = extraout_XMM0_Da_02;
      }
      fVar39 = fVar45;
      if (fVar46 <= fVar45) {
        fVar39 = fVar46;
      }
      uVar50 = 0;
      piStack_240 = (int *)&UNK_140980264;
      ::utils::geometry::circle::Circle2d::get_normal_at_u();
      uVar47 = extraout_XMM0_Da_05;
      uVar49 = extraout_XMM0_Db;
    }
    else {
      uVar47 = (undefined4)*(undefined8 *)((longlong)extraout_RAX_01 + 0x25c);
      fVar39 = (float)((ulonglong)*(undefined8 *)((longlong)extraout_RAX_01 + 0x25c) >> 0x20);
      uVar49 = 0;
      uVar50 = 0;
    }
    fVar46 = __real_c0c00000;
    if ((*piVar2 != 6 || (char)piVar2[1] != '\x02') &&
       ((*piVar2 != 6 || (fVar46 = __real_c1000000, (char)piVar2[1] != '\x01')))) {
                    /* WARNING: Subroutine does not return */
      piStack_240 = (int *)0x140980932;
      core::panicking::panic();
    }
    piStack_240 = (int *)&UNK_1409802b4;
    resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX_02 >> 0x20 == 0) {
                    /* WARNING: Subroutine does not return */
      piStack_240 = (int *)&UNK_1409809d1;
      core::option::unwrap_failed();
    }
    piStack_240 = (int *)&UNK_1409802db;
    bevy_ecs::system::query::Query::
    get<ref_<country_core::resources::roofs::roof_shape::Roof>,tuple_<>_>();
    if ((int)local_170 != 3) {
      local_148._4_4_ = (float)((ulonglong)local_170 >> 0x20);
      uStack_140._0_4_ = SUB84(local_168,0);
      uStack_140._4_4_ = (float)((ulonglong)local_168 >> 0x20);
      puStack_218 = &UNK_142ab6fb8;
                    /* WARNING: Subroutine does not return */
      piStack_240 = (int *)&UNK_140980a01;
      local_148._0_4_ = (float)(int)local_170;
      core::result::unwrap_failed();
    }
    local_148._0_4_ = fVar43;
    local_148._4_4_ = fVar52;
    uStack_140._0_4_ = fVar42;
    piStack_240 = (int *)&UNK_14098030b;
    snap_to_roof_ridge();
    if ((int)local_170 == 0) {
      cVar34 = '\0';
      fVar48 = extraout_XMM0_Da_01;
      fVar44 = fVar52;
    }
    else {
      fVar43 = local_170._4_4_;
      if ((int)local_170 == 1) {
        local_148._0_4_ = 0.0;
        uStack_138._0_4_ = 0.3;
        uStack_138._4_4_ = local_170._4_4_;
        local_128 = CONCAT35(local_128._5_3_,0x3f800000);
        piStack_240 = (int *)&UNK_14098037b;
        country_core::utils::debug_viz::DebugSphere::show();
        piStack_240 = (int *)&UNK_140980380;
        alloc::alloc::exchange_malloc();
        local_180 = CONCAT44(local_168._4_4_,fVar43);
        piStack_240 = (int *)&UNK_1409803a6;
        pfStack_150 = extraout_RAX_03;
        ::utils::swizzlers::impl$1::xvy();
        *pfStack_150 = fVar43;
        pfStack_150[1] = (float)local_168;
        pfStack_150[2] = local_168._4_4_;
        pfStack_150[5] = (float)uStack_140;
        *(ulonglong *)(pfStack_150 + 3) = CONCAT44(local_148._4_4_,(float)local_148);
        local_148._0_4_ = 2.8026e-45;
        local_148._4_4_ = 0.0;
        uStack_140._0_4_ = SUB84(pfStack_150,0);
        uStack_140._4_4_ = (float)((ulonglong)pfStack_150 >> 0x20);
        uStack_138._0_4_ = 2.8026e-45;
        uStack_138._4_4_ = 0.0;
        local_130 = (undefined8 *)&DAT_8000000000000000;
        uStack_118 = 0;
        uStack_108 = 0x40400000;
        uStack_104 = 0x101;
        piStack_240 = (int *)&UNK_14098041c;
        country_core::utils::debug_viz::DebugLine::show();
        cVar34 = '\x02';
        fVar48 = extraout_XMM0_Da_01;
        fVar44 = (float)local_168;
        fVar42 = local_168._4_4_;
      }
      else {
        uStack_1f8 = (undefined1 *)CONCAT44(fVar39,uVar47);
        uStack_1f0 = CONCAT44(uVar50,uVar49);
        local_1b8 = (undefined *)CONCAT44(fVar46,extraout_XMM0_Da_01);
        piStack_240 = (int *)&UNK_140980459;
        alloc::alloc::exchange_malloc();
        local_148._0_4_ = 2.8026e-45;
        local_148._4_4_ = 0.0;
        uStack_140._0_4_ = SUB84(extraout_RAX_04,0);
        uStack_140._4_4_ = (float)((ulonglong)extraout_RAX_04 >> 0x20);
        uStack_138._0_4_ = 2.8026e-45;
        uStack_138._4_4_ = 0.0;
        local_130 = (undefined8 *)0x8000000000000000;
        uStack_118 = 0;
        uStack_108 = 0x40400000;
        uStack_104 = 0x101;
        fVar48 = fVar45 - fVar43;
        fVar42 = fStack_154 * fVar43 + local_160 * fVar48;
        fVar44 = (float)((ulonglong)local_168 >> 0x20);
        fVar46 = fVar44 + _UNK_142a89ef4;
        local_160 = local_160 + _UNK_142a89ef8;
        fVar39 = fStack_15c + _UNK_142a89efc;
        *extraout_RAX_04 = SUB84(local_168,0) + ___xmm_00000000000000003e4ccccd00000000;
        extraout_RAX_04[1] = fVar46;
        extraout_RAX_04[2] = local_160;
        extraout_RAX_04[3] = fVar39;
        *(ulonglong *)(extraout_RAX_04 + 4) =
             CONCAT44(fStack_154 + 0.0,__real_3e4ccccd + fStack_158);
        piStack_240 = (int *)&UNK_140980508;
        country_core::utils::debug_viz::DebugLine::show();
        fStack_158 = fVar43 * fStack_158;
        fVar43 = fVar43 * fStack_15c + fVar48 * SUB84(local_168,0);
        fVar44 = fStack_158 + fVar48 * fVar44;
        local_148._0_4_ = 0.0;
        uStack_138._0_4_ = 0.3;
        local_130 = (undefined8 *)CONCAT44(fVar42,fVar44);
        local_128 = CONCAT35(local_128._5_3_,0x3f800000);
        piStack_240 = (int *)&UNK_140980559;
        uStack_138._4_4_ = fVar43;
        country_core::utils::debug_viz::DebugSphere::show();
        piStack_240 = (int *)&UNK_14098055e;
        alloc::alloc::exchange_malloc();
        local_180 = CONCAT44(fVar42,fVar43);
        piStack_240 = (int *)&UNK_140980588;
        pfStack_150 = extraout_RAX_05;
        ::utils::swizzlers::impl$1::xvy();
        *pfStack_150 = fVar43;
        pfStack_150[1] = fVar44;
        pfStack_150[2] = fVar42;
        pfStack_150[5] = (float)uStack_140;
        *(ulonglong *)(pfStack_150 + 3) = CONCAT44(local_148._4_4_,(float)local_148);
        local_148._0_4_ = 2.8026e-45;
        local_148._4_4_ = 0.0;
        uStack_140._0_4_ = SUB84(pfStack_150,0);
        uStack_140._4_4_ = (float)((ulonglong)pfStack_150 >> 0x20);
        uStack_138._0_4_ = 2.8026e-45;
        uStack_138._4_4_ = 0.0;
        local_130 = (undefined8 *)&DAT_8000000000000000;
        uStack_118 = 0;
        uStack_108 = 0x40400000;
        uStack_104 = 0x101;
        piStack_240 = (int *)&UNK_1409805e9;
        country_core::utils::debug_viz::DebugLine::show();
        cVar34 = '\x02';
        fVar48 = (float)local_1b8;
        uVar47 = (undefined4)uStack_1f8;
        fVar39 = uStack_1f8._4_4_;
        fVar46 = local_1b8._4_4_;
      }
    }
    fVar51 = 0.0;
    if (0.0 <= fVar48) {
      fVar51 = fVar48;
    }
    fVar52 = fVar52 + fVar46 + __real_40200000;
    if (fVar51 <= fVar45) {
      fVar45 = fVar51;
    }
    lVar7 = local_f8[0x50];
    uVar49 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    puVar32 = (undefined4 *)in_stack_00000038[2];
    *puVar32 = uVar49;
    puVar21 = (undefined4 *)*in_stack_00000038;
    *puVar21 = 1;
    puVar21[1] = fVar52;
    puVar21[2] = 2;
    puVar21[3] = fVar43;
    puVar21[4] = fVar44;
    puVar21[5] = fVar42;
    *(ulonglong *)(puVar21 + 6) = CONCAT44(fVar39,uVar47);
    *(longlong *)(puVar21 + 8) = lVar4;
    puVar21[10] = fVar45;
    *(undefined1 *)(puVar21 + 0xb) = uStack_f9;
    puVar21[0xc] = fVar43;
    puVar21[0xd] = fVar44;
    puVar21[0xe] = fVar42;
    puVar21[0xf] = (int)lVar7;
    puVar21[0x10] = 0;
    puVar21[0x11] = fVar43;
    puVar21[0x12] = fVar44;
    puVar21[0x13] = fVar42;
    cVar36 = *(char *)*in_stack_00000048;
    goto joined_r0x0001409806ab;
  case 3:
  case 4:
  case 5:
  case 0xf:
  case 0x10:
    local_170 = &local_1a8;
    local_168 = collision::world::impl$86::fmt;
    local_148._0_4_ = 85.720276;
    local_148._4_4_ = 1.4013e-45;
    uStack_140._0_4_ = 2.8026e-45;
    uStack_140._4_4_ = 0.0;
    local_128 = 0;
    uStack_138 = &local_170;
    local_130 = (undefined8 *)0x1;
                    /* WARNING: Subroutine does not return */
    piStack_240 = (int *)&UNK_1409809a1;
    core::panicking::panic_fmt();
  case 0x11:
    piStack_240 = (int *)0x140981692;
    core::fmt::Formatter::debug_tuple_field1_finish();
    return;
  case 0x12:
  case 0x18:
    ppiVar20 = (int **)auStack_230;
    uStack_1f8 = &DAT_142ab7524;
    puStack_200 = &DAT_142ab7100;
    uStack_1f0 = 4;
    local_210 = (undefined8 *)&DAT_0000000c;
    local_208 = puVar18;
    break;
  case 0x13:
    pcVar16 = &DAT_0000000b;
  case 0x1a:
    pcVar16[-0x77] = pcVar16[-0x77] | 0x24;
    *(undefined2 *)((longlong)pcVar16 * 5) = in_FS;
    *pcVar16 = *pcVar16 + (char)pcVar16;
    pcVar16[-0x73] = pcVar16[-0x73] + '$';
    local_208 = &DAT_142ab7524;
    local_210 = &local_1b8;
    puStack_218 = &DAT_00000006;
    goto code_r0x000140981c20;
  case 0x14:
    ppiVar20 = (int **)auStack_230;
    uStack_1f8 = puStack_238;
    puStack_200 = &UNK_142ab7330;
    uStack_1f0 = 9;
    local_210 = (undefined8 *)&DAT_0000000a;
    local_208 = puVar18;
    break;
  case 0x15:
    goto code_r0x000140981b6c;
  case 0x16:
    goto code_r0x000140981c20;
  case 0x17:
    local_1b8 = (undefined *)CONCAT44(local_1b8._4_4_,0x42ab7524);
    local_208 = &UNK_142ab72e8;
code_r0x000140981b6c:
    local_210 = &local_1b8;
    puStack_218 = &DAT_0000000a;
    goto code_r0x000140981c20;
  case 0x19:
    *pcVar16 = *pcVar16;
    (&UNK_142ab74ad)[iVar1] = (&UNK_142ab74ad)[iVar1] + '$';
  case 0x1b:
    local_1b8 = &UNK_142ab752c;
    local_208 = &UNK_142ab71a8;
    local_210 = &local_1b8;
    puStack_218 = &DAT_00000007;
    goto code_r0x000140981c20;
  case 0x1c:
  case 0x1f:
    *(undefined **)(puStack_238 + 0x38) = &DAT_142ab7524;
    *(undefined **)(puStack_238 + 0x30) = &UNK_142ab7330;
    *(undefined1 **)(puStack_238 + 0x28) = puVar18;
    *(undefined8 *)(puStack_238 + 0x40) = 9;
    *(undefined8 *)(puStack_238 + 0x20) = 10;
    ppiVar20 = (int **)puStack_238;
    break;
  case 0x1d:
    ppiVar20 = (int **)auStack_230;
    uStack_1f8 = &DAT_142ab7524;
    puStack_200 = &DAT_142ab7100;
    local_208 = puStack_238;
    uStack_1f0 = 4;
    local_210 = (undefined8 *)&DAT_0000000c;
    break;
  case 0x1e:
    ppiVar20 = &piStack_240;
    local_210 = (undefined8 *)&DAT_142ab7524;
    puStack_200 = (undefined *)0x4;
    uStack_220 = 0xc;
    piStack_240 = piVar2;
    puStack_218 = puVar18;
    break;
  case 0x20:
    local_210 = (undefined8 *)CONCAT44(local_210._4_4_,(int)puVar18);
    uStack_1d8 = 5;
    local_1d4 = 0.0;
    uStack_1f8 = (undefined1 *)0x4;
    puStack_218 = (undefined *)0x7;
    piStack_240 = (int *)&UNK_14098195b;
    core::fmt::Formatter::debug_struct_field3_finish();
    return;
  case 0x21:
    (&UNK_142ab74ad)[iVar1] = (&UNK_142ab74ad)[iVar1] | 0x24;
    *(undefined2 *)((longlong)pcVar16 * 5) = in_FS;
    *pcVar16 = *pcVar16 + (char)pcVar16;
    (&UNK_142ab74b1)[iVar1] = (&UNK_142ab74b1)[iVar1] + '$';
    local_208 = &DAT_142ab7524;
    local_210 = &local_1b8;
    puStack_218 = &DAT_00000007;
code_r0x000140981c20:
    piStack_240 = (int *)&UNK_140981c28;
    core::fmt::Formatter::debug_struct_field1_finish();
    return;
  }
  *(undefined **)((longlong)ppiVar20 + -8) = &UNK_140981b2e;
  core::fmt::Formatter::debug_struct_field2_finish();
  return;
}

