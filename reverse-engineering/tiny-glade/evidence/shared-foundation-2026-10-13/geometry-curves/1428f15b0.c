
void FUN_1428f15b0(void)

{
  code *pcVar1;
  
  FUN_1411f80d0();
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

