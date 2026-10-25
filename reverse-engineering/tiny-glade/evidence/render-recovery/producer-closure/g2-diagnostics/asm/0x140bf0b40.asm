==== 0x140bf0b40  file_off=0xbeff40
  140bf0b40  4883ec28                 sub rsp, 0x28
  140bf0b44  4883b97008000000         cmp qword ptr [rcx + 0x870], 0
  140bf0b4c  7417                     je 0x140bf0b65
  140bf0b4e  833a01                   cmp dword ptr [rdx], 1
  140bf0b51  7518                     jne 0x140bf0b6b
  140bf0b53  4889c8                   mov rax, rcx
  140bf0b56  488b4a48                 mov rcx, qword ptr [rdx + 0x48]
  140bf0b5a  4883c428                 add rsp, 0x28
  140bf0b5e  48ffa0a0080000           jmp qword ptr [rax + 0x8a0]
  140bf0b65  90                       nop 
  140bf0b66  4883c428                 add rsp, 0x28
  140bf0b6a  c3                       ret 
  140bf0b6b  488d0d0650f201           lea rcx, [rip + 0x1f25006] -> 0x142b15b78
  140bf0b72  4c8d057750f201           lea r8, [rip + 0x1f25077] -> 0x142b15bf0
  140bf0b79  ba4c000000               mov edx, 0x4c
  140bf0b7e  e8ad88ce01               call 0x1428d9430
  140bf0b83  cc                       int3 
  140bf0b84  cc                       int3 
  140bf0b85  cc                       int3 
  140bf0b86  cc                       int3 
  140bf0b87  cc                       int3 
  140bf0b88  cc                       int3 
  140bf0b89  cc                       int3 
  140bf0b8a  cc                       int3 
  140bf0b8b  cc                       int3 
  140bf0b8c  cc                       int3 
  140bf0b8d  cc                       int3 
  140bf0b8e  cc                       int3 
  140bf0b8f  cc                       int3 
