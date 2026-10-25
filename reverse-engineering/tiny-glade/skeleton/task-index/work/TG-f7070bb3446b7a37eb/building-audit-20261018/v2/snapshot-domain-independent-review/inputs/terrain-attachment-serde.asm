140a4b5a0: 55  push rbp
140a4b5a1: 56  push rsi
140a4b5a2: 57  push rdi
140a4b5a3: 4881eca0000000  sub rsp, 0xa0
140a4b5aa: 488dac2480000000  lea rbp, [rsp + 0x80]
140a4b5b2: 0f297510  movaps xmmword ptr [rbp + 0x10], xmm6
140a4b5b6: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
140a4b5be: 4889d7  mov rdi, rdx
140a4b5c1: 4889ce  mov rsi, rcx
140a4b5c4: f30f107208  movss xmm6, dword ptr [rdx + 8]
140a4b5c9: 48b80000000000000080  movabs rax, 0x8000000000000000
140a4b5d3: 488945d0  mov qword ptr [rbp - 0x30], rax
140a4b5d7: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
140a4b5df: 48c745f800000000  mov qword ptr [rbp - 8], 0
140a4b5e7: c6450701  mov byte ptr [rbp + 7], 1
140a4b5eb: 488d15173f0802  lea rdx, [rip + 0x2083f17]
140a4b5f2: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b5f6: 41b806000000  mov r8d, 6
140a4b5fc: 4989f9  mov r9, rdi
140a4b5ff: e86cb0b400  call 0x141596670
140a4b604: 4885c0  test rax, rax
140a4b607: 742f  je 0x140a4b638
140a4b609: 488d4de8  lea rcx, [rbp - 0x18]
140a4b60d: 48894608  mov qword ptr [rsi + 8], rax
140a4b611: c60606  mov byte ptr [rsi], 6
140a4b614: e827d9e4ff  call 0x140898f40
140a4b619: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b61d: 4889d0  mov rax, rdx
140a4b620: 48f7d8  neg rax
140a4b623: 7045  jo 0x140a4b66a
140a4b625: 7343  jae 0x140a4b66a
140a4b627: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b62b: 41b801000000  mov r8d, 1
140a4b631: e8ea85bcff  call 0x140613c20
140a4b636: eb32  jmp 0x140a4b66a
140a4b638: 0f57c0  xorps xmm0, xmm0
140a4b63b: 0f2ef0  ucomiss xmm6, xmm0
140a4b63e: 753c  jne 0x140a4b67c
140a4b640: 7a3a  jp 0x140a4b67c
140a4b642: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
140a4b646: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
140a4b64a: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
140a4b64e: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
140a4b652: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
140a4b656: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
140a4b65a: c6450700  mov byte ptr [rbp + 7], 0
140a4b65e: 488d55a0  lea rdx, [rbp - 0x60]
140a4b662: 4889f1  mov rcx, rsi
140a4b665: e856ae1300  call 0x140b864c0
140a4b66a: 4889f0  mov rax, rsi
140a4b66d: 0f287510  movaps xmm6, xmmword ptr [rbp + 0x10]
140a4b671: 4881c4a0000000  add rsp, 0xa0
140a4b678: 5f  pop rdi
140a4b679: 5e  pop rsi
140a4b67a: 5d  pop rbp
140a4b67b: c3  ret
140a4b67c: 4883c708  add rdi, 8
140a4b680: c6450701  mov byte ptr [rbp + 7], 1
140a4b684: 488d15843e0802  lea rdx, [rip + 0x2083e84]
140a4b68b: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b68f: 41b80b000000  mov r8d, 0xb
140a4b695: 4989f9  mov r9, rdi
140a4b698: e883a3b400  call 0x141595a20
140a4b69d: 4885c0  test rax, rax
140a4b6a0: 0f8563ffffff  jne 0x140a4b609
140a4b6a6: eb9a  jmp 0x140a4b642
140a4b6a8: 0f1f840000000000  nop dword ptr [rax + rax]
140a4b6b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b6b5: 55  push rbp
140a4b6b6: 56  push rsi
140a4b6b7: 57  push rdi
140a4b6b8: 4883ec30  sub rsp, 0x30
140a4b6bc: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b6c3: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
140a4b6c8: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b6cc: 488d045500000000  lea rax, [rdx*2]
140a4b6d4: 4885c0  test rax, rax
140a4b6d7: 740f  je 0x140a4b6e8
140a4b6d9: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b6dd: 41b801000000  mov r8d, 1
140a4b6e3: e83885bcff  call 0x140613c20
140a4b6e8: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
140a4b6ed: 4883c430  add rsp, 0x30
140a4b6f1: 5f  pop rdi
140a4b6f2: 5e  pop rsi
140a4b6f3: 5d  pop rbp
140a4b6f4: c3  ret
140a4b6f5: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a4b700: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b705: 55  push rbp
140a4b706: 56  push rsi
140a4b707: 57  push rdi
140a4b708: 4883ec30  sub rsp, 0x30
140a4b70c: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b713: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
140a4b718: 807d0700  cmp byte ptr [rbp + 7], 0
140a4b71c: 750d  jne 0x140a4b72b
140a4b71e: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
140a4b723: 4883c430  add rsp, 0x30
140a4b727: 5f  pop rdi
140a4b728: 5e  pop rsi
140a4b729: 5d  pop rbp
140a4b72a: c3  ret
140a4b72b: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b72f: e82c24e1ff  call 0x14085db60
140a4b734: ebe8  jmp 0x140a4b71e
