0x1416eca30 push rbp
0x1416eca31 push r15
0x1416eca33 push r14
0x1416eca35 push rsi
0x1416eca36 push rdi
0x1416eca37 push rbx
0x1416eca38 sub rsp, 0xb8
0x1416eca3f lea rbp, [rsp + 0x80]
0x1416eca47 movaps xmmword ptr [rbp + 0x20], xmm7
0x1416eca4b movaps xmmword ptr [rbp + 0x10], xmm6
0x1416eca4f mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x1416eca57 mov rsi, rcx
0x1416eca5a lea rcx, [rbp - 0x30]
0x1416eca5e call 0x141285b20
0x1416eca63 movzx eax, byte ptr [rbp - 0x30]
0x1416eca67 cmp al, 0x16
0x1416eca69 jne 0x1416eca7d
0x1416eca6b mov rdi, qword ptr [rbp - 0x28]
0x1416eca6f mov qword ptr [rsi + 8], rdi
0x1416eca73 mov eax, 1
0x1416eca78 jmp 0x1416ecc1c
0x1416eca7d mov ecx, dword ptr [rbp - 0x2f]
0x1416eca80 mov edx, dword ptr [rbp - 0x2c]
0x1416eca83 mov dword ptr [rbp - 0x4c], edx
0x1416eca86 mov dword ptr [rbp - 0x4f], ecx
0x1416eca89 mov rcx, qword ptr [rbp - 0x28]
0x1416eca8d movups xmm0, xmmword ptr [rbp - 0x20]
0x1416eca91 movups xmmword ptr [rbp - 0x40], xmm0
0x1416eca95 mov byte ptr [rbp - 0x50], al
0x1416eca98 mov qword ptr [rbp - 0x48], rcx
0x1416eca9c lea rdi, [rip + 0x15a31f5]
0x1416ecaa3 mov qword ptr [rsp + 0x20], rdi
0x1416ecaa8 mov qword ptr [rsp + 0x28], 2
0x1416ecab1 lea r8, [rip + 0x15a31c8]
0x1416ecab8 lea rcx, [rbp - 0x30]
0x1416ecabc lea rdx, [rbp - 0x50]
0x1416ecac0 mov r9d, 0x11
0x1416ecac6 call 0x141282e60
0x1416ecacb cmp dword ptr [rbp - 0x30], 1
0x1416ecacf jne 0x1416ecb09
0x1416ecad1 mov rcx, qword ptr [rbp - 0x28]
0x1416ecad5 mov rax, qword ptr [rcx]
0x1416ecad8 cmp rax, 1
0x1416ecadc mov qword ptr [rbp], rcx
0x1416ecae0 je 0x1416ecb55
0x1416ecae2 test rax, rax
0x1416ecae5 jne 0x1416ecb5e
0x1416ecae7 mov rax, qword ptr [rbp]
0x1416ecaeb mov rdx, qword ptr [rax + 0x10]
0x1416ecaef test rdx, rdx
0x1416ecaf2 je 0x1416ecb5e
0x1416ecaf4 mov rax, qword ptr [rbp]
0x1416ecaf8 mov rcx, qword ptr [rax + 8]
0x1416ecafc mov r8d, 1
0x1416ecb02 call 0x140613c20
0x1416ecb07 jmp 0x1416ecb5e
0x1416ecb09 mov r14d, dword ptr [rbp - 0x2c]
0x1416ecb0d mov rbx, qword ptr [rbp - 0x28]
0x1416ecb11 movsd xmm6, qword ptr [rbp - 0x20]
0x1416ecb16 mov r15, qword ptr [rbp - 0x18]
0x1416ecb1a lea rcx, [rbp - 0x50]
0x1416ecb1e call 0x1416e72c0
0x1416ecb23 lea rdi, [rsi + 4]
0x1416ecb27 cmp r14d, 2
0x1416ecb2b jne 0x1416ecb36
0x1416ecb2d movshdup xmm7, xmm6
0x1416ecb31 jmp 0x1416ecbfb
0x1416ecb36 mov qword ptr [rsi + 8], rbx
0x1416ecb3a movlps qword ptr [rsi + 0x10], xmm6
0x1416ecb3e test r14d, r14d
0x1416ecb41 je 0x1416ecc16
0x1416ecb47 mov qword ptr [rsi + 0x18], r15
0x1416ecb4b mov eax, 1
0x1416ecb50 jmp 0x1416ecc18
0x1416ecb55 add rcx, 8
0x1416ecb59 call 0x1405e7a00
0x1416ecb5e mov edx, 0x28
0x1416ecb63 mov r8d, 8
0x1416ecb69 mov rcx, qword ptr [rbp]
0x1416ecb6d call 0x140613c20
0x1416ecb72 mov qword ptr [rsp + 0x20], rdi
0x1416ecb77 mov qword ptr [rsp + 0x28], 2
0x1416ecb80 lea r8, [rip + 0x15a3131]
0x1416ecb87 lea rcx, [rbp - 0x30]
0x1416ecb8b lea rdx, [rbp - 0x50]
0x1416ecb8f mov r9d, 0x11
0x1416ecb95 call 0x141283000
0x1416ecb9a cmp dword ptr [rbp - 0x30], 2
0x1416ecb9e jne 0x1416ecbe0
0x1416ecba0 mov rcx, qword ptr [rbp - 0x28]
0x1416ecba4 mov rax, qword ptr [rcx]
0x1416ecba7 cmp rax, 1
0x1416ecbab mov qword ptr [rbp], rcx
0x1416ecbaf je 0x1416ecc39
0x1416ecbb5 test rax, rax
0x1416ecbb8 jne 0x1416ecc42
0x1416ecbbe mov rax, qword ptr [rbp]
0x1416ecbc2 mov rdx, qword ptr [rax + 0x10]
0x1416ecbc6 test rdx, rdx
0x1416ecbc9 je 0x1416ecc42
0x1416ecbcb mov rax, qword ptr [rbp]
0x1416ecbcf mov rcx, qword ptr [rax + 8]
0x1416ecbd3 mov r8d, 1
0x1416ecbd9 call 0x140613c20
0x1416ecbde jmp 0x1416ecc42
0x1416ecbe0 mov rbx, qword ptr [rbp - 0x30]
0x1416ecbe4 movss xmm6, dword ptr [rbp - 0x28]
0x1416ecbe9 movss xmm7, dword ptr [rbp - 0x24]
0x1416ecbee lea rcx, [rbp - 0x50]
0x1416ecbf2 call 0x1416e72c0
0x1416ecbf7 lea rdi, [rsi + 4]
0x1416ecbfb test ebx, ebx
0x1416ecbfd jne 0x1416ecc78
0x1416ecbff shr rbx, 0x20
0x1416ecc03 movd xmm1, ebx
0x1416ecc07 lea rcx, [rsi + 8]
0x1416ecc0b movaps xmm2, xmm6
0x1416ecc0e movaps xmm3, xmm7
0x1416ecc11 call 0x140c97540
0x1416ecc16 xor eax, eax
0x1416ecc18 mov dword ptr [rdi], eax
0x1416ecc1a xor eax, eax
0x1416ecc1c mov dword ptr [rsi], eax
0x1416ecc1e mov rax, rsi
0x1416ecc21 movaps xmm6, xmmword ptr [rbp + 0x10]
0x1416ecc25 movaps xmm7, xmmword ptr [rbp + 0x20]
0x1416ecc29 add rsp, 0xb8
0x1416ecc30 pop rbx
0x1416ecc31 pop rdi
0x1416ecc32 pop rsi
0x1416ecc33 pop r14
0x1416ecc35 pop r15
0x1416ecc37 pop rbp
0x1416ecc38 ret 
0x1416ecc39 add rcx, 8
0x1416ecc3d call 0x1405e7a00
0x1416ecc42 mov edx, 0x28
0x1416ecc47 mov r8d, 8
0x1416ecc4d mov rcx, qword ptr [rbp]
0x1416ecc51 call 0x140613c20
0x1416ecc56 lea rcx, [rip + 0x15a30a3]
0x1416ecc5d mov edx, 0x49
0x1416ecc62 call 0x1428fa000
0x1416ecc67 mov rdi, rax
0x1416ecc6a lea rcx, [rbp - 0x50]
0x1416ecc6e call 0x1416e72c0
0x1416ecc73 jmp 0x1416eca6f
0x1416ecc78 lea rax, [rip + 0x152cd31]
0x1416ecc7f mov qword ptr [rbp - 0x30], rax
0x1416ecc83 mov qword ptr [rbp - 0x28], 1
0x1416ecc8b mov qword ptr [rbp - 0x20], 8
0x1416ecc93 xorps xmm0, xmm0
0x1416ecc96 movups xmmword ptr [rbp - 0x18], xmm0
0x1416ecc9a lea rdx, [rip + 0x152cd1f]
0x1416ecca1 lea rcx, [rbp - 0x30]
0x1416ecca5 call 0x1428d9390
0x1416eccaa int3 
0x1416eccab nop dword ptr [rax + rax]
0x1416eccb0 mov qword ptr [rsp + 0x10], rdx
0x1416eccb5 push rbp
0x1416eccb6 push r15
0x1416eccb8 push r14
0x1416eccba push rsi
0x1416eccbb push rdi
0x1416eccbc push rbx
0x1416eccbd sub rsp, 0x58
0x1416eccc1 lea rbp, [rdx + 0x80]
0x1416eccc8 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ecccd movaps xmmword ptr [rsp + 0x40], xmm6
0x1416eccd2 mov edx, 0x28
0x1416eccd7 mov r8d, 8
0x1416eccdd mov rcx, qword ptr [rbp]
0x1416ecce1 call 0x140613c20
0x1416ecce6 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ecceb movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416eccf0 add rsp, 0x58
0x1416eccf4 pop rbx
0x1416eccf5 pop rdi
0x1416eccf6 pop rsi
0x1416eccf7 pop r14
0x1416eccf9 pop r15
0x1416eccfb pop rbp
0x1416eccfc ret 
0x1416eccfd nop dword ptr [rax]
0x1416ecd00 mov qword ptr [rsp + 0x10], rdx
0x1416ecd05 push rbp
0x1416ecd06 push r15
0x1416ecd08 push r14
0x1416ecd0a push rsi
0x1416ecd0b push rdi
0x1416ecd0c push rbx
0x1416ecd0d sub rsp, 0x58
0x1416ecd11 lea rbp, [rdx + 0x80]
0x1416ecd18 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ecd1d movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ecd22 mov edx, 0x28
0x1416ecd27 mov r8d, 8
0x1416ecd2d mov rcx, qword ptr [rbp]
0x1416ecd31 call 0x140613c20
0x1416ecd36 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ecd3b movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416ecd40 add rsp, 0x58
0x1416ecd44 pop rbx
0x1416ecd45 pop rdi
0x1416ecd46 pop rsi
0x1416ecd47 pop r14
0x1416ecd49 pop r15
0x1416ecd4b pop rbp
0x1416ecd4c ret 
0x1416ecd4d nop dword ptr [rax]
0x1416ecd50 mov qword ptr [rsp + 0x10], rdx
0x1416ecd55 push rbp
0x1416ecd56 push r15
0x1416ecd58 push r14
0x1416ecd5a push rsi
0x1416ecd5b push rdi
0x1416ecd5c push rbx
0x1416ecd5d sub rsp, 0x58
0x1416ecd61 lea rbp, [rdx + 0x80]
0x1416ecd68 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ecd6d movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ecd72 lea rcx, [rbp - 0x50]
0x1416ecd76 call 0x1416e72c0
0x1416ecd7b movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ecd80 movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416ecd85 add rsp, 0x58
0x1416ecd89 pop rbx
0x1416ecd8a pop rdi
0x1416ecd8b pop rsi
0x1416ecd8c pop r14
0x1416ecd8e pop r15
0x1416ecd90 pop rbp
0x1416ecd91 ret 
