0x140fb8340 push r15
0x140fb8342 push r14
0x140fb8344 push r13
0x140fb8346 push r12
0x140fb8348 push rsi
0x140fb8349 push rdi
0x140fb834a push rbp
0x140fb834b push rbx
0x140fb834c sub rsp, 0xd8
0x140fb8353 mov rdi, rdx
0x140fb8356 mov rsi, rcx
0x140fb8359 mov qword ptr [rsp + 0x28], r8
0x140fb835e mov qword ptr [rsp + 0x30], r9
0x140fb8363 mov r13, qword ptr [rdx + 0x18]
0x140fb8367 lea rbx, [rsp + 0x28]
0x140fb836c mov rcx, rbx
0x140fb836f call 0x1405bdda0
0x140fb8374 mov qword ptr [rsp + 0x38], rax
0x140fb8379 movabs r15, 0x41223169ff28813b
0x140fb8383 mov qword ptr [rsp + 0x40], r15
0x140fb8388 movabs r12, 0xa79b7268a2a968d9
0x140fb8392 mov qword ptr [rsp + 0x48], r12
0x140fb8397 lea rcx, [rsp + 0x38]
0x140fb839c call 0x1405bdf20
0x140fb83a1 mov qword ptr [rsp + 0x38], r13
0x140fb83a6 mov qword ptr [rsp + 0x40], rax
0x140fb83ab lea rcx, [rsp + 0x38]
0x140fb83b0 call 0x1405bd910
0x140fb83b5 mov qword ptr [rsp + 0x38], rax
0x140fb83ba mov rbp, qword ptr [rdi + 0x10]
0x140fb83be cmp qword ptr [rbp + 0x1b8], 0
0x140fb83c6 je 0x140fb847e
0x140fb83cc mov r14, rax
0x140fb83cf lea rcx, [rbp + 0x1c0]
0x140fb83d6 lea rdx, [rsp + 0x38]
0x140fb83db call 0x1405b0d70
0x140fb83e0 mov rcx, qword ptr [rbp + 0x1a0]
0x140fb83e7 mov rdx, qword ptr [rbp + 0x1a8]
0x140fb83ee mov r8, rdx
0x140fb83f1 and r8, rax
0x140fb83f4 shr rax, 0x39
0x140fb83f8 movd xmm0, eax
0x140fb83fc punpcklbw xmm0, xmm0
0x140fb8400 pshuflw xmm0, xmm0, 0
0x140fb8405 pshufd xmm0, xmm0, 0
0x140fb840a lea rax, [rcx - 0xc0]
0x140fb8411 xor r9d, r9d
0x140fb8414 pcmpeqd xmm1, xmm1
0x140fb8418 movdqu xmm2, xmmword ptr [rcx + r8]
0x140fb841e movdqa xmm3, xmm2
0x140fb8422 pcmpeqb xmm3, xmm0
0x140fb8426 pmovmskb r10d, xmm3
0x140fb842b test r10d, r10d
0x140fb842e je 0x140fb8460
0x140fb8430 tzcnt r11d, r10d
0x140fb8435 add r11, r8
0x140fb8438 and r11, rdx
0x140fb843b neg r11
0x140fb843e lea r11, [r11 + r11*2]
0x140fb8442 shl r11, 6
0x140fb8446 cmp qword ptr [rax + r11], r14
0x140fb844a je 0x140fb85d4
0x140fb8450 lea r11d, [r10 - 1]
0x140fb8454 and r11w, r10w
0x140fb8458 mov r10d, r11d
0x140fb845b jne 0x140fb8430
0x140fb845d nop dword ptr [rax]
0x140fb8460 pcmpeqb xmm2, xmm1
0x140fb8464 pmovmskb r10d, xmm2
0x140fb8469 test r10d, r10d
0x140fb846c jne 0x140fb847e
0x140fb846e add r8, r9
0x140fb8471 add r8, 0x10
0x140fb8475 add r9, 0x10
0x140fb8479 and r8, rdx
0x140fb847c jmp 0x140fb8418
0x140fb847e lea r14, [rdi + 0x10]
0x140fb8482 mov rax, qword ptr [rip + 0x2673f1f] ; 0x14362c3a8 2093634301000000b0c3624301000000 f32=227.57470703125
0x140fb8489 mov rax, qword ptr [rax]
0x140fb848c test rax, rax
0x140fb848f je 0x140fb857b
0x140fb8495 mov qword ptr [rsp + 0xb8], rdi
0x140fb849d lea rax, [rip - 0xa84c04] ; 0x1405338a0 4989d0488b01488b51084889c1e9ae4a f32=427082.28125
0x140fb84a4 mov qword ptr [rsp + 0xc0], rax
0x140fb84ac mov qword ptr [rsp + 0xc8], rbx
0x140fb84b4 mov qword ptr [rsp + 0xd0], rax
0x140fb84bc lea rcx, [rip + 0x1bfb9b5] ; 0x142bb3e78 813dbb42010000002800000000000000 f32=93.62012481689453
0x140fb84c3 call 0x14067dd80
0x140fb84c8 movdqu xmm0, xmmword ptr [rax]
0x140fb84cc mov eax, dword ptr [rax + 0x10]
0x140fb84cf mov qword ptr [rsp + 0x68], 1
0x140fb84d8 lea rcx, [rip + 0x1bfb931] ; 0x142bb3e10 746f6f6c6261723a3a6d65693a3a636f f32=1.1578383735592298e+27
0x140fb84df mov qword ptr [rsp + 0x70], rcx
0x140fb84e4 mov qword ptr [rsp + 0x78], 0x14
0x140fb84ed lea rdx, [rip + 0x1bfb94c] ; 0x142bb3e40 243ebb42010000000c00000000000000 f32=93.62136840820312
0x140fb84f4 mov qword ptr [rsp + 0x88], rdx
0x140fb84fc mov qword ptr [rsp + 0x90], 2
0x140fb8508 lea rdx, [rsp + 0xb8]
0x140fb8510 mov qword ptr [rsp + 0x98], rdx
0x140fb8518 mov qword ptr [rsp + 0xa0], 2
0x140fb8524 mov qword ptr [rsp + 0xa8], 0
0x140fb8530 mov qword ptr [rsp + 0x38], 0
0x140fb8539 mov qword ptr [rsp + 0x40], rcx
0x140fb853e mov qword ptr [rsp + 0x48], 0x14
0x140fb8547 mov qword ptr [rsp + 0x50], 0
0x140fb8550 movdqu xmmword ptr [rsp + 0x58], xmm0
0x140fb8556 mov dword ptr [rsp + 0x80], 1
0x140fb8561 mov dword ptr [rsp + 0x84], eax
0x140fb8568 lea rcx, [rsp + 0x27]
0x140fb856d lea rdx, [rsp + 0x38]
0x140fb8572 call 0x140d4a330
0x140fb8577 mov r13, qword ptr [rdi + 0x18]
0x140fb857b movdqu xmm0, xmmword ptr [rsp + 0x28]
0x140fb8581 movdqu xmmword ptr [rsp + 0xb8], xmm0
0x140fb858a lea rcx, [rsp + 0xb8]
0x140fb8592 call 0x1405bdda0
0x140fb8597 mov qword ptr [rsp + 0x38], rax
0x140fb859c mov qword ptr [rsp + 0x40], r15
0x140fb85a1 mov qword ptr [rsp + 0x48], r12
0x140fb85a6 lea rcx, [rsp + 0x38]
0x140fb85ab call 0x1405bdf20
0x140fb85b0 mov qword ptr [rsp + 0x38], r13
0x140fb85b5 mov qword ptr [rsp + 0x40], rax
0x140fb85ba lea rcx, [rsp + 0x38]
0x140fb85bf call 0x1405bd910
0x140fb85c4 mov rcx, rsi
0x140fb85c7 mov rdx, r14
0x140fb85ca mov r8, rax
0x140fb85cd call 0x1406861b0
0x140fb85d2 jmp 0x140fb85e3
0x140fb85d4 mov qword ptr [rsi], rbp
0x140fb85d7 mov qword ptr [rsi + 8], r14
0x140fb85db mov qword ptr [rsi + 0x10], 0
0x140fb85e3 mov rax, rsi
0x140fb85e6 add rsp, 0xd8
0x140fb85ed pop rbx
0x140fb85ee pop rbp
0x140fb85ef pop rdi
0x140fb85f0 pop rsi
0x140fb85f1 pop r12
0x140fb85f3 pop r13
0x140fb85f5 pop r14
0x140fb85f7 pop r15
0x140fb85f9 ret 