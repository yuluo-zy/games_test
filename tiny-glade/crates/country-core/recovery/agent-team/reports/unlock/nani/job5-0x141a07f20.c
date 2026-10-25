
void __cdecl
bevy_ecs::bundle::BundleInfo::
write_components<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>_>
          (void)

{
  char cVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  code *pcVar4;
  ulonglong *in_RCX;
  longlong in_R8;
  ulonglong uVar5;
  longlong lVar6;
  char *in_stack_00000028;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  uint in_stack_00000050;
  undefined4 in_stack_00000058;
  undefined4 *in_stack_00000060;
  char in_stack_00000068;
  undefined4 local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 local_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  
  local_68 = *(undefined8 *)(in_stack_00000060 + 0x10);
  local_78 = in_stack_00000060[0xc];
  uStack_74 = in_stack_00000060[0xd];
  uStack_70 = in_stack_00000060[0xe];
  uStack_6c = in_stack_00000060[0xf];
  uVar5 = *in_RCX;
  uVar2 = *(ulonglong *)(in_R8 + 0x40);
  if ((uVar5 < uVar2) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar5 * 8), uVar3 != 0)) {
    lVar6 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar6 = 0;
  }
  if (*in_stack_00000028 == '\0') {
    memmove((void *)(*(size_t *)(lVar6 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar6 + 0x10)),&local_78,*(size_t *)(lVar6 + 8));
    *(undefined4 *)(*(longlong *)(lVar6 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141a08067:
    *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  else {
    if (in_stack_00000068 == '\0') {
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141a08067;
    }
    if (((uVar5 < uVar2) &&
        (uVar5 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar5 * 8), uVar5 != 0)) &&
       (pcVar4 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar5 * 0x30),
       pcVar4 != (code *)0x0)) {
      (*pcVar4)(&local_78);
    }
  }
  local_78 = *in_stack_00000060;
  uStack_74 = in_stack_00000060[1];
  uStack_70 = in_stack_00000060[2];
  uStack_6c = in_stack_00000060[3];
  local_68 = *(undefined8 *)(in_stack_00000060 + 4);
  uStack_60 = in_stack_00000060[6];
  uStack_5c = in_stack_00000060[7];
  local_58 = in_stack_00000060[8];
  uStack_54 = in_stack_00000060[9];
  uStack_50 = in_stack_00000060[10];
  uStack_4c = in_stack_00000060[0xb];
  uVar5 = in_RCX[1];
  if ((uVar5 < uVar2) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar5 * 8), uVar3 != 0)) {
    lVar6 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar6 = 0;
  }
  if (in_stack_00000028[1] == '\0') {
    uVar5 = (ulonglong)in_stack_00000050;
    memmove((void *)(*(size_t *)(lVar6 + 8) * uVar5 + *(longlong *)(lVar6 + 0x10)),&local_78,
            *(size_t *)(lVar6 + 8));
    *(undefined4 *)(*(longlong *)(lVar6 + 0x20) + uVar5 * 4) = in_stack_00000058;
    *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + uVar5 * 4) = in_stack_00000058;
    uVar5 = in_RCX[2];
    cVar1 = in_stack_00000028[2];
  }
  else {
    if (in_stack_00000068 == '\0') {
      storage::blob_array::BlobArray::replace_unchecked();
      *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
           in_stack_00000058;
    }
    else if (((uVar5 < uVar2) &&
             (uVar5 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar5 * 8), uVar5 != 0)) &&
            (pcVar4 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar5 * 0x30),
            pcVar4 != (code *)0x0)) {
      (*pcVar4)(&local_78);
      uVar5 = in_RCX[2];
      cVar1 = in_stack_00000028[2];
      goto joined_r0x000141a081b9;
    }
    uVar5 = in_RCX[2];
    cVar1 = in_stack_00000028[2];
  }
joined_r0x000141a081b9:
  if ((uVar5 < uVar2) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar5 * 8), uVar3 != 0)) {
    lVar6 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar6 = 0;
  }
  if (cVar1 == '\0') {
    uVar5 = (ulonglong)in_stack_00000050;
    memmove((void *)(*(size_t *)(lVar6 + 8) * uVar5 + *(longlong *)(lVar6 + 0x10)),&local_78,
            *(size_t *)(lVar6 + 8));
    *(undefined4 *)(*(longlong *)(lVar6 + 0x20) + uVar5 * 4) = in_stack_00000058;
    *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + uVar5 * 4) = in_stack_00000058;
  }
  else if (in_stack_00000068 == '\0') {
    storage::blob_array::BlobArray::replace_unchecked();
    *(undefined4 *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  else if (((uVar5 < uVar2) &&
           (uVar5 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar5 * 8), uVar5 != 0)) &&
          (pcVar4 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar5 * 0x30),
          pcVar4 != (code *)0x0)) {
    (*pcVar4)(&local_78);
  }
  for (; in_stack_00000038 != in_stack_00000040; in_stack_00000038 = in_stack_00000038 + 0x10) {
    component::RequiredComponentConstructor::initialize();
  }
  return;
}

