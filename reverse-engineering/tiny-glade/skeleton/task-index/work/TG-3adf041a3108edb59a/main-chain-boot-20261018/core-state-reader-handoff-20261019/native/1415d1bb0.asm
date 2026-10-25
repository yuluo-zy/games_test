1415d1bb0: 55  push rbp
1415d1bb1: 4156  push r14
1415d1bb3: 56  push rsi
1415d1bb4: 57  push rdi
1415d1bb5: 53  push rbx
1415d1bb6: 4881ecb0000000  sub rsp, 0xb0
1415d1bbd: 488dac2480000000  lea rbp, [rsp + 0x80]
1415d1bc5: 48c74528feffffff  mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
1415d1bcd: 4889ce  mov rsi, rcx
1415d1bd0: 0f104218  movups xmm0, xmmword ptr [rdx + 0x18]
1415d1bd4: 0f104a28  movups xmm1, xmmword ptr [rdx + 0x28]
1415d1bd8: 0f294d00  movaps xmmword ptr [rbp], xmm1
1415d1bdc: 0f2945f0  movaps xmmword ptr [rbp - 0x10], xmm0
1415d1be0: 4c8b0a  mov r9, qword ptr [rdx]
1415d1be3: 488b4a08  mov rcx, qword ptr [rdx + 8]
1415d1be7: 488b5210  mov rdx, qword ptr [rdx + 0x10]
1415d1beb: 4883fa04  cmp rdx, 4
1415d1bef: 0f85a2000000  jne 0x1415d1c97
1415d1bf5: 8b01  mov eax, dword ptr [rcx]
1415d1bf7: 3b0553d54f01  cmp eax, dword ptr [rip + 0x14fd553]
1415d1bfd: 741a  je 0x1415d1c19
1415d1bff: 8b01  mov eax, dword ptr [rcx]
1415d1c01: 3b05b3ca4f01  cmp eax, dword ptr [rip + 0x14fcab3]
1415d1c07: 0f858a000000  jne 0x1415d1c97
1415d1c0d: b301  mov bl, 1
1415d1c0f: 4531f6  xor r14d, r14d
1415d1c12: 4d85c9  test r9, r9
1415d1c15: 750c  jne 0x1415d1c23
1415d1c17: eb18  jmp 0x1415d1c31
1415d1c19: 31db  xor ebx, ebx
1415d1c1b: 4531f6  xor r14d, r14d
1415d1c1e: 4d85c9  test r9, r9
1415d1c21: 740e  je 0x1415d1c31
1415d1c23: 41b801000000  mov r8d, 1
1415d1c29: 4c89ca  mov rdx, r9
1415d1c2c: e8ef1f04ff  call 0x140613c20
1415d1c31: 0f2845f0  movaps xmm0, xmmword ptr [rbp - 0x10]
1415d1c35: 0f284d00  movaps xmm1, xmmword ptr [rbp]
1415d1c39: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
1415d1c3d: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415d1c41: 4584f6  test r14b, r14b
1415d1c44: 7416  je 0x1415d1c5c
1415d1c46: 48897e08  mov qword ptr [rsi + 8], rdi
1415d1c4a: c60602  mov byte ptr [rsi], 2
1415d1c4d: c6452700  mov byte ptr [rbp + 0x27], 0
1415d1c51: 488d4da0  lea rcx, [rbp - 0x60]
1415d1c55: e8d6c72bff  call 0x14088e430
1415d1c5a: eb2a  jmp 0x1415d1c86
1415d1c5c: 0f2845f0  movaps xmm0, xmmword ptr [rbp - 0x10]
1415d1c60: 0f284d00  movaps xmm1, xmmword ptr [rbp]
1415d1c64: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415d1c68: 0f1145d0  movups xmmword ptr [rbp - 0x30], xmm0
1415d1c6c: 881e  mov byte ptr [rsi], bl
1415d1c6e: 0f1045c9  movups xmm0, xmmword ptr [rbp - 0x37]
1415d1c72: 0f104dd9  movups xmm1, xmmword ptr [rbp - 0x27]
1415d1c76: 0f114601  movups xmmword ptr [rsi + 1], xmm0
1415d1c7a: 0f114e11  movups xmmword ptr [rsi + 0x11], xmm1
1415d1c7e: 488b45e8  mov rax, qword ptr [rbp - 0x18]
1415d1c82: 48894620  mov qword ptr [rsi + 0x20], rax
1415d1c86: 4889f0  mov rax, rsi
1415d1c89: 4881c4b0000000  add rsp, 0xb0
1415d1c90: 5b  pop rbx
1415d1c91: 5f  pop rdi
1415d1c92: 5e  pop rsi
1415d1c93: 415e  pop r14
1415d1c95: 5d  pop rbp
1415d1c96: c3  ret 
1415d1c97: 4c894d18  mov qword ptr [rbp + 0x18], r9
1415d1c9b: 4c8d05b6d44f01  lea r8, [rip + 0x14fd4b6]
1415d1ca2: 41b902000000  mov r9d, 2
1415d1ca8: 48894d10  mov qword ptr [rbp + 0x10], rcx
1415d1cac: e86f813201  call 0x1428f9e20
1415d1cb1: 4889c7  mov rdi, rax
1415d1cb4: 41b601  mov r14b, 1
1415d1cb7: 4c8b4d18  mov r9, qword ptr [rbp + 0x18]
1415d1cbb: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415d1cbf: 4d85c9  test r9, r9
1415d1cc2: 0f855bffffff  jne 0x1415d1c23
1415d1cc8: e964ffffff  jmp 0x1415d1c31
1415d1ccd: 0f1f00  nop dword ptr [rax]
1415d1cd0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415d1cd5: 55  push rbp
1415d1cd6: 4156  push r14
1415d1cd8: 56  push rsi
1415d1cd9: 57  push rdi
1415d1cda: 53  push rbx
1415d1cdb: 4883ec20  sub rsp, 0x20
1415d1cdf: 488daa80000000  lea rbp, [rdx + 0x80]
1415d1ce6: 48837d1800  cmp qword ptr [rbp + 0x18], 0
1415d1ceb: 7413  je 0x1415d1d00
1415d1ced: 41b801000000  mov r8d, 1
1415d1cf3: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415d1cf7: 488b5518  mov rdx, qword ptr [rbp + 0x18]
1415d1cfb: e8201f04ff  call 0x140613c20
1415d1d00: c6452701  mov byte ptr [rbp + 0x27], 1
1415d1d04: 4883c420  add rsp, 0x20
1415d1d08: 5b  pop rbx
1415d1d09: 5f  pop rdi
1415d1d0a: 5e  pop rsi
1415d1d0b: 415e  pop r14
1415d1d0d: 5d  pop rbp
1415d1d0e: c3  ret 
1415d1d0f: 90  nop 
1415d1d10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415d1d15: 55  push rbp
1415d1d16: 4156  push r14
1415d1d18: 56  push rsi
1415d1d19: 57  push rdi
1415d1d1a: 53  push rbx
1415d1d1b: 4883ec20  sub rsp, 0x20
1415d1d1f: 488daa80000000  lea rbp, [rdx + 0x80]
1415d1d26: 807d2700  cmp byte ptr [rbp + 0x27], 0
1415d1d2a: 7409  je 0x1415d1d35
1415d1d2c: 488d4df0  lea rcx, [rbp - 0x10]
1415d1d30: e8fbc62bff  call 0x14088e430
1415d1d35: 90  nop 
1415d1d36: 4883c420  add rsp, 0x20
1415d1d3a: 5b  pop rbx
1415d1d3b: 5f  pop rdi
1415d1d3c: 5e  pop rsi
1415d1d3d: 415e  pop r14
1415d1d3f: 5d  pop rbp
1415d1d40: c3  ret 
