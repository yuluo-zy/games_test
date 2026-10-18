0x1416eaa40 55 push rbp
0x1416eaa41 56 push rsi
0x1416eaa42 57 push rdi
0x1416eaa43 4881ec90000000 sub rsp, 0x90
0x1416eaa4a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416eaa52 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x1416eaa5a 4889d7 mov rdi, rdx
0x1416eaa5d 4889ce mov rsi, rcx
0x1416eaa60 48b80000000000000080 movabs rax, 0x8000000000000000
0x1416eaa6a 488945d0 mov qword ptr [rbp - 0x30], rax
0x1416eaa6e 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1416eaa76 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1416eaa7e 4c8d4a08 lea r9, [rdx + 8]
0x1416eaa82 c6450701 mov byte ptr [rbp + 7], 1
0x1416eaa86 488d152a4f5a01 lea rdx, [rip + 0x15a4f2a]
0x1416eaa8d 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eaa91 41b80a000000 mov r8d, 0xa
0x1416eaa97 e884afeaff call 0x141595a20
0x1416eaa9c 4885c0 test rax, rax
0x1416eaa9f 0f85dc000000 jne 0x1416eab81
0x1416eaaa5 4c8d4f0c lea r9, [rdi + 0xc]
0x1416eaaa9 c6450701 mov byte ptr [rbp + 7], 1
0x1416eaaad 488d150d4f5a01 lea rdx, [rip + 0x15a4f0d]
0x1416eaab4 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eaab8 41b809000000 mov r8d, 9
0x1416eaabe e85dafeaff call 0x141595a20
0x1416eaac3 4885c0 test rax, rax
0x1416eaac6 0f85b5000000 jne 0x1416eab81
0x1416eaacc c6450701 mov byte ptr [rbp + 7], 1
0x1416eaad0 488d15f34e5a01 lea rdx, [rip + 0x15a4ef3]
0x1416eaad7 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eaadb 41b80d000000 mov r8d, 0xd
0x1416eaae1 4989f9 mov r9, rdi
0x1416eaae4 e887bbeaff call 0x141596670
0x1416eaae9 4885c0 test rax, rax
0x1416eaaec 0f858f000000 jne 0x1416eab81
0x1416eaaf2 4c8d4f10 lea r9, [rdi + 0x10]
0x1416eaaf6 c6450701 mov byte ptr [rbp + 7], 1
0x1416eaafa 488d15d64e5a01 lea rdx, [rip + 0x15a4ed6]
0x1416eab01 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eab05 41b80f000000 mov r8d, 0xf
0x1416eab0b e810afeaff call 0x141595a20
0x1416eab10 4885c0 test rax, rax
0x1416eab13 756c jne 0x1416eab81
0x1416eab15 4c8d4f18 lea r9, [rdi + 0x18]
0x1416eab19 c6450701 mov byte ptr [rbp + 7], 1
0x1416eab1d 488d15c24e5a01 lea rdx, [rip + 0x15a4ec2]
0x1416eab24 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eab28 41b809000000 mov r8d, 9
0x1416eab2e e83da0eaff call 0x141594b70
0x1416eab33 4885c0 test rax, rax
0x1416eab36 7549 jne 0x1416eab81
0x1416eab38 4c8d4f14 lea r9, [rdi + 0x14]
0x1416eab3c c6450701 mov byte ptr [rbp + 7], 1
0x1416eab40 488d15a84e5a01 lea rdx, [rip + 0x15a4ea8]
0x1416eab47 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eab4b 41b80e000000 mov r8d, 0xe
0x1416eab51 e8caaeeaff call 0x141595a20
0x1416eab56 4885c0 test rax, rax
0x1416eab59 7526 jne 0x1416eab81
0x1416eab5b 4883c719 add rdi, 0x19
0x1416eab5f c6450701 mov byte ptr [rbp + 7], 1
0x1416eab63 488d15934e5a01 lea rdx, [rip + 0x15a4e93]
0x1416eab6a 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eab6e 41b80e000000 mov r8d, 0xe
0x1416eab74 4989f9 mov r9, rdi
0x1416eab77 e87489eaff call 0x1415934f0
0x1416eab7c 4885c0 test rax, rax
0x1416eab7f 743b je 0x1416eabbc
0x1416eab81 488d4de8 lea rcx, [rbp - 0x18]
0x1416eab85 48894608 mov qword ptr [rsi + 8], rax
0x1416eab89 c60606 mov byte ptr [rsi], 6
0x1416eab8c e8afe31aff call 0x140898f40
0x1416eab91 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x1416eab95 4889d0 mov rax, rdx
0x1416eab98 48f7d8 neg rax
0x1416eab9b 7011 jo 0x1416eabae
0x1416eab9d 730f jae 0x1416eabae
0x1416eab9f 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x1416eaba3 41b801000000 mov r8d, 1
0x1416eaba9 e87290f2fe call 0x140613c20
0x1416eabae 4889f0 mov rax, rsi
0x1416eabb1 4881c490000000 add rsp, 0x90
0x1416eabb8 5f pop rdi
0x1416eabb9 5e pop rsi
0x1416eabba 5d pop rbp
0x1416eabbb c3 ret 
0x1416eabbc 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x1416eabc0 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x1416eabc4 0f1055f0 movups xmm2, xmmword ptr [rbp - 0x10]
0x1416eabc8 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x1416eabcc 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1416eabd0 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1416eabd4 c6450700 mov byte ptr [rbp + 7], 0
0x1416eabd8 488d55a0 lea rdx, [rbp - 0x60]
0x1416eabdc 4889f1 mov rcx, rsi
0x1416eabdf e8dcb849ff call 0x140b864c0
0x1416eabe4 ebc8 jmp 0x1416eabae
0x1416eabe6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416eabf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eabf5 55 push rbp
0x1416eabf6 56 push rsi
0x1416eabf7 57 push rdi
0x1416eabf8 4883ec20 sub rsp, 0x20
0x1416eabfc 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eac03 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x1416eac07 488d045500000000 lea rax, [rdx*2]
0x1416eac0f 4885c0 test rax, rax
0x1416eac12 740f je 0x1416eac23
0x1416eac14 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x1416eac18 41b801000000 mov r8d, 1
0x1416eac1e e8fd8ff2fe call 0x140613c20
0x1416eac23 90 nop 
0x1416eac24 4883c420 add rsp, 0x20
0x1416eac28 5f pop rdi
0x1416eac29 5e pop rsi
0x1416eac2a 5d pop rbp
0x1416eac2b c3 ret 
0x1416eac2c 0f1f4000 nop dword ptr [rax]
0x1416eac30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eac35 55 push rbp
0x1416eac36 56 push rsi
0x1416eac37 57 push rdi
0x1416eac38 4883ec20 sub rsp, 0x20
0x1416eac3c 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eac43 807d0700 cmp byte ptr [rbp + 7], 0
0x1416eac47 7508 jne 0x1416eac51
0x1416eac49 4883c420 add rsp, 0x20
0x1416eac4d 5f pop rdi
0x1416eac4e 5e pop rsi
0x1416eac4f 5d pop rbp
0x1416eac50 c3 ret 
0x1416eac51 488d4dd0 lea rcx, [rbp - 0x30]
0x1416eac55 e8062f17ff call 0x14085db60
0x1416eac5a ebed jmp 0x1416eac49
