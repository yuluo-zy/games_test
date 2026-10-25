
void __cdecl country_core::resources::stairs::stairs_assembly_params::AlignKind::outgoing_dir(void)

{
  char *in_RCX;
  
  if ((*in_RCX != '\0') && (*in_RCX != '\x01')) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  return;
}

