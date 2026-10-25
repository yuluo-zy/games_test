
void __cdecl country_core::systems::collision::world::RaycastWorld::get_collider(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  
  rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
  if (extraout_RAX != (undefined8 *)0x0) {
    uVar2 = *extraout_RAX;
    uVar1 = *(undefined4 *)((longlong)extraout_RAX + 0x4c);
    uVar3 = *(undefined8 *)((longlong)extraout_RAX + 0x3c);
    *in_RCX = *(undefined8 *)((longlong)extraout_RAX + 0x34);
    in_RCX[1] = uVar3;
    in_RCX[2] = *(undefined8 *)((longlong)extraout_RAX + 0x44);
    *(undefined4 *)(in_RCX + 3) = uVar1;
    *(uint *)(in_RCX + 4) = (uint)((ulonglong)uVar2 >> 0x20) | 1;
    *(int *)((longlong)in_RCX + 0x24) = (int)uVar2;
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::option::expect_failed();
}

