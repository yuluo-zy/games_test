
void __cdecl system_decorator::apply_decorator_op::apply_decorator_op(void)

{
  longlong *in_RCX;
  byte in_DL;
  longlong lVar1;
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  
  if ((in_stack_00000028 == 0 & in_DL) != 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  lVar1 = 0;
  if (*in_RCX - 3U < 2) {
    lVar1 = *in_RCX + -2;
  }
  if (lVar1 == 0) {
    lVar1 = in_RCX[2];
    if (in_stack_00000028 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    archive_current_decorators_in_storage<ref_<str_>_>();
    if (lVar1 != 3) {
      country_core::resources::walls::decorator_archivist::DecoratorArchivist::
      archive_current_decorators_in_storage<ref_<str_>_>();
    }
    if (in_stack_00000030 != (longlong *)0x0) {
      lVar1 = in_stack_00000030[2];
      if (lVar1 == *in_stack_00000030) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(in_stack_00000030[1] + lVar1 * 0x20) = 3;
      in_stack_00000030[2] = lVar1 + 1;
      country_core::resources::walls::decorator::DecoratorLocator::new();
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorLocator,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
    }
    country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_change();
  }
  else if (lVar1 == 1) {
    if (in_stack_00000028 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    archive_current_decorators_in_storage<ref_<str_>_>();
    if (in_stack_00000030 != (longlong *)0x0) {
      lVar1 = in_stack_00000030[2];
      if (lVar1 == *in_stack_00000030) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(in_stack_00000030[1] + lVar1 * 0x20) = 4;
      in_stack_00000030[2] = lVar1 + 1;
      country_core::resources::walls::decorator::DecoratorLocator::new();
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorLocator,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
    }
    country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_place();
  }
  else {
    if (in_stack_00000028 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    archive_current_decorators_in_storage<ref_<str_>_>();
    if (in_stack_00000030 != (longlong *)0x0) {
      lVar1 = in_stack_00000030[2];
      if (lVar1 == *in_stack_00000030) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(in_stack_00000030[1] + lVar1 * 0x20) = 5;
      in_stack_00000030[2] = lVar1 + 1;
    }
    country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_remove();
  }
  country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send();
  return;
}

