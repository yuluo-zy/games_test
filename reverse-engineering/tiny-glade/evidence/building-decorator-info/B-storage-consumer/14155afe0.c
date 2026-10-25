
void __cdecl
country_core::resources::walls::decorator_storage::Decorators::
insert<country_core::resources::walls::decorator_storage::DecoMoveToStorage>(void)

{
  longlong extraout_RAX;
  undefined8 *in_RCX;
  ulonglong uVar1;
  uint in_R8D;
  undefined1 local_ec [4];
  undefined1 local_e8 [8];
  ulonglong local_e0;
  undefined1 *local_d8;
  code *local_d0;
  undefined1 *local_c8;
  code *local_c0;
  code *local_b0;
  undefined **local_a8;
  undefined8 local_a0;
  undefined1 **local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined7 local_38;
  char cStack_31;
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
            ();
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (cStack_31 == '\x02') {
    uVar1 = (ulonglong)in_R8D;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::decorator::DecoratorId,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if (extraout_RAX != 3) {
      local_d8 = local_ec;
      local_d0 = decorator::impl$27::fmt;
      local_c8 = local_e8;
      local_c0 = decorator_storage::impl$25::fmt;
      local_b0 = decorator_storage::impl$25::fmt;
      local_a8 = &anon_087c9b6d159f545cda87747878cb277f_52_llvm_10190951805537787575;
      local_a0 = 3;
      local_88 = 0;
      local_98 = &local_d8;
      local_90 = 3;
      local_e0 = uVar1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    *(undefined1 *)((longlong)in_RCX + 0x47) = 2;
  }
  else {
    in_RCX[8] = CONCAT17(cStack_31,local_38);
    in_RCX[6] = local_48;
    in_RCX[7] = uStack_40;
    in_RCX[4] = local_58;
    in_RCX[5] = uStack_50;
    in_RCX[2] = local_68;
    in_RCX[3] = uStack_60;
    *in_RCX = local_78;
    in_RCX[1] = uStack_70;
    decorator_storage::impl$8::update();
  }
  return;
}

