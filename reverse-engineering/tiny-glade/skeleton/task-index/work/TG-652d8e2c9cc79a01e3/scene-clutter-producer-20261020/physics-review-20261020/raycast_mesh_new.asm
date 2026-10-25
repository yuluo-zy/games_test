0x1408ceb10 4889c8 mov rax, rcx
0x1408ceb13 48895118 mov qword ptr [rcx + 0x18], rdx
0x1408ceb17 410f1000 movups xmm0, xmmword ptr [r8]
0x1408ceb1b 0f1101 movups xmmword ptr [rcx], xmm0
0x1408ceb1e 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x1408ceb22 48894810 mov qword ptr [rax + 0x10], rcx
0x1408ceb26 c3 ret
