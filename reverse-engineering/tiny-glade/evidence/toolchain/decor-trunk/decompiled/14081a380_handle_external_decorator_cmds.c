// ADDR 0x14081a380
// SIG void __cdecl handle_external_decorator_cmds(void)
// ORIG system::handle_external_decorator_cmds


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  undefined4 uVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  longlong lVar12;
  void *pvVar13;
  undefined4 *puVar14;
  undefined1 auVar15 [16];
  uint uVar16;
  uint uVar17;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  int extraout_EAX;
  int extraout_EAX_00;
  uint uVar20;
  uint uVar21;
  longlong lVar22;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong uVar23;
  undefined1 *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong extraout_RAX_04;
  undefined1 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  undefined8 *in_RCX;
  ulonglong uVar24;
  int iVar25;
  int iVar26;
  undefined8 *in_RDX;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  undefined1 (*pauVar30) [16];
  longlong lVar31;
  undefined8 *in_R8;
  ulonglong uVar32;
  longlong in_R9;
  longlong lVar33;
  ushort uVar34;
  longlong lVar35;
  undefined1 (*pauVar36) [16];
  longlong lVar37;
  undefined1 *puVar38;
  byte bVar39;
  char cVar40;
  char cVar42;
  char cVar43;
  char cVar44;
  undefined1 auVar41 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  longlong in_stack_00000028;
  longlong in_stack_00000038;
  longlong in_stack_00000048;
  undefined1 (*local_258) [16];
  undefined4 local_250;
  undefined4 uStack_24c;
  longlong local_240;
  longlong local_1c8;
  longlong lStack_1c0;
  ulonglong local_1b8;
  ulonglong uStack_1b0;
  undefined8 local_150;
  uint local_148;
  uint local_130;
  ushort uStack_12c;
  uint local_128;
  byte bStack_124;
  longlong local_118;
  
  puVar7 = (ulonglong *)*in_R8;
  lVar28 = in_R8[1];
  uVar8 = *puVar7;
  uVar24 = 0;
  uVar23 = uVar8 - *(ulonglong *)(lVar28 + 0x18);
  if (uVar8 < *(ulonglong *)(lVar28 + 0x18)) {
    uVar23 = uVar24;
  }
  uVar27 = uVar8 - *(ulonglong *)(lVar28 + 0x38);
  if (uVar8 < *(ulonglong *)(lVar28 + 0x38)) {
    uVar27 = uVar24;
  }
  lVar37 = uVar23 * 0x18 + *(longlong *)(lVar28 + 8);
  uVar8 = *(ulonglong *)(lVar28 + 0x10) - uVar23;
  if (*(ulonglong *)(lVar28 + 0x10) < uVar23) {
    lVar37 = 8;
    uVar8 = uVar24;
  }
  lVar22 = uVar27 * 0x18 + *(longlong *)(lVar28 + 0x28);
  uVar23 = *(ulonglong *)(lVar28 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar28 + 0x30) < uVar27) {
    lVar22 = 8;
    uVar23 = uVar24;
  }
  *puVar7 = (*(longlong *)(lVar28 + 0x40) - uVar23) - uVar8;
  auVar19 = ___xmm_7465646279746573646f72616e646f6d;
  auVar18 = ___xmm_6c7967656e657261736f6d6570736575;
  lVar28 = lVar22 + uVar23 * 0x18;
  puVar9 = *(undefined4 **)(in_R9 + 0x10);
  uVar1 = *(undefined4 *)(in_R9 + 0x1c);
  uVar2 = *(undefined4 *)(in_stack_00000028 + 0x1c);
  puVar10 = *(undefined4 **)(in_stack_00000028 + 0x10);
  uVar3 = *(undefined4 *)(in_stack_00000038 + 0x1c);
  puVar11 = *(undefined4 **)(in_stack_00000038 + 0x10);
  lVar29 = lVar37;
  if (lVar37 != 0) goto LAB_14081a54b;
