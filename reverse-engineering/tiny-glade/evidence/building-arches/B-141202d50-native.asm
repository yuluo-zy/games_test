141202d50 push rbp
141202d51 push r15
141202d53 push r14
141202d55 push r13
141202d57 push r12
141202d59 push rsi
141202d5a push rdi
141202d5b push rbx
141202d5c sub rsp, 0xa8
141202d63 lea rbp, [rsp + 0x80]
141202d6b mov qword ptr [rbp + 0x20], 0xfffffffffffffffe
141202d73 mov rdi, rdx
141202d76 mov rsi, rcx
141202d79 mov r15, qword ptr [rdx]
141202d7c mov r12, qword ptr [rdx + 8]
141202d80 mov rax, r12
141202d83 sub rax, r15
141202d86 je 0x141202dce
141202d88 mov rbx, r8
141202d8b shr rax, 2
141202d8f movabs r13, 0xaaaaaaaaaaaaaaab
141202d99 imul r13, rax
141202d9d lea r14, [r13*4]
141202da5 movzx eax, byte ptr [rip + 0x2431e5d]
141202dac mov edx, 4
141202db1 mov rcx, r14
141202db4 call 0x140613c10
141202db9 test rax, rax
141202dbc jne 0x141202dd6
141202dbe mov ecx, 4
141202dc3 mov rdx, r14
141202dc6 mov r8, rbx
141202dc9 call 0x1428d8fe3
141202dce xor r13d, r13d
141202dd1 mov eax, 4
141202dd6 mov qword ptr [rbp + 8], r13
141202dda mov qword ptr [rbp + 0x10], rax
141202dde lea rcx, [rbp + 0x18]
141202de2 mov qword ptr [rbp + 0x18], 0
141202dea mov rdx, qword ptr [rdi + 0x40]
141202dee mov qword ptr [rbp - 0x18], rdx
141202df2 movups xmm0, xmmword ptr [rdi + 0x10]
141202df6 movups xmm1, xmmword ptr [rdi + 0x20]
141202dfa movups xmm2, xmmword ptr [rdi + 0x30]
141202dfe movups xmmword ptr [rbp - 0x28], xmm2
141202e02 movups xmmword ptr [rbp - 0x38], xmm1
141202e06 movups xmmword ptr [rbp - 0x48], xmm0
141202e0a mov qword ptr [rbp - 0x58], r15
141202e0e mov qword ptr [rbp - 0x50], r12
141202e12 mov qword ptr [rbp - 0x10], rcx
141202e16 mov qword ptr [rbp - 8], 0
141202e1e mov qword ptr [rbp], rax
141202e22 lea rcx, [rbp - 0x58]
141202e26 lea rdx, [rbp - 0x10]
141202e2a call 0x1412083a0
141202e2f mov rax, qword ptr [rbp + 0x18]
141202e33 mov qword ptr [rsi + 0x10], rax
141202e37 movups xmm0, xmmword ptr [rbp + 8]
141202e3b movups xmmword ptr [rsi], xmm0
141202e3e mov rax, rsi
141202e41 add rsp, 0xa8
141202e48 pop rbx
141202e49 pop rdi
141202e4a pop rsi
141202e4b pop r12
141202e4d pop r13
141202e4f pop r14
141202e51 pop r15
141202e53 pop rbp
141202e54 ret 
