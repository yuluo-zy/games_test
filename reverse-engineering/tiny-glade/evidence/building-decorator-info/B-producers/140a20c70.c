
void __cdecl
country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_place(void)

{
  longlong in_RCX;
  
  if (*(char *)(in_RCX + 0x1b0) == '\0') {
    get_or_insert_default();
    Decorators::insert<country_core::resources::walls::decorator_storage::DecoMoveToStorage>();
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

