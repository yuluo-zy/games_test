1416e77e0: 55  push rbp
1416e77e1: 56  push rsi
1416e77e2: 57  push rdi
1416e77e3: 4881ec90000000  sub rsp, 0x90
1416e77ea: 488dac2480000000  lea rbp, [rsp + 0x80]
1416e77f2: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
1416e77fa: 4889d7  mov rdi, rdx
1416e77fd: 4889ce  mov rsi, rcx
1416e7800: 48b80000000000000080  movabs rax, 0x8000000000000000
1416e780a: 488945d0  mov qword ptr [rbp - 0x30], rax
1416e780e: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
1416e7816: 48c745f800000000  mov qword ptr [rbp - 8], 0
1416e781e: c6450701  mov byte ptr [rbp + 7], 1
1416e7822: 488d150f7a5a01  lea rdx, [rip + 0x15a7a0f]
1416e7829: 488d4dd0  lea rcx, [rbp - 0x30]
1416e782d: 41b801000000  mov r8d, 1
1416e7833: 4989f9  mov r9, rdi
1416e7836: e8e5e1eaff  call 0x141595a20
1416e783b: 4885c0  test rax, rax
1416e783e: 7526  jne 0x1416e7866
1416e7840: 4883c704  add rdi, 4
1416e7844: c6450701  mov byte ptr [rbp + 7], 1
1416e7848: 488d15ea795a01  lea rdx, [rip + 0x15a79ea]
1416e784f: 488d4dd0  lea rcx, [rbp - 0x30]
1416e7853: 41b801000000  mov r8d, 1
1416e7859: 4989f9  mov r9, rdi
1416e785c: e8bfe1eaff  call 0x141595a20
1416e7861: 4885c0  test rax, rax
1416e7864: 742f  je 0x1416e7895
1416e7866: 488d4de8  lea rcx, [rbp - 0x18]
1416e786a: 48894608  mov qword ptr [rsi + 8], rax
1416e786e: c60606  mov byte ptr [rsi], 6
1416e7871: e8ca161bff  call 0x140898f40
1416e7876: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
1416e787a: 4889d0  mov rax, rdx
1416e787d: 48f7d8  neg rax
1416e7880: 703b  jo 0x1416e78bd
1416e7882: 7339  jae 0x1416e78bd
1416e7884: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1416e7888: 41b801000000  mov r8d, 1
1416e788e: e88dc3f2fe  call 0x140613c20
1416e7893: eb28  jmp 0x1416e78bd
1416e7895: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
1416e7899: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
1416e789d: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
1416e78a1: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
1416e78a5: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
1416e78a9: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1416e78ad: c6450700  mov byte ptr [rbp + 7], 0
1416e78b1: 488d55a0  lea rdx, [rbp - 0x60]
1416e78b5: 4889f1  mov rcx, rsi
1416e78b8: e803ec49ff  call 0x140b864c0
1416e78bd: 4889f0  mov rax, rsi
1416e78c0: 4881c490000000  add rsp, 0x90
1416e78c7: 5f  pop rdi
1416e78c8: 5e  pop rsi
1416e78c9: 5d  pop rbp
1416e78ca: c3  ret
1416e78cb: 0f1f440000  nop dword ptr [rax + rax]
1416e78d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416e78d5: 55  push rbp
1416e78d6: 56  push rsi
1416e78d7: 57  push rdi
1416e78d8: 4883ec20  sub rsp, 0x20
1416e78dc: 488daa80000000  lea rbp, [rdx + 0x80]
1416e78e3: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
1416e78e7: 488d045500000000  lea rax, [rdx*2]
1416e78ef: 4885c0  test rax, rax
1416e78f2: 740f  je 0x1416e7903
1416e78f4: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1416e78f8: 41b801000000  mov r8d, 1
1416e78fe: e81dc3f2fe  call 0x140613c20
1416e7903: 90  nop
1416e7904: 4883c420  add rsp, 0x20
1416e7908: 5f  pop rdi
1416e7909: 5e  pop rsi
1416e790a: 5d  pop rbp
1416e790b: c3  ret
1416e790c: 0f1f4000  nop dword ptr [rax]
1416e7910: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416e7915: 55  push rbp
1416e7916: 56  push rsi
1416e7917: 57  push rdi
1416e7918: 4883ec20  sub rsp, 0x20
1416e791c: 488daa80000000  lea rbp, [rdx + 0x80]
1416e7923: 807d0700  cmp byte ptr [rbp + 7], 0
1416e7927: 7508  jne 0x1416e7931
1416e7929: 4883c420  add rsp, 0x20
1416e792d: 5f  pop rdi
1416e792e: 5e  pop rsi
1416e792f: 5d  pop rbp
1416e7930: c3  ret
1416e7931: 488d4dd0  lea rcx, [rbp - 0x30]
1416e7935: e8266217ff  call 0x14085db60
1416e793a: ebed  jmp 0x1416e7929
