0x14157c5e0 55 push rbp
0x14157c5e1 56 push rsi
0x14157c5e2 57 push rdi
0x14157c5e3 4881ec90000000 sub rsp, 0x90
0x14157c5ea 488dac2480000000 lea rbp, [rsp + 0x80]
0x14157c5f2 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x14157c5fa 4889d7 mov rdi, rdx
0x14157c5fd 4889ce mov rsi, rcx
0x14157c600 48b80000000000000080 movabs rax, 0x8000000000000000
0x14157c60a 488945d0 mov qword ptr [rbp - 0x30], rax
0x14157c60e 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x14157c616 48c745f800000000 mov qword ptr [rbp - 8], 0
0x14157c61e c6450701 mov byte ptr [rbp + 7], 1
0x14157c622 488d1547ad6f01 lea rdx, [rip + 0x16fad47]
0x14157c629 488d4dd0 lea rcx, [rbp - 0x30]
0x14157c62d 41b80b000000 mov r8d, 0xb
0x14157c633 4989f9 mov r9, rdi
0x14157c636 e835a00100 call 0x141596670
0x14157c63b 4885c0 test rax, rax
0x14157c63e 756c jne 0x14157c6ac
0x14157c640 4c8d4f10 lea r9, [rdi + 0x10]
0x14157c644 c6450701 mov byte ptr [rbp + 7], 1
0x14157c648 488d152cad6f01 lea rdx, [rip + 0x16fad2c]
0x14157c64f 488d4dd0 lea rcx, [rbp - 0x30]
0x14157c653 41b805000000 mov r8d, 5
0x14157c659 e8c2930100 call 0x141595a20
0x14157c65e 4885c0 test rax, rax
0x14157c661 7549 jne 0x14157c6ac
0x14157c663 4c8d4f14 lea r9, [rdi + 0x14]
0x14157c667 c6450701 mov byte ptr [rbp + 7], 1
0x14157c66b 488d150ead6f01 lea rdx, [rip + 0x16fad0e]
0x14157c672 488d4dd0 lea rcx, [rbp - 0x30]
0x14157c676 41b806000000 mov r8d, 6
0x14157c67c e89f930100 call 0x141595a20
0x14157c681 4885c0 test rax, rax
0x14157c684 7526 jne 0x14157c6ac
0x14157c686 4883c708 add rdi, 8
0x14157c68a c6450701 mov byte ptr [rbp + 7], 1
0x14157c68e 488d15f1ac6f01 lea rdx, [rip + 0x16facf1]
0x14157c695 488d4dd0 lea rcx, [rbp - 0x30]
0x14157c699 41b806000000 mov r8d, 6
0x14157c69f 4989f9 mov r9, rdi
0x14157c6a2 e8c99f0100 call 0x141596670
0x14157c6a7 4885c0 test rax, rax
0x14157c6aa 743b je 0x14157c6e7
0x14157c6ac 488d4de8 lea rcx, [rbp - 0x18]
0x14157c6b0 48894608 mov qword ptr [rsi + 8], rax
0x14157c6b4 c60606 mov byte ptr [rsi], 6
0x14157c6b7 e884c831ff call 0x140898f40
0x14157c6bc 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x14157c6c0 4889d0 mov rax, rdx
0x14157c6c3 48f7d8 neg rax
0x14157c6c6 7011 jo 0x14157c6d9
0x14157c6c8 730f jae 0x14157c6d9
0x14157c6ca 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x14157c6ce 41b801000000 mov r8d, 1
0x14157c6d4 e8477509ff call 0x140613c20
0x14157c6d9 4889f0 mov rax, rsi
0x14157c6dc 4881c490000000 add rsp, 0x90
0x14157c6e3 5f pop rdi
0x14157c6e4 5e pop rsi
0x14157c6e5 5d pop rbp
0x14157c6e6 c3 ret 
0x14157c6e7 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x14157c6eb 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x14157c6ef 0f1055f0 movups xmm2, xmmword ptr [rbp - 0x10]
0x14157c6f3 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x14157c6f7 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x14157c6fb 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x14157c6ff c6450700 mov byte ptr [rbp + 7], 0
0x14157c703 488d55a0 lea rdx, [rbp - 0x60]
0x14157c707 4889f1 mov rcx, rsi
0x14157c70a e8b19d60ff call 0x140b864c0
0x14157c70f ebc8 jmp 0x14157c6d9
0x14157c711 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14157c720 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14157c725 55 push rbp
0x14157c726 56 push rsi
0x14157c727 57 push rdi
0x14157c728 4883ec20 sub rsp, 0x20
0x14157c72c 488daa80000000 lea rbp, [rdx + 0x80]
0x14157c733 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x14157c737 488d045500000000 lea rax, [rdx*2]
0x14157c73f 4885c0 test rax, rax
0x14157c742 740f je 0x14157c753
0x14157c744 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x14157c748 41b801000000 mov r8d, 1
0x14157c74e e8cd7409ff call 0x140613c20
0x14157c753 90 nop 
0x14157c754 4883c420 add rsp, 0x20
0x14157c758 5f pop rdi
0x14157c759 5e pop rsi
0x14157c75a 5d pop rbp
0x14157c75b c3 ret 
0x14157c75c 0f1f4000 nop dword ptr [rax]
0x14157c760 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14157c765 55 push rbp
0x14157c766 56 push rsi
0x14157c767 57 push rdi
0x14157c768 4883ec20 sub rsp, 0x20
0x14157c76c 488daa80000000 lea rbp, [rdx + 0x80]
0x14157c773 807d0700 cmp byte ptr [rbp + 7], 0
0x14157c777 7508 jne 0x14157c781
0x14157c779 4883c420 add rsp, 0x20
0x14157c77d 5f pop rdi
0x14157c77e 5e pop rsi
0x14157c77f 5d pop rbp
0x14157c780 c3 ret 
0x14157c781 488d4dd0 lea rcx, [rbp - 0x30]
0x14157c785 e8d6132eff call 0x14085db60
0x14157c78a ebed jmp 0x14157c779
