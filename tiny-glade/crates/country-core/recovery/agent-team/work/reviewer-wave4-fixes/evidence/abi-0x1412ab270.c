
// WARNING: Type propagation algorithm not settling

void __cdecl
system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::
build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>(void)

{
  undefined8 *puVar1;
  float *pfVar2;
  byte bVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  int iVar15;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong lVar16;
  byte *in_RCX;
  uint in_EDX;
  longlong *plVar17;
  byte bVar18;
  longlong lVar19;
  float fVar20;
  float fVar28;
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  float fVar29;
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  undefined8 local_118;
  undefined4 local_110;
  longlong *local_100;
  longlong *local_f8;
  undefined8 local_e8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong *local_c8;
  undefined4 uStack_b8;
  undefined4 local_b4;
  
  fVar6 = *(float *)(in_RCX + 0xc0);
  fVar7 = *(float *)(in_RCX + 0xc4);
  fVar8 = *(float *)(in_RCX + 200);
  fVar44 = *(float *)(in_RCX + 0xcc);
  fVar42 = (float)*(undefined8 *)(in_RCX + 0xd0);
  fVar43 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0xd0) >> 0x20);
  fVar41 = *(float *)(in_RCX + 0xd8);
  local_110 = *(undefined4 *)(in_RCX + 0xe4);
  local_118 = *(undefined8 *)(in_RCX + 0xdc);
  fVar20 = *(float *)(in_RCX + 0x140);
  fVar37 = (float)*(undefined8 *)(in_RCX + 0x144);
  fVar38 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0x144) >> 0x20);
  if (0.0 < SQRT(fVar38 * fVar38 + fVar20 * fVar20 + fVar37 * fVar37)) {
    fVar39 = fVar44 * fVar44;
    fVar40 = fVar8 * fVar8 + fVar7 * fVar7 + fVar6 * fVar6;
    fVar29 = fVar8 * fVar38 + fVar7 * fVar37 + fVar6 * fVar20;
    fVar29 = fVar29 + fVar29;
    fVar28 = fVar44 + fVar44;
    fVar42 = fVar42 + fVar28 * (fVar7 * fVar38 - fVar37 * fVar8) +
                      fVar29 * fVar6 + (fVar39 - fVar40) * fVar20;
    fVar43 = fVar43 + fVar28 * (fVar8 * fVar20 - fVar38 * fVar6) +
                      fVar29 * fVar7 + (fVar39 - fVar40) * fVar37;
    fVar41 = fVar41 + fVar28 * (fVar6 * fVar37 - fVar20 * fVar7) +
                      fVar29 * fVar8 + (fVar39 - fVar40) * fVar38;
  }
  if (*(int *)(in_RCX + 0x30) == 1) {
    local_110 = *(undefined4 *)(in_RCX + 0x3c);
    local_118 = *(undefined8 *)(in_RCX + 0x34);
  }
  uVar5 = *(undefined8 *)(in_RCX + 0xe8);
  plVar17 = *(longlong **)(in_RCX + 0xb0);
  switch(*(undefined8 *)(in_RCX + 0xb8)) {
  case 9:
    if ((char)plVar17[1] == 'r' && *plVar17 == 0x6562755f696e616e) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::UberInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    break;
  case 10:
    if ((short)plVar17[1] == 0x6874 && *plVar17 == 0x6f6c635f696e616e) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::ClothInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    if ((short)plVar17[1] == 0x656b && *plVar17 == 0x6f6d735f696e616e) {
      country_core::resources::walls::decorator::DecoratorRank::new();
      country_core::components::drawable::impl$1::from();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SmokeInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    break;
  case 0x11:
    auVar21[0] = -((char)*plVar17 == 'n');
    auVar21[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar21[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar21[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar21[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar21[5] = -(*(char *)((longlong)plVar17 + 5) == 'b');
    auVar21[6] = -(*(char *)((longlong)plVar17 + 6) == 'a');
    auVar21[7] = -(*(char *)((longlong)plVar17 + 7) == 'l');
    auVar21[8] = -((char)plVar17[1] == 'c');
    auVar21[9] = -(*(char *)((longlong)plVar17 + 9) == 'o');
    auVar21[10] = -(*(char *)((longlong)plVar17 + 10) == 'n');
    auVar21[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == 'y');
    auVar21[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == '_');
    auVar21[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'd');
    auVar21[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 'o');
    auVar21[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'o');
    auVar30[0] = -((char)plVar17[2] == 'r');
    auVar30[1] = 0xff;
    auVar30[2] = 0xff;
    auVar30[3] = 0xff;
    auVar30[4] = 0xff;
    auVar30[5] = 0xff;
    auVar30[6] = 0xff;
    auVar30[7] = 0xff;
    auVar30[8] = 0xff;
    auVar30[9] = 0xff;
    auVar30[10] = 0xff;
    auVar30[0xb] = 0xff;
    auVar30[0xc] = 0xff;
    auVar30[0xd] = 0xff;
    auVar30[0xe] = 0xff;
    auVar30[0xf] = 0xff;
    auVar30 = auVar30 & auVar21;
    if ((ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar30[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::BalconyDoorInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    break;
  case 0x15:
    auVar35[0] = -(*(char *)((longlong)plVar17 + 5) == 'l');
    auVar35[1] = -(*(char *)((longlong)plVar17 + 6) == 'a');
    auVar35[2] = -(*(char *)((longlong)plVar17 + 7) == 'n');
    auVar35[3] = -((char)plVar17[1] == 't');
    auVar35[4] = -(*(char *)((longlong)plVar17 + 9) == 'e');
    auVar35[5] = -(*(char *)((longlong)plVar17 + 10) == 'r');
    auVar35[6] = -(*(char *)((longlong)plVar17 + 0xb) == 'n');
    auVar35[7] = -(*(char *)((longlong)plVar17 + 0xc) == '_');
    auVar35[8] = -(*(char *)((longlong)plVar17 + 0xd) == 'e');
    auVar35[9] = -(*(char *)((longlong)plVar17 + 0xe) == 'm');
    auVar35[10] = -(*(char *)((longlong)plVar17 + 0xf) == 'i');
    auVar35[0xb] = -((char)plVar17[2] == 's');
    auVar35[0xc] = -(*(char *)((longlong)plVar17 + 0x11) == 's');
    auVar35[0xd] = -(*(char *)((longlong)plVar17 + 0x12) == 'i');
    auVar35[0xe] = -(*(char *)((longlong)plVar17 + 0x13) == 'v');
    auVar35[0xf] = -(*(char *)((longlong)plVar17 + 0x14) == 'e');
    auVar26[0] = -((char)*plVar17 == 'n');
    auVar26[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar26[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar26[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar26[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar26[5] = -(*(char *)((longlong)plVar17 + 5) == 'l');
    auVar26[6] = -(*(char *)((longlong)plVar17 + 6) == 'a');
    auVar26[7] = -(*(char *)((longlong)plVar17 + 7) == 'n');
    auVar26[8] = -((char)plVar17[1] == 't');
    auVar26[9] = -(*(char *)((longlong)plVar17 + 9) == 'e');
    auVar26[10] = -(*(char *)((longlong)plVar17 + 10) == 'r');
    auVar26[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == 'n');
    auVar26[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == '_');
    auVar26[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'e');
    auVar26[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 'm');
    auVar26[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'i');
    auVar26 = auVar26 & auVar35;
    if ((ushort)((ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar26[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::LanternEmissiveInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    break;
  case 0x17:
    auVar36[0] = -(*(char *)((longlong)plVar17 + 7) == 'l');
    auVar36[1] = -((char)plVar17[1] == 'i');
    auVar36[2] = -(*(char *)((longlong)plVar17 + 9) == 'd');
    auVar36[3] = -(*(char *)((longlong)plVar17 + 10) == '_');
    auVar36[4] = -(*(char *)((longlong)plVar17 + 0xb) == 'v');
    auVar36[5] = -(*(char *)((longlong)plVar17 + 0xc) == 'e');
    auVar36[6] = -(*(char *)((longlong)plVar17 + 0xd) == 'r');
    auVar36[7] = -(*(char *)((longlong)plVar17 + 0xe) == 't');
    auVar36[8] = -(*(char *)((longlong)plVar17 + 0xf) == 'e');
    auVar36[9] = -((char)plVar17[2] == 'x');
    auVar36[10] = -(*(char *)((longlong)plVar17 + 0x11) == '_');
    auVar36[0xb] = -(*(char *)((longlong)plVar17 + 0x12) == 'c');
    auVar36[0xc] = -(*(char *)((longlong)plVar17 + 0x13) == 'o');
    auVar36[0xd] = -(*(char *)((longlong)plVar17 + 0x14) == 'l');
    auVar36[0xe] = -(*(char *)((longlong)plVar17 + 0x15) == 'o');
    auVar36[0xf] = -(*(char *)((longlong)plVar17 + 0x16) == 'r');
    auVar27[0] = -((char)*plVar17 == 'n');
    auVar27[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar27[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar27[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar27[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar27[5] = -(*(char *)((longlong)plVar17 + 5) == 's');
    auVar27[6] = -(*(char *)((longlong)plVar17 + 6) == 'o');
    auVar27[7] = -(*(char *)((longlong)plVar17 + 7) == 'l');
    auVar27[8] = -((char)plVar17[1] == 'i');
    auVar27[9] = -(*(char *)((longlong)plVar17 + 9) == 'd');
    auVar27[10] = -(*(char *)((longlong)plVar17 + 10) == '_');
    auVar27[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == 'v');
    auVar27[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == 'e');
    auVar27[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'r');
    auVar27[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 't');
    auVar27[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'e');
    auVar27 = auVar27 & auVar36;
    if ((ushort)((ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar27[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_00;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 3;
code_r0x0001412abcec:
      *(undefined1 *)(lVar16 + 0x58 + lVar19) = 0;
      *(ulonglong *)(lVar16 + 0x68 + lVar19) = CONCAT44(local_b4,uStack_b8);
      *(code **)(lVar16 + 0x78 + lVar19) = bevy_ecs::error::handler::panic;
      lVar19 = lVar19 + 0x88;
code_r0x0001412abded:
      plVar17[2] = lVar19;
      local_e8 = CONCAT44(local_b4,uStack_b8);
      local_100 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      local_f8 = local_c8;
      goto LAB_1412abe15;
    }
    break;
  case 0x19:
    auVar31[0] = -(*(char *)((longlong)plVar17 + 9) == 'i');
    auVar31[1] = -(*(char *)((longlong)plVar17 + 10) == 'c');
    auVar31[2] = -(*(char *)((longlong)plVar17 + 0xb) == '_');
    auVar31[3] = -(*(char *)((longlong)plVar17 + 0xc) == 'w');
    auVar31[4] = -(*(char *)((longlong)plVar17 + 0xd) == 'i');
    auVar31[5] = -(*(char *)((longlong)plVar17 + 0xe) == 'n');
    auVar31[6] = -(*(char *)((longlong)plVar17 + 0xf) == 'd');
    auVar31[7] = -((char)plVar17[2] == 'o');
    auVar31[8] = -(*(char *)((longlong)plVar17 + 0x11) == 'w');
    auVar31[9] = -(*(char *)((longlong)plVar17 + 0x12) == '_');
    auVar31[10] = -(*(char *)((longlong)plVar17 + 0x13) == 'b');
    auVar31[0xb] = -(*(char *)((longlong)plVar17 + 0x14) == 'r');
    auVar31[0xc] = -(*(char *)((longlong)plVar17 + 0x15) == 'i');
    auVar31[0xd] = -(*(char *)((longlong)plVar17 + 0x16) == 'c');
    auVar31[0xe] = -(*(char *)((longlong)plVar17 + 0x17) == 'k');
    auVar31[0xf] = -((char)plVar17[3] == 's');
    auVar22[0] = -((char)*plVar17 == 'n');
    auVar22[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar22[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar22[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar22[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar22[5] = -(*(char *)((longlong)plVar17 + 5) == 'g');
    auVar22[6] = -(*(char *)((longlong)plVar17 + 6) == 'o');
    auVar22[7] = -(*(char *)((longlong)plVar17 + 7) == 't');
    auVar22[8] = -((char)plVar17[1] == 'h');
    auVar22[9] = -(*(char *)((longlong)plVar17 + 9) == 'i');
    auVar22[10] = -(*(char *)((longlong)plVar17 + 10) == 'c');
    auVar22[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == '_');
    auVar22[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == 'w');
    auVar22[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'i');
    auVar22[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 'n');
    auVar22[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'd');
    auVar22 = auVar22 & auVar31;
    if ((ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar22[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    break;
  case 0x1b:
    auVar32[0] = -(*(char *)((longlong)plVar17 + 0xb) == 'r');
    auVar32[1] = -(*(char *)((longlong)plVar17 + 0xc) == '_');
    auVar32[2] = -(*(char *)((longlong)plVar17 + 0xd) == 'S');
    auVar32[3] = -(*(char *)((longlong)plVar17 + 0xe) == 'o');
    auVar32[4] = -(*(char *)((longlong)plVar17 + 0xf) == 'l');
    auVar32[5] = -((char)plVar17[2] == 'i');
    auVar32[6] = -(*(char *)((longlong)plVar17 + 0x11) == 'd');
    auVar32[7] = -(*(char *)((longlong)plVar17 + 0x12) == 'A');
    auVar32[8] = -(*(char *)((longlong)plVar17 + 0x13) == 'l');
    auVar32[9] = -(*(char *)((longlong)plVar17 + 0x14) == 'b');
    auVar32[10] = -(*(char *)((longlong)plVar17 + 0x15) == 'e');
    auVar32[0xb] = -(*(char *)((longlong)plVar17 + 0x16) == 'd');
    auVar32[0xc] = -(*(char *)((longlong)plVar17 + 0x17) == 'o');
    auVar32[0xd] = -((char)plVar17[3] == 'M');
    auVar32[0xe] = -(*(char *)((longlong)plVar17 + 0x19) == 'a');
    auVar32[0xf] = -(*(char *)((longlong)plVar17 + 0x1a) == 'p');
    auVar23[0] = -((char)*plVar17 == 'n');
    auVar23[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar23[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar23[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar23[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar23[5] = -(*(char *)((longlong)plVar17 + 5) == 'c');
    auVar23[6] = -(*(char *)((longlong)plVar17 + 6) == 'l');
    auVar23[7] = -(*(char *)((longlong)plVar17 + 7) == 'u');
    auVar23[8] = -((char)plVar17[1] == 't');
    auVar23[9] = -(*(char *)((longlong)plVar17 + 9) == 't');
    auVar23[10] = -(*(char *)((longlong)plVar17 + 10) == 'e');
    auVar23[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == 'r');
    auVar23[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == '_');
    auVar23[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'S');
    auVar23[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 'o');
    auVar23[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'l');
    auVar23 = auVar23 & auVar32;
    if ((ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar23[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bVar18 = in_RCX[0x150];
      if (bVar18 == 9) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x98) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 0xf;
code_r0x0001412abdc7:
      *(byte *)(lVar16 + 0x58 + lVar19) = bVar18;
      *(undefined1 *)(lVar16 + 0x68 + lVar19) = 0;
      *(ulonglong *)(lVar16 + 0x78 + lVar19) = CONCAT44(local_b4,uStack_b8);
      *(code **)(lVar16 + 0x88 + lVar19) = bevy_ecs::error::handler::panic;
      lVar19 = lVar19 + 0x98;
      goto code_r0x0001412abded;
    }
    break;
  case 0x1e:
    auVar33[0] = -(*(char *)((longlong)plVar17 + 0xe) == 't');
    auVar33[1] = -(*(char *)((longlong)plVar17 + 0xf) == 'e');
    auVar33[2] = -((char)plVar17[2] == 'x');
    auVar33[3] = -(*(char *)((longlong)plVar17 + 0x11) == '_');
    auVar33[4] = -(*(char *)((longlong)plVar17 + 0x12) == 'c');
    auVar33[5] = -(*(char *)((longlong)plVar17 + 0x13) == 'o');
    auVar33[6] = -(*(char *)((longlong)plVar17 + 0x14) == 'l');
    auVar33[7] = -(*(char *)((longlong)plVar17 + 0x15) == 'o');
    auVar33[8] = -(*(char *)((longlong)plVar17 + 0x16) == 'r');
    auVar33[9] = -(*(char *)((longlong)plVar17 + 0x17) == '_');
    auVar33[10] = -((char)plVar17[3] == 'w');
    auVar33[0xb] = -(*(char *)((longlong)plVar17 + 0x19) == 'i');
    auVar33[0xc] = -(*(char *)((longlong)plVar17 + 0x1a) == 'n');
    auVar33[0xd] = -(*(char *)((longlong)plVar17 + 0x1b) == 'd');
    auVar33[0xe] = -(*(char *)((longlong)plVar17 + 0x1c) == 'o');
    auVar33[0xf] = -(*(char *)((longlong)plVar17 + 0x1d) == 'w');
    auVar24[0] = -((char)*plVar17 == 'n');
    auVar24[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar24[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar24[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar24[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar24[5] = -(*(char *)((longlong)plVar17 + 5) == 's');
    auVar24[6] = -(*(char *)((longlong)plVar17 + 6) == 'o');
    auVar24[7] = -(*(char *)((longlong)plVar17 + 7) == 'l');
    auVar24[8] = -((char)plVar17[1] == 'i');
    auVar24[9] = -(*(char *)((longlong)plVar17 + 9) == 'd');
    auVar24[10] = -(*(char *)((longlong)plVar17 + 10) == '_');
    auVar24[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == 'v');
    auVar24[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == 'e');
    auVar24[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'r');
    auVar24[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 't');
    auVar24[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'e');
    auVar24 = auVar24 & auVar33;
    if ((ushort)((ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar24[0xf] >> 7) << 0xf) == 0xffff) {
      if (*(longlong *)in_RCX != 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_03;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 4;
      goto code_r0x0001412abcec;
    }
    auVar34[0] = -(*(char *)((longlong)plVar17 + 0xe) == 't');
    auVar34[1] = -(*(char *)((longlong)plVar17 + 0xf) == 'e');
    auVar34[2] = -((char)plVar17[2] == 'x');
    auVar34[3] = -(*(char *)((longlong)plVar17 + 0x11) == '_');
    auVar34[4] = -(*(char *)((longlong)plVar17 + 0x12) == 'c');
    auVar34[5] = -(*(char *)((longlong)plVar17 + 0x13) == 'o');
    auVar34[6] = -(*(char *)((longlong)plVar17 + 0x14) == 'l');
    auVar34[7] = -(*(char *)((longlong)plVar17 + 0x15) == 'o');
    auVar34[8] = -(*(char *)((longlong)plVar17 + 0x16) == 'r');
    auVar34[9] = -(*(char *)((longlong)plVar17 + 0x17) == '_');
    auVar34[10] = -((char)plVar17[3] == 'b');
    auVar34[0xb] = -(*(char *)((longlong)plVar17 + 0x19) == 'r');
    auVar34[0xc] = -(*(char *)((longlong)plVar17 + 0x1a) == 'i');
    auVar34[0xd] = -(*(char *)((longlong)plVar17 + 0x1b) == 'c');
    auVar34[0xe] = -(*(char *)((longlong)plVar17 + 0x1c) == 'k');
    auVar34[0xf] = -(*(char *)((longlong)plVar17 + 0x1d) == 's');
    auVar25[0] = -((char)*plVar17 == 'n');
    auVar25[1] = -(*(char *)((longlong)plVar17 + 1) == 'a');
    auVar25[2] = -(*(char *)((longlong)plVar17 + 2) == 'n');
    auVar25[3] = -(*(char *)((longlong)plVar17 + 3) == 'i');
    auVar25[4] = -(*(char *)((longlong)plVar17 + 4) == '_');
    auVar25[5] = -(*(char *)((longlong)plVar17 + 5) == 's');
    auVar25[6] = -(*(char *)((longlong)plVar17 + 6) == 'o');
    auVar25[7] = -(*(char *)((longlong)plVar17 + 7) == 'l');
    auVar25[8] = -((char)plVar17[1] == 'i');
    auVar25[9] = -(*(char *)((longlong)plVar17 + 9) == 'd');
    auVar25[10] = -(*(char *)((longlong)plVar17 + 10) == '_');
    auVar25[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == 'v');
    auVar25[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == 'e');
    auVar25[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == 'r');
    auVar25[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == 't');
    auVar25[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == 'e');
    auVar25 = auVar25 & auVar34;
    if ((ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar25[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>_>
                ();
      goto LAB_1412abe15;
    }
    break;
  case 0x23:
    iVar15 = memcmp(plVar17,&UNK_142c2ae10,0x23);
    if (iVar15 == 0) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bVar18 = in_RCX[0x150];
      if (bVar18 == 9) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x98) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_04;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 0x10;
      goto code_r0x0001412abdc7;
    }
    break;
  case 0x24:
    iVar15 = memcmp(plVar17,&UNK_142c2ae33,0x24);
    if (iVar15 == 0) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_01;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 5;
      goto code_r0x0001412abcec;
    }
  }
  country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name();
  country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  bevy_ecs::system::commands::Commands::
  spawn<country_core::components::drawable::DrawableMeshBundle>();
LAB_1412abe15:
  if ((in_RCX[0xf0] & 1) == 0) {
    iVar15 = *(int *)(in_RCX + 0x40);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar4 = *(undefined4 *)(in_RCX + 0xf4);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_id::ColorId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined4 *)(lVar16 + 0x18 + lVar19) = uVar4;
    *(undefined1 *)(lVar16 + 0x1c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x20;
    iVar15 = *(int *)(in_RCX + 0x40);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x58) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::GothicWindowBrickDeformation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    uVar5 = *(undefined8 *)(in_RCX + 0x4c);
    uVar9 = *(undefined8 *)(in_RCX + 0x54);
    uVar10 = *(undefined8 *)(in_RCX + 0x5c);
    uVar11 = *(undefined8 *)(in_RCX + 100);
    uVar12 = *(undefined8 *)(in_RCX + 0x6c);
    uVar13 = *(undefined8 *)(in_RCX + 0x70);
    uVar14 = *(undefined8 *)(in_RCX + 0x78);
    puVar1 = (undefined8 *)(lVar16 + 0x18 + lVar19);
    *puVar1 = *(undefined8 *)(in_RCX + 0x44);
    puVar1[1] = uVar5;
    puVar1 = (undefined8 *)(lVar16 + 0x28 + lVar19);
    *puVar1 = uVar9;
    puVar1[1] = uVar10;
    puVar1 = (undefined8 *)(lVar16 + 0x38 + lVar19);
    *puVar1 = uVar11;
    puVar1[1] = uVar12;
    puVar1 = (undefined8 *)(lVar16 + 0x44 + lVar19);
    *puVar1 = uVar13;
    puVar1[1] = uVar14;
    *(undefined1 *)(lVar16 + 0x54 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x58;
    bVar18 = in_RCX[0x114];
  }
  else {
    bVar18 = in_RCX[0x114];
  }
  if (bVar18 == 2) {
    iVar15 = *(int *)(in_RCX + 0x80);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::BrickColor>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    uVar5 = *(undefined8 *)(in_RCX + 0x108);
    puVar1 = (undefined8 *)(lVar16 + 0x18 + lVar19);
    *puVar1 = *(undefined8 *)(in_RCX + 0x100);
    puVar1[1] = uVar5;
    *(undefined8 *)(lVar16 + 0x28 + lVar19) = *(undefined8 *)(in_RCX + 0x110);
    *(undefined1 *)(lVar16 + 0x30 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x38;
    iVar15 = *(int *)(in_RCX + 0x80);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::ClothlineCloth>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = *(undefined8 *)(in_RCX + 0x84);
    *(undefined4 *)(lVar16 + 0x20 + lVar19) = *(undefined4 *)(in_RCX + 0x8c);
    *(undefined1 *)(lVar16 + 0x24 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x28;
    bVar18 = in_RCX[0x14e];
  }
  else {
    bVar18 = in_RCX[0x14e];
  }
  if (bVar18 == 0) {
    bVar18 = in_RCX[0x10];
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::SemiTransparent>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined1 *)(lVar16 + 0x18 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x20;
    bVar18 = in_RCX[0x10];
  }
  if ((bVar18 & 1) == 0) {
    bVar18 = in_RCX[0x14f];
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar5 = *(undefined8 *)(in_RCX + 0x18);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::window_lights::WindowLight>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = uVar5;
    *(uint *)(lVar16 + 0x20 + lVar19) = in_EDX;
    *(undefined1 *)(lVar16 + 0x28 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x30;
    bVar18 = in_RCX[0x14f];
  }
  if (bVar18 == 0) {
    bVar18 = in_RCX[0x14c];
  }
  else {
    fVar37 = fVar44 * fVar44;
    fVar38 = fVar8 * fVar8 + fVar7 * fVar7 + fVar6 * fVar6;
    fVar20 = fVar8 * 1.0 + fVar7 * 0.0 + fVar6 * 0.0;
    fVar20 = fVar20 + fVar20;
    fVar28 = fVar44 + fVar44;
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::BalconyDoor>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
    *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
    *(ulonglong *)(lVar16 + 0x24 + lVar19) =
         CONCAT44(fVar28 * (fVar8 * 0.0 - fVar6 * 1.0) + fVar20 * fVar7 + (fVar37 - fVar38) * 0.0,
                  fVar28 * (fVar7 * 1.0 - fVar8 * 0.0) + fVar20 * fVar6 + (fVar37 - fVar38) * 0.0);
    *(float *)(lVar16 + 0x2c + lVar19) =
         fVar28 * (fVar6 * 0.0 - fVar7 * 0.0) + fVar20 * fVar8 + (fVar37 - fVar38) * 1.0;
    *(undefined1 *)(lVar16 + 0x30 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x38;
    bVar18 = in_RCX[0x14c];
  }
  if (bVar18 == 2) {
    iVar15 = *(int *)(in_RCX + 0x20);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    bVar3 = in_RCX[0x14d];
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::Smoke>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(ulonglong *)(lVar16 + 0x18 + lVar19) = (ulonglong)in_EDX;
    *(undefined4 *)(lVar16 + 0x20 + lVar19) = 0;
    *(byte *)(lVar16 + 0x24 + lVar19) = bVar18;
    *(byte *)(lVar16 + 0x25 + lVar19) = bVar3;
    *(undefined1 *)(lVar16 + 0x28 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x30;
    iVar15 = *(int *)(in_RCX + 0x20);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar5 = *(undefined8 *)(in_RCX + 0x28);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::AlbedoMap>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = uVar5;
    *(undefined1 *)(lVar16 + 0x20 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x28;
    bVar18 = *in_RCX;
  }
  else {
    bVar18 = *in_RCX;
  }
  if ((bVar18 & 1) == 0) {
    iVar15 = *(int *)(in_RCX + 0xf8);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar5 = *(undefined8 *)(in_RCX + 8);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::NormalMap>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = uVar5;
    *(undefined1 *)(lVar16 + 0x20 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x28;
    iVar15 = *(int *)(in_RCX + 0xf8);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar4 = *(undefined4 *)(in_RCX + 0xfc);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::RoofFlagCloth>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined4 *)(lVar16 + 0x18 + lVar19) = uVar4;
    *(undefined1 *)(lVar16 + 0x1c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x20;
  }
  if (in_RCX[0x13c] == 2) {
    iVar15 = *(int *)(in_RCX + 0x118);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::WallFlagCloth>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    uVar5 = *(undefined8 *)(in_RCX + 0x134);
    puVar1 = (undefined8 *)(lVar16 + 0x18 + lVar19);
    *puVar1 = *(undefined8 *)(in_RCX + 300);
    puVar1[1] = uVar5;
    *(undefined4 *)(lVar16 + 0x28 + lVar19) = *(undefined4 *)(in_RCX + 0x13c);
    *(undefined1 *)(lVar16 + 0x2c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x30;
    iVar15 = *(int *)(in_RCX + 0x118);
  }
  if (iVar15 == 1) {
    fVar20 = *(float *)(in_RCX + 0x11c);
    fVar37 = *(float *)(in_RCX + 0x120);
    fVar38 = *(float *)(in_RCX + 0x124);
    fVar28 = *(float *)(in_RCX + 0x128);
    fVar39 = fVar44 * fVar44;
    fVar40 = fVar8 * fVar8 + fVar7 * fVar7 + fVar6 * fVar6;
    fVar29 = fVar8 * fVar38 + fVar7 * fVar37 + fVar6 * fVar20;
    fVar29 = fVar29 + fVar29;
    fVar44 = fVar44 + fVar44;
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::LanternLightParams>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(ulonglong *)(lVar16 + 0x18 + lVar19) =
         CONCAT44(fVar43 + fVar44 * (fVar8 * fVar20 - fVar38 * fVar6) +
                           fVar29 * fVar7 + (fVar39 - fVar40) * fVar37,
                  fVar42 + fVar44 * (fVar7 * fVar38 - fVar37 * fVar8) +
                           fVar29 * fVar6 + (fVar39 - fVar40) * fVar20);
    *(float *)(lVar16 + 0x20 + lVar19) =
         fVar41 + fVar44 * (fVar6 * fVar37 - fVar20 * fVar7) +
                  fVar29 * fVar8 + (fVar39 - fVar40) * fVar38;
    pfVar2 = (float *)(lVar16 + 0x24 + lVar19);
    *pfVar2 = fVar20;
    pfVar2[1] = fVar37;
    pfVar2[2] = fVar38;
    pfVar2[3] = fVar28;
    *(uint *)(lVar16 + 0x34 + lVar19) = in_EDX;
    *(undefined1 *)(lVar16 + 0x38 + lVar19) = 0;
    *(undefined1 *)(lVar16 + 0x3c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x40;
    iVar15 = *(int *)(in_RCX + 0x90);
  }
  else {
    iVar15 = *(int *)(in_RCX + 0x90);
  }
  if (iVar15 == 1) {
    if (local_100 == (longlong *)0x0) {
      local_100 = local_f8;
    }
    lVar19 = local_100[2];
    if ((ulonglong)(*local_100 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = local_100[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::SittableForBirds>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(uint *)(lVar16 + 0x18 + lVar19) = in_EDX;
    *(undefined8 *)(lVar16 + 0x1c + lVar19) = *(undefined8 *)(in_RCX + 0x94);
    *(undefined4 *)(lVar16 + 0x24 + lVar19) = *(undefined4 *)(in_RCX + 0x9c);
    *(undefined1 *)(lVar16 + 0x28 + lVar19) = 0;
    local_100[2] = lVar19 + 0x30;
  }
  decorator_visual_state::impl$5::add_decorator_visual_entity();
  return;
}

