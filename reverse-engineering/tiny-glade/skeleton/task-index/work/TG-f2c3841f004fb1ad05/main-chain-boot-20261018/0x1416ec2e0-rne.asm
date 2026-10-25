0x1416ec2e0 push r14
0x1416ec2e2 push rsi
0x1416ec2e3 push rdi
0x1416ec2e4 push rbx
0x1416ec2e5 sub rsp, 0x58
0x1416ec2e9 movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ec2ee mov rsi, rdx
0x1416ec2f1 mov rdi, rcx
0x1416ec2f4 test byte ptr [rcx], 1
0x1416ec2f7 je 0x1416ec39b
0x1416ec2fd lea rdx, [rsi + 8]
0x1416ec301 mov rbx, qword ptr [rsi]
0x1416ec304 mov rax, qword ptr [rbx + 0x10]
0x1416ec308 cmp qword ptr [rbx], rax
0x1416ec30b je 0x1416ec4f5
0x1416ec311 mov rcx, qword ptr [rbx + 8]
0x1416ec315 mov byte ptr [rcx + rax], 0x7b
0x1416ec319 inc rax
0x1416ec31c mov qword ptr [rbx + 0x10], rax
0x1416ec320 lea r8, [rip + 0x15a390b]
0x1416ec327 mov r9d, 9
0x1416ec32d mov rcx, rsi
0x1416ec330 call 0x141580d20
0x1416ec335 test rax, rax
0x1416ec338 jne 0x1416ec4b6
0x1416ec33e add rdi, 4
0x1416ec342 mov rbx, qword ptr [rsi]
0x1416ec345 mov rdx, qword ptr [rbx + 0x10]
0x1416ec349 cmp qword ptr [rbx], rdx
0x1416ec34c je 0x1416ec553
0x1416ec352 mov rax, qword ptr [rbx + 8]
0x1416ec356 mov byte ptr [rax + rdx], 0x3a
0x1416ec35a inc rdx
0x1416ec35d mov qword ptr [rbx + 0x10], rdx
0x1416ec361 mov rcx, rdi
0x1416ec364 mov rdx, rsi
0x1416ec367 call 0x14157c4c0
0x1416ec36c test rax, rax
0x1416ec36f jne 0x1416ec469
0x1416ec375 mov rsi, qword ptr [rsi]
0x1416ec378 mov rdx, qword ptr [rsi + 0x10]
0x1416ec37c cmp qword ptr [rsi], rdx
0x1416ec37f je 0x1416ec5cb
0x1416ec385 mov rax, qword ptr [rsi + 8]
0x1416ec389 mov byte ptr [rax + rdx], 0x7d
0x1416ec38d inc rdx
0x1416ec390 mov qword ptr [rsi + 0x10], rdx
0x1416ec394 xor eax, eax
0x1416ec396 jmp 0x1416ec469
0x1416ec39b lea rdx, [rsi + 8]
0x1416ec39f mov rbx, qword ptr [rsi]
0x1416ec3a2 mov rax, qword ptr [rbx + 0x10]
0x1416ec3a6 cmp qword ptr [rbx], rax
0x1416ec3a9 je 0x1416ec524
0x1416ec3af mov rcx, qword ptr [rbx + 8]
0x1416ec3b3 mov byte ptr [rcx + rax], 0x7b
0x1416ec3b7 inc rax
0x1416ec3ba mov qword ptr [rbx + 0x10], rax
0x1416ec3be lea r8, [rip + 0x15a3867]
0x1416ec3c5 mov r9d, 6
0x1416ec3cb mov rcx, rsi
0x1416ec3ce call 0x141580d20
0x1416ec3d3 test rax, rax
0x1416ec3d6 jne 0x1416ec4b6
0x1416ec3dc mov rbx, qword ptr [rsi]
0x1416ec3df mov rdx, qword ptr [rbx + 0x10]
0x1416ec3e3 cmp qword ptr [rbx], rdx
0x1416ec3e6 je 0x1416ec579
0x1416ec3ec lea r9, [rdi + 4]
0x1416ec3f0 mov rax, qword ptr [rbx + 8]
0x1416ec3f4 mov byte ptr [rax + rdx], 0x3a
0x1416ec3f8 inc rdx
0x1416ec3fb mov qword ptr [rbx + 0x10], rdx
0x1416ec3ff movss xmm6, dword ptr [rdi + 0x10]
0x1416ec404 mov rbx, qword ptr [rsi]
0x1416ec407 mov rdx, qword ptr [rbx + 0x10]
0x1416ec40b cmp qword ptr [rbx], rdx
0x1416ec40e je 0x1416ec59f
0x1416ec414 mov rax, qword ptr [rbx + 8]
0x1416ec418 mov byte ptr [rax + rdx], 0x7b
0x1416ec41c inc rdx
0x1416ec41f mov qword ptr [rbx + 0x10], rdx
0x1416ec423 mov qword ptr [rsp + 0x30], rsi
0x1416ec428 mov byte ptr [rsp + 0x38], 1
0x1416ec42d lea rdx, [rip + 0x15a2e06]
0x1416ec434 lea rcx, [rsp + 0x30]
0x1416ec439 mov r8d, 6
0x1416ec43f call 0x14158ce20
0x1416ec444 test rax, rax
0x1416ec447 jne 0x1416ec469
0x1416ec449 lea r9, [rdi + 0xc]
0x1416ec44d lea rdx, [rip + 0x15a2dec]
0x1416ec454 lea rcx, [rsp + 0x30]
0x1416ec459 mov r8d, 6
0x1416ec45f call 0x141586090
0x1416ec464 test rax, rax
0x1416ec467 je 0x1416ec478
0x1416ec469 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ec46e add rsp, 0x58
0x1416ec472 pop rbx
0x1416ec473 pop rdi
0x1416ec474 pop rsi
0x1416ec475 pop r14
0x1416ec477 ret 
0x1416ec478 xorps xmm0, xmm0
0x1416ec47b ucomiss xmm6, xmm0
0x1416ec47e jne 0x1416ec4cc
0x1416ec480 jp 0x1416ec4cc
0x1416ec482 cmp byte ptr [rsp + 0x38], 0
0x1416ec487 je 0x1416ec375
0x1416ec48d mov rax, qword ptr [rsp + 0x30]
0x1416ec492 mov rdi, qword ptr [rax]
0x1416ec495 mov rdx, qword ptr [rdi + 0x10]
0x1416ec499 cmp qword ptr [rdi], rdx
0x1416ec49c je 0x1416ec5f1
0x1416ec4a2 mov rax, qword ptr [rdi + 8]
0x1416ec4a6 mov byte ptr [rax + rdx], 0x7d
0x1416ec4aa inc rdx
0x1416ec4ad mov qword ptr [rdi + 0x10], rdx
0x1416ec4b1 jmp 0x1416ec375
0x1416ec4b6 mov rcx, rax
0x1416ec4b9 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ec4be add rsp, 0x58
0x1416ec4c2 pop rbx
0x1416ec4c3 pop rdi
0x1416ec4c4 pop rsi
0x1416ec4c5 pop r14
0x1416ec4c7 jmp 0x1428c98a0
0x1416ec4cc add rdi, 0x10
0x1416ec4d0 lea rdx, [rip + 0x15a2d6f]
0x1416ec4d7 lea rcx, [rsp + 0x30]
0x1416ec4dc mov r8d, 8
0x1416ec4e2 mov r9, rdi
0x1416ec4e5 call 0x141585400
0x1416ec4ea test rax, rax
0x1416ec4ed jne 0x1416ec469
0x1416ec4f3 jmp 0x1416ec482
0x1416ec4f5 mov qword ptr [rsp + 0x20], 1
0x1416ec4fe mov r8d, 1
0x1416ec504 mov r9d, 1
0x1416ec50a mov rcx, rbx
0x1416ec50d mov r14, rdx
0x1416ec510 mov rdx, rax
0x1416ec513 call 0x1428a9a60
0x1416ec518 mov rdx, r14
0x1416ec51b mov rax, qword ptr [rbx + 0x10]
0x1416ec51f jmp 0x1416ec311
0x1416ec524 mov qword ptr [rsp + 0x20], 1
0x1416ec52d mov r8d, 1
0x1416ec533 mov r9d, 1
0x1416ec539 mov rcx, rbx
0x1416ec53c mov r14, rdx
0x1416ec53f mov rdx, rax
0x1416ec542 call 0x1428a9a60
0x1416ec547 mov rdx, r14
0x1416ec54a mov rax, qword ptr [rbx + 0x10]
0x1416ec54e jmp 0x1416ec3af
0x1416ec553 mov qword ptr [rsp + 0x20], 1
0x1416ec55c mov r8d, 1
0x1416ec562 mov r9d, 1
0x1416ec568 mov rcx, rbx
0x1416ec56b call 0x1428a9a60
0x1416ec570 mov rdx, qword ptr [rbx + 0x10]
0x1416ec574 jmp 0x1416ec352
0x1416ec579 mov qword ptr [rsp + 0x20], 1
0x1416ec582 mov r8d, 1
0x1416ec588 mov r9d, 1
0x1416ec58e mov rcx, rbx
0x1416ec591 call 0x1428a9a60
0x1416ec596 mov rdx, qword ptr [rbx + 0x10]
0x1416ec59a jmp 0x1416ec3ec
0x1416ec59f mov qword ptr [rsp + 0x20], 1
0x1416ec5a8 mov r8d, 1
0x1416ec5ae mov r14, r9
0x1416ec5b1 mov r9d, 1
0x1416ec5b7 mov rcx, rbx
0x1416ec5ba call 0x1428a9a60
0x1416ec5bf mov r9, r14
0x1416ec5c2 mov rdx, qword ptr [rbx + 0x10]
0x1416ec5c6 jmp 0x1416ec414
0x1416ec5cb mov qword ptr [rsp + 0x20], 1
0x1416ec5d4 mov r8d, 1
0x1416ec5da mov r9d, 1
0x1416ec5e0 mov rcx, rsi
0x1416ec5e3 call 0x1428a9a60
0x1416ec5e8 mov rdx, qword ptr [rsi + 0x10]
0x1416ec5ec jmp 0x1416ec385
0x1416ec5f1 mov qword ptr [rsp + 0x20], 1
0x1416ec5fa mov r8d, 1
0x1416ec600 mov r9d, 1
0x1416ec606 mov rcx, rdi
0x1416ec609 call 0x1428a9a60
0x1416ec60e mov rdx, qword ptr [rdi + 0x10]
0x1416ec612 jmp 0x1416ec4a2
