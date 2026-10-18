0x1416ea700 55 push rbp
0x1416ea701 56 push rsi
0x1416ea702 57 push rdi
0x1416ea703 4881ec90000000 sub rsp, 0x90
0x1416ea70a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416ea712 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x1416ea71a 4889d7 mov rdi, rdx
0x1416ea71d 4889ce mov rsi, rcx
0x1416ea720 48b80000000000000080 movabs rax, 0x8000000000000000
0x1416ea72a 488945d0 mov qword ptr [rbp - 0x30], rax
0x1416ea72e 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1416ea736 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1416ea73e c6450701 mov byte ptr [rbp + 7], 1
0x1416ea742 488d15bb515a01 lea rdx, [rip + 0x15a51bb]
0x1416ea749 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea74d 41b802000000 mov r8d, 2
0x1416ea753 4989f9 mov r9, rdi
0x1416ea756 e8a5eeeaff call 0x141599600
0x1416ea75b 4885c0 test rax, rax
0x1416ea75e 0f85dd000000 jne 0x1416ea841
0x1416ea764 4c8d4f24 lea r9, [rdi + 0x24]
0x1416ea768 c6450701 mov byte ptr [rbp + 7], 1
0x1416ea76c 488d1593515a01 lea rdx, [rip + 0x15a5193]
0x1416ea773 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea777 41b806000000 mov r8d, 6
0x1416ea77d e80e9aeaff call 0x141594190
0x1416ea782 4885c0 test rax, rax
0x1416ea785 0f85b6000000 jne 0x1416ea841
0x1416ea78b 4c8d4f08 lea r9, [rdi + 8]
0x1416ea78f c6450701 mov byte ptr [rbp + 7], 1
0x1416ea793 488d1572515a01 lea rdx, [rip + 0x15a5172]
0x1416ea79a 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea79e 41b80a000000 mov r8d, 0xa
0x1416ea7a4 e8a7d1eaff call 0x141597950
0x1416ea7a9 4885c0 test rax, rax
0x1416ea7ac 0f858f000000 jne 0x1416ea841
0x1416ea7b2 4c8d4f4c lea r9, [rdi + 0x4c]
0x1416ea7b6 c6450701 mov byte ptr [rbp + 7], 1
0x1416ea7ba 488d1555515a01 lea rdx, [rip + 0x15a5155]
0x1416ea7c1 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea7c5 41b807000000 mov r8d, 7
0x1416ea7cb e850b2eaff call 0x141595a20
0x1416ea7d0 4885c0 test rax, rax
0x1416ea7d3 756c jne 0x1416ea841
0x1416ea7d5 4c8d4f50 lea r9, [rdi + 0x50]
0x1416ea7d9 c6450701 mov byte ptr [rbp + 7], 1
0x1416ea7dd 488d153c505a01 lea rdx, [rip + 0x15a503c]
0x1416ea7e4 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea7e8 41b808000000 mov r8d, 8
0x1416ea7ee e85deaeaff call 0x141599250
0x1416ea7f3 4885c0 test rax, rax
0x1416ea7f6 7549 jne 0x1416ea841
0x1416ea7f8 4c8d4f54 lea r9, [rdi + 0x54]
0x1416ea7fc c6450701 mov byte ptr [rbp + 7], 1
0x1416ea800 488d1516515a01 lea rdx, [rip + 0x15a5116]
0x1416ea807 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea80b 41b80b000000 mov r8d, 0xb
0x1416ea811 e84af6eaff call 0x141599e60
0x1416ea816 4885c0 test rax, rax
0x1416ea819 7526 jne 0x1416ea841
0x1416ea81b 4883c740 add rdi, 0x40
0x1416ea81f c6450701 mov byte ptr [rbp + 7], 1
0x1416ea823 488d15fe505a01 lea rdx, [rip + 0x15a50fe]
0x1416ea82a 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea82e 41b80f000000 mov r8d, 0xf
0x1416ea834 4989f9 mov r9, rdi
0x1416ea837 e8f4c2eaff call 0x141596b30
0x1416ea83c 4885c0 test rax, rax
0x1416ea83f 743b je 0x1416ea87c
0x1416ea841 488d4de8 lea rcx, [rbp - 0x18]
0x1416ea845 48894608 mov qword ptr [rsi + 8], rax
0x1416ea849 c60606 mov byte ptr [rsi], 6
0x1416ea84c e8efe61aff call 0x140898f40
0x1416ea851 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x1416ea855 4889d0 mov rax, rdx
0x1416ea858 48f7d8 neg rax
0x1416ea85b 7011 jo 0x1416ea86e
0x1416ea85d 730f jae 0x1416ea86e
0x1416ea85f 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x1416ea863 41b801000000 mov r8d, 1
0x1416ea869 e8b293f2fe call 0x140613c20
0x1416ea86e 4889f0 mov rax, rsi
0x1416ea871 4881c490000000 add rsp, 0x90
0x1416ea878 5f pop rdi
0x1416ea879 5e pop rsi
0x1416ea87a 5d pop rbp
0x1416ea87b c3 ret 
0x1416ea87c 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x1416ea880 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x1416ea884 0f1055f0 movups xmm2, xmmword ptr [rbp - 0x10]
0x1416ea888 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x1416ea88c 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1416ea890 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1416ea894 c6450700 mov byte ptr [rbp + 7], 0
0x1416ea898 488d55a0 lea rdx, [rbp - 0x60]
0x1416ea89c 4889f1 mov rcx, rsi
0x1416ea89f e81cbc49ff call 0x140b864c0
0x1416ea8a4 ebc8 jmp 0x1416ea86e
0x1416ea8a6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416ea8b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea8b5 55 push rbp
0x1416ea8b6 56 push rsi
0x1416ea8b7 57 push rdi
0x1416ea8b8 4883ec20 sub rsp, 0x20
0x1416ea8bc 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea8c3 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x1416ea8c7 488d045500000000 lea rax, [rdx*2]
0x1416ea8cf 4885c0 test rax, rax
0x1416ea8d2 740f je 0x1416ea8e3
0x1416ea8d4 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x1416ea8d8 41b801000000 mov r8d, 1
0x1416ea8de e83d93f2fe call 0x140613c20
0x1416ea8e3 90 nop 
0x1416ea8e4 4883c420 add rsp, 0x20
0x1416ea8e8 5f pop rdi
0x1416ea8e9 5e pop rsi
0x1416ea8ea 5d pop rbp
0x1416ea8eb c3 ret 
0x1416ea8ec 0f1f4000 nop dword ptr [rax]
0x1416ea8f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea8f5 55 push rbp
0x1416ea8f6 56 push rsi
0x1416ea8f7 57 push rdi
0x1416ea8f8 4883ec20 sub rsp, 0x20
0x1416ea8fc 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea903 807d0700 cmp byte ptr [rbp + 7], 0
0x1416ea907 7508 jne 0x1416ea911
0x1416ea909 4883c420 add rsp, 0x20
0x1416ea90d 5f pop rdi
0x1416ea90e 5e pop rsi
0x1416ea90f 5d pop rbp
0x1416ea910 c3 ret 
0x1416ea911 488d4dd0 lea rcx, [rbp - 0x30]
0x1416ea915 e8463217ff call 0x14085db60
0x1416ea91a ebed jmp 0x1416ea909
