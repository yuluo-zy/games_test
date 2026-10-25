140a4b2a0: 55  push rbp
140a4b2a1: 56  push rsi
140a4b2a2: 57  push rdi
140a4b2a3: 4881ec90000000  sub rsp, 0x90
140a4b2aa: 488dac2480000000  lea rbp, [rsp + 0x80]
140a4b2b2: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
140a4b2ba: 4889d7  mov rdi, rdx
140a4b2bd: 4889ce  mov rsi, rcx
140a4b2c0: 48b80000000000000080  movabs rax, 0x8000000000000000
140a4b2ca: 488945d0  mov qword ptr [rbp - 0x30], rax
140a4b2ce: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
140a4b2d6: 48c745f800000000  mov qword ptr [rbp - 8], 0
140a4b2de: c6450701  mov byte ptr [rbp + 7], 1
140a4b2e2: 488d156d410802  lea rdx, [rip + 0x208416d]
140a4b2e9: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b2ed: 41b805000000  mov r8d, 5
140a4b2f3: 4989f9  mov r9, rdi
140a4b2f6: e845d9b400  call 0x141598c40
140a4b2fb: 4885c0  test rax, rax
140a4b2fe: 7526  jne 0x140a4b326
140a4b300: 4883c708  add rdi, 8
140a4b304: c6450701  mov byte ptr [rbp + 7], 1
140a4b308: 488d154c410802  lea rdx, [rip + 0x208414c]
140a4b30f: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b313: 41b806000000  mov r8d, 6
140a4b319: 4989f9  mov r9, rdi
140a4b31c: e80f9bb400  call 0x141594e30
140a4b321: 4885c0  test rax, rax
140a4b324: 742f  je 0x140a4b355
140a4b326: 488d4de8  lea rcx, [rbp - 0x18]
140a4b32a: 48894608  mov qword ptr [rsi + 8], rax
140a4b32e: c60606  mov byte ptr [rsi], 6
140a4b331: e80adce4ff  call 0x140898f40
140a4b336: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b33a: 4889d0  mov rax, rdx
140a4b33d: 48f7d8  neg rax
140a4b340: 703b  jo 0x140a4b37d
140a4b342: 7339  jae 0x140a4b37d
140a4b344: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b348: 41b801000000  mov r8d, 1
140a4b34e: e8cd88bcff  call 0x140613c20
140a4b353: eb28  jmp 0x140a4b37d
140a4b355: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
140a4b359: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
140a4b35d: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
140a4b361: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
140a4b365: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
140a4b369: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
140a4b36d: c6450700  mov byte ptr [rbp + 7], 0
140a4b371: 488d55a0  lea rdx, [rbp - 0x60]
140a4b375: 4889f1  mov rcx, rsi
140a4b378: e843b11300  call 0x140b864c0
140a4b37d: 4889f0  mov rax, rsi
140a4b380: 4881c490000000  add rsp, 0x90
140a4b387: 5f  pop rdi
140a4b388: 5e  pop rsi
140a4b389: 5d  pop rbp
140a4b38a: c3  ret
140a4b38b: 0f1f440000  nop dword ptr [rax + rax]
140a4b390: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b395: 55  push rbp
140a4b396: 56  push rsi
140a4b397: 57  push rdi
140a4b398: 4883ec20  sub rsp, 0x20
140a4b39c: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b3a3: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b3a7: 488d045500000000  lea rax, [rdx*2]
140a4b3af: 4885c0  test rax, rax
140a4b3b2: 740f  je 0x140a4b3c3
140a4b3b4: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b3b8: 41b801000000  mov r8d, 1
140a4b3be: e85d88bcff  call 0x140613c20
140a4b3c3: 90  nop
140a4b3c4: 4883c420  add rsp, 0x20
140a4b3c8: 5f  pop rdi
140a4b3c9: 5e  pop rsi
140a4b3ca: 5d  pop rbp
140a4b3cb: c3  ret
140a4b3cc: 0f1f4000  nop dword ptr [rax]
140a4b3d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b3d5: 55  push rbp
140a4b3d6: 56  push rsi
140a4b3d7: 57  push rdi
140a4b3d8: 4883ec20  sub rsp, 0x20
140a4b3dc: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b3e3: 807d0700  cmp byte ptr [rbp + 7], 0
140a4b3e7: 7508  jne 0x140a4b3f1
140a4b3e9: 4883c420  add rsp, 0x20
140a4b3ed: 5f  pop rdi
140a4b3ee: 5e  pop rsi
140a4b3ef: 5d  pop rbp
140a4b3f0: c3  ret
140a4b3f1: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b3f5: e86627e1ff  call 0x14085db60
140a4b3fa: ebed  jmp 0x140a4b3e9
