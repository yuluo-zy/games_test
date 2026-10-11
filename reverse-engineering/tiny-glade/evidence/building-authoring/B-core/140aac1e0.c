
/* WARNING: Removing unreachable block (ram,0x000140aac4e4) */
/* WARNING: Removing unreachable block (ram,0x000140aac4f8) */
/* WARNING: Removing unreachable block (ram,0x000140aac50f) */
/* WARNING: Removing unreachable block (ram,0x000140aac580) */
/* WARNING: Removing unreachable block (ram,0x000140aac58c) */
/* WARNING: Removing unreachable block (ram,0x000140aac5b2) */
/* WARNING: Removing unreachable block (ram,0x000140aac5bb) */

void __cdecl country_core::systems::wall::delete_wall::delete_wall(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  ulonglong *puVar8;
  ulonglong *puVar9;
  ulonglong *puVar10;
  undefined4 *puVar11;
  ulonglong uVar12;
  undefined8 *in_RCX;
  longlong in_RDX;
  ulonglong *puVar13;
  ulonglong uVar14;
  ulonglong *puVar15;
  ulonglong *puVar16;
  longlong in_R8;
  longlong in_R9;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  longlong in_stack_00000028;
  ulonglong *local_158;
  ulonglong *puStack_150;
  longlong local_c0;
  longlong local_b8;
  undefined8 local_a8;
  ulonglong *puStack_a0;
  ulonglong *puStack_98;
  ulonglong *local_90;
  ulonglong *local_88;
  ulonglong *local_80;
  
  puVar4 = (ulonglong *)*in_RCX;
  lVar18 = in_RCX[1];
  uVar5 = *puVar4;
  uVar17 = 0;
  uVar12 = uVar5 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar12 = uVar17;
  }
  uVar14 = uVar5 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar14 = uVar17;
  }
  local_b8 = uVar12 * 0x10 + *(longlong *)(lVar18 + 8);
  uVar5 = *(ulonglong *)(lVar18 + 0x10) - uVar12;
  if (*(ulonglong *)(lVar18 + 0x10) < uVar12) {
    local_b8 = 8;
    uVar5 = uVar17;
  }
  local_c0 = uVar14 * 0x10 + *(longlong *)(lVar18 + 0x28);
  uVar12 = *(ulonglong *)(lVar18 + 0x30) - uVar14;
  if (*(ulonglong *)(lVar18 + 0x30) < uVar14) {
    local_c0 = 8;
    uVar12 = uVar17;
  }
  *puVar4 = (*(longlong *)(lVar18 + 0x40) - uVar12) - uVar5;
  lVar19 = uVar5 * 0x10 + local_b8;
  lVar18 = uVar12 * 0x10 + local_c0;
  uVar1 = *(undefined4 *)(in_RDX + 0x1c);
  puVar6 = *(undefined4 **)(in_RDX + 0x10);
  uVar2 = *(undefined4 *)(in_R9 + 0x1c);
  puVar7 = *(undefined4 **)(in_R9 + 0x10);
  puVar8 = *(ulonglong **)(in_stack_00000028 + 8);
  puVar9 = *(ulonglong **)(in_stack_00000028 + 0x18);
  puVar10 = *(ulonglong **)(in_stack_00000028 + 0x10);
  uVar3 = *(undefined4 *)(in_R8 + 0x1c);
  puVar11 = *(undefined4 **)(in_R8 + 0x10);
LAB_140aac318:
  if ((local_b8 == 0) || (local_b8 == lVar19)) {
    if ((local_c0 == 0) || (local_c0 == lVar18)) {
      return;
    }
    local_c0 = local_c0 + 0x10;
    local_b8 = 0;
  }
  else {
    local_b8 = local_b8 + 0x10;
  }
  *puVar4 = *puVar4 + 1;
  *puVar6 = uVar1;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::walls::WallId,bevy_ecs::entity::Entity>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,bevy_ecs::entity::Entity>_>
            ();
  *puVar7 = uVar2;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::walls::WallId,bevy_ecs::entity::Entity>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,bevy_ecs::entity::Entity>_>
            ();
  puVar16 = (ulonglong *)0x0;
  if (local_a8._4_4_ != 0) {
    puVar16 = local_a8;
  }
  if ((ulonglong)puVar16 >> 0x20 != 0) {
    bevy_ecs::system::commands::EntityCommands::despawn();
    local_a8 = puVar8;
    puStack_a0 = puVar10;
    puStack_98 = puVar9;
    local_90 = puVar16;
  }
  puVar16 = local_a8;
  *puVar11 = uVar3;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>_>
            ();
  if (!SBORROW8(0,(longlong)local_a8)) {
    local_158 = puStack_a0;
    puStack_150 = puStack_98;
  }
  local_a8 = local_158;
  puStack_a0 = puStack_150;
  puStack_98 = (ulonglong *)0x0;
  puVar13 = (ulonglong *)0x0;
  do {
    puVar15 = puVar16;
    if (puVar13 == (ulonglong *)0x0) goto LAB_140aac4c0;
    if (local_90 != local_80) goto LAB_140aac53f;
    do {
      do {
        if (local_88 != (ulonglong *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        puStack_98 = (ulonglong *)0x0;
        puVar15 = puVar16;
LAB_140aac4c0:
        if (SBORROW8(0,(longlong)puVar15)) goto LAB_140aac318;
        local_80 = local_158 + (longlong)puStack_150;
        puStack_98 = local_158;
        local_90 = local_158;
        puVar16 = (ulonglong *)&DAT_8000000000000000;
        puVar13 = local_158;
        local_88 = puVar15;
      } while (local_158 == local_80);
LAB_140aac53f:
      puVar15 = local_90 + 1;
      uVar5 = *local_90;
      local_90 = puVar15;
    } while (uVar5 >> 0x20 == 0);
    bevy_ecs::system::commands::EntityCommands::despawn();
  } while( true );
}

