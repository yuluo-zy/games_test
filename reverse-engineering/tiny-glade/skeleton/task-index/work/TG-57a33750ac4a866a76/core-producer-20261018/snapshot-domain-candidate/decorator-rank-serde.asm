140a48d80: 55  push rbp
140a48d81: 56  push rsi
140a48d82: 4881ec98000000  sub rsp, 0x98
140a48d89: 488dac2480000000  lea rbp, [rsp + 0x80]
140a48d91: 48c74510feffffff  mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
140a48d99: 4989d1  mov r9, rdx
140a48d9c: 4889ce  mov rsi, rcx
140a48d9f: 48b80000000000000080  movabs rax, 0x8000000000000000
140a48da9: 488945d8  mov qword ptr [rbp - 0x28], rax
140a48dad: 48c745f000000000  mov qword ptr [rbp - 0x10], 0
140a48db5: 48c7450000000000  mov qword ptr [rbp], 0
140a48dbd: c6450f01  mov byte ptr [rbp + 0xf], 1
140a48dc1: 488d155e5b0802  lea rdx, [rip + 0x2085b5e]
140a48dc8: 488d4dd8  lea rcx, [rbp - 0x28]
140a48dcc: 41b801000000  mov r8d, 1
140a48dd2: e8e9e4b400  call 0x1415972c0
140a48dd7: 4885c0  test rax, rax
140a48dda: 742f  je 0x140a48e0b
140a48ddc: 488d4df0  lea rcx, [rbp - 0x10]
140a48de0: 48894608  mov qword ptr [rsi + 8], rax
140a48de4: c60606  mov byte ptr [rsi], 6
140a48de7: e85401e5ff  call 0x140898f40
140a48dec: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
140a48df0: 4889d0  mov rax, rdx
140a48df3: 48f7d8  neg rax
140a48df6: 703b  jo 0x140a48e33
140a48df8: 7339  jae 0x140a48e33
140a48dfa: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
140a48dfe: 41b801000000  mov r8d, 1
140a48e04: e817aebcff  call 0x140613c20
140a48e09: eb28  jmp 0x140a48e33
140a48e0b: 0f1045d8  movups xmm0, xmmword ptr [rbp - 0x28]
140a48e0f: 0f104de8  movups xmm1, xmmword ptr [rbp - 0x18]
140a48e13: 0f1055f8  movups xmm2, xmmword ptr [rbp - 8]
140a48e17: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
140a48e1b: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
140a48e1f: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
140a48e23: c6450f00  mov byte ptr [rbp + 0xf], 0
140a48e27: 488d55a0  lea rdx, [rbp - 0x60]
140a48e2b: 4889f1  mov rcx, rsi
140a48e2e: e88dd61300  call 0x140b864c0
140a48e33: 4889f0  mov rax, rsi
140a48e36: 4881c498000000  add rsp, 0x98
140a48e3d: 5e  pop rsi
140a48e3e: 5d  pop rbp
140a48e3f: c3  ret
140a48e40: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48e45: 55  push rbp
140a48e46: 56  push rsi
140a48e47: 4883ec28  sub rsp, 0x28
140a48e4b: 488daa80000000  lea rbp, [rdx + 0x80]
140a48e52: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
140a48e56: 488d045500000000  lea rax, [rdx*2]
140a48e5e: 4885c0  test rax, rax
140a48e61: 740f  je 0x140a48e72
140a48e63: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
140a48e67: 41b801000000  mov r8d, 1
140a48e6d: e8aeadbcff  call 0x140613c20
140a48e72: 90  nop
140a48e73: 4883c428  add rsp, 0x28
140a48e77: 5e  pop rsi
140a48e78: 5d  pop rbp
140a48e79: c3  ret
140a48e7a: 660f1f440000  nop word ptr [rax + rax]
140a48e80: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48e85: 55  push rbp
140a48e86: 56  push rsi
140a48e87: 4883ec28  sub rsp, 0x28
140a48e8b: 488daa80000000  lea rbp, [rdx + 0x80]
140a48e92: 807d0f00  cmp byte ptr [rbp + 0xf], 0
140a48e96: 7507  jne 0x140a48e9f
140a48e98: 4883c428  add rsp, 0x28
140a48e9c: 5e  pop rsi
140a48e9d: 5d  pop rbp
140a48e9e: c3  ret
140a48e9f: 488d4dd8  lea rcx, [rbp - 0x28]
140a48ea3: e8b84ce1ff  call 0x14085db60
140a48ea8: ebee  jmp 0x140a48e98
