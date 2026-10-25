
void __cdecl
core::iter::adapters::map::impl$2::
next<country_core::resources::critters::sheep::control::SheepSnapshot,bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::sheep::Sheep>,ref_mut_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env_0>
          (void)

{
  uint uVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined8 *puVar7;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong lVar8;
  uint *puVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  
  lVar14 = *(longlong *)(in_RDX + 0x18);
  uVar11 = *(ulonglong *)(in_RDX + 0xb8);
  if (*(char *)(in_RDX + 0xc0) == '\0') {
    if (uVar11 == *(ulonglong *)(in_RDX + 0xb0)) {
      puVar9 = *(uint **)(in_RDX + 0x20);
      do {
        if (puVar9 == *(uint **)(in_RDX + 0x28)) goto LAB_1412f05f2;
        *(uint **)(in_RDX + 0x20) = puVar9 + 1;
        uVar1 = *puVar9;
        lVar12 = *(longlong *)(*(longlong *)(in_RDX + 0x10) + 8);
        puVar9 = puVar9 + 1;
      } while (*(longlong *)(lVar12 + 0x58 + (ulonglong)uVar1 * 0xa0) == 0);
      lVar12 = lVar12 + (ulonglong)uVar1 * 0xa0;
      lVar10 = *(longlong *)(*(longlong *)(in_RDX + 8) + 8);
      lVar6 = *(longlong *)(lVar10 + 0x18 + (ulonglong)*(uint *)(lVar12 + 0x94) * 0x48);
      lVar4 = *(longlong *)(lVar10 + 0x38 + (ulonglong)*(uint *)(lVar12 + 0x94) * 0x48);
      lVar13 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar14 + 0x110) * 8) * 0x30;
      lVar10 = *(longlong *)(lVar6 + 0x10 + lVar13);
      *(longlong *)(in_RDX + 0x50) = lVar10;
      puVar7 = (undefined8 *)(lVar6 + 0x20 + lVar13);
      uVar2 = puVar7[1];
      *(undefined8 *)(in_RDX + 0x58) = *puVar7;
      *(undefined8 *)(in_RDX + 0x60) = uVar2;
      lVar3 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar14 + 0x118) * 8) * 0x30;
      lVar13 = *(longlong *)(lVar6 + 0x10 + lVar3);
      *(longlong *)(in_RDX + 0x70) = lVar13;
      puVar7 = (undefined8 *)(lVar6 + 0x20 + lVar3);
      uVar2 = puVar7[1];
      *(undefined8 *)(in_RDX + 0x78) = *puVar7;
      *(undefined8 *)(in_RDX + 0x80) = uVar2;
      lVar4 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar14 + 0x120) * 8) * 0x30;
      lVar14 = *(longlong *)(lVar6 + 0x10 + lVar4);
      *(longlong *)(in_RDX + 0x90) = lVar14;
      puVar7 = (undefined8 *)(lVar6 + 0x20 + lVar4);
      uVar2 = puVar7[1];
      *(undefined8 *)(in_RDX + 0x98) = *puVar7;
      *(undefined8 *)(in_RDX + 0xa0) = uVar2;
      lVar6 = *(longlong *)(lVar12 + 0x50);
      uVar2 = *(undefined8 *)(lVar12 + 0x58);
      *(longlong *)(in_RDX + 0x40) = lVar6;
      *(undefined8 *)(in_RDX + 0x48) = uVar2;
      *(undefined8 *)(in_RDX + 0xb0) = uVar2;
      uVar11 = 0;
    }
    else {
      lVar6 = *(longlong *)(in_RDX + 0x40);
      lVar10 = *(longlong *)(in_RDX + 0x50);
      lVar13 = *(longlong *)(in_RDX + 0x70);
      lVar14 = *(longlong *)(in_RDX + 0x90);
    }
    uVar5 = (ulonglong)*(uint *)(lVar6 + 8 + uVar11 * 0x10);
  }
  else {
    if (uVar11 == *(ulonglong *)(in_RDX + 0xb0)) {
      puVar9 = *(uint **)(in_RDX + 0x20);
      do {
        if (puVar9 == *(uint **)(in_RDX + 0x28)) {
LAB_1412f05f2:
          *(undefined4 *)in_RCX = 2;
          return;
        }
        *(uint **)(in_RDX + 0x20) = puVar9 + 1;
        uVar1 = *puVar9;
        lVar6 = *(longlong *)(*(longlong *)(in_RDX + 8) + 8);
        lVar12 = *(longlong *)(lVar6 + 0x10 + (ulonglong)uVar1 * 0x48);
        puVar9 = puVar9 + 1;
      } while (lVar12 == 0);
      lVar6 = lVar6 + (ulonglong)uVar1 * 0x48;
      lVar4 = *(longlong *)(lVar6 + 0x18);
      lVar3 = *(longlong *)(lVar6 + 0x38);
      lVar13 = ~*(ulonglong *)(lVar3 + *(longlong *)(lVar14 + 0x110) * 8) * 0x30;
      lVar10 = *(longlong *)(lVar4 + 0x10 + lVar13);
      *(longlong *)(in_RDX + 0x50) = lVar10;
      puVar7 = (undefined8 *)(lVar4 + 0x20 + lVar13);
      uVar2 = puVar7[1];
      *(undefined8 *)(in_RDX + 0x58) = *puVar7;
      *(undefined8 *)(in_RDX + 0x60) = uVar2;
      lVar8 = ~*(ulonglong *)(lVar3 + *(longlong *)(lVar14 + 0x118) * 8) * 0x30;
      lVar13 = *(longlong *)(lVar4 + 0x10 + lVar8);
      *(longlong *)(in_RDX + 0x70) = lVar13;
      puVar7 = (undefined8 *)(lVar4 + 0x20 + lVar8);
      uVar2 = puVar7[1];
      *(undefined8 *)(in_RDX + 0x78) = *puVar7;
      *(undefined8 *)(in_RDX + 0x80) = uVar2;
      lVar3 = ~*(ulonglong *)(lVar3 + *(longlong *)(lVar14 + 0x120) * 8) * 0x30;
      lVar14 = *(longlong *)(lVar4 + 0x10 + lVar3);
      *(longlong *)(in_RDX + 0x90) = lVar14;
      puVar7 = (undefined8 *)(lVar4 + 0x20 + lVar3);
      uVar2 = puVar7[1];
      *(undefined8 *)(in_RDX + 0x98) = *puVar7;
      *(undefined8 *)(in_RDX + 0xa0) = uVar2;
      *(undefined8 *)(in_RDX + 0x30) = *(undefined8 *)(lVar6 + 8);
      *(longlong *)(in_RDX + 0x38) = lVar12;
      *(longlong *)(in_RDX + 0xb0) = lVar12;
      uVar11 = 0;
    }
    else {
      lVar10 = *(longlong *)(in_RDX + 0x50);
      lVar13 = *(longlong *)(in_RDX + 0x70);
      lVar14 = *(longlong *)(in_RDX + 0x90);
    }
    uVar5 = uVar11 & 0xffffffff;
  }
  lVar10 = uVar5 * 0x30 + lVar10;
  lVar13 = uVar5 * 0x68 + lVar13;
  puVar7 = (undefined8 *)(uVar5 * 0x10 + lVar14);
  *(ulonglong *)(in_RDX + 0xb8) = uVar11 + 1;
  *(undefined4 *)((longlong)in_RCX + 0x24) = *(undefined4 *)(lVar10 + 0x18);
  *(undefined8 *)((longlong)in_RCX + 0x1c) = *(undefined8 *)(lVar10 + 0x10);
  uVar2 = puVar7[1];
  *in_RCX = *puVar7;
  in_RCX[1] = uVar2;
  in_RCX[2] = *(undefined8 *)(lVar13 + 0x30);
  *(undefined4 *)(in_RCX + 3) = *(undefined4 *)(lVar13 + 0x38);
  return;
}

