
void __cdecl
alloc::sync::impl$74::
to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>_>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_env_0>_>
          (void)

{
  undefined8 *puVar1;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *in_RCX;
  size_t sVar2;
  longlong local_e0;
  void *local_d8;
  longlong local_d0;
  undefined1 local_c8 [144];
  
  memcpy(local_c8,in_RCX,0x90);
  vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>_>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_env_0>_>
            ();
  sVar2 = local_d0 * 4;
  arcinner_layout_for_value_layout();
  puVar1 = extraout_RAX;
  if (sVar2 != 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    puVar1 = extraout_RAX_00;
  }
  if (puVar1 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::handle_alloc_error();
  }
  *puVar1 = 1;
  puVar1[1] = 1;
  memcpy(puVar1 + 2,local_d8,local_d0 * 4);
  if (local_e0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

