0x14158e770 55 push rbp
0x14158e771 4157 push r15
0x14158e773 4156 push r14
0x14158e775 4154 push r12
0x14158e777 56 push rsi
0x14158e778 57 push rdi
0x14158e779 53 push rbx
0x14158e77a 4881ec90000000 sub rsp, 0x90
0x14158e781 488dac2480000000 lea rbp, [rsp + 0x80]
0x14158e789 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x14158e791 4d89c7 mov r15, r8
0x14158e794 4d85c0 test r8, r8
0x14158e797 0f8855010000 js 0x14158e8f2
0x14158e79d 4c89cf mov rdi, r9
0x14158e7a0 4889ce mov rsi, rcx
0x14158e7a3 742d je 0x14158e7d2
0x14158e7a5 4989d6 mov r14, rdx
0x14158e7a8 0fb6055a640a02 movzx eax, byte ptr [rip + 0x20a645a]
0x14158e7af bb01000000 mov ebx, 1
0x14158e7b4 ba01000000 mov edx, 1
0x14158e7b9 4c89f9 mov rcx, r15
0x14158e7bc e84f5408ff call 0x140613c10
0x14158e7c1 4885c0 test rax, rax
0x14158e7c4 0f842a010000 je 0x14158e8f4
0x14158e7ca 4989c4 mov r12, rax
0x14158e7cd 4c89f2 mov rdx, r14
0x14158e7d0 eb06 jmp 0x14158e7d8
0x14158e7d2 41bc01000000 mov r12d, 1
0x14158e7d8 4c89e1 mov rcx, r12
0x14158e7db 4d89f8 mov r8, r15
0x14158e7de e81d543901 call 0x142923c00
0x14158e7e3 4c897dc0 mov qword ptr [rbp - 0x40], r15
0x14158e7e7 4c8965c8 mov qword ptr [rbp - 0x38], r12
0x14158e7eb 4c897dd0 mov qword ptr [rbp - 0x30], r15
0x14158e7ef 803f00 cmp byte ptr [rdi], 0
0x14158e7f2 0fb60510640a02 movzx eax, byte ptr [rip + 0x20a6410]
0x14158e7f9 7425 je 0x14158e820
0x14158e7fb bf04000000 mov edi, 4
0x14158e800 b904000000 mov ecx, 4
0x14158e805 ba01000000 mov edx, 1
0x14158e80a e8015408ff call 0x140613c10
0x14158e80f 4885c0 test rax, rax
0x14158e812 0f84ee000000 je 0x14158e906
0x14158e818 c700576f6f64 mov dword ptr [rax], 0x646f6f57
0x14158e81e eb2f jmp 0x14158e84f
0x14158e820 bf05000000 mov edi, 5
0x14158e825 b905000000 mov ecx, 5
0x14158e82a ba01000000 mov edx, 1
0x14158e82f e8dc5308ff call 0x140613c10
0x14158e834 4885c0 test rax, rax
0x14158e837 0f84e9000000 je 0x14158e926
0x14158e83d 0fb60d68107001 movzx ecx, byte ptr [rip + 0x1701068]
0x14158e844 884804 mov byte ptr [rax + 4], cl
0x14158e847 8b0d5b107001 mov ecx, dword ptr [rip + 0x170105b]
0x14158e84d 8908 mov dword ptr [rax], ecx
0x14158e84f c645a003 mov byte ptr [rbp - 0x60], 3
0x14158e853 48897da8 mov qword ptr [rbp - 0x58], rdi
0x14158e857 488945b0 mov qword ptr [rbp - 0x50], rax
0x14158e85b 48897db8 mov qword ptr [rbp - 0x48], rdi
0x14158e85f 4883c618 add rsi, 0x18
0x14158e863 488d4dd8 lea rcx, [rbp - 0x28]
0x14158e867 4c8d45c0 lea r8, [rbp - 0x40]
0x14158e86b 4c8d4da0 lea r9, [rbp - 0x60]
0x14158e86f 4889f2 mov rdx, rsi
0x14158e872 e8b9d1eaff call 0x14143ba30
0x14158e877 0fb645d8 movzx eax, byte ptr [rbp - 0x28]
0x14158e87b 4883f806 cmp rax, 6
0x14158e87f 7754 ja 0x14158e8d5
0x14158e881 488d0db49a6e01 lea rcx, [rip + 0x16e9ab4]
0x14158e888 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x14158e88c 4801c8 add rax, rcx
0x14158e88f ffe0 jmp rax
0x14158e891 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x14158e895 4885d2 test rdx, rdx
0x14158e898 7444 je 0x14158e8de
0x14158e89a 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x14158e89e 41b801000000 mov r8d, 1
0x14158e8a4 eb28 jmp 0x14158e8ce
0x14158e8a6 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x14158e8aa 488b55f0 mov rdx, qword ptr [rbp - 0x10]
0x14158e8ae 48894d00 mov qword ptr [rbp], rcx
0x14158e8b2 e8b9a42fff call 0x140888d70
0x14158e8b7 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x14158e8bb 4885d2 test rdx, rdx
0x14158e8be 741e je 0x14158e8de
0x14158e8c0 48c1e205 shl rdx, 5
0x14158e8c4 41b808000000 mov r8d, 8
0x14158e8ca 488b4d00 mov rcx, qword ptr [rbp]
0x14158e8ce e84d5308ff call 0x140613c20
0x14158e8d3 eb09 jmp 0x14158e8de
0x14158e8d5 488d4de0 lea rcx, [rbp - 0x20]
0x14158e8d9 e862a630ff call 0x140898f40
0x14158e8de 31c0 xor eax, eax
0x14158e8e0 4881c490000000 add rsp, 0x90
0x14158e8e7 5b pop rbx
0x14158e8e8 5f pop rdi
0x14158e8e9 5e pop rsi
0x14158e8ea 415c pop r12
0x14158e8ec 415e pop r14
0x14158e8ee 415f pop r15
0x14158e8f0 5d pop rbp
0x14158e8f1 c3 ret 
0x14158e8f2 31db xor ebx, ebx
0x14158e8f4 4c8d05dd986e01 lea r8, [rip + 0x16e98dd]
0x14158e8fb 4889d9 mov rcx, rbx
0x14158e8fe 4c89fa mov rdx, r15
0x14158e901 e8dda63401 call 0x1428d8fe3
0x14158e906 4c8965f8 mov qword ptr [rbp - 8], r12
0x14158e90a 4c897d00 mov qword ptr [rbp], r15
0x14158e90e 4c8d05f3097001 lea r8, [rip + 0x17009f3]
0x14158e915 b901000000 mov ecx, 1
0x14158e91a ba04000000 mov edx, 4
0x14158e91f e8bfa63401 call 0x1428d8fe3
0x14158e924 eb1e jmp 0x14158e944
0x14158e926 4c8965f8 mov qword ptr [rbp - 8], r12
0x14158e92a 4c897d00 mov qword ptr [rbp], r15
0x14158e92e 4c8d05d3097001 lea r8, [rip + 0x17009d3]
0x14158e935 b901000000 mov ecx, 1
0x14158e93a ba05000000 mov edx, 5
0x14158e93f e89fa63401 call 0x1428d8fe3
0x14158e944 0f0b ud2 
0x14158e946 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14158e950 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158e955 55 push rbp
0x14158e956 4157 push r15
0x14158e958 4156 push r14
0x14158e95a 4154 push r12
0x14158e95c 56 push rsi
0x14158e95d 57 push rdi
0x14158e95e 53 push rbx
0x14158e95f 4883ec20 sub rsp, 0x20
0x14158e963 488daa80000000 lea rbp, [rdx + 0x80]
0x14158e96a 48837d0000 cmp qword ptr [rbp], 0
0x14158e96f 7413 je 0x14158e984
0x14158e971 41b801000000 mov r8d, 1
0x14158e977 488b4df8 mov rcx, qword ptr [rbp - 8]
0x14158e97b 488b5500 mov rdx, qword ptr [rbp]
0x14158e97f e89c5208ff call 0x140613c20
0x14158e984 90 nop 
0x14158e985 4883c420 add rsp, 0x20
0x14158e989 5b pop rbx
0x14158e98a 5f pop rdi
0x14158e98b 5e pop rsi
0x14158e98c 415c pop r12
0x14158e98e 415e pop r14
0x14158e990 415f pop r15
0x14158e992 5d pop rbp
0x14158e993 c3 ret 
0x14158e994 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14158e9a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158e9a5 55 push rbp
0x14158e9a6 4157 push r15
0x14158e9a8 4156 push r14
0x14158e9aa 4154 push r12
0x14158e9ac 56 push rsi
0x14158e9ad 57 push rdi
0x14158e9ae 53 push rbx
0x14158e9af 4883ec20 sub rsp, 0x20
0x14158e9b3 488daa80000000 lea rbp, [rdx + 0x80]
0x14158e9ba 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x14158e9be 4885d2 test rdx, rdx
0x14158e9c1 7413 je 0x14158e9d6
0x14158e9c3 48c1e205 shl rdx, 5
0x14158e9c7 41b808000000 mov r8d, 8
0x14158e9cd 488b4d00 mov rcx, qword ptr [rbp]
0x14158e9d1 e84a5208ff call 0x140613c20
0x14158e9d6 90 nop 
0x14158e9d7 4883c420 add rsp, 0x20
0x14158e9db 5b pop rbx
0x14158e9dc 5f pop rdi
0x14158e9dd 5e pop rsi
0x14158e9de 415c pop r12
0x14158e9e0 415e pop r14
0x14158e9e2 415f pop r15
0x14158e9e4 5d pop rbp
0x14158e9e5 c3 ret 
