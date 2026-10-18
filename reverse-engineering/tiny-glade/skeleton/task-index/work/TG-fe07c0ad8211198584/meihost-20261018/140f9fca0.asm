0x140f9fca0 push r15
0x140f9fca2 push r14
0x140f9fca4 push rsi
0x140f9fca5 push rdi
0x140f9fca6 push rbx
0x140f9fca7 sub rsp, 0x60
0x140f9fcab movaps xmmword ptr [rsp + 0x50], xmm6
0x140f9fcb0 mov rbx, rdx
0x140f9fcb3 mov rdi, rcx
0x140f9fcb6 mov rsi, qword ptr [rsp + 0xb8]
0x140f9fcbe mov qword ptr [rsp + 0x40], r8
0x140f9fcc3 mov qword ptr [rsp + 0x48], r9
0x140f9fcc8 lea rcx, [rsp + 0x40]
0x140f9fccd call 0x1405bdda0
0x140f9fcd2 mov qword ptr [rsp + 0x28], rax
0x140f9fcd7 movabs rax, 0x12ddabfdd3347ef1
0x140f9fce1 mov qword ptr [rsp + 0x30], rax
0x140f9fce6 movabs rax, 0x1f6d60c8858b8ff
0x140f9fcf0 mov qword ptr [rsp + 0x38], rax
0x140f9fcf5 lea rcx, [rsp + 0x28]
0x140f9fcfa call 0x1405bdf20
0x140f9fcff mov rcx, qword ptr [rbx + 8]
0x140f9fd03 mov qword ptr [rsp + 0x28], rcx
0x140f9fd08 mov qword ptr [rsp + 0x30], rax
0x140f9fd0d lea rcx, [rsp + 0x28]
0x140f9fd12 call 0x1405bd910
0x140f9fd17 lea r14, [rsp + 0x28]
0x140f9fd1c mov rcx, r14
0x140f9fd1f mov rdx, rbx
0x140f9fd22 mov r8, rax
0x140f9fd25 call 0x1406861b0
0x140f9fd2a xor r15d, r15d
0x140f9fd2d mov rcx, r14
0x140f9fd30 xor edx, edx
0x140f9fd32 call 0x140f9e130
0x140f9fd37 test rsi, rsi
0x140f9fd3a js 0x140f9fddb
0x140f9fd40 mov rbx, rax
0x140f9fd43 je 0x140f9fd69
0x140f9fd45 movzx eax, byte ptr [rip + 0x2694ebd] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10 str=b'Z\x19\x030\x14'
0x140f9fd4c mov r15d, 1
0x140f9fd52 mov edx, 1
0x140f9fd57 mov rcx, rsi
0x140f9fd5a call 0x140613c10
0x140f9fd5f test rax, rax
0x140f9fd62 je 0x140f9fddb
0x140f9fd64 mov r14, rax
0x140f9fd67 jmp 0x140f9fd6f
0x140f9fd69 mov r14d, 1
0x140f9fd6f movss xmm6, dword ptr [rsp + 0xc0]
0x140f9fd78 mov rdx, qword ptr [rsp + 0xb0]
0x140f9fd80 mov rcx, r14
0x140f9fd83 mov r8, rsi
0x140f9fd86 call 0x142923c00
0x140f9fd8b mov rdx, qword ptr [rbx]
0x140f9fd8e test rdx, rdx
0x140f9fd91 je 0x140f9fda2
0x140f9fd93 mov rcx, qword ptr [rbx + 8]
0x140f9fd97 mov r8d, 1
0x140f9fd9d call 0x140613c20
0x140f9fda2 mov qword ptr [rbx], rsi
0x140f9fda5 mov qword ptr [rbx + 8], r14
0x140f9fda9 mov qword ptr [rbx + 0x10], rsi
0x140f9fdad movss dword ptr [rbx + 0x18], xmm6
0x140f9fdb2 mov byte ptr [rbx + 0x1c], 0
0x140f9fdb6 mov rax, qword ptr [rsp + 0x38]
0x140f9fdbb mov qword ptr [rdi + 0x10], rax
0x140f9fdbf movups xmm0, xmmword ptr [rsp + 0x28]
0x140f9fdc4 movups xmmword ptr [rdi], xmm0
0x140f9fdc7 mov rax, rdi
0x140f9fdca movaps xmm6, xmmword ptr [rsp + 0x50]
0x140f9fdcf add rsp, 0x60
0x140f9fdd3 pop rbx
0x140f9fdd4 pop rdi
0x140f9fdd5 pop rsi
0x140f9fdd6 pop r14
0x140f9fdd8 pop r15
0x140f9fdda ret 