
void __cdecl
bevy_ecs::bundle::BundleInfo::
write_components<country_core::components::drawable::DrawableMeshBundle,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>_>
          (void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  code *pcVar3;
  ulonglong *in_RCX;
  longlong in_R8;
  ulonglong uVar4;
  longlong lVar5;
  char *in_stack_00000028;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  uint in_stack_00000050;
  undefined4 in_stack_00000058;
  undefined8 *in_stack_00000060;
  char in_stack_00000068;
  undefined8 local_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  
  local_78 = in_stack_00000060[6];
  uVar4 = *in_RCX;
  uVar1 = *(ulonglong *)(in_R8 + 0x40);
  if ((uVar4 < uVar1) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar4 * 8), uVar2 != 0)) {
    lVar5 = ~uVar2 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar5 = 0;
  }
  if (*in_stack_00000028 == '\0') {
    memmove((void *)(*(size_t *)(lVar5 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar5 + 0x10)),&local_78,*(size_t *)(lVar5 + 8));
    *(undefined4 *)(*(longlong *)(lVar5 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141fe7b8f:
    *(undefined4 *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  else {
    if (in_stack_00000068 == '\0') {
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141fe7b8f;
    }
    if (((uVar4 < uVar1) &&
        (uVar4 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar4 * 8), uVar4 != 0)) &&
       (pcVar3 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar4 * 0x30),
       pcVar3 != (code *)0x0)) {
      (*pcVar3)(&local_78);
    }
  }
  local_78 = in_stack_00000060[7];
  uVar4 = in_RCX[1];
  if ((uVar4 < uVar1) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar4 * 8), uVar2 != 0)) {
    lVar5 = ~uVar2 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[1] == '\0') goto LAB_141fe7be6;
LAB_141fe7c13:
    if (in_stack_00000068 != '\0') {
      if (((uVar4 < uVar1) &&
          (uVar4 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar4 * 8), uVar4 != 0)) &&
         (pcVar3 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar4 * 0x30),
         pcVar3 != (code *)0x0)) {
        (*pcVar3)(&local_78);
      }
      goto LAB_141fe7c67;
    }
    storage::blob_array::BlobArray::replace_unchecked();
  }
  else {
    lVar5 = 0;
    if (in_stack_00000028[1] != '\0') goto LAB_141fe7c13;
LAB_141fe7be6:
    memmove((void *)(*(size_t *)(lVar5 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar5 + 0x10)),&local_78,*(size_t *)(lVar5 + 8));
    *(undefined4 *)(*(longlong *)(lVar5 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  *(undefined4 *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
       in_stack_00000058;
LAB_141fe7c67:
  local_78 = *in_stack_00000060;
  uStack_70 = *(undefined4 *)(in_stack_00000060 + 1);
  uStack_6c = *(undefined4 *)((longlong)in_stack_00000060 + 0xc);
  local_68 = *(undefined4 *)(in_stack_00000060 + 2);
  uStack_64 = *(undefined4 *)((longlong)in_stack_00000060 + 0x14);
  uStack_60 = *(undefined4 *)(in_stack_00000060 + 3);
  uStack_5c = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
  local_58 = *(undefined4 *)(in_stack_00000060 + 4);
  uStack_54 = *(undefined4 *)((longlong)in_stack_00000060 + 0x24);
  uStack_50 = *(undefined4 *)(in_stack_00000060 + 5);
  uStack_4c = *(undefined4 *)((longlong)in_stack_00000060 + 0x2c);
  uVar4 = in_RCX[2];
  if ((uVar4 < uVar1) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar4 * 8), uVar2 != 0)) {
    lVar5 = ~uVar2 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar5 = 0;
  }
  if (in_stack_00000028[2] == '\0') {
    uVar4 = (ulonglong)in_stack_00000050;
    memmove((void *)(*(size_t *)(lVar5 + 8) * uVar4 + *(longlong *)(lVar5 + 0x10)),&local_78,
            *(size_t *)(lVar5 + 8));
    *(undefined4 *)(*(longlong *)(lVar5 + 0x20) + uVar4 * 4) = in_stack_00000058;
    *(undefined4 *)(*(longlong *)(lVar5 + 0x28) + uVar4 * 4) = in_stack_00000058;
  }
  else if (in_stack_00000068 == '\0') {
    storage::blob_array::BlobArray::replace_unchecked();
    *(undefined4 *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  else if (((uVar4 < uVar1) &&
           (uVar4 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar4 * 8), uVar4 != 0)) &&
          (pcVar3 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar4 * 0x30),
          pcVar3 != (code *)0x0)) {
    (*pcVar3)(&local_78);
  }
  for (; in_stack_00000038 != in_stack_00000040; in_stack_00000038 = in_stack_00000038 + 0x10) {
    component::RequiredComponentConstructor::initialize();
  }
  return;
}

