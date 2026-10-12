
void __cdecl country_core::systems::wall::deferred_wall_history::PendingWallChanged::try_new(void)

{
  byte bVar1;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  longlong *in_R8;
  longlong *in_R9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar2;
  float fVar3;
  float fVar4;
  char in_stack_00000028;
  longlong local_158;
  longlong lStack_150;
  longlong local_148;
  longlong lStack_140;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  longlong lStack_120;
  longlong local_118;
  longlong lStack_110;
  longlong local_108;
  longlong lStack_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  longlong local_c8;
  longlong lStack_c0;
  longlong local_b8;
  longlong lStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  longlong lStack_80;
  longlong *local_70;
  longlong *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  if (in_R8[2] == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (in_R9[2] == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  local_70 = in_R8;
  local_68 = in_R9;
  if (in_stack_00000028 == '\0') {
    if (in_R8[1] + in_R8[2] * 8 == 8) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    fVar2 = 0.0;
    if (0.0 <= extraout_XMM0_Da_00) {
      fVar2 = extraout_XMM0_Da_00;
    }
    fVar3 = 0.0;
    fVar4 = __real_3f800000;
    if (fVar2 <= __real_3f800000) {
      fVar4 = fVar2;
    }
  }
  else {
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    fVar2 = 0.0;
    if (0.0 <= extraout_XMM0_Da) {
      fVar2 = extraout_XMM0_Da;
    }
    fVar3 = __real_3f800000;
    fVar4 = __real_3f800000;
    if (fVar2 <= __real_3f800000) {
      fVar3 = fVar2;
    }
  }
  bVar1 = -(fVar3 < fVar4);
  if (fVar4 < fVar3) {
    bVar1 = (fVar3 < fVar4) * '\x02';
  }
  if (bVar1 < 2) {
    *in_RCX = 0x8000000000000000;
    if (*local_68 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_68[3] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (*local_70 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_70[3] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    local_f8 = local_70[0xc];
    lStack_f0 = local_70[0xd];
    local_108 = local_70[10];
    lStack_100 = local_70[0xb];
    local_118 = local_70[8];
    lStack_110 = local_70[9];
    local_158 = *local_70;
    lStack_150 = local_70[1];
    local_148 = local_70[2];
    lStack_140 = local_70[3];
    local_138 = local_70[4];
    lStack_130 = local_70[5];
    local_128 = local_70[6];
    lStack_120 = local_70[7];
    local_e8 = *local_68;
    lStack_e0 = local_68[1];
    local_d8 = local_68[2];
    lStack_d0 = local_68[3];
    local_c8 = local_68[4];
    lStack_c0 = local_68[5];
    local_b8 = local_68[6];
    lStack_b0 = local_68[7];
    local_a8 = local_68[8];
    lStack_a0 = local_68[9];
    local_98 = local_68[10];
    lStack_90 = local_68[0xb];
    local_88 = local_68[0xc];
    lStack_80 = local_68[0xd];
    memcpy(in_RCX,&local_158,0xe0);
    in_RCX[0x1c] = in_RDX;
    *(undefined4 *)(in_RCX + 0x1d) = 0;
    *(float *)((longlong)in_RCX + 0xec) = fVar3;
    *(float *)(in_RCX + 0x1e) = fVar4;
  }
  return;
}

