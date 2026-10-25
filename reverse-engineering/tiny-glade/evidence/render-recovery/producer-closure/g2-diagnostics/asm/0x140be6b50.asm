==== 0x140be6b50  file_off=0xbe5f50
  140be6b50  56                       push rsi
  140be6b51  4883ec20                 sub rsp, 0x20
  140be6b55  48833de3e2a40200         cmp qword ptr [rip + 0x2a4e2e3], 0 -> 0x143634e40
  140be6b5d  7418                     je 0x140be6b77
  140be6b5f  4889ce                   mov rsi, rcx
  140be6b62  488d15d7e2a402           lea rdx, [rip + 0x2a4e2d7] -> 0x143634e40
  140be6b69  e8c2a8cd00               call 0x1418c1430
  140be6b6e  4889f0                   mov rax, rsi
  140be6b71  4883c420                 add rsp, 0x20
  140be6b75  5e                       pop rsi
  140be6b76  c3                       ret 
  140be6b77  488d0df2c3f201           lea rcx, [rip + 0x1f2c3f2] -> 0x142b12f70
  140be6b7e  e88d27cf01               call 0x1428d9310
  140be6b83  cc                       int3 
  140be6b84  cc                       int3 
  140be6b85  cc                       int3 
  140be6b86  cc                       int3 
  140be6b87  cc                       int3 
  140be6b88  cc                       int3 
  140be6b89  cc                       int3 
  140be6b8a  cc                       int3 
  140be6b8b  cc                       int3 
  140be6b8c  cc                       int3 
  140be6b8d  cc                       int3 
  140be6b8e  cc                       int3 
  140be6b8f  cc                       int3 
