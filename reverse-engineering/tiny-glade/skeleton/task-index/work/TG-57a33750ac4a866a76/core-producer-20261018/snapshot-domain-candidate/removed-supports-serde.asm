140b3bad0: 55  push rbp
140b3bad1: 56  push rsi
140b3bad2: 4881ec98000000  sub rsp, 0x98
140b3bad9: 488dac2480000000  lea rbp, [rsp + 0x80]
140b3bae1: 48c74510feffffff  mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
140b3bae9: 4989d1  mov r9, rdx
140b3baec: 4889ce  mov rsi, rcx
140b3baef: 48b80000000000000080  movabs rax, 0x8000000000000000
140b3baf9: 488945d8  mov qword ptr [rbp - 0x28], rax
140b3bafd: 48c745f000000000  mov qword ptr [rbp - 0x10], 0
140b3bb05: 48c7450000000000  mov qword ptr [rbp], 0
140b3bb0d: c6450f01  mov byte ptr [rbp + 0xf], 1
140b3bb11: 488d156a27fb01  lea rdx, [rip + 0x1fb276a]
140b3bb18: 488d4dd8  lea rcx, [rbp - 0x28]
140b3bb1c: 41b805000000  mov r8d, 5
140b3bb22: e869b9a500  call 0x141597490
140b3bb27: 4885c0  test rax, rax
140b3bb2a: 742f  je 0x140b3bb5b
140b3bb2c: 488d4df0  lea rcx, [rbp - 0x10]
140b3bb30: 48894608  mov qword ptr [rsi + 8], rax
140b3bb34: c60606  mov byte ptr [rsi], 6
140b3bb37: e804d4d5ff  call 0x140898f40
140b3bb3c: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
140b3bb40: 4889d0  mov rax, rdx
140b3bb43: 48f7d8  neg rax
140b3bb46: 703b  jo 0x140b3bb83
140b3bb48: 7339  jae 0x140b3bb83
140b3bb4a: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
140b3bb4e: 41b801000000  mov r8d, 1
140b3bb54: e8c780adff  call 0x140613c20
140b3bb59: eb28  jmp 0x140b3bb83
140b3bb5b: 0f1045d8  movups xmm0, xmmword ptr [rbp - 0x28]
140b3bb5f: 0f104de8  movups xmm1, xmmword ptr [rbp - 0x18]
140b3bb63: 0f1055f8  movups xmm2, xmmword ptr [rbp - 8]
140b3bb67: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
140b3bb6b: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
140b3bb6f: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
140b3bb73: c6450f00  mov byte ptr [rbp + 0xf], 0
140b3bb77: 488d55a0  lea rdx, [rbp - 0x60]
140b3bb7b: 4889f1  mov rcx, rsi
140b3bb7e: e83da90400  call 0x140b864c0
140b3bb83: 4889f0  mov rax, rsi
140b3bb86: 4881c498000000  add rsp, 0x98
140b3bb8d: 5e  pop rsi
140b3bb8e: 5d  pop rbp
140b3bb8f: c3  ret
140b3bb90: 4889542410  mov qword ptr [rsp + 0x10], rdx
140b3bb95: 55  push rbp
140b3bb96: 56  push rsi
140b3bb97: 4883ec28  sub rsp, 0x28
140b3bb9b: 488daa80000000  lea rbp, [rdx + 0x80]
140b3bba2: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
140b3bba6: 488d045500000000  lea rax, [rdx*2]
140b3bbae: 4885c0  test rax, rax
140b3bbb1: 740f  je 0x140b3bbc2
140b3bbb3: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
140b3bbb7: 41b801000000  mov r8d, 1
140b3bbbd: e85e80adff  call 0x140613c20
140b3bbc2: 90  nop
140b3bbc3: 4883c428  add rsp, 0x28
140b3bbc7: 5e  pop rsi
140b3bbc8: 5d  pop rbp
140b3bbc9: c3  ret
140b3bbca: 660f1f440000  nop word ptr [rax + rax]
140b3bbd0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140b3bbd5: 55  push rbp
140b3bbd6: 56  push rsi
140b3bbd7: 4883ec28  sub rsp, 0x28
140b3bbdb: 488daa80000000  lea rbp, [rdx + 0x80]
140b3bbe2: 807d0f00  cmp byte ptr [rbp + 0xf], 0
140b3bbe6: 7507  jne 0x140b3bbef
140b3bbe8: 4883c428  add rsp, 0x28
140b3bbec: 5e  pop rsi
140b3bbed: 5d  pop rbp
140b3bbee: c3  ret
140b3bbef: 488d4dd8  lea rcx, [rbp - 0x28]
140b3bbf3: e8681fd2ff  call 0x14085db60
140b3bbf8: ebee  jmp 0x140b3bbe8
