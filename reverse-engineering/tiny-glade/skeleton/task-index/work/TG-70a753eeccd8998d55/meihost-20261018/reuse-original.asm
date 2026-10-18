0x140683ec0 push r15
0x140683ec2 push r14
0x140683ec4 push r12
0x140683ec6 push rsi
0x140683ec7 push rdi
0x140683ec8 push rbp
0x140683ec9 push rbx
0x140683eca sub rsp, 0x210
0x140683ed1 movaps xmmword ptr [rsp + 0x200], xmm6
0x140683ed9 mov rdi, rdx
0x140683edc mov rsi, rcx
0x140683edf mov qword ptr [rsp + 0x28], rdx
0x140683ee4 mov qword ptr [rsp + 0xb0], rdx
0x140683eec lea rbx, [rcx + 0x1c0]
0x140683ef3 lea rdx, [rsp + 0xb0]
0x140683efb mov rcx, rbx
0x140683efe call 0x1405b0d70
0x140683f03 mov rcx, rax
0x140683f06 mov r15, rax
0x140683f09 shr r15, 0x39
0x140683f0d mov rax, qword ptr [rsi + 0x1a0]
0x140683f14 mov rdx, qword ptr [rsi + 0x1a8]
0x140683f1b mov r8, rdx
0x140683f1e and r8, rcx
0x140683f21 movd xmm0, r15d
0x140683f26 punpcklbw xmm0, xmm0
0x140683f2a pshuflw xmm0, xmm0, 0
0x140683f2f pshufd xmm0, xmm0, 0
0x140683f34 lea r9, [rax - 0xc0]
0x140683f3b xor r10d, r10d
0x140683f3e pcmpeqd xmm1, xmm1
0x140683f42 movdqu xmm2, xmmword ptr [rax + r8]
0x140683f48 movdqa xmm3, xmm2
0x140683f4c pcmpeqb xmm3, xmm0
0x140683f50 pmovmskb r11d, xmm3
0x140683f55 test r11d, r11d
0x140683f58 je 0x140683f90
0x140683f5a tzcnt r14d, r11d
0x140683f5f add r14, r8
0x140683f62 and r14, rdx
0x140683f65 neg r14
0x140683f68 lea r14, [r14 + r14*2]
0x140683f6c shl r14, 6
0x140683f70 cmp qword ptr [r9 + r14], rdi
0x140683f74 je 0x140683fae
0x140683f76 lea ebp, [r11 - 1]
0x140683f7a and bp, r11w
0x140683f7e mov r11d, ebp
0x140683f81 jne 0x140683f5a
0x140683f83 nop word ptr cs:[rax + rax]
0x140683f90 pcmpeqb xmm2, xmm1
0x140683f94 pmovmskb r11d, xmm2
0x140683f99 test r11d, r11d
0x140683f9c jne 0x140683fb8
0x140683f9e add r8, r10
0x140683fa1 add r8, 0x10
0x140683fa5 add r10, 0x10
0x140683fa9 and r8, rdx
0x140683fac jmp 0x140683f42
0x140683fae add rax, r14
0x140683fb1 mov dl, 1
0x140683fb3 jmp 0x14068426a
0x140683fb8 mov r12, rcx
0x140683fbb cmp qword ptr [rsi + 0x1b0], 0
0x140683fc3 je 0x1406842ae
0x140683fc9 lea rbx, [rsi + 0x200]
0x140683fd0 lea rcx, [rsi + 0x220]
0x140683fd7 lea r14, [rsp + 0x28]
0x140683fdc mov rdx, r14
0x140683fdf call 0x1405b0d70
0x140683fe4 lea rcx, [rsp + 0xb0]
0x140683fec mov rdx, rbx
0x140683fef mov r8, rax
0x140683ff2 mov r9, r14
0x140683ff5 call 0x140fcc750
0x140683ffa mov rdx, qword ptr [rsp + 0xb8]
0x140684002 mov rax, rdx
0x140684005 neg rax
0x140684008 jno 0x140684045
0x14068400a xorps xmm6, xmm6
0x14068400d movaps xmmword ptr [rsp + 0x30], xmm6
0x140684012 movaps xmmword ptr [rsp + 0x40], xmm6
0x140684017 movaps xmmword ptr [rsp + 0x50], xmm6
0x14068401c movaps xmmword ptr [rsp + 0x60], xmm6
0x140684021 mov dword ptr [rsp + 0x70], 0
0x140684029 movaps xmmword ptr [rsp + 0x80], xmm6
0x140684031 movaps xmmword ptr [rsp + 0x90], xmm6
0x140684039 xor ebx, ebx
0x14068403b xor ebp, ebp
0x14068403d xor r14d, r14d
0x140684040 jmp 0x140684133
0x140684045 mov rcx, qword ptr [rsp + 0xc0]
0x14068404d movups xmm0, xmmword ptr [rsp + 0xc8]
0x140684055 movaps xmmword ptr [rsp + 0x170], xmm0
0x14068405d movups xmm0, xmmword ptr [rsp + 0xd8]
0x140684065 movaps xmmword ptr [rsp + 0x180], xmm0
0x14068406d movups xmm0, xmmword ptr [rsp + 0xe8]
0x140684075 movaps xmmword ptr [rsp + 0x190], xmm0
0x14068407d movups xmm0, xmmword ptr [rsp + 0xf8]
0x140684085 movaps xmmword ptr [rsp + 0x1a0], xmm0
0x14068408d movups xmm0, xmmword ptr [rsp + 0x108]
0x140684095 movaps xmmword ptr [rsp + 0x1b0], xmm0
0x14068409d movups xmm0, xmmword ptr [rsp + 0x118]
0x1406840a5 movaps xmmword ptr [rsp + 0x1c0], xmm0
0x1406840ad movups xmm0, xmmword ptr [rsp + 0x128]
0x1406840b5 movaps xmmword ptr [rsp + 0x1d0], xmm0
0x1406840bd movups xmm6, xmmword ptr [rsp + 0x138]
0x1406840c5 movzx ebp, byte ptr [rsp + 0x16e]
0x1406840cd movzx ebx, byte ptr [rsp + 0x16f]
0x1406840d5 and bl, 0xf3
0x1406840d8 xorps xmm0, xmm0
0x1406840db movaps xmmword ptr [rsp + 0x30], xmm0
0x1406840e0 movaps xmmword ptr [rsp + 0x40], xmm0
0x1406840e5 movaps xmmword ptr [rsp + 0x50], xmm0
0x1406840ea movaps xmmword ptr [rsp + 0x60], xmm0
0x1406840ef mov dword ptr [rsp + 0x70], 0
0x1406840f7 movaps xmmword ptr [rsp + 0x80], xmm0
0x1406840ff movaps xmmword ptr [rsp + 0x90], xmm0
0x140684107 movdqa xmm0, xmmword ptr [rsp + 0x1d0]
0x140684110 movdqa xmmword ptr [rsp + 0xa0], xmm0
0x140684119 mov r14d, 1
0x14068411f test rdx, rdx
0x140684122 je 0x140684133
0x140684124 shl rdx, 3
0x140684128 mov r8d, 8
0x14068412e call 0x140613c20
0x140684133 mov rcx, qword ptr [rsi + 0x1a0]
0x14068413a mov rax, qword ptr [rsi + 0x1a8]
0x140684141 and r12, rax
0x140684144 movdqu xmm0, xmmword ptr [rcx + r12]
0x14068414a pmovmskb edx, xmm0
0x14068414e test edx, edx
0x140684150 je 0x14068428a
0x140684156 tzcnt edx, edx
0x14068415a add rdx, r12
0x14068415d and rdx, rax
0x140684160 movzx r8d, byte ptr [rcx + rdx]
0x140684165 test r8b, r8b
0x140684168 jns 0x1406842ca
0x14068416e lea r9, [rdx - 0x10]
0x140684172 and r9, rax
0x140684175 mov byte ptr [rcx + rdx], r15b
0x140684179 mov byte ptr [rcx + r9 + 0x10], r15b
0x14068417e neg rdx
0x140684181 lea rdx, [rdx + rdx*2]
0x140684185 shl rdx, 6
0x140684189 lea rax, [rcx + rdx]
0x14068418d and r8b, 1
0x140684191 movzx r8d, r8b
0x140684195 sub qword ptr [rsi + 0x1b0], r8
0x14068419c mov qword ptr [rcx + rdx - 0xc0], rdi
0x1406841a4 mov qword ptr [rcx + rdx - 0xb8], 0
0x1406841b0 mov qword ptr [rcx + rdx - 0xb0], 8
0x1406841bc mov qword ptr [rcx + rdx - 0xa8], 0
0x1406841c8 movaps xmm0, xmmword ptr [rsp + 0x30]
0x1406841cd movaps xmm1, xmmword ptr [rsp + 0x40]
0x1406841d2 movdqa xmm2, xmmword ptr [rsp + 0x50]
0x1406841d8 movdqa xmm3, xmmword ptr [rsp + 0x60]
0x1406841de movups xmmword ptr [rcx + rdx - 0xa0], xmm0
0x1406841e6 movups xmmword ptr [rcx + rdx - 0x90], xmm1
0x1406841ee movdqu xmmword ptr [rcx + rdx - 0x80], xmm2
0x1406841f4 movdqu xmmword ptr [rcx + rdx - 0x70], xmm3
0x1406841fa mov r8d, dword ptr [rsp + 0x70]
0x1406841ff mov dword ptr [rcx + rdx - 0x60], r8d
0x140684204 mov word ptr [rcx + rdx - 0x5c], 0
0x14068420b mov byte ptr [rcx + rdx - 0x5a], 4
0x140684210 movaps xmm0, xmmword ptr [rsp + 0x80]
0x140684218 movdqa xmm1, xmmword ptr [rsp + 0x90]
0x140684221 movups xmmword ptr [rcx + rdx - 0x58], xmm0
0x140684226 movdqu xmmword ptr [rcx + rdx - 0x48], xmm1
0x14068422c movups xmmword ptr [rcx + rdx - 0x38], xmm6
0x140684231 mov dword ptr [rcx + rdx - 0x28], r14d
0x140684236 movaps xmm0, xmmword ptr [rsp + 0xa0]
0x14068423e movups xmmword ptr [rcx + rdx - 0x24], xmm0
0x140684243 movdqa xmm0, xmmword ptr [rip + 0x22e3595]
0x14068424b movdqu xmmword ptr [rcx + rdx - 0x14], xmm0
0x140684251 mov word ptr [rcx + rdx - 4], 0xd0d
0x140684258 mov byte ptr [rcx + rdx - 2], bpl
0x14068425d mov byte ptr [rcx + rdx - 1], bl
0x140684261 inc qword ptr [rsi + 0x1b8]
0x140684268 xor edx, edx
0x14068426a add rax, -0xb8
0x140684270 movaps xmm6, xmmword ptr [rsp + 0x200]
0x140684278 add rsp, 0x210
0x14068427f pop rbx
0x140684280 pop rbp
0x140684281 pop rdi
0x140684282 pop rsi
0x140684283 pop r12
0x140684285 pop r14
0x140684287 pop r15
0x140684289 ret 
0x14068428a mov r8d, 0x10
0x140684290 add r12, r8
0x140684293 and r12, rax
0x140684296 movdqu xmm0, xmmword ptr [rcx + r12]
0x14068429c pmovmskb edx, xmm0
0x1406842a0 add r8, 0x10
0x1406842a4 test edx, edx
0x1406842a6 jne 0x140684156
0x1406842ac jmp 0x140684290
0x1406842ae lea rcx, [rsi + 0x1a0]
0x1406842b5 mov edx, 1
0x1406842ba mov r8, rbx
0x1406842bd mov r9b, 1
0x1406842c0 call 0x1428e7ac0
0x1406842c5 jmp 0x140683fc9
0x1406842ca movdqa xmm0, xmmword ptr [rcx]
0x1406842ce pmovmskb edx, xmm0
0x1406842d2 tzcnt edx, edx
0x1406842d6 movzx r8d, byte ptr [rcx + rdx]
0x1406842db jmp 0x14068416e