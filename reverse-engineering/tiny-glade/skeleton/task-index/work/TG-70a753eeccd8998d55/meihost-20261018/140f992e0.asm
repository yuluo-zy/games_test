0x140f992e0 push r15
0x140f992e2 push r14
0x140f992e4 push rsi
0x140f992e5 push rdi
0x140f992e6 push rbp
0x140f992e7 push rbx
0x140f992e8 sub rsp, 0x78
0x140f992ec movaps xmmword ptr [rsp + 0x60], xmm9
0x140f992f2 movaps xmmword ptr [rsp + 0x50], xmm8
0x140f992f8 movaps xmmword ptr [rsp + 0x40], xmm7
0x140f992fd movaps xmmword ptr [rsp + 0x30], xmm6
0x140f99302 mov rdi, rdx
0x140f99305 mov rsi, rcx
0x140f99308 movzx ebp, byte ptr [rsp + 0xf8]
0x140f99310 mov qword ptr [rsp + 0x20], r8
0x140f99315 mov qword ptr [rsp + 0x28], r9
0x140f9931a lea rcx, [rsp + 0x20]
0x140f9931f call 0x1405bdda0
0x140f99324 movabs rbx, 0xb1695320004fb443
0x140f9932e xor rbx, rax
0x140f99331 mov rcx, rdi
0x140f99334 mov rdx, rbx
0x140f99337 call 0x140683ec0
0x140f9933c mov byte ptr [rax + 0x5d], 0
0x140f99340 test bpl, bpl
0x140f99343 je 0x140f9934c
0x140f99345 mov byte ptr [rax + 0xb7], 1
0x140f9934c test dl, 1
0x140f9934f jne 0x140f993e0
0x140f99355 movss xmm6, dword ptr [rsp + 0xf0]
0x140f9935e movss xmm7, dword ptr [rsp + 0xe8]
0x140f99367 mov r15, qword ptr [rsp + 0xe0]
0x140f9936f movss xmm8, dword ptr [rsp + 0xd8]
0x140f99379 movss xmm9, dword ptr [rsp + 0xd0]
0x140f99383 mov r14, qword ptr [rdi + 0x10]
0x140f99387 cmp r14, qword ptr [rdi]
0x140f9938a jne 0x140f9939b
0x140f9938c lea rdx, [rip + 0x1c15c35]
0x140f99393 mov rcx, rdi
0x140f99396 call 0x1406319d0
0x140f9939b mov rax, qword ptr [rdi + 8]
0x140f9939f imul rcx, r14, 0x38
0x140f993a3 mov dword ptr [rax + rcx], 4
0x140f993aa movss dword ptr [rax + rcx + 4], xmm9
0x140f993b1 movss dword ptr [rax + rcx + 8], xmm8
0x140f993b8 movss dword ptr [rax + rcx + 0xc], xmm7
0x140f993be movss dword ptr [rax + rcx + 0x10], xmm6
0x140f993c4 mov rdx, qword ptr [r15]
0x140f993c7 mov qword ptr [rax + rcx + 0x14], rdx
0x140f993cc mov edx, dword ptr [r15 + 8]
0x140f993d0 mov dword ptr [rax + rcx + 0x1c], edx
0x140f993d4 mov qword ptr [rax + rcx + 0x30], rbx
0x140f993d9 inc r14
0x140f993dc mov qword ptr [rdi + 0x10], r14
0x140f993e0 mov qword ptr [rsi], rdi
0x140f993e3 mov qword ptr [rsi + 8], rbx
0x140f993e7 mov qword ptr [rsi + 0x10], 0
0x140f993ef mov rax, rsi
0x140f993f2 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140f993f7 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140f993fc movaps xmm8, xmmword ptr [rsp + 0x50]
0x140f99402 movaps xmm9, xmmword ptr [rsp + 0x60]
0x140f99408 add rsp, 0x78
0x140f9940c pop rbx
0x140f9940d pop rbp
0x140f9940e pop rdi
0x140f9940f pop rsi
0x140f99410 pop r14
0x140f99412 pop r15
0x140f99414 ret 