// name: overlapping_windows
// addr: 0x1412218a0
// size: 1061
// signature: void __cdecl overlapping_windows(void)


void __cdecl system_wall_constructor::utils::half_timber::overlapping_windows(void)

{
  undefined1 auVar1 [16];
  ushort uVar2;
  uint uVar3;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  ulonglong uVar4;
  uint uVar5;
  uint uVar6;
  undefined1 (*pauVar7) [16];
  longlong lVar8;
  undefined1 uVar9;
  undefined1 (*pauVar10) [16];
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  undefined1 extraout_XMM0 [16];
  float in_stack_00000030;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_128;
  undefined8 uStack_120;
  longlong local_f8;
  float local_f0;
  undefined4 uStack_ec;
  float fStack_e8;
  undefined4 uStack_e4;
  
  local_f8 = 0;
  country_core::resources::walls::decorator_storage::DecoratorStorage::get();
  if (extraout_RAX != 0) {
    pauVar10 = *(undefined1 (**) [16])(extraout_RAX + 0x30);
    lVar8 = *(longlong *)(extraout_RAX + 0x48);
    auVar1 = *pauVar10;
    uVar5 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
    pauVar7 = pauVar10 + 1;
LAB_1412219b0:
    if (lVar8 != 0) {
      if ((short)uVar5 == 0) {
        do {
          auVar1 = *pauVar7;
          uVar2 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          pauVar10 = pauVar10 + -0x50;
          pauVar7 = pauVar7 + 1;
        } while (uVar2 == 0xffff);
        uVar6 = ~(uint)uVar2;
        uVar5 = uVar6 & -(uint)uVar2 - 2;
      }
      else {
        uVar6 = uVar5;
        uVar5 = uVar5 - 1 & uVar5;
      }
      uVar3 = 0;
      for (; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar3 = uVar3 + 1;
      }
      uVar4 = (ulonglong)uVar3;
      lVar8 = lVar8 + -1;
      if (((byte)pauVar10[uVar4 * -5 + -3][10] < 2) &&
         ((byte)(pauVar10[uVar4 * -5 + -1][0xe] - 0x38) < 3)) {
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
        get_window_aabb();
        local_f0 = local_f0 + 0.038;
        fStack_e8 = fStack_e8 + -0.038;
        if ((extraout_AL == '\t') && (pauVar10[uVar4 * -5 + -1][0xc] == '\x01')) {
          local_f0 = local_f0 + -0.14;
          fStack_e8 = fStack_e8 + 0.14;
        }
        ::utils::geometry::aabb::Aabb2::intersects();
        if (extraout_AL_00 != '\0') {
          uVar9 = pauVar10[uVar4 * -5 + -1][0xc];
          if (local_f8 == 0) {
            alloc::raw_vec::RawVec::
            grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
          }
          *(ulonglong *)(local_f8 * 0x14 + 4) = CONCAT44(uStack_ec,local_f0);
          *(ulonglong *)(local_f8 * 0x14 + 0xc) = CONCAT44(uStack_e4,fStack_e8);
          *(undefined1 *)(local_f8 * 0x14 + 0x14) = uVar9;
          *(char *)(local_f8 * 0x14 + 0x15) = extraout_AL;
          *(undefined1 *)(local_f8 * 0x14 + 0x16) = 0;
          local_f8 = local_f8 + 1;
        }
        ::utils::geometry::aabb::Aabb2::center();
        ::utils::geometry::aabb::Aabb2::width();
        if (extraout_XMM0_Da + extraout_XMM0_Da_00 * 0.5 <= in_stack_00000030) {
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          if (extraout_XMM0._0_4_ * 0.5 <= extraout_XMM0_Da_01) goto LAB_1412219b0;
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          ::utils::geometry::aabb::Aabb2::intersects();
          if (extraout_AL_02 == '\0') goto LAB_1412219b0;
          uVar9 = pauVar10[uVar4 * -5 + -1][0xc];
          local_128 = local_150;
          uStack_120 = uStack_148;
          if (local_f8 == 0) {
            alloc::raw_vec::RawVec::
            grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
          }
        }
        else {
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          ::utils::geometry::aabb::Aabb2::intersects();
          if (extraout_AL_01 == '\0') goto LAB_1412219b0;
          uVar9 = pauVar10[uVar4 * -5 + -1][0xc];
          local_128 = local_140;
          uStack_120 = uStack_138;
          if (local_f8 == 0) {
            alloc::raw_vec::RawVec::
            grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
          }
        }
        auVar1._8_8_ = uStack_120;
        auVar1._0_8_ = local_128;
        *(undefined1 (*) [16])(local_f8 * 0x14 + 4) = auVar1;
        *(undefined1 *)(local_f8 * 0x14 + 0x14) = uVar9;
        *(char *)(local_f8 * 0x14 + 0x15) = extraout_AL;
        *(undefined1 *)(local_f8 * 0x14 + 0x16) = 1;
        local_f8 = local_f8 + 1;
      }
      goto LAB_1412219b0;
    }
  }
  in_RCX[2] = local_f8;
  *in_RCX = 0;
  in_RCX[1] = 4;
  return;
}

