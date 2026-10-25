140a4b740: 55  push rbp
140a4b741: 56  push rsi
140a4b742: 57  push rdi
140a4b743: 4881eca0000000  sub rsp, 0xa0
140a4b74a: 488dac2480000000  lea rbp, [rsp + 0x80]
140a4b752: 0f297510  movaps xmmword ptr [rbp + 0x10], xmm6
140a4b756: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
140a4b75e: 4889d7  mov rdi, rdx
140a4b761: 4889ce  mov rsi, rcx
140a4b764: f30f107210  movss xmm6, dword ptr [rdx + 0x10]
140a4b769: 48b80000000000000080  movabs rax, 0x8000000000000000
140a4b773: 488945d0  mov qword ptr [rbp - 0x30], rax
140a4b777: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
140a4b77f: 48c745f800000000  mov qword ptr [rbp - 8], 0
140a4b787: c6450701  mov byte ptr [rbp + 7], 1
140a4b78b: 488d15dd3d0802  lea rdx, [rip + 0x2083ddd]
140a4b792: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b796: 41b80a000000  mov r8d, 0xa
140a4b79c: 4989f9  mov r9, rdi
140a4b79f: e83cd2b400  call 0x1415989e0
140a4b7a4: 4885c0  test rax, rax
140a4b7a7: 7523  jne 0x140a4b7cc
140a4b7a9: 4c8d4f08  lea r9, [rdi + 8]
140a4b7ad: c6450701  mov byte ptr [rbp + 7], 1
140a4b7b1: 488d15c13d0802  lea rdx, [rip + 0x2083dc1]
140a4b7b8: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b7bc: 41b802000000  mov r8d, 2
140a4b7c2: e8a9aeb400  call 0x141596670
140a4b7c7: 4885c0  test rax, rax
140a4b7ca: 742f  je 0x140a4b7fb
140a4b7cc: 488d4de8  lea rcx, [rbp - 0x18]
140a4b7d0: 48894608  mov qword ptr [rsi + 8], rax
140a4b7d4: c60606  mov byte ptr [rsi], 6
140a4b7d7: e864d7e4ff  call 0x140898f40
140a4b7dc: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b7e0: 4889d0  mov rax, rdx
140a4b7e3: 48f7d8  neg rax
140a4b7e6: 7045  jo 0x140a4b82d
140a4b7e8: 7343  jae 0x140a4b82d
140a4b7ea: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b7ee: 41b801000000  mov r8d, 1
140a4b7f4: e82784bcff  call 0x140613c20
140a4b7f9: eb32  jmp 0x140a4b82d
140a4b7fb: 0f57c0  xorps xmm0, xmm0
140a4b7fe: 0f2ef0  ucomiss xmm6, xmm0
140a4b801: 753c  jne 0x140a4b83f
140a4b803: 7a3a  jp 0x140a4b83f
140a4b805: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
140a4b809: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
140a4b80d: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
140a4b811: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
140a4b815: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
140a4b819: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
140a4b81d: c6450700  mov byte ptr [rbp + 7], 0
140a4b821: 488d55a0  lea rdx, [rbp - 0x60]
140a4b825: 4889f1  mov rcx, rsi
140a4b828: e893ac1300  call 0x140b864c0
140a4b82d: 4889f0  mov rax, rsi
140a4b830: 0f287510  movaps xmm6, xmmword ptr [rbp + 0x10]
140a4b834: 4881c4a0000000  add rsp, 0xa0
140a4b83b: 5f  pop rdi
140a4b83c: 5e  pop rsi
140a4b83d: 5d  pop rbp
140a4b83e: c3  ret
140a4b83f: 4883c710  add rdi, 0x10
140a4b843: c6450701  mov byte ptr [rbp + 7], 1
140a4b847: 488d15c13c0802  lea rdx, [rip + 0x2083cc1]
140a4b84e: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b852: 41b80b000000  mov r8d, 0xb
140a4b858: 4989f9  mov r9, rdi
140a4b85b: e8c0a1b400  call 0x141595a20
140a4b860: 4885c0  test rax, rax
140a4b863: 0f8563ffffff  jne 0x140a4b7cc
140a4b869: eb9a  jmp 0x140a4b805
140a4b86b: 0f1f440000  nop dword ptr [rax + rax]
140a4b870: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b875: 55  push rbp
140a4b876: 56  push rsi
140a4b877: 57  push rdi
140a4b878: 4883ec30  sub rsp, 0x30
140a4b87c: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b883: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
140a4b888: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b88c: 488d045500000000  lea rax, [rdx*2]
140a4b894: 4885c0  test rax, rax
140a4b897: 740f  je 0x140a4b8a8
140a4b899: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b89d: 41b801000000  mov r8d, 1
140a4b8a3: e87883bcff  call 0x140613c20
140a4b8a8: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
140a4b8ad: 4883c430  add rsp, 0x30
140a4b8b1: 5f  pop rdi
140a4b8b2: 5e  pop rsi
140a4b8b3: 5d  pop rbp
140a4b8b4: c3  ret
140a4b8b5: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a4b8c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b8c5: 55  push rbp
140a4b8c6: 56  push rsi
140a4b8c7: 57  push rdi
140a4b8c8: 4883ec30  sub rsp, 0x30
140a4b8cc: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b8d3: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
140a4b8d8: 807d0700  cmp byte ptr [rbp + 7], 0
140a4b8dc: 750d  jne 0x140a4b8eb
140a4b8de: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
140a4b8e3: 4883c430  add rsp, 0x30
140a4b8e7: 5f  pop rdi
140a4b8e8: 5e  pop rsi
140a4b8e9: 5d  pop rbp
140a4b8ea: c3  ret
140a4b8eb: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b8ef: e86c22e1ff  call 0x14085db60
140a4b8f4: ebe8  jmp 0x140a4b8de
