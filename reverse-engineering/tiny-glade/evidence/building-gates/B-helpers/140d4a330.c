
void __cdecl log::__private_api::impl$2::log(void)

{
  undefined *puVar1;
  undefined *puVar2;
  
  puVar1 = &DAT_142b45148;
  if (DAT_143639318 == 2) {
    puVar1 = PTR_DAT_14362c3a0;
  }
  puVar2 = &anon_d7930390cc6d146afe44e7a8e6a05df0_3_llvm_9299697823655079385;
  if (DAT_143639318 == 2) {
    puVar2 = DAT_14362c398;
  }
                    /* WARNING: Could not recover jumptable at 0x000140d4a35d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(puVar1 + 0x20))(puVar2);
  return;
}

