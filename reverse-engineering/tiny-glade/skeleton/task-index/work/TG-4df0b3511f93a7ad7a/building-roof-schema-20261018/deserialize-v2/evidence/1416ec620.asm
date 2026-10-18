0x1416ec620 55 push rbp
0x1416ec621 4157 push r15
0x1416ec623 4156 push r14
0x1416ec625 56 push rsi
0x1416ec626 57 push rdi
0x1416ec627 53 push rbx
0x1416ec628 4881ec18010000 sub rsp, 0x118
0x1416ec62f 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416ec637 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
0x1416ec63e 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
0x1416ec642 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
0x1416ec64a 4889ce mov rsi, rcx
0x1416ec64d 0fb602 movzx eax, byte ptr [rdx]
0x1416ec650 488d0d39385a01 lea rcx, [rip + 0x15a3839]
0x1416ec657 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x1416ec65b 4801c8 add rax, rcx
0x1416ec65e ffe0 jmp rax
0x1416ec660 b012 mov al, 0x12
0x1416ec662 e9a3000000 jmp 0x1416ec70a
0x1416ec667 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1416ec66b 488945e0 mov qword ptr [rbp - 0x20], rax
0x1416ec66f 0f104208 movups xmm0, xmmword ptr [rdx + 8]
0x1416ec673 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x1416ec677 488d4d30 lea rcx, [rbp + 0x30]
0x1416ec67b 488d55d0 lea rdx, [rbp - 0x30]
0x1416ec67f e8bc8aecff call 0x1415b5140
0x1416ec684 0fb64530 movzx eax, byte ptr [rbp + 0x30]
0x1416ec688 3c16 cmp al, 0x16
0x1416ec68a 7475 je 0x1416ec701
0x1416ec68c eb7c jmp 0x1416ec70a
0x1416ec68e 8b4a08 mov ecx, dword ptr [rdx + 8]
0x1416ec691 488b4210 mov rax, qword ptr [rdx + 0x10]
0x1416ec695 48894538 mov qword ptr [rbp + 0x38], rax
0x1416ec699 c1e103 shl ecx, 3
0x1416ec69c b804080a00 mov eax, 0xa0804
0x1416ec6a1 d3e8 shr eax, cl
0x1416ec6a3 eb65 jmp 0x1416ec70a
0x1416ec6a5 0f104208 movups xmm0, xmmword ptr [rdx + 8]
0x1416ec6a9 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1416ec6ad 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1416ec6b1 48894508 mov qword ptr [rbp + 8], rax
0x1416ec6b5 0f1045f1 movups xmm0, xmmword ptr [rbp - 0xf]
0x1416ec6b9 0f114531 movups xmmword ptr [rbp + 0x31], xmm0
0x1416ec6bd 488b4500 mov rax, qword ptr [rbp]
0x1416ec6c1 48894540 mov qword ptr [rbp + 0x40], rax
0x1416ec6c5 488b4508 mov rax, qword ptr [rbp + 8]
0x1416ec6c9 48894548 mov qword ptr [rbp + 0x48], rax
0x1416ec6cd b00c mov al, 0xc
0x1416ec6cf eb39 jmp 0x1416ec70a
0x1416ec6d1 0fb64201 movzx eax, byte ptr [rdx + 1]
0x1416ec6d5 884531 mov byte ptr [rbp + 0x31], al
0x1416ec6d8 31c0 xor eax, eax
0x1416ec6da eb2e jmp 0x1416ec70a
0x1416ec6dc 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1416ec6e0 488945c0 mov qword ptr [rbp - 0x40], rax
0x1416ec6e4 0f104208 movups xmm0, xmmword ptr [rdx + 8]
0x1416ec6e8 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
0x1416ec6ec 488d4d30 lea rcx, [rbp + 0x30]
0x1416ec6f0 488d55b0 lea rdx, [rbp - 0x50]
0x1416ec6f4 e8b7deecff call 0x1415ba5b0
0x1416ec6f9 0fb64530 movzx eax, byte ptr [rbp + 0x30]
0x1416ec6fd 3c16 cmp al, 0x16
0x1416ec6ff 7509 jne 0x1416ec70a
0x1416ec701 488b7d38 mov rdi, qword ptr [rbp + 0x38]
0x1416ec705 e9d7010000 jmp 0x1416ec8e1
0x1416ec70a 8b4d31 mov ecx, dword ptr [rbp + 0x31]
0x1416ec70d 8b5534 mov edx, dword ptr [rbp + 0x34]
0x1416ec710 895514 mov dword ptr [rbp + 0x14], edx
0x1416ec713 894d11 mov dword ptr [rbp + 0x11], ecx
0x1416ec716 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1416ec71a 0f104540 movups xmm0, xmmword ptr [rbp + 0x40]
0x1416ec71e 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1416ec722 884510 mov byte ptr [rbp + 0x10], al
0x1416ec725 48894d18 mov qword ptr [rbp + 0x18], rcx
0x1416ec729 488d3d68355a01 lea rdi, [rip + 0x15a3568]
0x1416ec730 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1416ec735 48c744242802000000 mov qword ptr [rsp + 0x28], 2
0x1416ec73e 4c8d053b355a01 lea r8, [rip + 0x15a353b]
0x1416ec745 488d4d30 lea rcx, [rbp + 0x30]
0x1416ec749 488d5510 lea rdx, [rbp + 0x10]
0x1416ec74d 41b911000000 mov r9d, 0x11
0x1416ec753 e80867b9ff call 0x141282e60
0x1416ec758 837d3001 cmp dword ptr [rbp + 0x30], 1
0x1416ec75c 7538 jne 0x1416ec796
0x1416ec75e 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1416ec762 488b01 mov rax, qword ptr [rcx]
0x1416ec765 4883f801 cmp rax, 1
0x1416ec769 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416ec76d 7473 je 0x1416ec7e2
0x1416ec76f 4885c0 test rax, rax
0x1416ec772 7577 jne 0x1416ec7eb
0x1416ec774 488b4560 mov rax, qword ptr [rbp + 0x60]
0x1416ec778 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1416ec77c 4885d2 test rdx, rdx
0x1416ec77f 746a je 0x1416ec7eb
0x1416ec781 488b4560 mov rax, qword ptr [rbp + 0x60]
0x1416ec785 488b4808 mov rcx, qword ptr [rax + 8]
0x1416ec789 41b801000000 mov r8d, 1
0x1416ec78f e88c74f2fe call 0x140613c20
0x1416ec794 eb55 jmp 0x1416ec7eb
0x1416ec796 448b7534 mov r14d, dword ptr [rbp + 0x34]
0x1416ec79a 488b5d38 mov rbx, qword ptr [rbp + 0x38]
0x1416ec79e f20f107540 movsd xmm6, qword ptr [rbp + 0x40]
0x1416ec7a3 4c8b7d48 mov r15, qword ptr [rbp + 0x48]
0x1416ec7a7 488d4d10 lea rcx, [rbp + 0x10]
0x1416ec7ab e810abffff call 0x1416e72c0
0x1416ec7b0 488d7e04 lea rdi, [rsi + 4]
0x1416ec7b4 4183fe02 cmp r14d, 2
0x1416ec7b8 7509 jne 0x1416ec7c3
0x1416ec7ba f30f16fe movshdup xmm7, xmm6
0x1416ec7be e9bd000000 jmp 0x1416ec880
0x1416ec7c3 48895e08 mov qword ptr [rsi + 8], rbx
0x1416ec7c7 0f137610 movlps qword ptr [rsi + 0x10], xmm6
0x1416ec7cb 4585f6 test r14d, r14d
0x1416ec7ce 0f84cb000000 je 0x1416ec89f
0x1416ec7d4 4c897e18 mov qword ptr [rsi + 0x18], r15
0x1416ec7d8 b801000000 mov eax, 1
0x1416ec7dd e9bf000000 jmp 0x1416ec8a1
0x1416ec7e2 4883c108 add rcx, 8
0x1416ec7e6 e815b2effe call 0x1405e7a00
0x1416ec7eb ba28000000 mov edx, 0x28
0x1416ec7f0 41b808000000 mov r8d, 8
0x1416ec7f6 488b4d60 mov rcx, qword ptr [rbp + 0x60]
0x1416ec7fa e82174f2fe call 0x140613c20
0x1416ec7ff 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1416ec804 48c744242802000000 mov qword ptr [rsp + 0x28], 2
0x1416ec80d 4c8d05a4345a01 lea r8, [rip + 0x15a34a4]
0x1416ec814 488d4d30 lea rcx, [rbp + 0x30]
0x1416ec818 488d5510 lea rdx, [rbp + 0x10]
0x1416ec81c 41b911000000 mov r9d, 0x11
0x1416ec822 e8d967b9ff call 0x141283000
0x1416ec827 837d3002 cmp dword ptr [rbp + 0x30], 2
0x1416ec82b 7538 jne 0x1416ec865
0x1416ec82d 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1416ec831 488b01 mov rax, qword ptr [rcx]
0x1416ec834 4883f801 cmp rax, 1
0x1416ec838 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416ec83c 7469 je 0x1416ec8a7
0x1416ec83e 4885c0 test rax, rax
0x1416ec841 756d jne 0x1416ec8b0
0x1416ec843 488b4560 mov rax, qword ptr [rbp + 0x60]
0x1416ec847 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1416ec84b 4885d2 test rdx, rdx
0x1416ec84e 7460 je 0x1416ec8b0
0x1416ec850 488b4560 mov rax, qword ptr [rbp + 0x60]
0x1416ec854 488b4808 mov rcx, qword ptr [rax + 8]
0x1416ec858 41b801000000 mov r8d, 1
0x1416ec85e e8bd73f2fe call 0x140613c20
0x1416ec863 eb4b jmp 0x1416ec8b0
0x1416ec865 488b5d30 mov rbx, qword ptr [rbp + 0x30]
0x1416ec869 f30f107538 movss xmm6, dword ptr [rbp + 0x38]
0x1416ec86e f30f107d3c movss xmm7, dword ptr [rbp + 0x3c]
0x1416ec873 488d4d10 lea rcx, [rbp + 0x10]
0x1416ec877 e844aaffff call 0x1416e72c0
0x1416ec87c 488d7e04 lea rdi, [rsi + 4]
0x1416ec880 85db test ebx, ebx
0x1416ec882 0f8582000000 jne 0x1416ec90a
0x1416ec888 48c1eb20 shr rbx, 0x20
0x1416ec88c 660f6ecb movd xmm1, ebx
0x1416ec890 488d4e08 lea rcx, [rsi + 8]
0x1416ec894 0f28d6 movaps xmm2, xmm6
0x1416ec897 0f28df movaps xmm3, xmm7
0x1416ec89a e8a1ac5aff call 0x140c97540
0x1416ec89f 31c0 xor eax, eax
0x1416ec8a1 8907 mov dword ptr [rdi], eax
0x1416ec8a3 31c0 xor eax, eax
0x1416ec8a5 eb43 jmp 0x1416ec8ea
0x1416ec8a7 4883c108 add rcx, 8
0x1416ec8ab e850b1effe call 0x1405e7a00
0x1416ec8b0 ba28000000 mov edx, 0x28
0x1416ec8b5 41b808000000 mov r8d, 8
0x1416ec8bb 488b4d60 mov rcx, qword ptr [rbp + 0x60]
0x1416ec8bf e85c73f2fe call 0x140613c20
0x1416ec8c4 488d0d35345a01 lea rcx, [rip + 0x15a3435]
0x1416ec8cb ba49000000 mov edx, 0x49
0x1416ec8d0 e82bd72001 call 0x1428fa000
0x1416ec8d5 4889c7 mov rdi, rax
0x1416ec8d8 488d4d10 lea rcx, [rbp + 0x10]
0x1416ec8dc e8dfa9ffff call 0x1416e72c0
0x1416ec8e1 48897e08 mov qword ptr [rsi + 8], rdi
0x1416ec8e5 b801000000 mov eax, 1
0x1416ec8ea 8906 mov dword ptr [rsi], eax
0x1416ec8ec 4889f0 mov rax, rsi
0x1416ec8ef 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
0x1416ec8f3 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
0x1416ec8fa 4881c418010000 add rsp, 0x118
0x1416ec901 5b pop rbx
0x1416ec902 5f pop rdi
0x1416ec903 5e pop rsi
0x1416ec904 415e pop r14
0x1416ec906 415f pop r15
0x1416ec908 5d pop rbp
0x1416ec909 c3 ret 
0x1416ec90a 488d059fd05201 lea rax, [rip + 0x152d09f]
0x1416ec911 48894530 mov qword ptr [rbp + 0x30], rax
0x1416ec915 48c7453801000000 mov qword ptr [rbp + 0x38], 1
0x1416ec91d 48c7454008000000 mov qword ptr [rbp + 0x40], 8
0x1416ec925 0f57c0 xorps xmm0, xmm0
0x1416ec928 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1416ec92c 488d158dd05201 lea rdx, [rip + 0x152d08d]
0x1416ec933 488d4d30 lea rcx, [rbp + 0x30]
0x1416ec937 e854ca1e01 call 0x1428d9390
0x1416ec93c cc int3 
0x1416ec93d 0f1f00 nop dword ptr [rax]
0x1416ec940 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ec945 55 push rbp
0x1416ec946 4157 push r15
0x1416ec948 4156 push r14
0x1416ec94a 56 push rsi
0x1416ec94b 57 push rdi
0x1416ec94c 53 push rbx
0x1416ec94d 4883ec58 sub rsp, 0x58
0x1416ec951 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ec958 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ec95d 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ec962 ba28000000 mov edx, 0x28
0x1416ec967 41b808000000 mov r8d, 8
0x1416ec96d 488b4d60 mov rcx, qword ptr [rbp + 0x60]
0x1416ec971 e8aa72f2fe call 0x140613c20
0x1416ec976 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ec97b 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416ec980 4883c458 add rsp, 0x58
0x1416ec984 5b pop rbx
0x1416ec985 5f pop rdi
0x1416ec986 5e pop rsi
0x1416ec987 415e pop r14
0x1416ec989 415f pop r15
0x1416ec98b 5d pop rbp
0x1416ec98c c3 ret 
0x1416ec98d 0f1f00 nop dword ptr [rax]
0x1416ec990 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ec995 55 push rbp
0x1416ec996 4157 push r15
0x1416ec998 4156 push r14
0x1416ec99a 56 push rsi
0x1416ec99b 57 push rdi
0x1416ec99c 53 push rbx
0x1416ec99d 4883ec58 sub rsp, 0x58
0x1416ec9a1 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ec9a8 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ec9ad 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ec9b2 ba28000000 mov edx, 0x28
0x1416ec9b7 41b808000000 mov r8d, 8
0x1416ec9bd 488b4d60 mov rcx, qword ptr [rbp + 0x60]
0x1416ec9c1 e85a72f2fe call 0x140613c20
0x1416ec9c6 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ec9cb 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416ec9d0 4883c458 add rsp, 0x58
0x1416ec9d4 5b pop rbx
0x1416ec9d5 5f pop rdi
0x1416ec9d6 5e pop rsi
0x1416ec9d7 415e pop r14
0x1416ec9d9 415f pop r15
0x1416ec9db 5d pop rbp
0x1416ec9dc c3 ret 
0x1416ec9dd 0f1f00 nop dword ptr [rax]
0x1416ec9e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ec9e5 55 push rbp
0x1416ec9e6 4157 push r15
0x1416ec9e8 4156 push r14
0x1416ec9ea 56 push rsi
0x1416ec9eb 57 push rdi
0x1416ec9ec 53 push rbx
0x1416ec9ed 4883ec58 sub rsp, 0x58
0x1416ec9f1 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ec9f8 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ec9fd 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1416eca02 488d4d10 lea rcx, [rbp + 0x10]
0x1416eca06 e8b5a8ffff call 0x1416e72c0
0x1416eca0b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416eca10 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416eca15 4883c458 add rsp, 0x58
0x1416eca19 5b pop rbx
0x1416eca1a 5f pop rdi
0x1416eca1b 5e pop rsi
0x1416eca1c 415e pop r14
0x1416eca1e 415f pop r15
0x1416eca20 5d pop rbp
0x1416eca21 c3 ret 
