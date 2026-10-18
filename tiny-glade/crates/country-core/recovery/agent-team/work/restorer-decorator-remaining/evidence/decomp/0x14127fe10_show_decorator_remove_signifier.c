// 0x14127fe10
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "format_inner", "address": "140d79f90"}, {"name": "handle_alloc_error", "address": "1428d9000"}, {"name": "new", "address": "14089d8b0"}, {"name": "write", "address": "140b2cf90"}, {"name": "xz_projective_rotation_bad", "address": "140c911d0"}]

void __cdecl system_decorator::ui_systems::move_ops::show_decorator_remove_signifier(void)

{
  undefined *puVar1;
  undefined8 uVar2;
  undefined8 *extraout_RAX;
  undefined8 *in_RDX;
  undefined8 *in_R8;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  longlong local_e0;
  undefined1 *local_c8;
  code *pcStack_c0;
  undefined1 **local_b8;
  code *pcStack_b0;
  undefined8 local_a8;
  undefined *local_98;
  undefined8 uStack_90;
  undefined1 **local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 local_60;
  float local_58;
  undefined1 *local_50;
  float local_48;
  undefined1 local_44 [4];
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  fVar3 = *(float *)(in_RDX + 1) - *(float *)(in_R8 + 1);
  local_58 = *(float *)((longlong)in_RDX + 0x14);
  local_60 = *(undefined8 *)((longlong)in_RDX + 0xc);
  fVar4 = (float)*in_RDX;
  fVar5 = (float)((ulonglong)*in_RDX >> 0x20);
  fVar6 = fVar4 - (float)*in_R8;
  fVar7 = fVar5 - (float)((ulonglong)*in_R8 >> 0x20);
  fVar3 = SQRT(fVar7 * fVar7 + fVar6 * fVar6 + fVar3 * fVar3);
  local_48 = fVar3 * local_58 + *(float *)(in_RDX + 1);
  local_50 = (undefined1 *)
             CONCAT44(fVar3 * (float)((ulonglong)local_60 >> 0x20) + fVar5,
                      fVar3 * (float)local_60 + fVar4);
  ::utils::xz_projective_rotation_bad();
  uVar2 = uStack_90;
  puVar1 = local_98;
  local_c8 = local_44;
  pcStack_c0 = country_core::resources::walls::decorator::impl$27::fmt;
  local_b8 = &local_50;
  pcStack_b0 = glam::f32::vec3::impl$93::fmt;
  local_98 = &DAT_142c247e0;
  uStack_90 = 2;
  local_88 = &local_c8;
  uStack_80 = 2;
  local_78 = 0;
  alloc::fmt::format::format_inner();
  uStack_90 = CONCAT44(uStack_90._4_4_,local_48);
  local_98 = local_50;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    extraout_RAX[2] = puVar1;
    extraout_RAX[3] = uVar2;
    extraout_RAX[4] = local_98;
    *(undefined4 *)(extraout_RAX + 5) = (undefined4)uStack_90;
    country_core::systems::ui::signifiers::SignifierFactory::new();
    local_a8 = local_78;
    local_b8 = local_88;
    pcStack_b0 = (code *)uStack_80;
    local_c8 = local_98;
    pcStack_c0 = (code *)uStack_90;
    country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
    if (local_e0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

