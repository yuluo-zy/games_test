140b363e0: 56  push rsi
140b363e1: 4883ec60  sub rsp, 0x60
140b363e5: 4889ce  mov rsi, rcx
140b363e8: 488d4c2428  lea rcx, [rsp + 0x28]
140b363ed: e8fe66a600  call 0x14159caf0
140b363f2: 488b4c2428  mov rcx, qword ptr [rsp + 0x28]
140b363f7: 488b442430  mov rax, qword ptr [rsp + 0x30]
140b363fc: 4889ca  mov rdx, rcx
140b363ff: 48f7da  neg rdx
140b36402: 7113  jno 0x140b36417
140b36404: 48b90000000000000080  movabs rcx, 0x8000000000000000
140b3640e: 48894608  mov qword ptr [rsi + 8], rax
140b36412: 48890e  mov qword ptr [rsi], rcx
140b36415: eb34  jmp 0x140b3644b
140b36417: 488b542438  mov rdx, qword ptr [rsp + 0x38]
140b3641c: 48c1e204  shl rdx, 4
140b36420: 4801c2  add rdx, rax
140b36423: 4889442440  mov qword ptr [rsp + 0x40], rax
140b36428: 4889442448  mov qword ptr [rsp + 0x48], rax
140b3642d: 48894c2450  mov qword ptr [rsp + 0x50], rcx
140b36432: 4889542458  mov qword ptr [rsp + 0x58], rdx
140b36437: 4c8d051a281002  lea r8, [rip + 0x210281a]
140b3643e: 488d542440  lea rdx, [rsp + 0x40]
140b36443: 4889f1  mov rcx, rsi
140b36446: e8c510c000  call 0x141737510
140b3644b: 4889f0  mov rax, rsi
140b3644e: 4883c460  add rsp, 0x60
140b36452: 5e  pop rsi
140b36453: c3  ret
