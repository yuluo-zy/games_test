
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Switch with 1 destination removed at 0x0001422cd9b5 */
/* WARNING: Exceeded maximum restarts with more pending */

void __cdecl
system_clutter::autoclutter::add_autoclutter_on_windows::add_autoclutter_on_windows(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong in_RCX;
  longlong *in_R9;
  
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar1 = *in_R9;
  lVar2 = *(longlong *)(lVar1 + 0x10);
  *(undefined8 *)(lVar1 + 0x10) = 0;
  if (lVar2 != 0) {
    if (**(char **)(lVar1 + 8) != '\x06') {
      **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
    }
  }
  return;
}

