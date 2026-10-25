0x1416eb8a0 push r14
0x1416eb8a2 push rsi
0x1416eb8a3 push rdi
0x1416eb8a4 push rbx
0x1416eb8a5 sub rsp, 0x38
0x1416eb8a9 mov rsi, rdx
0x1416eb8ac mov edx, dword ptr [rcx]
0x1416eb8ae add edx, -2
0x1416eb8b1 cmp edx, 2
0x1416eb8b4 mov eax, 2
0x1416eb8b9 cmovb eax, edx
0x1416eb8bc test eax, eax
0x1416eb8be je 0x1416eb8e9
0x1416eb8c0 cmp eax, 1
0x1416eb8c3 jne 0x1416eb91c
0x1416eb8c5 lea rdx, [rsi + 8]
0x1416eb8c9 lea r8, [rip + 0x15a4234]
0x1416eb8d0 mov r9d, 3
0x1416eb8d6 mov rcx, rsi
0x1416eb8d9 call 0x141580d20
0x1416eb8de test rax, rax
0x1416eb8e1 je 0x1416eba06
0x1416eb8e7 jmp 0x1416eb90b
0x1416eb8e9 lea rdx, [rsi + 8]
0x1416eb8ed lea r8, [rip + 0x15a420e]
0x1416eb8f4 mov r9d, 2
0x1416eb8fa mov rcx, rsi
0x1416eb8fd call 0x141580d20
0x1416eb902 test rax, rax
0x1416eb905 je 0x1416eba06
0x1416eb90b mov rcx, rax
0x1416eb90e add rsp, 0x38
0x1416eb912 pop rbx
0x1416eb913 pop rdi
0x1416eb914 pop rsi
0x1416eb915 pop r14
0x1416eb917 jmp 0x1428c98a0
0x1416eb91c mov rdi, rcx
0x1416eb91f lea rdx, [rsi + 8]
0x1416eb923 mov rbx, qword ptr [rsi]
0x1416eb926 mov rax, qword ptr [rbx + 0x10]
0x1416eb92a cmp qword ptr [rbx], rax
0x1416eb92d je 0x1416eba48
0x1416eb933 mov rcx, qword ptr [rbx + 8]
0x1416eb937 mov byte ptr [rcx + rax], 0x7b
0x1416eb93b inc rax
0x1416eb93e mov qword ptr [rbx + 0x10], rax
0x1416eb942 lea r8, [rip + 0x15a41be]
0x1416eb949 mov r9d, 0xc
0x1416eb94f mov rcx, rsi
0x1416eb952 call 0x141580d20
0x1416eb957 test rax, rax
0x1416eb95a jne 0x1416eba3e
0x1416eb960 mov rbx, qword ptr [rsi]
0x1416eb963 mov rdx, qword ptr [rbx + 0x10]
0x1416eb967 cmp qword ptr [rbx], rdx
0x1416eb96a je 0x1416eba77
0x1416eb970 mov rax, qword ptr [rbx + 8]
0x1416eb974 mov byte ptr [rax + rdx], 0x3a
0x1416eb978 inc rdx
0x1416eb97b mov qword ptr [rbx + 0x10], rdx
0x1416eb97f mov rbx, qword ptr [rsi]
0x1416eb982 mov rdx, qword ptr [rbx + 0x10]
0x1416eb986 cmp qword ptr [rbx], rdx
0x1416eb989 je 0x1416eba9d
0x1416eb98f mov rax, qword ptr [rbx + 8]
0x1416eb993 mov byte ptr [rax + rdx], 0x7b
0x1416eb997 inc rdx
0x1416eb99a mov qword ptr [rbx + 0x10], rdx
0x1416eb99e mov qword ptr [rsp + 0x28], rsi
0x1416eb9a3 mov byte ptr [rsp + 0x30], 1
0x1416eb9a8 lea rdx, [rip + 0x15a3f57]
0x1416eb9af lea rcx, [rsp + 0x28]
0x1416eb9b4 mov r8d, 6
0x1416eb9ba mov r9, rdi
0x1416eb9bd call 0x141586760
0x1416eb9c2 test rax, rax
0x1416eb9c5 jne 0x1416eba08
0x1416eb9c7 mov rax, qword ptr [rsp + 0x28]
0x1416eb9cc mov rsi, qword ptr [rax]
0x1416eb9cf cmp byte ptr [rsp + 0x30], 0
0x1416eb9d4 je 0x1416eba12
0x1416eb9d6 mov rdx, qword ptr [rsi + 0x10]
0x1416eb9da cmp qword ptr [rsi], rdx
0x1416eb9dd je 0x1416ebac3
0x1416eb9e3 mov rax, qword ptr [rsi + 8]
0x1416eb9e7 mov byte ptr [rax + rdx], 0x7d
0x1416eb9eb inc rdx
0x1416eb9ee mov qword ptr [rsi + 0x10], rdx
0x1416eb9f2 cmp qword ptr [rsi], rdx
0x1416eb9f5 je 0x1416eba1b
0x1416eb9f7 mov rax, qword ptr [rsi + 8]
0x1416eb9fb mov byte ptr [rax + rdx], 0x7d
0x1416eb9ff inc rdx
0x1416eba02 mov qword ptr [rsi + 0x10], rdx
0x1416eba06 xor eax, eax
0x1416eba08 add rsp, 0x38
0x1416eba0c pop rbx
0x1416eba0d pop rdi
0x1416eba0e pop rsi
0x1416eba0f pop r14
0x1416eba11 ret 
0x1416eba12 mov rdx, qword ptr [rsi + 0x10]
0x1416eba16 cmp qword ptr [rsi], rdx
0x1416eba19 jne 0x1416eb9f7
0x1416eba1b mov qword ptr [rsp + 0x20], 1
0x1416eba24 mov r8d, 1
0x1416eba2a mov r9d, 1
0x1416eba30 mov rcx, rsi
0x1416eba33 call 0x1428a9a60
0x1416eba38 mov rdx, qword ptr [rsi + 0x10]
0x1416eba3c jmp 0x1416eb9f7
0x1416eba3e mov rcx, rax
0x1416eba41 call 0x1428c98a0
0x1416eba46 jmp 0x1416eba08
0x1416eba48 mov qword ptr [rsp + 0x20], 1
0x1416eba51 mov r8d, 1
0x1416eba57 mov r9d, 1
0x1416eba5d mov rcx, rbx
0x1416eba60 mov r14, rdx
0x1416eba63 mov rdx, rax
0x1416eba66 call 0x1428a9a60
0x1416eba6b mov rdx, r14
0x1416eba6e mov rax, qword ptr [rbx + 0x10]
0x1416eba72 jmp 0x1416eb933
0x1416eba77 mov qword ptr [rsp + 0x20], 1
0x1416eba80 mov r8d, 1
0x1416eba86 mov r9d, 1
0x1416eba8c mov rcx, rbx
0x1416eba8f call 0x1428a9a60
0x1416eba94 mov rdx, qword ptr [rbx + 0x10]
0x1416eba98 jmp 0x1416eb970
0x1416eba9d mov qword ptr [rsp + 0x20], 1
0x1416ebaa6 mov r8d, 1
0x1416ebaac mov r9d, 1
0x1416ebab2 mov rcx, rbx
0x1416ebab5 call 0x1428a9a60
0x1416ebaba mov rdx, qword ptr [rbx + 0x10]
0x1416ebabe jmp 0x1416eb98f
0x1416ebac3 mov qword ptr [rsp + 0x20], 1
0x1416ebacc mov r8d, 1
0x1416ebad2 mov r9d, 1
0x1416ebad8 mov rcx, rsi
0x1416ebadb call 0x1428a9a60
0x1416ebae0 mov rdx, qword ptr [rsi + 0x10]
0x1416ebae4 jmp 0x1416eb9e3
