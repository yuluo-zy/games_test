0x140ca67a0 56 push rsi
0x140ca67a1 4883ec50 sub rsp, 0x50
0x140ca67a5 4883b9f805000000 cmp qword ptr [rcx + 0x5f8], 0
0x140ca67ad 7528 jne 0x140ca67d7
0x140ca67af 488b9100060000 mov rdx, qword ptr [rcx + 0x600]
0x140ca67b6 4885d2 test rdx, rdx
0x140ca67b9 7412 je 0x140ca67cd
0x140ca67bb 4c8b8108060000 mov r8, qword ptr [rcx + 0x608]
0x140ca67c2 4889ce mov rsi, rcx
0x140ca67c5 e89664cb00 call 0x14195cc60
0x140ca67ca 4889f1 mov rcx, rsi
0x140ca67cd 4883c450 add rsp, 0x50
0x140ca67d1 5e pop rsi
0x140ca67d2 e9b90b0500 jmp 0x140cf7390
0x140ca67d7 488d05f294e801 lea rax, [rip + 0x1e894f2]
0x140ca67de 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ca67e3 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140ca67ec 48c744243008000000 mov qword ptr [rsp + 0x30], 8
0x140ca67f5 0f57c0 xorps xmm0, xmm0
0x140ca67f8 0f11442438 movups xmmword ptr [rsp + 0x38], xmm0
0x140ca67fd 488d15dc94e801 lea rdx, [rip + 0x1e894dc]
0x140ca6804 488d4c2420 lea rcx, [rsp + 0x20]
0x140ca6809 e8822bc301 call 0x1428d9390
0x140ca680e cc int3 
