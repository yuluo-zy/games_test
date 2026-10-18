140a4b400: 55  push rbp
140a4b401: 56  push rsi
140a4b402: 57  push rdi
140a4b403: 4881eca0000000  sub rsp, 0xa0
140a4b40a: 488dac2480000000  lea rbp, [rsp + 0x80]
140a4b412: 0f297510  movaps xmmword ptr [rbp + 0x10], xmm6
140a4b416: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
140a4b41e: 4889d7  mov rdi, rdx
140a4b421: 4889ce  mov rsi, rcx
140a4b424: f30f107208  movss xmm6, dword ptr [rdx + 8]
140a4b429: 48b80000000000000080  movabs rax, 0x8000000000000000
140a4b433: 488945d0  mov qword ptr [rbp - 0x30], rax
140a4b437: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
140a4b43f: 48c745f800000000  mov qword ptr [rbp - 8], 0
140a4b447: c6450701  mov byte ptr [rbp + 7], 1
140a4b44b: 488d1564400802  lea rdx, [rip + 0x2084064]
140a4b452: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b456: 41b807000000  mov r8d, 7
140a4b45c: 4989f9  mov r9, rdi
140a4b45f: e80cb2b400  call 0x141596670
140a4b464: 4885c0  test rax, rax
140a4b467: 742f  je 0x140a4b498
140a4b469: 488d4de8  lea rcx, [rbp - 0x18]
140a4b46d: 48894608  mov qword ptr [rsi + 8], rax
140a4b471: c60606  mov byte ptr [rsi], 6
140a4b474: e8c7dae4ff  call 0x140898f40
140a4b479: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b47d: 4889d0  mov rax, rdx
140a4b480: 48f7d8  neg rax
140a4b483: 7045  jo 0x140a4b4ca
140a4b485: 7343  jae 0x140a4b4ca
140a4b487: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b48b: 41b801000000  mov r8d, 1
140a4b491: e88a87bcff  call 0x140613c20
140a4b496: eb32  jmp 0x140a4b4ca
140a4b498: 0f57c0  xorps xmm0, xmm0
140a4b49b: 0f2ef0  ucomiss xmm6, xmm0
140a4b49e: 753c  jne 0x140a4b4dc
140a4b4a0: 7a3a  jp 0x140a4b4dc
140a4b4a2: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
140a4b4a6: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
140a4b4aa: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
140a4b4ae: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
140a4b4b2: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
140a4b4b6: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
140a4b4ba: c6450700  mov byte ptr [rbp + 7], 0
140a4b4be: 488d55a0  lea rdx, [rbp - 0x60]
140a4b4c2: 4889f1  mov rcx, rsi
140a4b4c5: e8f6af1300  call 0x140b864c0
140a4b4ca: 4889f0  mov rax, rsi
140a4b4cd: 0f287510  movaps xmm6, xmmword ptr [rbp + 0x10]
140a4b4d1: 4881c4a0000000  add rsp, 0xa0
140a4b4d8: 5f  pop rdi
140a4b4d9: 5e  pop rsi
140a4b4da: 5d  pop rbp
140a4b4db: c3  ret
140a4b4dc: 4883c708  add rdi, 8
140a4b4e0: c6450701  mov byte ptr [rbp + 7], 1
140a4b4e4: 488d15d23f0802  lea rdx, [rip + 0x2083fd2]
140a4b4eb: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b4ef: 41b814000000  mov r8d, 0x14
140a4b4f5: 4989f9  mov r9, rdi
140a4b4f8: e823a5b400  call 0x141595a20
140a4b4fd: 4885c0  test rax, rax
140a4b500: 0f8563ffffff  jne 0x140a4b469
140a4b506: eb9a  jmp 0x140a4b4a2
140a4b508: 0f1f840000000000  nop dword ptr [rax + rax]
140a4b510: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b515: 55  push rbp
140a4b516: 56  push rsi
140a4b517: 57  push rdi
140a4b518: 4883ec30  sub rsp, 0x30
140a4b51c: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b523: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
140a4b528: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
140a4b52c: 488d045500000000  lea rax, [rdx*2]
140a4b534: 4885c0  test rax, rax
140a4b537: 740f  je 0x140a4b548
140a4b539: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
140a4b53d: 41b801000000  mov r8d, 1
140a4b543: e8d886bcff  call 0x140613c20
140a4b548: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
140a4b54d: 4883c430  add rsp, 0x30
140a4b551: 5f  pop rdi
140a4b552: 5e  pop rsi
140a4b553: 5d  pop rbp
140a4b554: c3  ret
140a4b555: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a4b560: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b565: 55  push rbp
140a4b566: 56  push rsi
140a4b567: 57  push rdi
140a4b568: 4883ec30  sub rsp, 0x30
140a4b56c: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b573: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
140a4b578: 807d0700  cmp byte ptr [rbp + 7], 0
140a4b57c: 750d  jne 0x140a4b58b
140a4b57e: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
140a4b583: 4883c430  add rsp, 0x30
140a4b587: 5f  pop rdi
140a4b588: 5e  pop rsi
140a4b589: 5d  pop rbp
140a4b58a: c3  ret
140a4b58b: 488d4dd0  lea rcx, [rbp - 0x30]
140a4b58f: e8cc25e1ff  call 0x14085db60
140a4b594: ebe8  jmp 0x140a4b57e
