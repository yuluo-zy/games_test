
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
utils::threshold_range::
threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env_0<system_wall_constructor::construct_fences::construct_fences::closure_env_0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env_1<system_wall_constructor::construct_fences::construct_fences::closure_env_0>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong uVar3;
  byte *pbVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined1 auVar8 [16];
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong in_RCX;
  ulonglong uVar17;
  ulonglong in_RDX;
  longlong lVar18;
  undefined8 *in_R9;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float in_XMM2_Da;
  uint uVar31;
  undefined1 auVar30 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  
  fVar16 = _UNK_142ac2e3c;
  fVar15 = _UNK_142ac2e38;
  fVar14 = _UNK_142ac2e34;
  fVar13 = ___xmm_00000000000000007f7fffff7f7fffff;
  fVar12 = _UNK_142ac2e2c;
  fVar11 = _UNK_142ac2e28;
  fVar10 = _UNK_142ac2e24;
  fVar9 = ___xmm_0000000000000000ff7fffffff7fffff;
  if (in_RDX != 0) {
    lVar2 = in_R9[3];
    uVar3 = in_R9[5];
    pbVar4 = (byte *)in_R9[6];
    plVar5 = (longlong *)*in_R9;
    plVar6 = (longlong *)in_R9[1];
    plVar7 = (longlong *)in_R9[8];
    uVar20 = 0;
    do {
      while( true ) {
        uVar17 = uVar20 + 1;
        if (uVar20 + 1 < in_RDX) {
          uVar17 = in_RDX;
        }
        uVar19 = uVar20;
        if (*(float *)(in_RCX + uVar20 * 4) <= in_XMM2_Da) break;
        do {
          if (uVar17 - 1 == uVar20) {
            return;
          }
          lVar1 = uVar20 * 4;
          uVar20 = uVar20 + 1;
        } while (in_XMM2_Da < *(float *)(in_RCX + 4 + lVar1));
        if (in_RDX <= uVar20) {
          return;
        }
      }
      do {
        uVar21 = uVar17;
        if (uVar17 - 1 == uVar19) break;
        uVar21 = uVar19 + 1;
        lVar1 = uVar19 * 4;
        uVar19 = uVar21;
      } while (*(float *)(in_RCX + 4 + lVar1) <= in_XMM2_Da);
      if (uVar21 < uVar20) {
                    /* WARNING: Subroutine does not return */
        core::slice::index::slice_index_order_fail();
      }
      if (uVar3 < uVar21) {
                    /* WARNING: Subroutine does not return */
        core::slice::index::slice_end_index_len_fail();
      }
      if ((*pbVar4 & 1) == 0) {
        lVar1 = *(longlong *)(lVar2 + 0x10);
        fVar22 = fVar13;
        fVar23 = fVar14;
        fVar26 = fVar9;
        fVar27 = fVar10;
        if (lVar1 != 0) {
          lVar18 = 0;
          fVar24 = fVar15;
          fVar25 = fVar16;
          fVar28 = fVar11;
          fVar29 = fVar12;
          do {
            uVar17 = *(ulonglong *)(*(longlong *)(lVar2 + 8) + lVar18);
            auVar30._8_8_ = 0;
            auVar30._0_8_ = uVar17;
            uVar31 = (uint)(uVar17 >> 0x20);
            auVar32._8_8_ = 0;
            auVar32._0_8_ = uVar17;
            auVar8._4_4_ = fVar23;
            auVar8._0_4_ = fVar22;
            auVar8._8_4_ = fVar24;
            auVar8._12_4_ = fVar25;
            auVar33 = minps(auVar32,auVar8);
            fVar24 = (float)(~-(uint)NAN(fVar24) & auVar33._8_4_);
            fVar25 = (float)(~-(uint)NAN(fVar25) & auVar33._12_4_);
            fVar22 = (float)(~-(uint)NAN(fVar22) & auVar33._0_4_ | (uint)uVar17 & -(uint)NAN(fVar22)
                            );
            fVar23 = (float)(~-(uint)NAN(fVar23) & auVar33._4_4_ | uVar31 & -(uint)NAN(fVar23));
            auVar33._4_4_ = fVar27;
            auVar33._0_4_ = fVar26;
            auVar33._8_4_ = fVar28;
            auVar33._12_4_ = fVar29;
            auVar33 = maxps(auVar30,auVar33);
            fVar28 = (float)(~-(uint)NAN(fVar28) & auVar33._8_4_);
            fVar29 = (float)(~-(uint)NAN(fVar29) & auVar33._12_4_);
            fVar26 = (float)(~-(uint)NAN(fVar26) & auVar33._0_4_ | (uint)uVar17 & -(uint)NAN(fVar26)
                            );
            fVar27 = (float)(~-(uint)NAN(fVar27) & auVar33._4_4_ | uVar31 & -(uint)NAN(fVar27));
            lVar18 = lVar18 + 8;
          } while (lVar1 << 3 != lVar18);
        }
        pbVar4[0] = 1;
        pbVar4[1] = 0;
        pbVar4[2] = 0;
        pbVar4[3] = 0;
        *(float *)(pbVar4 + 4) = fVar22;
        *(float *)(pbVar4 + 8) = fVar23;
        *(float *)(pbVar4 + 0xc) = fVar26;
        *(float *)(pbVar4 + 0x10) = fVar27;
      }
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      system_wall_constructor::construct_fences::process_segment::
      process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env_0>();
      *plVar5 = *plVar5 + extraout_RAX;
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      *plVar6 = *plVar6 + extraout_RAX_00 + uVar21 + uVar20;
      lVar1 = plVar7[2];
      if (lVar1 == *plVar7) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar18 = plVar7[1];
      *(ulonglong *)(lVar18 + lVar1 * 0x10) = uVar20;
      *(ulonglong *)(lVar18 + 8 + lVar1 * 0x10) = uVar21;
      plVar7[2] = lVar1 + 1;
      uVar20 = uVar21;
    } while (uVar21 < in_RDX);
  }
  return;
}

