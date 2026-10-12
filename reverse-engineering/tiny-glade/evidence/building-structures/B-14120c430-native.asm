14120c430 push rbp
14120c431 push r15
14120c433 push r14
14120c435 push r12
14120c437 push rsi
14120c438 push rdi
14120c439 push rbx
14120c43a sub rsp, 0x30
14120c43e lea rbp, [rsp + 0x30]
14120c443 mov qword ptr [rbp - 8], 0xfffffffffffffffe
14120c44b mov rdi, rdx
14120c44e mov rsi, rcx
14120c451 mov ecx, dword ptr [rcx + 0x50]
14120c454 mov rdx, qword ptr [rsi + 8]
14120c458 cmp rdx, rcx
14120c45b jbe 0x14120c66b
14120c461 mov rdx, qword ptr [rsi]
14120c464 movzx r8d, byte ptr [rdx + rcx]
14120c469 cmp r8, 0x20
14120c46d jae 0x14120c4e2
14120c46f mov rax, rcx
14120c472 shl rax, 5
14120c476 or rax, r8
14120c479 inc r8d
14120c47c mov byte ptr [rdx + rcx], r8b
14120c480 mov rdx, qword ptr [rsi + 0x30]
14120c484 cmp rax, rdx
14120c487 jae 0x14120c677
14120c48d mov rdx, qword ptr [rsi + 0x28]
14120c491 shl rax, 6
14120c495 movups xmm0, xmmword ptr [rdi]
14120c498 movups xmm1, xmmword ptr [rdi + 0x10]
14120c49c movups xmm2, xmmword ptr [rdi + 0x20]
14120c4a0 movups xmm3, xmmword ptr [rdi + 0x30]
14120c4a4 movups xmmword ptr [rdx + rax + 0x30], xmm3
14120c4a9 movups xmmword ptr [rdx + rax + 0x20], xmm2
14120c4ae movups xmmword ptr [rdx + rax + 0x10], xmm1
14120c4b3 movups xmmword ptr [rdx + rax], xmm0
14120c4b7 mov rdx, qword ptr [rsi + 0x48]
14120c4bb mov eax, ecx
14120c4bd shr eax, 6
14120c4c0 cmp rax, rdx
14120c4c3 jae 0x14120c686
14120c4c9 mov rdx, qword ptr [rsi + 0x40]
14120c4cd mov r8d, 1
14120c4d3 shl r8, cl
14120c4d6 lock or qword ptr [rdx + rax*8], r8
14120c4db xor esi, esi
14120c4dd jmp 0x14120c63b
14120c4e2 mov r14, qword ptr [rsi + 0x38]
14120c4e6 mov ecx, 8
14120c4eb xor eax, eax
14120c4ed lock cmpxchg qword ptr [r14], rcx
14120c4f2 jne 0x14120c64c
14120c4f8 mov rcx, r14
14120c4fb call 0x140c81c70
14120c500 mov rbx, r14
14120c503 or rbx, 1
14120c507 mov rcx, rbx
14120c50a call 0x140c81c70
14120c50f mov rax, qword ptr [r14 + 0x18]
14120c513 test rax, rax
14120c516 je 0x14120c61a
14120c51c lea rcx, [rax - 1]
14120c520 mov qword ptr [r14 + 0x18], rcx
14120c524 mov qword ptr [rbp - 0x10], r14
14120c528 mov rcx, qword ptr [r14 + 0x10]
14120c52c mov r14d, dword ptr [rcx + rax*4 - 4]
14120c531 mov r15, qword ptr [rsi + 0x20]
14120c535 mov r12, qword ptr [r15 + 0x10]
14120c539 cmp r12, qword ptr [r15]
14120c53c jne 0x14120c54d
14120c53e lea rdx, [rip + 0x1a0e66b]
14120c545 mov rcx, r15
14120c548 call 0x140b5f870
14120c54d mov rax, qword ptr [r15 + 8]
14120c551 mov dword ptr [rax + r12*4], r14d
14120c555 inc r12
14120c558 mov qword ptr [r15 + 0x10], r12
14120c55c mov rdx, qword ptr [rsi + 8]
14120c560 cmp rdx, r14
14120c563 jbe 0x14120c695
14120c569 mov rcx, r14
14120c56c shl rcx, 5
14120c570 mov rax, qword ptr [rsi]
14120c573 mov byte ptr [rax + r14], 1
14120c578 mov rdx, qword ptr [rsi + 0x30]
14120c57c cmp rcx, rdx
14120c57f jae 0x14120c6a6
14120c585 mov rax, qword ptr [rsi + 0x28]
14120c589 shl rcx, 6
14120c58d movups xmm0, xmmword ptr [rdi]
14120c590 movups xmm1, xmmword ptr [rdi + 0x10]
14120c594 movups xmm2, xmmword ptr [rdi + 0x20]
14120c598 movups xmm3, xmmword ptr [rdi + 0x30]
14120c59c movups xmmword ptr [rax + rcx + 0x30], xmm3
14120c5a1 movups xmmword ptr [rax + rcx + 0x20], xmm2
14120c5a6 movups xmmword ptr [rax + rcx + 0x10], xmm1
14120c5ab movups xmmword ptr [rax + rcx], xmm0
14120c5af mov rdx, qword ptr [rsi + 0x18]
14120c5b3 mov eax, r14d
14120c5b6 shr eax, 6
14120c5b9 cmp rax, rdx
14120c5bc jae 0x14120c6b4
14120c5c2 mov r8d, 1
14120c5c8 mov ecx, r14d
14120c5cb shl r8, cl
14120c5ce mov rcx, qword ptr [rsi + 0x10]
14120c5d2 lock or qword ptr [rcx + rax*8], r8
14120c5d7 mov rdx, qword ptr [rsi + 0x48]
14120c5db cmp rax, rdx
14120c5de jae 0x14120c6c5
14120c5e4 mov rcx, qword ptr [rsi + 0x40]
14120c5e8 lock or qword ptr [rcx + rax*8], r8
14120c5ed mov dword ptr [rsi + 0x50], r14d
14120c5f1 mov rdi, qword ptr [rbp - 0x10]
14120c5f5 mov rcx, rdi
14120c5f8 call 0x140c81e10
14120c5fd mov rcx, rbx
14120c600 call 0x140c81e10
14120c605 xor esi, esi
14120c607 mov eax, 8
14120c60c lock cmpxchg qword ptr [rdi], rsi
14120c611 je 0x14120c63b
14120c613 xor esi, esi
14120c615 mov rcx, rdi
14120c618 jmp 0x14120c662
14120c61a mov rcx, r14
14120c61d call 0x140c81e10
14120c622 mov rcx, rbx
14120c625 call 0x140c81e10
14120c62a xor ecx, ecx
14120c62c mov eax, 8
14120c631 lock cmpxchg qword ptr [r14], rcx
14120c636 mov sil, 1
14120c639 jne 0x14120c65f
14120c63b mov eax, esi
14120c63d add rsp, 0x30
14120c641 pop rbx
14120c642 pop rdi
14120c643 pop rsi
14120c644 pop r12
14120c646 pop r14
14120c648 pop r15
14120c64a pop rbp
14120c64b ret 
14120c64c mov rcx, r14
14120c64f mov r8d, 0x3b9aca00
14120c655 call 0x1428cb5f0
14120c65a jmp 0x14120c4f8
14120c65f mov rcx, r14
14120c662 xor edx, edx
14120c664 call 0x1428cbf60
14120c669 jmp 0x14120c63b
14120c66b lea r8, [rip + 0x1a0e526]
14120c672 call 0x1428d9518
14120c677 lea r8, [rip + 0x1a0e57a]
14120c67e mov rcx, rax
14120c681 call 0x1428d9518
14120c686 lea r8, [rip + 0x1a0e3eb]
14120c68d mov rcx, rax
14120c690 call 0x1428d9518
14120c695 lea r8, [rip + 0x1a0e52c]
14120c69c mov rcx, r14
14120c69f call 0x1428d9518
14120c6a4 jmp 0x14120c6d4
14120c6a6 lea r8, [rip + 0x1a0e533]
14120c6ad call 0x1428d9518
14120c6b2 jmp 0x14120c6d4
14120c6b4 lea r8, [rip + 0x1a0e3bd]
14120c6bb mov rcx, rax
14120c6be call 0x1428d9518
14120c6c3 jmp 0x14120c6d4
14120c6c5 lea r8, [rip + 0x1a0e3ac]
14120c6cc mov rcx, rax
14120c6cf call 0x1428d9518
14120c6d4 ud2 
