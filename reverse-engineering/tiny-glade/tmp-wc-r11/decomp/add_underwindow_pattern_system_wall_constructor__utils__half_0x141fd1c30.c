// name: add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>
// addr: 0x141fd1c30
// size: 1907
// signature: void __cdecl add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>(void)


void __cdecl
system_wall_constructor::utils::half_timber::
add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env_0>
          (void)

{
  byte bVar1;
  char extraout_AL;
  int extraout_EAX;
  longlong in_RCX;
  longlong in_RDX;
  float *pfVar2;
  longlong lVar3;
  longlong lVar4;
  float fVar5;
  float fVar6;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined1 *in_stack_00000028;
  float in_stack_00000030;
  byte *in_stack_00000040;
  undefined8 *in_stack_00000048;
  int in_stack_00000058;
  float in_stack_00000060;
  float local_218;
  float local_214;
  float local_210;
  float local_20c;
  float local_208;
  float local_204;
  float fStack_1f0;
  float fStack_1ec;
  undefined8 local_1b8;
  float fStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  float fStack_1a4;
  undefined1 *local_190;
  byte *local_188;
  undefined8 local_180;
  float local_178 [7];
  float local_15c;
  float local_154;
  float local_14c;
  longlong local_140;
  undefined8 *local_138;
  code *local_130;
  undefined4 local_108;
  int local_104;
  undefined4 local_fc;
  float local_f8;
  float fStack_f4;
  undefined8 uStack_f0;
  
  if ((*(byte *)(in_RCX + 0x11) & 0x1e) != 0x16) {
    local_178[1] = *(float *)(in_RCX + 4);
    fVar6 = *(float *)(in_RDX + 4);
    fVar9 = local_178[1] - fVar6;
    if (fVar9 <= 0.0) {
      fVar9 = 0.0;
    }
    if ((0.5 < fVar9) && (fVar9 < 2.0)) {
      fVar9 = in_XMM3_Da * 0.5 + in_XMM2_Da * 0.5;
      *in_stack_00000028 = 1;
      local_178[1] = local_178[1] + 0.13;
      local_178[3] = (float)(-(uint)(fVar6 == in_stack_00000060) & 0xbe23d70b) + fVar6 + 0.2;
      local_178[0] = in_XMM2_Da + 0.1;
      local_178[2] = fVar9 + 0.1;
      local_178[4] = fVar9 + -0.1;
      local_178[5] = local_178[3];
      local_178[6] = in_XMM3_Da + -0.1;
      local_15c = local_178[1];
      local_154 = local_178[1];
      local_14c = local_178[1];
      local_140 = 3;
      bVar1 = *in_stack_00000040;
      local_188 = in_stack_00000040 + 4;
      local_180 = *in_stack_00000048;
      local_190 = (undefined1 *)in_stack_00000048[1];
      local_f8 = in_stack_00000030;
      fStack_f4 = in_stack_00000030;
      uStack_f0 = 0;
      lVar3 = 3;
      pfVar2 = local_178 + 3;
      lVar4 = 0;
      do {
        fVar6 = pfVar2[-3];
        if ((NAN(fVar6 / in_stack_00000030)) || (NAN(pfVar2[-1] / in_stack_00000030))) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar9 = *pfVar2;
        fVar5 = fmodf(pfVar2[-1] / in_stack_00000030,1.0);
        fVar6 = fmodf(fVar6 / in_stack_00000030,1.0);
        fVar6 = (float)((uint)(fVar6 + 1.0) & -(uint)(fVar6 < 0.0) |
                       ~-(uint)(fVar6 < 0.0) & (uint)fVar6) * local_f8;
        fVar5 = (float)((uint)(fVar5 + 1.0) & -(uint)(fVar5 < 0.0) |
                       ~-(uint)(fVar5 < 0.0) & (uint)fVar5) * fStack_f4;
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        ::utils::wall_space::WallSpaceCoord::to_world_space();
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        ::utils::wall_space::WallSpaceCoord::to_world_space();
        fVar6 = (fVar5 * 0.5 + fVar6 * 0.5) / in_stack_00000030;
        local_1b8 = CONCAT44(local_1b8._4_4_,fVar6);
        if ((fVar6 < 0.0) || (1.0 < fVar6)) {
          local_138 = &local_1b8;
          local_130 = core::fmt::float::impl$5::fmt;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(fVar6)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u();
        if (((bVar1 & 1) != 0) ||
           (::utils::geometry::circle::Circle2d::get_pos_at_u(),
           fVar5 = extraout_XMM0_Da_00 - (local_20c * 0.5 + local_218 * 0.5),
           fVar6 = fVar6 - (local_204 * 0.5 + local_210 * 0.5),
           SQRT(fVar6 * fVar6 + fVar5 * fVar5) <= 0.078)) {
          ::utils::swizzlers::impl$0::x0y();
          fVar5 = local_204 - local_210;
          fVar7 = local_20c - local_218;
          fVar8 = local_208 - local_214;
          fVar6 = 1.0 / SQRT(fVar8 * fVar8 + fVar7 * fVar7 + fVar5 * fVar5);
          fVar10 = fStack_1f0 * 0.15;
          local_138 = (undefined8 *)CONCAT44(fVar6 * fVar8,fVar6 * fVar7);
          local_130 = (code *)CONCAT44(local_130._4_4_,fVar6 * fVar5);
          ::utils::rotation_from_normal();
          local_1b8 = CONCAT44(fVar9 * 0.15 + fVar9 * 0.15 + (local_208 + local_214) * 0.5,
                               extraout_XMM0_Da * 0.15 + extraout_XMM0_Da * 0.15 +
                               (local_218 + local_20c) * 0.5);
          uStack_1ac = 0x3e19999a;
          uStack_1a8 = 0x3e19999a;
          fVar6 = (fStack_1f0 + fStack_1f0) * extraout_XMM0_Da - (fVar9 + fVar9) * fStack_1ec;
          _fStack_1f0 = CONCAT44(fVar6 * 0.15,fVar6 * 0.15);
          fStack_1b0 = fVar10 + fVar10 + (local_204 + local_210) * 0.5;
          fStack_1a4 = SQRT((local_210 - local_204) * (local_210 - local_204) +
                            (local_214 - local_208) * (local_214 - local_208) +
                            (local_218 - local_20c) * (local_218 - local_20c));
          country_core::resources::render::impl$1::from();
          minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction()
          ;
          minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction()
          ;
          local_104 = extraout_EAX + in_stack_00000058;
          local_108 = 0;
          local_fc = 0;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
          lVar3 = local_140;
          if (extraout_AL != '\0') {
            *local_190 = 1;
          }
        }
        lVar4 = lVar4 + 1;
        pfVar2 = pfVar2 + 4;
      } while (lVar4 != lVar3);
    }
  }
  return;
}

