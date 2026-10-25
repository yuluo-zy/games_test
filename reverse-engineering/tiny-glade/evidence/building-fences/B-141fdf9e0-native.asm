141fdf9e0 push rbp
141fdf9e1 push r15
141fdf9e3 push r14
141fdf9e5 push r13
141fdf9e7 push r12
141fdf9e9 push rsi
141fdf9ea push rdi
141fdf9eb push rbx
141fdf9ec sub rsp, 0x128
141fdf9f3 lea rbp, [rsp + 0x80]
141fdf9fb movaps xmmword ptr [rbp + 0x90], xmm6
141fdfa02 mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
141fdfa0d movaps xmm0, xmm3
141fdfa10 mov rdi, r8
141fdfa13 mov rbx, rdx
141fdfa16 mov qword ptr [rbp - 8], rcx
141fdfa1a call 0x142923e80
141fdfa1f movaps xmm6, xmm0
141fdfa22 movzx eax, byte ptr [rip + 0x16551e0]
141fdfa29 mov ecx, 8
141fdfa2e mov edx, 8
141fdfa33 call 0x140613c10
141fdfa38 test rax, rax
141fdfa3b je 0x141fdfd09
141fdfa41 mov qword ptr [rax], 0
141fdfa48 mov qword ptr [rbp + 0x60], 1
141fdfa50 mov qword ptr [rbp + 0x68], rax
141fdfa54 mov qword ptr [rbp + 0x70], 1
141fdfa5c mov qword ptr [rbp], rbx
141fdfa60 mov qword ptr [rbp + 8], rdi
141fdfa64 mov qword ptr [rbp + 0x18], rdi
141fdfa68 lea r13, [rbp + 0x20]
141fdfa6c mov qword ptr [rbp + 0x20], 0
141fdfa74 mov qword ptr [rbp + 0x38], 0
141fdfa7c xor ecx, ecx
141fdfa7e mov rsi, rbp
141fdfa81 lea rbx, [rbp - 0x50]
141fdfa85 lea r14, [rbp - 0x20]
141fdfa89 mov rax, rdi
141fdfa8c jmp 0x141fdfaa7
141fdfa8e nop 
141fdfa90 mov rax, qword ptr [rbp + 0x68]
141fdfa94 mov qword ptr [rax + r12*8], r15
141fdfa98 inc r12
141fdfa9b mov qword ptr [rbp + 0x70], r12
141fdfa9f mov rcx, qword ptr [rbp + 0x10]
141fdfaa3 mov rax, qword ptr [rbp + 0x18]
141fdfaa7 cmp rcx, rax
141fdfaaa jae 0x141fdfc53
141fdfab0 lea rax, [rcx + 1]
141fdfab4 mov qword ptr [rbp + 0x10], rax
141fdfab8 mov rdx, qword ptr [rbp + 8]
141fdfabc cmp rdx, rcx
141fdfabf jbe 0x141fdfcfb
141fdfac5 lea rax, [rcx + rcx*2]
141fdfac9 shl rax, 2
141fdfacd add rax, qword ptr [rbp]
141fdfad1 cmp qword ptr [rbp + 0x20], 0
141fdfad6 je 0x141fdfaf0
141fdfad8 mov rcx, qword ptr [rbp + 0x28]
141fdfadc mov rdx, qword ptr [rbp + 0x30]
141fdfae0 mov qword ptr [rbp + 0x30], rax
141fdfae4 mov qword ptr [rbp + 0x28], rdx
141fdfae8 mov qword ptr [rbp + 0x20], rcx
141fdfaec jmp 0x141fdfb31
141fdfaee nop 
141fdfaf0 mov qword ptr [rbp - 0x20], 1
141fdfaf8 mov qword ptr [rbp - 0x18], rax
141fdfafc mov qword ptr [rbp - 0x10], rsi
141fdfb00 mov rcx, rbx
141fdfb03 mov rdx, r14
141fdfb06 call 0x14123ac50
141fdfb0b mov rax, qword ptr [rbp - 0x40]
141fdfb0f mov qword ptr [r13 + 0x10], rax
141fdfb13 movups xmm0, xmmword ptr [rbp - 0x50]
141fdfb17 movups xmmword ptr [r13], xmm0
141fdfb1c mov rcx, qword ptr [rbp + 0x20]
141fdfb20 test rcx, rcx
141fdfb23 je 0x141fdfc53
141fdfb29 mov rdx, qword ptr [rbp + 0x28]
141fdfb2d mov rax, qword ptr [rbp + 0x30]
141fdfb31 mov r15, qword ptr [rbp + 0x38]
141fdfb35 inc r15
141fdfb38 mov qword ptr [rbp + 0x38], r15
141fdfb3c mov r8d, dword ptr [rcx + 8]
141fdfb40 mov dword ptr [rbp + 0x58], r8d
141fdfb44 mov rcx, qword ptr [rcx]
141fdfb47 mov qword ptr [rbp + 0x50], rcx
141fdfb4b mov ecx, dword ptr [rdx + 8]
141fdfb4e mov dword ptr [rbp + 0x80], ecx
141fdfb54 mov rcx, qword ptr [rdx]
141fdfb57 mov qword ptr [rbp + 0x78], rcx
141fdfb5b mov rcx, qword ptr [rax]
141fdfb5e mov qword ptr [rbp + 0x40], rcx
141fdfb62 mov eax, dword ptr [rax + 8]
141fdfb65 mov dword ptr [rbp + 0x48], eax
141fdfb68 movss xmm1, dword ptr [rbp + 0x50]
141fdfb6d movss xmm0, dword ptr [rbp + 0x78]
141fdfb72 ucomiss xmm1, xmm0
141fdfb75 jne 0x141fdfb9a
141fdfb77 jp 0x141fdfb9a
141fdfb79 movss xmm2, dword ptr [rbp + 0x54]
141fdfb7e ucomiss xmm2, dword ptr [rbp + 0x7c]
141fdfb82 jne 0x141fdfb9a
141fdfb84 jp 0x141fdfb9a
141fdfb86 movss xmm2, dword ptr [rbp + 0x58]
141fdfb8b ucomiss xmm2, dword ptr [rbp + 0x80]
141fdfb92 jne 0x141fdfb9a
141fdfb94 jnp 0x141fdfca9
141fdfb9a movss xmm2, dword ptr [rbp + 0x40]
141fdfb9f ucomiss xmm0, xmm2
141fdfba2 jne 0x141fdfbd0
141fdfba4 jp 0x141fdfbd0
141fdfba6 movss xmm4, dword ptr [rbp + 0x7c]
141fdfbab movss xmm3, dword ptr [rbp + 0x80]
141fdfbb3 ucomiss xmm4, dword ptr [rbp + 0x44]
141fdfbb7 movss xmm4, dword ptr [rbp + 0x48]
141fdfbbc jne 0x141fdfbdd
141fdfbbe jp 0x141fdfbdd
141fdfbc0 ucomiss xmm3, xmm4
141fdfbc3 jne 0x141fdfbdd
141fdfbc5 jp 0x141fdfbdd
141fdfbc7 jmp 0x141fdfcd2
141fdfbcc nop dword ptr [rax]
141fdfbd0 movss xmm3, dword ptr [rbp + 0x80]
141fdfbd8 movss xmm4, dword ptr [rbp + 0x48]
141fdfbdd subss xmm4, xmm3
141fdfbe1 subss xmm3, dword ptr [rbp + 0x58]
141fdfbe6 subss xmm2, xmm0
141fdfbea subss xmm0, xmm1
141fdfbee movaps xmm1, xmm0
141fdfbf1 mulss xmm1, xmm2
141fdfbf5 movaps xmm5, xmm4
141fdfbf8 mulss xmm5, xmm3
141fdfbfc addss xmm5, xmm1
141fdfc00 mulss xmm0, xmm0
141fdfc04 mulss xmm3, xmm3
141fdfc08 addss xmm3, xmm0
141fdfc0c mulss xmm2, xmm2
141fdfc10 mulss xmm4, xmm4
141fdfc14 addss xmm4, xmm2
141fdfc18 mulss xmm4, xmm3
141fdfc1c xorps xmm0, xmm0
141fdfc1f sqrtss xmm0, xmm4
141fdfc23 divss xmm5, xmm0
141fdfc27 ucomiss xmm6, xmm5
141fdfc2a jbe 0x141fdfa9f
141fdfc30 mov r12, qword ptr [rbp + 0x70]
141fdfc34 cmp r12, qword ptr [rbp + 0x60]
141fdfc38 jne 0x141fdfa90
141fdfc3e lea rcx, [rbp + 0x60]
141fdfc42 lea rdx, [rip + 0xfeb4ff]
141fdfc49 call 0x140b81df0
141fdfc4e jmp 0x141fdfa90
141fdfc53 mov rsi, qword ptr [rbp + 0x70]
141fdfc57 cmp rsi, qword ptr [rbp + 0x60]
141fdfc5b jne 0x141fdfc6d
141fdfc5d lea rdx, [rip + 0xfeb4cc]
141fdfc64 lea rcx, [rbp + 0x60]
141fdfc68 call 0x140b81df0
141fdfc6d dec rdi
141fdfc70 mov rax, qword ptr [rbp + 0x68]
141fdfc74 mov qword ptr [rax + rsi*8], rdi
141fdfc78 inc rsi
141fdfc7b mov qword ptr [rbp + 0x70], rsi
141fdfc7f mov rax, qword ptr [rbp - 8]
141fdfc83 mov qword ptr [rax + 0x10], rsi
141fdfc87 movups xmm0, xmmword ptr [rbp + 0x60]
141fdfc8b movups xmmword ptr [rax], xmm0
141fdfc8e movaps xmm6, xmmword ptr [rbp + 0x90]
141fdfc95 add rsp, 0x128
141fdfc9c pop rbx
141fdfc9d pop rdi
141fdfc9e pop rsi
141fdfc9f pop r12
141fdfca1 pop r13
141fdfca3 pop r14
141fdfca5 pop r15
141fdfca7 pop rbp
141fdfca8 ret 
141fdfca9 mov qword ptr [rbp - 0x50], 0
141fdfcb1 lea rax, [rip + 0xfeb4c0]
141fdfcb8 mov qword ptr [rsp + 0x20], rax
141fdfcbd lea rdx, [rbp + 0x50]
141fdfcc1 lea r8, [rbp + 0x78]
141fdfcc5 lea r9, [rbp - 0x50]
141fdfcc9 mov cl, 1
141fdfccb call 0x142914b5a
141fdfcd0 jmp 0x141fdfd07
141fdfcd2 mov qword ptr [rbp - 0x50], 0
141fdfcda lea rax, [rip + 0xfeb47f]
141fdfce1 mov qword ptr [rsp + 0x20], rax
141fdfce6 lea rdx, [rbp + 0x78]
141fdfcea lea r8, [rbp + 0x40]
141fdfcee lea r9, [rbp - 0x50]
141fdfcf2 mov cl, 1
141fdfcf4 call 0x142914b5a
141fdfcf9 jmp 0x141fdfd07
141fdfcfb lea r8, [rip + 0xc3f11e]
141fdfd02 call 0x1428d9518
141fdfd07 ud2 
141fdfd09 mov ecx, 8
141fdfd0e mov edx, 8
141fdfd13 call 0x1428d9000
141fdfd18 int3 
