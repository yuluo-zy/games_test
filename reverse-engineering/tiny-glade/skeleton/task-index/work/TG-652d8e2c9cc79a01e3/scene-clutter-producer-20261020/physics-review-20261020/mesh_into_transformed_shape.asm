0x140b26d20 56 push rsi
0x140b26d21 57 push rdi
0x140b26d22 4883ec48 sub rsp, 0x48
0x140b26d26 4889ce mov rsi, rcx
0x140b26d29 488b7a20 mov rdi, qword ptr [rdx + 0x20]
0x140b26d2d 4c8d4750 lea r8, [rdi + 0x50]
0x140b26d31 488d4c2420 lea rcx, [rsp + 0x20]
0x140b26d36 e845ae1600 call 0x140c91b80
0x140b26d3b 0f28442420 movaps xmm0, xmmword ptr [rsp + 0x20]
0x140b26d40 0f284c2430 movaps xmm1, xmmword ptr [rsp + 0x30]
0x140b26d45 488b4720 mov rax, qword ptr [rdi + 0x20]
0x140b26d49 488b4f28 mov rcx, qword ptr [rdi + 0x28]
0x140b26d4d f048ff00 lock inc qword ptr [rax]
0x140b26d51 7e21 jle 0x140b26d74
0x140b26d53 0f110e movups xmmword ptr [rsi], xmm1
0x140b26d56 0f134610 movlps qword ptr [rsi + 0x10], xmm0
0x140b26d5a 0f12c0 movhlps xmm0, xmm0
0x140b26d5d f30f114618 movss dword ptr [rsi + 0x18], xmm0
0x140b26d62 48894620 mov qword ptr [rsi + 0x20], rax
0x140b26d66 48894e28 mov qword ptr [rsi + 0x28], rcx
0x140b26d6a 4889f0 mov rax, rsi
0x140b26d6d 4883c448 add rsp, 0x48
0x140b26d71 5f pop rdi
0x140b26d72 5e pop rsi
0x140b26d73 c3 ret 
0x140b26d74 0f0b ud2 
