// ADDR 0x1408252b0
// SIG void __cdecl set_colliders(void)
// ORIG system::set_colliders


void __cdecl system_decorator::blueprint::DecoratorBlueprint::set_colliders(void)

{
  uint uVar1;
  longlong *plVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined1 auVar5 [16];
  undefined4 extraout_EAX;
  int iVar6;
  longlong in_RCX;
  undefined8 in_RDX;
  undefined8 in_R8;
  uint *in_R9;
  undefined8 *in_stack_00000028;
  undefined8 local_d8;
  code *pcStack_d0;
  code *local_c0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined *local_88;
  undefined8 uStack_80;
  undefined8 *local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_58;
  longlong local_50;
  undefined1 local_42;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if ((((*in_R9 & 0x7fffffff) < 0x7f800000) && ((in_R9[1] & 0x7fffffff) < 0x7f800000)) &&
     ((in_R9[2] & 0x7fffffff) < 0x7f800000)) {
    uVar3 = *in_stack_00000028;
    uVar4 = in_stack_00000028[1];
    local_41 = 1;
    local_42 = 1;
    local_88 = (undefined *)uVar3;
    uStack_80 = uVar4;
    local_58 = in_R8;
    glam::f32::sse2::vec4::Vec4::is_finite_mask();
    auVar5._8_8_ = pcStack_d0;
    auVar5._0_8_ = local_d8;
    iVar6 = movmskps(extraout_EAX,auVar5);
    local_a8 = uVar3;
    uStack_a0 = uVar4;
    if (iVar6 == 0xf) {
      uVar1 = in_R9[2];
      local_88 = *(undefined **)in_R9;
      local_98 = in_RDX;
      local_50 = in_RCX;
      uStack_80._0_4_ = uVar1;
      if (*(int *)(in_RCX + 0x50) == 0) {
        plVar2 = *(longlong **)(in_RCX + 0x60);
        LOCK();
        *plVar2 = *plVar2 + -1;
        UNLOCK();
        if (*plVar2 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      *(undefined4 *)(local_50 + 0x50) = 0;
      *(undefined **)(local_50 + 0x54) = local_88;
      *(uint *)(local_50 + 0x5c) = (uint)uStack_80;
      *(undefined8 *)(local_50 + 0x60) = local_98;
      *(undefined8 *)(local_50 + 0x70) = local_a8;
      *(undefined8 *)(local_50 + 0x78) = uStack_a0;
      local_a8 = local_58;
      uStack_80 = CONCAT44(uStack_80._4_4_,in_R9[2]);
      local_88 = *(undefined **)in_R9;
      local_98 = *in_stack_00000028;
      uStack_90 = in_stack_00000028[1];
      if (*(int *)(local_50 + 0x80) == 0) {
        plVar2 = *(longlong **)(local_50 + 0x90);
        LOCK();
        *plVar2 = *plVar2 + -1;
        UNLOCK();
        if (*plVar2 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      *(undefined4 *)(local_50 + 0x80) = 0;
      *(undefined **)(local_50 + 0x84) = local_88;
      *(uint *)(local_50 + 0x8c) = (uint)uStack_80;
      *(undefined8 *)(local_50 + 0x90) = local_a8;
      *(undefined8 *)(local_50 + 0xa0) = local_98;
      *(undefined8 *)(local_50 + 0xa8) = uStack_90;
      return;
    }
  }
  pcStack_d0 = glam::f32::vec3::impl$93::fmt;
  local_c0 = glam::f32::sse2::quat::impl$1::fmt;
  local_88 = &DAT_142a91630;
  uStack_80 = 2;
  local_68 = 0;
  local_78 = &local_d8;
  local_70 = 2;
  local_41 = 1;
  local_42 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


