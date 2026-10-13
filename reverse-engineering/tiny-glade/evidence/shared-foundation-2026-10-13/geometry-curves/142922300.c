
void FUN_142922300(void)

{
  code *pcVar1;
  
  FUN_14261ef30();
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

