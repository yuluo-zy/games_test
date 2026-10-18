14157d810: 55  push rbp
14157d811: 56  push rsi
14157d812: 57  push rdi
14157d813: 4881ec90000000  sub rsp, 0x90
14157d81a: 488dac2480000000  lea rbp, [rsp + 0x80]
14157d822: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
14157d82a: 4889d7  mov rdi, rdx
14157d82d: 4889ce  mov rsi, rcx
14157d830: 48b80000000000000080  movabs rax, 0x8000000000000000
14157d83a: 488945d0  mov qword ptr [rbp - 0x30], rax
14157d83e: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
14157d846: 48c745f800000000  mov qword ptr [rbp - 8], 0
14157d84e: 4c8d4a10  lea r9, [rdx + 0x10]
14157d852: c6450701  mov byte ptr [rbp + 7], 1
14157d856: 488d15899f6f01  lea rdx, [rip + 0x16f9f89]
14157d85d: 488d4dd0  lea rcx, [rbp - 0x30]
14157d861: 41b805000000  mov r8d, 5
14157d867: e8a4700100  call 0x141594910
14157d86c: 4885c0  test rax, rax
14157d86f: 7522  jne 0x14157d893
14157d871: c6450701  mov byte ptr [rbp + 7], 1
14157d875: 488d156f9f6f01  lea rdx, [rip + 0x16f9f6f]
14157d87c: 488d4dd0  lea rcx, [rbp - 0x30]
14157d880: 41b808000000  mov r8d, 8
14157d886: 4989f9  mov r9, rdi
14157d889: e812b60100  call 0x141598ea0
14157d88e: 4885c0  test rax, rax
14157d891: 742f  je 0x14157d8c2
14157d893: 488d4de8  lea rcx, [rbp - 0x18]
14157d897: 48894608  mov qword ptr [rsi + 8], rax
14157d89b: c60606  mov byte ptr [rsi], 6
14157d89e: e89db631ff  call 0x140898f40
14157d8a3: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
14157d8a7: 4889d0  mov rax, rdx
14157d8aa: 48f7d8  neg rax
14157d8ad: 703b  jo 0x14157d8ea
14157d8af: 7339  jae 0x14157d8ea
14157d8b1: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
14157d8b5: 41b801000000  mov r8d, 1
14157d8bb: e8606309ff  call 0x140613c20
14157d8c0: eb28  jmp 0x14157d8ea
14157d8c2: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
14157d8c6: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
14157d8ca: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
14157d8ce: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
14157d8d2: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
14157d8d6: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
14157d8da: c6450700  mov byte ptr [rbp + 7], 0
14157d8de: 488d55a0  lea rdx, [rbp - 0x60]
14157d8e2: 4889f1  mov rcx, rsi
14157d8e5: e8d68b60ff  call 0x140b864c0
14157d8ea: 4889f0  mov rax, rsi
14157d8ed: 4881c490000000  add rsp, 0x90
14157d8f4: 5f  pop rdi
14157d8f5: 5e  pop rsi
14157d8f6: 5d  pop rbp
14157d8f7: c3  ret
14157d8f8: 0f1f840000000000  nop dword ptr [rax + rax]
14157d900: 4889542410  mov qword ptr [rsp + 0x10], rdx
14157d905: 55  push rbp
14157d906: 56  push rsi
14157d907: 57  push rdi
14157d908: 4883ec20  sub rsp, 0x20
14157d90c: 488daa80000000  lea rbp, [rdx + 0x80]
14157d913: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
14157d917: 488d045500000000  lea rax, [rdx*2]
14157d91f: 4885c0  test rax, rax
14157d922: 740f  je 0x14157d933
14157d924: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
14157d928: 41b801000000  mov r8d, 1
14157d92e: e8ed6209ff  call 0x140613c20
14157d933: 90  nop
14157d934: 4883c420  add rsp, 0x20
14157d938: 5f  pop rdi
14157d939: 5e  pop rsi
14157d93a: 5d  pop rbp
14157d93b: c3  ret
14157d93c: 0f1f4000  nop dword ptr [rax]
14157d940: 4889542410  mov qword ptr [rsp + 0x10], rdx
14157d945: 55  push rbp
14157d946: 56  push rsi
14157d947: 57  push rdi
14157d948: 4883ec20  sub rsp, 0x20
14157d94c: 488daa80000000  lea rbp, [rdx + 0x80]
14157d953: 807d0700  cmp byte ptr [rbp + 7], 0
14157d957: 7508  jne 0x14157d961
14157d959: 4883c420  add rsp, 0x20
14157d95d: 5f  pop rdi
14157d95e: 5e  pop rsi
14157d95f: 5d  pop rbp
14157d960: c3  ret
14157d961: 488d4dd0  lea rcx, [rbp - 0x30]
14157d965: e8f6012eff  call 0x14085db60
14157d96a: ebed  jmp 0x14157d959
