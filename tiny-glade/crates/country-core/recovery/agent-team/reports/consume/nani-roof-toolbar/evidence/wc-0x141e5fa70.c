
void __cdecl
bevy_ecs::bundle::BundleInfo::
write_components<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>_>
          (void)

{
  char cVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  code *pcVar4;
  ulonglong *in_RCX;
  ulonglong uVar5;
  longlong in_R8;
  longlong lVar6;
  char *in_stack_00000028;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  uint in_stack_00000050;
  undefined4 in_stack_00000058;
  undefined8 *in_stack_00000060;
  char in_stack_00000068;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_70;
  undefined8 local_68;
  ulonglong local_60;
  ulonglong *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_68 = in_stack_00000060[2];
  local_70 = in_stack_00000060[3];
  local_98 = *in_stack_00000060;
  uStack_90 = in_stack_00000060[1];
  uVar2 = *in_RCX;
  uVar5 = *(ulonglong *)(in_R8 + 0x40);
  if ((uVar2 < uVar5) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar2 * 8), uVar3 != 0)) {
    lVar6 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar6 = 0;
  }
  local_58 = in_RCX;
  if (*in_stack_00000028 == '\0') {
    local_60 = uVar5;
    memmove((void *)(*(size_t *)(lVar6 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar6 + 0x10)),&local_98,*(size_t *)(lVar6 + 8));
    *(undefined4 *)(*(longlong *)(lVar6 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
    in_RCX = local_58;
LAB_141e5fbd2:
    *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
    uVar5 = local_60;
  }
  else {
    if (in_stack_00000068 == '\0') {
      local_49 = 0;
      local_60 = uVar5;
      storage::blob_array::BlobArray::replace_unchecked();
      in_RCX = local_58;
      goto LAB_141e5fbd2;
    }
    if (((uVar2 < uVar5) &&
        (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar2 * 8), uVar2 != 0)) &&
       (pcVar4 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar2 * 0x30),
       pcVar4 != (code *)0x0)) {
      local_49 = 0;
      (*pcVar4)(&local_98);
    }
  }
  local_98 = local_68;
  uStack_90 = local_70;
  local_88 = *(undefined4 *)(in_stack_00000060 + 4);
  uStack_84 = *(undefined4 *)((longlong)in_stack_00000060 + 0x24);
  uStack_80 = *(undefined4 *)(in_stack_00000060 + 5);
  uStack_7c = *(undefined4 *)((longlong)in_stack_00000060 + 0x2c);
  uVar2 = in_RCX[1];
  if (uVar2 < uVar5) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar2 * 8);
    if (uVar3 == 0) {
      cVar1 = in_stack_00000028[1];
      goto joined_r0x000141e5fc5b;
    }
    lVar6 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[1] == '\0') goto LAB_141e5fc2a;
LAB_141e5fc5d:
    if (in_stack_00000068 != '\0') {
      if (((uVar2 < uVar5) &&
          (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar2 * 8), uVar2 != 0)) &&
         (pcVar4 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar2 * 0x30),
         pcVar4 != (code *)0x0)) {
        local_49 = 1;
        (*pcVar4)(&local_98);
      }
      goto joined_r0x000141e5fcc8;
    }
    local_49 = 1;
    storage::blob_array::BlobArray::replace_unchecked();
  }
  else {
    cVar1 = in_stack_00000028[1];
joined_r0x000141e5fc5b:
    lVar6 = 0;
    if (cVar1 != '\0') goto LAB_141e5fc5d;
LAB_141e5fc2a:
    memmove((void *)(*(size_t *)(lVar6 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar6 + 0x10)),&local_98,*(size_t *)(lVar6 + 8));
    *(undefined4 *)(*(longlong *)(lVar6 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
       in_stack_00000058;
joined_r0x000141e5fcc8:
  for (; in_stack_00000038 != in_stack_00000040; in_stack_00000038 = in_stack_00000038 + 0x10) {
    component::RequiredComponentConstructor::initialize();
  }
  return;
}

