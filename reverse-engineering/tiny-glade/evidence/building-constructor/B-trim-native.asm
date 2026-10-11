14123eef0 push rbp
14123eef1 push r15
14123eef3 push r14
14123eef5 push r13
14123eef7 push r12
14123eef9 push rsi
14123eefa push rdi
14123eefb push rbx
14123eefc sub rsp, 0x128
14123ef03 lea rbp, [rsp + 0x80]
14123ef0b movaps xmmword ptr [rbp + 0x90], xmm11
14123ef13 movaps xmmword ptr [rbp + 0x80], xmm10
14123ef1b movaps xmmword ptr [rbp + 0x70], xmm9
14123ef20 movaps xmmword ptr [rbp + 0x60], xmm8
14123ef25 movaps xmmword ptr [rbp + 0x50], xmm7
14123ef29 movaps xmmword ptr [rbp + 0x40], xmm6
14123ef2d mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
14123ef35 mov qword ptr [rbp + 0x28], r9
14123ef39 mov r14, r8
14123ef3c mov r15, rdx
14123ef3f mov qword ptr [rbp - 0x30], rcx
14123ef43 call 0x140c16a00
14123ef48 test al, al
14123ef4a mov byte ptr [rbp + 0x37], al
14123ef4d je 0x14123efab
14123ef4f mov edi, 0x34
14123ef54 mov ecx, 0x34
14123ef59 call 0x14123eac0
14123ef5e lea r12, [rip + 0x19e0363]
14123ef65 cmp rax, 1
14123ef69 jne 0x14123f016
14123ef6f mov r9, rdx
14123ef72 test rdx, rdx
14123ef75 je 0x14123efb4
14123ef77 cmp r9, 0x34
14123ef7b jae 0x14123efb2
14123ef7d lea rax, [rip + 0x19e0344]
14123ef84 cmp byte ptr [r9 + rax], 0xbf
14123ef89 jg 0x14123efb4
14123ef8b lea rax, [rip + 0x19e03ee]
14123ef92 mov qword ptr [rsp + 0x20], rax
14123ef97 lea rcx, [rip + 0x19e032a]
14123ef9e mov edx, 0x34
14123efa3 xor r8d, r8d
14123efa6 call 0x1428d9940
14123efab xor eax, eax
14123efad jmp 0x14123f168
14123efb2 jne 0x14123ef8b
14123efb4 mov rcx, r9
14123efb7 call 0x14123eac0
14123efbc cmp rax, 1
14123efc0 jne 0x14123f016
14123efc2 mov r8, rdx
14123efc5 add r8, 2
14123efc9 je 0x14123f004
14123efcb cmp r8, 0x34
14123efcf jae 0x14123f002
14123efd1 lea rax, [rip + 0x19e02f0]
14123efd8 cmp byte ptr [r8 + rax], 0xbf
14123efdd jg 0x14123f004
14123efdf lea rax, [rip + 0x19e03b2]
14123efe6 mov qword ptr [rsp + 0x20], rax
14123efeb lea rcx, [rip + 0x19e02d6]
14123eff2 mov edx, 0x34
14123eff7 mov r9d, 0x34
14123effd call 0x1428d9940
14123f002 jne 0x14123efdf
14123f004 mov edi, 0x32
14123f009 sub rdi, rdx
14123f00c lea r12, [rip + 0x19e02b5]
14123f013 add r12, r8
14123f016 lea r8, [rip + 0x19e03e1]
14123f01d lea r13, [rip + 0x19e03a4]
14123f024 nop word ptr cs:[rax + rax]
14123f030 movsx eax, byte ptr [r8 - 1]
14123f035 test eax, eax
14123f037 js 0x14123f050
14123f039 dec r8
14123f03c cmp eax, 0x5c
14123f03f jne 0x14123f0a4
14123f041 jmp 0x14123f0b5
14123f043 nop word ptr cs:[rax + rax]
14123f050 movzx ecx, byte ptr [r8 - 2]
14123f055 cmp cl, 0xc0
14123f058 jge 0x14123f07d
14123f05a movzx edx, byte ptr [r8 - 3]
14123f05f cmp dl, 0xc0
14123f062 jge 0x14123f086
14123f064 movzx r9d, byte ptr [r8 - 4]
14123f069 add r8, -4
14123f06d and r9d, 7
14123f071 shl r9d, 6
14123f075 and edx, 0x3f
14123f078 or edx, r9d
14123f07b jmp 0x14123f08d
14123f07d add r8, -2
14123f081 and ecx, 0x1f
14123f084 jmp 0x14123f095
14123f086 add r8, -3
14123f08a and edx, 0xf
14123f08d shl edx, 6
14123f090 and ecx, 0x3f
14123f093 or ecx, edx
14123f095 shl ecx, 6
14123f098 and al, 0x3f
14123f09a movzx eax, al
14123f09d or eax, ecx
14123f09f cmp eax, 0x5c
14123f0a2 je 0x14123f0b5
14123f0a4 cmp eax, 0x2f
14123f0a7 je 0x14123f0b5
14123f0a9 cmp r8, r13
14123f0ac jne 0x14123f030
14123f0ae mov esi, 0x36
14123f0b3 jmp 0x14123f101
14123f0b5 lea r13, [rip + 0x19e030c]
14123f0bc sub r8, r13
14123f0bf inc r8
14123f0c2 je 0x14123f0f6
14123f0c4 cmp r8, 0x36
14123f0c8 jae 0x14123f0f4
14123f0ca cmp byte ptr [r8 + r13], 0xbf
14123f0cf jg 0x14123f0f6
14123f0d1 lea rax, [rip + 0x19e0288]
14123f0d8 mov qword ptr [rsp + 0x20], rax
14123f0dd lea rcx, [rip + 0x19e02e4]
14123f0e4 mov edx, 0x36
14123f0e9 mov r9d, 0x36
14123f0ef call 0x1428d9940
14123f0f4 jne 0x14123f0d1
14123f0f6 mov esi, 0x36
14123f0fb sub rsi, r8
14123f0fe add r13, r8
14123f101 call 0x140c18130
14123f106 mov rcx, rax
14123f109 mov rax, qword ptr [rax]
14123f10c cmp rax, 1
14123f110 jne 0x14123f486
14123f116 add rcx, 8
14123f11a cmp qword ptr [rcx], 0
14123f11e jne 0x14123f80b
14123f124 mov qword ptr [rcx], 0xffffffffffffffff
14123f12b mov qword ptr [rbp - 0x18], rcx
14123f12f add rcx, 8
14123f133 mov qword ptr [rsp + 0x20], rsi
14123f138 mov qword ptr [rsp + 0x30], 0
14123f141 mov qword ptr [rsp + 0x28], 1
14123f14a mov rdx, r12
14123f14d mov r8, rdi
14123f150 mov r9, r13
14123f153 call 0x140c16d40
14123f158 mov rcx, qword ptr [rbp - 0x18]
14123f15c inc qword ptr [rcx]
14123f15f mov qword ptr [rbp - 0x20], rax
14123f163 mov eax, 1
14123f168 movss xmm7, dword ptr [rbp + 0x128]
14123f170 movss xmm6, dword ptr [rbp + 0x120]
14123f178 mov r12, qword ptr [rbp + 0x110]
14123f17f mov qword ptr [rbp - 0x28], rax
14123f183 mov qword ptr [rbp + 0x10], 0
14123f18b mov qword ptr [rbp + 0x18], 8
14123f193 mov qword ptr [rbp + 0x20], 0
14123f19b lea rsi, [r15 + r14*4]
14123f19f maxss xmm6, dword ptr [rip + 0x16e67dd]
14123f1a7 mov al, 1
14123f1a9 movss xmm8, dword ptr [rip + 0x16e66be]
14123f1b2 lea r13, [rbp + 0x10]
14123f1b6 lea r14, [rbp - 0x10]
14123f1ba cmp r15, rsi
14123f1bd je 0x14123f249
14123f1c3 nop word ptr cs:[rax + rax]
14123f1d0 lea rbx, [r15 + 4]
14123f1d4 test al, 1
14123f1d6 je 0x14123f1f0
14123f1d8 cmp rbx, rsi
14123f1db je 0x14123f249
14123f1dd mov rdi, rbx
14123f1e0 lea rbx, [r15 + 8]
14123f1e4 jmp 0x14123f1f6
14123f1e6 nop word ptr cs:[rax + rax]
14123f1f0 mov rdi, r15
14123f1f3 mov r15, rcx
14123f1f6 movss xmm2, dword ptr [r15]
14123f1fb movss xmm0, dword ptr [rdi]
14123f1ff mov rax, qword ptr [rbp + 0x20]
14123f203 mov rcx, qword ptr [rbp + 0x28]
14123f207 mov qword ptr [rbp - 0x10], rcx
14123f20b mov qword ptr [rbp], rax
14123f20f movss dword ptr [rbp + 8], xmm2
14123f214 addss xmm2, xmm0
14123f218 mulss xmm2, xmm8
14123f21d mulss xmm2, xmm6
14123f221 movss dword ptr [rbp + 0xc], xmm0
14123f226 mov qword ptr [rbp - 8], r13
14123f22a mov rcx, r12
14123f22d mov rdx, qword ptr [rbp + 0x118]
14123f234 mov r9, r14
14123f237 call 0x141fe0750
14123f23c xor eax, eax
14123f23e mov rcx, rdi
14123f241 mov r15, rbx
14123f244 cmp r15, rsi
14123f247 jne 0x14123f1d0
14123f249 movzx r15d, byte ptr [rbp + 0x130]
14123f251 test r15b, r15b
14123f254 je 0x14123f25f
14123f256 movss xmm8, dword ptr [rip + 0x178de09]
14123f25f mov rbx, qword ptr [rbp + 0x20]
14123f263 test rbx, rbx
14123f266 je 0x14123f3b9
14123f26c movss xmm6, dword ptr [rip + 0x17614f0]
14123f274 divss xmm6, xmm7
14123f278 mulss xmm8, xmm6
14123f27d mov rsi, qword ptr [rbp + 0x18]
14123f281 movss xmm9, dword ptr [rip + 0x16e667a]
14123f28a movaps xmm10, xmm9
14123f28e divss xmm10, xmm8
14123f293 mov rdi, rbx
14123f296 shl rdi, 5
14123f29a xor r14d, r14d
14123f29d xorps xmm11, xmm11
14123f2a1 jmp 0x14123f2b9
14123f2a3 nop word ptr cs:[rax + rax]
14123f2b0 add r14, 0x20
14123f2b4 cmp rdi, r14
14123f2b7 je 0x14123f323
14123f2b9 movss xmm0, dword ptr [rsi + r14 + 0x18]
14123f2c0 ucomiss xmm0, xmm11
14123f2c4 jne 0x14123f2c8
14123f2c6 jnp 0x14123f2ed
14123f2c8 divss xmm0, xmm8
14123f2cd call 0x142923fa0
14123f2d2 mulss xmm0, xmm8
14123f2d7 xorps xmm1, xmm1
14123f2da maxss xmm1, xmm0
14123f2de movaps xmm0, xmm9
14123f2e2 minss xmm0, xmm1
14123f2e6 movss dword ptr [rsi + r14 + 0x18], xmm0
14123f2ed movss xmm0, dword ptr [rsi + r14 + 0x1c]
14123f2f4 ucomiss xmm0, xmm9
14123f2f8 jne 0x14123f2fc
14123f2fa jnp 0x14123f2b0
14123f2fc mulss xmm0, xmm10
14123f301 call 0x142923fa0
14123f306 mulss xmm0, xmm8
14123f30b xorps xmm1, xmm1
14123f30e maxss xmm1, xmm0
14123f312 movaps xmm0, xmm9
14123f316 minss xmm0, xmm1
14123f31a movss dword ptr [rsi + r14 + 0x1c], xmm0
14123f321 jmp 0x14123f2b0
14123f323 test r15b, r15b
14123f326 je 0x14123f3cb
14123f32c test rbx, rbx
14123f32f je 0x14123f6cf
14123f335 movss xmm0, dword ptr [rip + 0x16e6533]
14123f33d mulss xmm0, xmm6
14123f341 xor eax, eax
14123f343 movss xmm1, dword ptr [rip + 0x16e6525]
14123f34b jmp 0x14123f35d
14123f34d nop dword ptr [rax]
14123f350 add rax, 0x20
14123f354 cmp rdi, rax
14123f357 je 0x14123f4b0
14123f35d movss xmm3, dword ptr [rsi + rax + 0x18]
14123f363 movss xmm2, dword ptr [rsi + rax + 0x1c]
14123f369 movaps xmm4, xmm2
14123f36c subss xmm4, xmm3
14123f370 ucomiss xmm6, xmm4
14123f373 jb 0x14123f350
14123f375 addss xmm3, xmm2
14123f379 mulss xmm3, xmm1
14123f37d subss xmm3, xmm0
14123f381 xorps xmm4, xmm4
14123f384 maxss xmm4, xmm3
14123f388 movaps xmm3, xmm9
14123f38c minss xmm3, xmm4
14123f390 movss dword ptr [rsi + rax + 0x18], xmm3
14123f396 addss xmm3, xmm2
14123f39a mulss xmm3, xmm1
14123f39e addss xmm3, xmm0
14123f3a2 xorps xmm2, xmm2
14123f3a5 maxss xmm2, xmm3
14123f3a9 movaps xmm3, xmm9
14123f3ad minss xmm3, xmm2
14123f3b1 movss dword ptr [rsi + rax + 0x1c], xmm3
14123f3b7 jmp 0x14123f350
14123f3b9 ucomiss xmm7, dword ptr [rip + 0x183deb0]
14123f3c0 seta al
14123f3c3 or al, r15b
14123f3c6 jmp 0x14123f6cf
14123f3cb ucomiss xmm7, dword ptr [rip + 0x183de9e]
14123f3d2 jbe 0x14123f4b0
14123f3d8 test rbx, rbx
14123f3db je 0x14123f6cf
14123f3e1 movss xmm0, dword ptr [rsi + 0x1c]
14123f3e6 subss xmm0, dword ptr [rsi + 0x18]
14123f3eb mov eax, 1
14123f3f0 ucomiss xmm0, xmm6
14123f3f3 mov edx, 1
14123f3f8 jbe 0x14123f41e
14123f3fa lea rcx, [rsi + 0x3c]
14123f3fe nop 
14123f400 cmp rbx, rdx
14123f403 je 0x14123f4a7
14123f409 movss xmm0, dword ptr [rcx]
14123f40d subss xmm0, dword ptr [rcx - 4]
14123f412 inc rdx
14123f415 add rcx, 0x20
14123f419 ucomiss xmm0, xmm6
14123f41c ja 0x14123f400
14123f41e mov rcx, rbx
14123f421 sub rcx, rdx
14123f424 je 0x14123f4a9
14123f42a shl rdx, 5
14123f42e add rsi, rdx
14123f431 mov eax, 1
14123f436 jmp 0x14123f44c
14123f438 nop dword ptr [rax + rax]
14123f440 inc rax
14123f443 add rsi, 0x20
14123f447 dec rcx
14123f44a je 0x14123f4a9
14123f44c movss xmm0, dword ptr [rsi + 0x1c]
14123f451 subss xmm0, dword ptr [rsi + 0x18]
14123f456 ucomiss xmm0, xmm6
14123f459 jbe 0x14123f440
14123f45b mov rdx, rax
14123f45e shl rdx, 5
14123f462 mov r8, rsi
14123f465 sub r8, rdx
14123f468 movups xmm0, xmmword ptr [rsi]
14123f46b movups xmm1, xmmword ptr [rsi + 0x10]
14123f46f neg rdx
14123f472 movups xmmword ptr [rsi + rdx + 0x10], xmm1
14123f477 movups xmmword ptr [r8], xmm0
14123f47b add rsi, 0x20
14123f47f dec rcx
14123f482 jne 0x14123f44c
14123f484 jmp 0x14123f4a9
14123f486 test rax, rax
14123f489 jne 0x14123f7ff
14123f48f xor edx, edx
14123f491 call 0x1428f15c0
14123f496 mov rcx, rax
14123f499 test rax, rax
14123f49c jne 0x14123f11a
14123f4a2 jmp 0x14123f7ff
14123f4a7 xor eax, eax
14123f4a9 sub rbx, rax
14123f4ac mov qword ptr [rbp + 0x20], rbx
14123f4b0 test rbx, rbx
14123f4b3 je 0x14123f6cf
14123f4b9 mov rdi, rbx
14123f4bc shl rdi, 4
14123f4c0 movzx eax, byte ptr [rip + 0x23f5742]
14123f4c7 mov edx, 8
14123f4cc mov rcx, rdi
14123f4cf call 0x140613c10
14123f4d4 test rax, rax
14123f4d7 je 0x14123f817
14123f4dd mov r15, qword ptr [rbp + 0x18]
14123f4e1 mov rcx, qword ptr [rbp + 0x20]
14123f4e5 mov qword ptr [rbp - 0x10], rbx
14123f4e9 mov qword ptr [rbp - 8], rax
14123f4ed mov qword ptr [rbp], 0
14123f4f5 test rcx, rcx
14123f4f8 je 0x14123f6df
14123f4fe shl rcx, 5
14123f502 add rcx, r15
14123f505 mov qword ptr [rbp - 0x18], rcx
14123f509 mov eax, 0x10
14123f50e mov r13d, 3
14123f514 xor esi, esi
14123f516 xor edi, edi
14123f518 jmp 0x14123f555
14123f51a nop word ptr [rax + rax]
14123f520 add r15, 0x20
14123f524 mov rax, qword ptr [rbp - 8]
14123f528 shl rdi, 4
14123f52c mov qword ptr [rax + rdi], rbx
14123f530 mov qword ptr [rax + rdi + 8], r14
14123f535 mov qword ptr [rbp], r12
14123f539 mov r9, qword ptr [rbp + 0x28]
14123f53d lea rax, [r9 + 0x10]
14123f541 add rsi, 0x20
14123f545 inc r13
14123f548 mov rdi, r12
14123f54b cmp r15, qword ptr [rbp - 0x18]
14123f54f je 0x14123f798
14123f555 mov r9, rax
14123f558 lea r12, [rdi + 1]
14123f55c mov rax, qword ptr [rbp + 0x20]
14123f560 cmp rax, r12
14123f563 mov qword ptr [rbp + 0x28], r9
14123f567 jbe 0x14123f5b0
14123f569 mov rcx, qword ptr [rbp + 0x18]
14123f56d mov r8, r12
14123f570 shl r8, 5
14123f574 lea rdx, [rcx + r8]
14123f578 movss xmm0, dword ptr [r15 + 0x14]
14123f57e ucomiss xmm0, dword ptr [rcx + r8 + 0x10]
14123f584 jne 0x14123f5f8
14123f586 jp 0x14123f5f8
14123f588 movss xmm3, dword ptr [rdx + 0x18]
14123f58d movss xmm2, dword ptr [rdx + 0x1c]
14123f592 movss xmm1, dword ptr [r15 + 0x18]
14123f598 ucomiss xmm1, xmm3
14123f59b jb 0x14123f5ba
14123f59d ucomiss xmm2, xmm1
14123f5a0 jb 0x14123f5ba
14123f5a2 lea r14, [rdi + 2]
14123f5a6 mov rbx, r12
14123f5a9 jmp 0x14123f5fd
14123f5ab nop dword ptr [rax + rax]
14123f5b0 xor ebx, ebx
14123f5b2 xor r14d, r14d
14123f5b5 jmp 0x14123f6b0
14123f5ba movss xmm4, dword ptr [r15 + 0x1c]
14123f5c0 ucomiss xmm4, xmm3
14123f5c3 setae r8b
14123f5c7 movaps xmm5, xmm1
14123f5ca cmpleps xmm5, xmm2
14123f5ce movaps xmm6, xmm2
14123f5d1 cmpleps xmm6, xmm4
14123f5d5 andps xmm6, xmm5
14123f5d8 movd r9d, xmm6
14123f5dd test r9b, 1
14123f5e1 jne 0x14123f5a2
14123f5e3 cmpleps xmm1, xmm3
14123f5e7 cmpleps xmm4, xmm2
14123f5eb orps xmm4, xmm1
14123f5ee movd r9d, xmm4
14123f5f3 and r8b, r9b
14123f5f6 jne 0x14123f5a2
14123f5f8 xor ebx, ebx
14123f5fa xor r14d, r14d
14123f5fd shl rax, 5
14123f601 lea r8, [rcx + rax]
14123f605 add rdx, 0x20
14123f609 cmp rdx, r8
14123f60c je 0x14123f6b0
14123f612 add rax, -0x40
14123f616 mov rdx, r13
14123f619 mov r8, rsi
14123f61c jmp 0x14123f63a
14123f61e nop 
14123f620 lea r9, [rdx - 1]
14123f624 cmp rbx, r14
14123f627 cmove rbx, r9
14123f62b mov r14, rdx
14123f62e add r8, 0x20
14123f632 inc rdx
14123f635 cmp rax, r8
14123f638 je 0x14123f6b0
14123f63a ucomiss xmm0, dword ptr [rcx + r8 + 0x50]
14123f640 jne 0x14123f62e
14123f642 jp 0x14123f62e
14123f644 movss xmm3, dword ptr [rcx + r8 + 0x58]
14123f64b movss xmm2, dword ptr [rcx + r8 + 0x5c]
14123f652 movss xmm1, dword ptr [r15 + 0x18]
14123f658 ucomiss xmm1, xmm3
14123f65b jb 0x14123f662
14123f65d ucomiss xmm2, xmm1
14123f660 jae 0x14123f620
14123f662 movss xmm4, dword ptr [r15 + 0x1c]
14123f668 ucomiss xmm4, xmm3
14123f66b setae r9b
14123f66f movaps xmm5, xmm1
14123f672 cmpleps xmm5, xmm2
14123f676 movaps xmm6, xmm2
14123f679 cmpleps xmm6, xmm4
14123f67d andps xmm6, xmm5
14123f680 movd r10d, xmm6
14123f685 test r10b, 1
14123f689 jne 0x14123f620
14123f68b cmpleps xmm1, xmm3
14123f68f cmpleps xmm4, xmm2
14123f693 orps xmm4, xmm1
14123f696 movd r10d, xmm4
14123f69b and r9b, r10b
14123f69e je 0x14123f62e
14123f6a0 jmp 0x14123f620
14123f6a5 nop word ptr cs:[rax + rax]
14123f6b0 cmp rdi, qword ptr [rbp - 0x10]
14123f6b4 jne 0x14123f520
14123f6ba lea rcx, [rbp - 0x10]
14123f6be lea rdx, [rip + 0x19dfd53]
14123f6c5 call 0x1407404d0
14123f6ca jmp 0x14123f520
14123f6cf mov qword ptr [rbp - 0x10], 0
14123f6d7 mov qword ptr [rbp - 8], 8
14123f6df mov qword ptr [rbp], 0
14123f6e7 mov rax, qword ptr [rbp + 0x20]
14123f6eb mov rsi, qword ptr [rbp - 0x30]
14123f6ef mov qword ptr [rsi + 0x10], rax
14123f6f3 movups xmm0, xmmword ptr [rbp + 0x10]
14123f6f7 movups xmmword ptr [rsi], xmm0
14123f6fa mov rdx, qword ptr [rbp - 0x10]
14123f6fe test rdx, rdx
14123f701 je 0x14123f716
14123f703 mov rcx, qword ptr [rbp - 8]
14123f707 shl rdx, 4
14123f70b mov r8d, 8
14123f711 call 0x140613c20
14123f716 cmp qword ptr [rbp - 0x28], 0
14123f71b je 0x14123f75f
14123f71d call 0x140c18130
14123f722 mov rcx, rax
14123f725 mov rax, qword ptr [rax]
14123f728 cmp rax, 1
14123f72c jne 0x14123f7e7
14123f732 add rcx, 8
14123f736 cmp qword ptr [rcx], 0
14123f73a jne 0x14123f80b
14123f740 mov qword ptr [rcx], 0xffffffffffffffff
14123f747 mov qword ptr [rbp + 0x28], rcx
14123f74b add rcx, 8
14123f74f mov rdx, qword ptr [rbp - 0x20]
14123f753 call 0x140c170f0
14123f758 mov rax, qword ptr [rbp + 0x28]
14123f75c inc qword ptr [rax]
14123f75f mov rax, rsi
14123f762 movaps xmm6, xmmword ptr [rbp + 0x40]
14123f766 movaps xmm7, xmmword ptr [rbp + 0x50]
14123f76a movaps xmm8, xmmword ptr [rbp + 0x60]
14123f76f movaps xmm9, xmmword ptr [rbp + 0x70]
14123f774 movaps xmm10, xmmword ptr [rbp + 0x80]
14123f77c movaps xmm11, xmmword ptr [rbp + 0x90]
14123f784 add rsp, 0x128
14123f78b pop rbx
14123f78c pop rdi
14123f78d pop rsi
14123f78e pop r12
14123f790 pop r13
14123f792 pop r14
14123f794 pop r15
14123f796 pop rbp
14123f797 ret 
14123f798 mov rax, qword ptr [rbp + 0x18]
14123f79c mov rcx, qword ptr [rbp + 0x20]
14123f7a0 mov rdx, qword ptr [rbp - 8]
14123f7a4 mov qword ptr [rbp], 0
14123f7ac test rcx, rcx
14123f7af je 0x14123f6e7
14123f7b5 shl rcx, 5
14123f7b9 xor r8d, r8d
14123f7bc nop dword ptr [rax]
14123f7c0 test r9, r9
14123f7c3 je 0x14123f6e7
14123f7c9 movups xmm0, xmmword ptr [rdx]
14123f7cc add rdx, 0x10
14123f7d0 movups xmmword ptr [rax + r8], xmm0
14123f7d5 add r9, -0x10
14123f7d9 add r8, 0x20
14123f7dd cmp rcx, r8
14123f7e0 jne 0x14123f7c0
14123f7e2 jmp 0x14123f6e7
14123f7e7 test rax, rax
14123f7ea jne 0x14123f7ff
14123f7ec xor edx, edx
14123f7ee call 0x1428f15c0
14123f7f3 mov rcx, rax
14123f7f6 test rax, rax
14123f7f9 jne 0x14123f736
14123f7ff lea rcx, [rip + 0x19d9ca2]
14123f806 call 0x1428d80d0
14123f80b lea rcx, [rip + 0x19d9d06]
14123f812 call 0x1428d9250
14123f817 lea r8, [rip + 0x19dfbe2]
14123f81e mov ecx, 8
14123f823 mov rdx, rdi
14123f826 call 0x1428d8fe3
14123f82b ud2 
14123f82d nop dword ptr [rax]
14123f830 mov qword ptr [rsp + 0x10], rdx
14123f835 push rbp
14123f836 push r15
14123f838 push r14
14123f83a push r13
14123f83c push r12
14123f83e push rsi
14123f83f push rdi
14123f840 push rbx
14123f841 sub rsp, 0x98
14123f848 lea rbp, [rdx + 0x80]
14123f84f movaps xmmword ptr [rsp + 0x30], xmm11
14123f855 movaps xmmword ptr [rsp + 0x40], xmm10
14123f85b movaps xmmword ptr [rsp + 0x50], xmm9
14123f861 movaps xmmword ptr [rsp + 0x60], xmm8
14123f867 movaps xmmword ptr [rsp + 0x70], xmm7
14123f86c movaps xmmword ptr [rsp + 0x80], xmm6
14123f874 mov rax, qword ptr [rbp - 0x18]
14123f878 inc qword ptr [rax]
14123f87b movaps xmm6, xmmword ptr [rsp + 0x80]
14123f883 movaps xmm7, xmmword ptr [rsp + 0x70]
14123f888 movaps xmm8, xmmword ptr [rsp + 0x60]
14123f88e movaps xmm9, xmmword ptr [rsp + 0x50]
14123f894 movaps xmm10, xmmword ptr [rsp + 0x40]
14123f89a movaps xmm11, xmmword ptr [rsp + 0x30]
14123f8a0 add rsp, 0x98
14123f8a7 pop rbx
14123f8a8 pop rdi
14123f8a9 pop rsi
14123f8aa pop r12
14123f8ac pop r13
14123f8ae pop r14
14123f8b0 pop r15
14123f8b2 pop rbp
14123f8b3 ret 
14123f8b4 nop word ptr cs:[rax + rax]
14123f8c0 mov qword ptr [rsp + 0x10], rdx
14123f8c5 push rbp
14123f8c6 push r15
14123f8c8 push r14
14123f8ca push r13
14123f8cc push r12
14123f8ce push rsi
14123f8cf push rdi