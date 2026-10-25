
void __cdecl country_core::systems::collision::world::impl$21::to_offset_collider_builder(void)

{
  longlong lVar1;
  longlong *plVar2;
  code *pcVar3;
  undefined8 *in_RCX;
  longlong in_R8;
  undefined8 local_d8;
  undefined4 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 local_44;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined2 local_34;
  undefined8 local_31;
  undefined1 local_29;
  
  ::utils::IsoTransform::from_translation_rotation();
  ::utils::IsoTransform::nested_in();
  plVar2 = *(longlong **)(in_R8 + 0x20);
  LOCK();
  lVar1 = *plVar2;
  *plVar2 = *plVar2 + 1;
  UNLOCK();
  if (*plVar2 != 0 && SCARRY8(lVar1,1) == *plVar2 < 0) {
    rapier3d::geometry::collider::ColliderBuilder::new();
    in_RCX[0xc] = local_68;
    in_RCX[10] = local_78;
    in_RCX[0xb] = uStack_70;
    in_RCX[8] = local_88;
    in_RCX[9] = uStack_80;
    in_RCX[6] = local_98;
    in_RCX[7] = uStack_90;
    in_RCX[4] = local_a8;
    in_RCX[5] = uStack_a0;
    *(undefined4 *)(in_RCX + 0x12) = uStack_38;
    *(undefined2 *)((longlong)in_RCX + 0x94) = local_34;
    *(undefined1 *)((longlong)in_RCX + 0x96) = 1;
    *(undefined8 *)((longlong)in_RCX + 0x97) = local_31;
    *(undefined1 *)((longlong)in_RCX + 0x9f) = local_29;
    *(undefined8 *)((longlong)in_RCX + 0x84) = local_44;
    *(ulonglong *)((longlong)in_RCX + 0x8c) = CONCAT44(uStack_38,uStack_3c);
    in_RCX[2] = local_d8;
    *(undefined4 *)(in_RCX + 3) = uStack_d0;
    in_RCX[0xd] = local_c8;
    in_RCX[0xe] = uStack_c0;
    *(undefined4 *)(in_RCX + 0x10) = uStack_d0;
    in_RCX[0xf] = local_d8;
    *in_RCX = local_c8;
    in_RCX[1] = uStack_c0;
    return;
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

