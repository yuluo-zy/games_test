// ADDR 0x141fe5c20
// SIG void __cdecl intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>(void)
// ORIG system::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>


void __cdecl
system_decorator::collision_detection::
intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env_0>
          (void)

{
  undefined1 auVar1 [16];
  longlong lVar2;
  ushort uVar3;
  int extraout_EAX;
  int extraout_EAX_00;
  int iVar4;
  uint uVar5;
  ulonglong in_RDX;
  longlong in_R9;
  longlong *in_stack_00000038;
  undefined1 local_84 [4];
  undefined1 (*local_80) [16];
  undefined1 (*local_78) [16];
  undefined1 *local_70;
  ushort local_68;
  longlong local_60;
  undefined1 *local_58 [3];
  undefined1 *local_40;
  undefined1 local_38 [8];
  undefined1 **local_30;
  
  if ((in_RDX & 1) == 0) {
    local_80 = *(undefined1 (**) [16])(in_R9 + 0x30);
    local_70 = *local_80 + *(longlong *)(in_R9 + 0x38) + 1;
    auVar1 = *local_80;
    local_68 = ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
    local_78 = local_80 + 1;
    local_60 = *(longlong *)(in_R9 + 0x48);
    local_58[0] = local_38;
    local_40 = local_84;
    local_30 = local_58;
    while (local_60 != 0) {
      uVar5 = (uint)local_68;
      if (local_68 == 0) {
        do {
          auVar1 = *local_78;
          uVar3 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
          local_80 = local_80 + -0x50;
          local_78 = local_78 + 1;
        } while (uVar3 == 0xffff);
        uVar5 = ~(uint)uVar3;
        local_68 = (ushort)uVar5 & -uVar3 - 2;
      }
      else {
        local_68 = local_68 - 1 & local_68;
        if (local_80 == (undefined1 (*) [16])0x0) {
          return;
        }
      }
      local_60 = local_60 + -1;
      iVar4 = 0;
      for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        iVar4 = iVar4 + 1;
      }
      iVar4 = (int)local_80 + iVar4 * -0x50 + -0x50;
      core::ops::function::impls::impl$3::
      call_mut<tuple_<tuple_<ref_<country_core::resources::walls::decorator::DecoratorId>,ref_<country_core::resources::walls::decorator::DecoratorInfo>_>_>,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env_0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env_0>_>
                ();
      if (extraout_EAX_00 != 0) {
        lVar2 = in_stack_00000038[2];
        if (lVar2 == *in_stack_00000038) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        *(int *)(in_stack_00000038[1] + lVar2 * 4) = iVar4;
        in_stack_00000038[2] = lVar2 + 1;
        local_30 = local_58;
      }
    }
  }
  else {
    local_80 = *(undefined1 (**) [16])(in_R9 + 0x30);
    local_70 = *local_80 + *(longlong *)(in_R9 + 0x38) + 1;
    auVar1 = *local_80;
    local_68 = ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
    local_78 = local_80 + 1;
    local_60 = *(longlong *)(in_R9 + 0x48);
    local_58[0] = local_38;
    local_40 = local_84;
    local_30 = local_58;
    while (local_60 != 0) {
      uVar5 = (uint)local_68;
      if (local_68 == 0) {
        do {
          auVar1 = *local_78;
          uVar3 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
          local_80 = local_80 + -0x50;
          local_78 = local_78 + 1;
        } while (uVar3 == 0xffff);
        uVar5 = ~(uint)uVar3;
        local_68 = (ushort)uVar5 & -uVar3 - 2;
      }
      else {
        local_68 = local_68 - 1 & local_68;
        if (local_80 == (undefined1 (*) [16])0x0) {
          return;
        }
      }
      local_60 = local_60 + -1;
      iVar4 = 0;
      for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        iVar4 = iVar4 + 1;
      }
      iVar4 = (int)local_80 + iVar4 * -0x50 + -0x50;
      core::ops::function::impls::impl$3::
      call_mut<tuple_<tuple_<ref_<country_core::resources::walls::decorator::DecoratorId>,ref_<country_core::resources::walls::decorator::DecoratorInfo>_>_>,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env_0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env_0>_>
                ();
      if (extraout_EAX != 0) {
        lVar2 = in_stack_00000038[2];
        if (lVar2 == *in_stack_00000038) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        *(int *)(in_stack_00000038[1] + lVar2 * 4) = iVar4;
        in_stack_00000038[2] = lVar2 + 1;
        local_30 = local_58;
      }
    }
  }
  return;
}


