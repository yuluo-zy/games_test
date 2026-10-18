// addr 0x1408cc680 sig void __cdecl legacy_raycast(void) size 227

void __cdecl country_core::systems::collision::world::RaycastWorld::legacy_raycast(void)

{
  undefined8 *in_RCX;
  undefined4 uStack_4c;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined4 local_28;
  undefined8 local_24;
  undefined4 local_1c;
  int local_14;
  undefined4 uStack_10;
  undefined4 local_c;
  
  Rapier::raycast();
  if (local_14 == 2) {
    *(undefined4 *)(in_RCX + 6) = 2;
  }
  else {
    *in_RCX = local_48;
    in_RCX[1] = uStack_40;
    *(undefined4 *)(in_RCX + 4) = local_28;
    *(undefined8 *)((longlong)in_RCX + 0x24) = local_24;
    *(undefined4 *)((longlong)in_RCX + 0x2c) = local_1c;
    in_RCX[2] = local_38;
    in_RCX[3] = local_30;
    in_RCX[6] = CONCAT44(uStack_10,local_14);
    in_RCX[7] = CONCAT44(uStack_4c,local_c);
  }
  return;
}


// CALLEES: [{"address": "1408cdc60", "name": "raycast"}]