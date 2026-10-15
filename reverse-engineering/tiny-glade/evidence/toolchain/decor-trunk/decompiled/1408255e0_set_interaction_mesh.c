// ADDR 0x1408255e0
// SIG void __cdecl set_interaction_mesh(void)
// ORIG system::set_interaction_mesh


void __cdecl system_decorator::blueprint::DecoratorBlueprint::set_interaction_mesh(void)

{
  longlong *plVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined1 auVar4 [16];
  undefined4 extraout_EAX;
  int iVar5;
  longlong in_RCX;
  undefined8 in_RDX;
  uint *in_R8;
  undefined8 *in_R9;
  undefined8 local_b8;
  code *pcStack_b0;
  code *local_a0;
  undefined **local_98;
  undefined8 uStack_90;
  undefined8 *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_58;
  longlong local_50;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if ((((*in_R8 & 0x7fffffff) < 0x7f800000) && ((in_R8[1] & 0x7fffffff) < 0x7f800000)) &&
     ((in_R8[2] & 0x7fffffff) < 0x7f800000)) {
    uVar2 = *in_R9;
    uVar3 = in_R9[1];
    local_41 = 1;
    local_98 = (undefined **)uVar2;
    uStack_90 = uVar3;
    glam::f32::sse2::vec4::Vec4::is_finite_mask();
    auVar4._8_8_ = pcStack_b0;
    auVar4._0_8_ = local_b8;
    iVar5 = movmskps(extraout_EAX,auVar4);
    if (iVar5 == 0xf) {
      uStack_90 = CONCAT44(uStack_90._4_4_,in_R8[2]);
      local_98 = *(undefined ***)in_R8;
      local_58 = in_RDX;
      local_50 = in_RCX;
      if (*(int *)(in_RCX + 0x80) == 0) {
        plVar1 = *(longlong **)(in_RCX + 0x90);
        LOCK();
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (*plVar1 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      *(undefined4 *)(local_50 + 0x80) = 0;
      *(undefined ***)(local_50 + 0x84) = local_98;
      *(undefined4 *)(local_50 + 0x8c) = (undefined4)uStack_90;
      *(undefined8 *)(local_50 + 0x90) = local_58;
      *(undefined8 *)(local_50 + 0xa0) = uVar2;
      *(undefined8 *)(local_50 + 0xa8) = uVar3;
      return;
    }
  }
  pcStack_b0 = glam::f32::vec3::impl$93::fmt;
  local_a0 = glam::f32::sse2::quat::impl$1::fmt;
  local_98 = &anon_607a6278b2141009247d0776e3b766a7_30_llvm_14222842113206979732;
  uStack_90 = 2;
  local_78 = 0;
  local_88 = &local_b8;
  local_80 = 2;
  local_41 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


