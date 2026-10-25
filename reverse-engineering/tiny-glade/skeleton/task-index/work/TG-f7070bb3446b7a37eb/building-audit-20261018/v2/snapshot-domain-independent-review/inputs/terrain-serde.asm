1408e6b70: 55  push rbp
1408e6b71: 56  push rsi
1408e6b72: 57  push rdi
1408e6b73: 4881ec90000000  sub rsp, 0x90
1408e6b7a: 488dac2480000000  lea rbp, [rsp + 0x80]
1408e6b82: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
1408e6b8a: 4889d7  mov rdi, rdx
1408e6b8d: 4889ce  mov rsi, rcx
1408e6b90: 48b80000000000000080  movabs rax, 0x8000000000000000
1408e6b9a: 488945d0  mov qword ptr [rbp - 0x30], rax
1408e6b9e: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
1408e6ba6: 48c745f800000000  mov qword ptr [rbp - 8], 0
1408e6bae: 4c8d4a08  lea r9, [rdx + 8]
1408e6bb2: c6450701  mov byte ptr [rbp + 7], 1
1408e6bb6: 488d152cf11b02  lea rdx, [rip + 0x21bf12c]
1408e6bbd: 488d4dd0  lea rcx, [rbp - 0x30]
1408e6bc1: 41b807000000  mov r8d, 7
1408e6bc7: e864c4ca00  call 0x141593030
1408e6bcc: 4885c0  test rax, rax
1408e6bcf: 7526  jne 0x1408e6bf7
1408e6bd1: 4883c738  add rdi, 0x38
1408e6bd5: c6450701  mov byte ptr [rbp + 7], 1
1408e6bd9: 488d1510f11b02  lea rdx, [rip + 0x21bf110]
1408e6be0: 488d4dd0  lea rcx, [rbp - 0x30]
1408e6be4: 41b80e000000  mov r8d, 0xe
1408e6bea: 4989f9  mov r9, rdi
1408e6bed: e8aef8ca00  call 0x1415964a0
1408e6bf2: 4885c0  test rax, rax
1408e6bf5: 742f  je 0x1408e6c26
1408e6bf7: 488d4de8  lea rcx, [rbp - 0x18]
1408e6bfb: 48894608  mov qword ptr [rsi + 8], rax
1408e6bff: c60606  mov byte ptr [rsi], 6
1408e6c02: e83923fbff  call 0x140898f40
1408e6c07: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
1408e6c0b: 4889d0  mov rax, rdx
1408e6c0e: 48f7d8  neg rax
1408e6c11: 703b  jo 0x1408e6c4e
1408e6c13: 7339  jae 0x1408e6c4e
1408e6c15: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1408e6c19: 41b801000000  mov r8d, 1
1408e6c1f: e8fccfd2ff  call 0x140613c20
1408e6c24: eb28  jmp 0x1408e6c4e
1408e6c26: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
1408e6c2a: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
1408e6c2e: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
1408e6c32: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
1408e6c36: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
1408e6c3a: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1408e6c3e: c6450700  mov byte ptr [rbp + 7], 0
1408e6c42: 488d55a0  lea rdx, [rbp - 0x60]
1408e6c46: 4889f1  mov rcx, rsi
1408e6c49: e872f82900  call 0x140b864c0
1408e6c4e: 4889f0  mov rax, rsi
1408e6c51: 4881c490000000  add rsp, 0x90
1408e6c58: 5f  pop rdi
1408e6c59: 5e  pop rsi
1408e6c5a: 5d  pop rbp
1408e6c5b: c3  ret
1408e6c5c: 0f1f4000  nop dword ptr [rax]
1408e6c60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1408e6c65: 55  push rbp
1408e6c66: 56  push rsi
1408e6c67: 57  push rdi
1408e6c68: 4883ec20  sub rsp, 0x20
1408e6c6c: 488daa80000000  lea rbp, [rdx + 0x80]
1408e6c73: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
1408e6c77: 488d045500000000  lea rax, [rdx*2]
1408e6c7f: 4885c0  test rax, rax
1408e6c82: 740f  je 0x1408e6c93
1408e6c84: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1408e6c88: 41b801000000  mov r8d, 1
1408e6c8e: e88dcfd2ff  call 0x140613c20
1408e6c93: 90  nop
1408e6c94: 4883c420  add rsp, 0x20
1408e6c98: 5f  pop rdi
1408e6c99: 5e  pop rsi
1408e6c9a: 5d  pop rbp
1408e6c9b: c3  ret
1408e6c9c: 0f1f4000  nop dword ptr [rax]
1408e6ca0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1408e6ca5: 55  push rbp
1408e6ca6: 56  push rsi
1408e6ca7: 57  push rdi
1408e6ca8: 4883ec20  sub rsp, 0x20
1408e6cac: 488daa80000000  lea rbp, [rdx + 0x80]
1408e6cb3: 807d0700  cmp byte ptr [rbp + 7], 0
1408e6cb7: 7508  jne 0x1408e6cc1
1408e6cb9: 4883c420  add rsp, 0x20
1408e6cbd: 5f  pop rdi
1408e6cbe: 5e  pop rsi
1408e6cbf: 5d  pop rbp
1408e6cc0: c3  ret
1408e6cc1: 488d4dd0  lea rcx, [rbp - 0x30]
1408e6cc5: e8966ef7ff  call 0x14085db60
1408e6cca: ebed  jmp 0x1408e6cb9