LAB_14081a560:
  if ((lVar22 == 0) || (lVar22 == lVar28)) {
    puVar7 = (ulonglong *)*in_RCX;
    lVar28 = in_RCX[1];
    uVar8 = *puVar7;
    uVar24 = 0;
    uVar23 = uVar8 - *(ulonglong *)(lVar28 + 0x18);
    if (uVar8 < *(ulonglong *)(lVar28 + 0x18)) {
      uVar23 = uVar24;
    }
    uVar27 = uVar8 - *(ulonglong *)(lVar28 + 0x38);
    if (uVar8 < *(ulonglong *)(lVar28 + 0x38)) {
      uVar27 = uVar24;
    }
    lVar37 = uVar23 * 0x28 + *(longlong *)(lVar28 + 8);
    uVar8 = *(ulonglong *)(lVar28 + 0x10) - uVar23;
    if (*(ulonglong *)(lVar28 + 0x10) < uVar23) {
      lVar37 = 8;
      uVar8 = uVar24;
    }
    lVar22 = uVar27 * 0x28 + *(longlong *)(lVar28 + 0x28);
    uVar23 = *(ulonglong *)(lVar28 + 0x30) - uVar27;
    if (*(ulonglong *)(lVar28 + 0x30) < uVar27) {
      lVar22 = 8;
      uVar23 = uVar24;
    }
    *puVar7 = (*(longlong *)(lVar28 + 0x40) - uVar23) - uVar8;
    lVar28 = lVar37 + uVar8 * 0x28;
    lVar29 = lVar22 + uVar23 * 0x28;
    uVar6 = *(undefined4 *)(in_stack_00000048 + 0x1c);
    puVar14 = *(undefined4 **)(in_stack_00000048 + 0x10);
    if (lVar37 == 0) goto LAB_14081b100;
    do {
      if (lVar37 == lVar28) goto LAB_14081b100;
      lVar37 = lVar37 + 0x28;
      while( true ) {
        *puVar7 = *puVar7 + 1;
        *puVar9 = uVar1;
        *puVar10 = uVar2;
        *puVar11 = uVar3;
        apply_decorator_op::apply_decorator_op();
        *puVar14 = uVar6;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_06 == (undefined8 *)0x0) goto LAB_14081b4be;
        *extraout_RAX_06 = 1;
        extraout_RAX_06[1] = 1;
        extraout_RAX_06[2] = 5;
        country_core::resources::history::History::add();
        if (lVar37 != 0) break;
LAB_14081b100:
        if ((lVar22 == 0) || (lVar22 == lVar29)) {
          puVar7 = (ulonglong *)*in_RDX;
          lVar28 = in_RDX[1];
          uVar8 = *puVar7;
          uVar24 = 0;
          uVar23 = uVar8 - *(ulonglong *)(lVar28 + 0x18);
          if (uVar8 < *(ulonglong *)(lVar28 + 0x18)) {
            uVar23 = uVar24;
          }
          uVar27 = uVar8 - *(ulonglong *)(lVar28 + 0x38);
          if (uVar8 < *(ulonglong *)(lVar28 + 0x38)) {
            uVar27 = uVar24;
          }
          lVar37 = uVar23 * 0x30 + *(longlong *)(lVar28 + 8);
          uVar8 = *(ulonglong *)(lVar28 + 0x10) - uVar23;
          if (*(ulonglong *)(lVar28 + 0x10) < uVar23) {
            lVar37 = 8;
            uVar8 = uVar24;
          }
          lVar22 = uVar27 * 0x30 + *(longlong *)(lVar28 + 0x28);
          uVar23 = *(ulonglong *)(lVar28 + 0x30) - uVar27;
          if (*(ulonglong *)(lVar28 + 0x30) < uVar27) {
            lVar22 = 8;
            uVar23 = uVar24;
          }
          *puVar7 = (*(longlong *)(lVar28 + 0x40) - uVar23) - uVar8;
          lVar28 = uVar8 * 0x30 + lVar37;
          lVar29 = uVar23 * 0x30 + lVar22;
          if (lVar37 == 0) goto LAB_14081b2f0;
          goto LAB_14081b2d0;
        }
        lVar22 = lVar22 + 0x28;
        lVar37 = 0;
      }
    } while( true );
  }
  lVar31 = 0;
  lVar12 = lVar22 + 0x18;
  lVar29 = lVar22;
  do {
    lVar22 = lVar12;
    *puVar7 = *puVar7 + 1;
    iVar25 = *(int *)(lVar29 + 0x10);
    country_core::resources::stairs::stairs_state::StairSegment::new();
    iVar26 = *(int *)(lVar29 + 0x10);
    country_core::resources::stairs::stairs_state::StairSegment::new();
    country_core::resources::stairs::stairs_state::StairSegment::new();
    country_core::resources::walls::decorator_storage::DecoratorStorage::get();
    if ((extraout_RAX != 0) &&
       (hashbrown::raw::impl$13::
        clone<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,alloc::alloc::Global>
                  (), local_258 != (undefined1 (*) [16])0x0)) {
      auVar41 = *local_258;
      uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar41 >> 7,0) & 1) |
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
      pauVar36 = local_258 + 1;
      local_118 = local_240;
      pauVar30 = local_258;
      while (local_118 != 0) {
        if ((short)uVar20 == 0) {
          do {
            auVar41 = *pauVar36;
            uVar34 = (ushort)(SUB161(auVar41 >> 7,0) & 1) |
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
                     (ushort)(byte)(auVar41[0xf] >> 7) << 0xf;
            pauVar30 = pauVar30 + -0x50;
            pauVar36 = pauVar36 + 1;
          } while (uVar34 == 0xffff);
          uVar21 = ~(uint)uVar34;
          uVar20 = uVar21 & -(uint)uVar34 - 2;
        }
        else {
          uVar21 = uVar20;
          uVar20 = uVar20 - 1 & uVar20;
        }
        local_118 = local_118 + -1;
        uVar16 = 0;
        for (; (uVar21 & 1) == 0; uVar21 = uVar21 >> 1 | 0x80000000) {
          uVar16 = uVar16 + 1;
        }
        uVar23 = (ulonglong)uVar16;
        if (pauVar30[uVar23 * -5 + -3][10] == '\x04') {
          iVar4 = *(int *)(pauVar30[uVar23 * -5 + -5] + 8);
          iVar5 = *(int *)(pauVar30[uVar23 * -5 + -5] + 0xc);
          if ((iVar5 == iVar25 && iVar4 == extraout_EAX) ||
             ((iVar4 == extraout_EAX_00 && (iVar5 == iVar26)))) {
            _ZN120__LT_bevy_ecs__system__schedule_system__InfallibleSystemWrapper_LT_S_GT__u20_as_u20_bevy_ecs__system__system__System_GT_26archetype_component_access17h41094436587a7fc8E_llvm_9579840674613932800
                      ();
            country_core::resources::stairs::stairs_assembly_params::impl$2::from();
            if (extraout_RAX_00[3] != 0) {
              auVar41 = *(undefined1 (*) [16])(extraout_RAX_00 + 4);
              auVar45._0_8_ = auVar41._0_8_;
              auVar45._8_4_ = auVar41._0_4_;
              auVar45._12_4_ = auVar41._4_4_;
              auVar45 = auVar45 ^ auVar18;
              auVar46._0_8_ = auVar41._8_8_;
              auVar46._8_4_ = auVar41._8_4_;
              auVar46._12_4_ = auVar41._12_4_;
              auVar46 = auVar46 ^ auVar19;
              local_130 = (uint)extraout_RAX_01;
              uStack_12c = (ushort)((ulonglong)extraout_RAX_01 >> 0x20);
              core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
              local_1b8 = auVar46._0_8_;
              uStack_1b0 = auVar46._8_8_;
              local_1c8 = auVar45._0_8_;
              uVar32 = local_1c8 + local_1b8;
              lStack_1c0 = auVar45._8_8_;
              uVar24 = (local_1b8 << 0xd | local_1b8 >> 0x33) ^ uVar32;
              uVar23 = (uStack_1b0 << 0x10 | uStack_1b0 >> 0x30) ^ lStack_1c0 + uStack_1b0;
              uVar27 = lStack_1c0 + uStack_1b0 + uVar24;
              uVar32 = (uVar32 << 0x20 | uVar32 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0x11 | uVar24 >> 0x2f) ^ uVar27;
              uVar23 = (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar32;
              uVar32 = uVar32 + uVar24;
              uVar27 = ((uVar27 << 0x20 | uVar27 >> 0x20) ^ 0xff) + uVar23;
              uVar24 = (uVar24 << 0xd | uVar24 >> 0x33) ^ uVar32;
              uVar23 = (uVar23 << 0x10 | uVar23 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar24;
              uVar32 = (uVar32 << 0x20 | uVar32 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0x11 | uVar24 >> 0x2f) ^ uVar27;
              uVar23 = (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar32;
              uVar32 = uVar32 + uVar24;
              uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0xd | uVar24 >> 0x33) ^ uVar32;
              uVar23 = (uVar23 << 0x10 | uVar23 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar24;
              uVar32 = (uVar32 << 0x20 | uVar32 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0x11 | uVar24 >> 0x2f) ^ uVar27;
              uVar23 = (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar32;
              uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0xd | uVar24 >> 0x33) ^ uVar32 + uVar24;
              uVar23 = (uVar23 << 0x10 | uVar23 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar24;
              uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) ^ (uVar24 << 0x11 | uVar24 >> 0x2f) ^
                       (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar27;
              lVar12 = *extraout_RAX_00;
              bVar39 = (byte)(uVar27 >> 0x39);
              auVar41 = ZEXT216(CONCAT11(bVar39,bVar39));
              auVar41 = pshuflw(auVar41,auVar41,0);
              lVar33 = 0;
              do {
                uVar27 = uVar27 & extraout_RAX_00[1];
                auVar45 = *(undefined1 (*) [16])(lVar12 + uVar27);
                cVar40 = auVar41[0];
                auVar51[0] = -(auVar45[0] == cVar40);
                cVar42 = auVar41[1];
                auVar51[1] = -(auVar45[1] == cVar42);
                cVar43 = auVar41[2];
                auVar51[2] = -(auVar45[2] == cVar43);
                cVar44 = auVar41[3];
                auVar51[3] = -(auVar45[3] == cVar44);
                auVar51[4] = -(auVar45[4] == cVar40);
                auVar51[5] = -(auVar45[5] == cVar42);
                auVar51[6] = -(auVar45[6] == cVar43);
                auVar51[7] = -(auVar45[7] == cVar44);
                auVar51[8] = -(auVar45[8] == cVar40);
                auVar51[9] = -(auVar45[9] == cVar42);
                auVar51[10] = -(auVar45[10] == cVar43);
                auVar51[0xb] = -(auVar45[0xb] == cVar44);
                auVar51[0xc] = -(auVar45[0xc] == cVar40);
                auVar51[0xd] = -(auVar45[0xd] == cVar42);
                auVar51[0xe] = -(auVar45[0xe] == cVar43);
                auVar51[0xf] = -(auVar45[0xf] == cVar44);
                uVar34 = (ushort)(SUB161(auVar51 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
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
                         (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar51[0xf] >> 7) << 0xf;
                uVar21 = (uint)uVar34;
                while (uVar34 != 0) {
                  uVar16 = 0;
                  for (uVar17 = uVar21; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
                    uVar16 = uVar16 + 1;
                  }
                  lVar35 = (uVar16 + uVar27 & extraout_RAX_00[1]) * -0x78;
                  if (extraout_RAX_01 == *(longlong *)(lVar12 + -0x78 + lVar35)) {
                    uVar23 = *(ulonglong *)(lVar12 + -0x60 + lVar35);
                    auVar41._8_8_ = 0;
                    auVar41._0_8_ = uVar23;
                    uVar24 = SUB168(auVar41 * ZEXT816(0xc),0);
                    if ((SUB168(auVar41 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar24)) {
LAB_14081b4e7:
                    // WARNING: Subroutine does not return
                      alloc::raw_vec::handle_error();
                    }
                    pvVar13 = *(void **)(lVar12 + -0x68 + lVar35);
                    if (uVar24 == 0) {
                      puVar38 = &DAT_00000004;
                      uVar23 = 0;
                    }
                    else {
                      std::alloc::__default_lib_allocator::__rust_alloc();
                      puVar38 = extraout_RAX_02;
                      if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_14081b4e7;
                    }
                    memcpy(puVar38,pvVar13,uVar24);
                    local_130 = (uint)uVar23;
                    uStack_12c = (ushort)(uVar23 >> 0x20);
                    local_128 = (uint)puVar38;
                    bStack_124 = (byte)((ulonglong)puVar38 >> 0x20);
                    ::utils::curve::Curve::
                    try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
                    if (!SBORROW8(0,local_1c8)) {
                      if (local_1c8 != 0) {
                        std::alloc::__default_lib_allocator::__rust_dealloc();
                      }
                      if (uStack_1b0 != 0) {
                        std::alloc::__default_lib_allocator::__rust_dealloc();
                      }
                    }
                    goto LAB_14081aa40;
                  }
                  uVar34 = (ushort)(uVar21 - 1) & (ushort)uVar21;
                  uVar21 = CONCAT22((short)(uVar21 - 1 >> 0x10),uVar34);
                }
                auVar47[0] = -(auVar45[0] == -1);
                auVar47[1] = -(auVar45[1] == -1);
                auVar47[2] = -(auVar45[2] == -1);
                auVar47[3] = -(auVar45[3] == -1);
                auVar47[4] = -(auVar45[4] == -1);
                auVar47[5] = -(auVar45[5] == -1);
                auVar47[6] = -(auVar45[6] == -1);
                auVar47[7] = -(auVar45[7] == -1);
                auVar47[8] = -(auVar45[8] == -1);
                auVar47[9] = -(auVar45[9] == -1);
                auVar47[10] = -(auVar45[10] == -1);
                auVar47[0xb] = -(auVar45[0xb] == -1);
                auVar47[0xc] = -(auVar45[0xc] == -1);
                auVar47[0xd] = -(auVar45[0xd] == -1);
                auVar47[0xe] = -(auVar45[0xe] == -1);
                auVar47[0xf] = -(auVar45[0xf] == -1);
                if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar47 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar47 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar47 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar47 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar47 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar47 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar47 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar47 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar47 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar47 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar47 >> 0x5f,0) & 1) != 0) || (SUB161(auVar47 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') break;
                uVar27 = uVar27 + lVar33 + 0x10;
                lVar33 = lVar33 + 0x10;
              } while( true );
            }
LAB_14081aa40:
            _ZN120__LT_bevy_ecs__system__schedule_system__InfallibleSystemWrapper_LT_S_GT__u20_as_u20_bevy_ecs__system__system__System_GT_26archetype_component_access17h41094436587a7fc8E_llvm_9579840674613932800
                      ();
            country_core::resources::stairs::stairs_assembly_params::impl$2::from();
            if (extraout_RAX_03[3] != 0) {
              auVar41 = *(undefined1 (*) [16])(extraout_RAX_03 + 4);
              auVar48._0_8_ = auVar41._0_8_;
              auVar48._8_4_ = auVar41._0_4_;
              auVar48._12_4_ = auVar41._4_4_;
              auVar48 = auVar48 ^ auVar18;
              auVar49._0_8_ = auVar41._8_8_;
              auVar49._8_4_ = auVar41._8_4_;
              auVar49._12_4_ = auVar41._12_4_;
              auVar49 = auVar49 ^ auVar19;
              local_130 = (uint)extraout_RAX_04;
              uStack_12c = (ushort)((ulonglong)extraout_RAX_04 >> 0x20);
              core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
              local_1b8 = auVar49._0_8_;
              uStack_1b0 = auVar49._8_8_;
              local_1c8 = auVar48._0_8_;
              uVar32 = local_1c8 + local_1b8;
              lStack_1c0 = auVar48._8_8_;
              uVar24 = (local_1b8 << 0xd | local_1b8 >> 0x33) ^ uVar32;
              uVar23 = (uStack_1b0 << 0x10 | uStack_1b0 >> 0x30) ^ lStack_1c0 + uStack_1b0;
              uVar27 = lStack_1c0 + uStack_1b0 + uVar24;
              uVar32 = (uVar32 << 0x20 | uVar32 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0x11 | uVar24 >> 0x2f) ^ uVar27;
              uVar23 = (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar32;
              uVar32 = uVar32 + uVar24;
              uVar27 = ((uVar27 << 0x20 | uVar27 >> 0x20) ^ 0xff) + uVar23;
              uVar24 = (uVar24 << 0xd | uVar24 >> 0x33) ^ uVar32;
              uVar23 = (uVar23 << 0x10 | uVar23 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar24;
              uVar32 = (uVar32 << 0x20 | uVar32 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0x11 | uVar24 >> 0x2f) ^ uVar27;
              uVar23 = (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar32;
              uVar32 = uVar32 + uVar24;
              uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0xd | uVar24 >> 0x33) ^ uVar32;
              uVar23 = (uVar23 << 0x10 | uVar23 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar24;
              uVar32 = (uVar32 << 0x20 | uVar32 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0x11 | uVar24 >> 0x2f) ^ uVar27;
              uVar23 = (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar32;
              uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar23;
              uVar24 = (uVar24 << 0xd | uVar24 >> 0x33) ^ uVar32 + uVar24;
              uVar23 = (uVar23 << 0x10 | uVar23 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar24;
              uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) ^ (uVar24 << 0x11 | uVar24 >> 0x2f) ^
                       (uVar23 << 0x15 | uVar23 >> 0x2b) ^ uVar27;
              lVar12 = *extraout_RAX_03;
              bVar39 = (byte)(uVar27 >> 0x39);
              auVar41 = ZEXT216(CONCAT11(bVar39,bVar39));
              auVar41 = pshuflw(auVar41,auVar41,0);
              lVar33 = 0;
              do {
                uVar27 = uVar27 & extraout_RAX_03[1];
                auVar45 = *(undefined1 (*) [16])(lVar12 + uVar27);
                cVar40 = auVar41[0];
                auVar52[0] = -(auVar45[0] == cVar40);
                cVar42 = auVar41[1];
                auVar52[1] = -(auVar45[1] == cVar42);
                cVar43 = auVar41[2];
                auVar52[2] = -(auVar45[2] == cVar43);
                cVar44 = auVar41[3];
                auVar52[3] = -(auVar45[3] == cVar44);
                auVar52[4] = -(auVar45[4] == cVar40);
                auVar52[5] = -(auVar45[5] == cVar42);
                auVar52[6] = -(auVar45[6] == cVar43);
                auVar52[7] = -(auVar45[7] == cVar44);
                auVar52[8] = -(auVar45[8] == cVar40);
                auVar52[9] = -(auVar45[9] == cVar42);
                auVar52[10] = -(auVar45[10] == cVar43);
                auVar52[0xb] = -(auVar45[0xb] == cVar44);
                auVar52[0xc] = -(auVar45[0xc] == cVar40);
                auVar52[0xd] = -(auVar45[0xd] == cVar42);
                auVar52[0xe] = -(auVar45[0xe] == cVar43);
                auVar52[0xf] = -(auVar45[0xf] == cVar44);
                uVar34 = (ushort)(SUB161(auVar52 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1 |
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
                         (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar52[0xf] >> 7) << 0xf;
                uVar21 = (uint)uVar34;
                while (uVar34 != 0) {
                  uVar16 = 0;
                  for (uVar17 = uVar21; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
                    uVar16 = uVar16 + 1;
                  }
                  lVar35 = (uVar16 + uVar27 & extraout_RAX_03[1]) * -0x78;
                  if (extraout_RAX_04 == *(longlong *)(lVar12 + -0x78 + lVar35)) {
                    uVar23 = *(ulonglong *)(lVar12 + -0x60 + lVar35);
                    auVar15._8_8_ = 0;
                    auVar15._0_8_ = uVar23;
                    uVar24 = SUB168(auVar15 * ZEXT816(0xc),0);
                    if ((SUB168(auVar15 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar24)) {
LAB_14081b4d0:
                    // WARNING: Subroutine does not return
                      alloc::raw_vec::handle_error();
                    }
                    pvVar13 = *(void **)(lVar12 + -0x68 + lVar35);
                    if (uVar24 == 0) {
                      puVar38 = &DAT_00000004;
                      uVar23 = 0;
                    }
                    else {
                      std::alloc::__default_lib_allocator::__rust_alloc();
                      puVar38 = extraout_RAX_05;
                      if (extraout_RAX_05 == (undefined1 *)0x0) goto LAB_14081b4d0;
                    }
                    memcpy(puVar38,pvVar13,uVar24);
                    local_130 = (uint)uVar23;
                    uStack_12c = (ushort)(uVar23 >> 0x20);
                    local_128 = (uint)puVar38;
                    bStack_124 = (byte)((ulonglong)puVar38 >> 0x20);
                    ::utils::curve::Curve::
                    try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
                    if (!SBORROW8(0,local_1c8)) {
                      if (local_1c8 != 0) {
                        std::alloc::__default_lib_allocator::__rust_dealloc();
                      }
                      if (uStack_1b0 != 0) {
                        std::alloc::__default_lib_allocator::__rust_dealloc();
                      }
                    }
                    goto LAB_14081ad80;
                  }
                  uVar34 = (ushort)(uVar21 - 1) & (ushort)uVar21;
                  uVar21 = CONCAT22((short)(uVar21 - 1 >> 0x10),uVar34);
                }
                auVar50[0] = -(auVar45[0] == -1);
                auVar50[1] = -(auVar45[1] == -1);
                auVar50[2] = -(auVar45[2] == -1);
                auVar50[3] = -(auVar45[3] == -1);
                auVar50[4] = -(auVar45[4] == -1);
                auVar50[5] = -(auVar45[5] == -1);
                auVar50[6] = -(auVar45[6] == -1);
                auVar50[7] = -(auVar45[7] == -1);
                auVar50[8] = -(auVar45[8] == -1);
                auVar50[9] = -(auVar45[9] == -1);
                auVar50[10] = -(auVar45[10] == -1);
                auVar50[0xb] = -(auVar45[0xb] == -1);
                auVar50[0xc] = -(auVar45[0xc] == -1);
                auVar50[0xd] = -(auVar45[0xd] == -1);
                auVar50[0xe] = -(auVar45[0xe] == -1);
                auVar50[0xf] = -(auVar45[0xf] == -1);
                if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar50 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar50 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar50 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar50 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar50 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar50 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar50 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar50 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar50 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar50 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar50 >> 0x5f,0) & 1) != 0) || (SUB161(auVar50 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar50 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar50 >> 0x77,0) & 1) != 0) || auVar50[0xf] < '\0') break;
                uVar27 = uVar27 + lVar33 + 0x10;
                lVar33 = lVar33 + 0x10;
              } while( true );
            }
LAB_14081ad80:
            if (iVar5 == iVar25 && iVar4 == extraout_EAX) {
              if ((*(uint *)(lVar29 + 0x10) <= *(uint *)(lVar29 + 8)) ||
                 (*(uint *)(lVar29 + 0xc) <= *(uint *)(lVar29 + 8))) {
                ::utils::inverse_lerp();
              }
            }
            else if ((*(uint *)(lVar29 + 0xc) <= *(uint *)(lVar29 + 0x10)) ||
                    (*(uint *)(lVar29 + 0xc) <= *(uint *)(lVar29 + 8))) {
              ::utils::inverse_lerp();
            }
            bevy_app::plugin::Plugin::ready<country_core::systems::main_camera::MainCameraPlugin>();
            country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
            try_position();
            if ((local_130 & 1) == 0) {
              local_148 = 0;
              local_150 = 0;
            }
            else {
              local_148 = (uint)bStack_124;
              local_150 = CONCAT44(local_128,(uint)uStack_12c);
            }
            local_130 = (uint)local_150;
            uStack_12c = (ushort)((ulonglong)local_150 >> 0x20);
            local_128 = local_148;
            bStack_124 = 0;
            *puVar9 = uVar1;
            *puVar10 = uVar2;
            *puVar11 = uVar3;
            apply_decorator_op::apply_decorator_op();
          }
        }
      }
      if ((CONCAT44(uStack_24c,local_250) != 0) && (CONCAT44(uStack_24c,local_250) * 0x51 != -0x61))
      {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    lVar29 = lVar31;
    if (lVar31 == 0) goto LAB_14081a560;
LAB_14081a54b:
    if (lVar29 == uVar8 * 0x18 + lVar37) goto LAB_14081a560;
    lVar31 = lVar29 + 0x18;
    lVar12 = lVar22;
  } while( true );
LAB_14081b2d0:
  if (lVar37 == lVar28) goto LAB_14081b2f0;
  lVar37 = lVar37 + 0x30;
  do {
    *puVar7 = *puVar7 + 1;
    *puVar9 = uVar1;
    *puVar10 = uVar2;
    *puVar11 = uVar3;
    apply_decorator_op::apply_decorator_op();
    *puVar14 = uVar6;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_07 == (undefined8 *)0x0) {
LAB_14081b4be:
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_07 = 1;
    extraout_RAX_07[1] = 1;
    extraout_RAX_07[2] = 3;
    country_core::resources::history::History::add();
    if (lVar37 != 0) goto LAB_14081b2d0;
LAB_14081b2f0:
    if ((lVar22 == 0) || (lVar22 == lVar29)) {
      return;
    }
    lVar22 = lVar22 + 0x30;
    lVar37 = 0;
  } while( true );
}


