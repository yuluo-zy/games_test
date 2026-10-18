0x141e9d960 push rbp
0x141e9d961 push r15
0x141e9d963 push r14
0x141e9d965 push r12
0x141e9d967 push rsi
0x141e9d968 push rdi
0x141e9d969 push rbx
0x141e9d96a sub rsp, 0x70
0x141e9d96e lea rbp, [rsp + 0x70]
0x141e9d973 mov qword ptr [rbp - 8], 0xfffffffffffffffe
0x141e9d97b mov rdi, rdx
0x141e9d97e mov rsi, rcx
0x141e9d981 call 0x140c16a00
0x141e9d986 test al, al
0x141e9d988 mov byte ptr [rbp - 0x10], al
0x141e9d98b je 0x141e9d9e9
0x141e9d98d mov ebx, 0x29
0x141e9d992 mov ecx, 0x29
0x141e9d997 call 0x141e9ca20
0x141e9d99c lea r14, [rip + 0x10f7a70] ; b'toolbar::text::update_text_renderer_scale::f'
0x141e9d9a3 cmp rax, 1
0x141e9d9a7 jne 0x141e9da54
0x141e9d9ad mov r9, rdx
0x141e9d9b0 test rdx, rdx
0x141e9d9b3 je 0x141e9d9f2
0x141e9d9b5 cmp r9, 0x29
0x141e9d9b9 jae 0x141e9d9f0
0x141e9d9bb lea rax, [rip + 0x10f7a51] ; b'toolbar::text::update_text_renderer_scale::f'
0x141e9d9c2 cmp byte ptr [r9 + rax], 0xbf
0x141e9d9c7 jg 0x141e9d9f2
0x141e9d9c9 lea rax, [rip + 0x10f7af0]
0x141e9d9d0 mov qword ptr [rsp + 0x20], rax
0x141e9d9d5 lea rcx, [rip + 0x10f7a37] ; b'toolbar::text::update_text_renderer_scale::f'
0x141e9d9dc mov edx, 0x29
0x141e9d9e1 xor r8d, r8d
0x141e9d9e4 call 0x1428d9940
0x141e9d9e9 xor eax, eax
0x141e9d9eb jmp 0x141e9dbad
0x141e9d9f0 jne 0x141e9d9c9
0x141e9d9f2 mov rcx, r9
0x141e9d9f5 call 0x141e9ca20
0x141e9d9fa cmp rax, 1
0x141e9d9fe jne 0x141e9da54
0x141e9da00 mov r8, rdx
0x141e9da03 add r8, 2
0x141e9da07 je 0x141e9da42
0x141e9da09 cmp r8, 0x29
0x141e9da0d jae 0x141e9da40
0x141e9da0f lea rax, [rip + 0x10f79fd] ; b'toolbar::text::update_text_renderer_scale::f'
0x141e9da16 cmp byte ptr [r8 + rax], 0xbf
0x141e9da1b jg 0x141e9da42
0x141e9da1d lea rax, [rip + 0x10f7ab4]
0x141e9da24 mov qword ptr [rsp + 0x20], rax
0x141e9da29 lea rcx, [rip + 0x10f79e3] ; b'toolbar::text::update_text_renderer_scale::f'
0x141e9da30 mov edx, 0x29
0x141e9da35 mov r9d, 0x29
0x141e9da3b call 0x1428d9940
0x141e9da40 jne 0x141e9da1d
0x141e9da42 mov ebx, 0x27
0x141e9da47 sub rbx, rdx
0x141e9da4a lea r14, [rip + 0x10f79c2] ; b'toolbar::text::update_text_renderer_scale::f'
0x141e9da51 add r14, r8
0x141e9da54 lea r8, [rip + 0x10f7b1f]
0x141e9da5b lea r15, [rip + 0x10f7af6] ; b'crates/systems/toolbar/src/text.rs'
0x141e9da62 nop word ptr cs:[rax + rax]
0x141e9da70 movsx eax, byte ptr [r8 - 1]
0x141e9da75 test eax, eax
0x141e9da77 js 0x141e9da90
0x141e9da79 dec r8
0x141e9da7c cmp eax, 0x5c
0x141e9da7f jne 0x141e9dae4
0x141e9da81 jmp 0x141e9daf6
0x141e9da83 nop word ptr cs:[rax + rax]
0x141e9da90 movzx ecx, byte ptr [r8 - 2]
0x141e9da95 cmp cl, 0xc0
0x141e9da98 jge 0x141e9dabd
0x141e9da9a movzx edx, byte ptr [r8 - 3]
0x141e9da9f cmp dl, 0xc0
0x141e9daa2 jge 0x141e9dac6
0x141e9daa4 movzx r9d, byte ptr [r8 - 4]
0x141e9daa9 add r8, -4
0x141e9daad and r9d, 7
0x141e9dab1 shl r9d, 6
0x141e9dab5 and edx, 0x3f
0x141e9dab8 or edx, r9d
0x141e9dabb jmp 0x141e9dacd
0x141e9dabd add r8, -2
0x141e9dac1 and ecx, 0x1f
0x141e9dac4 jmp 0x141e9dad5
0x141e9dac6 add r8, -3
0x141e9daca and edx, 0xf
0x141e9dacd shl edx, 6
0x141e9dad0 and ecx, 0x3f
0x141e9dad3 or ecx, edx
0x141e9dad5 shl ecx, 6
0x141e9dad8 and al, 0x3f
0x141e9dada movzx eax, al
0x141e9dadd or eax, ecx
0x141e9dadf cmp eax, 0x5c
0x141e9dae2 je 0x141e9daf6
0x141e9dae4 cmp eax, 0x2f
0x141e9dae7 je 0x141e9daf6
0x141e9dae9 cmp r8, r15
0x141e9daec jne 0x141e9da70
0x141e9daee mov r12d, 0x22
0x141e9daf4 jmp 0x141e9db43
0x141e9daf6 lea r15, [rip + 0x10f7a5b] ; b'crates/systems/toolbar/src/text.rs'
0x141e9dafd sub r8, r15
0x141e9db00 inc r8
0x141e9db03 je 0x141e9db37
0x141e9db05 cmp r8, 0x22
0x141e9db09 jae 0x141e9db35
0x141e9db0b cmp byte ptr [r8 + r15], 0xbf
0x141e9db10 jg 0x141e9db37
0x141e9db12 lea rax, [rip + 0x10f7987]
0x141e9db19 mov qword ptr [rsp + 0x20], rax
0x141e9db1e lea rcx, [rip + 0x10f7a33] ; b'crates/systems/toolbar/src/text.rs'
0x141e9db25 mov edx, 0x22
0x141e9db2a mov r9d, 0x22
0x141e9db30 call 0x1428d9940
0x141e9db35 jne 0x141e9db12
0x141e9db37 mov r12d, 0x22
0x141e9db3d sub r12, r8
0x141e9db40 add r15, r8
0x141e9db43 call 0x140c18130
0x141e9db48 mov rcx, rax
0x141e9db4b mov rax, qword ptr [rax]
0x141e9db4e cmp rax, 1
0x141e9db52 jne 0x141e9dc23
0x141e9db58 add rcx, 8
0x141e9db5c cmp qword ptr [rcx], 0
0x141e9db60 jne 0x141e9dc5d
0x141e9db66 mov qword ptr [rcx], 0xffffffffffffffff
0x141e9db6d mov qword ptr [rbp - 0x18], rcx
0x141e9db71 add rcx, 8
0x141e9db75 mov qword ptr [rsp + 0x20], r12
0x141e9db7a mov qword ptr [rsp + 0x30], 0
0x141e9db83 mov qword ptr [rsp + 0x28], 1
0x141e9db8c mov rdx, r14
0x141e9db8f mov r8, rbx
0x141e9db92 mov r9, r15
0x141e9db95 call 0x140c16d40
0x141e9db9a mov rbx, rax
0x141e9db9d mov rax, qword ptr [rbp - 0x18]
0x141e9dba1 inc qword ptr [rax]
0x141e9dba4 mov qword ptr [rbp - 0x30], rbx
0x141e9dba8 mov eax, 1
0x141e9dbad mov qword ptr [rbp - 0x38], rax
0x141e9dbb1 mov eax, dword ptr [rsi + 0x1c]
0x141e9dbb4 mov rcx, qword ptr [rsi + 0x10]
0x141e9dbb8 mov dword ptr [rcx], eax
0x141e9dbba mov rax, qword ptr [rdi]
0x141e9dbbd mov eax, dword ptr [rax + 4]
0x141e9dbc0 cvtsi2ss xmm1, rax
0x141e9dbc5 mov rcx, qword ptr [rsi]
0x141e9dbc8 divss xmm1, dword ptr [rip + 0xabda60]
0x141e9dbd0 call 0x140b8f3f0
0x141e9dbd5 cmp byte ptr [rbp - 0x10], 0
0x141e9dbd9 je 0x141e9dc14
0x141e9dbdb call 0x140c18130
0x141e9dbe0 mov rcx, rax
0x141e9dbe3 mov rax, qword ptr [rax]
0x141e9dbe6 cmp rax, 1
0x141e9dbea jne 0x141e9dc3d
0x141e9dbec add rcx, 8
0x141e9dbf0 cmp qword ptr [rcx], 0
0x141e9dbf4 jne 0x141e9dc5d
0x141e9dbf6 mov qword ptr [rcx], 0xffffffffffffffff
0x141e9dbfd mov qword ptr [rbp - 0x10], rcx
0x141e9dc01 add rcx, 8
0x141e9dc05 mov rdx, rbx
0x141e9dc08 call 0x140c170f0
0x141e9dc0d mov rax, qword ptr [rbp - 0x10]
0x141e9dc11 inc qword ptr [rax]
0x141e9dc14 add rsp, 0x70
0x141e9dc18 pop rbx
0x141e9dc19 pop rdi
0x141e9dc1a pop rsi
0x141e9dc1b pop r12
0x141e9dc1d pop r14
0x141e9dc1f pop r15
0x141e9dc21 pop rbp
0x141e9dc22 ret 
0x141e9dc23 test rax, rax
0x141e9dc26 jne 0x141e9dc51
0x141e9dc28 xor edx, edx
0x141e9dc2a call 0x1428e6250
0x141e9dc2f mov rcx, rax
0x141e9dc32 test rax, rax
0x141e9dc35 jne 0x141e9db5c
0x141e9dc3b jmp 0x141e9dc51
0x141e9dc3d test rax, rax
0x141e9dc40 jne 0x141e9dc51
0x141e9dc42 xor edx, edx
0x141e9dc44 call 0x1428e6250
0x141e9dc49 mov rcx, rax
0x141e9dc4c test rax, rax
0x141e9dc4f jne 0x141e9dbf0
0x141e9dc51 lea rcx, [rip + 0xd17680]
0x141e9dc58 call 0x1428d80d0
0x141e9dc5d lea rcx, [rip + 0xd176e4]
0x141e9dc64 call 0x1428d9250
0x141e9dc69 int3 
0x141e9dc6a nop word ptr [rax + rax]
0x141e9dc70 mov qword ptr [rsp + 0x10], rdx
0x141e9dc75 push rbp
0x141e9dc76 push r15
0x141e9dc78 push r14
0x141e9dc7a push r12
0x141e9dc7c push rsi
0x141e9dc7d push rdi
0x141e9dc7e push rbx
0x141e9dc7f sub rsp, 0x40
0x141e9dc83 lea rbp, [rdx + 0x70]
0x141e9dc87 mov rax, qword ptr [rbp - 0x18]
0x141e9dc8b inc qword ptr [rax]
0x141e9dc8e add rsp, 0x40
0x141e9dc92 pop rbx
0x141e9dc93 pop rdi
0x141e9dc94 pop rsi
0x141e9dc95 pop r12
0x141e9dc97 pop r14
0x141e9dc99 pop r15
0x141e9dc9b pop rbp
0x141e9dc9c ret 
0x141e9dc9d nop dword ptr [rax]
0x141e9dca0 mov qword ptr [rsp + 0x10], rdx
0x141e9dca5 push rbp
0x141e9dca6 push r15
0x141e9dca8 push r14
0x141e9dcaa push r12
0x141e9dcac push rsi
0x141e9dcad push rdi
0x141e9dcae push rbx
0x141e9dcaf sub rsp, 0x40
0x141e9dcb3 lea rbp, [rdx + 0x70]
0x141e9dcb7 cmp byte ptr [rbp - 0x10], 0
0x141e9dcbb je 0x141e9dcdd
0x141e9dcbd lea rax, [rbp - 0x30]
0x141e9dcc1 mov qword ptr [rbp - 0x20], rax
0x141e9dcc5 lea rax, [rbp - 0x20]
0x141e9dcc9 mov qword ptr [rbp - 0x28], rax
0x141e9dccd lea rcx, [rip + 0x10f776c]
0x141e9dcd4 lea rdx, [rbp - 0x28]
0x141e9dcd8 call 0x140fcfc20
0x141e9dcdd nop 
0x141e9dcde add rsp, 0x40
0x141e9dce2 pop rbx
0x141e9dce3 pop rdi
0x141e9dce4 pop rsi
0x141e9dce5 pop r12
0x141e9dce7 pop r14
0x141e9dce9 pop r15
0x141e9dceb pop rbp
0x141e9dcec ret 
0x141e9dced nop dword ptr [rax]
0x141e9dcf0 mov qword ptr [rsp + 0x10], rdx
0x141e9dcf5 push rbp
0x141e9dcf6 push r15
0x141e9dcf8 push r14
0x141e9dcfa push r12
0x141e9dcfc push rsi
0x141e9dcfd push rdi
0x141e9dcfe push rbx
0x141e9dcff sub rsp, 0x40
0x141e9dd03 lea rbp, [rdx + 0x70]
0x141e9dd07 mov rax, qword ptr [rbp - 0x10]
0x141e9dd0b inc qword ptr [rax]
0x141e9dd0e add rsp, 0x40
0x141e9dd12 pop rbx
0x141e9dd13 pop rdi
0x141e9dd14 pop rsi
0x141e9dd15 pop r12
0x141e9dd17 pop r14
0x141e9dd19 pop r15
0x141e9dd1b pop rbp
0x141e9dd1c ret 