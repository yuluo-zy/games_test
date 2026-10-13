// addr=0x140a97a30 name=startup_deferred_raster
// signature=void __cdecl startup_deferred_raster(void)


void __cdecl country_core::startup::startup_deferred_raster::startup_deferred_raster(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 *in_R9;
  
  load_shader_into_library<ref_<str_>_>();
  uVar1 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar2 = (undefined4 *)in_R9[2];
  *puVar2 = uVar1;
  puVar3 = (undefined8 *)*in_R9;
  puVar3[2] = 1;
  puVar3[3] = extraout_RAX;
  load_shader_into_library<ref_<str_>_>();
  *puVar2 = uVar1;
  puVar3[6] = 1;
  puVar3[7] = extraout_RAX_00;
  load_shader_into_library<ref_<str_>_>();
  *puVar2 = uVar1;
  *puVar3 = 1;
  puVar3[1] = extraout_RAX_01;
  load_shader_into_library<ref_<str_>_>();
  *puVar2 = uVar1;
  puVar3[4] = 1;
  puVar3[5] = extraout_RAX_02;
  return;
}


