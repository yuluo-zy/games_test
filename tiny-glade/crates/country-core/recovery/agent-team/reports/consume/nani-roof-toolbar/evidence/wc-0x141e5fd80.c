
void __cdecl
bevy_ecs::bundle::BundleInfo::
write_components<tuple_<toolbar::resouces_and_components::TaffyNode,toolbar::resouces_and_components::UiScaleWithUserSetting,country_core::components::UiInteractionState,toolbar::resouces_and_components::UiAnimationState,country_core::components::UiMouseInteractable,country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::components::UiDrawOrder>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>_>
          (void)

{
  undefined4 uVar1;
  char cVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  code *pcVar7;
  undefined3 uVar8;
  ulonglong uVar9;
  ulonglong *in_RCX;
  longlong in_R8;
  longlong lVar10;
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
  ulonglong local_70;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_88 = *(undefined4 *)(in_stack_00000060 + 10);
  local_98 = in_stack_00000060[8];
  uStack_90 = in_stack_00000060[9];
  uVar9 = *in_RCX;
  local_70 = *(ulonglong *)(in_R8 + 0x40);
  if ((uVar9 < local_70) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar3 != 0)) {
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
  }
  else {
    lVar10 = 0;
  }
  uVar8 = *(undefined3 *)(in_stack_00000060 + 0xb);
  uVar1 = *(undefined4 *)((longlong)in_stack_00000060 + 0x54);
  uVar4 = *in_stack_00000060;
  uVar5 = in_stack_00000060[1];
  uVar6 = in_stack_00000060[0xc];
  if (*in_stack_00000028 == '\0') {
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e5fec8:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
    uVar9 = in_RCX[1];
    if (local_70 <= uVar9) goto LAB_141e5ff3c;
LAB_141e5fee4:
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) goto LAB_141e5ff3c;
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[1] != '\0') goto LAB_141e5ff4c;
LAB_141e5ff0d:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e5ffa7:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  else {
    if (in_stack_00000068 == '\0') {
      local_61 = 0;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e5fec8;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 0;
      (*pcVar7)(&local_98);
    }
    uVar9 = in_RCX[1];
    if (uVar9 < local_70) goto LAB_141e5fee4;
LAB_141e5ff3c:
    lVar10 = 0;
    if (in_stack_00000028[1] == '\0') goto LAB_141e5ff0d;
LAB_141e5ff4c:
    if (in_stack_00000068 == '\0') {
      local_61 = 0;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e5ffa7;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 0;
      (*pcVar7)(&local_98);
    }
  }
  local_98 = CONCAT53(local_98._3_5_,uVar8);
  uVar9 = in_RCX[2];
  if (uVar9 < local_70) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) {
      cVar2 = in_stack_00000028[2];
      goto joined_r0x000141e60048;
    }
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[2] == '\0') goto LAB_141e60009;
LAB_141e6004a:
    if (in_stack_00000068 == '\0') {
      local_61 = 0;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e600a1;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 0;
      (*pcVar7)(&local_98);
    }
  }
  else {
    cVar2 = in_stack_00000028[2];
joined_r0x000141e60048:
    lVar10 = 0;
    if (cVar2 != '\0') goto LAB_141e6004a;
LAB_141e60009:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e600a1:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  local_98._0_4_ = *(undefined4 *)(in_stack_00000060 + 4);
  local_98._4_4_ = *(undefined4 *)((longlong)in_stack_00000060 + 0x24);
  uStack_90 = in_stack_00000060[5];
  uVar9 = in_RCX[3];
  if (uVar9 < local_70) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) {
      cVar2 = in_stack_00000028[3];
      goto joined_r0x000141e6013a;
    }
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[3] == '\0') goto LAB_141e600fb;
LAB_141e6013c:
    if (in_stack_00000068 == '\0') {
      local_61 = 0;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e60193;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 0;
      (*pcVar7)(&local_98);
    }
  }
  else {
    cVar2 = in_stack_00000028[3];
joined_r0x000141e6013a:
    lVar10 = 0;
    if (cVar2 != '\0') goto LAB_141e6013c;
LAB_141e600fb:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e60193:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  local_98 = CONCAT44(local_98._4_4_,uVar1);
  uVar9 = in_RCX[4];
  if (uVar9 < local_70) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) {
      cVar2 = in_stack_00000028[4];
      goto joined_r0x000141e60226;
    }
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[4] == '\0') goto LAB_141e601e7;
LAB_141e60228:
    if (in_stack_00000068 == '\0') {
      local_61 = 0;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e6027f;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 0;
      (*pcVar7)(&local_98);
    }
  }
  else {
    cVar2 = in_stack_00000028[4];
joined_r0x000141e60226:
    lVar10 = 0;
    if (cVar2 != '\0') goto LAB_141e60228;
LAB_141e601e7:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e6027f:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  local_98 = in_stack_00000060[6];
  uStack_90 = in_stack_00000060[7];
  uVar9 = in_RCX[5];
  if (uVar9 < local_70) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) {
      cVar2 = in_stack_00000028[5];
      goto joined_r0x000141e60318;
    }
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[5] == '\0') goto LAB_141e602d9;
LAB_141e6031a:
    if (in_stack_00000068 == '\0') {
      local_61 = 0;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e60371;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 0;
      (*pcVar7)(&local_98);
    }
  }
  else {
    cVar2 = in_stack_00000028[5];
joined_r0x000141e60318:
    lVar10 = 0;
    if (cVar2 != '\0') goto LAB_141e6031a;
LAB_141e602d9:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e60371:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  local_88 = *(undefined4 *)(in_stack_00000060 + 2);
  uStack_84 = *(undefined4 *)((longlong)in_stack_00000060 + 0x14);
  uStack_80 = *(undefined4 *)(in_stack_00000060 + 3);
  uStack_7c = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
  uVar9 = in_RCX[6];
  local_98 = uVar4;
  uStack_90 = uVar5;
  if (uVar9 < local_70) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) {
      cVar2 = in_stack_00000028[6];
      goto joined_r0x000141e6041b;
    }
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[6] == '\0') goto LAB_141e603dc;
LAB_141e6041d:
    if (in_stack_00000068 == '\0') {
      local_61 = 1;
      storage::blob_array::BlobArray::replace_unchecked();
      goto LAB_141e60474;
    }
    if (((uVar9 < local_70) &&
        (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
       (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
       pcVar7 != (code *)0x0)) {
      local_61 = 1;
      (*pcVar7)(&local_98);
    }
  }
  else {
    cVar2 = in_stack_00000028[6];
joined_r0x000141e6041b:
    lVar10 = 0;
    if (cVar2 != '\0') goto LAB_141e6041d;
LAB_141e603dc:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
LAB_141e60474:
    *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  uVar9 = in_RCX[7];
  local_98 = uVar6;
  if (uVar9 < local_70) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8);
    if (uVar3 == 0) {
      cVar2 = in_stack_00000028[7];
      goto joined_r0x000141e60520;
    }
    lVar10 = ~uVar3 * 0x30 + *(longlong *)(in_R8 + 0x18);
    if (in_stack_00000028[7] == '\0') goto LAB_141e604d9;
LAB_141e60522:
    if (in_stack_00000068 != '\0') {
      if (((uVar9 < local_70) &&
          (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x38) + uVar9 * 8), uVar9 != 0)) &&
         (pcVar7 = *(code **)(*(longlong *)(in_R8 + 0x18) + 0x18 + ~uVar9 * 0x30),
         pcVar7 != (code *)0x0)) {
        local_61 = 1;
        (*pcVar7)(&local_98);
      }
      goto joined_r0x000141e605a2;
    }
    local_61 = 1;
    storage::blob_array::BlobArray::replace_unchecked();
  }
  else {
    cVar2 = in_stack_00000028[7];
joined_r0x000141e60520:
    lVar10 = 0;
    if (cVar2 != '\0') goto LAB_141e60522;
LAB_141e604d9:
    memmove((void *)(*(size_t *)(lVar10 + 8) * (ulonglong)in_stack_00000050 +
                    *(longlong *)(lVar10 + 0x10)),&local_98,*(size_t *)(lVar10 + 8));
    *(undefined4 *)(*(longlong *)(lVar10 + 0x20) + (ulonglong)in_stack_00000050 * 4) =
         in_stack_00000058;
  }
  *(undefined4 *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)in_stack_00000050 * 4) =
       in_stack_00000058;
joined_r0x000141e605a2:
  for (; in_stack_00000038 != in_stack_00000040; in_stack_00000038 = in_stack_00000038 + 0x10) {
    component::RequiredComponentConstructor::initialize();
  }
  return;
}

