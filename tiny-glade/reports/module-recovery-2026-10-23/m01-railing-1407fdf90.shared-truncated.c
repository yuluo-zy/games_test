
void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing(void)

{
  longlong lVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  bool bVar14;
  ulonglong *in_RCX;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
  float in_XMM3_Da;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000040;
  
  if (in_stack_00000030 != 0) {
    lVar1 = in_stack_00000028 + in_stack_00000030 * 0xc;
    lVar18 = 0;
    lVar16 = in_stack_00000028;
    do {
      fastrand::Rng::f32();
      fastrand::Rng::f32();
      fastrand::Rng::f32();
      if (lVar18 == in_stack_00000040) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      lVar18 = lVar18 + 1;
      ::utils::swizzlers::impl$0::x0y();
      ::utils::swizzlers::impl$0::x0y();
      country_core::resources::render::Affine3Packed::from_cols();
      uVar17 = *in_RCX;
      uVar15 = uVar17 + 0xa0761d6478bd642f;
      *in_RCX = uVar15;
      auVar2._8_8_ = 0;
      auVar2._0_8_ = uVar15 ^ 0xe7037ed1a0b428db;
      auVar8._8_8_ = 0;
      auVar8._0_8_ = uVar15;
      if ((SUB164(auVar2 * auVar8,8) ^ SUB164(auVar2 * auVar8,0)) * 1000 < 0x128) {
        uVar17 = uVar17 + 0x40ec3ac8f17ac85e;
        do {
          uVar15 = uVar15 + 0xa0761d6478bd642f;
          auVar3._8_8_ = 0;
          auVar3._0_8_ = uVar17 ^ 0xe7037ed1a0b428db;
          auVar9._8_8_ = 0;
          auVar9._0_8_ = uVar17;
          uVar17 = uVar17 + 0xa0761d6478bd642f;
        } while ((SUB164(auVar3 * auVar9,8) ^ SUB164(auVar3 * auVar9,0)) * 1000 < 0x128);
        *in_RCX = uVar15;
      }
      lVar16 = lVar16 + 0xc;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
    } while (lVar16 != lVar1);
    if (in_stack_00000030 != 0) {
      bVar14 = true;
      lVar18 = 0;
      do {
        lVar16 = in_stack_00000028 + 0xc;
        if (bVar14) {
          if (lVar16 == lVar1) {
            return;
          }
          lVar16 = in_stack_00000028 + 0x18;
        }
        fastrand::Rng::f32();
        fastrand::Rng::f32();
        fastrand::Rng::f32();
        if (lVar18 == in_stack_00000040) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        country_core::resources::render::Affine3Packed::from_cols();
        if (in_XMM3_Da < 1.6) {
          uVar17 = *in_RCX;
          uVar15 = uVar17 + 0xa0761d6478bd642f;
          *in_RCX = uVar15;
          auVar4._8_8_ = 0;
          auVar4._0_8_ = uVar15 ^ 0xe7037ed1a0b428db;
          auVar10._8_8_ = 0;
          auVar10._0_8_ = uVar15;
          if ((SUB164(auVar4 * auVar10,8) ^ SUB164(auVar4 * auVar10,0)) * 1000 < 0x128) {
            uVar17 = uVar17 + 0x40ec3ac8f17ac85e;
            do {
              uVar15 = uVar15 + 0xa0761d6478bd642f;
              auVar5._8_8_ = 0;
              auVar5._0_8_ = uVar17 ^ 0xe7037ed1a0b428db;
              auVar11._8_8_ = 0;
              auVar11._0_8_ = uVar17;
              uVar17 = uVar17 + 0xa0761d6478bd642f;
            } while ((SUB164(auVar5 * auVar11,8) ^ SUB164(auVar5 * auVar11,0)) * 1000 < 0x128);
            *in_RCX = uVar15;
          }
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        }
        fastrand::Rng::f32();
        fastrand::Rng::f32();
        fastrand::Rng::f32();
        country_core::resources::render::Affine3Packed::from_cols();
        uVar17 = *in_RCX;
        uVar15 = uVar17 + 0xa0761d6478bd642f;
        *in_RCX = uVar15;
        auVar6._8_8_ = 0;
        auVar6._0_8_ = uVar15 ^ 0xe7037ed1a0b428db;
        auVar12._8_8_ = 0;
        auVar12._0_8_ = uVar15;
        if ((SUB164(auVar6 * auVar12,8) ^ SUB164(auVar6 * auVar12,0)) * 1000 < 0x128) {
          uVar17 = uVar17 + 0x40ec3ac8f17ac85e;
          do {
            uVar15 = uVar15 + 0xa0761d6478bd642f;
            auVar7._8_8_ = 0;
            auVar7._0_8_ = uVar17 ^ 0xe7037ed1a0b428db;
            auVar13._8_8_ = 0;
            auVar13._0_8_ = uVar17;
            uVar17 = uVar17 + 0xa0761d6478bd642f;
          } while ((SUB164(auVar7 * auVar13,8) ^ SUB164(auVar7 * auVar13,0)) * 1000 < 0x128);
          *in_RCX = uVar15;
        }
        lVar18 = lVar18 + 1;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        bVar14 = false;
        in_stack_00000028 = lVar16;
      } while (lVar16 != lVar1);
    }
  }
  return;
}


