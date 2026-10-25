// 0x1409b8340 call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$35,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_> size=231

void __cdecl
core::ops::function::FnOnce::
call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_35,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
          (void)

{
  int iVar1;
  longlong in_RCX;
  int *in_RDX;
  int *piVar2;
  byte bVar3;
  undefined1 local_118 [160];
  byte local_78;
  undefined2 local_77;
  undefined1 local_75;
  int local_74;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined8 local_40;
  undefined8 uStack_38;
  undefined8 local_30;
  
  bVar3 = *(byte *)(in_RCX + 0xa8);
  if (*in_RDX == 1) {
    piVar2 = in_RDX + 1;
    bVar3 = bVar3 & 0xfc | 2;
  }
  else {
    piVar2 = (int *)(in_RCX + 0xac);
  }
  iVar1 = *piVar2;
  memcpy(local_118,(void *)(in_RCX + 8),0xa0);
  local_77 = *(undefined2 *)(in_RCX + 0xa9);
  local_75 = *(undefined1 *)(in_RCX + 0xab);
  local_70 = *(undefined8 *)(in_RCX + 0xb0);
  uStack_68 = *(undefined8 *)(in_RCX + 0xb8);
  local_60 = *(undefined8 *)(in_RCX + 0xc0);
  uStack_58 = *(undefined8 *)(in_RCX + 200);
  local_50 = *(undefined8 *)(in_RCX + 0xd0);
  uStack_48 = *(undefined8 *)(in_RCX + 0xd8);
  local_40 = *(undefined8 *)(in_RCX + 0xe0);
  uStack_38 = *(undefined8 *)(in_RCX + 0xe8);
  local_30 = *(undefined8 *)(in_RCX + 0xf0);
  local_78 = bVar3;
  local_74 = iVar1;
  country_core::render::pass_builder::draw_builder::DrawBuilder::draw();
  return;
}

