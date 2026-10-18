0x140f9e970 push rbp
0x140f9e971 push r15
0x140f9e973 push r14
0x140f9e975 push r13
0x140f9e977 push r12
0x140f9e979 push rsi
0x140f9e97a push rdi
0x140f9e97b push rbx
0x140f9e97c sub rsp, 0x108
0x140f9e983 lea rbp, [rsp + 0x80]
0x140f9e98b movdqa xmmword ptr [rbp + 0x70], xmm8
0x140f9e991 movdqa xmmword ptr [rbp + 0x60], xmm7
0x140f9e996 movdqa xmmword ptr [rbp + 0x50], xmm6
0x140f9e99b mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x140f9e9a3 mov qword ptr [rbp - 0x40], r8
0x140f9e9a7 mov qword ptr [rbp - 0x38], r9
0x140f9e9ab mov rsi, qword ptr [rdx]
0x140f9e9ae mov rax, qword ptr [rsi + 0x248]
0x140f9e9b5 mov qword ptr [rbp + 0x40], rax
0x140f9e9b9 test rax, rax
0x140f9e9bc je 0x140f9eb70
0x140f9e9c2 mov rdi, r9
0x140f9e9c5 mov r14, rdx
0x140f9e9c8 mov rbx, rcx
0x140f9e9cb lea rcx, [rsi + 0x250]
0x140f9e9d2 mov qword ptr [rbp], r8
0x140f9e9d6 mov rdx, r8
0x140f9e9d9 mov r8, r9
0x140f9e9dc call 0x1405b1a70
0x140f9e9e1 mov r12, qword ptr [rsi + 0x230]
0x140f9e9e8 mov r15, qword ptr [rsi + 0x238]
0x140f9e9ef mov r13, r15
0x140f9e9f2 and r13, rax
0x140f9e9f5 shr rax, 0x39
0x140f9e9f9 movd xmm0, eax
0x140f9e9fd punpcklbw xmm0, xmm0
0x140f9ea01 pshuflw xmm0, xmm0, 0
0x140f9ea06 pshufd xmm6, xmm0, 0
0x140f9ea0b lea rdx, [r12 - 0x20]
0x140f9ea10 xor ecx, ecx
0x140f9ea12 pcmpeqd xmm7, xmm7
0x140f9ea16 movdqu xmm8, xmmword ptr [r12 + r13]
0x140f9ea1c movdqa xmm0, xmm8
0x140f9ea21 pcmpeqb xmm0, xmm6
0x140f9ea25 pmovmskb esi, xmm0
0x140f9ea29 test esi, esi
0x140f9ea2b je 0x140f9ea80
0x140f9ea2d mov qword ptr [rbp + 8], rbx
0x140f9ea31 mov qword ptr [rbp - 0x10], rdx
0x140f9ea35 mov qword ptr [rbp - 8], rcx
0x140f9ea39 tzcnt ebx, esi
0x140f9ea3d add rbx, r13
0x140f9ea40 and rbx, r15
0x140f9ea43 mov rcx, rbx
0x140f9ea46 shl rcx, 5
0x140f9ea4a mov rax, rdx
0x140f9ea4d sub rax, rcx
0x140f9ea50 cmp qword ptr [rax + 0x10], rdi
0x140f9ea54 jne 0x140f9ea6a
0x140f9ea56 mov rdx, qword ptr [rax + 8]
0x140f9ea5a mov rcx, qword ptr [rbp]
0x140f9ea5e mov r8, rdi
0x140f9ea61 call 0x142923bf0
0x140f9ea66 test eax, eax
0x140f9ea68 je 0x140f9eaa5
0x140f9ea6a lea eax, [rsi - 1]
0x140f9ea6d and ax, si
0x140f9ea70 mov esi, eax
0x140f9ea72 mov rbx, qword ptr [rbp + 8]
0x140f9ea76 mov rdx, qword ptr [rbp - 0x10]
0x140f9ea7a mov rcx, qword ptr [rbp - 8]
0x140f9ea7e jne 0x140f9ea39
0x140f9ea80 pcmpeqb xmm8, xmm7
0x140f9ea85 pmovmskb eax, xmm8
0x140f9ea8a test eax, eax
0x140f9ea8c jne 0x140f9eb7e
0x140f9ea92 add r13, rcx
0x140f9ea95 add r13, 0x10
0x140f9ea99 add rcx, 0x10
0x140f9ea9d and r13, r15
0x140f9eaa0 jmp 0x140f9ea16
0x140f9eaa5 neg rbx
0x140f9eaa8 shl rbx, 5
0x140f9eaac mov rsi, qword ptr [r12 + rbx - 8]
0x140f9eab1 lock inc qword ptr [rsi]
0x140f9eab5 jle 0x140f9ec21
0x140f9eabb mov qword ptr [rbp - 0x30], rsi
0x140f9eabf mov r15, qword ptr [r14 + 8]
0x140f9eac3 mov rax, qword ptr [rbp + 0xf0]
0x140f9eaca mov qword ptr [rbp + 0x10], rax
0x140f9eace movabs rax, 0x41223169ff28813b
0x140f9ead8 mov qword ptr [rbp + 0x18], rax
0x140f9eadc movabs rax, 0xa79b7268a2a968d9
0x140f9eae6 mov qword ptr [rbp + 0x20], rax
0x140f9eaea lea rcx, [rbp + 0x10]
0x140f9eaee call 0x1405bdf20
0x140f9eaf3 mov qword ptr [rbp + 0x10], r15
0x140f9eaf7 mov qword ptr [rbp + 0x18], rax
0x140f9eafb lea rcx, [rbp + 0x10]
0x140f9eaff call 0x1405bd910
0x140f9eb04 mov qword ptr [rbp + 0x40], rsi
0x140f9eb08 lea r9, [rsi + 0x10]
0x140f9eb0c mov rcx, r14
0x140f9eb0f mov rdx, r15
0x140f9eb12 mov r8, rax
0x140f9eb15 call 0x14068eb50
0x140f9eb1a mov rsi, qword ptr [rbp + 8]
0x140f9eb1e mov rax, qword ptr [r14 + 0x10]
0x140f9eb22 mov qword ptr [rsi + 0x20], rax
0x140f9eb26 movups xmm0, xmmword ptr [r14]
0x140f9eb2a movups xmmword ptr [rsi + 0x10], xmm0
0x140f9eb2e mov rax, qword ptr [rbp]
0x140f9eb32 mov qword ptr [rsi], rax
0x140f9eb35 mov qword ptr [rsi + 8], rdi
0x140f9eb39 mov rax, qword ptr [rbp + 0x40]
0x140f9eb3d lock dec qword ptr [rax]
0x140f9eb41 jne 0x140f9eb4c
0x140f9eb43 lea rcx, [rbp - 0x30]
0x140f9eb47 call 0x14068d180
0x140f9eb4c mov rax, rsi
0x140f9eb4f movaps xmm6, xmmword ptr [rbp + 0x50]
0x140f9eb53 movaps xmm7, xmmword ptr [rbp + 0x60]
0x140f9eb57 movaps xmm8, xmmword ptr [rbp + 0x70]
0x140f9eb5c add rsp, 0x108
0x140f9eb63 pop rbx
0x140f9eb64 pop rdi
0x140f9eb65 pop rsi
0x140f9eb66 pop r12
0x140f9eb68 pop r13
0x140f9eb6a pop r14
0x140f9eb6c pop r15
0x140f9eb6e pop rbp
0x140f9eb6f ret 
0x140f9eb70 mov r12, qword ptr [rsi + 0x230]
0x140f9eb77 mov r15, qword ptr [rsi + 0x238]
0x140f9eb7e lea rax, [r12 + r15]
0x140f9eb82 inc rax
0x140f9eb85 movdqa xmm0, xmmword ptr [r12]
0x140f9eb8b pmovmskb ecx, xmm0
0x140f9eb8f not ecx
0x140f9eb91 mov word ptr [rbp + 0x28], cx
0x140f9eb95 mov qword ptr [rbp + 0x10], r12
0x140f9eb99 add r12, 0x10
0x140f9eb9d mov qword ptr [rbp + 0x18], r12
0x140f9eba1 mov qword ptr [rbp + 0x20], rax
0x140f9eba5 mov rax, qword ptr [rbp + 0x40]
0x140f9eba9 mov qword ptr [rbp + 0x30], rax
0x140f9ebad lea r8, [rip + 0x1c10224] ; 0x142baedd8 80edba42010000005800000000000000 f32=93.4638671875
0x140f9ebb4 lea rsi, [rbp - 0x58]
0x140f9ebb8 lea rdx, [rbp + 0x10]
0x140f9ebbc mov rcx, rsi
0x140f9ebbf call 0x140fcee70
0x140f9ebc4 lea rax, [rbp - 0x40]
0x140f9ebc8 mov qword ptr [rbp - 0x30], rax
0x140f9ebcc lea rax, [rip - 0xa6b333] ; 0x1405338a0 4989d0488b01488b51084889c1e9ae4a f32=427082.28125
0x140f9ebd3 mov qword ptr [rbp - 0x28], rax
0x140f9ebd7 mov qword ptr [rbp - 0x20], rsi
0x140f9ebdb lea rax, [rip + 0x2f15e] ; 0x140fcdd40 415741565657534883ec404c8b71084c f32=53145124208640.0
0x140f9ebe2 mov qword ptr [rbp - 0x18], rax
0x140f9ebe6 lea rax, [rip + 0x1c11bd3] ; 0x142bb07c0 9e07bb42010000001500000000000000 f32=93.51487731933594
0x140f9ebed mov qword ptr [rbp + 0x10], rax
0x140f9ebf1 mov qword ptr [rbp + 0x18], 2
0x140f9ebf9 mov qword ptr [rbp + 0x30], 0
0x140f9ec01 lea rax, [rbp - 0x30]
0x140f9ec05 mov qword ptr [rbp + 0x20], rax
0x140f9ec09 mov qword ptr [rbp + 0x28], 2
0x140f9ec11 lea rdx, [rip + 0x1c11bc8] ; 0x142bb07e0 7607bb42010000002800000000000000 f32=93.51457214355469
0x140f9ec18 lea rcx, [rbp + 0x10]
0x140f9ec1c call 0x1428d9390
0x140f9ec21 ud2 
0x140f9ec23 nop word ptr cs:[rax + rax]
0x140f9ec30 mov qword ptr [rsp + 0x10], rdx
0x140f9ec35 push rbp
0x140f9ec36 push r15
0x140f9ec38 push r14
0x140f9ec3a push r13
0x140f9ec3c push r12
0x140f9ec3e push rsi
0x140f9ec3f push rdi
0x140f9ec40 push rbx
0x140f9ec41 sub rsp, 0x58
0x140f9ec45 lea rbp, [rdx + 0x80]
0x140f9ec4c movdqa xmmword ptr [rsp + 0x20], xmm8
0x140f9ec53 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140f9ec59 movdqa xmmword ptr [rsp + 0x40], xmm6
0x140f9ec5f mov rdx, qword ptr [rbp - 0x58]
0x140f9ec63 test rdx, rdx
0x140f9ec66 je 0x140f9ec7b
0x140f9ec68 mov rcx, qword ptr [rbp - 0x50]
0x140f9ec6c shl rdx, 3
0x140f9ec70 mov r8d, 8
0x140f9ec76 call 0x140613c20
0x140f9ec7b movaps xmm6, xmmword ptr [rsp + 0x40]
0x140f9ec80 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140f9ec85 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140f9ec8b add rsp, 0x58
0x140f9ec8f pop rbx
0x140f9ec90 pop rdi
0x140f9ec91 pop rsi
0x140f9ec92 pop r12
0x140f9ec94 pop r13
0x140f9ec96 pop r14
0x140f9ec98 pop r15
0x140f9ec9a pop rbp
0x140f9ec9b ret 
0x140f9ec9c nop dword ptr [rax]
0x140f9eca0 mov qword ptr [rsp + 0x10], rdx
0x140f9eca5 push rbp
0x140f9eca6 push r15
0x140f9eca8 push r14
0x140f9ecaa push r13
0x140f9ecac push r12
0x140f9ecae push rsi
0x140f9ecaf push rdi
0x140f9ecb0 push rbx
0x140f9ecb1 sub rsp, 0x58
0x140f9ecb5 lea rbp, [rdx + 0x80]
0x140f9ecbc movdqa xmmword ptr [rsp + 0x20], xmm8
0x140f9ecc3 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140f9ecc9 movdqa xmmword ptr [rsp + 0x40], xmm6
0x140f9eccf mov rax, qword ptr [rbp + 0x40]
0x140f9ecd3 lock dec qword ptr [rax]
0x140f9ecd7 jne 0x140f9ece2
0x140f9ecd9 lea rcx, [rbp - 0x30]
0x140f9ecdd call 0x14068d180
0x140f9ece2 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140f9ece7 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140f9ecec movaps xmm8, xmmword ptr [rsp + 0x20]
0x140f9ecf2 add rsp, 0x58
0x140f9ecf6 pop rbx
0x140f9ecf7 pop rdi
0x140f9ecf8 pop rsi
0x140f9ecf9 pop r12
0x140f9ecfb pop r13
0x140f9ecfd pop r14
0x140f9ecff pop r15
0x140f9ed01 pop rbp
0x140f9ed02 ret 